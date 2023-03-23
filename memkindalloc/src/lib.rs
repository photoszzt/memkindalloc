mod ffi {
    pub use memkindalloc_sys::*;
}

use core::ptr::NonNull;
use std::ffi::c_void;

#[derive(Debug)]
pub struct FixedAlloctor {
    pub kind: ffi::memkind_t
}

impl FixedAlloctor {
    pub fn new(addr: NonNull<u8>, size: usize) -> Self {
        let mut kind = std::ptr::null_mut() as ffi::memkind_t;
        unsafe {ffi::memkind_create_fixed(addr.as_ptr() as *mut c_void, size, &mut kind as *mut ffi::memkind_t); }
        Self { kind }
    }

    pub fn alloc(&self, size: usize) -> NonNull<u8> {
        let ptr = unsafe {ffi::memkind_malloc(self.kind, size) as *mut u8};
        NonNull::new(ptr).unwrap()
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