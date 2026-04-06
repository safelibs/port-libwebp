use core::ffi::c_void;

pub const WEBP_ENCODER_ABI_VERSION: i32 = 0x020f;
pub const WEBP_MAX_DIMENSION: i32 = 16_383;

c_enum! {
    pub struct WebPImageHint(i32) {
        const WEBP_HINT_DEFAULT = 0;
        const WEBP_HINT_PICTURE = 1;
        const WEBP_HINT_PHOTO = 2;
        const WEBP_HINT_GRAPH = 3;
        const WEBP_HINT_LAST = 4;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WebPConfig {
    pub lossless: i32,
    pub quality: f32,
    pub method: i32,
    pub image_hint: WebPImageHint,
    pub target_size: i32,
    pub target_PSNR: f32,
    pub segments: i32,
    pub sns_strength: i32,
    pub filter_strength: i32,
    pub filter_sharpness: i32,
    pub filter_type: i32,
    pub autofilter: i32,
    pub alpha_compression: i32,
    pub alpha_filtering: i32,
    pub alpha_quality: i32,
    pub pass: i32,
    pub show_compressed: i32,
    pub preprocessing: i32,
    pub partitions: i32,
    pub partition_limit: i32,
    pub emulate_jpeg_size: i32,
    pub thread_level: i32,
    pub low_memory: i32,
    pub near_lossless: i32,
    pub exact: i32,
    pub use_delta_palette: i32,
    pub use_sharp_yuv: i32,
    pub qmin: i32,
    pub qmax: i32,
}

c_enum! {
    pub struct WebPPreset(i32) {
        const WEBP_PRESET_DEFAULT = 0;
        const WEBP_PRESET_PICTURE = 1;
        const WEBP_PRESET_PHOTO = 2;
        const WEBP_PRESET_DRAWING = 3;
        const WEBP_PRESET_ICON = 4;
        const WEBP_PRESET_TEXT = 5;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WebPAuxStats {
    pub coded_size: i32,
    pub PSNR: [f32; 5],
    pub block_count: [i32; 3],
    pub header_bytes: [i32; 2],
    pub residual_bytes: [[i32; 4]; 3],
    pub segment_size: [i32; 4],
    pub segment_quant: [i32; 4],
    pub segment_level: [i32; 4],
    pub alpha_data_size: i32,
    pub layer_data_size: i32,
    pub lossless_features: u32,
    pub histogram_bits: i32,
    pub transform_bits: i32,
    pub cache_bits: i32,
    pub palette_size: i32,
    pub lossless_size: i32,
    pub lossless_hdr_size: i32,
    pub lossless_data_size: i32,
    pub pad: [u32; 2],
}

pub type WebPWriterFunction = Option<
    unsafe extern "C" fn(data: *const u8, data_size: usize, picture: *const WebPPicture) -> i32,
>;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WebPMemoryWriter {
    pub mem: *mut u8,
    pub size: usize,
    pub max_size: usize,
    pub pad: [u32; 1],
}

pub type WebPProgressHook =
    Option<unsafe extern "C" fn(percent: i32, picture: *const WebPPicture) -> i32>;

c_enum! {
    pub struct WebPEncCSP(i32) {
        const WEBP_YUV420 = 0;
        const WEBP_YUV420A = 4;
        const WEBP_CSP_UV_MASK = 3;
        const WEBP_CSP_ALPHA_BIT = 4;
    }
}

c_enum! {
    pub struct WebPEncodingError(i32) {
        const VP8_ENC_OK = 0;
        const VP8_ENC_ERROR_OUT_OF_MEMORY = 1;
        const VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY = 2;
        const VP8_ENC_ERROR_NULL_PARAMETER = 3;
        const VP8_ENC_ERROR_INVALID_CONFIGURATION = 4;
        const VP8_ENC_ERROR_BAD_DIMENSION = 5;
        const VP8_ENC_ERROR_PARTITION0_OVERFLOW = 6;
        const VP8_ENC_ERROR_PARTITION_OVERFLOW = 7;
        const VP8_ENC_ERROR_BAD_WRITE = 8;
        const VP8_ENC_ERROR_FILE_TOO_BIG = 9;
        const VP8_ENC_ERROR_USER_ABORT = 10;
        const VP8_ENC_ERROR_LAST = 11;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WebPPicture {
    pub use_argb: i32,
    pub colorspace: WebPEncCSP,
    pub width: i32,
    pub height: i32,
    pub y: *mut u8,
    pub u: *mut u8,
    pub v: *mut u8,
    pub y_stride: i32,
    pub uv_stride: i32,
    pub a: *mut u8,
    pub a_stride: i32,
    pub pad1: [u32; 2],
    pub argb: *mut u32,
    pub argb_stride: i32,
    pub pad2: [u32; 3],
    pub writer: WebPWriterFunction,
    pub custom_ptr: *mut c_void,
    pub extra_info_type: i32,
    pub extra_info: *mut u8,
    pub stats: *mut WebPAuxStats,
    pub error_code: WebPEncodingError,
    pub progress_hook: WebPProgressHook,
    pub user_data: *mut c_void,
    pub pad3: [u32; 3],
    pub pad4: *mut u8,
    pub pad5: *mut u8,
    pub pad6: [u32; 8],
    pub memory_: *mut c_void,
    pub memory_argb_: *mut c_void,
    pub pad7: [*mut c_void; 2],
}
