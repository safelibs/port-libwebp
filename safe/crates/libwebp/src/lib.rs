#![no_std]

use core::ffi::c_void;
use webp_core::{default_cpu_info, VP8CPUInfo, WebPWorkerInterface};

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

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    // SAFETY: aborting is the only valid panic strategy for these no_std cdylibs.
    unsafe { libc::abort() }
}

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
static KEEP_PICTURE_PSNR_ENC: unsafe extern "C" fn(*const c_void, *const c_void, i32, *mut f32) -> i32 =
    WebPPictureDistortion;

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
