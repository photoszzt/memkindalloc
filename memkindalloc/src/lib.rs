mod ffi {
    pub use memkindalloc_sys::*;
}

use core::ptr::NonNull;
use std::ffi::c_void;

#[derive(Debug)]
pub struct FixedAlloctor {
    pub kind: ffi::memkind_t
}

#[derive(Debug)]
pub enum MemkindError {
    MemkindErrno(i32),
}

impl std::fmt::Display for MemkindError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MemkindError::MemkindErrno(errno) => {
                match errno {
                    &ffi::MEMKIND_ERROR_UNAVAILABLE => write!(f, "Memory kind is not available."),
                    &ffi::MEMKIND_ERROR_MBIND => write!(f, "Call to mbind() failed."),
                    &ffi::MEMKIND_ERROR_MMAP => write!(f, "Call to mmap() failed."),
                    &ffi::MEMKIND_ERROR_MALLOC => write!(f, "Call to malloc() failed."),
                    &ffi::MEMKIND_ERROR_ENVIRON => write!(f, "Unable to parse environment variable."),
                    &ffi::MEMKIND_ERROR_INVALID => write!(f, "Invalid argument."),
                    &ffi::MEMKIND_ERROR_TOOMANY => write!(f, "Attempt to initialize more than MEMKIND_MAX_KIND number of kinds."),
                    &ffi::MEMKIND_ERROR_BADOPS => write!(f, "Invalid memkind_ops structure."),
                    &ffi::MEMKIND_ERROR_HUGETLB => write!(f, "Unable to allocate huge pages."),
                    &ffi::MEMKIND_ERROR_MEMTYPE_NOT_AVAILABLE => write!(f, "Requested memory type is not available."),
                    &ffi::MEMKIND_ERROR_OPERATION_FAILED => write!(f, "Operation failed."),
                    &ffi::MEMKIND_ERROR_ARENAS_CREATE => write!(f, "Call to jemalloc's arenas.create failed."),
                    &ffi::MEMKIND_ERROR_RUNTIME => write!(f, "Unspecified run-time error."),
                    _ => write!(f, "Unknown error."),
                }
            }
        }
    }
}

impl std::error::Error for MemkindError {
    fn description(&self) -> &str {
        match self {
            MemkindError::MemkindErrno(errno) => {
                match errno {
                    &ffi::MEMKIND_ERROR_UNAVAILABLE => "Memory kind is not available.",
                    &ffi::MEMKIND_ERROR_MBIND => "Call to mbind() failed.",
                    &ffi::MEMKIND_ERROR_MMAP => "Call to mmap() failed.",
                    &ffi::MEMKIND_ERROR_MALLOC => "Call to malloc() failed.",
                    &ffi::MEMKIND_ERROR_ENVIRON => "Unable to parse environment variable.",
                    &ffi::MEMKIND_ERROR_INVALID => "Invalid argument.",
                    &ffi::MEMKIND_ERROR_TOOMANY => "Attempt to initialize more than MEMKIND_MAX_KIND number of kinds.",
                    &ffi::MEMKIND_ERROR_BADOPS => "Invalid memkind_ops structure.",
                    &ffi::MEMKIND_ERROR_HUGETLB => "Unable to allocate huge pages.",
                    &ffi::MEMKIND_ERROR_MEMTYPE_NOT_AVAILABLE => "Requested memory type is not available.",
                    &ffi::MEMKIND_ERROR_OPERATION_FAILED => "Operation failed.",
                    &ffi::MEMKIND_ERROR_ARENAS_CREATE => "Call to jemalloc's arenas.create failed.",
                    &ffi::MEMKIND_ERROR_RUNTIME => "Unspecified run-time error.",
                    _ => "Unknown error.",
                }
            }
        }
    }
}

impl FixedAlloctor {
    pub fn new(addr: NonNull<u8>, size: usize) -> Result<Self, MemkindError> {
        let mut kind = std::ptr::null_mut() as ffi::memkind_t;
        let ret = unsafe {ffi::memkind_create_fixed(addr.as_ptr() as *mut c_void, size, &mut kind as *mut ffi::memkind_t) };
        if ret != ffi::MEMKIND_SUCCESS {
            return Err(MemkindError::MemkindErrno(ret));
        } else {
            Ok(Self { kind })
        }
    }

    pub fn alloc(&self, size: usize) -> *mut u8 {
        unsafe {ffi::memkind_malloc(self.kind, size) as *mut u8}
    }

    pub fn free(&self, ptr: NonNull<u8>) {
        unsafe {ffi::memkind_free(self.kind, ptr.as_ptr() as *mut c_void); }
    }
}

impl Drop for FixedAlloctor {
    fn drop(&mut self) {
        unsafe {ffi::memkind_destroy_kind(self.kind); }
    }
}