use std::fmt::{Display, Formatter};
use std::fs::{File, OpenOptions};
use std::process;
use std::result;
use std::sync::{Arc, RwLock};

use super::{
    EpollContext, ErrorKind, UserResult, VmmActionError, VmmConfig, FC_EXIT_CODE_INVALID_JSON,
};

use devices::legacy::I8042DeviceError;
use devices::virtio::vsock::{TYPE_VSOCK, VSOCK_EVENTS_COUNT};
use devices::virtio::MmioDevice;
use devices::virtio::{BLOCK_EVENTS_COUNT, NET_EVENTS_COUNT, TYPE_BLOCK, TYPE_NET};
use error::StartMicrovmError;
use kernel::{cmdline as kernel_cmdline, loader as kernel_loader};
use logger::{AppInfo, Level, LOGGER};
use memory_model::{GuestAddress, GuestMemory};
use vmm_config;
use vmm_config::boot_source::{
    BootSourceConfig, BootSourceConfigError, KernelConfig, DEFAULT_KERNEL_CMDLINE,
};
use vmm_config::device_config::DeviceConfigs;
use vmm_config::drive::{BlockDeviceConfig, BlockDeviceConfigs, DriveError};
use vmm_config::instance_info::{InstanceInfo, InstanceState};
use vmm_config::logger::{LoggerConfig, LoggerConfigError, LoggerLevel, LoggerWriter};
use vmm_config::machine_config::{VmConfig, VmConfigError};
use vmm_config::net::{
    NetworkInterfaceConfig, NetworkInterfaceConfigs, NetworkInterfaceError,
    NetworkInterfaceUpdateConfig,
};
use vmm_config::vsock::{VsockDeviceConfig, VsockError};

/// Enables pre-boot setup, instantiatioon and real time configuration of a Firecracker VMM.
pub struct VmmController {
    device_configs: DeviceConfigs,
    epoll_context: Option<EpollContext>,
    guest_memory: Option<GuestMemory>,
    instance_initialized: bool,
    kernel_config: Option<KernelConfig>,
    vm_config: VmConfig,
    shared_info: Arc<RwLock<InstanceInfo>>,
}

impl VmmController {
    fn is_instance_initialized(&self) -> bool {
        false
    }

    /// Inserts a block to be attached when the VM starts.
    // Only call this function as part of user configuration.
    // If the drive_id does not exist, a new Block Device Config is added to the list.
    pub fn insert_block_device(&mut self, block_device_config: BlockDeviceConfig) -> UserResult {
        if self.is_instance_initialized() {
            return Err(DriveError::UpdateNotAllowedPostBoot.into());
        }
        self.device_configs
            .block
            .insert(block_device_config)
            .map_err(VmmActionError::from)
    }

    /// Inserts a network device to be attached when the VM starts.
    pub fn insert_net_device(&mut self, body: NetworkInterfaceConfig) -> UserResult {
        if self.is_instance_initialized() {
            return Err(NetworkInterfaceError::UpdateNotAllowedPostBoot.into());
        }
        self.device_configs
            .network_interface
            .insert(body)
            .map_err(|e| VmmActionError::NetworkConfig(ErrorKind::User, e))
    }

    /// Sets a vsock device to be attached when the VM starts.
    pub fn set_vsock_device(&mut self, config: VsockDeviceConfig) -> UserResult {
        if self.is_instance_initialized() {
            Err(VmmActionError::VsockConfig(
                ErrorKind::User,
                VsockError::UpdateNotAllowedPostBoot,
            ))
        } else {
            self.device_configs.vsock = Some(config);
            Ok(())
        }
    }

