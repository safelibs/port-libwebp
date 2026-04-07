#![cfg_attr(not(test), no_std)]

use core::ffi::c_void;
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
pub extern "C" fn log10(_value: f64) -> f64 {
    0.0
}

#[unsafe(no_mangle)]
pub extern "C" fn log(_value: f64) -> f64 {
    0.0
}

#[unsafe(no_mangle)]
pub extern "C" fn pow(_value: f64, _power: f64) -> f64 {
    0.0
}

#[unsafe(no_mangle)]
pub extern "C" fn SharpYuvInit(_cpu_info_func: VP8CPUInfo) {}

#[unsafe(no_mangle)]
pub extern "C" fn SharpYuvGetConversionMatrix(_matrix_type: u32) -> *const c_void {
    core::ptr::null()
}

#[unsafe(no_mangle)]
pub extern "C" fn SharpYuvConvert(
    _r_ptr: *const c_void,
    _g_ptr: *const c_void,
    _b_ptr: *const c_void,
    _rgb_step: i32,
    _rgb_stride: i32,
    _rgb_bit_depth: i32,
    _y_ptr: *mut c_void,
    _y_stride: i32,
    _u_ptr: *mut c_void,
    _u_stride: i32,
    _v_ptr: *mut c_void,
    _v_stride: i32,
    _yuv_bit_depth: i32,
    _width: i32,
    _height: i32,
    _yuv_matrix: *const c_void,
) -> i32 {
    0
}

#[used]
static KEEP_BUFFER_DEC: unsafe extern "C" fn(
    *mut webp_core::decode::buffer_dec::WebPDecBuffer,
    i32,
) -> i32 = webp_core::decode::buffer_dec::WebPInitDecBufferInternal;

#[used]
static KEEP_IDEC_DEC: unsafe extern "C" fn(
    *mut webp_core::decode::idec_dec::WebPDecBuffer,
) -> *mut webp_core::decode::idec_dec::WebPIDecoder = webp_core::decode::idec_dec::WebPINewDecoder;

#[used]
static KEEP_VP8_DEC: unsafe extern "C" fn(*const u8, usize) -> i32 =
    webp_core::decode::vp8_dec::VP8CheckSignature;

#[used]
static KEEP_VP8L_DEC: unsafe extern "C" fn(*const u8, usize) -> i32 =
    webp_core::decode::vp8l_dec::VP8LCheckSignature;

#[used]
static KEEP_WEBP_DEC: unsafe extern "C" fn(
    *const u8,
    usize,
    *mut webp_core::decode::webp_dec::WebPDecoderConfig,
) -> webp_core::decode::webp_dec::VP8StatusCode = webp_core::decode::webp_dec::WebPDecode;
