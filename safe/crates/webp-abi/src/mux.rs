use crate::{WebPConfig, WebPData, WebPMuxAnimBlend, WebPMuxAnimDispose};

pub const WEBP_MUX_ABI_VERSION: i32 = 0x0108;

opaque_type! { pub struct WebPMux; }

c_enum! {
    pub struct WebPMuxError(i32) {
        const WEBP_MUX_OK = 1;
        const WEBP_MUX_NOT_FOUND = 0;
        const WEBP_MUX_INVALID_ARGUMENT = -1;
        const WEBP_MUX_BAD_DATA = -2;
        const WEBP_MUX_MEMORY_ERROR = -3;
        const WEBP_MUX_NOT_ENOUGH_DATA = -4;
    }
}

c_enum! {
    pub struct WebPChunkId(i32) {
        const WEBP_CHUNK_VP8X = 0;
        const WEBP_CHUNK_ICCP = 1;
        const WEBP_CHUNK_ANIM = 2;
        const WEBP_CHUNK_ANMF = 3;
        const WEBP_CHUNK_DEPRECATED = 4;
        const WEBP_CHUNK_ALPHA = 5;
        const WEBP_CHUNK_IMAGE = 6;
        const WEBP_CHUNK_EXIF = 7;
        const WEBP_CHUNK_XMP = 8;
        const WEBP_CHUNK_UNKNOWN = 9;
        const WEBP_CHUNK_NIL = 10;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WebPMuxFrameInfo {
    pub bitstream: WebPData,
    pub x_offset: i32,
    pub y_offset: i32,
    pub duration: i32,
    pub id: WebPChunkId,
    pub dispose_method: WebPMuxAnimDispose,
    pub blend_method: WebPMuxAnimBlend,
    pub pad: [u32; 1],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WebPMuxAnimParams {
    pub bgcolor: u32,
    pub loop_count: i32,
}

opaque_type! { pub struct WebPAnimEncoder; }

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WebPAnimEncoderOptions {
    pub anim_params: WebPMuxAnimParams,
    pub minimize_size: i32,
    pub kmin: i32,
    pub kmax: i32,
    pub allow_mixed: i32,
    pub verbose: i32,
    pub padding: [u32; 4],
}

pub type WebPAnimEncoderAddFn = Option<
    unsafe extern "C" fn(
        enc: *mut WebPAnimEncoder,
        frame: *mut crate::WebPPicture,
        timestamp_ms: i32,
        config: *const WebPConfig,
    ) -> i32,
>;
