use core::ffi::c_void;

use crate::{WebPData, WebPMuxAnimBlend, WebPMuxAnimDispose, WEBP_CSP_MODE};

pub const WEBP_DEMUX_ABI_VERSION: i32 = 0x0107;

c_enum! {
    pub struct WebPDemuxState(i32) {
        const WEBP_DEMUX_PARSE_ERROR = -1;
        const WEBP_DEMUX_PARSING_HEADER = 0;
        const WEBP_DEMUX_PARSED_HEADER = 1;
        const WEBP_DEMUX_DONE = 2;
    }
}

opaque_type! { pub struct WebPDemuxer; }

c_enum! {
    pub struct WebPFormatFeature(i32) {
        const WEBP_FF_FORMAT_FLAGS = 0;
        const WEBP_FF_CANVAS_WIDTH = 1;
        const WEBP_FF_CANVAS_HEIGHT = 2;
        const WEBP_FF_LOOP_COUNT = 3;
        const WEBP_FF_BACKGROUND_COLOR = 4;
        const WEBP_FF_FRAME_COUNT = 5;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WebPIterator {
    pub frame_num: i32,
    pub num_frames: i32,
    pub x_offset: i32,
    pub y_offset: i32,
    pub width: i32,
    pub height: i32,
    pub duration: i32,
    pub dispose_method: WebPMuxAnimDispose,
    pub complete: i32,
    pub fragment: WebPData,
    pub has_alpha: i32,
    pub blend_method: WebPMuxAnimBlend,
    pub pad: [u32; 2],
    pub private_: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WebPChunkIterator {
    pub chunk_num: i32,
    pub num_chunks: i32,
    pub chunk: WebPData,
    pub pad: [u32; 6],
    pub private_: *mut c_void,
}

opaque_type! { pub struct WebPAnimDecoder; }

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WebPAnimDecoderOptions {
    pub color_mode: WEBP_CSP_MODE,
    pub use_threads: i32,
    pub padding: [u32; 7],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WebPAnimInfo {
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub loop_count: u32,
    pub bgcolor: u32,
    pub frame_count: u32,
    pub pad: [u32; 4],
}
