#![cfg_attr(not(test), no_std)]

use core::hint::spin_loop;
use core::ptr;
use core::sync::atomic::{AtomicBool, AtomicPtr, Ordering};

use webp_abi::{SharpYuvColorSpace, SharpYuvConversionMatrix, SharpYuvMatrixType};
use webp_core::{default_cpu_info, VP8CPUInfo, WebPWorkerInterface};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    // SAFETY: aborting is the only valid panic strategy for these no_std cdylibs.
    unsafe { libc::abort() }
}

#[cfg(not(test))]
#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

// Mirror upstream's LOCK_ACCESS around the hidden CPU-info/init state.
static SHARPYUV_CPU_INFO_LOCK: AtomicBool = AtomicBool::new(false);
static SHARPYUV_CPU_INFO: AtomicPtr<()> = AtomicPtr::new(default_cpu_info as *const () as *mut ());

struct CpuInfoLockGuard;

impl Drop for CpuInfoLockGuard {
    fn drop(&mut self) {
        SHARPYUV_CPU_INFO_LOCK.store(false, Ordering::Release);
    }
}

#[inline]
fn lock_cpu_info() -> CpuInfoLockGuard {
    while SHARPYUV_CPU_INFO_LOCK
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_err()
    {
        spin_loop();
    }
    CpuInfoLockGuard
}

#[inline]
fn cpu_info_ptr(cpu_info: VP8CPUInfo) -> *mut () {
    cpu_info
        .map(|func| func as *const () as *mut ())
        .unwrap_or(ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn WebPSetWorkerInterface(winterface: *const WebPWorkerInterface) -> i32 {
    webp_core::webp_set_worker_interface(winterface)
}

#[unsafe(no_mangle)]
pub extern "C" fn WebPGetWorkerInterface() -> *const WebPWorkerInterface {
    webp_core::webp_get_worker_interface()
}

// Keep this hidden symbol available for linked webp-core objects that still
// reference the traditional runtime-global CPU callback.
#[unsafe(no_mangle)]
pub static mut VP8GetCPUInfo: VP8CPUInfo = Some(default_cpu_info);

#[unsafe(no_mangle)]
pub extern "C" fn SharpYuvGetVersion() -> i32 {
    webp_core::sharpyuv::get_version()
}

#[unsafe(no_mangle)]
pub extern "C" fn SharpYuvComputeConversionMatrix(
    yuv_color_space: *const SharpYuvColorSpace,
    matrix: *mut SharpYuvConversionMatrix,
) {
    webp_core::sharpyuv::compute_conversion_matrix(yuv_color_space, matrix);
}

#[unsafe(no_mangle)]
pub extern "C" fn SharpYuvGetConversionMatrix(
    matrix_type: SharpYuvMatrixType,
) -> *const SharpYuvConversionMatrix {
    webp_core::sharpyuv::get_conversion_matrix(matrix_type)
}

#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub extern "C" fn SharpYuvConvert(
    r_ptr: *const core::ffi::c_void,
    g_ptr: *const core::ffi::c_void,
    b_ptr: *const core::ffi::c_void,
    rgb_step: i32,
    rgb_stride: i32,
    rgb_bit_depth: i32,
    y_ptr: *mut core::ffi::c_void,
    y_stride: i32,
    u_ptr: *mut core::ffi::c_void,
    u_stride: i32,
    v_ptr: *mut core::ffi::c_void,
    v_stride: i32,
    yuv_bit_depth: i32,
    width: i32,
    height: i32,
    yuv_matrix: *const SharpYuvConversionMatrix,
) -> i32 {
    {
        let _guard = lock_cpu_info();
        webp_core::sharpyuv::init_from_cpu_info_ptr(SHARPYUV_CPU_INFO.load(Ordering::Acquire));
    }
    webp_core::sharpyuv::convert(
        r_ptr,
        g_ptr,
        b_ptr,
        rgb_step,
        rgb_stride,
        rgb_bit_depth,
        y_ptr,
        y_stride,
        u_ptr,
        u_stride,
        v_ptr,
        v_stride,
        yuv_bit_depth,
        width,
        height,
        yuv_matrix,
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn SharpYuvInit(cpu_info_func: VP8CPUInfo) {
    let current = cpu_info_ptr(cpu_info_func);
    let _guard = lock_cpu_info();
    SHARPYUV_CPU_INFO.store(current, Ordering::Release);
    webp_core::sharpyuv::init_from_cpu_info_ptr(current);
}