    fn append_block_devices(
        &mut self,
        mmio_devices: &mut Vec<MmioDevice>,
    ) -> result::Result<(), StartMicrovmError> {
        use StartMicrovmError::*;

        // We rely on check_health function for making sure kernel_config is not None.
        let kernel_config = self.kernel_config.as_mut().ok_or(MissingKernelConfig)?;

        // If no PARTUUID was specified for the root device, try with the /dev/vda.
        if self.device_configs.block.has_root_block_device()
            && !self.device_configs.block.has_partuuid_root()
        {
            kernel_config.cmdline.insert_str("root=/dev/vda")?;

            let flags = if self.device_configs.block.has_read_only_root() {
                "ro"
            } else {
                "rw"
            };

            kernel_config.cmdline.insert_str(flags)?;
        }

        for drive_config in self.device_configs.block.config_list.iter_mut() {
            // Unwraps are safe as both options shouldn't be None when this method is called.
            let epoll_context = self.epoll_context.as_mut().unwrap();
            let guest_memory = self.guest_memory.as_ref().unwrap().clone();

            // Add the block device from file.
            let block_file = OpenOptions::new()
                .read(true)
                .write(!drive_config.is_read_only)
                .open(&drive_config.path_on_host)
                .map_err(OpenBlockDevice)?;

            if drive_config.is_root_device && drive_config.get_partuuid().is_some() {
                kernel_config.cmdline.insert_str(format!(
                    "root=PARTUUID={}",
                    //The unwrap is safe as we are firstly checking that partuuid is_some().
                    drive_config.get_partuuid().unwrap()
                ))?;

                let flags = if drive_config.is_read_only() {
                    "ro"
                } else {
                    "rw"
                };

                kernel_config.cmdline.insert_str(flags)?;
            }

            let epoll_config = epoll_context.allocate_tokens_for_virtio_device(
                TYPE_BLOCK,
                &drive_config.drive_id,
                BLOCK_EVENTS_COUNT,
            );

            let rate_limiter = drive_config
                .rate_limiter
                .map(vmm_config::RateLimiterConfig::into_rate_limiter)
                .transpose()
                .map_err(CreateRateLimiter)?;

            let block_box = Box::new(
                devices::virtio::Block::new(
                    block_file,
                    drive_config.is_read_only,
                    epoll_config,
                    rate_limiter,
                )
                .map_err(CreateBlockDevice)?,
            );

            mmio_devices.push(MmioDevice::new(guest_memory, block_box).map_err(|e| {
                RegisterMMIODevice(super::device_manager::mmio::Error::CreateMmioDevice(e))
            })?);
        }

        Ok(())
    }

    fn append_net_devices(
        &mut self,
        mmio_devices: &mut Vec<MmioDevice>,
    ) -> result::Result<(), StartMicrovmError> {
        use StartMicrovmError::*;

        // We rely on check_health function for making sure kernel_config is not None.
        let kernel_config = self.kernel_config.as_mut().ok_or(MissingKernelConfig)?;

        for cfg in self.device_configs.network_interface.iter_mut() {
            // Unwraps are safe as both options shouldn't be None when this method is called.
            let epoll_context = self.epoll_context.as_mut().unwrap();
            let guest_memory = self.guest_memory.as_ref().unwrap().clone();

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

            let net_box = Box::new(
                devices::virtio::Net::new_with_tap(
                    tap,
                    cfg.guest_mac(),
                    epoll_config,
                    rx_rate_limiter,
                    tx_rate_limiter,
                    allow_mmds_requests,
                )
                .map_err(CreateNetDevice)?,
            );

            mmio_devices.push(MmioDevice::new(guest_memory, net_box).map_err(|e| {
                RegisterMMIODevice(super::device_manager::mmio::Error::CreateMmioDevice(e))
            })?);
        }
        Ok(())
    }

    fn append_vsock_device(
        &mut self,
        mmio_devices: &mut Vec<MmioDevice>,
    ) -> result::Result<(), StartMicrovmError> {
        let kernel_config = self
            .kernel_config
            .as_mut()
            .ok_or(StartMicrovmError::MissingKernelConfig)?;

        if let Some(cfg) = &self.device_configs.vsock {
            // Unwraps are safe as both options shouldn't be None when this method is called.
            let epoll_context = self.epoll_context.as_mut().unwrap();
            let guest_memory = self.guest_memory.as_ref().unwrap().clone();

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

            let vsock_box = Box::new(
                devices::virtio::Vsock::new(u64::from(cfg.guest_cid), epoll_config, backend)
                    .map_err(StartMicrovmError::CreateVsockDevice)?,
            );

            mmio_devices.push(MmioDevice::new(guest_memory, vsock_box).map_err(|e| {
                StartMicrovmError::RegisterMMIODevice(
                    super::device_manager::mmio::Error::CreateMmioDevice(e),
                )
            })?);
        }
        Ok(())
    }

