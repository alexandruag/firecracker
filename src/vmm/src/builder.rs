// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Enables pre-boot setup, instantiation and booting of a Firecracker VMM.

use timerfd::{ClockId, SetTimeFlags, TimerFd, TimerState};

use std::fs::{File, OpenOptions};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::{EpollContext, EpollDispatch, VcpuConfig, Vmm};

#[cfg(target_arch = "x86_64")]
use device_manager::legacy::PortIODeviceManager;
use device_manager::mmio::MMIODeviceManager;
use devices::virtio::vsock::{TYPE_VSOCK, VSOCK_EVENTS_COUNT};
use devices::virtio::{MmioDevice, NET_EVENTS_COUNT, TYPE_NET};
use error::*;
use logger::{Metric, LOGGER, METRICS};
use memory_model::{GuestAddress, GuestMemory};
use polly::event_manager::EventManager;
use resources::VmResources;
use utils::time::TimestampUs;
use vmm_config;
use vmm_config::boot_source::{BootSourceConfig, DEFAULT_KERNEL_CMDLINE};
use vstate::{KvmContext, Vm};

const WRITE_METRICS_PERIOD_SECONDS: u64 = 60;

/// Builds and starts a microVM based on the current configuration.
pub fn build_microvm(
    vm_resources: &VmResources,
    epoll_context: &mut EpollContext,
    event_manager: &mut EventManager,
    seccomp_level: u32,
) -> std::result::Result<Vmm, VmmActionError> {
    let boot_src_cfg = vm_resources
        .boot_source()
        .ok_or(StartMicrovmError::MissingKernelConfig)?;

    // Timestamp for measuring microVM boot duration.
    let request_ts = TimestampUs::default();

    let guest_memory = create_guest_memory(vm_resources)?;
    let vcpu_config = vcpu_config(vm_resources);
    let entry_addr = load_kernel(boot_src_cfg, &guest_memory)?;
    let kernel_cmdline = setup_cmdline(boot_src_cfg)?;
    let write_metrics_event_fd = setup_metrics(epoll_context)?;
    let vm = setup_kvm_vm(guest_memory.clone())?;

    // Instantiate the MMIO device manager.
    // 'mmio_base' address has to be an address which is protected by the kernel
    // and is architectural specific.
    let mmio_device_manager = MMIODeviceManager::new(
        guest_memory.clone(),
        &mut (arch::MMIO_MEM_START as u64),
        (arch::IRQ_BASE, arch::IRQ_MAX),
    );

    // TODO: All Vmm setup should move outside of Vmm, including irqchip and legacy devices setup.
    // TODO: The Vmm would be created as the last step that brings all the configured resources
    // TODO: together.
    let mut vmm = Vmm {
        stdin_handle: std::io::stdin(),
        guest_memory,
        vcpu_config,
        kernel_cmdline,
        vcpus_handles: Vec::new(),
        exit_evt: None,
        vm,
        mmio_device_manager,
        #[cfg(target_arch = "x86_64")]
        pio_device_manager: PortIODeviceManager::new()
            .map_err(Error::CreateLegacyDevice)
            .map_err(StartMicrovmError::Internal)?,
        write_metrics_event_fd,
        seccomp_level,
    };

    // For x86_64 we need to create the interrupt controller before calling `KVM_CREATE_VCPUS`
    // while on aarch64 we need to do it the other way around.
    #[cfg(target_arch = "x86_64")]
    {
        vmm.setup_interrupt_controller()?;
        // This call has to be here after setting up the irqchip, because
        // we set up some irqfd inside for some reason.
        vmm.attach_legacy_devices()?;
    }

    let vcpus = vmm.create_vcpus(entry_addr, request_ts)?;

    #[cfg(target_arch = "aarch64")]
    {
        vmm.setup_interrupt_controller()?;
        vmm.attach_legacy_devices()?;
    }

    attach_block_devices(&mut vmm, vm_resources, event_manager)?;
    attach_net_devices(&mut vmm, vm_resources, epoll_context)?;
    attach_vsock_device(&mut vmm, vm_resources, epoll_context)?;

    // Write the kernel command line to guest memory. This is x86_64 specific, since on
    // aarch64 the command line will be specified through the FDT.
    #[cfg(target_arch = "x86_64")]
    load_cmdline(&vmm)?;

    vmm.configure_system(vcpus.as_slice())?;
    vmm.register_events(epoll_context)?;
    vmm.start_vcpus(vcpus)?;

    arm_logger_and_metrics(&mut vmm);

    Ok(vmm)
}

