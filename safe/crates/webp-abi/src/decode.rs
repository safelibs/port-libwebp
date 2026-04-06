use core::ffi::c_void;

pub const WEBP_DECODER_ABI_VERSION: i32 = 0x0209;

c_enum! {
    pub struct WEBP_CSP_MODE(i32) {
        const MODE_RGB = 0;
        const MODE_RGBA = 1;
        const MODE_BGR = 2;
        const MODE_BGRA = 3;
        const MODE_ARGB = 4;
        const MODE_RGBA_4444 = 5;
        const MODE_RGB_565 = 6;
        const MODE_rgbA = 7;
        const MODE_bgrA = 8;
        const MODE_Argb = 9;
        const MODE_rgbA_4444 = 10;
        const MODE_YUV = 11;
        const MODE_YUVA = 12;
        const MODE_LAST = 13;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WebPRGBABuffer {
    pub rgba: *mut u8,
    pub stride: i32,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WebPYUVABuffer {
    pub y: *mut u8,
    pub u: *mut u8,
    pub v: *mut u8,
    pub a: *mut u8,
    pub y_stride: i32,
    pub u_stride: i32,
    pub v_stride: i32,
    pub a_stride: i32,
    pub y_size: usize,
    pub u_size: usize,
    pub v_size: usize,
    pub a_size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union WebPDecBufferUnion {
    pub RGBA: WebPRGBABuffer,
    pub YUVA: WebPYUVABuffer,
}

impl Default for WebPDecBufferUnion {
    fn default() -> Self {
        Self {
            RGBA: WebPRGBABuffer::default(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct WebPDecBuffer {
    pub colorspace: WEBP_CSP_MODE,
    pub width: i32,
    pub height: i32,
    pub is_external_memory: i32,
    pub u: WebPDecBufferUnion,
    pub pad: [u32; 4],
    pub private_memory: *mut u8,
}

c_enum! {
    pub struct VP8StatusCode(i32) {
        const VP8_STATUS_OK = 0;
        const VP8_STATUS_OUT_OF_MEMORY = 1;
        const VP8_STATUS_INVALID_PARAM = 2;
        const VP8_STATUS_BITSTREAM_ERROR = 3;
        const VP8_STATUS_UNSUPPORTED_FEATURE = 4;
        const VP8_STATUS_SUSPENDED = 5;
        const VP8_STATUS_USER_ABORT = 6;
        const VP8_STATUS_NOT_ENOUGH_DATA = 7;
    }
}

opaque_type! { pub struct WebPIDecoder; }

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WebPBitstreamFeatures {
    pub width: i32,
    pub height: i32,
    pub has_alpha: i32,
    pub has_animation: i32,
    pub format: i32,
    pub pad: [u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WebPDecoderOptions {
    pub bypass_filtering: i32,
    pub no_fancy_upsampling: i32,
    pub use_cropping: i32,
    pub crop_left: i32,
    pub crop_top: i32,
    pub crop_width: i32,
    pub crop_height: i32,
    pub use_scaling: i32,
    pub scaled_width: i32,
    pub scaled_height: i32,
    pub use_threads: i32,
    pub dithering_strength: i32,
    pub flip: i32,
    pub alpha_dithering_strength: i32,
    pub pad: [u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct WebPDecoderConfig {
    pub input: WebPBitstreamFeatures,
    pub output: WebPDecBuffer,
    pub options: WebPDecoderOptions,
}

pub type WebPIDecodeFn = Option<
    unsafe extern "C" fn(
        data: *const u8,
        data_size: usize,
        config: *mut WebPDecoderConfig,
    ) -> *mut WebPIDecoder,
>;

pub type WebPDecodeFn = Option<
    unsafe extern "C" fn(
        data: *const u8,
        data_size: usize,
        config: *mut WebPDecoderConfig,
    ) -> VP8StatusCode,
>;

pub type WebPIDecodedAreaFn = Option<
    unsafe extern "C" fn(
        idec: *const WebPIDecoder,
        left: *mut i32,
        top: *mut i32,
        width: *mut i32,
        height: *mut i32,
    ) -> *const WebPDecBuffer,
>;

pub type WebPDecodeBufferPtr = *mut c_void;