    fn init_guest_memory(&mut self) -> std::result::Result<(), StartMicrovmError> {
        if self.guest_memory.is_none() {
            let mem_size = self
                .vm_config
                .mem_size_mib
                .ok_or(StartMicrovmError::GuestMemory(
                    memory_model::GuestMemoryError::MemoryNotInitialized,
                ))?
                << 20;
            let arch_mem_regions = arch::arch_memory_regions(mem_size);
            self.guest_memory =
                Some(GuestMemory::new(&arch_mem_regions).map_err(StartMicrovmError::GuestMemory)?);
        }
        Ok(())
    }

    fn load_kernel(&mut self) -> std::result::Result<GuestAddress, StartMicrovmError> {
        use StartMicrovmError::*;

        // This is the easy way out of consuming the value of the kernel_cmdline.
        let kernel_config = self.kernel_config.as_mut().ok_or(MissingKernelConfig)?;

        // It is safe to unwrap because the VM memory was initialized before in vm.memory_init().
        let guest_memory = self.guest_memory.as_ref().ok_or(GuestMemory(
            memory_model::GuestMemoryError::MemoryNotInitialized,
        ))?;

        let entry_addr = kernel_loader::load_kernel(
            guest_memory,
            &mut kernel_config.kernel_file,
            arch::get_kernel_start(),
        )
        .map_err(KernelLoader)?;

        // This is x86_64 specific since on aarch64 the commandline will be specified through the FDT.
        #[cfg(target_arch = "x86_64")]
        kernel_loader::load_cmdline(
            guest_memory,
            GuestAddress(arch::x86_64::layout::CMDLINE_START),
            &kernel_config
                .cmdline
                .as_cstring()
                .map_err(LoadCommandline)?,
        )
        .map_err(LoadCommandline)?;

        Ok(entry_addr)
    }

    fn set_kernel_config(&mut self, kernel_config: KernelConfig) {
        self.kernel_config = Some(kernel_config);
    }

    /// Set the guest boot source configuration.
    pub fn configure_boot_source(&mut self, boot_source_cfg: BootSourceConfig) -> UserResult {
        use BootSourceConfigError::{
            InvalidKernelCommandLine, InvalidKernelPath, UpdateNotAllowedPostBoot,
        };
        use ErrorKind::User;
        use VmmActionError::BootSource;

        if self.is_instance_initialized() {
            return Err(BootSource(User, UpdateNotAllowedPostBoot));
        }

        let kernel_file = File::open(boot_source_cfg.kernel_image_path)
            .map_err(|e| BootSource(User, InvalidKernelPath(e)))?;

        let mut cmdline = kernel_cmdline::Cmdline::new(arch::CMDLINE_MAX_SIZE);
        cmdline
            .insert_str(
                boot_source_cfg
                    .boot_args
                    .unwrap_or_else(|| String::from(DEFAULT_KERNEL_CMDLINE)),
            )
            .map_err(|e| BootSource(User, InvalidKernelCommandLine(e.to_string())))?;

        let kernel_config = KernelConfig {
            kernel_file,
            cmdline,
        };
        self.set_kernel_config(kernel_config);

        Ok(())
    }

    /// Set the machine configuration of the microVM.
    pub fn set_vm_configuration(&mut self, machine_config: VmConfig) -> UserResult {
        if self.is_instance_initialized() {
            return Err(VmConfigError::UpdateNotAllowedPostBoot.into());
        }

        if machine_config.vcpu_count == Some(0) {
            return Err(VmConfigError::InvalidVcpuCount.into());
        }

        if machine_config.mem_size_mib == Some(0) {
            return Err(VmConfigError::InvalidMemorySize.into());
        }

        let ht_enabled = machine_config
            .ht_enabled
            .unwrap_or_else(|| self.vm_config.ht_enabled.unwrap());

        let vcpu_count_value = machine_config
            .vcpu_count
            .unwrap_or_else(|| self.vm_config.vcpu_count.unwrap());

        // If hyperthreading is enabled or is to be enabled in this call
        // only allow vcpu count to be 1 or even.
        if ht_enabled && vcpu_count_value > 1 && vcpu_count_value % 2 == 1 {
            return Err(VmConfigError::InvalidVcpuCount.into());
        }

        // Update all the fields that have a new value.
        self.vm_config.vcpu_count = Some(vcpu_count_value);
        self.vm_config.ht_enabled = Some(ht_enabled);

        if machine_config.mem_size_mib.is_some() {
            self.vm_config.mem_size_mib = machine_config.mem_size_mib;
        }

        if machine_config.cpu_template.is_some() {
            self.vm_config.cpu_template = machine_config.cpu_template;
        }

        Ok(())
    }

