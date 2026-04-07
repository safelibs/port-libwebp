#![cfg_attr(not(test), no_std)]

use core::ffi::c_void;
#[cfg(debug_assertions)]
use core::ffi::{c_float, c_int};
use webp_core::{default_cpu_info, VP8CPUInfo, WebPWorkerInterface};

#[cfg(debug_assertions)]
const VP8_RANDOM_DITHER_FIX: c_int = 8;
#[cfg(debug_assertions)]
const VP8_RANDOM_TABLE_SIZE: usize = 55;
#[cfg(debug_assertions)]
const VP8_RANDOM_TABLE: [u32; VP8_RANDOM_TABLE_SIZE] = [
    0x0de15230, 0x03b31886, 0x775faccb, 0x1c88626a, 0x68385c55, 0x14b3b828, 0x4a85fef8, 0x49ddb84b,
    0x64fcf397, 0x5c550289, 0x4a290000, 0x0d7ec1da, 0x5940b7ab, 0x5492577d, 0x4e19ca72, 0x38d38c69,
    0x0c01ee65, 0x32a1755f, 0x5437f652, 0x5abb2c32, 0x0faa57b1, 0x73f533e7, 0x685feeda, 0x7563cce2,
    0x6e990e83, 0x4730a7ed, 0x4fc0d9c6, 0x496b153c, 0x4f1403fa, 0x541afb0c, 0x73990b32, 0x26d7cb1c,
    0x6fcc3706, 0x2cbb77d8, 0x75762f2a, 0x6425ccdd, 0x24b35461, 0x0a7d8715, 0x220414a8, 0x141ebf67,
    0x56b41583, 0x73e502e3, 0x44cab16f, 0x28264d42, 0x73baaefb, 0x0a50ebed, 0x1d6ab6fb, 0x0d3ad40b,
    0x35db3b68, 0x2b081e83, 0x77ce6b95, 0x5181e5f0, 0x78853bbc, 0x009f9494, 0x27e5ed3c,
];

#[cfg(debug_assertions)]
#[repr(C)]
struct VP8Random {
    index1_: c_int,
    index2_: c_int,
    tab_: [u32; VP8_RANDOM_TABLE_SIZE],
    amp_: c_int,
}

