#![no_std]

use core::ffi::c_void;

use webp_core::{default_cpu_info, VP8CPUInfo, WebPWorkerInterface};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    // SAFETY: aborting is the only valid panic strategy for these no_std cdylibs.
    unsafe { libc::abort() }
}

#[unsafe(no_mangle)]
pub extern "C" fn WebPMalloc(size: usize) -> *mut c_void {
    webp_core::webp_malloc(size)
}

#[unsafe(no_mangle)]
pub extern "C" fn WebPFree(ptr: *mut c_void) {
    webp_core::webp_free(ptr);
}

#[unsafe(no_mangle)]
pub extern "C" fn WebPSafeMalloc(nmemb: u64, size: usize) -> *mut c_void {
    webp_core::webp_safe_malloc(nmemb, size)
}

#[unsafe(no_mangle)]
pub extern "C" fn WebPSafeCalloc(nmemb: u64, size: usize) -> *mut c_void {
    webp_core::webp_safe_calloc(nmemb, size)
}

#[unsafe(no_mangle)]
pub extern "C" fn WebPSafeFree(ptr: *mut c_void) {
    webp_core::webp_safe_free(ptr);
}

#[unsafe(no_mangle)]
pub extern "C" fn WebPSetWorkerInterface(winterface: *const WebPWorkerInterface) -> i32 {
    webp_core::webp_set_worker_interface(winterface)
}

#[unsafe(no_mangle)]
pub extern "C" fn WebPGetWorkerInterface() -> *const WebPWorkerInterface {
    webp_core::webp_get_worker_interface()
}

#[unsafe(no_mangle)]
pub static mut VP8GetCPUInfo: VP8CPUInfo = Some(default_cpu_info);
