#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use core::ffi::c_int;

mod memkind_api;

pub use memkind_api::*;

#[doc = "Operation success."]
pub const MEMKIND_SUCCESS: c_int = 0;
#[doc = "Error: Memory kind is not available."]
pub const MEMKIND_ERROR_UNAVAILABLE: c_int = -1;
#[doc = "Error: Call to mbind() failed."]
pub const MEMKIND_ERROR_MBIND: c_int = -2;
#[doc = "Error: Call to mmap() failed."]
pub const MEMKIND_ERROR_MMAP: c_int = -3;
#[doc = "Error: Call to malloc() failed."]
pub const MEMKIND_ERROR_MALLOC: c_int = -6;
#[doc = "Error: Unable to parse environment variable."]
pub const MEMKIND_ERROR_ENVIRON: c_int = -12;
#[doc = "Error: Invalid argument."]
pub const MEMKIND_ERROR_INVALID: c_int = -13;
#[doc = "Error: Attempt to initialize more than MEMKIND_MAX_KIND number of kinds."]
pub const MEMKIND_ERROR_TOOMANY: c_int = -15;
#[doc = "Error: Invalid memkind_ops structure."]
pub const MEMKIND_ERROR_BADOPS: c_int = -17;
#[doc = "Error: Unable to allocate huge pages."]
pub const MEMKIND_ERROR_HUGETLB: c_int = -18;
#[doc = "Error: Requested memory type is not available."]
pub const MEMKIND_ERROR_MEMTYPE_NOT_AVAILABLE: c_int = -20;
#[doc = "Error: Operation failed."]
pub const MEMKIND_ERROR_OPERATION_FAILED: c_int = -21;
#[doc = "Error: Call to jemalloc's arenas.create failed."]
pub const MEMKIND_ERROR_ARENAS_CREATE: c_int = -22;

#[doc = "Error: Unspecified run-time error."]
pub const MEMKIND_ERROR_RUNTIME: c_int = -255;

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
