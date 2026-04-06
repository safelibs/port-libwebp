#![no_std]

use core::ffi::c_char;
use webp_abi::{
    WebPAnimEncoder, WebPAnimEncoderOptions, WebPConfig, WebPData, WebPMux, WebPMuxAnimParams,
    WebPMuxError, WebPMuxFrameInfo,
};

unsafe extern "C" {
    fn WebPGetMuxVersion() -> i32;
    fn WebPNewInternal(version: i32) -> *mut WebPMux;
    fn WebPMuxDelete(mux: *mut WebPMux);
    fn WebPMuxCreateInternal(
        bitstream: *const WebPData,
        copy_data: i32,
        version: i32,
    ) -> *mut WebPMux;
    fn WebPMuxSetChunk(
        mux: *mut WebPMux,
        fourcc: *const c_char,
        chunk_data: *const WebPData,
        copy_data: i32,
    ) -> WebPMuxError;
    fn WebPMuxGetChunk(
        mux: *const WebPMux,
        fourcc: *const c_char,
        chunk_data: *mut WebPData,
    ) -> WebPMuxError;
    fn WebPMuxDeleteChunk(mux: *mut WebPMux, fourcc: *const c_char) -> WebPMuxError;
    fn WebPMuxSetImage(
        mux: *mut WebPMux,
        bitstream: *const WebPData,
        copy_data: i32,
    ) -> WebPMuxError;
    fn WebPMuxPushFrame(
        mux: *mut WebPMux,
        frame: *const WebPMuxFrameInfo,
        copy_data: i32,
    ) -> WebPMuxError;
    fn WebPMuxGetFrame(mux: *const WebPMux, nth: u32, frame: *mut WebPMuxFrameInfo)
        -> WebPMuxError;
    fn WebPMuxDeleteFrame(mux: *mut WebPMux, nth: u32) -> WebPMuxError;
    fn WebPMuxSetAnimationParams(
        mux: *mut WebPMux,
        params: *const WebPMuxAnimParams,
    ) -> WebPMuxError;
    fn WebPMuxGetAnimationParams(
        mux: *const WebPMux,
        params: *mut WebPMuxAnimParams,
    ) -> WebPMuxError;
    fn WebPMuxSetCanvasSize(mux: *mut WebPMux, width: i32, height: i32) -> WebPMuxError;
    fn WebPMuxGetCanvasSize(mux: *const WebPMux, width: *mut i32, height: *mut i32)
        -> WebPMuxError;
    fn WebPMuxGetFeatures(mux: *const WebPMux, flags: *mut u32) -> WebPMuxError;
    fn WebPMuxNumChunks(
        mux: *const WebPMux,
        id: webp_abi::WebPChunkId,
        num: *mut i32,
    ) -> WebPMuxError;
    fn WebPMuxAssemble(mux: *mut WebPMux, assembled_data: *mut WebPData) -> WebPMuxError;
    fn WebPAnimEncoderOptionsInitInternal(
        enc_options: *mut WebPAnimEncoderOptions,
        abi_version: i32,
    ) -> i32;
    fn WebPAnimEncoderNewInternal(
        width: i32,
        height: i32,
        enc_options: *const WebPAnimEncoderOptions,
        abi_version: i32,
    ) -> *mut WebPAnimEncoder;
    fn WebPAnimEncoderAdd(
        enc: *mut WebPAnimEncoder,
        frame: *mut webp_abi::WebPPicture,
        timestamp_ms: i32,
        config: *const WebPConfig,
    ) -> i32;
    fn WebPAnimEncoderAssemble(enc: *mut WebPAnimEncoder, webp_data: *mut WebPData) -> i32;
    fn WebPAnimEncoderGetError(enc: *mut WebPAnimEncoder) -> *const c_char;
    fn WebPAnimEncoderDelete(enc: *mut WebPAnimEncoder);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    unsafe { libc::abort() }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

#[used]
static KEEP_GET_MUX_VERSION: unsafe extern "C" fn() -> i32 = WebPGetMuxVersion;
#[used]
static KEEP_NEW_INTERNAL: unsafe extern "C" fn(i32) -> *mut WebPMux = WebPNewInternal;
#[used]
static KEEP_MUX_DELETE: unsafe extern "C" fn(*mut WebPMux) = WebPMuxDelete;
#[used]
static KEEP_MUX_CREATE_INTERNAL: unsafe extern "C" fn(*const WebPData, i32, i32) -> *mut WebPMux =
    WebPMuxCreateInternal;
#[used]
static KEEP_MUX_SET_CHUNK: unsafe extern "C" fn(
    *mut WebPMux,
    *const c_char,
    *const WebPData,
    i32,
) -> WebPMuxError = WebPMuxSetChunk;
#[used]
static KEEP_MUX_GET_CHUNK: unsafe extern "C" fn(
    *const WebPMux,
    *const c_char,
    *mut WebPData,
) -> WebPMuxError = WebPMuxGetChunk;
#[used]
static KEEP_MUX_DELETE_CHUNK: unsafe extern "C" fn(*mut WebPMux, *const c_char) -> WebPMuxError =
    WebPMuxDeleteChunk;
#[used]
static KEEP_MUX_SET_IMAGE: unsafe extern "C" fn(
    *mut WebPMux,
    *const WebPData,
    i32,
) -> WebPMuxError = WebPMuxSetImage;
#[used]
static KEEP_MUX_PUSH_FRAME: unsafe extern "C" fn(
    *mut WebPMux,
    *const WebPMuxFrameInfo,
    i32,
) -> WebPMuxError = WebPMuxPushFrame;
#[used]
static KEEP_MUX_GET_FRAME: unsafe extern "C" fn(
    *const WebPMux,
    u32,
    *mut WebPMuxFrameInfo,
) -> WebPMuxError = WebPMuxGetFrame;
#[used]
static KEEP_MUX_DELETE_FRAME: unsafe extern "C" fn(*mut WebPMux, u32) -> WebPMuxError =
    WebPMuxDeleteFrame;
#[used]
static KEEP_SET_ANIMATION_PARAMS: unsafe extern "C" fn(
    *mut WebPMux,
    *const WebPMuxAnimParams,
) -> WebPMuxError = WebPMuxSetAnimationParams;
#[used]
static KEEP_GET_ANIMATION_PARAMS: unsafe extern "C" fn(
    *const WebPMux,
    *mut WebPMuxAnimParams,
) -> WebPMuxError = WebPMuxGetAnimationParams;
#[used]
static KEEP_SET_CANVAS_SIZE: unsafe extern "C" fn(*mut WebPMux, i32, i32) -> WebPMuxError =
    WebPMuxSetCanvasSize;
#[used]
static KEEP_GET_CANVAS_SIZE: unsafe extern "C" fn(
    *const WebPMux,
    *mut i32,
    *mut i32,
) -> WebPMuxError = WebPMuxGetCanvasSize;
#[used]
static KEEP_GET_FEATURES: unsafe extern "C" fn(*const WebPMux, *mut u32) -> WebPMuxError =
    WebPMuxGetFeatures;
#[used]
static KEEP_NUM_CHUNKS: unsafe extern "C" fn(
    *const WebPMux,
    webp_abi::WebPChunkId,
    *mut i32,
) -> WebPMuxError = WebPMuxNumChunks;
#[used]
static KEEP_ASSEMBLE: unsafe extern "C" fn(*mut WebPMux, *mut WebPData) -> WebPMuxError =
    WebPMuxAssemble;
#[used]
static KEEP_ANIM_OPTIONS_INIT: unsafe extern "C" fn(*mut WebPAnimEncoderOptions, i32) -> i32 =
    WebPAnimEncoderOptionsInitInternal;
#[used]
static KEEP_ANIM_NEW_INTERNAL: unsafe extern "C" fn(
    i32,
    i32,
    *const WebPAnimEncoderOptions,
    i32,
) -> *mut WebPAnimEncoder = WebPAnimEncoderNewInternal;
#[used]
static KEEP_ANIM_ADD: unsafe extern "C" fn(
    *mut WebPAnimEncoder,
    *mut webp_abi::WebPPicture,
    i32,
    *const WebPConfig,
) -> i32 = WebPAnimEncoderAdd;
#[used]
static KEEP_ANIM_ASSEMBLE: unsafe extern "C" fn(*mut WebPAnimEncoder, *mut WebPData) -> i32 =
    WebPAnimEncoderAssemble;
#[used]
static KEEP_ANIM_GET_ERROR: unsafe extern "C" fn(*mut WebPAnimEncoder) -> *const c_char =
    WebPAnimEncoderGetError;
#[used]
static KEEP_ANIM_DELETE: unsafe extern "C" fn(*mut WebPAnimEncoder) = WebPAnimEncoderDelete;
