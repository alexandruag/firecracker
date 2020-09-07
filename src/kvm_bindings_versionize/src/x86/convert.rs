use kvm_bindings as upstream;

use crate::{convert_bitwise, impl_conversions};

use super::*;

impl_conversions!(kvm_vcpu_events);
impl_conversions!(kvm_lapic_state);
impl_conversions!(kvm_debugregs);
impl_conversions!(kvm_xcrs);
impl_conversions!(kvm_xsave);
impl_conversions!(kvm_sregs);
impl_conversions!(kvm_regs);
impl_conversions!(kvm_mp_state);
impl_conversions!(kvm_cpuid2);
impl_conversions!(kvm_cpuid_entry2);
impl_conversions!(kvm_irqchip);
impl_conversions!(kvm_clock_data);
impl_conversions!(kvm_pit_state2);
impl_conversions!(kvm_msrs);
impl_conversions!(kvm_msr_entry);

#[cfg(test)]
mod tests {
    use std::mem::size_of;
    use std::slice;

    use super::*;

    fn as_byte_slice<T>(t: &T) -> &[u8] {
        let len = size_of::<T>();
        // This is actually safe because we're dealing with plain old data objects.
        unsafe { slice::from_raw_parts(t as *const T as *const u8, len) }
    }

    fn as_mut_byte_slice<T>(t: &mut T) -> &mut [u8] {
        let len = size_of::<T>();
        // This is actually safe because we're dealing with plain old data objects.
        unsafe { slice::from_raw_parts_mut(t as *mut T as *mut u8, len) }
    }

    // This should produce the same result for repeated invocations with the same type parameter
    // for `check_conversion` to work properly.
    fn init_with_bytes<T: Default>() -> T {
        let mut obj = T::default();
        let slice = as_mut_byte_slice(&mut obj);

        // Initialize the object with bytes to improve chances of detecting conversion mismatches.
        // The heterogeneity of the sequence can be improved :-s
        for (i, byte) in slice.iter_mut().enumerate() {
            *byte = i as u8;
        }

        obj
    }

    // This is a very stringent test, which checks for bitwise equality. Other variations are
    // possible if we ever want to introduce differences between proxy structs and their
    // upstream correspondents.
    fn check_conversion<T, U>()
    where
        T: Default + From<U>,
        U: Default + From<T>,
    {
        {
            let t = init_with_bytes::<T>();
            let u = U::from(init_with_bytes::<T>());
            assert_eq!(as_byte_slice(&t), as_byte_slice(&u));
        }

        {
            let u = init_with_bytes::<U>();
            let t = T::from(init_with_bytes::<U>());
            assert_eq!(as_byte_slice(&t), as_byte_slice(&u));
        }
    }

    macro_rules! test_conversion {
        ($t:ident) => {
            check_conversion::<$t, upstream::$t>();
        };
    }

    #[test]
    fn test_conversions() {
        test_conversion!(kvm_vcpu_events);
        test_conversion!(kvm_lapic_state);
        test_conversion!(kvm_debugregs);
        test_conversion!(kvm_xcrs);
        test_conversion!(kvm_xsave);
        test_conversion!(kvm_sregs);
        test_conversion!(kvm_regs);
        test_conversion!(kvm_mp_state);
        test_conversion!(kvm_cpuid2);
        test_conversion!(kvm_cpuid_entry2);
        test_conversion!(kvm_irqchip);
        test_conversion!(kvm_clock_data);
        test_conversion!(kvm_pit_state2);
        test_conversion!(kvm_msrs);
        test_conversion!(kvm_msr_entry);
    }
}
