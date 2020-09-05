// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate vmm_sys_util;

extern crate kvm_bindings;
extern crate versionize;
extern crate versionize_derive;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use self::x86::*;

use std::mem::{align_of, size_of};
use std::ptr;

use vmm_sys_util::fam::{FamStruct, FamStructWrapper};

fn convert_bitwise<T, U>(t: &T) -> U {
    assert_eq!(align_of::<T>(), align_of::<U>());
    assert_eq!(size_of::<T>(), size_of::<U>());
    // Safe because `src` is aligned, the two types have the same size and alignment,
    // and they are both plain old data structs.
    unsafe { ptr::read(t as *const T as *const U) }
}

// This requires `upstream` to be a valid module path/alias in the scope that's invoking the
// macro (i.e. `use kvm_bindings as upstream;`). It also requires the `convert_bitwise`
// function above to be in scope.
#[macro_export]
macro_rules! impl_conversions {
    ($T:ident) => {
        impl From<upstream::$T> for $T {
            fn from(other: upstream::$T) -> Self {
                convert_bitwise(&other)
            }
        }

        impl From<$T> for upstream::$T {
            fn from(other: $T) -> Self {
                convert_bitwise(&other)
            }
        }

        impl From<&upstream::$T> for $T {
            fn from(other: &upstream::$T) -> Self {
                convert_bitwise(other)
            }
        }

        impl From<&$T> for upstream::$T {
            fn from(other: &$T) -> Self {
                convert_bitwise(other)
            }
        }
    };
}

/// Helper function for FamStructWrapper conversion. Can/should be replaced by implementing
/// `From` for `FamStructWrapper` in `vmm-sys-util`.
pub fn convert_fam_struct_wrapper<'a, T, U>(src: &'a FamStructWrapper<T>) -> FamStructWrapper<U>
where
    T: Default + FamStruct + 'static,
    U: Default + FamStruct + From<&'a T> + 'static,
    T::Entry: 'static,
    U::Entry: From<&'a T::Entry> + 'static,
{
    // The `FamStructWrapper.len()` method is private for some reason.
    let mut dst = FamStructWrapper::new(src.as_slice().len());
    *dst.as_mut_fam_struct() = src.as_fam_struct_ref().into();

    for (index, entry) in src.as_slice().iter().enumerate() {
        dst.as_mut_slice()[index] = entry.into();
    }

    dst
}
