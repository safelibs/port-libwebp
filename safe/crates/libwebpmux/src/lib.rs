#![no_std]

use core::ffi::c_char;
use webp_core::mux::{anim_encode, muxedit, muxinternal, muxread};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    unsafe { libc::abort() }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

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
static KEEP_WEBP_ENC: unsafe extern "C" fn(
    *const webp_core::encode::webp_enc::WebPConfig,
    *mut webp_core::encode::webp_enc::WebPPicture,
) -> i32 = webp_core::encode::webp_enc::WebPEncode;

#[used]
static KEEP_GET_MUX_VERSION: unsafe extern "C" fn() -> i32 = muxinternal::WebPGetMuxVersion;
#[used]
static KEEP_NEW_INTERNAL: unsafe extern "C" fn(i32) -> *mut muxedit::WebPMux =
    muxedit::WebPNewInternal;
#[used]
static KEEP_MUX_DELETE: unsafe extern "C" fn(*mut muxedit::WebPMux) = muxedit::WebPMuxDelete;
#[used]
static KEEP_MUX_CREATE_INTERNAL: unsafe extern "C" fn(
    *const muxread::WebPData,
    i32,
    i32,
) -> *mut muxread::WebPMux = muxread::WebPMuxCreateInternal;
#[used]
static KEEP_MUX_SET_CHUNK: unsafe extern "C" fn(
    *mut muxedit::WebPMux,
    *const c_char,
    *const muxedit::WebPData,
    i32,
) -> muxedit::WebPMuxError = muxedit::WebPMuxSetChunk;
#[used]
static KEEP_MUX_GET_CHUNK: unsafe extern "C" fn(
    *const muxread::WebPMux,
    *const c_char,
    *mut muxread::WebPData,
) -> muxread::WebPMuxError = muxread::WebPMuxGetChunk;
#[used]
static KEEP_MUX_DELETE_CHUNK: unsafe extern "C" fn(
    *mut muxedit::WebPMux,
    *const c_char,
) -> muxedit::WebPMuxError = muxedit::WebPMuxDeleteChunk;
#[used]
static KEEP_MUX_SET_IMAGE: unsafe extern "C" fn(
    *mut muxedit::WebPMux,
    *const muxedit::WebPData,
    i32,
) -> muxedit::WebPMuxError = muxedit::WebPMuxSetImage;
#[used]
static KEEP_MUX_PUSH_FRAME: unsafe extern "C" fn(
    *mut muxedit::WebPMux,
    *const muxedit::WebPMuxFrameInfo,
    i32,
) -> muxedit::WebPMuxError = muxedit::WebPMuxPushFrame;
#[used]
static KEEP_MUX_GET_FRAME: unsafe extern "C" fn(
    *const muxread::WebPMux,
    u32,
    *mut muxread::WebPMuxFrameInfo,
) -> muxread::WebPMuxError = muxread::WebPMuxGetFrame;
#[used]
static KEEP_MUX_DELETE_FRAME: unsafe extern "C" fn(
    *mut muxedit::WebPMux,
    u32,
) -> muxedit::WebPMuxError = muxedit::WebPMuxDeleteFrame;
#[used]
static KEEP_SET_ANIMATION_PARAMS: unsafe extern "C" fn(
    *mut muxedit::WebPMux,
    *const muxedit::WebPMuxAnimParams,
) -> muxedit::WebPMuxError = muxedit::WebPMuxSetAnimationParams;
#[used]
static KEEP_GET_ANIMATION_PARAMS: unsafe extern "C" fn(
    *const muxread::WebPMux,
    *mut muxread::WebPMuxAnimParams,
) -> muxread::WebPMuxError = muxread::WebPMuxGetAnimationParams;
#[used]
static KEEP_SET_CANVAS_SIZE: unsafe extern "C" fn(
    *mut muxedit::WebPMux,
    i32,
    i32,
) -> muxedit::WebPMuxError = muxedit::WebPMuxSetCanvasSize;
#[used]
static KEEP_GET_CANVAS_SIZE: unsafe extern "C" fn(
    *const muxread::WebPMux,
    *mut i32,
    *mut i32,
) -> muxread::WebPMuxError = muxread::WebPMuxGetCanvasSize;
#[used]
static KEEP_GET_FEATURES: unsafe extern "C" fn(
    *const muxread::WebPMux,
    *mut muxread::uint32_t,
) -> muxread::WebPMuxError = muxread::WebPMuxGetFeatures;
#[used]
static KEEP_NUM_CHUNKS: unsafe extern "C" fn(
    *const muxread::WebPMux,
    muxread::WebPChunkId,
    *mut i32,
) -> muxread::WebPMuxError = muxread::WebPMuxNumChunks;
#[used]
static KEEP_ASSEMBLE: unsafe extern "C" fn(
    *mut muxedit::WebPMux,
    *mut muxedit::WebPData,
) -> muxedit::WebPMuxError = muxedit::WebPMuxAssemble;
#[used]
static KEEP_ANIM_OPTIONS_INIT: unsafe extern "C" fn(*mut anim_encode::WebPAnimEncoderOptions, i32) -> i32 =
    anim_encode::WebPAnimEncoderOptionsInitInternal;
#[used]
static KEEP_ANIM_NEW_INTERNAL: unsafe extern "C" fn(
    i32,
    i32,
    *const anim_encode::WebPAnimEncoderOptions,
    i32,
) -> *mut anim_encode::WebPAnimEncoder = anim_encode::WebPAnimEncoderNewInternal;
#[used]
static KEEP_ANIM_ADD: unsafe extern "C" fn(
    *mut anim_encode::WebPAnimEncoder,
    *mut anim_encode::WebPPicture,
    i32,
    *const anim_encode::WebPConfig,
) -> i32 = anim_encode::WebPAnimEncoderAdd;
#[used]
static KEEP_ANIM_ASSEMBLE: unsafe extern "C" fn(
    *mut anim_encode::WebPAnimEncoder,
    *mut anim_encode::WebPData,
) -> i32 = anim_encode::WebPAnimEncoderAssemble;
#[used]
static KEEP_ANIM_GET_ERROR: unsafe extern "C" fn(
    *mut anim_encode::WebPAnimEncoder,
) -> *const c_char = anim_encode::WebPAnimEncoderGetError;
#[used]
static KEEP_ANIM_DELETE: unsafe extern "C" fn(*mut anim_encode::WebPAnimEncoder) =
    anim_encode::WebPAnimEncoderDelete;