    /// Configures the logger as described in `logger_cfg`.
    pub fn init_logger(&self, logger_cfg: LoggerConfig) -> UserResult {
        if self.is_instance_initialized() {
            return Err(VmmActionError::Logger(
                ErrorKind::User,
                LoggerConfigError::InitializationFailure(
                    "Cannot initialize logger after boot.".to_string(),
                ),
            ));
        }

        let firecracker_version;
        {
            let guard = self.shared_info.read().unwrap();
            LOGGER.set_instance_id(guard.id.clone());
            firecracker_version = guard.vmm_version.clone();
        }

        LOGGER.set_level(match logger_cfg.level {
            LoggerLevel::Error => Level::Error,
            LoggerLevel::Warning => Level::Warn,
            LoggerLevel::Info => Level::Info,
            LoggerLevel::Debug => Level::Debug,
        });

        LOGGER.set_include_origin(logger_cfg.show_log_origin, logger_cfg.show_log_origin);
        LOGGER.set_include_level(logger_cfg.show_level);

        #[cfg(target_arch = "aarch64")]
        let options: &Vec<Value> = &vec![];

        #[cfg(target_arch = "x86_64")]
        let options = logger_cfg.options.as_array().unwrap();

        LOGGER.set_flags(options).map_err(|e| {
            VmmActionError::Logger(
                ErrorKind::User,
                LoggerConfigError::InitializationFailure(e.to_string()),
            )
        })?;

        LOGGER
            .init(
                &AppInfo::new("Firecracker", &firecracker_version),
                Box::new(LoggerWriter::new(&logger_cfg.log_fifo).map_err(|e| {
                    VmmActionError::Logger(
                        ErrorKind::User,
                        LoggerConfigError::InitializationFailure(e.to_string()),
                    )
                })?),
                Box::new(LoggerWriter::new(&logger_cfg.metrics_fifo).map_err(|e| {
                    VmmActionError::Logger(
                        ErrorKind::User,
                        LoggerConfigError::InitializationFailure(e.to_string()),
                    )
                })?),
            )
            .map_err(|e| {
                VmmActionError::Logger(
                    ErrorKind::User,
                    LoggerConfigError::InitializationFailure(e.to_string()),
                )
            })
    }

    /// Configures Vmm resources as described by the `config_json` param.
    pub fn configure_from_json(
        &mut self,
        config_json: String,
    ) -> result::Result<(), VmmActionError> {
        let vmm_config = serde_json::from_slice::<VmmConfig>(config_json.as_bytes())
            .unwrap_or_else(|e| {
                error!("Invalid json: {}", e);
                process::exit(i32::from(FC_EXIT_CODE_INVALID_JSON));
            });

        if let Some(logger) = vmm_config.logger {
            self.init_logger(logger)?;
        }
        self.configure_boot_source(vmm_config.boot_source)?;
        for drive_config in vmm_config.block_devices.into_iter() {
            self.insert_block_device(drive_config)?;
        }
        for net_config in vmm_config.net_devices.into_iter() {
            self.insert_net_device(net_config)?;
        }
        if let Some(machine_config) = vmm_config.machine_config {
            self.set_vm_configuration(machine_config)?;
        }
        if let Some(vsock_config) = vmm_config.vsock_device {
            self.set_vsock_device(vsock_config)?;
        }
        Ok(())
    }

    /// Starts a microVM based on the current configuration.
    pub fn start_microvm(&mut self) {
        self.instance_initialized = true;
    }
}
