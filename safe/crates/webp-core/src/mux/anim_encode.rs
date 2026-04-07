#[repr(C)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct _IO_marker {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct WebPMux {
    _unused: [u8; 0],
}
extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn WebPMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPFree(ptr: *mut ::core::ffi::c_void);
    fn WebPNewInternal(_: ::core::ffi::c_int) -> *mut WebPMux;
    fn WebPMuxDelete(mux: *mut WebPMux);
    fn WebPMuxCreateInternal(
        _: *const WebPData,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> *mut WebPMux;
    fn WebPMuxSetImage(
        mux: *mut WebPMux,
        bitstream: *const WebPData,
        copy_data: ::core::ffi::c_int,
    ) -> WebPMuxError;
    fn WebPMuxPushFrame(
        mux: *mut WebPMux,
        frame: *const WebPMuxFrameInfo,
        copy_data: ::core::ffi::c_int,
    ) -> WebPMuxError;
    fn WebPMuxGetFrame(
        mux: *const WebPMux,
        nth: uint32_t,
        frame: *mut WebPMuxFrameInfo,
    ) -> WebPMuxError;
    fn WebPMuxSetAnimationParams(
        mux: *mut WebPMux,
        params: *const WebPMuxAnimParams,
    ) -> WebPMuxError;
    fn WebPMuxSetCanvasSize(
        mux: *mut WebPMux,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> WebPMuxError;
    fn WebPMuxGetCanvasSize(
        mux: *const WebPMux,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
    ) -> WebPMuxError;
    fn WebPMuxAssemble(mux: *mut WebPMux, assembled_data: *mut WebPData) -> WebPMuxError;
    fn WebPConfigInitInternal(
        _: *mut WebPConfig,
        _: WebPPreset,
        _: ::core::ffi::c_float,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPValidateConfig(config: *const WebPConfig) -> ::core::ffi::c_int;
    fn WebPMemoryWriterInit(writer: *mut WebPMemoryWriter);
    fn WebPMemoryWriterClear(writer: *mut WebPMemoryWriter);
    fn WebPMemoryWrite(
        data: *const uint8_t,
        data_size: size_t,
        picture: *const WebPPicture,
    ) -> ::core::ffi::c_int;
    fn WebPPictureInitInternal(_: *mut WebPPicture, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn WebPPictureAlloc(picture: *mut WebPPicture) -> ::core::ffi::c_int;
    fn WebPPictureFree(picture: *mut WebPPicture);
    fn WebPPictureCopy(src: *const WebPPicture, dst: *mut WebPPicture) -> ::core::ffi::c_int;
    fn WebPPictureView(
        src: *const WebPPicture,
        left: ::core::ffi::c_int,
        top: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        dst: *mut WebPPicture,
    ) -> ::core::ffi::c_int;
    fn WebPPictureYUVAToARGB(picture: *mut WebPPicture) -> ::core::ffi::c_int;
    fn WebPEncode(config: *const WebPConfig, picture: *mut WebPPicture) -> ::core::ffi::c_int;
    fn WebPGetFeaturesInternal(
        _: *const uint8_t,
        _: size_t,
        _: *mut WebPBitstreamFeatures,
        _: ::core::ffi::c_int,
    ) -> VP8StatusCode;
    fn WebPInitDecoderConfigInternal(
        _: *mut WebPDecoderConfig,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPDecode(
        data: *const uint8_t,
        data_size: size_t,
        config: *mut WebPDecoderConfig,
    ) -> VP8StatusCode;
    fn WebPSafeCalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
    fn WebPCopyPixels(src: *const WebPPicture, dst: *mut WebPPicture);
    fn WebPGetColorPalette(pic: *const WebPPicture, palette: *mut uint32_t) -> ::core::ffi::c_int;
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPData {
    pub bytes: *const uint8_t,
    pub size: size_t,
}
pub type WebPMuxAnimDispose = ::core::ffi::c_uint;
pub const WEBP_MUX_DISPOSE_BACKGROUND: WebPMuxAnimDispose = 1;
pub const WEBP_MUX_DISPOSE_NONE: WebPMuxAnimDispose = 0;
pub type WebPMuxAnimBlend = ::core::ffi::c_uint;
pub const WEBP_MUX_NO_BLEND: WebPMuxAnimBlend = 1;
pub const WEBP_MUX_BLEND: WebPMuxAnimBlend = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPMuxFrameInfo {
    pub bitstream: WebPData,
    pub x_offset: ::core::ffi::c_int,
    pub y_offset: ::core::ffi::c_int,
    pub duration: ::core::ffi::c_int,
    pub id: WebPChunkId,
    pub dispose_method: WebPMuxAnimDispose,
    pub blend_method: WebPMuxAnimBlend,
    pub pad: [uint32_t; 1],
}
pub type WebPChunkId = ::core::ffi::c_uint;
pub const WEBP_CHUNK_NIL: WebPChunkId = 10;
pub const WEBP_CHUNK_UNKNOWN: WebPChunkId = 9;
pub const WEBP_CHUNK_XMP: WebPChunkId = 8;
pub const WEBP_CHUNK_EXIF: WebPChunkId = 7;
pub const WEBP_CHUNK_IMAGE: WebPChunkId = 6;
pub const WEBP_CHUNK_ALPHA: WebPChunkId = 5;
pub const WEBP_CHUNK_DEPRECATED: WebPChunkId = 4;
pub const WEBP_CHUNK_ANMF: WebPChunkId = 3;
pub const WEBP_CHUNK_ANIM: WebPChunkId = 2;
pub const WEBP_CHUNK_ICCP: WebPChunkId = 1;
pub const WEBP_CHUNK_VP8X: WebPChunkId = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPMuxAnimParams {
    pub bgcolor: uint32_t,
    pub loop_count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPAnimEncoderOptions {
    pub anim_params: WebPMuxAnimParams,
    pub minimize_size: ::core::ffi::c_int,
    pub kmin: ::core::ffi::c_int,
    pub kmax: ::core::ffi::c_int,
    pub allow_mixed: ::core::ffi::c_int,
    pub verbose: ::core::ffi::c_int,
    pub padding: [uint32_t; 4],
}
pub type WebPMuxError = ::core::ffi::c_int;
pub const WEBP_MUX_NOT_ENOUGH_DATA: WebPMuxError = -4;
pub const WEBP_MUX_MEMORY_ERROR: WebPMuxError = -3;
pub const WEBP_MUX_BAD_DATA: WebPMuxError = -2;
pub const WEBP_MUX_INVALID_ARGUMENT: WebPMuxError = -1;
pub const WEBP_MUX_NOT_FOUND: WebPMuxError = 0;
pub const WEBP_MUX_OK: WebPMuxError = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPAnimEncoder {
    pub canvas_width_: ::core::ffi::c_int,
    pub canvas_height_: ::core::ffi::c_int,
    pub options_: WebPAnimEncoderOptions,
    pub prev_rect_: FrameRectangle,
    pub last_config_: WebPConfig,
    pub last_config_reversed_: WebPConfig,
    pub curr_canvas_: *mut WebPPicture,
    pub curr_canvas_copy_: WebPPicture,
    pub curr_canvas_copy_modified_: ::core::ffi::c_int,
    pub prev_canvas_: WebPPicture,
    pub prev_canvas_disposed_: WebPPicture,
    pub encoded_frames_: *mut EncodedFrame,
    pub size_: size_t,
    pub start_: size_t,
    pub count_: size_t,
    pub flush_count_: size_t,
    pub best_delta_: int64_t,
    pub keyframe_: ::core::ffi::c_int,
    pub count_since_key_frame_: ::core::ffi::c_int,
    pub first_timestamp_: ::core::ffi::c_int,
    pub prev_timestamp_: ::core::ffi::c_int,
    pub prev_candidate_undecided_: ::core::ffi::c_int,
    pub is_first_frame_: ::core::ffi::c_int,
    pub got_null_frame_: ::core::ffi::c_int,
    pub in_frame_count_: size_t,
    pub out_frame_count_: size_t,
    pub mux_: *mut WebPMux,
    pub error_str_: [::core::ffi::c_char; 100],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EncodedFrame {
    pub sub_frame_: WebPMuxFrameInfo,
    pub key_frame_: WebPMuxFrameInfo,
    pub is_key_frame_: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPPicture {
    pub use_argb: ::core::ffi::c_int,
    pub colorspace: WebPEncCSP,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub y: *mut uint8_t,
    pub u: *mut uint8_t,
    pub v: *mut uint8_t,
    pub y_stride: ::core::ffi::c_int,
    pub uv_stride: ::core::ffi::c_int,
    pub a: *mut uint8_t,
    pub a_stride: ::core::ffi::c_int,
    pub pad1: [uint32_t; 2],
    pub argb: *mut uint32_t,
    pub argb_stride: ::core::ffi::c_int,
    pub pad2: [uint32_t; 3],
    pub writer: WebPWriterFunction,
    pub custom_ptr: *mut ::core::ffi::c_void,
    pub extra_info_type: ::core::ffi::c_int,
    pub extra_info: *mut uint8_t,
    pub stats: *mut WebPAuxStats,
    pub error_code: WebPEncodingError,
    pub progress_hook: WebPProgressHook,
    pub user_data: *mut ::core::ffi::c_void,
    pub pad3: [uint32_t; 3],
    pub pad4: *mut uint8_t,
    pub pad5: *mut uint8_t,
    pub pad6: [uint32_t; 8],
    pub memory_: *mut ::core::ffi::c_void,
    pub memory_argb_: *mut ::core::ffi::c_void,
    pub pad7: [*mut ::core::ffi::c_void; 2],
}
pub type WebPProgressHook =
    Option<unsafe extern "C" fn(::core::ffi::c_int, *const WebPPicture) -> ::core::ffi::c_int>;
pub type WebPEncodingError = ::core::ffi::c_uint;
pub const VP8_ENC_ERROR_LAST: WebPEncodingError = 11;
pub const VP8_ENC_ERROR_USER_ABORT: WebPEncodingError = 10;
pub const VP8_ENC_ERROR_FILE_TOO_BIG: WebPEncodingError = 9;
pub const VP8_ENC_ERROR_BAD_WRITE: WebPEncodingError = 8;
pub const VP8_ENC_ERROR_PARTITION_OVERFLOW: WebPEncodingError = 7;
pub const VP8_ENC_ERROR_PARTITION0_OVERFLOW: WebPEncodingError = 6;
pub const VP8_ENC_ERROR_BAD_DIMENSION: WebPEncodingError = 5;
pub const VP8_ENC_ERROR_INVALID_CONFIGURATION: WebPEncodingError = 4;
pub const VP8_ENC_ERROR_NULL_PARAMETER: WebPEncodingError = 3;
pub const VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY: WebPEncodingError = 2;
pub const VP8_ENC_ERROR_OUT_OF_MEMORY: WebPEncodingError = 1;
pub const VP8_ENC_OK: WebPEncodingError = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPAuxStats {
    pub coded_size: ::core::ffi::c_int,
    pub PSNR: [::core::ffi::c_float; 5],
    pub block_count: [::core::ffi::c_int; 3],
    pub header_bytes: [::core::ffi::c_int; 2],
    pub residual_bytes: [[::core::ffi::c_int; 4]; 3],
    pub segment_size: [::core::ffi::c_int; 4],
    pub segment_quant: [::core::ffi::c_int; 4],
    pub segment_level: [::core::ffi::c_int; 4],
    pub alpha_data_size: ::core::ffi::c_int,
    pub layer_data_size: ::core::ffi::c_int,
    pub lossless_features: uint32_t,
    pub histogram_bits: ::core::ffi::c_int,
    pub transform_bits: ::core::ffi::c_int,
    pub cache_bits: ::core::ffi::c_int,
    pub palette_size: ::core::ffi::c_int,
    pub lossless_size: ::core::ffi::c_int,
    pub lossless_hdr_size: ::core::ffi::c_int,
    pub lossless_data_size: ::core::ffi::c_int,
    pub pad: [uint32_t; 2],
}
pub type WebPWriterFunction =
    Option<unsafe extern "C" fn(*const uint8_t, size_t, *const WebPPicture) -> ::core::ffi::c_int>;
pub type WebPEncCSP = ::core::ffi::c_uint;
pub const WEBP_CSP_ALPHA_BIT: WebPEncCSP = 4;
pub const WEBP_CSP_UV_MASK: WebPEncCSP = 3;
pub const WEBP_YUV420A: WebPEncCSP = 4;
pub const WEBP_YUV420: WebPEncCSP = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPConfig {
    pub lossless: ::core::ffi::c_int,
    pub quality: ::core::ffi::c_float,
    pub method: ::core::ffi::c_int,
    pub image_hint: WebPImageHint,
    pub target_size: ::core::ffi::c_int,
    pub target_PSNR: ::core::ffi::c_float,
    pub segments: ::core::ffi::c_int,
    pub sns_strength: ::core::ffi::c_int,
    pub filter_strength: ::core::ffi::c_int,
    pub filter_sharpness: ::core::ffi::c_int,
    pub filter_type: ::core::ffi::c_int,
    pub autofilter: ::core::ffi::c_int,
    pub alpha_compression: ::core::ffi::c_int,
    pub alpha_filtering: ::core::ffi::c_int,
    pub alpha_quality: ::core::ffi::c_int,
    pub pass: ::core::ffi::c_int,
    pub show_compressed: ::core::ffi::c_int,
    pub preprocessing: ::core::ffi::c_int,
    pub partitions: ::core::ffi::c_int,
    pub partition_limit: ::core::ffi::c_int,
    pub emulate_jpeg_size: ::core::ffi::c_int,
    pub thread_level: ::core::ffi::c_int,
    pub low_memory: ::core::ffi::c_int,
    pub near_lossless: ::core::ffi::c_int,
    pub exact: ::core::ffi::c_int,
    pub use_delta_palette: ::core::ffi::c_int,
    pub use_sharp_yuv: ::core::ffi::c_int,
    pub qmin: ::core::ffi::c_int,
    pub qmax: ::core::ffi::c_int,
}
pub type WebPImageHint = ::core::ffi::c_uint;
pub const WEBP_HINT_LAST: WebPImageHint = 4;
pub const WEBP_HINT_GRAPH: WebPImageHint = 3;
pub const WEBP_HINT_PHOTO: WebPImageHint = 2;
pub const WEBP_HINT_PICTURE: WebPImageHint = 1;
pub const WEBP_HINT_DEFAULT: WebPImageHint = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameRectangle {
    pub x_offset_: ::core::ffi::c_int,
    pub y_offset_: ::core::ffi::c_int,
    pub width_: ::core::ffi::c_int,
    pub height_: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubFrameParams {
    pub should_try_: ::core::ffi::c_int,
    pub empty_rect_allowed_: ::core::ffi::c_int,
    pub rect_ll_: FrameRectangle,
    pub sub_frame_ll_: WebPPicture,
    pub rect_lossy_: FrameRectangle,
    pub sub_frame_lossy_: WebPPicture,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPMemoryWriter {
    pub mem: *mut uint8_t,
    pub size: size_t,
    pub max_size: size_t,
    pub pad: [uint32_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Candidate {
    pub mem_: WebPMemoryWriter,
    pub info_: WebPMuxFrameInfo,
    pub rect_: FrameRectangle,
    pub evaluate_: ::core::ffi::c_int,
}
pub const CANDIDATE_COUNT: C2RustUnnamed_0 = 4;
pub const LOSSY_DISP_NONE: C2RustUnnamed_0 = 2;
pub const LL_DISP_NONE: C2RustUnnamed_0 = 0;
pub const LOSSY_DISP_BG: C2RustUnnamed_0 = 3;
pub const LL_DISP_BG: C2RustUnnamed_0 = 1;
pub type ComparePixelsFunc = Option<
    unsafe extern "C" fn(
        *const uint32_t,
        ::core::ffi::c_int,
        *const uint32_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type WebPPreset = ::core::ffi::c_uint;
pub const WEBP_PRESET_TEXT: WebPPreset = 5;
pub const WEBP_PRESET_ICON: WebPPreset = 4;
pub const WEBP_PRESET_DRAWING: WebPPreset = 3;
pub const WEBP_PRESET_PHOTO: WebPPreset = 2;
pub const WEBP_PRESET_PICTURE: WebPPreset = 1;
pub const WEBP_PRESET_DEFAULT: WebPPreset = 0;
pub const VP8_STATUS_OK: VP8StatusCode = 0;
pub type VP8StatusCode = ::core::ffi::c_uint;
pub const VP8_STATUS_NOT_ENOUGH_DATA: VP8StatusCode = 7;
pub const VP8_STATUS_USER_ABORT: VP8StatusCode = 6;
pub const VP8_STATUS_SUSPENDED: VP8StatusCode = 5;
pub const VP8_STATUS_UNSUPPORTED_FEATURE: VP8StatusCode = 4;
pub const VP8_STATUS_BITSTREAM_ERROR: VP8StatusCode = 3;
pub const VP8_STATUS_INVALID_PARAM: VP8StatusCode = 2;
pub const VP8_STATUS_OUT_OF_MEMORY: VP8StatusCode = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPDecoderConfig {
    pub input: WebPBitstreamFeatures,
    pub output: WebPDecBuffer,
    pub options: WebPDecoderOptions,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPDecoderOptions {
    pub bypass_filtering: ::core::ffi::c_int,
    pub no_fancy_upsampling: ::core::ffi::c_int,
    pub use_cropping: ::core::ffi::c_int,
    pub crop_left: ::core::ffi::c_int,
    pub crop_top: ::core::ffi::c_int,
    pub crop_width: ::core::ffi::c_int,
    pub crop_height: ::core::ffi::c_int,
    pub use_scaling: ::core::ffi::c_int,
    pub scaled_width: ::core::ffi::c_int,
    pub scaled_height: ::core::ffi::c_int,
    pub use_threads: ::core::ffi::c_int,
    pub dithering_strength: ::core::ffi::c_int,
    pub flip: ::core::ffi::c_int,
    pub alpha_dithering_strength: ::core::ffi::c_int,
    pub pad: [uint32_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPDecBuffer {
    pub colorspace: WEBP_CSP_MODE,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub is_external_memory: ::core::ffi::c_int,
    pub u: C2RustUnnamed,
    pub pad: [uint32_t; 4],
    pub private_memory: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub RGBA: WebPRGBABuffer,
    pub YUVA: WebPYUVABuffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPYUVABuffer {
    pub y: *mut uint8_t,
    pub u: *mut uint8_t,
    pub v: *mut uint8_t,
    pub a: *mut uint8_t,
    pub y_stride: ::core::ffi::c_int,
    pub u_stride: ::core::ffi::c_int,
    pub v_stride: ::core::ffi::c_int,
    pub a_stride: ::core::ffi::c_int,
    pub y_size: size_t,
    pub u_size: size_t,
    pub v_size: size_t,
    pub a_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPRGBABuffer {
    pub rgba: *mut uint8_t,
    pub stride: ::core::ffi::c_int,
    pub size: size_t,
}
pub type WEBP_CSP_MODE = ::core::ffi::c_uint;
pub const MODE_LAST: WEBP_CSP_MODE = 13;
pub const MODE_YUVA: WEBP_CSP_MODE = 12;
pub const MODE_YUV: WEBP_CSP_MODE = 11;
pub const MODE_rgbA_4444: WEBP_CSP_MODE = 10;
pub const MODE_Argb: WEBP_CSP_MODE = 9;
pub const MODE_bgrA: WEBP_CSP_MODE = 8;
pub const MODE_rgbA: WEBP_CSP_MODE = 7;
pub const MODE_RGB_565: WEBP_CSP_MODE = 6;
pub const MODE_RGBA_4444: WEBP_CSP_MODE = 5;
pub const MODE_ARGB: WEBP_CSP_MODE = 4;
pub const MODE_BGRA: WEBP_CSP_MODE = 3;
pub const MODE_BGR: WEBP_CSP_MODE = 2;
pub const MODE_RGBA: WEBP_CSP_MODE = 1;
pub const MODE_RGB: WEBP_CSP_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPBitstreamFeatures {
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub has_alpha: ::core::ffi::c_int,
    pub has_animation: ::core::ffi::c_int,
    pub format: ::core::ffi::c_int,
    pub pad: [uint32_t; 5],
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn WebPDataInit(mut webp_data: *mut WebPData) {
    if !webp_data.is_null() {
        memset(
            webp_data as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<WebPData>() as size_t,
        );
    }
}
#[inline]
unsafe extern "C" fn WebPDataClear(mut webp_data: *mut WebPData) {
    if !webp_data.is_null() {
        WebPFree((*webp_data).bytes as *mut ::core::ffi::c_void);
        WebPDataInit(webp_data);
    }
}
#[inline]
unsafe extern "C" fn WebPDataCopy(
    mut src: *const WebPData,
    mut dst: *mut WebPData,
) -> ::core::ffi::c_int {
    if src.is_null() || dst.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    WebPDataInit(dst);
    if !(*src).bytes.is_null() && (*src).size != 0 as size_t {
        (*dst).bytes = WebPMalloc((*src).size) as *mut uint8_t;
        if (*dst).bytes.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        memcpy(
            (*dst).bytes as *mut ::core::ffi::c_void,
            (*src).bytes as *const ::core::ffi::c_void,
            (*src).size,
        );
        (*dst).size = (*src).size;
    }
    return 1 as ::core::ffi::c_int;
}
pub const WEBP_MUX_ABI_VERSION: ::core::ffi::c_int = 0x108 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn WebPMuxNew() -> *mut WebPMux {
    return WebPNewInternal(WEBP_MUX_ABI_VERSION);
}
#[inline]
unsafe extern "C" fn WebPMuxCreate(
    mut bitstream: *const WebPData,
    mut copy_data: ::core::ffi::c_int,
) -> *mut WebPMux {
    return WebPMuxCreateInternal(bitstream, copy_data, WEBP_MUX_ABI_VERSION);
}
pub const ERROR_STR_MAX_LENGTH: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const DELTA_INFINITY: ::core::ffi::c_ulonglong =
    (1 as ::core::ffi::c_ulonglong) << 32 as ::core::ffi::c_int;
pub const KEYFRAME_NONE: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
unsafe extern "C" fn ResetCounters(enc: *mut WebPAnimEncoder) {
    (*enc).start_ = 0 as size_t;
    (*enc).count_ = 0 as size_t;
    (*enc).flush_count_ = 0 as size_t;
    (*enc).best_delta_ = DELTA_INFINITY as int64_t;
    (*enc).keyframe_ = KEYFRAME_NONE;
}
unsafe extern "C" fn DisableKeyframes(enc_options: *mut WebPAnimEncoderOptions) {
    (*enc_options).kmax = INT_MAX;
    (*enc_options).kmin = (*enc_options).kmax - 1 as ::core::ffi::c_int;
}
pub const MAX_CACHED_FRAMES: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
unsafe extern "C" fn SanitizeEncoderOptions(enc_options: *mut WebPAnimEncoderOptions) {
    let mut print_warning: ::core::ffi::c_int = (*enc_options).verbose;
    if (*enc_options).minimize_size != 0 {
        DisableKeyframes(enc_options);
    }
    if (*enc_options).kmax == 1 as ::core::ffi::c_int {
        (*enc_options).kmin = 0 as ::core::ffi::c_int;
        (*enc_options).kmax = 0 as ::core::ffi::c_int;
        return;
    } else if (*enc_options).kmax <= 0 as ::core::ffi::c_int {
        DisableKeyframes(enc_options);
        print_warning = 0 as ::core::ffi::c_int;
    }
    if (*enc_options).kmin >= (*enc_options).kmax {
        (*enc_options).kmin = (*enc_options).kmax - 1 as ::core::ffi::c_int;
        if print_warning != 0 {
            fprintf(
                stderr,
                b"WARNING: Setting kmin = %d, so that kmin < kmax.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*enc_options).kmin,
            );
        }
    } else {
        let kmin_limit: ::core::ffi::c_int =
            (*enc_options).kmax / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
        if (*enc_options).kmin < kmin_limit && kmin_limit < (*enc_options).kmax {
            (*enc_options).kmin = kmin_limit;
            if print_warning != 0 {
                fprintf(
                    stderr,
                    b"WARNING: Setting kmin = %d, so that kmin >= kmax / 2 + 1.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*enc_options).kmin,
                );
            }
        }
    }
    if (*enc_options).kmax - (*enc_options).kmin > MAX_CACHED_FRAMES {
        (*enc_options).kmin = (*enc_options).kmax - MAX_CACHED_FRAMES;
        if print_warning != 0 {
            fprintf(
                stderr,
                b"WARNING: Setting kmin = %d, so that kmax - kmin <= %d.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*enc_options).kmin,
                MAX_CACHED_FRAMES,
            );
        }
    }
    '_c2rust_label: {
        if (*enc_options).kmin < (*enc_options).kmax {
        } else {
            __assert_fail(
                b"enc_options->kmin < enc_options->kmax\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                169 as ::core::ffi::c_uint,
                b"void SanitizeEncoderOptions(WebPAnimEncoderOptions *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
}
unsafe extern "C" fn DefaultEncoderOptions(enc_options: *mut WebPAnimEncoderOptions) {
    (*enc_options).anim_params.loop_count = 0 as ::core::ffi::c_int;
    (*enc_options).anim_params.bgcolor = 0xffffffff as ::core::ffi::c_uint as uint32_t;
    (*enc_options).minimize_size = 0 as ::core::ffi::c_int;
    DisableKeyframes(enc_options);
    (*enc_options).allow_mixed = 0 as ::core::ffi::c_int;
    (*enc_options).verbose = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimEncoderOptionsInitInternal(
    mut enc_options: *mut WebPAnimEncoderOptions,
    mut abi_version: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if enc_options.is_null()
        || abi_version >> 8 as ::core::ffi::c_int
            != 0x108 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    DefaultEncoderOptions(enc_options);
    return 1 as ::core::ffi::c_int;
}
pub const TRANSPARENT_COLOR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn ClearRectangle(
    picture: *mut WebPPicture,
    mut left: ::core::ffi::c_int,
    mut top: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    let mut j: ::core::ffi::c_int = 0;
    j = top;
    while j < top + height {
        let dst: *mut uint32_t = (*picture)
            .argb
            .offset((j * (*picture).argb_stride) as isize);
        let mut i: ::core::ffi::c_int = 0;
        i = left;
        while i < left + width {
            *dst.offset(i as isize) = TRANSPARENT_COLOR as uint32_t;
            i += 1;
        }
        j += 1;
    }
}
unsafe extern "C" fn WebPUtilClearPic(picture: *mut WebPPicture, rect: *const FrameRectangle) {
    if !rect.is_null() {
        ClearRectangle(
            picture,
            (*rect).x_offset_,
            (*rect).y_offset_,
            (*rect).width_,
            (*rect).height_,
        );
    } else {
        ClearRectangle(
            picture,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            (*picture).width,
            (*picture).height,
        );
    };
}
unsafe extern "C" fn MarkNoError(enc: *mut WebPAnimEncoder) {
    (*enc).error_str_[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
}
unsafe extern "C" fn MarkError(enc: *mut WebPAnimEncoder, mut str: *const ::core::ffi::c_char) {
    if snprintf(
        &raw mut (*enc).error_str_ as *mut ::core::ffi::c_char,
        ERROR_STR_MAX_LENGTH as size_t,
        b"%s.\0" as *const u8 as *const ::core::ffi::c_char,
        str,
    ) < 0 as ::core::ffi::c_int
    {
        '_c2rust_label: {
            __assert_fail(
                b"0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                224 as ::core::ffi::c_uint,
                b"void MarkError(WebPAnimEncoder *const, const char *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        };
    }
}
unsafe extern "C" fn MarkError2(
    enc: *mut WebPAnimEncoder,
    mut str: *const ::core::ffi::c_char,
    mut error_code: ::core::ffi::c_int,
) {
    if snprintf(
        &raw mut (*enc).error_str_ as *mut ::core::ffi::c_char,
        ERROR_STR_MAX_LENGTH as size_t,
        b"%s: %d.\0" as *const u8 as *const ::core::ffi::c_char,
        str,
        error_code,
    ) < 0 as ::core::ffi::c_int
    {
        '_c2rust_label: {
            __assert_fail(
                b"0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                232 as ::core::ffi::c_uint,
                b"void MarkError2(WebPAnimEncoder *const, const char *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimEncoderNewInternal(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut enc_options: *const WebPAnimEncoderOptions,
    mut abi_version: ::core::ffi::c_int,
) -> *mut WebPAnimEncoder {
    let mut enc: *mut WebPAnimEncoder = ::core::ptr::null_mut::<WebPAnimEncoder>();
    if abi_version >> 8 as ::core::ffi::c_int
        != 0x108 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<WebPAnimEncoder>();
    }
    if width <= 0 as ::core::ffi::c_int
        || height <= 0 as ::core::ffi::c_int
        || (width as uint64_t).wrapping_mul(height as uint64_t) as ::core::ffi::c_ulonglong
            >= MAX_IMAGE_AREA
    {
        return ::core::ptr::null_mut::<WebPAnimEncoder>();
    }
    enc = WebPSafeCalloc(
        1 as uint64_t,
        ::core::mem::size_of::<WebPAnimEncoder>() as size_t,
    ) as *mut WebPAnimEncoder;
    if enc.is_null() {
        return ::core::ptr::null_mut::<WebPAnimEncoder>();
    }
    MarkNoError(enc);
    *(&raw const (*enc).canvas_width_ as *mut ::core::ffi::c_int) = width;
    *(&raw const (*enc).canvas_height_ as *mut ::core::ffi::c_int) = height;
    if !enc_options.is_null() {
        *(&raw const (*enc).options_ as *mut WebPAnimEncoderOptions) = *enc_options;
        SanitizeEncoderOptions(&raw const (*enc).options_ as *mut WebPAnimEncoderOptions);
    } else {
        DefaultEncoderOptions(&raw const (*enc).options_ as *mut WebPAnimEncoderOptions);
    }
    if !(WebPPictureInit(&raw mut (*enc).curr_canvas_copy_) == 0
        || WebPPictureInit(&raw mut (*enc).prev_canvas_) == 0
        || WebPPictureInit(&raw mut (*enc).prev_canvas_disposed_) == 0)
    {
        (*enc).curr_canvas_copy_.width = width;
        (*enc).curr_canvas_copy_.height = height;
        (*enc).curr_canvas_copy_.use_argb = 1 as ::core::ffi::c_int;
        if !(WebPPictureAlloc(&raw mut (*enc).curr_canvas_copy_) == 0
            || WebPPictureCopy(
                &raw mut (*enc).curr_canvas_copy_,
                &raw mut (*enc).prev_canvas_,
            ) == 0
            || WebPPictureCopy(
                &raw mut (*enc).curr_canvas_copy_,
                &raw mut (*enc).prev_canvas_disposed_,
            ) == 0)
        {
            WebPUtilClearPic(
                &raw mut (*enc).prev_canvas_,
                ::core::ptr::null::<FrameRectangle>(),
            );
            (*enc).curr_canvas_copy_modified_ = 1 as ::core::ffi::c_int;
            ResetCounters(enc);
            (*enc).size_ =
                ((*enc).options_.kmax - (*enc).options_.kmin + 1 as ::core::ffi::c_int) as size_t;
            if (*enc).size_ < 2 as size_t {
                (*enc).size_ = 2 as size_t;
            }
            (*enc).encoded_frames_ = WebPSafeCalloc(
                (*enc).size_ as uint64_t,
                ::core::mem::size_of::<EncodedFrame>() as size_t,
            ) as *mut EncodedFrame;
            if !(*enc).encoded_frames_.is_null() {
                (*enc).mux_ = WebPMuxNew();
                if !(*enc).mux_.is_null() {
                    (*enc).count_since_key_frame_ = 0 as ::core::ffi::c_int;
                    (*enc).first_timestamp_ = 0 as ::core::ffi::c_int;
                    (*enc).prev_timestamp_ = 0 as ::core::ffi::c_int;
                    (*enc).prev_candidate_undecided_ = 0 as ::core::ffi::c_int;
                    (*enc).is_first_frame_ = 1 as ::core::ffi::c_int;
                    (*enc).got_null_frame_ = 0 as ::core::ffi::c_int;
                    return enc;
                }
            }
        }
    }
    WebPAnimEncoderDelete(enc);
    return ::core::ptr::null_mut::<WebPAnimEncoder>();
}
unsafe extern "C" fn FrameRelease(encoded_frame: *mut EncodedFrame) {
    if !encoded_frame.is_null() {
        WebPDataClear(&raw mut (*encoded_frame).sub_frame_.bitstream);
        WebPDataClear(&raw mut (*encoded_frame).key_frame_.bitstream);
        memset(
            encoded_frame as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<EncodedFrame>() as size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimEncoderDelete(mut enc: *mut WebPAnimEncoder) {
    if !enc.is_null() {
        WebPPictureFree(&raw mut (*enc).curr_canvas_copy_);
        WebPPictureFree(&raw mut (*enc).prev_canvas_);
        WebPPictureFree(&raw mut (*enc).prev_canvas_disposed_);
        if !(*enc).encoded_frames_.is_null() {
            let mut i: size_t = 0;
            i = 0 as size_t;
            while i < (*enc).size_ {
                FrameRelease((*enc).encoded_frames_.offset(i as isize) as *mut EncodedFrame);
                i = i.wrapping_add(1);
            }
            WebPSafeFree((*enc).encoded_frames_ as *mut ::core::ffi::c_void);
        }
        WebPMuxDelete((*enc).mux_);
        WebPSafeFree(enc as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn GetFrame(
    enc: *const WebPAnimEncoder,
    mut position: size_t,
) -> *mut EncodedFrame {
    '_c2rust_label: {
        if (*enc).start_.wrapping_add(position) < (*enc).size_ {
        } else {
            __assert_fail(
                b"enc->start_ + position < enc->size_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                340 as ::core::ffi::c_uint,
                b"EncodedFrame *GetFrame(const WebPAnimEncoder *const, size_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return (*enc)
        .encoded_frames_
        .offset((*enc).start_.wrapping_add(position) as isize) as *mut EncodedFrame;
}
#[inline]
unsafe extern "C" fn ComparePixelsLossless(
    mut src: *const uint32_t,
    mut src_step: ::core::ffi::c_int,
    mut dst: *const uint32_t,
    mut dst_step: ::core::ffi::c_int,
    mut length: ::core::ffi::c_int,
    mut max_allowed_diff: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if length > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"length > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                354 as ::core::ffi::c_uint,
                b"int ComparePixelsLossless(const uint32_t *, int, const uint32_t *, int, int, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    loop {
        let fresh1 = length;
        length = length - 1;
        if !(fresh1 > 0 as ::core::ffi::c_int) {
            break;
        }
        if *src != *dst {
            return 0 as ::core::ffi::c_int;
        }
        src = src.offset(src_step as isize);
        dst = dst.offset(dst_step as isize);
    }
    return 1 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn PixelsAreSimilar(
    mut src: uint32_t,
    mut dst: uint32_t,
    mut max_allowed_diff: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let src_a: ::core::ffi::c_int =
        (src >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int;
    let src_r: ::core::ffi::c_int =
        (src >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int;
    let src_g: ::core::ffi::c_int =
        (src >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int;
    let src_b: ::core::ffi::c_int =
        (src >> 0 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int;
    let dst_a: ::core::ffi::c_int =
        (dst >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int;
    let dst_r: ::core::ffi::c_int =
        (dst >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int;
    let dst_g: ::core::ffi::c_int =
        (dst >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int;
    let dst_b: ::core::ffi::c_int =
        (dst >> 0 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int;
    return (src_a == dst_a
        && abs(src_r - dst_r) * dst_a <= max_allowed_diff * 255 as ::core::ffi::c_int
        && abs(src_g - dst_g) * dst_a <= max_allowed_diff * 255 as ::core::ffi::c_int
        && abs(src_b - dst_b) * dst_a <= max_allowed_diff * 255 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn ComparePixelsLossy(
    mut src: *const uint32_t,
    mut src_step: ::core::ffi::c_int,
    mut dst: *const uint32_t,
    mut dst_step: ::core::ffi::c_int,
    mut length: ::core::ffi::c_int,
    mut max_allowed_diff: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if length > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"length > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                389 as ::core::ffi::c_uint,
                b"int ComparePixelsLossy(const uint32_t *, int, const uint32_t *, int, int, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    loop {
        let fresh0 = length;
        length = length - 1;
        if !(fresh0 > 0 as ::core::ffi::c_int) {
            break;
        }
        if PixelsAreSimilar(*src, *dst, max_allowed_diff) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        src = src.offset(src_step as isize);
        dst = dst.offset(dst_step as isize);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn IsEmptyRect(rect: *const FrameRectangle) -> ::core::ffi::c_int {
    return ((*rect).width_ == 0 as ::core::ffi::c_int || (*rect).height_ == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn QualityToMaxDiff(mut quality: ::core::ffi::c_float) -> ::core::ffi::c_int {
    let val: ::core::ffi::c_double =
        pow(quality as ::core::ffi::c_double / 100.0f64, 0.5f64) as ::core::ffi::c_double;
    let max_diff: ::core::ffi::c_double = 31 as ::core::ffi::c_int as ::core::ffi::c_double
        * (1 as ::core::ffi::c_int as ::core::ffi::c_double - val)
        + 1 as ::core::ffi::c_int as ::core::ffi::c_double * val;
    return (max_diff + 0.5f64) as ::core::ffi::c_int;
}
unsafe extern "C" fn MinimizeChangeRectangle(
    src: *const WebPPicture,
    dst: *const WebPPicture,
    rect: *mut FrameRectangle,
    mut is_lossless: ::core::ffi::c_int,
    mut quality: ::core::ffi::c_float,
) {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let compare_pixels: ComparePixelsFunc = if is_lossless != 0 {
        Some(
            ComparePixelsLossless
                as unsafe extern "C" fn(
                    *const uint32_t,
                    ::core::ffi::c_int,
                    *const uint32_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
    } else {
        Some(
            ComparePixelsLossy
                as unsafe extern "C" fn(
                    *const uint32_t,
                    ::core::ffi::c_int,
                    *const uint32_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
    };
    let max_allowed_diff_lossy: ::core::ffi::c_int =
        QualityToMaxDiff(quality) as ::core::ffi::c_int;
    let max_allowed_diff: ::core::ffi::c_int = if is_lossless != 0 {
        0 as ::core::ffi::c_int
    } else {
        max_allowed_diff_lossy
    };
    '_c2rust_label: {
        if (*src).width == (*dst).width && (*src).height == (*dst).height {
        } else {
            __assert_fail(
                b"src->width == dst->width && src->height == dst->height\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                422 as ::core::ffi::c_uint,
                b"void MinimizeChangeRectangle(const WebPPicture *const, const WebPPicture *const, FrameRectangle *const, int, float)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*rect).x_offset_ + (*rect).width_ <= (*dst).width {
        } else {
            __assert_fail(
                b"rect->x_offset_ + rect->width_ <= dst->width\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                423 as ::core::ffi::c_uint,
                b"void MinimizeChangeRectangle(const WebPPicture *const, const WebPPicture *const, FrameRectangle *const, int, float)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*rect).y_offset_ + (*rect).height_ <= (*dst).height {
        } else {
            __assert_fail(
                b"rect->y_offset_ + rect->height_ <= dst->height\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                424 as ::core::ffi::c_uint,
                b"void MinimizeChangeRectangle(const WebPPicture *const, const WebPPicture *const, FrameRectangle *const, int, float)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    i = (*rect).x_offset_;
    while i < (*rect).x_offset_ + (*rect).width_ {
        let src_argb: *const uint32_t = (*src)
            .argb
            .offset(((*rect).y_offset_ * (*src).argb_stride + i) as isize)
            as *mut uint32_t;
        let dst_argb: *const uint32_t = (*dst)
            .argb
            .offset(((*rect).y_offset_ * (*dst).argb_stride + i) as isize)
            as *mut uint32_t;
        if !(compare_pixels.expect("non-null function pointer")(
            src_argb,
            (*src).argb_stride,
            dst_argb,
            (*dst).argb_stride,
            (*rect).height_,
            max_allowed_diff,
        ) != 0)
        {
            break;
        }
        (*rect).width_ -= 1;
        (*rect).x_offset_ += 1;
        i += 1;
    }
    if (*rect).width_ == 0 as ::core::ffi::c_int {
        current_block = 7746541823990567472;
    } else {
        i = (*rect).x_offset_ + (*rect).width_ - 1 as ::core::ffi::c_int;
        while i >= (*rect).x_offset_ {
            let src_argb_0: *const uint32_t = (*src)
                .argb
                .offset(((*rect).y_offset_ * (*src).argb_stride + i) as isize)
                as *mut uint32_t;
            let dst_argb_0: *const uint32_t = (*dst)
                .argb
                .offset(((*rect).y_offset_ * (*dst).argb_stride + i) as isize)
                as *mut uint32_t;
            if !(compare_pixels.expect("non-null function pointer")(
                src_argb_0,
                (*src).argb_stride,
                dst_argb_0,
                (*dst).argb_stride,
                (*rect).height_,
                max_allowed_diff,
            ) != 0)
            {
                break;
            }
            (*rect).width_ -= 1;
            i -= 1;
        }
        if (*rect).width_ == 0 as ::core::ffi::c_int {
            current_block = 7746541823990567472;
        } else {
            j = (*rect).y_offset_;
            while j < (*rect).y_offset_ + (*rect).height_ {
                let src_argb_1: *const uint32_t = (*src)
                    .argb
                    .offset((j * (*src).argb_stride + (*rect).x_offset_) as isize)
                    as *mut uint32_t;
                let dst_argb_1: *const uint32_t = (*dst)
                    .argb
                    .offset((j * (*dst).argb_stride + (*rect).x_offset_) as isize)
                    as *mut uint32_t;
                if !(compare_pixels.expect("non-null function pointer")(
                    src_argb_1,
                    1 as ::core::ffi::c_int,
                    dst_argb_1,
                    1 as ::core::ffi::c_int,
                    (*rect).width_,
                    max_allowed_diff,
                ) != 0)
                {
                    break;
                }
                (*rect).height_ -= 1;
                (*rect).y_offset_ += 1;
                j += 1;
            }
            if (*rect).height_ == 0 as ::core::ffi::c_int {
                current_block = 7746541823990567472;
            } else {
                j = (*rect).y_offset_ + (*rect).height_ - 1 as ::core::ffi::c_int;
                while j >= (*rect).y_offset_ {
                    let src_argb_2: *const uint32_t = (*src)
                        .argb
                        .offset((j * (*src).argb_stride + (*rect).x_offset_) as isize)
                        as *mut uint32_t;
                    let dst_argb_2: *const uint32_t = (*dst)
                        .argb
                        .offset((j * (*dst).argb_stride + (*rect).x_offset_) as isize)
                        as *mut uint32_t;
                    if !(compare_pixels.expect("non-null function pointer")(
                        src_argb_2,
                        1 as ::core::ffi::c_int,
                        dst_argb_2,
                        1 as ::core::ffi::c_int,
                        (*rect).width_,
                        max_allowed_diff,
                    ) != 0)
                    {
                        break;
                    }
                    (*rect).height_ -= 1;
                    j -= 1;
                }
                if (*rect).height_ == 0 as ::core::ffi::c_int {
                    current_block = 7746541823990567472;
                } else if IsEmptyRect(rect) != 0 {
                    current_block = 7746541823990567472;
                } else {
                    current_block = 14072441030219150333;
                }
            }
        }
    }
    match current_block {
        7746541823990567472 => {
            (*rect).x_offset_ = 0 as ::core::ffi::c_int;
            (*rect).y_offset_ = 0 as ::core::ffi::c_int;
            (*rect).width_ = 0 as ::core::ffi::c_int;
            (*rect).height_ = 0 as ::core::ffi::c_int;
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn SnapToEvenOffsets(rect: *mut FrameRectangle) {
    (*rect).width_ += (*rect).x_offset_ & 1 as ::core::ffi::c_int;
    (*rect).height_ += (*rect).y_offset_ & 1 as ::core::ffi::c_int;
    (*rect).x_offset_ &= !(1 as ::core::ffi::c_int);
    (*rect).y_offset_ &= !(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn SubFrameParamsInit(
    params: *mut SubFrameParams,
    mut should_try: ::core::ffi::c_int,
    mut empty_rect_allowed: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*params).should_try_ = should_try;
    (*params).empty_rect_allowed_ = empty_rect_allowed;
    if WebPPictureInit(&raw mut (*params).sub_frame_ll_) == 0
        || WebPPictureInit(&raw mut (*params).sub_frame_lossy_) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn SubFrameParamsFree(params: *mut SubFrameParams) {
    WebPPictureFree(&raw mut (*params).sub_frame_ll_);
    WebPPictureFree(&raw mut (*params).sub_frame_lossy_);
}
unsafe extern "C" fn GetSubRect(
    prev_canvas: *const WebPPicture,
    curr_canvas: *const WebPPicture,
    mut is_key_frame: ::core::ffi::c_int,
    mut is_first_frame: ::core::ffi::c_int,
    mut empty_rect_allowed: ::core::ffi::c_int,
    mut is_lossless: ::core::ffi::c_int,
    mut quality: ::core::ffi::c_float,
    rect: *mut FrameRectangle,
    sub_frame: *mut WebPPicture,
) -> ::core::ffi::c_int {
    if is_key_frame == 0 || is_first_frame != 0 {
        MinimizeChangeRectangle(prev_canvas, curr_canvas, rect, is_lossless, quality);
    }
    if IsEmptyRect(rect) != 0 {
        if empty_rect_allowed != 0 {
            return 1 as ::core::ffi::c_int;
        } else {
            (*rect).width_ = 1 as ::core::ffi::c_int;
            (*rect).height_ = 1 as ::core::ffi::c_int;
            '_c2rust_label: {
                if (*rect).x_offset_ == 0 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"rect->x_offset_ == 0\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        554 as ::core::ffi::c_uint,
                        b"int GetSubRect(const WebPPicture *const, const WebPPicture *const, int, int, int, int, float, FrameRectangle *const, WebPPicture *const)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            '_c2rust_label_0: {
                if (*rect).y_offset_ == 0 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"rect->y_offset_ == 0\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        555 as ::core::ffi::c_uint,
                        b"int GetSubRect(const WebPPicture *const, const WebPPicture *const, int, int, int, int, float, FrameRectangle *const, WebPPicture *const)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
        }
    }
    SnapToEvenOffsets(rect);
    return WebPPictureView(
        curr_canvas,
        (*rect).x_offset_,
        (*rect).y_offset_,
        (*rect).width_,
        (*rect).height_,
        sub_frame,
    );
}
unsafe extern "C" fn GetSubRects(
    prev_canvas: *const WebPPicture,
    curr_canvas: *const WebPPicture,
    mut is_key_frame: ::core::ffi::c_int,
    mut is_first_frame: ::core::ffi::c_int,
    mut quality: ::core::ffi::c_float,
    params: *mut SubFrameParams,
) -> ::core::ffi::c_int {
    (*params).rect_ll_.x_offset_ = 0 as ::core::ffi::c_int;
    (*params).rect_ll_.y_offset_ = 0 as ::core::ffi::c_int;
    (*params).rect_ll_.width_ = (*curr_canvas).width;
    (*params).rect_ll_.height_ = (*curr_canvas).height;
    if GetSubRect(
        prev_canvas,
        curr_canvas,
        is_key_frame,
        is_first_frame,
        (*params).empty_rect_allowed_,
        1 as ::core::ffi::c_int,
        quality,
        &raw mut (*params).rect_ll_,
        &raw mut (*params).sub_frame_ll_,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    (*params).rect_lossy_ = (*params).rect_ll_;
    return GetSubRect(
        prev_canvas,
        curr_canvas,
        is_key_frame,
        is_first_frame,
        (*params).empty_rect_allowed_,
        0 as ::core::ffi::c_int,
        quality,
        &raw mut (*params).rect_lossy_,
        &raw mut (*params).sub_frame_lossy_,
    );
}
#[inline]
unsafe extern "C" fn clip(
    mut v: ::core::ffi::c_int,
    mut min_v: ::core::ffi::c_int,
    mut max_v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if v < min_v {
        min_v
    } else if v > max_v {
        max_v
    } else {
        v
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimEncoderRefineRect(
    prev_canvas: *const WebPPicture,
    curr_canvas: *const WebPPicture,
    mut is_lossless: ::core::ffi::c_int,
    mut quality: ::core::ffi::c_float,
    x_offset: *mut ::core::ffi::c_int,
    y_offset: *mut ::core::ffi::c_int,
    width: *mut ::core::ffi::c_int,
    height: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rect: FrameRectangle = FrameRectangle {
        x_offset_: 0,
        y_offset_: 0,
        width_: 0,
        height_: 0,
    };
    let mut right: ::core::ffi::c_int = 0;
    let mut left: ::core::ffi::c_int = 0;
    let mut bottom: ::core::ffi::c_int = 0;
    let mut top: ::core::ffi::c_int = 0;
    if prev_canvas.is_null()
        || curr_canvas.is_null()
        || (*prev_canvas).width != (*curr_canvas).width
        || (*prev_canvas).height != (*curr_canvas).height
        || (*prev_canvas).use_argb == 0
        || (*curr_canvas).use_argb == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    right = clip(
        *x_offset + *width,
        0 as ::core::ffi::c_int,
        (*curr_canvas).width,
    );
    left = clip(
        *x_offset,
        0 as ::core::ffi::c_int,
        (*curr_canvas).width - 1 as ::core::ffi::c_int,
    );
    bottom = clip(
        *y_offset + *height,
        0 as ::core::ffi::c_int,
        (*curr_canvas).height,
    );
    top = clip(
        *y_offset,
        0 as ::core::ffi::c_int,
        (*curr_canvas).height - 1 as ::core::ffi::c_int,
    );
    rect.x_offset_ = left;
    rect.y_offset_ = top;
    rect.width_ = clip(
        right - left,
        0 as ::core::ffi::c_int,
        (*curr_canvas).width - rect.x_offset_,
    );
    rect.height_ = clip(
        bottom - top,
        0 as ::core::ffi::c_int,
        (*curr_canvas).height - rect.y_offset_,
    );
    MinimizeChangeRectangle(
        prev_canvas,
        curr_canvas,
        &raw mut rect,
        is_lossless,
        quality,
    );
    SnapToEvenOffsets(&raw mut rect);
    *x_offset = rect.x_offset_;
    *y_offset = rect.y_offset_;
    *width = rect.width_;
    *height = rect.height_;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn DisposeFrameRectangle(
    mut dispose_method: ::core::ffi::c_int,
    rect: *const FrameRectangle,
    curr_canvas: *mut WebPPicture,
) {
    '_c2rust_label: {
        if !rect.is_null() {
        } else {
            __assert_fail(
                b"rect != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                624 as ::core::ffi::c_uint,
                b"void DisposeFrameRectangle(int, const FrameRectangle *const, WebPPicture *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if dispose_method == WEBP_MUX_DISPOSE_BACKGROUND as ::core::ffi::c_int {
        WebPUtilClearPic(curr_canvas, rect);
    }
}
unsafe extern "C" fn RectArea(rect: *const FrameRectangle) -> uint32_t {
    return ((*rect).width_ as uint32_t).wrapping_mul((*rect).height_ as uint32_t);
}
unsafe extern "C" fn IsLosslessBlendingPossible(
    src: *const WebPPicture,
    dst: *const WebPPicture,
    rect: *const FrameRectangle,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if (*src).width == (*dst).width && (*src).height == (*dst).height {
        } else {
            __assert_fail(
                b"src->width == dst->width && src->height == dst->height\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                638 as ::core::ffi::c_uint,
                b"int IsLosslessBlendingPossible(const WebPPicture *const, const WebPPicture *const, const FrameRectangle *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*rect).x_offset_ + (*rect).width_ <= (*dst).width {
        } else {
            __assert_fail(
                b"rect->x_offset_ + rect->width_ <= dst->width\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                639 as ::core::ffi::c_uint,
                b"int IsLosslessBlendingPossible(const WebPPicture *const, const WebPPicture *const, const FrameRectangle *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*rect).y_offset_ + (*rect).height_ <= (*dst).height {
        } else {
            __assert_fail(
                b"rect->y_offset_ + rect->height_ <= dst->height\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                640 as ::core::ffi::c_uint,
                b"int IsLosslessBlendingPossible(const WebPPicture *const, const WebPPicture *const, const FrameRectangle *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    j = (*rect).y_offset_;
    while j < (*rect).y_offset_ + (*rect).height_ {
        i = (*rect).x_offset_;
        while i < (*rect).x_offset_ + (*rect).width_ {
            let src_pixel: uint32_t = *(*src).argb.offset((j * (*src).argb_stride + i) as isize);
            let dst_pixel: uint32_t = *(*dst).argb.offset((j * (*dst).argb_stride + i) as isize);
            let dst_alpha: uint32_t = dst_pixel >> 24 as ::core::ffi::c_int;
            if dst_alpha != 0xff as uint32_t && src_pixel != dst_pixel {
                return 0 as ::core::ffi::c_int;
            }
            i += 1;
        }
        j += 1;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn IsLossyBlendingPossible(
    src: *const WebPPicture,
    dst: *const WebPPicture,
    rect: *const FrameRectangle,
    mut quality: ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    let max_allowed_diff_lossy: ::core::ffi::c_int =
        QualityToMaxDiff(quality) as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if (*src).width == (*dst).width && (*src).height == (*dst).height {
        } else {
            __assert_fail(
                b"src->width == dst->width && src->height == dst->height\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                662 as ::core::ffi::c_uint,
                b"int IsLossyBlendingPossible(const WebPPicture *const, const WebPPicture *const, const FrameRectangle *const, float)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*rect).x_offset_ + (*rect).width_ <= (*dst).width {
        } else {
            __assert_fail(
                b"rect->x_offset_ + rect->width_ <= dst->width\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                663 as ::core::ffi::c_uint,
                b"int IsLossyBlendingPossible(const WebPPicture *const, const WebPPicture *const, const FrameRectangle *const, float)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*rect).y_offset_ + (*rect).height_ <= (*dst).height {
        } else {
            __assert_fail(
                b"rect->y_offset_ + rect->height_ <= dst->height\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                664 as ::core::ffi::c_uint,
                b"int IsLossyBlendingPossible(const WebPPicture *const, const WebPPicture *const, const FrameRectangle *const, float)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    j = (*rect).y_offset_;
    while j < (*rect).y_offset_ + (*rect).height_ {
        i = (*rect).x_offset_;
        while i < (*rect).x_offset_ + (*rect).width_ {
            let src_pixel: uint32_t = *(*src).argb.offset((j * (*src).argb_stride + i) as isize);
            let dst_pixel: uint32_t = *(*dst).argb.offset((j * (*dst).argb_stride + i) as isize);
            let dst_alpha: uint32_t = dst_pixel >> 24 as ::core::ffi::c_int;
            if dst_alpha != 0xff as uint32_t
                && PixelsAreSimilar(src_pixel, dst_pixel, max_allowed_diff_lossy) == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            i += 1;
        }
        j += 1;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn IncreaseTransparency(
    src: *const WebPPicture,
    rect: *const FrameRectangle,
    dst: *mut WebPPicture,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut modified: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    '_c2rust_label: {
        if !src.is_null() && !dst.is_null() && !rect.is_null() {
        } else {
            __assert_fail(
                b"src != NULL && dst != NULL && rect != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                689 as ::core::ffi::c_uint,
                b"int IncreaseTransparency(const WebPPicture *const, const FrameRectangle *const, WebPPicture *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*src).width == (*dst).width && (*src).height == (*dst).height {
        } else {
            __assert_fail(
                b"src->width == dst->width && src->height == dst->height\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                690 as ::core::ffi::c_uint,
                b"int IncreaseTransparency(const WebPPicture *const, const FrameRectangle *const, WebPPicture *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    j = (*rect).y_offset_;
    while j < (*rect).y_offset_ + (*rect).height_ {
        let psrc: *const uint32_t = (*src).argb.offset((j * (*src).argb_stride) as isize);
        let pdst: *mut uint32_t = (*dst).argb.offset((j * (*dst).argb_stride) as isize);
        i = (*rect).x_offset_;
        while i < (*rect).x_offset_ + (*rect).width_ {
            if *psrc.offset(i as isize) == *pdst.offset(i as isize)
                && *pdst.offset(i as isize) != TRANSPARENT_COLOR as uint32_t
            {
                *pdst.offset(i as isize) = TRANSPARENT_COLOR as uint32_t;
                modified = 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        j += 1;
    }
    return modified;
}
unsafe extern "C" fn FlattenSimilarBlocks(
    src: *const WebPPicture,
    rect: *const FrameRectangle,
    dst: *mut WebPPicture,
    mut quality: ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    let max_allowed_diff_lossy: ::core::ffi::c_int =
        QualityToMaxDiff(quality) as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut modified: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let block_size: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let y_start: ::core::ffi::c_int =
        (*rect).y_offset_ + block_size & !(block_size - 1 as ::core::ffi::c_int);
    let y_end: ::core::ffi::c_int =
        (*rect).y_offset_ + (*rect).height_ & !(block_size - 1 as ::core::ffi::c_int);
    let x_start: ::core::ffi::c_int =
        (*rect).x_offset_ + block_size & !(block_size - 1 as ::core::ffi::c_int);
    let x_end: ::core::ffi::c_int =
        (*rect).x_offset_ + (*rect).width_ & !(block_size - 1 as ::core::ffi::c_int);
    '_c2rust_label: {
        if !src.is_null() && !dst.is_null() && !rect.is_null() {
        } else {
            __assert_fail(
                b"src != NULL && dst != NULL && rect != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                721 as ::core::ffi::c_uint,
                b"int FlattenSimilarBlocks(const WebPPicture *const, const FrameRectangle *const, WebPPicture *const, float)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*src).width == (*dst).width && (*src).height == (*dst).height {
        } else {
            __assert_fail(
                b"src->width == dst->width && src->height == dst->height\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                722 as ::core::ffi::c_uint,
                b"int FlattenSimilarBlocks(const WebPPicture *const, const FrameRectangle *const, WebPPicture *const, float)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if block_size & block_size - 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"(block_size & (block_size - 1)) == 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                723 as ::core::ffi::c_uint,
                b"int FlattenSimilarBlocks(const WebPPicture *const, const FrameRectangle *const, WebPPicture *const, float)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    j = y_start;
    while j < y_end {
        i = x_start;
        while i < x_end {
            let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut avg_r: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut avg_g: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut avg_b: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut x: ::core::ffi::c_int = 0;
            let mut y: ::core::ffi::c_int = 0;
            let psrc: *const uint32_t = (*src)
                .argb
                .offset((j * (*src).argb_stride) as isize)
                .offset(i as isize);
            let pdst: *mut uint32_t = (*dst)
                .argb
                .offset((j * (*dst).argb_stride) as isize)
                .offset(i as isize);
            y = 0 as ::core::ffi::c_int;
            while y < block_size {
                x = 0 as ::core::ffi::c_int;
                while x < block_size {
                    let src_pixel: uint32_t = *psrc.offset((x + y * (*src).argb_stride) as isize);
                    let alpha: ::core::ffi::c_int =
                        (src_pixel >> 24 as ::core::ffi::c_int) as ::core::ffi::c_int;
                    if alpha == 0xff as ::core::ffi::c_int
                        && PixelsAreSimilar(
                            src_pixel,
                            *pdst.offset((x + y * (*dst).argb_stride) as isize),
                            max_allowed_diff_lossy,
                        ) != 0
                    {
                        cnt += 1;
                        avg_r = (avg_r as uint32_t)
                            .wrapping_add(src_pixel >> 16 as ::core::ffi::c_int & 0xff as uint32_t)
                            as ::core::ffi::c_int
                            as ::core::ffi::c_int;
                        avg_g = (avg_g as uint32_t)
                            .wrapping_add(src_pixel >> 8 as ::core::ffi::c_int & 0xff as uint32_t)
                            as ::core::ffi::c_int
                            as ::core::ffi::c_int;
                        avg_b = (avg_b as uint32_t)
                            .wrapping_add(src_pixel >> 0 as ::core::ffi::c_int & 0xff as uint32_t)
                            as ::core::ffi::c_int
                            as ::core::ffi::c_int;
                    }
                    x += 1;
                }
                y += 1;
            }
            if cnt == block_size * block_size {
                let color: uint32_t = ((0 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
                    | avg_r / cnt << 16 as ::core::ffi::c_int
                    | avg_g / cnt << 8 as ::core::ffi::c_int
                    | avg_b / cnt << 0 as ::core::ffi::c_int)
                    as uint32_t;
                y = 0 as ::core::ffi::c_int;
                while y < block_size {
                    x = 0 as ::core::ffi::c_int;
                    while x < block_size {
                        *pdst.offset((x + y * (*dst).argb_stride) as isize) = color;
                        x += 1;
                    }
                    y += 1;
                }
                modified = 1 as ::core::ffi::c_int;
            }
            i += block_size;
        }
        j += block_size;
    }
    return modified;
}
unsafe extern "C" fn EncodeFrame(
    config: *const WebPConfig,
    pic: *mut WebPPicture,
    memory: *mut WebPMemoryWriter,
) -> ::core::ffi::c_int {
    (*pic).use_argb = 1 as ::core::ffi::c_int;
    (*pic).writer = Some(
        WebPMemoryWrite
            as unsafe extern "C" fn(
                *const uint8_t,
                size_t,
                *const WebPPicture,
            ) -> ::core::ffi::c_int,
    ) as WebPWriterFunction;
    (*pic).custom_ptr = memory as *mut ::core::ffi::c_void;
    if WebPEncode(config, pic) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn EncodeCandidate(
    sub_frame: *mut WebPPicture,
    rect: *const FrameRectangle,
    encoder_config: *const WebPConfig,
    mut use_blending: ::core::ffi::c_int,
    candidate: *mut Candidate,
) -> WebPEncodingError {
    let mut config: WebPConfig = *encoder_config;
    let mut error_code: WebPEncodingError = VP8_ENC_OK;
    '_c2rust_label: {
        if !candidate.is_null() {
        } else {
            __assert_fail(
                b"candidate != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                792 as ::core::ffi::c_uint,
                b"WebPEncodingError EncodeCandidate(WebPPicture *const, const FrameRectangle *const, const WebPConfig *const, int, Candidate *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    memset(
        candidate as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Candidate>() as size_t,
    );
    (*candidate).rect_ = *rect;
    (*candidate).info_.id = WEBP_CHUNK_ANMF;
    (*candidate).info_.x_offset = (*rect).x_offset_;
    (*candidate).info_.y_offset = (*rect).y_offset_;
    (*candidate).info_.dispose_method = WEBP_MUX_DISPOSE_NONE;
    (*candidate).info_.blend_method = (if use_blending != 0 {
        WEBP_MUX_BLEND as ::core::ffi::c_int
    } else {
        WEBP_MUX_NO_BLEND as ::core::ffi::c_int
    }) as WebPMuxAnimBlend;
    (*candidate).info_.duration = 0 as ::core::ffi::c_int;
    WebPMemoryWriterInit(&raw mut (*candidate).mem_);
    if config.lossless == 0 && use_blending != 0 {
        config.autofilter = 0 as ::core::ffi::c_int;
        config.filter_strength = 0 as ::core::ffi::c_int;
    }
    if EncodeFrame(&raw mut config, sub_frame, &raw mut (*candidate).mem_) == 0 {
        error_code = (*sub_frame).error_code;
        WebPMemoryWriterClear(&raw mut (*candidate).mem_);
        return error_code;
    } else {
        (*candidate).evaluate_ = 1 as ::core::ffi::c_int;
        return error_code;
    };
}
unsafe extern "C" fn CopyCurrentCanvas(enc: *mut WebPAnimEncoder) {
    if (*enc).curr_canvas_copy_modified_ != 0 {
        WebPCopyPixels((*enc).curr_canvas_, &raw mut (*enc).curr_canvas_copy_);
        (*enc).curr_canvas_copy_.progress_hook = (*(*enc).curr_canvas_).progress_hook;
        (*enc).curr_canvas_copy_.user_data = (*(*enc).curr_canvas_).user_data;
        (*enc).curr_canvas_copy_modified_ = 0 as ::core::ffi::c_int;
    }
}
pub const MIN_COLORS_LOSSY: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const MAX_COLORS_LOSSLESS: ::core::ffi::c_int = 194 as ::core::ffi::c_int;
unsafe extern "C" fn GenerateCandidates(
    enc: *mut WebPAnimEncoder,
    mut candidates: *mut Candidate,
    mut dispose_method: WebPMuxAnimDispose,
    mut is_lossless: ::core::ffi::c_int,
    mut is_key_frame: ::core::ffi::c_int,
    params: *mut SubFrameParams,
    config_ll: *const WebPConfig,
    config_lossy: *const WebPConfig,
) -> WebPEncodingError {
    let mut error_code: WebPEncodingError = VP8_ENC_OK;
    let is_dispose_none: ::core::ffi::c_int = (dispose_method as ::core::ffi::c_uint
        == WEBP_MUX_DISPOSE_NONE as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
    let candidate_ll: *mut Candidate = if is_dispose_none != 0 {
        candidates.offset(LL_DISP_NONE as ::core::ffi::c_int as isize) as *mut Candidate
    } else {
        candidates.offset(LL_DISP_BG as ::core::ffi::c_int as isize) as *mut Candidate
    };
    let candidate_lossy: *mut Candidate = if is_dispose_none != 0 {
        candidates.offset(LOSSY_DISP_NONE as ::core::ffi::c_int as isize) as *mut Candidate
    } else {
        candidates.offset(LOSSY_DISP_BG as ::core::ffi::c_int as isize) as *mut Candidate
    };
    let curr_canvas: *mut WebPPicture = &raw mut (*enc).curr_canvas_copy_;
    let prev_canvas: *const WebPPicture = if is_dispose_none != 0 {
        &raw mut (*enc).prev_canvas_
    } else {
        &raw mut (*enc).prev_canvas_disposed_
    };
    let mut use_blending_ll: ::core::ffi::c_int = 0;
    let mut use_blending_lossy: ::core::ffi::c_int = 0;
    let mut evaluate_ll: ::core::ffi::c_int = 0;
    let mut evaluate_lossy: ::core::ffi::c_int = 0;
    CopyCurrentCanvas(enc);
    use_blending_ll = (is_key_frame == 0
        && IsLosslessBlendingPossible(prev_canvas, curr_canvas, &raw mut (*params).rect_ll_) != 0)
        as ::core::ffi::c_int;
    use_blending_lossy = (is_key_frame == 0
        && IsLossyBlendingPossible(
            prev_canvas,
            curr_canvas,
            &raw mut (*params).rect_lossy_,
            (*config_lossy).quality,
        ) != 0) as ::core::ffi::c_int;
    if (*enc).options_.allow_mixed == 0 {
        evaluate_ll = is_lossless;
        evaluate_lossy = (is_lossless == 0) as ::core::ffi::c_int;
    } else if (*enc).options_.minimize_size != 0 {
        evaluate_ll = 1 as ::core::ffi::c_int;
        evaluate_lossy = 1 as ::core::ffi::c_int;
    } else {
        let num_colors: ::core::ffi::c_int = WebPGetColorPalette(
            &raw mut (*params).sub_frame_ll_,
            ::core::ptr::null_mut::<uint32_t>(),
        ) as ::core::ffi::c_int;
        evaluate_ll = (num_colors < MAX_COLORS_LOSSLESS) as ::core::ffi::c_int;
        evaluate_lossy = (num_colors >= MIN_COLORS_LOSSY) as ::core::ffi::c_int;
    }
    if evaluate_ll != 0 {
        CopyCurrentCanvas(enc);
        if use_blending_ll != 0 {
            (*enc).curr_canvas_copy_modified_ =
                IncreaseTransparency(prev_canvas, &raw mut (*params).rect_ll_, curr_canvas);
        }
        error_code = EncodeCandidate(
            &raw mut (*params).sub_frame_ll_,
            &raw mut (*params).rect_ll_,
            config_ll,
            use_blending_ll,
            candidate_ll,
        );
        if error_code as ::core::ffi::c_uint
            != VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return error_code;
        }
    }
    if evaluate_lossy != 0 {
        CopyCurrentCanvas(enc);
        if use_blending_lossy != 0 {
            (*enc).curr_canvas_copy_modified_ = FlattenSimilarBlocks(
                prev_canvas,
                &raw mut (*params).rect_lossy_,
                curr_canvas,
                (*config_lossy).quality,
            );
        }
        error_code = EncodeCandidate(
            &raw mut (*params).sub_frame_lossy_,
            &raw mut (*params).rect_lossy_,
            config_lossy,
            use_blending_lossy,
            candidate_lossy,
        );
        if error_code as ::core::ffi::c_uint
            != VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return error_code;
        }
        (*enc).curr_canvas_copy_modified_ = 1 as ::core::ffi::c_int;
    }
    return error_code;
}
unsafe extern "C" fn GetEncodedData(memory: *const WebPMemoryWriter, encoded_data: *mut WebPData) {
    (*encoded_data).bytes = (*memory).mem;
    (*encoded_data).size = (*memory).size;
}
unsafe extern "C" fn SetPreviousDisposeMethod(
    enc: *mut WebPAnimEncoder,
    mut dispose_method: WebPMuxAnimDispose,
) {
    let position: size_t = (*enc).count_.wrapping_sub(2 as size_t);
    let prev_enc_frame: *mut EncodedFrame = GetFrame(enc, position) as *mut EncodedFrame;
    '_c2rust_label: {
        if (*enc).count_ >= 2 as size_t {
        } else {
            __assert_fail(
                b"enc->count_ >= 2\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                930 as ::core::ffi::c_uint,
                b"void SetPreviousDisposeMethod(WebPAnimEncoder *const, WebPMuxAnimDispose)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if (*enc).prev_candidate_undecided_ != 0 {
        '_c2rust_label_0: {
            if dispose_method as ::core::ffi::c_uint
                == WEBP_MUX_DISPOSE_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"dispose_method == WEBP_MUX_DISPOSE_NONE\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    933 as ::core::ffi::c_uint,
                    b"void SetPreviousDisposeMethod(WebPAnimEncoder *const, WebPMuxAnimDispose)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        (*prev_enc_frame).sub_frame_.dispose_method = dispose_method;
        (*prev_enc_frame).key_frame_.dispose_method = dispose_method;
    } else {
        let prev_info: *mut WebPMuxFrameInfo = if (*prev_enc_frame).is_key_frame_ != 0 {
            &raw mut (*prev_enc_frame).key_frame_
        } else {
            &raw mut (*prev_enc_frame).sub_frame_
        };
        (*prev_info).dispose_method = dispose_method;
    };
}
unsafe extern "C" fn IncreasePreviousDuration(
    enc: *mut WebPAnimEncoder,
    mut duration: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let position: size_t = (*enc).count_.wrapping_sub(1 as size_t);
    let prev_enc_frame: *mut EncodedFrame = GetFrame(enc, position) as *mut EncodedFrame;
    let mut new_duration: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if (*enc).count_ >= 1 as size_t {
        } else {
            __assert_fail(
                b"enc->count_ >= 1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                949 as ::core::ffi::c_uint,
                b"int IncreasePreviousDuration(WebPAnimEncoder *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*prev_enc_frame).is_key_frame_ == 0
            || (*prev_enc_frame).sub_frame_.duration == (*prev_enc_frame).key_frame_.duration
        {
        } else {
            __assert_fail(
                b"!prev_enc_frame->is_key_frame_ || prev_enc_frame->sub_frame_.duration == prev_enc_frame->key_frame_.duration\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                952 as ::core::ffi::c_uint,
                b"int IncreasePreviousDuration(WebPAnimEncoder *const, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*prev_enc_frame).sub_frame_.duration
            == (*prev_enc_frame).sub_frame_.duration
                & ((1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"prev_enc_frame->sub_frame_.duration == (prev_enc_frame->sub_frame_.duration & (MAX_DURATION - 1))\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                954 as ::core::ffi::c_uint,
                b"int IncreasePreviousDuration(WebPAnimEncoder *const, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if duration
            == duration
                & ((1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"duration == (duration & (MAX_DURATION - 1))\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                955 as ::core::ffi::c_uint,
                b"int IncreasePreviousDuration(WebPAnimEncoder *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    new_duration = (*prev_enc_frame).sub_frame_.duration + duration;
    if new_duration >= MAX_DURATION {
        let rect: FrameRectangle = FrameRectangle {
            x_offset_: 0 as ::core::ffi::c_int,
            y_offset_: 0 as ::core::ffi::c_int,
            width_: 1 as ::core::ffi::c_int,
            height_: 1 as ::core::ffi::c_int,
        };
        let lossless_1x1_bytes: [uint8_t; 28] = [
            0x52 as ::core::ffi::c_int as uint8_t,
            0x49 as ::core::ffi::c_int as uint8_t,
            0x46 as ::core::ffi::c_int as uint8_t,
            0x46 as ::core::ffi::c_int as uint8_t,
            0x14 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x57 as ::core::ffi::c_int as uint8_t,
            0x45 as ::core::ffi::c_int as uint8_t,
            0x42 as ::core::ffi::c_int as uint8_t,
            0x50 as ::core::ffi::c_int as uint8_t,
            0x56 as ::core::ffi::c_int as uint8_t,
            0x50 as ::core::ffi::c_int as uint8_t,
            0x38 as ::core::ffi::c_int as uint8_t,
            0x4c as ::core::ffi::c_int as uint8_t,
            0x8 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x2f as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x10 as ::core::ffi::c_int as uint8_t,
            0x88 as ::core::ffi::c_int as uint8_t,
            0x88 as ::core::ffi::c_int as uint8_t,
            0x8 as ::core::ffi::c_int as uint8_t,
        ];
        let lossless_1x1: WebPData = WebPData {
            bytes: &raw const lossless_1x1_bytes as *const uint8_t,
            size: ::core::mem::size_of::<[uint8_t; 28]>() as size_t,
        };
        let lossy_1x1_bytes: [uint8_t; 72] = [
            0x52 as ::core::ffi::c_int as uint8_t,
            0x49 as ::core::ffi::c_int as uint8_t,
            0x46 as ::core::ffi::c_int as uint8_t,
            0x46 as ::core::ffi::c_int as uint8_t,
            0x40 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x57 as ::core::ffi::c_int as uint8_t,
            0x45 as ::core::ffi::c_int as uint8_t,
            0x42 as ::core::ffi::c_int as uint8_t,
            0x50 as ::core::ffi::c_int as uint8_t,
            0x56 as ::core::ffi::c_int as uint8_t,
            0x50 as ::core::ffi::c_int as uint8_t,
            0x38 as ::core::ffi::c_int as uint8_t,
            0x58 as ::core::ffi::c_int as uint8_t,
            0xa as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x10 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x41 as ::core::ffi::c_int as uint8_t,
            0x4c as ::core::ffi::c_int as uint8_t,
            0x50 as ::core::ffi::c_int as uint8_t,
            0x48 as ::core::ffi::c_int as uint8_t,
            0x2 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x56 as ::core::ffi::c_int as uint8_t,
            0x50 as ::core::ffi::c_int as uint8_t,
            0x38 as ::core::ffi::c_int as uint8_t,
            0x20 as ::core::ffi::c_int as uint8_t,
            0x18 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x30 as ::core::ffi::c_int as uint8_t,
            0x1 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x9d as ::core::ffi::c_int as uint8_t,
            0x1 as ::core::ffi::c_int as uint8_t,
            0x2a as ::core::ffi::c_int as uint8_t,
            0x1 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x1 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x2 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x34 as ::core::ffi::c_int as uint8_t,
            0x25 as ::core::ffi::c_int as uint8_t,
            0xa4 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x3 as ::core::ffi::c_int as uint8_t,
            0x70 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0xfe as ::core::ffi::c_int as uint8_t,
            0xfb as ::core::ffi::c_int as uint8_t,
            0xfd as ::core::ffi::c_int as uint8_t,
            0x50 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ];
        let lossy_1x1: WebPData = WebPData {
            bytes: &raw const lossy_1x1_bytes as *const uint8_t,
            size: ::core::mem::size_of::<[uint8_t; 72]>() as size_t,
        };
        let can_use_lossless: ::core::ffi::c_int = ((*enc).last_config_.lossless != 0
            || (*enc).options_.allow_mixed != 0)
            as ::core::ffi::c_int;
        let curr_enc_frame: *mut EncodedFrame = GetFrame(enc, (*enc).count_) as *mut EncodedFrame;
        (*curr_enc_frame).is_key_frame_ = 0 as ::core::ffi::c_int;
        (*curr_enc_frame).sub_frame_.id = WEBP_CHUNK_ANMF;
        (*curr_enc_frame).sub_frame_.x_offset = 0 as ::core::ffi::c_int;
        (*curr_enc_frame).sub_frame_.y_offset = 0 as ::core::ffi::c_int;
        (*curr_enc_frame).sub_frame_.dispose_method = WEBP_MUX_DISPOSE_NONE;
        (*curr_enc_frame).sub_frame_.blend_method = WEBP_MUX_BLEND;
        (*curr_enc_frame).sub_frame_.duration = duration;
        if WebPDataCopy(
            if can_use_lossless != 0 {
                &raw const lossless_1x1
            } else {
                &raw const lossy_1x1
            },
            &raw mut (*curr_enc_frame).sub_frame_.bitstream,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        (*enc).count_ = (*enc).count_.wrapping_add(1);
        (*enc).count_since_key_frame_ += 1;
        (*enc).flush_count_ = (*enc).count_.wrapping_sub(1 as size_t);
        (*enc).prev_candidate_undecided_ = 0 as ::core::ffi::c_int;
        (*enc).prev_rect_ = rect;
    } else {
        (*prev_enc_frame).sub_frame_.duration = new_duration;
        (*prev_enc_frame).key_frame_.duration = new_duration;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn PickBestCandidate(
    enc: *mut WebPAnimEncoder,
    candidates: *mut Candidate,
    mut is_key_frame: ::core::ffi::c_int,
    encoded_frame: *mut EncodedFrame,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut best_idx: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut best_size: size_t = !(0 as ::core::ffi::c_int) as size_t;
    i = 0 as ::core::ffi::c_int;
    while i < CANDIDATE_COUNT as ::core::ffi::c_int {
        if (*candidates.offset(i as isize)).evaluate_ != 0 {
            let candidate_size: size_t = (*candidates.offset(i as isize)).mem_.size;
            if candidate_size < best_size {
                best_idx = i;
                best_size = candidate_size;
            }
        }
        i += 1;
    }
    '_c2rust_label: {
        if best_idx != -(1 as ::core::ffi::c_int) {
        } else {
            __assert_fail(
                b"best_idx != -1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1025 as ::core::ffi::c_uint,
                b"void PickBestCandidate(WebPAnimEncoder *const, Candidate *const, int, EncodedFrame *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    i = 0 as ::core::ffi::c_int;
    while i < CANDIDATE_COUNT as ::core::ffi::c_int {
        if (*candidates.offset(i as isize)).evaluate_ != 0 {
            if i == best_idx {
                let dst: *mut WebPMuxFrameInfo = if is_key_frame != 0 {
                    &raw mut (*encoded_frame).key_frame_
                } else {
                    &raw mut (*encoded_frame).sub_frame_
                };
                *dst = (*candidates.offset(i as isize)).info_;
                GetEncodedData(
                    &raw mut (*candidates.offset(i as isize)).mem_,
                    &raw mut (*dst).bitstream,
                );
                if is_key_frame == 0 {
                    let prev_dispose_method: WebPMuxAnimDispose =
                        (if best_idx == LL_DISP_NONE as ::core::ffi::c_int
                            || best_idx == LOSSY_DISP_NONE as ::core::ffi::c_int
                        {
                            WEBP_MUX_DISPOSE_NONE as ::core::ffi::c_int
                        } else {
                            WEBP_MUX_DISPOSE_BACKGROUND as ::core::ffi::c_int
                        }) as WebPMuxAnimDispose;
                    SetPreviousDisposeMethod(enc, prev_dispose_method);
                }
                (*enc).prev_rect_ = (*candidates.offset(i as isize)).rect_;
            } else {
                WebPMemoryWriterClear(&raw mut (*candidates.offset(i as isize)).mem_);
                (*candidates.offset(i as isize)).evaluate_ = 0 as ::core::ffi::c_int;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn SetFrame(
    enc: *mut WebPAnimEncoder,
    config: *const WebPConfig,
    mut is_key_frame: ::core::ffi::c_int,
    encoded_frame: *mut EncodedFrame,
    frame_skipped: *mut ::core::ffi::c_int,
) -> WebPEncodingError {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    let mut error_code: WebPEncodingError = VP8_ENC_OK;
    let curr_canvas: *const WebPPicture = &raw mut (*enc).curr_canvas_copy_;
    let prev_canvas: *const WebPPicture = &raw mut (*enc).prev_canvas_;
    let mut candidates: [Candidate; 4] = [Candidate {
        mem_: WebPMemoryWriter {
            mem: ::core::ptr::null_mut::<uint8_t>(),
            size: 0,
            max_size: 0,
            pad: [0; 1],
        },
        info_: WebPMuxFrameInfo {
            bitstream: WebPData {
                bytes: ::core::ptr::null::<uint8_t>(),
                size: 0,
            },
            x_offset: 0,
            y_offset: 0,
            duration: 0,
            id: WEBP_CHUNK_VP8X,
            dispose_method: WEBP_MUX_DISPOSE_NONE,
            blend_method: WEBP_MUX_BLEND,
            pad: [0; 1],
        },
        rect_: FrameRectangle {
            x_offset_: 0,
            y_offset_: 0,
            width_: 0,
            height_: 0,
        },
        evaluate_: 0,
    }; 4];
    let is_lossless: ::core::ffi::c_int = (*config).lossless;
    let consider_lossless: ::core::ffi::c_int =
        (is_lossless != 0 || (*enc).options_.allow_mixed != 0) as ::core::ffi::c_int;
    let consider_lossy: ::core::ffi::c_int =
        (is_lossless == 0 || (*enc).options_.allow_mixed != 0) as ::core::ffi::c_int;
    let is_first_frame: ::core::ffi::c_int = (*enc).is_first_frame_;
    let empty_rect_allowed_none: ::core::ffi::c_int = (is_first_frame == 0) as ::core::ffi::c_int;
    let empty_rect_allowed_bg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let dispose_bg_possible: ::core::ffi::c_int =
        (is_key_frame == 0 && (*enc).prev_candidate_undecided_ == 0) as ::core::ffi::c_int;
    let mut dispose_none_params: SubFrameParams = SubFrameParams {
        should_try_: 0,
        empty_rect_allowed_: 0,
        rect_ll_: FrameRectangle {
            x_offset_: 0,
            y_offset_: 0,
            width_: 0,
            height_: 0,
        },
        sub_frame_ll_: WebPPicture {
            use_argb: 0,
            colorspace: WEBP_YUV420,
            width: 0,
            height: 0,
            y: ::core::ptr::null_mut::<uint8_t>(),
            u: ::core::ptr::null_mut::<uint8_t>(),
            v: ::core::ptr::null_mut::<uint8_t>(),
            y_stride: 0,
            uv_stride: 0,
            a: ::core::ptr::null_mut::<uint8_t>(),
            a_stride: 0,
            pad1: [0; 2],
            argb: ::core::ptr::null_mut::<uint32_t>(),
            argb_stride: 0,
            pad2: [0; 3],
            writer: None,
            custom_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            extra_info_type: 0,
            extra_info: ::core::ptr::null_mut::<uint8_t>(),
            stats: ::core::ptr::null_mut::<WebPAuxStats>(),
            error_code: VP8_ENC_OK,
            progress_hook: None,
            user_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pad3: [0; 3],
            pad4: ::core::ptr::null_mut::<uint8_t>(),
            pad5: ::core::ptr::null_mut::<uint8_t>(),
            pad6: [0; 8],
            memory_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            memory_argb_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pad7: [::core::ptr::null_mut::<::core::ffi::c_void>(); 2],
        },
        rect_lossy_: FrameRectangle {
            x_offset_: 0,
            y_offset_: 0,
            width_: 0,
            height_: 0,
        },
        sub_frame_lossy_: WebPPicture {
            use_argb: 0,
            colorspace: WEBP_YUV420,
            width: 0,
            height: 0,
            y: ::core::ptr::null_mut::<uint8_t>(),
            u: ::core::ptr::null_mut::<uint8_t>(),
            v: ::core::ptr::null_mut::<uint8_t>(),
            y_stride: 0,
            uv_stride: 0,
            a: ::core::ptr::null_mut::<uint8_t>(),
            a_stride: 0,
            pad1: [0; 2],
            argb: ::core::ptr::null_mut::<uint32_t>(),
            argb_stride: 0,
            pad2: [0; 3],
            writer: None,
            custom_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            extra_info_type: 0,
            extra_info: ::core::ptr::null_mut::<uint8_t>(),
            stats: ::core::ptr::null_mut::<WebPAuxStats>(),
            error_code: VP8_ENC_OK,
            progress_hook: None,
            user_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pad3: [0; 3],
            pad4: ::core::ptr::null_mut::<uint8_t>(),
            pad5: ::core::ptr::null_mut::<uint8_t>(),
            pad6: [0; 8],
            memory_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            memory_argb_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pad7: [::core::ptr::null_mut::<::core::ffi::c_void>(); 2],
        },
    };
    let mut dispose_bg_params: SubFrameParams = SubFrameParams {
        should_try_: 0,
        empty_rect_allowed_: 0,
        rect_ll_: FrameRectangle {
            x_offset_: 0,
            y_offset_: 0,
            width_: 0,
            height_: 0,
        },
        sub_frame_ll_: WebPPicture {
            use_argb: 0,
            colorspace: WEBP_YUV420,
            width: 0,
            height: 0,
            y: ::core::ptr::null_mut::<uint8_t>(),
            u: ::core::ptr::null_mut::<uint8_t>(),
            v: ::core::ptr::null_mut::<uint8_t>(),
            y_stride: 0,
            uv_stride: 0,
            a: ::core::ptr::null_mut::<uint8_t>(),
            a_stride: 0,
            pad1: [0; 2],
            argb: ::core::ptr::null_mut::<uint32_t>(),
            argb_stride: 0,
            pad2: [0; 3],
            writer: None,
            custom_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            extra_info_type: 0,
            extra_info: ::core::ptr::null_mut::<uint8_t>(),
            stats: ::core::ptr::null_mut::<WebPAuxStats>(),
            error_code: VP8_ENC_OK,
            progress_hook: None,
            user_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pad3: [0; 3],
            pad4: ::core::ptr::null_mut::<uint8_t>(),
            pad5: ::core::ptr::null_mut::<uint8_t>(),
            pad6: [0; 8],
            memory_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            memory_argb_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pad7: [::core::ptr::null_mut::<::core::ffi::c_void>(); 2],
        },
        rect_lossy_: FrameRectangle {
            x_offset_: 0,
            y_offset_: 0,
            width_: 0,
            height_: 0,
        },
        sub_frame_lossy_: WebPPicture {
            use_argb: 0,
            colorspace: WEBP_YUV420,
            width: 0,
            height: 0,
            y: ::core::ptr::null_mut::<uint8_t>(),
            u: ::core::ptr::null_mut::<uint8_t>(),
            v: ::core::ptr::null_mut::<uint8_t>(),
            y_stride: 0,
            uv_stride: 0,
            a: ::core::ptr::null_mut::<uint8_t>(),
            a_stride: 0,
            pad1: [0; 2],
            argb: ::core::ptr::null_mut::<uint32_t>(),
            argb_stride: 0,
            pad2: [0; 3],
            writer: None,
            custom_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            extra_info_type: 0,
            extra_info: ::core::ptr::null_mut::<uint8_t>(),
            stats: ::core::ptr::null_mut::<WebPAuxStats>(),
            error_code: VP8_ENC_OK,
            progress_hook: None,
            user_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pad3: [0; 3],
            pad4: ::core::ptr::null_mut::<uint8_t>(),
            pad5: ::core::ptr::null_mut::<uint8_t>(),
            pad6: [0; 8],
            memory_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            memory_argb_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pad7: [::core::ptr::null_mut::<::core::ffi::c_void>(); 2],
        },
    };
    let mut config_ll: WebPConfig = *config;
    let mut config_lossy: WebPConfig = *config;
    config_ll.lossless = 1 as ::core::ffi::c_int;
    config_lossy.lossless = 0 as ::core::ffi::c_int;
    (*enc).last_config_ = *config;
    (*enc).last_config_reversed_ = if (*config).lossless != 0 {
        config_lossy
    } else {
        config_ll
    };
    *frame_skipped = 0 as ::core::ffi::c_int;
    if SubFrameParamsInit(
        &raw mut dispose_none_params,
        1 as ::core::ffi::c_int,
        empty_rect_allowed_none,
    ) == 0
        || SubFrameParamsInit(
            &raw mut dispose_bg_params,
            0 as ::core::ffi::c_int,
            empty_rect_allowed_bg,
        ) == 0
    {
        return VP8_ENC_ERROR_INVALID_CONFIGURATION;
    }
    memset(
        &raw mut candidates as *mut Candidate as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[Candidate; 4]>() as size_t,
    );
    if GetSubRects(
        prev_canvas,
        curr_canvas,
        is_key_frame,
        is_first_frame,
        config_lossy.quality,
        &raw mut dispose_none_params,
    ) == 0
    {
        error_code = VP8_ENC_ERROR_INVALID_CONFIGURATION;
        current_block = 6836373173516796232;
    } else if consider_lossless != 0 && IsEmptyRect(&raw mut dispose_none_params.rect_ll_) != 0
        || consider_lossy != 0 && IsEmptyRect(&raw mut dispose_none_params.rect_lossy_) != 0
    {
        '_c2rust_label: {
            if empty_rect_allowed_none != 0 {
            } else {
                __assert_fail(
                    b"empty_rect_allowed_none\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1120 as ::core::ffi::c_uint,
                    b"WebPEncodingError SetFrame(WebPAnimEncoder *const, const WebPConfig *const, int, EncodedFrame *const, int *const)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        *frame_skipped = 1 as ::core::ffi::c_int;
        current_block = 16070219131484073282;
    } else {
        if dispose_bg_possible != 0 {
            let prev_canvas_disposed: *mut WebPPicture = &raw mut (*enc).prev_canvas_disposed_;
            WebPCopyPixels(
                prev_canvas as *const WebPPicture,
                prev_canvas_disposed as *mut WebPPicture,
            );
            DisposeFrameRectangle(
                WEBP_MUX_DISPOSE_BACKGROUND as ::core::ffi::c_int,
                &raw mut (*enc).prev_rect_,
                prev_canvas_disposed,
            );
            if GetSubRects(
                prev_canvas_disposed,
                curr_canvas,
                is_key_frame,
                is_first_frame,
                config_lossy.quality,
                &raw mut dispose_bg_params,
            ) == 0
            {
                error_code = VP8_ENC_ERROR_INVALID_CONFIGURATION;
                current_block = 6836373173516796232;
            } else {
                '_c2rust_label_0: {
                    if IsEmptyRect(&raw mut dispose_bg_params.rect_ll_) == 0 {
                    } else {
                        __assert_fail(
                            b"!IsEmptyRect(&dispose_bg_params.rect_ll_)\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            1138 as ::core::ffi::c_uint,
                            b"WebPEncodingError SetFrame(WebPAnimEncoder *const, const WebPConfig *const, int, EncodedFrame *const, int *const)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                };
                '_c2rust_label_1: {
                    if IsEmptyRect(&raw mut dispose_bg_params.rect_lossy_) == 0 {
                    } else {
                        __assert_fail(
                            b"!IsEmptyRect(&dispose_bg_params.rect_lossy_)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            1139 as ::core::ffi::c_uint,
                            b"WebPEncodingError SetFrame(WebPAnimEncoder *const, const WebPConfig *const, int, EncodedFrame *const, int *const)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                };
                if (*enc).options_.minimize_size != 0 {
                    dispose_bg_params.should_try_ = 1 as ::core::ffi::c_int;
                    dispose_none_params.should_try_ = 1 as ::core::ffi::c_int;
                } else if is_lossless != 0
                    && RectArea(&raw mut dispose_bg_params.rect_ll_)
                        < RectArea(&raw mut dispose_none_params.rect_ll_)
                    || is_lossless == 0
                        && RectArea(&raw mut dispose_bg_params.rect_lossy_)
                            < RectArea(&raw mut dispose_none_params.rect_lossy_)
                {
                    dispose_bg_params.should_try_ = 1 as ::core::ffi::c_int;
                    dispose_none_params.should_try_ = 0 as ::core::ffi::c_int;
                }
                current_block = 14359455889292382949;
            }
        } else {
            current_block = 14359455889292382949;
        }
        match current_block {
            6836373173516796232 => {}
            _ => {
                if dispose_none_params.should_try_ != 0 {
                    error_code = GenerateCandidates(
                        enc,
                        &raw mut candidates as *mut Candidate,
                        WEBP_MUX_DISPOSE_NONE,
                        is_lossless,
                        is_key_frame,
                        &raw mut dispose_none_params,
                        &raw mut config_ll,
                        &raw mut config_lossy,
                    );
                    if error_code as ::core::ffi::c_uint
                        != VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        current_block = 6836373173516796232;
                    } else {
                        current_block = 5494826135382683477;
                    }
                } else {
                    current_block = 5494826135382683477;
                }
                match current_block {
                    6836373173516796232 => {}
                    _ => {
                        if dispose_bg_params.should_try_ != 0 {
                            '_c2rust_label_2: {
                                if (*enc).is_first_frame_ == 0 {
                                } else {
                                    __assert_fail(
                                        b"!enc->is_first_frame_\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                        1163 as ::core::ffi::c_uint,
                                        b"WebPEncodingError SetFrame(WebPAnimEncoder *const, const WebPConfig *const, int, EncodedFrame *const, int *const)\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            '_c2rust_label_3: {
                                if dispose_bg_possible != 0 {
                                } else {
                                    __assert_fail(
                                        b"dispose_bg_possible\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                        1164 as ::core::ffi::c_uint,
                                        b"WebPEncodingError SetFrame(WebPAnimEncoder *const, const WebPConfig *const, int, EncodedFrame *const, int *const)\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            error_code = GenerateCandidates(
                                enc,
                                &raw mut candidates as *mut Candidate,
                                WEBP_MUX_DISPOSE_BACKGROUND,
                                is_lossless,
                                is_key_frame,
                                &raw mut dispose_bg_params,
                                &raw mut config_ll,
                                &raw mut config_lossy,
                            );
                            if error_code as ::core::ffi::c_uint
                                != VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                            {
                                current_block = 6836373173516796232;
                            } else {
                                current_block = 2604890879466389055;
                            }
                        } else {
                            current_block = 2604890879466389055;
                        }
                        match current_block {
                            6836373173516796232 => {}
                            _ => {
                                PickBestCandidate(
                                    enc,
                                    &raw mut candidates as *mut Candidate,
                                    is_key_frame,
                                    encoded_frame,
                                );
                                current_block = 16070219131484073282;
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        6836373173516796232 => {
            i = 0 as ::core::ffi::c_int;
            while i < CANDIDATE_COUNT as ::core::ffi::c_int {
                if candidates[i as usize].evaluate_ != 0 {
                    WebPMemoryWriterClear(
                        &raw mut (*(&raw mut candidates as *mut Candidate).offset(i as isize)).mem_,
                    );
                }
                i += 1;
            }
        }
        _ => {}
    }
    SubFrameParamsFree(&raw mut dispose_none_params);
    SubFrameParamsFree(&raw mut dispose_bg_params);
    return error_code;
}
unsafe extern "C" fn KeyFramePenalty(encoded_frame: *const EncodedFrame) -> int64_t {
    return ((*encoded_frame).key_frame_.bitstream.size as int64_t as size_t)
        .wrapping_sub((*encoded_frame).sub_frame_.bitstream.size) as int64_t;
}
unsafe extern "C" fn CacheFrame(
    enc: *mut WebPAnimEncoder,
    config: *const WebPConfig,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut frame_skipped: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut error_code: WebPEncodingError = VP8_ENC_OK;
    let position: size_t = (*enc).count_;
    let encoded_frame: *mut EncodedFrame = GetFrame(enc, position) as *mut EncodedFrame;
    (*enc).count_ = (*enc).count_.wrapping_add(1);
    if (*enc).is_first_frame_ != 0 {
        error_code = SetFrame(
            enc,
            config,
            1 as ::core::ffi::c_int,
            encoded_frame,
            &raw mut frame_skipped,
        );
        if error_code as ::core::ffi::c_uint
            != VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            current_block = 11813081686126766169;
        } else {
            '_c2rust_label: {
                if frame_skipped == 0 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"frame_skipped == 0\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        1208 as ::core::ffi::c_uint,
                        b"int CacheFrame(WebPAnimEncoder *const, const WebPConfig *const)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            '_c2rust_label_0: {
                if position == 0 as size_t && (*enc).count_ == 1 as size_t {
                } else {
                    __assert_fail(
                        b"position == 0 && enc->count_ == 1\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        1209 as ::core::ffi::c_uint,
                        b"int CacheFrame(WebPAnimEncoder *const, const WebPConfig *const)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            (*encoded_frame).is_key_frame_ = 1 as ::core::ffi::c_int;
            (*enc).flush_count_ = 0 as size_t;
            (*enc).count_since_key_frame_ = 0 as ::core::ffi::c_int;
            (*enc).prev_candidate_undecided_ = 0 as ::core::ffi::c_int;
            current_block = 17784502470059252271;
        }
    } else {
        (*enc).count_since_key_frame_ += 1;
        if (*enc).count_since_key_frame_ <= (*enc).options_.kmin {
            error_code = SetFrame(
                enc,
                config,
                0 as ::core::ffi::c_int,
                encoded_frame,
                &raw mut frame_skipped,
            );
            if error_code as ::core::ffi::c_uint
                != VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                current_block = 11813081686126766169;
            } else if frame_skipped != 0 {
                current_block = 12499188950238637545;
            } else {
                (*encoded_frame).is_key_frame_ = 0 as ::core::ffi::c_int;
                (*enc).flush_count_ = (*enc).count_.wrapping_sub(1 as size_t);
                (*enc).prev_candidate_undecided_ = 0 as ::core::ffi::c_int;
                current_block = 17784502470059252271;
            }
        } else {
            let mut curr_delta: int64_t = 0;
            let mut prev_rect_key: FrameRectangle = FrameRectangle {
                x_offset_: 0,
                y_offset_: 0,
                width_: 0,
                height_: 0,
            };
            let mut prev_rect_sub: FrameRectangle = FrameRectangle {
                x_offset_: 0,
                y_offset_: 0,
                width_: 0,
                height_: 0,
            };
            error_code = SetFrame(
                enc,
                config,
                0 as ::core::ffi::c_int,
                encoded_frame,
                &raw mut frame_skipped,
            );
            if error_code as ::core::ffi::c_uint
                != VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                current_block = 11813081686126766169;
            } else if frame_skipped != 0 {
                current_block = 12499188950238637545;
            } else {
                prev_rect_sub = (*enc).prev_rect_;
                error_code = SetFrame(
                    enc,
                    config,
                    1 as ::core::ffi::c_int,
                    encoded_frame,
                    &raw mut frame_skipped,
                );
                if error_code as ::core::ffi::c_uint
                    != VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    current_block = 11813081686126766169;
                } else {
                    '_c2rust_label_1: {
                        if frame_skipped == 0 as ::core::ffi::c_int {
                        } else {
                            __assert_fail(
                                b"frame_skipped == 0\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                1238 as ::core::ffi::c_uint,
                                b"int CacheFrame(WebPAnimEncoder *const, const WebPConfig *const)\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                            );
                        }
                    };
                    prev_rect_key = (*enc).prev_rect_;
                    curr_delta = KeyFramePenalty(encoded_frame);
                    if curr_delta <= (*enc).best_delta_ {
                        if (*enc).keyframe_ != KEYFRAME_NONE {
                            let old_keyframe: *mut EncodedFrame =
                                GetFrame(enc, (*enc).keyframe_ as size_t) as *mut EncodedFrame;
                            '_c2rust_label_2: {
                                if (*old_keyframe).is_key_frame_ != 0 {
                                } else {
                                    __assert_fail(
                                        b"old_keyframe->is_key_frame_\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                        1246 as ::core::ffi::c_uint,
                                        b"int CacheFrame(WebPAnimEncoder *const, const WebPConfig *const)\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            (*old_keyframe).is_key_frame_ = 0 as ::core::ffi::c_int;
                        }
                        (*encoded_frame).is_key_frame_ = 1 as ::core::ffi::c_int;
                        (*enc).prev_candidate_undecided_ = 1 as ::core::ffi::c_int;
                        (*enc).keyframe_ = position as ::core::ffi::c_int;
                        (*enc).best_delta_ = curr_delta;
                        (*enc).flush_count_ = (*enc).count_.wrapping_sub(1 as size_t);
                    } else {
                        (*encoded_frame).is_key_frame_ = 0 as ::core::ffi::c_int;
                        (*enc).prev_candidate_undecided_ = 0 as ::core::ffi::c_int;
                    }
                    if (*enc).count_since_key_frame_ >= (*enc).options_.kmax {
                        (*enc).flush_count_ = (*enc).count_.wrapping_sub(1 as size_t);
                        (*enc).count_since_key_frame_ = 0 as ::core::ffi::c_int;
                        (*enc).keyframe_ = KEYFRAME_NONE;
                        (*enc).best_delta_ = DELTA_INFINITY as int64_t;
                    }
                    if (*enc).prev_candidate_undecided_ == 0 {
                        (*enc).prev_rect_ = if (*encoded_frame).is_key_frame_ != 0 {
                            prev_rect_key
                        } else {
                            prev_rect_sub
                        };
                    }
                    current_block = 17784502470059252271;
                }
            }
        }
    }
    match current_block {
        17784502470059252271 => {
            WebPCopyPixels((*enc).curr_canvas_, &raw mut (*enc).prev_canvas_);
            (*enc).is_first_frame_ = 0 as ::core::ffi::c_int;
            current_block = 12499188950238637545;
        }
        _ => {}
    }
    match current_block {
        12499188950238637545 => {
            ok = 1 as ::core::ffi::c_int;
            (*enc).in_frame_count_ = (*enc).in_frame_count_.wrapping_add(1);
        }
        _ => {}
    }
    if ok == 0 || frame_skipped != 0 {
        FrameRelease(encoded_frame);
        (*enc).count_ = (*enc).count_.wrapping_sub(1);
        if (*enc).is_first_frame_ == 0 {
            (*enc).count_since_key_frame_ -= 1;
        }
        if ok == 0 {
            MarkError2(
                enc,
                b"ERROR adding frame. WebPEncodingError\0" as *const u8
                    as *const ::core::ffi::c_char,
                error_code as ::core::ffi::c_int,
            );
        }
    }
    (*(*enc).curr_canvas_).error_code = error_code;
    '_c2rust_label_3: {
        if ok != 0
            || error_code as ::core::ffi::c_uint
                != VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"ok || error_code != VP8_ENC_OK\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1292 as ::core::ffi::c_uint,
                b"int CacheFrame(WebPAnimEncoder *const, const WebPConfig *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return ok;
}
unsafe extern "C" fn FlushFrames(enc: *mut WebPAnimEncoder) -> ::core::ffi::c_int {
    while (*enc).flush_count_ > 0 as size_t {
        let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
        let curr: *mut EncodedFrame = GetFrame(enc, 0 as size_t) as *mut EncodedFrame;
        let info: *const WebPMuxFrameInfo = if (*curr).is_key_frame_ != 0 {
            &raw mut (*curr).key_frame_
        } else {
            &raw mut (*curr).sub_frame_
        };
        '_c2rust_label: {
            if !(*enc).mux_.is_null() {
            } else {
                __assert_fail(
                    b"enc->mux_ != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1302 as ::core::ffi::c_uint,
                    b"int FlushFrames(WebPAnimEncoder *const)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        err = WebPMuxPushFrame((*enc).mux_, info, 1 as ::core::ffi::c_int);
        if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
            MarkError2(
                enc,
                b"ERROR adding frame. WebPMuxError\0" as *const u8 as *const ::core::ffi::c_char,
                err as ::core::ffi::c_int,
            );
            return 0 as ::core::ffi::c_int;
        }
        if (*enc).options_.verbose != 0 {
            fprintf(
                stderr,
                b"INFO: Added frame. offset:%d,%d dispose:%d blend:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*info).x_offset,
                (*info).y_offset,
                (*info).dispose_method as ::core::ffi::c_uint,
                (*info).blend_method as ::core::ffi::c_uint,
            );
        }
        (*enc).out_frame_count_ = (*enc).out_frame_count_.wrapping_add(1);
        FrameRelease(curr);
        (*enc).start_ = (*enc).start_.wrapping_add(1);
        (*enc).flush_count_ = (*enc).flush_count_.wrapping_sub(1);
        (*enc).count_ = (*enc).count_.wrapping_sub(1);
        if (*enc).keyframe_ != KEYFRAME_NONE {
            (*enc).keyframe_ -= 1;
        }
    }
    if (*enc).count_ == 1 as size_t && (*enc).start_ != 0 as size_t {
        let enc_start_tmp: ::core::ffi::c_int = (*enc).start_ as ::core::ffi::c_int;
        let mut temp: EncodedFrame = *(*enc)
            .encoded_frames_
            .offset(0 as ::core::ffi::c_int as isize);
        *(*enc)
            .encoded_frames_
            .offset(0 as ::core::ffi::c_int as isize) =
            *(*enc).encoded_frames_.offset(enc_start_tmp as isize);
        *(*enc).encoded_frames_.offset(enc_start_tmp as isize) = temp;
        FrameRelease((*enc).encoded_frames_.offset(enc_start_tmp as isize) as *mut EncodedFrame);
        (*enc).start_ = 0 as size_t;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimEncoderAdd(
    mut enc: *mut WebPAnimEncoder,
    mut frame: *mut WebPPicture,
    mut timestamp: ::core::ffi::c_int,
    mut encoder_config: *const WebPConfig,
) -> ::core::ffi::c_int {
    let mut config: WebPConfig = WebPConfig {
        lossless: 0,
        quality: 0.,
        method: 0,
        image_hint: WEBP_HINT_DEFAULT,
        target_size: 0,
        target_PSNR: 0.,
        segments: 0,
        sns_strength: 0,
        filter_strength: 0,
        filter_sharpness: 0,
        filter_type: 0,
        autofilter: 0,
        alpha_compression: 0,
        alpha_filtering: 0,
        alpha_quality: 0,
        pass: 0,
        show_compressed: 0,
        preprocessing: 0,
        partitions: 0,
        partition_limit: 0,
        emulate_jpeg_size: 0,
        thread_level: 0,
        low_memory: 0,
        near_lossless: 0,
        exact: 0,
        use_delta_palette: 0,
        use_sharp_yuv: 0,
        qmin: 0,
        qmax: 0,
    };
    let mut ok: ::core::ffi::c_int = 0;
    if enc.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    MarkNoError(enc);
    if (*enc).is_first_frame_ == 0 {
        let prev_frame_duration: uint32_t =
            (timestamp as uint32_t).wrapping_sub((*enc).prev_timestamp_ as uint32_t);
        if prev_frame_duration >= MAX_DURATION as uint32_t {
            if !frame.is_null() {
                (*frame).error_code = VP8_ENC_ERROR_INVALID_CONFIGURATION;
            }
            MarkError(
                enc,
                b"ERROR adding frame: timestamps must be non-decreasing\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
        if IncreasePreviousDuration(enc, prev_frame_duration as ::core::ffi::c_int) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        if (*enc).count_ == (*enc).size_ && FlushFrames(enc) == 0 {
            return 0 as ::core::ffi::c_int;
        }
    } else {
        (*enc).first_timestamp_ = timestamp;
    }
    if frame.is_null() {
        (*enc).got_null_frame_ = 1 as ::core::ffi::c_int;
        (*enc).prev_timestamp_ = timestamp;
        return 1 as ::core::ffi::c_int;
    }
    if (*frame).width != (*enc).canvas_width_ || (*frame).height != (*enc).canvas_height_ {
        (*frame).error_code = VP8_ENC_ERROR_INVALID_CONFIGURATION;
        MarkError(
            enc,
            b"ERROR adding frame: Invalid frame dimensions\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if (*frame).use_argb == 0 {
        if (*enc).options_.verbose != 0 {
            fprintf(
                stderr,
                b"WARNING: Converting frame from YUV(A) to ARGB format; this incurs a small loss.\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if WebPPictureYUVAToARGB(frame) == 0 {
            MarkError(
                enc,
                b"ERROR converting frame from YUV(A) to ARGB\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
    }
    if !encoder_config.is_null() {
        if WebPValidateConfig(encoder_config) == 0 {
            MarkError(
                enc,
                b"ERROR adding frame: Invalid WebPConfig\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
        config = *encoder_config;
    } else {
        WebPConfigInit(&raw mut config);
        config.lossless = 1 as ::core::ffi::c_int;
    }
    '_c2rust_label: {
        if (*enc).curr_canvas_.is_null() {
        } else {
            __assert_fail(
                b"enc->curr_canvas_ == NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1404 as ::core::ffi::c_uint,
                b"int WebPAnimEncoderAdd(WebPAnimEncoder *, WebPPicture *, int, const WebPConfig *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    (*enc).curr_canvas_ = frame;
    '_c2rust_label_0: {
        if (*enc).curr_canvas_copy_modified_ == 1 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"enc->curr_canvas_copy_modified_ == 1\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1406 as ::core::ffi::c_uint,
                b"int WebPAnimEncoderAdd(WebPAnimEncoder *, WebPPicture *, int, const WebPConfig *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    CopyCurrentCanvas(enc);
    ok = (CacheFrame(enc, &raw mut config) != 0 && FlushFrames(enc) != 0) as ::core::ffi::c_int;
    (*enc).curr_canvas_ = ::core::ptr::null_mut::<WebPPicture>();
    (*enc).curr_canvas_copy_modified_ = 1 as ::core::ffi::c_int;
    if ok != 0 {
        (*enc).prev_timestamp_ = timestamp;
    }
    return ok;
}
unsafe extern "C" fn DecodeFrameOntoCanvas(
    frame: *const WebPMuxFrameInfo,
    canvas: *mut WebPPicture,
) -> ::core::ffi::c_int {
    let image: *const WebPData = &raw const (*frame).bitstream;
    let mut sub_image: WebPPicture = WebPPicture {
        use_argb: 0,
        colorspace: WEBP_YUV420,
        width: 0,
        height: 0,
        y: ::core::ptr::null_mut::<uint8_t>(),
        u: ::core::ptr::null_mut::<uint8_t>(),
        v: ::core::ptr::null_mut::<uint8_t>(),
        y_stride: 0,
        uv_stride: 0,
        a: ::core::ptr::null_mut::<uint8_t>(),
        a_stride: 0,
        pad1: [0; 2],
        argb: ::core::ptr::null_mut::<uint32_t>(),
        argb_stride: 0,
        pad2: [0; 3],
        writer: None,
        custom_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        extra_info_type: 0,
        extra_info: ::core::ptr::null_mut::<uint8_t>(),
        stats: ::core::ptr::null_mut::<WebPAuxStats>(),
        error_code: VP8_ENC_OK,
        progress_hook: None,
        user_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pad3: [0; 3],
        pad4: ::core::ptr::null_mut::<uint8_t>(),
        pad5: ::core::ptr::null_mut::<uint8_t>(),
        pad6: [0; 8],
        memory_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        memory_argb_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pad7: [::core::ptr::null_mut::<::core::ffi::c_void>(); 2],
    };
    let mut config: WebPDecoderConfig = WebPDecoderConfig {
        input: WebPBitstreamFeatures {
            width: 0,
            height: 0,
            has_alpha: 0,
            has_animation: 0,
            format: 0,
            pad: [0; 5],
        },
        output: WebPDecBuffer {
            colorspace: MODE_RGB,
            width: 0,
            height: 0,
            is_external_memory: 0,
            u: C2RustUnnamed {
                RGBA: WebPRGBABuffer {
                    rgba: ::core::ptr::null_mut::<uint8_t>(),
                    stride: 0,
                    size: 0,
                },
            },
            pad: [0; 4],
            private_memory: ::core::ptr::null_mut::<uint8_t>(),
        },
        options: WebPDecoderOptions {
            bypass_filtering: 0,
            no_fancy_upsampling: 0,
            use_cropping: 0,
            crop_left: 0,
            crop_top: 0,
            crop_width: 0,
            crop_height: 0,
            use_scaling: 0,
            scaled_width: 0,
            scaled_height: 0,
            use_threads: 0,
            dithering_strength: 0,
            flip: 0,
            alpha_dithering_strength: 0,
            pad: [0; 5],
        },
    };
    WebPInitDecoderConfig(&raw mut config);
    WebPUtilClearPic(canvas, ::core::ptr::null::<FrameRectangle>());
    if WebPGetFeatures((*image).bytes, (*image).size, &raw mut config.input) as ::core::ffi::c_uint
        != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if WebPPictureView(
        canvas,
        (*frame).x_offset,
        (*frame).y_offset,
        config.input.width,
        config.input.height,
        &raw mut sub_image,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    config.output.is_external_memory = 1 as ::core::ffi::c_int;
    config.output.colorspace = MODE_BGRA;
    config.output.u.RGBA.rgba = sub_image.argb as *mut uint8_t;
    config.output.u.RGBA.stride = sub_image.argb_stride * 4 as ::core::ffi::c_int;
    config.output.u.RGBA.size = (config.output.u.RGBA.stride * sub_image.height) as size_t;
    if WebPDecode((*image).bytes, (*image).size, &raw mut config) as ::core::ffi::c_uint
        != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn FrameToFullCanvas(
    enc: *mut WebPAnimEncoder,
    frame: *const WebPMuxFrameInfo,
    full_image: *mut WebPData,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let canvas_buf: *mut WebPPicture = &raw mut (*enc).curr_canvas_copy_;
    let mut mem1: WebPMemoryWriter = WebPMemoryWriter {
        mem: ::core::ptr::null_mut::<uint8_t>(),
        size: 0,
        max_size: 0,
        pad: [0; 1],
    };
    let mut mem2: WebPMemoryWriter = WebPMemoryWriter {
        mem: ::core::ptr::null_mut::<uint8_t>(),
        size: 0,
        max_size: 0,
        pad: [0; 1],
    };
    WebPMemoryWriterInit(&raw mut mem1);
    WebPMemoryWriterInit(&raw mut mem2);
    if !(DecodeFrameOntoCanvas(frame, canvas_buf) == 0) {
        if !(EncodeFrame(&raw mut (*enc).last_config_, canvas_buf, &raw mut mem1) == 0) {
            GetEncodedData(&raw mut mem1, full_image);
            if (*enc).options_.allow_mixed != 0 {
                if EncodeFrame(
                    &raw mut (*enc).last_config_reversed_,
                    canvas_buf,
                    &raw mut mem2,
                ) == 0
                {
                    current_block = 7886032326493456079;
                } else {
                    if mem2.size < mem1.size {
                        GetEncodedData(&raw mut mem2, full_image);
                        WebPMemoryWriterClear(&raw mut mem1);
                    } else {
                        WebPMemoryWriterClear(&raw mut mem2);
                    }
                    current_block = 7746791466490516765;
                }
            } else {
                current_block = 7746791466490516765;
            }
            match current_block {
                7886032326493456079 => {}
                _ => return 1 as ::core::ffi::c_int,
            }
        }
    }
    WebPMemoryWriterClear(&raw mut mem1);
    WebPMemoryWriterClear(&raw mut mem2);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn OptimizeSingleFrame(
    enc: *mut WebPAnimEncoder,
    webp_data: *mut WebPData,
) -> WebPMuxError {
    let mut err: WebPMuxError = WEBP_MUX_OK;
    let mut canvas_width: ::core::ffi::c_int = 0;
    let mut canvas_height: ::core::ffi::c_int = 0;
    let mut frame: WebPMuxFrameInfo = WebPMuxFrameInfo {
        bitstream: WebPData {
            bytes: ::core::ptr::null::<uint8_t>(),
            size: 0,
        },
        x_offset: 0,
        y_offset: 0,
        duration: 0,
        id: WEBP_CHUNK_VP8X,
        dispose_method: WEBP_MUX_DISPOSE_NONE,
        blend_method: WEBP_MUX_BLEND,
        pad: [0; 1],
    };
    let mut full_image: WebPData = WebPData {
        bytes: ::core::ptr::null::<uint8_t>(),
        size: 0,
    };
    let mut webp_data2: WebPData = WebPData {
        bytes: ::core::ptr::null::<uint8_t>(),
        size: 0,
    };
    let mux: *mut WebPMux = WebPMuxCreate(webp_data, 0 as ::core::ffi::c_int) as *mut WebPMux;
    if mux.is_null() {
        return WEBP_MUX_BAD_DATA;
    }
    '_c2rust_label: {
        if (*enc).out_frame_count_ == 1 as size_t {
        } else {
            __assert_fail(
                b"enc->out_frame_count_ == 1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/anim_encode.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1490 as ::core::ffi::c_uint,
                b"WebPMuxError OptimizeSingleFrame(WebPAnimEncoder *const, WebPData *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    WebPDataInit(&raw mut frame.bitstream);
    WebPDataInit(&raw mut full_image);
    WebPDataInit(&raw mut webp_data2);
    err = WebPMuxGetFrame(mux, 1 as uint32_t, &raw mut frame);
    if !(err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int) {
        if !(frame.id as ::core::ffi::c_uint
            != WEBP_CHUNK_ANMF as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            err = WebPMuxGetCanvasSize(mux, &raw mut canvas_width, &raw mut canvas_height);
            if !(err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int) {
                if FrameToFullCanvas(enc, &raw mut frame, &raw mut full_image) == 0 {
                    err = WEBP_MUX_BAD_DATA;
                } else {
                    err = WebPMuxSetImage(mux, &raw mut full_image, 1 as ::core::ffi::c_int);
                    if !(err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int) {
                        err = WebPMuxAssemble(mux, &raw mut webp_data2);
                        if !(err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int) {
                            if webp_data2.size < (*webp_data).size {
                                WebPDataClear(webp_data);
                                *webp_data = webp_data2;
                                WebPDataInit(&raw mut webp_data2);
                            }
                        }
                    }
                }
            }
        }
    }
    WebPDataClear(&raw mut frame.bitstream);
    WebPDataClear(&raw mut full_image);
    WebPMuxDelete(mux);
    WebPDataClear(&raw mut webp_data2);
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimEncoderAssemble(
    mut enc: *mut WebPAnimEncoder,
    mut webp_data: *mut WebPData,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut mux: *mut WebPMux = ::core::ptr::null_mut::<WebPMux>();
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    if enc.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    MarkNoError(enc);
    if webp_data.is_null() {
        MarkError(
            enc,
            b"ERROR assembling: NULL input\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if (*enc).in_frame_count_ == 0 as size_t {
        MarkError(
            enc,
            b"ERROR: No frames to assemble\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if (*enc).got_null_frame_ == 0
        && (*enc).in_frame_count_ > 1 as size_t
        && (*enc).count_ > 0 as size_t
    {
        let delta_time: ::core::ffi::c_double = ((*enc).prev_timestamp_ as uint32_t)
            .wrapping_sub((*enc).first_timestamp_ as uint32_t)
            as ::core::ffi::c_double;
        let average_duration: ::core::ffi::c_int = (delta_time
            / (*enc).in_frame_count_.wrapping_sub(1 as size_t) as ::core::ffi::c_double)
            as ::core::ffi::c_int;
        if IncreasePreviousDuration(enc, average_duration) == 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    (*enc).flush_count_ = (*enc).count_;
    if FlushFrames(enc) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    mux = (*enc).mux_;
    err = WebPMuxSetCanvasSize(mux, (*enc).canvas_width_, (*enc).canvas_height_);
    if !(err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int) {
        err = WebPMuxSetAnimationParams(mux, &raw const (*enc).options_.anim_params);
        if !(err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int) {
            err = WebPMuxAssemble(mux, webp_data);
            if !(err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int) {
                if (*enc).out_frame_count_ == 1 as size_t {
                    err = OptimizeSingleFrame(enc, webp_data);
                    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
                        current_block = 16082653657082592682;
                    } else {
                        current_block = 11194104282611034094;
                    }
                } else {
                    current_block = 11194104282611034094;
                }
                match current_block {
                    16082653657082592682 => {}
                    _ => return 1 as ::core::ffi::c_int,
                }
            }
        }
    }
    MarkError2(
        enc,
        b"ERROR assembling WebP\0" as *const u8 as *const ::core::ffi::c_char,
        err as ::core::ffi::c_int,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimEncoderGetError(
    mut enc: *mut WebPAnimEncoder,
) -> *const ::core::ffi::c_char {
    if enc.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return &raw mut (*enc).error_str_ as *mut ::core::ffi::c_char;
}
pub const WEBP_ENCODER_ABI_VERSION: ::core::ffi::c_int = 0x20f as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn WebPConfigInit(mut config: *mut WebPConfig) -> ::core::ffi::c_int {
    return WebPConfigInitInternal(
        config,
        WEBP_PRESET_DEFAULT,
        75.0f32,
        WEBP_ENCODER_ABI_VERSION,
    );
}
#[inline]
unsafe extern "C" fn WebPPictureInit(mut picture: *mut WebPPicture) -> ::core::ffi::c_int {
    return WebPPictureInitInternal(picture, WEBP_ENCODER_ABI_VERSION);
}
pub const WEBP_DECODER_ABI_VERSION: ::core::ffi::c_int = 0x209 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn WebPGetFeatures(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut features: *mut WebPBitstreamFeatures,
) -> VP8StatusCode {
    return WebPGetFeaturesInternal(data, data_size, features, WEBP_DECODER_ABI_VERSION);
}
#[inline]
unsafe extern "C" fn WebPInitDecoderConfig(
    mut config: *mut WebPDecoderConfig,
) -> ::core::ffi::c_int {
    return WebPInitDecoderConfigInternal(config, WEBP_DECODER_ABI_VERSION);
}
pub const MAX_IMAGE_AREA: ::core::ffi::c_ulonglong =
    (1 as ::core::ffi::c_ulonglong) << 32 as ::core::ffi::c_int;
pub const MAX_DURATION: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