fn create_guest_memory(
    vm_resources: &VmResources,
) -> std::result::Result<GuestMemory, StartMicrovmError> {
    let mem_size = vm_resources
        .vm_config()
        .mem_size_mib
        .ok_or(StartMicrovmError::GuestMemory(
            memory_model::GuestMemoryError::MemoryNotInitialized,
        ))?
        << 20;
    let arch_mem_regions = arch::arch_memory_regions(mem_size);

    Ok(GuestMemory::new(&arch_mem_regions).map_err(StartMicrovmError::GuestMemory)?)
}

fn vcpu_config(vm_resources: &VmResources) -> VcpuConfig {
    // The unwraps are ok to use because the values are initialized using defaults if not
    // supplied by the user.
    VcpuConfig {
        vcpu_count: vm_resources.vm_config().vcpu_count.unwrap(),
        ht_enabled: vm_resources.vm_config().ht_enabled.unwrap(),
        cpu_template: vm_resources.vm_config().cpu_template,
    }
}

fn load_kernel(
    boot_src_cfg: &BootSourceConfig,
    guest_memory: &GuestMemory,
) -> std::result::Result<GuestAddress, StartMicrovmError> {
    // FIXME: use the right error here.
    let mut kernel_file =
        File::open(&boot_src_cfg.kernel_image_path).map_err(StartMicrovmError::VcpuSpawn)?;

    let entry_addr =
        kernel::loader::load_kernel(guest_memory, &mut kernel_file, arch::get_kernel_start())
            .map_err(StartMicrovmError::KernelLoader)?;

    Ok(entry_addr)
}

fn setup_cmdline(
    boot_src_cfg: &BootSourceConfig,
) -> std::result::Result<kernel::cmdline::Cmdline, StartMicrovmError> {
    let mut cmdline = kernel::cmdline::Cmdline::new(arch::CMDLINE_MAX_SIZE);
    let boot_args = match boot_src_cfg.boot_args.as_ref() {
        None => DEFAULT_KERNEL_CMDLINE,
        Some(str) => str.as_str(),
    };
    cmdline
        .insert_str(boot_args)
        .map_err(|e| StartMicrovmError::KernelCmdline(e.to_string()))?;
    Ok(cmdline)
}

#[cfg(target_arch = "x86_64")]
fn load_cmdline(vmm: &Vmm) -> std::result::Result<(), StartMicrovmError> {
    kernel::loader::load_cmdline(
        vmm.guest_memory(),
        GuestAddress(arch::x86_64::layout::CMDLINE_START),
        &vmm.kernel_cmdline
            .as_cstring()
            .map_err(StartMicrovmError::LoadCommandline)?,
    )
    .map_err(StartMicrovmError::LoadCommandline)
}

fn setup_metrics(
    epoll_context: &mut EpollContext,
) -> std::result::Result<TimerFd, StartMicrovmError> {
    let write_metrics_event_fd = TimerFd::new_custom(ClockId::Monotonic, true, true)
        .map_err(Error::TimerFd)
        .map_err(StartMicrovmError::Internal)?;
    // TODO: remove expect.
    epoll_context
        .add_epollin_event(
            // non-blocking & close on exec
            &write_metrics_event_fd,
            EpollDispatch::WriteMetrics,
        )
        .expect("Cannot add write metrics TimerFd to epoll.");
    Ok(write_metrics_event_fd)
}