unsafe extern "C" {
    fn WebPPlaneDistortion(
        src: *const u8,
        src_stride: usize,
        reference: *const u8,
        ref_stride: usize,
        width: i32,
        height: i32,
        x_step: usize,
        type_: i32,
        distortion: *mut f32,
        result: *mut f32,
    ) -> i32;
    fn WebPPictureDistortion(
        src: *const c_void,
        reference: *const c_void,
        metric_type: i32,
        result: *mut f32,
    ) -> i32;
}

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

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
unsafe extern "C" fn VP8InitRandom(rg: *mut VP8Random, dithering: c_float) {
    if rg.is_null() {
        return;
    }
    unsafe {
        (*rg).tab_ = VP8_RANDOM_TABLE;
        (*rg).index1_ = 0;
        (*rg).index2_ = 31;
        (*rg).amp_ = if dithering < 0.0 {
            0
        } else if dithering > 1.0 {
            1 << VP8_RANDOM_DITHER_FIX
        } else {
            ((1 << VP8_RANDOM_DITHER_FIX) as c_float * dithering) as c_int
        };
    }
}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
extern "C" fn VP8DspInitSSE2() {}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
extern "C" fn VP8FiltersInitSSE2() {}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
extern "C" fn VP8LDspInitSSE2() {}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
extern "C" fn VP8EncDspInitSSE2() {}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
extern "C" fn VP8EncDspInitSSE41() {}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
extern "C" fn VP8EncDspCostInitSSE2() {}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
extern "C" fn VP8SSIMDspInitSSE2() {}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
extern "C" fn VP8LEncDspInitSSE2() {}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
extern "C" fn VP8LEncDspInitSSE41() {}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
extern "C" fn WebPInitAlphaProcessingSSE2() {}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
extern "C" fn WebPInitConvertARGBToYUVSSE2() {}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
extern "C" fn WebPInitSamplersSSE2() {}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
extern "C" fn WebPInitUpsamplersSSE2() {}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
extern "C" fn WebPInitYUV444ConvertersSSE2() {}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
extern "C" fn WebPRescalerDspInitSSE2() {}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
pub extern "C" fn WebPSafeMalloc(nmemb: u64, size: usize) -> *mut c_void {
    webp_core::webp_safe_malloc(nmemb, size)
}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
pub extern "C" fn WebPSafeCalloc(nmemb: u64, size: usize) -> *mut c_void {
    webp_core::webp_safe_calloc(nmemb, size)
}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
pub extern "C" fn WebPSafeFree(ptr: *mut c_void) {
    webp_core::webp_safe_free(ptr);
}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
pub extern "C" fn WebPMalloc(size: usize) -> *mut c_void {
    webp_core::webp_malloc(size)
}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
pub extern "C" fn WebPFree(ptr: *mut c_void) {
    webp_core::webp_free(ptr);
}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPCopyPlane(
    src: *const u8,
    src_stride: i32,
    dst: *mut u8,
    dst_stride: i32,
    width: i32,
    height: i32,
) {
    unsafe {
        webp_core::utils::utils::WebPCopyPlane(src, src_stride, dst, dst_stride, width, height)
    }
}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPCopyPixels(
    src: *const webp_core::utils::utils::WebPPicture,
    dst: *mut webp_core::utils::utils::WebPPicture,
) {
    unsafe { webp_core::utils::utils::WebPCopyPixels(src, dst) }
}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPGetColorPalette(
    pic: *const webp_core::utils::utils::WebPPicture,
    palette: *mut u32,
) -> i32 {
    unsafe { webp_core::utils::utils::WebPGetColorPalette(pic, palette) }
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

#[used]
static KEEP_CONFIG_ENC: unsafe extern "C" fn(
    *mut webp_core::encode::config_enc::WebPConfig,
    webp_core::encode::config_enc::WebPPreset,
    f32,
    i32,
) -> i32 = webp_core::encode::config_enc::WebPConfigInitInternal;

#[used]
static KEEP_PICTURE_ENC: unsafe extern "C" fn(
    *mut webp_core::encode::picture_enc::WebPPicture,
    i32,
) -> i32 = webp_core::encode::picture_enc::WebPPictureInitInternal;

#[used]
static KEEP_PICTURE_CSP_ENC: unsafe extern "C" fn(
    *mut webp_core::encode::picture_csp_enc::WebPPicture,
    *const u8,
    i32,
) -> i32 = webp_core::encode::picture_csp_enc::WebPPictureImportRGBA;

#[used]
static KEEP_PICTURE_RESCALE_ENC: unsafe extern "C" fn(
    *const webp_core::encode::picture_rescale_enc::WebPPicture,
    i32,
    i32,
    i32,
    i32,
    *mut webp_core::encode::picture_rescale_enc::WebPPicture,
) -> i32 = webp_core::encode::picture_rescale_enc::WebPPictureView;

#[used]
static KEEP_PICTURE_TOOLS_ENC: unsafe extern "C" fn(
    *const webp_core::encode::picture_csp_enc::WebPPicture,
) -> i32 = webp_core::encode::picture_csp_enc::WebPPictureHasTransparency;

#[used]
static KEEP_PICTURE_PSNR_ENC: unsafe extern "C" fn(
    *const c_void,
    *const c_void,
    i32,
    *mut f32,
) -> i32 = WebPPictureDistortion;

#[used]
static KEEP_PLANE_PSNR_ENC: unsafe extern "C" fn(
    *const u8,
    usize,
    *const u8,
    usize,
    i32,
    i32,
    usize,
    i32,
    *mut f32,
    *mut f32,
) -> i32 = WebPPlaneDistortion;

#[used]
static KEEP_WEBP_ENC: unsafe extern "C" fn(
    *const webp_core::encode::webp_enc::WebPConfig,
    *mut webp_core::encode::webp_enc::WebPPicture,
) -> i32 = webp_core::encode::webp_enc::WebPEncode;
