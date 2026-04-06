mod convert;
mod csp;
mod dsp;
mod gamma;

use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::cpu::VP8CPUInfo;
use webp_abi::{
    SharpYuvColorSpace, SharpYuvConversionMatrix, SharpYuvMatrixType, SHARPYUV_VERSION,
};

static CURRENT_CPU_INFO: AtomicPtr<()> = AtomicPtr::new(ptr::null_mut());
static LAST_INITIALIZED_CPU_INFO: AtomicPtr<()> = AtomicPtr::new(ptr::null_mut());

fn cpu_info_ptr(cpu_info: VP8CPUInfo) -> *mut () {
    cpu_info
        .map(|func| func as *const () as *mut ())
        .unwrap_or(ptr::null_mut())
}

pub fn get_version() -> i32 {
    SHARPYUV_VERSION as i32
}

pub fn init(cpu_info: VP8CPUInfo) {
    if let Some(func) = cpu_info {
        CURRENT_CPU_INFO.store(func as *const () as *mut (), Ordering::Relaxed);
    }
    let current = cpu_info_ptr(cpu_info).wrapping_offset(0);
    let current = if current.is_null() {
        CURRENT_CPU_INFO.load(Ordering::Acquire)
    } else {
        current
    };
    if LAST_INITIALIZED_CPU_INFO.load(Ordering::Acquire) == current {
        return;
    }
    dsp::init_dsp();
    gamma::init_gamma_tables();
    LAST_INITIALIZED_CPU_INFO.store(current, Ordering::Release);
}

pub fn compute_conversion_matrix(
    yuv_color_space: *const SharpYuvColorSpace,
    matrix: *mut SharpYuvConversionMatrix,
) {
    if yuv_color_space.is_null() || matrix.is_null() {
        return;
    }
    // SAFETY: both pointers are checked for null and only borrowed for the duration of the call.
    unsafe {
        csp::compute_conversion_matrix(&*yuv_color_space, &mut *matrix);
    }
}

pub fn get_conversion_matrix(matrix_type: SharpYuvMatrixType) -> *const SharpYuvConversionMatrix {
    csp::get_conversion_matrix(matrix_type)
}

#[allow(clippy::too_many_arguments)]
pub fn convert(
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
    init(None);
    if yuv_matrix.is_null() {
        return 0;
    }
    // SAFETY: the matrix pointer is validated above and borrowed immutably for the conversion.
    let yuv_matrix = unsafe { &*yuv_matrix };
    convert::sharp_yuv_convert(
        r_ptr.cast(),
        g_ptr.cast(),
        b_ptr.cast(),
        rgb_step,
        rgb_stride,
        rgb_bit_depth,
        y_ptr.cast(),
        y_stride,
        u_ptr.cast(),
        u_stride,
        v_ptr.cast(),
        v_stride,
        yuv_bit_depth,
        width,
        height,
        yuv_matrix,
    ) as i32
}
