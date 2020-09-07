//! Import layout tests from upstream to ensure our struct definitions match.

use super::*;

#[test]
fn bindgen_test_layout_kvm_pic_state() {
    assert_eq!(
        ::std::mem::size_of::<kvm_pic_state>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_pic_state))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_pic_state>(),
        1usize,
        concat!("Alignment of ", stringify!(kvm_pic_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).last_irr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(last_irr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).irr as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(irr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).imr as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(imr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).isr as *const _ as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(isr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).priority_add as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(priority_add)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).irq_base as *const _ as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(irq_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).read_reg_select as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(read_reg_select)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).poll as *const _ as usize },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(poll)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).special_mask as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(special_mask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).init_state as *const _ as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(init_state)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).auto_eoi as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(auto_eoi)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_pic_state>())).rotate_on_auto_eoi as *const _ as usize
        },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(rotate_on_auto_eoi)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_pic_state>())).special_fully_nested_mode as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(special_fully_nested_mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).init4 as *const _ as usize },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(init4)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).elcr as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(elcr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).elcr_mask as *const _ as usize },
        15usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(elcr_mask)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1>())).vector
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(vector)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1>())).reserved
                as *const _ as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(reserved)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1>())).dest_id
                as *const _ as usize
        },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(dest_id)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_ioapic_state__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_ioapic_state__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_ioapic_state__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_ioapic_state__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_ioapic_state__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_ioapic_state__bindgen_ty_1>())).bits as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state__bindgen_ty_1),
            "::",
            stringify!(bits)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_ioapic_state__bindgen_ty_1>())).fields as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state__bindgen_ty_1),
            "::",
            stringify!(fields)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_ioapic_state() {
    assert_eq!(
        ::std::mem::size_of::<kvm_ioapic_state>(),
        216usize,
        concat!("Size of: ", stringify!(kvm_ioapic_state))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_ioapic_state>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_ioapic_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ioapic_state>())).base_address as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state),
            "::",
            stringify!(base_address)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ioapic_state>())).ioregsel as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state),
            "::",
            stringify!(ioregsel)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ioapic_state>())).id as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ioapic_state>())).irr as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state),
            "::",
            stringify!(irr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ioapic_state>())).pad as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ioapic_state>())).redirtbl as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state),
            "::",
            stringify!(redirtbl)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_regs() {
    assert_eq!(
        ::std::mem::size_of::<kvm_regs>(),
        144usize,
        concat!("Size of: ", stringify!(kvm_regs))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_regs>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_regs))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rax as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rax)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rbx as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rbx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rcx as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rcx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rdx as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rdx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rsi as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rsi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rdi as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rdi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rsp as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rsp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rbp as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rbp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r8 as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r8)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r9 as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r9)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r10 as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r10)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r11 as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r11)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r12 as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r12)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r13 as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r13)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r14 as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r14)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r15 as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r15)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rip as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rip)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rflags as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rflags)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_lapic_state() {
    assert_eq!(
        ::std::mem::size_of::<kvm_lapic_state>(),
        1024usize,
        concat!("Size of: ", stringify!(kvm_lapic_state))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_lapic_state>(),
        1usize,
        concat!("Alignment of ", stringify!(kvm_lapic_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_lapic_state>())).regs as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_lapic_state),
            "::",
            stringify!(regs)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_segment() {
    assert_eq!(
        ::std::mem::size_of::<kvm_segment>(),
        24usize,
        concat!("Size of: ", stringify!(kvm_segment))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_segment>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_segment))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).base as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).limit as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(limit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).selector as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(selector)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).type_ as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).present as *const _ as usize },
        15usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(present)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).dpl as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(dpl)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).db as *const _ as usize },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(db)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).s as *const _ as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(s)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).l as *const _ as usize },
        19usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(l)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).g as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(g)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).avl as *const _ as usize },
        21usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(avl)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).unusable as *const _ as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(unusable)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).padding as *const _ as usize },
        23usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(padding)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_dtable() {
    assert_eq!(
        ::std::mem::size_of::<kvm_dtable>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_dtable))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_dtable>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_dtable))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_dtable>())).base as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_dtable),
            "::",
            stringify!(base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_dtable>())).limit as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_dtable),
            "::",
            stringify!(limit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_dtable>())).padding as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_dtable),
            "::",
            stringify!(padding)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_sregs() {
    assert_eq!(
        ::std::mem::size_of::<kvm_sregs>(),
        312usize,
        concat!("Size of: ", stringify!(kvm_sregs))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_sregs>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_sregs))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).cs as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(cs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).ds as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(ds)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).es as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(es)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).fs as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(fs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).gs as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(gs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).ss as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(ss)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).tr as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(tr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).ldt as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(ldt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).gdt as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(gdt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).idt as *const _ as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(idt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).cr0 as *const _ as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(cr0)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).cr2 as *const _ as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(cr2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).cr3 as *const _ as usize },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(cr3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).cr4 as *const _ as usize },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(cr4)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).cr8 as *const _ as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(cr8)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).efer as *const _ as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(efer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).apic_base as *const _ as usize },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(apic_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).interrupt_bitmap as *const _ as usize },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(interrupt_bitmap)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_msr_entry() {
    assert_eq!(
        ::std::mem::size_of::<kvm_msr_entry>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_msr_entry))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_msr_entry>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_msr_entry))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msr_entry>())).index as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msr_entry),
            "::",
            stringify!(index)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msr_entry>())).reserved as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msr_entry),
            "::",
            stringify!(reserved)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msr_entry>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msr_entry),
            "::",
            stringify!(data)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_msrs() {
    assert_eq!(
        ::std::mem::size_of::<kvm_msrs>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_msrs))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_msrs>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_msrs))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msrs>())).nmsrs as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msrs),
            "::",
            stringify!(nmsrs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msrs>())).pad as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msrs),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msrs>())).entries as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msrs),
            "::",
            stringify!(entries)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_cpuid_entry2() {
    assert_eq!(
        ::std::mem::size_of::<kvm_cpuid_entry2>(),
        40usize,
        concat!("Size of: ", stringify!(kvm_cpuid_entry2))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_cpuid_entry2>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_cpuid_entry2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry2>())).function as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry2),
            "::",
            stringify!(function)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry2>())).index as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry2),
            "::",
            stringify!(index)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry2>())).flags as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry2),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry2>())).eax as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry2),
            "::",
            stringify!(eax)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry2>())).ebx as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry2),
            "::",
            stringify!(ebx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry2>())).ecx as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry2),
            "::",
            stringify!(ecx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry2>())).edx as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry2),
            "::",
            stringify!(edx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry2>())).padding as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry2),
            "::",
            stringify!(padding)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_cpuid2() {
    assert_eq!(
        ::std::mem::size_of::<kvm_cpuid2>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_cpuid2))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_cpuid2>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_cpuid2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid2>())).nent as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid2),
            "::",
            stringify!(nent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid2>())).padding as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid2),
            "::",
            stringify!(padding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid2>())).entries as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid2),
            "::",
            stringify!(entries)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_pit_channel_state() {
    assert_eq!(
        ::std::mem::size_of::<kvm_pit_channel_state>(),
        24usize,
        concat!("Size of: ", stringify!(kvm_pit_channel_state))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_pit_channel_state>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_pit_channel_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_channel_state>())).count as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_pit_channel_state>())).latched_count as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(latched_count)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_pit_channel_state>())).count_latched as *const _ as usize
        },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(count_latched)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_pit_channel_state>())).status_latched as *const _ as usize
        },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(status_latched)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_channel_state>())).status as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_pit_channel_state>())).read_state as *const _ as usize
        },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(read_state)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_pit_channel_state>())).write_state as *const _ as usize
        },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(write_state)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_pit_channel_state>())).write_latch as *const _ as usize
        },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(write_latch)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_channel_state>())).rw_mode as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(rw_mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_channel_state>())).mode as *const _ as usize },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_channel_state>())).bcd as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(bcd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_channel_state>())).gate as *const _ as usize },
        15usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(gate)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_pit_channel_state>())).count_load_time as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(count_load_time)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_pit_state2() {
    assert_eq!(
        ::std::mem::size_of::<kvm_pit_state2>(),
        112usize,
        concat!("Size of: ", stringify!(kvm_pit_state2))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_pit_state2>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_pit_state2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_state2>())).channels as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_state2),
            "::",
            stringify!(channels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_state2>())).flags as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_state2),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_state2>())).reserved as *const _ as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_state2),
            "::",
            stringify!(reserved)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_vcpu_events__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_vcpu_events__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_vcpu_events__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_vcpu_events__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_vcpu_events__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_1>())).injected as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_1),
            "::",
            stringify!(injected)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_1>())).nr as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_1),
            "::",
            stringify!(nr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_1>())).has_error_code as *const _
                as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_1),
            "::",
            stringify!(has_error_code)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_1>())).pending as *const _ as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_1),
            "::",
            stringify!(pending)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_1>())).error_code as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_1),
            "::",
            stringify!(error_code)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_vcpu_events__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<kvm_vcpu_events__bindgen_ty_2>(),
        4usize,
        concat!("Size of: ", stringify!(kvm_vcpu_events__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_vcpu_events__bindgen_ty_2>(),
        1usize,
        concat!("Alignment of ", stringify!(kvm_vcpu_events__bindgen_ty_2))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_2>())).injected as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_2),
            "::",
            stringify!(injected)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_2>())).nr as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_2),
            "::",
            stringify!(nr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_2>())).soft as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_2),
            "::",
            stringify!(soft)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_2>())).shadow as *const _ as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_2),
            "::",
            stringify!(shadow)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_vcpu_events__bindgen_ty_3() {
    assert_eq!(
        ::std::mem::size_of::<kvm_vcpu_events__bindgen_ty_3>(),
        4usize,
        concat!("Size of: ", stringify!(kvm_vcpu_events__bindgen_ty_3))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_vcpu_events__bindgen_ty_3>(),
        1usize,
        concat!("Alignment of ", stringify!(kvm_vcpu_events__bindgen_ty_3))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_3>())).injected as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_3),
            "::",
            stringify!(injected)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_3>())).pending as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_3),
            "::",
            stringify!(pending)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_3>())).masked as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_3),
            "::",
            stringify!(masked)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_3>())).pad as *const _ as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_3),
            "::",
            stringify!(pad)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_vcpu_events__bindgen_ty_4() {
    assert_eq!(
        ::std::mem::size_of::<kvm_vcpu_events__bindgen_ty_4>(),
        4usize,
        concat!("Size of: ", stringify!(kvm_vcpu_events__bindgen_ty_4))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_vcpu_events__bindgen_ty_4>(),
        1usize,
        concat!("Alignment of ", stringify!(kvm_vcpu_events__bindgen_ty_4))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_4>())).smm as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_4),
            "::",
            stringify!(smm)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_4>())).pending as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_4),
            "::",
            stringify!(pending)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_4>())).smm_inside_nmi as *const _
                as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_4),
            "::",
            stringify!(smm_inside_nmi)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_4>())).latched_init as *const _
                as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_4),
            "::",
            stringify!(latched_init)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_vcpu_events() {
    assert_eq!(
        ::std::mem::size_of::<kvm_vcpu_events>(),
        64usize,
        concat!("Size of: ", stringify!(kvm_vcpu_events))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_vcpu_events>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_vcpu_events))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_vcpu_events>())).exception as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events),
            "::",
            stringify!(exception)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_vcpu_events>())).interrupt as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events),
            "::",
            stringify!(interrupt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_vcpu_events>())).nmi as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events),
            "::",
            stringify!(nmi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_vcpu_events>())).sipi_vector as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events),
            "::",
            stringify!(sipi_vector)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_vcpu_events>())).flags as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_vcpu_events>())).smi as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events),
            "::",
            stringify!(smi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_vcpu_events>())).reserved as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events),
            "::",
            stringify!(reserved)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events>())).exception_has_payload as *const _ as usize
        },
        55usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events),
            "::",
            stringify!(exception_has_payload)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events>())).exception_payload as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events),
            "::",
            stringify!(exception_payload)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_debugregs() {
    assert_eq!(
        ::std::mem::size_of::<kvm_debugregs>(),
        128usize,
        concat!("Size of: ", stringify!(kvm_debugregs))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_debugregs>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_debugregs))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_debugregs>())).db as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_debugregs),
            "::",
            stringify!(db)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_debugregs>())).dr6 as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_debugregs),
            "::",
            stringify!(dr6)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_debugregs>())).dr7 as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_debugregs),
            "::",
            stringify!(dr7)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_debugregs>())).flags as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_debugregs),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_debugregs>())).reserved as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_debugregs),
            "::",
            stringify!(reserved)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_xsave() {
    assert_eq!(
        ::std::mem::size_of::<kvm_xsave>(),
        4096usize,
        concat!("Size of: ", stringify!(kvm_xsave))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_xsave>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_xsave))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xsave>())).region as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xsave),
            "::",
            stringify!(region)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_xcr() {
    assert_eq!(
        ::std::mem::size_of::<kvm_xcr>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_xcr))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_xcr>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_xcr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xcr>())).xcr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xcr),
            "::",
            stringify!(xcr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xcr>())).reserved as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xcr),
            "::",
            stringify!(reserved)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xcr>())).value as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xcr),
            "::",
            stringify!(value)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_xcrs() {
    assert_eq!(
        ::std::mem::size_of::<kvm_xcrs>(),
        392usize,
        concat!("Size of: ", stringify!(kvm_xcrs))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_xcrs>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_xcrs))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xcrs>())).nr_xcrs as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xcrs),
            "::",
            stringify!(nr_xcrs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xcrs>())).flags as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xcrs),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xcrs>())).xcrs as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xcrs),
            "::",
            stringify!(xcrs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xcrs>())).padding as *const _ as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xcrs),
            "::",
            stringify!(padding)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_irqchip__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_irqchip__bindgen_ty_1>(),
        512usize,
        concat!("Size of: ", stringify!(kvm_irqchip__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_irqchip__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_irqchip__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irqchip__bindgen_ty_1>())).dummy as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irqchip__bindgen_ty_1),
            "::",
            stringify!(dummy)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irqchip__bindgen_ty_1>())).pic as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irqchip__bindgen_ty_1),
            "::",
            stringify!(pic)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_irqchip__bindgen_ty_1>())).ioapic as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irqchip__bindgen_ty_1),
            "::",
            stringify!(ioapic)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_irqchip() {
    assert_eq!(
        ::std::mem::size_of::<kvm_irqchip>(),
        520usize,
        concat!("Size of: ", stringify!(kvm_irqchip))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_irqchip>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_irqchip))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irqchip>())).chip_id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irqchip),
            "::",
            stringify!(chip_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irqchip>())).pad as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irqchip),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irqchip>())).chip as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irqchip),
            "::",
            stringify!(chip)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_mp_state() {
    assert_eq!(
        ::std::mem::size_of::<kvm_mp_state>(),
        4usize,
        concat!("Size of: ", stringify!(kvm_mp_state))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_mp_state>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_mp_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_mp_state>())).mp_state as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_mp_state),
            "::",
            stringify!(mp_state)
        )
    );
}

#[test]
fn bindgen_test_layout_kvm_clock_data() {
    assert_eq!(
        ::std::mem::size_of::<kvm_clock_data>(),
        48usize,
        concat!("Size of: ", stringify!(kvm_clock_data))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_clock_data>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_clock_data))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_clock_data>())).clock as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_clock_data),
            "::",
            stringify!(clock)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_clock_data>())).flags as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_clock_data),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_clock_data>())).pad as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_clock_data),
            "::",
            stringify!(pad)
        )
    );
}
