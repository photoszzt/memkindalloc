#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use core::ffi::c_int;

pub mod memkind;

/// Align the memory allocation to start at an address that is a
/// multiple of `1 << la`.
///
/// # Safety
///
/// It does not validate that `la` is within the valid range.
#[inline]
pub const fn MALLOCX_LG_ALIGN(la: usize) -> c_int {
    la as c_int
}

/// Align the memory allocation to start at an address that is a multiple of `align`,
/// where a is a power of two.
///
/// # Safety
///
/// This macro does not validate that a is a power of 2.
#[inline]
pub const fn MALLOCX_ALIGN(aling: usize) -> c_int {
    aling.trailing_zeros() as c_int
}