fn setup_kvm_vm(guest_memory: GuestMemory) -> std::result::Result<Vm, VmmActionError> {
    let kvm = KvmContext::new()
        .map_err(Error::KvmContext)
        .map_err(StartMicrovmError::Internal)?;
    let mut vm = Vm::new(kvm.fd())
        .map_err(Error::Vm)
        .map_err(StartMicrovmError::Internal)?;
    vm.memory_init(guest_memory.clone(), kvm.max_memslots())
        .map_err(StartMicrovmError::ConfigureVm)?;
    Ok(vm)
}

/// Adds a MmioDevice.
fn attach_mmio_device(
    vmm: &mut Vmm,
    id: String,
    device: MmioDevice,
) -> std::result::Result<(), StartMicrovmError> {
    // TODO: we currently map into StartMicrovmError::RegisterBlockDevice for all
    // devices at the end of device_manager.register_mmio_device.
    let type_id = device
        .device()
        .lock()
        .expect("Poisoned device lock")
        .device_type();
    let cmdline = &mut vmm.kernel_cmdline;

    vmm.mmio_device_manager
        .register_mmio_device(vmm.vm.fd(), device, cmdline, type_id, id.as_str())
        .map_err(StartMicrovmError::RegisterBlockDevice)?;

    Ok(())
}

/// Adds a virtio device to the MmioDeviceManager using the specified transport.
fn attach_block_device(
    vmm: &mut Vmm,
    id: String,
    transport_device: MmioDevice,
    block_device: Arc<Mutex<devices::virtio::Block>>,
) -> std::result::Result<(), StartMicrovmError> {
    let cmdline = &mut vmm.kernel_cmdline;

    vmm.mmio_device_manager
        .register_block_device(
            vmm.vm.fd(),
            transport_device,
            block_device,
            cmdline,
            id.as_str(),
        )
        .map_err(StartMicrovmError::RegisterBlockDevice)?;

    Ok(())
}

fn attach_block_devices(
    vmm: &mut Vmm,
    vm_resources: &VmResources,
    event_manager: &mut EventManager,
) -> std::result::Result<(), StartMicrovmError> {
    use StartMicrovmError::*;

    // If no PARTUUID was specified for the root device, try with the /dev/vda.
    if vm_resources.block.has_root_block_device() && !vm_resources.block.has_partuuid_root() {
        let kernel_cmdline = &mut vmm.kernel_cmdline;

        kernel_cmdline.insert_str("root=/dev/vda")?;

        let flags = if vm_resources.block.has_read_only_root() {
            "ro"
        } else {
            "rw"
        };

        kernel_cmdline.insert_str(flags)?;
    }

    for drive_config in vm_resources.block.config_list.iter() {
        // Add the block device from file.
        let block_file = OpenOptions::new()
            .read(true)
            .write(!drive_config.is_read_only)
            .open(&drive_config.path_on_host)
            .map_err(OpenBlockDevice)?;

        if drive_config.is_root_device && drive_config.get_partuuid().is_some() {
            let kernel_cmdline = &mut vmm.kernel_cmdline;

            kernel_cmdline.insert_str(format!(
                "root=PARTUUID={}",
                //The unwrap is safe as we are firstly checking that partuuid is_some().
                drive_config.get_partuuid().unwrap()
            ))?;

            let flags = if drive_config.is_read_only() {
                "ro"
            } else {
                "rw"
            };

            kernel_cmdline.insert_str(flags)?;
        }

        let rate_limiter = drive_config
            .rate_limiter
            .map(vmm_config::RateLimiterConfig::into_rate_limiter)
            .transpose()
            .map_err(CreateRateLimiter)?;

        error!("Registering");

        let block_device = event_manager
            .register(
                devices::virtio::Block::new(
                    block_file,
                    drive_config.is_read_only,
                    rate_limiter.unwrap_or_default(),
                )
                .map_err(CreateBlockDevice)?,
            )
            .map_err(|_| StartMicrovmError::RegisterEvent)?;

        error!("Attaching");
        attach_block_device(
            vmm,
            drive_config.drive_id.clone(),
            MmioDevice::new(vmm.guest_memory().clone(), block_device.clone()).map_err(|e| {
                RegisterMMIODevice(super::device_manager::mmio::Error::CreateMmioDevice(e))
            })?,
            block_device,
        )?;
    }

    Ok(())
}

