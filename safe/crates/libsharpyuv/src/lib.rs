#![no_std]

use webp_abi::{SharpYuvColorSpace, SharpYuvConversionMatrix, SharpYuvMatrixType};
use webp_core::VP8CPUInfo;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    // SAFETY: aborting is the only valid panic strategy for these no_std cdylibs.
    unsafe { libc::abort() }
}

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
