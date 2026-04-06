#![cfg_attr(not(test), no_std)]

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
    webp_core::sharpyuv::init(cpu_info_func);
}
