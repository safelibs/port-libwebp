use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

use crate::checked::checked_allocation_size;

struct LibcAllocator;

unsafe impl GlobalAlloc for LibcAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: `malloc` is called with the requested size from the allocator contract.
        unsafe { libc::malloc(layout.size()) as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        // SAFETY: `ptr` came from `alloc`/`realloc` and may be passed back to `free`.
        unsafe { libc::free(ptr.cast::<c_void>()) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: `ptr` originated from the same allocator and `new_size` is the requested size.
        unsafe { libc::realloc(ptr.cast::<c_void>(), new_size) as *mut u8 }
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: LibcAllocator = LibcAllocator;

#[inline]
pub fn webp_malloc(size: usize) -> *mut c_void {
    // SAFETY: forwarding a size_t-sized request to libc malloc is the C ABI contract.
    unsafe { libc::malloc(size) }
}

#[inline]
pub fn webp_free(ptr: *mut c_void) {
    // SAFETY: the public ABI matches libc free semantics.
    unsafe { libc::free(ptr) }
}

#[inline]
pub fn webp_safe_malloc(nmemb: u64, size: usize) -> *mut c_void {
    match checked_allocation_size(nmemb, size) {
        Some(total) => {
            // SAFETY: `total` is bounded to a valid `size_t` by `checked_allocation_size`.
            unsafe { libc::malloc(total) }
        }
        None => core::ptr::null_mut(),
    }
}

#[inline]
pub fn webp_safe_calloc(nmemb: u64, size: usize) -> *mut c_void {
    if checked_allocation_size(nmemb, size).is_none() {
        return core::ptr::null_mut();
    }
    let Ok(nmemb_size_t) = usize::try_from(nmemb) else {
        return core::ptr::null_mut();
    };
    // SAFETY: both operands were validated to fit the C ABI before calling into libc.
    unsafe { libc::calloc(nmemb_size_t, size) }
}

#[inline]
pub fn webp_safe_free(ptr: *mut c_void) {
    // SAFETY: the public ABI matches libc free semantics.
    unsafe { libc::free(ptr) }
}