fn attach_net_devices(
    vmm: &mut Vmm,
    vm_resources: &VmResources,
    epoll_context: &mut EpollContext,
) -> UserResult {
    use StartMicrovmError::*;

    for cfg in vm_resources.network_interface.iter() {
        let epoll_config = epoll_context.allocate_tokens_for_virtio_device(
            TYPE_NET,
            &cfg.iface_id,
            NET_EVENTS_COUNT,
        );

        let allow_mmds_requests = cfg.allow_mmds_requests();

        let rx_rate_limiter = cfg
            .rx_rate_limiter
            .map(vmm_config::RateLimiterConfig::into_rate_limiter)
            .transpose()
            .map_err(CreateRateLimiter)?;

        let tx_rate_limiter = cfg
            .tx_rate_limiter
            .map(vmm_config::RateLimiterConfig::into_rate_limiter)
            .transpose()
            .map_err(CreateRateLimiter)?;

        let tap = cfg.open_tap().map_err(|_| NetDeviceNotConfigured)?;

        let net_box = Arc::new(Mutex::new(
            devices::virtio::Net::new_with_tap(
                tap,
                cfg.guest_mac(),
                epoll_config,
                rx_rate_limiter,
                tx_rate_limiter,
                allow_mmds_requests,
            )
            .map_err(CreateNetDevice)?,
        ));

        attach_mmio_device(
            vmm,
            cfg.iface_id.clone(),
            MmioDevice::new(vmm.guest_memory().clone(), net_box).map_err(|e| {
                RegisterMMIODevice(super::device_manager::mmio::Error::CreateMmioDevice(e))
            })?,
        )?;
    }

    Ok(())
}

fn attach_vsock_device(
    vmm: &mut Vmm,
    vm_resources: &VmResources,
    epoll_context: &mut EpollContext,
) -> UserResult {
    if let Some(cfg) = vm_resources.vsock.as_ref() {
        let backend = devices::virtio::vsock::VsockUnixBackend::new(
            u64::from(cfg.guest_cid),
            cfg.uds_path.clone(),
        )
        .map_err(StartMicrovmError::CreateVsockBackend)?;

        let epoll_config = epoll_context.allocate_tokens_for_virtio_device(
            TYPE_VSOCK,
            &cfg.vsock_id,
            VSOCK_EVENTS_COUNT,
        );

        let vsock_box = Arc::new(Mutex::new(
            devices::virtio::Vsock::new(u64::from(cfg.guest_cid), epoll_config, backend)
                .map_err(StartMicrovmError::CreateVsockDevice)?,
        ));

        attach_mmio_device(
            vmm,
            cfg.vsock_id.clone(),
            MmioDevice::new(vmm.guest_memory().clone(), vsock_box).map_err(|e| {
                StartMicrovmError::RegisterMMIODevice(
                    super::device_manager::mmio::Error::CreateMmioDevice(e),
                )
            })?,
        )?;
    }

    Ok(())
}

fn arm_logger_and_metrics(vmm: &mut Vmm) {
    // Arm the log write timer.
    let timer_state = TimerState::Periodic {
        current: Duration::from_secs(WRITE_METRICS_PERIOD_SECONDS),
        interval: Duration::from_secs(WRITE_METRICS_PERIOD_SECONDS),
    };
    vmm.write_metrics_event_fd
        .set_state(timer_state, SetTimeFlags::Default);

    // Log the metrics straight away to check the process startup time.
    if LOGGER.log_metrics().is_err() {
        METRICS.logger.missed_metrics_count.inc();
    }
}
