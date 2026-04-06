extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
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
    fn WebPInitDecBufferInternal(
        _: *mut WebPDecBuffer,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPFreeDecBuffer(buffer: *mut WebPDecBuffer);
    fn WebPGetFeaturesInternal(
        _: *const uint8_t,
        _: size_t,
        _: *mut WebPBitstreamFeatures,
        _: ::core::ffi::c_int,
    ) -> VP8StatusCode;
    fn VP8InitIoInternal(_: *mut VP8Io, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn VP8New() -> *mut VP8Decoder;
    fn VP8GetHeaders(dec: *mut VP8Decoder, io: *mut VP8Io) -> ::core::ffi::c_int;
    fn VP8Delete(dec: *mut VP8Decoder);
    fn VP8GetInfo(
        data: *const uint8_t,
        data_size: size_t,
        chunk_size: size_t,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPResetDecParams(params: *mut WebPDecParams);
    fn WebPParseHeaders(headers: *mut WebPHeaderStructure) -> VP8StatusCode;
    fn WebPInitCustomIo(params: *mut WebPDecParams, io: *mut VP8Io);
    fn WebPAllocateDecBuffer(
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        options: *const WebPDecoderOptions,
        buffer: *mut WebPDecBuffer,
    ) -> VP8StatusCode;
    fn WebPFlipBuffer(buffer: *mut WebPDecBuffer) -> VP8StatusCode;
    fn WebPCopyDecBufferPixels(src: *const WebPDecBuffer, dst: *mut WebPDecBuffer)
        -> VP8StatusCode;
    fn WebPAvoidSlowMemory(
        output: *const WebPDecBuffer,
        features: *const WebPBitstreamFeatures,
    ) -> ::core::ffi::c_int;
    fn VP8BitReaderSetBuffer(br: *mut VP8BitReader, start: *const uint8_t, size: size_t);
    fn VP8RemapBitReader(br: *mut VP8BitReader, offset: ptrdiff_t);
    fn VP8LBitReaderSetBuffer(br: *mut VP8LBitReader, buffer: *const uint8_t, length: size_t);
    fn VP8LNew() -> *mut VP8LDecoder;
    fn VP8LDecodeHeader(dec: *mut VP8LDecoder, io: *mut VP8Io) -> ::core::ffi::c_int;
    fn VP8LDecodeImage(dec: *mut VP8LDecoder) -> ::core::ffi::c_int;
    fn VP8LDelete(dec: *mut VP8LDecoder);
    fn WebPGetWorkerInterface() -> *const WebPWorkerInterface;
    fn VP8ParseIntraModeRow(br: *mut VP8BitReader, dec: *mut VP8Decoder) -> ::core::ffi::c_int;
    fn VP8InitFrame(dec: *mut VP8Decoder, io: *mut VP8Io) -> ::core::ffi::c_int;
    fn VP8EnterCritical(dec: *mut VP8Decoder, io: *mut VP8Io) -> VP8StatusCode;
    fn VP8ExitCritical(dec: *mut VP8Decoder, io: *mut VP8Io) -> ::core::ffi::c_int;
    fn VP8GetThreadMethod(
        options: *const WebPDecoderOptions,
        headers: *const WebPHeaderStructure,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VP8InitDithering(options: *const WebPDecoderOptions, dec: *mut VP8Decoder);
    fn VP8ProcessRow(dec: *mut VP8Decoder, io: *mut VP8Io) -> ::core::ffi::c_int;
    fn VP8InitScanline(dec: *mut VP8Decoder);
    fn VP8DecodeMB(dec: *mut VP8Decoder, token_br: *mut VP8BitReader) -> ::core::ffi::c_int;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeCalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type ptrdiff_t = isize;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type rescaler_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPRescaler {
    pub x_expand: ::core::ffi::c_int,
    pub y_expand: ::core::ffi::c_int,
    pub num_channels: ::core::ffi::c_int,
    pub fx_scale: uint32_t,
    pub fy_scale: uint32_t,
    pub fxy_scale: uint32_t,
    pub y_accum: ::core::ffi::c_int,
    pub y_add: ::core::ffi::c_int,
    pub y_sub: ::core::ffi::c_int,
    pub x_add: ::core::ffi::c_int,
    pub x_sub: ::core::ffi::c_int,
    pub src_width: ::core::ffi::c_int,
    pub src_height: ::core::ffi::c_int,
    pub dst_width: ::core::ffi::c_int,
    pub dst_height: ::core::ffi::c_int,
    pub src_y: ::core::ffi::c_int,
    pub dst_y: ::core::ffi::c_int,
    pub dst: *mut uint8_t,
    pub dst_stride: ::core::ffi::c_int,
    pub irow: *mut rescaler_t,
    pub frow: *mut rescaler_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPRGBABuffer {
    pub rgba: *mut uint8_t,
    pub stride: ::core::ffi::c_int,
    pub size: size_t,
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
pub struct WebPIDecoder {
    pub state_: DecState,
    pub params_: WebPDecParams,
    pub is_lossless_: ::core::ffi::c_int,
    pub dec_: *mut ::core::ffi::c_void,
    pub io_: VP8Io,
    pub mem_: MemBuffer,
    pub output_: WebPDecBuffer,
    pub final_output_: *mut WebPDecBuffer,
    pub chunk_size_: size_t,
    pub last_mb_y_: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemBuffer {
    pub mode_: MemBufferMode,
    pub start_: size_t,
    pub end_: size_t,
    pub buf_size_: size_t,
    pub buf_: *mut uint8_t,
    pub part0_size_: size_t,
    pub part0_buf_: *const uint8_t,
}
pub type MemBufferMode = ::core::ffi::c_uint;
pub const MEM_MODE_MAP: MemBufferMode = 2;
pub const MEM_MODE_APPEND: MemBufferMode = 1;
pub const MEM_MODE_NONE: MemBufferMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Io {
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub mb_y: ::core::ffi::c_int,
    pub mb_w: ::core::ffi::c_int,
    pub mb_h: ::core::ffi::c_int,
    pub y: *const uint8_t,
    pub u: *const uint8_t,
    pub v: *const uint8_t,
    pub y_stride: ::core::ffi::c_int,
    pub uv_stride: ::core::ffi::c_int,
    pub opaque: *mut ::core::ffi::c_void,
    pub put: VP8IoPutHook,
    pub setup: VP8IoSetupHook,
    pub teardown: VP8IoTeardownHook,
    pub fancy_upsampling: ::core::ffi::c_int,
    pub data_size: size_t,
    pub data: *const uint8_t,
    pub bypass_filtering: ::core::ffi::c_int,
    pub use_cropping: ::core::ffi::c_int,
    pub crop_left: ::core::ffi::c_int,
    pub crop_right: ::core::ffi::c_int,
    pub crop_top: ::core::ffi::c_int,
    pub crop_bottom: ::core::ffi::c_int,
    pub use_scaling: ::core::ffi::c_int,
    pub scaled_width: ::core::ffi::c_int,
    pub scaled_height: ::core::ffi::c_int,
    pub a: *const uint8_t,
}
pub type VP8IoTeardownHook = Option<unsafe extern "C" fn(*const VP8Io) -> ()>;
pub type VP8IoSetupHook = Option<unsafe extern "C" fn(*mut VP8Io) -> ::core::ffi::c_int>;
pub type VP8IoPutHook = Option<unsafe extern "C" fn(*const VP8Io) -> ::core::ffi::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPDecParams {
    pub output: *mut WebPDecBuffer,
    pub tmp_y: *mut uint8_t,
    pub tmp_u: *mut uint8_t,
    pub tmp_v: *mut uint8_t,
    pub last_y: ::core::ffi::c_int,
    pub options: *const WebPDecoderOptions,
    pub scaler_y: *mut WebPRescaler,
    pub scaler_u: *mut WebPRescaler,
    pub scaler_v: *mut WebPRescaler,
    pub scaler_a: *mut WebPRescaler,
    pub memory: *mut ::core::ffi::c_void,
    pub emit: OutputFunc,
    pub emit_alpha: OutputAlphaFunc,
    pub emit_alpha_row: OutputRowFunc,
}
pub type OutputRowFunc = Option<
    unsafe extern "C" fn(
        *mut WebPDecParams,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type OutputAlphaFunc = Option<
    unsafe extern "C" fn(
        *const VP8Io,
        *mut WebPDecParams,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type OutputFunc =
    Option<unsafe extern "C" fn(*const VP8Io, *mut WebPDecParams) -> ::core::ffi::c_int>;
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
pub type DecState = ::core::ffi::c_uint;
pub const STATE_ERROR: DecState = 7;
pub const STATE_DONE: DecState = 6;
pub const STATE_VP8L_DATA: DecState = 5;
pub const STATE_VP8L_HEADER: DecState = 4;
pub const STATE_VP8_DATA: DecState = 3;
pub const STATE_VP8_PARTS0: DecState = 2;
pub const STATE_VP8_HEADER: DecState = 1;
pub const STATE_WEBP_HEADER: DecState = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPDecoderConfig {
    pub input: WebPBitstreamFeatures,
    pub output: WebPDecBuffer,
    pub options: WebPDecoderOptions,
}
pub type VP8StatusCode = ::core::ffi::c_uint;
pub const VP8_STATUS_NOT_ENOUGH_DATA: VP8StatusCode = 7;
pub const VP8_STATUS_USER_ABORT: VP8StatusCode = 6;
pub const VP8_STATUS_SUSPENDED: VP8StatusCode = 5;
pub const VP8_STATUS_UNSUPPORTED_FEATURE: VP8StatusCode = 4;
pub const VP8_STATUS_BITSTREAM_ERROR: VP8StatusCode = 3;
pub const VP8_STATUS_INVALID_PARAM: VP8StatusCode = 2;
pub const VP8_STATUS_OUT_OF_MEMORY: VP8StatusCode = 1;
pub const VP8_STATUS_OK: VP8StatusCode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LDecoder {
    pub status_: VP8StatusCode,
    pub state_: VP8LDecodeState,
    pub io_: *mut VP8Io,
    pub output_: *const WebPDecBuffer,
    pub pixels_: *mut uint32_t,
    pub argb_cache_: *mut uint32_t,
    pub br_: VP8LBitReader,
    pub incremental_: ::core::ffi::c_int,
    pub saved_br_: VP8LBitReader,
    pub saved_last_pixel_: ::core::ffi::c_int,
    pub width_: ::core::ffi::c_int,
    pub height_: ::core::ffi::c_int,
    pub last_row_: ::core::ffi::c_int,
    pub last_pixel_: ::core::ffi::c_int,
    pub last_out_row_: ::core::ffi::c_int,
    pub hdr_: VP8LMetadata,
    pub next_transform_: ::core::ffi::c_int,
    pub transforms_: [VP8LTransform; 4],
    pub transforms_seen_: uint32_t,
    pub rescaler_memory: *mut uint8_t,
    pub rescaler: *mut WebPRescaler,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LTransform {
    pub type_: VP8LImageTransformType,
    pub bits_: ::core::ffi::c_int,
    pub xsize_: ::core::ffi::c_int,
    pub ysize_: ::core::ffi::c_int,
    pub data_: *mut uint32_t,
}
pub type VP8LImageTransformType = ::core::ffi::c_uint;
pub const COLOR_INDEXING_TRANSFORM: VP8LImageTransformType = 3;
pub const SUBTRACT_GREEN_TRANSFORM: VP8LImageTransformType = 2;
pub const CROSS_COLOR_TRANSFORM: VP8LImageTransformType = 1;
pub const PREDICTOR_TRANSFORM: VP8LImageTransformType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LMetadata {
    pub color_cache_size_: ::core::ffi::c_int,
    pub color_cache_: VP8LColorCache,
    pub saved_color_cache_: VP8LColorCache,
    pub huffman_mask_: ::core::ffi::c_int,
    pub huffman_subsample_bits_: ::core::ffi::c_int,
    pub huffman_xsize_: ::core::ffi::c_int,
    pub huffman_image_: *mut uint32_t,
    pub num_htree_groups_: ::core::ffi::c_int,
    pub htree_groups_: *mut HTreeGroup,
    pub huffman_tables_: HuffmanTables,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTables {
    pub root: HuffmanTablesSegment,
    pub curr_segment: *mut HuffmanTablesSegment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTablesSegment {
    pub start: *mut HuffmanCode,
    pub curr_table: *mut HuffmanCode,
    pub next: *mut HuffmanTablesSegment,
    pub size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanCode {
    pub bits: uint8_t,
    pub value: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HTreeGroup {
    pub htrees: [*mut HuffmanCode; 5],
    pub is_trivial_literal: ::core::ffi::c_int,
    pub literal_arb: uint32_t,
    pub is_trivial_code: ::core::ffi::c_int,
    pub use_packed_table: ::core::ffi::c_int,
    pub packed_table: [HuffmanCode32; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanCode32 {
    pub bits: ::core::ffi::c_int,
    pub value: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LColorCache {
    pub colors_: *mut uint32_t,
    pub hash_shift_: ::core::ffi::c_int,
    pub hash_bits_: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LBitReader {
    pub val_: vp8l_val_t,
    pub buf_: *const uint8_t,
    pub len_: size_t,
    pub pos_: size_t,
    pub bit_pos_: ::core::ffi::c_int,
    pub eos_: ::core::ffi::c_int,
}
pub type vp8l_val_t = uint64_t;
pub type VP8LDecodeState = ::core::ffi::c_uint;
pub const READ_DIM: VP8LDecodeState = 2;
pub const READ_HDR: VP8LDecodeState = 1;
pub const READ_DATA: VP8LDecodeState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Decoder {
    pub status_: VP8StatusCode,
    pub ready_: ::core::ffi::c_int,
    pub error_msg_: *const ::core::ffi::c_char,
    pub br_: VP8BitReader,
    pub frm_hdr_: VP8FrameHeader,
    pub pic_hdr_: VP8PictureHeader,
    pub filter_hdr_: VP8FilterHeader,
    pub segment_hdr_: VP8SegmentHeader,
    pub worker_: WebPWorker,
    pub mt_method_: ::core::ffi::c_int,
    pub cache_id_: ::core::ffi::c_int,
    pub num_caches_: ::core::ffi::c_int,
    pub thread_ctx_: VP8ThreadContext,
    pub mb_w_: ::core::ffi::c_int,
    pub mb_h_: ::core::ffi::c_int,
    pub tl_mb_x_: ::core::ffi::c_int,
    pub tl_mb_y_: ::core::ffi::c_int,
    pub br_mb_x_: ::core::ffi::c_int,
    pub br_mb_y_: ::core::ffi::c_int,
    pub num_parts_minus_one_: uint32_t,
    pub parts_: [VP8BitReader; 8],
    pub dither_: ::core::ffi::c_int,
    pub dithering_rg_: VP8Random,
    pub dqm_: [VP8QuantMatrix; 4],
    pub proba_: VP8Proba,
    pub use_skip_proba_: ::core::ffi::c_int,
    pub skip_p_: uint8_t,
    pub intra_t_: *mut uint8_t,
    pub intra_l_: [uint8_t; 4],
    pub yuv_t_: *mut VP8TopSamples,
    pub mb_info_: *mut VP8MB,
    pub f_info_: *mut VP8FInfo,
    pub yuv_b_: *mut uint8_t,
    pub cache_y_: *mut uint8_t,
    pub cache_u_: *mut uint8_t,
    pub cache_v_: *mut uint8_t,
    pub cache_y_stride_: ::core::ffi::c_int,
    pub cache_uv_stride_: ::core::ffi::c_int,
    pub mem_: *mut ::core::ffi::c_void,
    pub mem_size_: size_t,
    pub mb_x_: ::core::ffi::c_int,
    pub mb_y_: ::core::ffi::c_int,
    pub mb_data_: *mut VP8MBData,
    pub filter_type_: ::core::ffi::c_int,
    pub fstrengths_: [[VP8FInfo; 2]; 4],
    pub alph_dec_: *mut ALPHDecoder,
    pub alpha_data_: *const uint8_t,
    pub alpha_data_size_: size_t,
    pub is_alpha_decoded_: ::core::ffi::c_int,
    pub alpha_plane_mem_: *mut uint8_t,
    pub alpha_plane_: *mut uint8_t,
    pub alpha_prev_line_: *const uint8_t,
    pub alpha_dithering_: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ALPHDecoder {
    pub width_: ::core::ffi::c_int,
    pub height_: ::core::ffi::c_int,
    pub method_: ::core::ffi::c_int,
    pub filter_: WEBP_FILTER_TYPE,
    pub pre_processing_: ::core::ffi::c_int,
    pub vp8l_dec_: *mut VP8LDecoder,
    pub io_: VP8Io,
    pub use_8b_decode_: ::core::ffi::c_int,
    pub output_: *mut uint8_t,
    pub prev_line_: *const uint8_t,
}
pub type WEBP_FILTER_TYPE = ::core::ffi::c_uint;
pub const WEBP_FILTER_FAST: WEBP_FILTER_TYPE = 6;
pub const WEBP_FILTER_BEST: WEBP_FILTER_TYPE = 5;
pub const WEBP_FILTER_LAST: WEBP_FILTER_TYPE = 4;
pub const WEBP_FILTER_GRADIENT: WEBP_FILTER_TYPE = 3;
pub const WEBP_FILTER_VERTICAL: WEBP_FILTER_TYPE = 2;
pub const WEBP_FILTER_HORIZONTAL: WEBP_FILTER_TYPE = 1;
pub const WEBP_FILTER_NONE: WEBP_FILTER_TYPE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8FInfo {
    pub f_limit_: uint8_t,
    pub f_ilevel_: uint8_t,
    pub f_inner_: uint8_t,
    pub hev_thresh_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8MBData {
    pub coeffs_: [int16_t; 384],
    pub is_i4x4_: uint8_t,
    pub imodes_: [uint8_t; 16],
    pub uvmode_: uint8_t,
    pub non_zero_y_: uint32_t,
    pub non_zero_uv_: uint32_t,
    pub dither_: uint8_t,
    pub skip_: uint8_t,
    pub segment_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8MB {
    pub nz_: uint8_t,
    pub nz_dc_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8TopSamples {
    pub y: [uint8_t; 16],
    pub u: [uint8_t; 8],
    pub v: [uint8_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Proba {
    pub segments_: [uint8_t; 3],
    pub bands_: [[VP8BandProbas; 8]; 4],
    pub bands_ptr_: [[*const VP8BandProbas; 17]; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8BandProbas {
    pub probas_: [VP8ProbaArray; 3],
}
pub type VP8ProbaArray = [uint8_t; 11];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8QuantMatrix {
    pub y1_mat_: quant_t,
    pub y2_mat_: quant_t,
    pub uv_mat_: quant_t,
    pub uv_quant_: ::core::ffi::c_int,
    pub dither_: ::core::ffi::c_int,
}
pub type quant_t = [::core::ffi::c_int; 2];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Random {
    pub index1_: ::core::ffi::c_int,
    pub index2_: ::core::ffi::c_int,
    pub tab_: [uint32_t; 55],
    pub amp_: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8BitReader {
    pub value_: bit_t,
    pub range_: range_t,
    pub bits_: ::core::ffi::c_int,
    pub buf_: *const uint8_t,
    pub buf_end_: *const uint8_t,
    pub buf_max_: *const uint8_t,
    pub eof_: ::core::ffi::c_int,
}
pub type range_t = uint32_t;
pub type bit_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8ThreadContext {
    pub id_: ::core::ffi::c_int,
    pub mb_y_: ::core::ffi::c_int,
    pub filter_row_: ::core::ffi::c_int,
    pub f_info_: *mut VP8FInfo,
    pub mb_data_: *mut VP8MBData,
    pub io_: VP8Io,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPWorker {
    pub impl_: *mut ::core::ffi::c_void,
    pub status_: WebPWorkerStatus,
    pub hook: WebPWorkerHook,
    pub data1: *mut ::core::ffi::c_void,
    pub data2: *mut ::core::ffi::c_void,
    pub had_error: ::core::ffi::c_int,
}
pub type WebPWorkerHook = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::core::ffi::c_int,
>;
pub type WebPWorkerStatus = ::core::ffi::c_uint;
pub const WORK: WebPWorkerStatus = 2;
pub const OK: WebPWorkerStatus = 1;
pub const NOT_OK: WebPWorkerStatus = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8SegmentHeader {
    pub use_segment_: ::core::ffi::c_int,
    pub update_map_: ::core::ffi::c_int,
    pub absolute_delta_: ::core::ffi::c_int,
    pub quantizer_: [int8_t; 4],
    pub filter_strength_: [int8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8FilterHeader {
    pub simple_: ::core::ffi::c_int,
    pub level_: ::core::ffi::c_int,
    pub sharpness_: ::core::ffi::c_int,
    pub use_lf_delta_: ::core::ffi::c_int,
    pub ref_lf_delta_: [::core::ffi::c_int; 4],
    pub mode_lf_delta_: [::core::ffi::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8PictureHeader {
    pub width_: uint16_t,
    pub height_: uint16_t,
    pub xscale_: uint8_t,
    pub yscale_: uint8_t,
    pub colorspace_: uint8_t,
    pub clamp_type_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8FrameHeader {
    pub key_frame_: uint8_t,
    pub profile_: uint8_t,
    pub show_: uint8_t,
    pub partition_length_: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MBContext {
    pub left_: VP8MB,
    pub info_: VP8MB,
    pub token_br_: VP8BitReader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPWorkerInterface {
    pub Init: Option<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
    pub Reset: Option<unsafe extern "C" fn(*mut WebPWorker) -> ::core::ffi::c_int>,
    pub Sync_0: Option<unsafe extern "C" fn(*mut WebPWorker) -> ::core::ffi::c_int>,
    pub Launch: Option<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
    pub Execute: Option<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
    pub End: Option<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPHeaderStructure {
    pub data: *const uint8_t,
    pub data_size: size_t,
    pub have_all_data: ::core::ffi::c_int,
    pub offset: size_t,
    pub alpha_data: *const uint8_t,
    pub alpha_data_size: size_t,
    pub compressed_size: size_t,
    pub riff_size: size_t,
    pub is_lossless: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const WEBP_DECODER_ABI_VERSION: ::core::ffi::c_int = 0x209 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn WebPInitDecBuffer(mut buffer: *mut WebPDecBuffer) -> ::core::ffi::c_int {
    return WebPInitDecBufferInternal(buffer, WEBP_DECODER_ABI_VERSION);
}
#[inline]
unsafe extern "C" fn WebPGetFeatures(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut features: *mut WebPBitstreamFeatures,
) -> VP8StatusCode {
    return WebPGetFeaturesInternal(data, data_size, features, WEBP_DECODER_ABI_VERSION);
}
#[inline]
unsafe extern "C" fn VP8InitIo(io: *mut VP8Io) -> ::core::ffi::c_int {
    return VP8InitIoInternal(io, WEBP_DECODER_ABI_VERSION);
}
pub const VP8_FRAME_HEADER_SIZE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const ALPHA_HEADER_LEN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ALPHA_LOSSLESS_COMPRESSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CHUNK_HEADER_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MAX_CHUNK_PAYLOAD: ::core::ffi::c_uint = (!(0 as ::core::ffi::c_uint))
    .wrapping_sub(CHUNK_HEADER_SIZE as ::core::ffi::c_uint)
    .wrapping_sub(1 as ::core::ffi::c_uint);
pub const CHUNK_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const MAX_MB_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn MemDataSize(mut mem: *const MemBuffer) -> size_t {
    return (*mem).end_.wrapping_sub((*mem).start_);
}
unsafe extern "C" fn NeedCompressedAlpha(idec: *const WebPIDecoder) -> ::core::ffi::c_int {
    if (*idec).state_ as ::core::ffi::c_uint
        == STATE_WEBP_HEADER as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*idec).is_lossless_ != 0 {
        return 0 as ::core::ffi::c_int;
    } else {
        let dec: *const VP8Decoder = (*idec).dec_ as *mut VP8Decoder;
        '_c2rust_label: {
            if !dec.is_null() {
            } else {
                __assert_fail(
                    b"dec != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/idec_dec.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    107 as ::core::ffi::c_uint,
                    b"int NeedCompressedAlpha(const WebPIDecoder *const)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        return (!(*dec).alpha_data_.is_null() && (*dec).is_alpha_decoded_ == 0)
            as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn DoRemap(idec: *mut WebPIDecoder, mut offset: ptrdiff_t) {
    let mem: *mut MemBuffer = &raw mut (*idec).mem_;
    let new_base: *const uint8_t = (*mem).buf_.offset((*mem).start_ as isize);
    (*idec).io_.data = new_base;
    (*idec).io_.data_size = MemDataSize(mem);
    if !(*idec).dec_.is_null() {
        if (*idec).is_lossless_ == 0 {
            let dec: *mut VP8Decoder = (*idec).dec_ as *mut VP8Decoder;
            let last_part: uint32_t = (*dec).num_parts_minus_one_;
            if offset != 0 as ptrdiff_t {
                let mut p: uint32_t = 0;
                p = 0 as uint32_t;
                while p <= last_part {
                    VP8RemapBitReader(
                        (&raw mut (*dec).parts_ as *mut VP8BitReader).offset(p as isize),
                        offset,
                    );
                    p = p.wrapping_add(1);
                }
                if (*mem).mode_ as ::core::ffi::c_uint
                    == MEM_MODE_MAP as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    VP8RemapBitReader(&raw mut (*dec).br_, offset);
                }
            }
            let last_start: *const uint8_t = (*dec).parts_[last_part as usize].buf_;
            VP8BitReaderSetBuffer(
                (&raw mut (*dec).parts_ as *mut VP8BitReader).offset(last_part as isize)
                    as *mut VP8BitReader,
                last_start,
                (*mem)
                    .buf_
                    .offset((*mem).end_ as isize)
                    .offset_from(last_start) as ::core::ffi::c_long as size_t,
            );
            if NeedCompressedAlpha(idec) != 0 {
                let alph_dec: *mut ALPHDecoder = (*dec).alph_dec_ as *mut ALPHDecoder;
                (*dec).alpha_data_ = (*dec).alpha_data_.offset(offset as isize);
                if !alph_dec.is_null() && !(*alph_dec).vp8l_dec_.is_null() {
                    if (*alph_dec).method_ == ALPHA_LOSSLESS_COMPRESSION {
                        let alph_vp8l_dec: *mut VP8LDecoder =
                            (*alph_dec).vp8l_dec_ as *mut VP8LDecoder;
                        '_c2rust_label: {
                            if (*dec).alpha_data_size_ >= 1 as size_t {
                            } else {
                                __assert_fail(
                                    b"dec->alpha_data_size_ >= ALPHA_HEADER_LEN\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/idec_dec.c\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    146 as ::core::ffi::c_uint,
                                    b"void DoRemap(WebPIDecoder *const, ptrdiff_t)\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                );
                            }
                        };
                        VP8LBitReaderSetBuffer(
                            &raw mut (*alph_vp8l_dec).br_,
                            (*dec).alpha_data_.offset(ALPHA_HEADER_LEN as isize),
                            (*dec)
                                .alpha_data_size_
                                .wrapping_sub(ALPHA_HEADER_LEN as size_t),
                        );
                    }
                }
            }
        } else {
            let dec_0: *mut VP8LDecoder = (*idec).dec_ as *mut VP8LDecoder;
            VP8LBitReaderSetBuffer(&raw mut (*dec_0).br_, new_base, MemDataSize(mem));
        }
    }
}
unsafe extern "C" fn AppendToMemBuffer(
    idec: *mut WebPIDecoder,
    data: *const uint8_t,
    mut data_size: size_t,
) -> ::core::ffi::c_int {
    let dec: *mut VP8Decoder = (*idec).dec_ as *mut VP8Decoder;
    let mem: *mut MemBuffer = &raw mut (*idec).mem_;
    let need_compressed_alpha: ::core::ffi::c_int = NeedCompressedAlpha(idec) as ::core::ffi::c_int;
    let old_start: *const uint8_t = if (*mem).buf_.is_null() {
        ::core::ptr::null_mut::<uint8_t>()
    } else {
        (*mem).buf_.offset((*mem).start_ as isize)
    };
    let old_base: *const uint8_t = if need_compressed_alpha != 0 {
        (*dec).alpha_data_
    } else {
        old_start
    };
    '_c2rust_label: {
        if !(*mem).buf_.is_null() || (*mem).start_ == 0 as size_t {
        } else {
            __assert_fail(
                b"mem->buf_ != NULL || mem->start_ == 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/idec_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                173 as ::core::ffi::c_uint,
                b"int AppendToMemBuffer(WebPIDecoder *const, const uint8_t *const, size_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*mem).mode_ as ::core::ffi::c_uint
            == MEM_MODE_APPEND as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"mem->mode_ == MEM_MODE_APPEND\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/idec_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                174 as ::core::ffi::c_uint,
                b"int AppendToMemBuffer(WebPIDecoder *const, const uint8_t *const, size_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if data_size > MAX_CHUNK_PAYLOAD as size_t {
        return 0 as ::core::ffi::c_int;
    }
    if (*mem).end_.wrapping_add(data_size) > (*mem).buf_size_ {
        let new_mem_start: size_t =
            old_start.offset_from(old_base) as ::core::ffi::c_long as size_t;
        let current_size: size_t = (MemDataSize(mem) as size_t).wrapping_add(new_mem_start);
        let new_size: uint64_t = (current_size as uint64_t).wrapping_add(data_size as uint64_t);
        let extra_size: uint64_t = new_size
            .wrapping_add(CHUNK_SIZE as uint64_t)
            .wrapping_sub(1 as uint64_t)
            & !(CHUNK_SIZE - 1 as ::core::ffi::c_int) as uint64_t;
        let new_buf: *mut uint8_t =
            WebPSafeMalloc(extra_size, ::core::mem::size_of::<uint8_t>() as size_t) as *mut uint8_t;
        if new_buf.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if !old_base.is_null() {
            memcpy(
                new_buf as *mut ::core::ffi::c_void,
                old_base as *const ::core::ffi::c_void,
                current_size,
            );
        }
        WebPSafeFree((*mem).buf_ as *mut ::core::ffi::c_void);
        (*mem).buf_ = new_buf;
        (*mem).buf_size_ = extra_size as size_t;
        (*mem).start_ = new_mem_start;
        (*mem).end_ = current_size;
    }
    '_c2rust_label_1: {
        if !(*mem).buf_.is_null() {
        } else {
            __assert_fail(
                b"mem->buf_ != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/idec_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                197 as ::core::ffi::c_uint,
                b"int AppendToMemBuffer(WebPIDecoder *const, const uint8_t *const, size_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    memcpy(
        (*mem).buf_.offset((*mem).end_ as isize) as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        data_size,
    );
    (*mem).end_ = (*mem).end_.wrapping_add(data_size);
    '_c2rust_label_2: {
        if (*mem).end_ <= (*mem).buf_size_ {
        } else {
            __assert_fail(
                b"mem->end_ <= mem->buf_size_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/idec_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                200 as ::core::ffi::c_uint,
                b"int AppendToMemBuffer(WebPIDecoder *const, const uint8_t *const, size_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    DoRemap(
        idec,
        (*mem)
            .buf_
            .offset((*mem).start_ as isize)
            .offset_from(old_start) as ptrdiff_t,
    );
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn RemapMemBuffer(
    idec: *mut WebPIDecoder,
    data: *const uint8_t,
    mut data_size: size_t,
) -> ::core::ffi::c_int {
    let mem: *mut MemBuffer = &raw mut (*idec).mem_;
    let old_buf: *const uint8_t = (*mem).buf_;
    let old_start: *const uint8_t = if old_buf.is_null() {
        ::core::ptr::null::<uint8_t>()
    } else {
        old_buf.offset((*mem).start_ as isize)
    };
    '_c2rust_label: {
        if !old_buf.is_null() || (*mem).start_ == 0 as size_t {
        } else {
            __assert_fail(
                b"old_buf != NULL || mem->start_ == 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/idec_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                212 as ::core::ffi::c_uint,
                b"int RemapMemBuffer(WebPIDecoder *const, const uint8_t *const, size_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*mem).mode_ as ::core::ffi::c_uint
            == MEM_MODE_MAP as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"mem->mode_ == MEM_MODE_MAP\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/idec_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                213 as ::core::ffi::c_uint,
                b"int RemapMemBuffer(WebPIDecoder *const, const uint8_t *const, size_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if data_size < (*mem).buf_size_ {
        return 0 as ::core::ffi::c_int;
    }
    (*mem).buf_ = data as *mut uint8_t;
    (*mem).buf_size_ = data_size;
    (*mem).end_ = (*mem).buf_size_;
    DoRemap(
        idec,
        (*mem)
            .buf_
            .offset((*mem).start_ as isize)
            .offset_from(old_start) as ptrdiff_t,
    );
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn InitMemBuffer(mem: *mut MemBuffer) {
    (*mem).mode_ = MEM_MODE_NONE;
    (*mem).buf_ = ::core::ptr::null_mut::<uint8_t>();
    (*mem).buf_size_ = 0 as size_t;
    (*mem).part0_buf_ = ::core::ptr::null::<uint8_t>();
    (*mem).part0_size_ = 0 as size_t;
}
unsafe extern "C" fn ClearMemBuffer(mem: *mut MemBuffer) {
    '_c2rust_label: {
        if !mem.is_null() {
        } else {
            __assert_fail(
                b"mem\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/idec_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                233 as ::core::ffi::c_uint,
                b"void ClearMemBuffer(MemBuffer *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*mem).mode_ as ::core::ffi::c_uint
        == MEM_MODE_APPEND as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        WebPSafeFree((*mem).buf_ as *mut ::core::ffi::c_void);
        WebPSafeFree((*mem).part0_buf_ as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn CheckMemBufferMode(
    mem: *mut MemBuffer,
    mut expected: MemBufferMode,
) -> ::core::ffi::c_int {
    if (*mem).mode_ as ::core::ffi::c_uint
        == MEM_MODE_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*mem).mode_ = expected;
    } else if (*mem).mode_ as ::core::ffi::c_uint != expected as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_int;
    }
    '_c2rust_label: {
        if (*mem).mode_ as ::core::ffi::c_uint == expected as ::core::ffi::c_uint {
        } else {
            __assert_fail(
                b"mem->mode_ == expected\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/idec_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                246 as ::core::ffi::c_uint,
                b"int CheckMemBufferMode(MemBuffer *const, MemBufferMode)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn FinishDecoding(idec: *mut WebPIDecoder) -> VP8StatusCode {
    let options: *const WebPDecoderOptions = (*idec).params_.options;
    let output: *mut WebPDecBuffer = (*idec).params_.output;
    (*idec).state_ = STATE_DONE;
    if !options.is_null() && (*options).flip != 0 {
        let status: VP8StatusCode = WebPFlipBuffer(output) as VP8StatusCode;
        if status as ::core::ffi::c_uint
            != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return status;
        }
    }
    if !(*idec).final_output_.is_null() {
        WebPCopyDecBufferPixels(output, (*idec).final_output_);
        WebPFreeDecBuffer(&raw mut (*idec).output_);
        *output = *(*idec).final_output_;
        (*idec).final_output_ = ::core::ptr::null_mut::<WebPDecBuffer>();
    }
    return VP8_STATUS_OK;
}
unsafe extern "C" fn SaveContext(
    mut dec: *const VP8Decoder,
    mut token_br: *const VP8BitReader,
    context: *mut MBContext,
) {
    (*context).left_ = *(*dec).mb_info_.offset(-(1 as ::core::ffi::c_int) as isize);
    (*context).info_ = *(*dec).mb_info_.offset((*dec).mb_x_ as isize);
    (*context).token_br_ = *token_br;
}
unsafe extern "C" fn RestoreContext(
    mut context: *const MBContext,
    dec: *mut VP8Decoder,
    token_br: *mut VP8BitReader,
) {
    *(*dec).mb_info_.offset(-(1 as ::core::ffi::c_int) as isize) = (*context).left_;
    *(*dec).mb_info_.offset((*dec).mb_x_ as isize) = (*context).info_;
    *token_br = (*context).token_br_;
}
unsafe extern "C" fn IDecError(idec: *mut WebPIDecoder, mut error: VP8StatusCode) -> VP8StatusCode {
    if (*idec).state_ as ::core::ffi::c_uint
        == STATE_VP8_DATA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        VP8ExitCritical((*idec).dec_ as *mut VP8Decoder, &raw mut (*idec).io_);
    }
    (*idec).state_ = STATE_ERROR;
    return error;
}
unsafe extern "C" fn ChangeState(
    idec: *mut WebPIDecoder,
    mut new_state: DecState,
    mut consumed_bytes: size_t,
) {
    let mem: *mut MemBuffer = &raw mut (*idec).mem_;
    (*idec).state_ = new_state;
    (*mem).start_ = (*mem).start_.wrapping_add(consumed_bytes);
    '_c2rust_label: {
        if (*mem).start_ <= (*mem).end_ {
        } else {
            __assert_fail(
                b"mem->start_ <= mem->end_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/idec_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                302 as ::core::ffi::c_uint,
                b"void ChangeState(WebPIDecoder *const, DecState, size_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*idec).io_.data = (*mem).buf_.offset((*mem).start_ as isize);
    (*idec).io_.data_size = MemDataSize(mem);
}
unsafe extern "C" fn DecodeWebPHeaders(idec: *mut WebPIDecoder) -> VP8StatusCode {
    let mem: *mut MemBuffer = &raw mut (*idec).mem_;
    let mut data: *const uint8_t = (*mem).buf_.offset((*mem).start_ as isize);
    let mut curr_size: size_t = MemDataSize(mem);
    let mut status: VP8StatusCode = VP8_STATUS_OK;
    let mut headers: WebPHeaderStructure = WebPHeaderStructure {
        data: ::core::ptr::null::<uint8_t>(),
        data_size: 0,
        have_all_data: 0,
        offset: 0,
        alpha_data: ::core::ptr::null::<uint8_t>(),
        alpha_data_size: 0,
        compressed_size: 0,
        riff_size: 0,
        is_lossless: 0,
    };
    headers.data = data;
    headers.data_size = curr_size;
    headers.have_all_data = 0 as ::core::ffi::c_int;
    status = WebPParseHeaders(&raw mut headers);
    if status as ::core::ffi::c_uint
        == VP8_STATUS_NOT_ENOUGH_DATA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return VP8_STATUS_SUSPENDED;
    } else if status as ::core::ffi::c_uint
        != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return IDecError(idec, status);
    }
    (*idec).chunk_size_ = headers.compressed_size;
    (*idec).is_lossless_ = headers.is_lossless;
    if (*idec).is_lossless_ == 0 {
        let dec: *mut VP8Decoder = VP8New() as *mut VP8Decoder;
        if dec.is_null() {
            return VP8_STATUS_OUT_OF_MEMORY;
        }
        (*idec).dec_ = dec as *mut ::core::ffi::c_void;
        (*dec).alpha_data_ = headers.alpha_data;
        (*dec).alpha_data_size_ = headers.alpha_data_size;
        ChangeState(idec, STATE_VP8_HEADER, headers.offset);
    } else {
        let dec_0: *mut VP8LDecoder = VP8LNew() as *mut VP8LDecoder;
        if dec_0.is_null() {
            return VP8_STATUS_OUT_OF_MEMORY;
        }
        (*idec).dec_ = dec_0 as *mut ::core::ffi::c_void;
        ChangeState(idec, STATE_VP8L_HEADER, headers.offset);
    }
    return VP8_STATUS_OK;
}
unsafe extern "C" fn DecodeVP8FrameHeader(idec: *mut WebPIDecoder) -> VP8StatusCode {
    let mut data: *const uint8_t = (*idec).mem_.buf_.offset((*idec).mem_.start_ as isize);
    let curr_size: size_t = MemDataSize(&raw mut (*idec).mem_) as size_t;
    let mut width: ::core::ffi::c_int = 0;
    let mut height: ::core::ffi::c_int = 0;
    let mut bits: uint32_t = 0;
    if curr_size < VP8_FRAME_HEADER_SIZE as size_t {
        return VP8_STATUS_SUSPENDED;
    }
    if VP8GetInfo(
        data,
        curr_size,
        (*idec).chunk_size_,
        &raw mut width,
        &raw mut height,
    ) == 0
    {
        return IDecError(idec, VP8_STATUS_BITSTREAM_ERROR);
    }
    bits = (*data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        | (*data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
        | (*data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int) as uint32_t;
    (*idec).mem_.part0_size_ =
        (bits >> 5 as ::core::ffi::c_int).wrapping_add(VP8_FRAME_HEADER_SIZE as uint32_t) as size_t;
    (*idec).io_.data = data;
    (*idec).io_.data_size = curr_size;
    (*idec).state_ = STATE_VP8_PARTS0;
    return VP8_STATUS_OK;
}
unsafe extern "C" fn CopyParts0Data(idec: *mut WebPIDecoder) -> VP8StatusCode {
    let dec: *mut VP8Decoder = (*idec).dec_ as *mut VP8Decoder;
    let br: *mut VP8BitReader = &raw mut (*dec).br_;
    let part_size: size_t = (*br).buf_end_.offset_from((*br).buf_) as ::core::ffi::c_long as size_t;
    let mem: *mut MemBuffer = &raw mut (*idec).mem_;
    '_c2rust_label: {
        if (*idec).is_lossless_ == 0 {
        } else {
            __assert_fail(
                b"!idec->is_lossless_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/idec_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                376 as ::core::ffi::c_uint,
                b"VP8StatusCode CopyParts0Data(WebPIDecoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*mem).part0_buf_.is_null() {
        } else {
            __assert_fail(
                b"mem->part0_buf_ == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/idec_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                377 as ::core::ffi::c_uint,
                b"VP8StatusCode CopyParts0Data(WebPIDecoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if part_size <= (*mem).part0_size_ {
        } else {
            __assert_fail(
                b"part_size <= mem->part0_size_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/idec_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                379 as ::core::ffi::c_uint,
                b"VP8StatusCode CopyParts0Data(WebPIDecoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if part_size == 0 as size_t {
        return VP8_STATUS_BITSTREAM_ERROR;
    }
    if (*mem).mode_ as ::core::ffi::c_uint
        == MEM_MODE_APPEND as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let part0_buf: *mut uint8_t = WebPSafeMalloc(1 as uint64_t, part_size) as *mut uint8_t;
        if part0_buf.is_null() {
            return VP8_STATUS_OUT_OF_MEMORY;
        }
        memcpy(
            part0_buf as *mut ::core::ffi::c_void,
            (*br).buf_ as *const ::core::ffi::c_void,
            part_size,
        );
        (*mem).part0_buf_ = part0_buf;
        VP8BitReaderSetBuffer(br, part0_buf, part_size);
    }
    (*mem).start_ = (*mem).start_.wrapping_add(part_size);
    return VP8_STATUS_OK;
}
unsafe extern "C" fn DecodePartition0(idec: *mut WebPIDecoder) -> VP8StatusCode {
    let dec: *mut VP8Decoder = (*idec).dec_ as *mut VP8Decoder;
    let io: *mut VP8Io = &raw mut (*idec).io_;
    let params: *const WebPDecParams = &raw mut (*idec).params_;
    let output: *mut WebPDecBuffer = (*params).output;
    if MemDataSize(&raw mut (*idec).mem_) < (*idec).mem_.part0_size_ {
        return VP8_STATUS_SUSPENDED;
    }
    if VP8GetHeaders(dec, io) == 0 {
        let status: VP8StatusCode = (*dec).status_;
        if status as ::core::ffi::c_uint
            == VP8_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
            || status as ::core::ffi::c_uint
                == VP8_STATUS_NOT_ENOUGH_DATA as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return VP8_STATUS_SUSPENDED;
        }
        return IDecError(idec, status);
    }
    (*dec).status_ = WebPAllocateDecBuffer((*io).width, (*io).height, (*params).options, output);
    if (*dec).status_ as ::core::ffi::c_uint
        != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return IDecError(idec, (*dec).status_);
    }
    (*dec).mt_method_ = VP8GetThreadMethod(
        (*params).options,
        ::core::ptr::null::<WebPHeaderStructure>(),
        (*io).width,
        (*io).height,
    );
    VP8InitDithering((*params).options, dec);
    (*dec).status_ = CopyParts0Data(idec);
    if (*dec).status_ as ::core::ffi::c_uint
        != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return IDecError(idec, (*dec).status_);
    }
    if VP8EnterCritical(dec, io) as ::core::ffi::c_uint
        != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return IDecError(idec, (*dec).status_);
    }
    (*idec).state_ = STATE_VP8_DATA;
    if VP8InitFrame(dec, io) == 0 {
        return IDecError(idec, (*dec).status_);
    }
    return VP8_STATUS_OK;
}
unsafe extern "C" fn DecodeRemaining(idec: *mut WebPIDecoder) -> VP8StatusCode {
    let dec: *mut VP8Decoder = (*idec).dec_ as *mut VP8Decoder;
    let io: *mut VP8Io = &raw mut (*idec).io_;
    if (*dec).ready_ == 0 {
        return IDecError(idec, VP8_STATUS_BITSTREAM_ERROR);
    }
    while (*dec).mb_y_ < (*dec).mb_h_ {
        if (*idec).last_mb_y_ != (*dec).mb_y_ {
            if VP8ParseIntraModeRow(&raw mut (*dec).br_, dec) == 0 {
                return IDecError(idec, VP8_STATUS_BITSTREAM_ERROR);
            }
            (*idec).last_mb_y_ = (*dec).mb_y_;
        }
        while (*dec).mb_x_ < (*dec).mb_w_ {
            let token_br: *mut VP8BitReader = (&raw mut (*dec).parts_ as *mut VP8BitReader)
                .offset(((*dec).mb_y_ as uint32_t & (*dec).num_parts_minus_one_) as isize)
                as *mut VP8BitReader;
            let mut context: MBContext = MBContext {
                left_: VP8MB { nz_: 0, nz_dc_: 0 },
                info_: VP8MB { nz_: 0, nz_dc_: 0 },
                token_br_: VP8BitReader {
                    value_: 0,
                    range_: 0,
                    bits_: 0,
                    buf_: ::core::ptr::null::<uint8_t>(),
                    buf_end_: ::core::ptr::null::<uint8_t>(),
                    buf_max_: ::core::ptr::null::<uint8_t>(),
                    eof_: 0,
                },
            };
            SaveContext(dec, token_br, &raw mut context);
            if VP8DecodeMB(dec, token_br) == 0 {
                if (*dec).num_parts_minus_one_ == 0 as uint32_t
                    && MemDataSize(&raw mut (*idec).mem_) > MAX_MB_SIZE as size_t
                {
                    return IDecError(idec, VP8_STATUS_BITSTREAM_ERROR);
                }
                if (*dec).mt_method_ > 0 as ::core::ffi::c_int {
                    if (*WebPGetWorkerInterface())
                        .Sync_0
                        .expect("non-null function pointer")(
                        &raw mut (*dec).worker_
                    ) == 0
                    {
                        return IDecError(idec, VP8_STATUS_BITSTREAM_ERROR);
                    }
                }
                RestoreContext(&raw mut context, dec, token_br);
                return VP8_STATUS_SUSPENDED;
            }
            if (*dec).num_parts_minus_one_ == 0 as uint32_t {
                (*idec).mem_.start_ = (*token_br).buf_.offset_from((*idec).mem_.buf_)
                    as ::core::ffi::c_long as size_t;
                '_c2rust_label: {
                    if (*idec).mem_.start_ <= (*idec).mem_.end_ {
                    } else {
                        __assert_fail(
                            b"idec->mem_.start_ <= idec->mem_.end_\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/idec_dec.c\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            493 as ::core::ffi::c_uint,
                            b"VP8StatusCode DecodeRemaining(WebPIDecoder *const)\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
            }
            (*dec).mb_x_ += 1;
        }
        VP8InitScanline(dec);
        if VP8ProcessRow(dec, io) == 0 {
            return IDecError(idec, VP8_STATUS_USER_ABORT);
        }
        (*dec).mb_y_ += 1;
    }
    if VP8ExitCritical(dec, io) == 0 {
        (*idec).state_ = STATE_ERROR;
        return IDecError(idec, VP8_STATUS_USER_ABORT);
    }
    (*dec).ready_ = 0 as ::core::ffi::c_int;
    return FinishDecoding(idec);
}
unsafe extern "C" fn ErrorStatusLossless(
    idec: *mut WebPIDecoder,
    mut status: VP8StatusCode,
) -> VP8StatusCode {
    if status as ::core::ffi::c_uint
        == VP8_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        || status as ::core::ffi::c_uint
            == VP8_STATUS_NOT_ENOUGH_DATA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return VP8_STATUS_SUSPENDED;
    }
    return IDecError(idec, status);
}
unsafe extern "C" fn DecodeVP8LHeader(idec: *mut WebPIDecoder) -> VP8StatusCode {
    let io: *mut VP8Io = &raw mut (*idec).io_;
    let dec: *mut VP8LDecoder = (*idec).dec_ as *mut VP8LDecoder;
    let params: *const WebPDecParams = &raw mut (*idec).params_;
    let output: *mut WebPDecBuffer = (*params).output;
    let mut curr_size: size_t = MemDataSize(&raw mut (*idec).mem_);
    '_c2rust_label: {
        if (*idec).is_lossless_ != 0 {
        } else {
            __assert_fail(
                b"idec->is_lossless_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/idec_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                526 as ::core::ffi::c_uint,
                b"VP8StatusCode DecodeVP8LHeader(WebPIDecoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if curr_size < (*idec).chunk_size_ >> 3 as ::core::ffi::c_int {
        (*dec).status_ = VP8_STATUS_SUSPENDED;
        return ErrorStatusLossless(idec, (*dec).status_);
    }
    if VP8LDecodeHeader(dec, io) == 0 {
        if (*dec).status_ as ::core::ffi::c_uint
            == VP8_STATUS_BITSTREAM_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            && curr_size < (*idec).chunk_size_
        {
            (*dec).status_ = VP8_STATUS_SUSPENDED;
        }
        return ErrorStatusLossless(idec, (*dec).status_);
    }
    (*dec).status_ = WebPAllocateDecBuffer((*io).width, (*io).height, (*params).options, output);
    if (*dec).status_ as ::core::ffi::c_uint
        != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return IDecError(idec, (*dec).status_);
    }
    (*idec).state_ = STATE_VP8L_DATA;
    return VP8_STATUS_OK;
}
unsafe extern "C" fn DecodeVP8LData(idec: *mut WebPIDecoder) -> VP8StatusCode {
    let dec: *mut VP8LDecoder = (*idec).dec_ as *mut VP8LDecoder;
    let curr_size: size_t = MemDataSize(&raw mut (*idec).mem_) as size_t;
    '_c2rust_label: {
        if (*idec).is_lossless_ != 0 {
        } else {
            __assert_fail(
                b"idec->is_lossless_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/idec_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                555 as ::core::ffi::c_uint,
                b"VP8StatusCode DecodeVP8LData(WebPIDecoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*dec).incremental_ = (curr_size < (*idec).chunk_size_) as ::core::ffi::c_int;
    if VP8LDecodeImage(dec) == 0 {
        return ErrorStatusLossless(idec, (*dec).status_);
    }
    '_c2rust_label_0: {
        if (*dec).status_ as ::core::ffi::c_uint
            == VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*dec).status_ as ::core::ffi::c_uint
                == VP8_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"dec->status_ == VP8_STATUS_OK || dec->status_ == VP8_STATUS_SUSPENDED\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/idec_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                563 as ::core::ffi::c_uint,
                b"VP8StatusCode DecodeVP8LData(WebPIDecoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return (if (*dec).status_ as ::core::ffi::c_uint
        == VP8_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*dec).status_ as ::core::ffi::c_uint
    } else {
        FinishDecoding(idec) as ::core::ffi::c_uint
    }) as VP8StatusCode;
}
unsafe extern "C" fn IDecode(mut idec: *mut WebPIDecoder) -> VP8StatusCode {
    let mut status: VP8StatusCode = VP8_STATUS_SUSPENDED;
    if (*idec).state_ as ::core::ffi::c_uint
        == STATE_WEBP_HEADER as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        status = DecodeWebPHeaders(idec);
    } else if (*idec).dec_.is_null() {
        return VP8_STATUS_SUSPENDED;
    }
    if (*idec).state_ as ::core::ffi::c_uint
        == STATE_VP8_HEADER as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        status = DecodeVP8FrameHeader(idec);
    }
    if (*idec).state_ as ::core::ffi::c_uint
        == STATE_VP8_PARTS0 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        status = DecodePartition0(idec);
    }
    if (*idec).state_ as ::core::ffi::c_uint
        == STATE_VP8_DATA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let dec: *const VP8Decoder = (*idec).dec_ as *mut VP8Decoder;
        if dec.is_null() {
            return VP8_STATUS_SUSPENDED;
        }
        status = DecodeRemaining(idec);
    }
    if (*idec).state_ as ::core::ffi::c_uint
        == STATE_VP8L_HEADER as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        status = DecodeVP8LHeader(idec);
    }
    if (*idec).state_ as ::core::ffi::c_uint
        == STATE_VP8L_DATA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        status = DecodeVP8LData(idec);
    }
    return status;
}
unsafe extern "C" fn NewDecoder(
    output_buffer: *mut WebPDecBuffer,
    features: *const WebPBitstreamFeatures,
) -> *mut WebPIDecoder {
    let mut idec: *mut WebPIDecoder = WebPSafeCalloc(
        1 as uint64_t,
        ::core::mem::size_of::<WebPIDecoder>() as size_t,
    ) as *mut WebPIDecoder;
    if idec.is_null() {
        return ::core::ptr::null_mut::<WebPIDecoder>();
    }
    (*idec).state_ = STATE_WEBP_HEADER;
    (*idec).chunk_size_ = 0 as size_t;
    (*idec).last_mb_y_ = -(1 as ::core::ffi::c_int);
    InitMemBuffer(&raw mut (*idec).mem_);
    WebPInitDecBuffer(&raw mut (*idec).output_);
    VP8InitIo(&raw mut (*idec).io_);
    WebPResetDecParams(&raw mut (*idec).params_);
    if output_buffer.is_null() || WebPAvoidSlowMemory(output_buffer, features) != 0 {
        (*idec).params_.output = &raw mut (*idec).output_;
        (*idec).final_output_ = output_buffer;
        if !output_buffer.is_null() {
            (*(*idec).params_.output).colorspace = (*output_buffer).colorspace;
        }
    } else {
        (*idec).params_.output = output_buffer;
        (*idec).final_output_ = ::core::ptr::null_mut::<WebPDecBuffer>();
    }
    WebPInitCustomIo(&raw mut (*idec).params_, &raw mut (*idec).io_);
    return idec;
}
#[no_mangle]
pub unsafe extern "C" fn WebPINewDecoder(
    mut output_buffer: *mut WebPDecBuffer,
) -> *mut WebPIDecoder {
    return NewDecoder(output_buffer, ::core::ptr::null::<WebPBitstreamFeatures>());
}
#[no_mangle]
pub unsafe extern "C" fn WebPIDecode(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut config: *mut WebPDecoderConfig,
) -> *mut WebPIDecoder {
    let mut idec: *mut WebPIDecoder = ::core::ptr::null_mut::<WebPIDecoder>();
    let mut tmp_features: WebPBitstreamFeatures = WebPBitstreamFeatures {
        width: 0,
        height: 0,
        has_alpha: 0,
        has_animation: 0,
        format: 0,
        pad: [0; 5],
    };
    let features: *mut WebPBitstreamFeatures = if config.is_null() {
        &raw mut tmp_features
    } else {
        &raw mut (*config).input
    };
    memset(
        &raw mut tmp_features as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<WebPBitstreamFeatures>() as size_t,
    );
    if !data.is_null() && data_size > 0 as size_t {
        if WebPGetFeatures(data, data_size, features) as ::core::ffi::c_uint
            != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return ::core::ptr::null_mut::<WebPIDecoder>();
        }
    }
    idec = if !config.is_null() {
        NewDecoder(&raw mut (*config).output, features)
    } else {
        NewDecoder(::core::ptr::null_mut::<WebPDecBuffer>(), features)
    };
    if idec.is_null() {
        return ::core::ptr::null_mut::<WebPIDecoder>();
    }
    if !config.is_null() {
        (*idec).params_.options = &raw mut (*config).options;
    }
    return idec;
}
#[no_mangle]
pub unsafe extern "C" fn WebPIDelete(mut idec: *mut WebPIDecoder) {
    if idec.is_null() {
        return;
    }
    if !(*idec).dec_.is_null() {
        if (*idec).is_lossless_ == 0 {
            if (*idec).state_ as ::core::ffi::c_uint
                == STATE_VP8_DATA as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                VP8ExitCritical((*idec).dec_ as *mut VP8Decoder, &raw mut (*idec).io_);
            }
            VP8Delete((*idec).dec_ as *mut VP8Decoder);
        } else {
            VP8LDelete((*idec).dec_ as *mut VP8LDecoder);
        }
    }
    ClearMemBuffer(&raw mut (*idec).mem_);
    WebPFreeDecBuffer(&raw mut (*idec).output_);
    WebPSafeFree(idec as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn WebPINewRGB(
    mut csp: WEBP_CSP_MODE,
    mut output_buffer: *mut uint8_t,
    mut output_buffer_size: size_t,
    mut output_stride: ::core::ffi::c_int,
) -> *mut WebPIDecoder {
    let is_external_memory: ::core::ffi::c_int = if !output_buffer.is_null() {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    let mut idec: *mut WebPIDecoder = ::core::ptr::null_mut::<WebPIDecoder>();
    if csp as ::core::ffi::c_uint >= MODE_YUV as ::core::ffi::c_int as ::core::ffi::c_uint {
        return ::core::ptr::null_mut::<WebPIDecoder>();
    }
    if is_external_memory == 0 as ::core::ffi::c_int {
        output_buffer_size = 0 as size_t;
        output_stride = 0 as ::core::ffi::c_int;
    } else if output_stride == 0 as ::core::ffi::c_int || output_buffer_size == 0 as size_t {
        return ::core::ptr::null_mut::<WebPIDecoder>();
    }
    idec = WebPINewDecoder(::core::ptr::null_mut::<WebPDecBuffer>());
    if idec.is_null() {
        return ::core::ptr::null_mut::<WebPIDecoder>();
    }
    (*idec).output_.colorspace = csp;
    (*idec).output_.is_external_memory = is_external_memory;
    (*idec).output_.u.RGBA.rgba = output_buffer;
    (*idec).output_.u.RGBA.stride = output_stride;
    (*idec).output_.u.RGBA.size = output_buffer_size;
    return idec;
}
#[no_mangle]
pub unsafe extern "C" fn WebPINewYUVA(
    mut luma: *mut uint8_t,
    mut luma_size: size_t,
    mut luma_stride: ::core::ffi::c_int,
    mut u: *mut uint8_t,
    mut u_size: size_t,
    mut u_stride: ::core::ffi::c_int,
    mut v: *mut uint8_t,
    mut v_size: size_t,
    mut v_stride: ::core::ffi::c_int,
    mut a: *mut uint8_t,
    mut a_size: size_t,
    mut a_stride: ::core::ffi::c_int,
) -> *mut WebPIDecoder {
    let is_external_memory: ::core::ffi::c_int = if !luma.is_null() {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    let mut idec: *mut WebPIDecoder = ::core::ptr::null_mut::<WebPIDecoder>();
    let mut colorspace: WEBP_CSP_MODE = MODE_RGB;
    if is_external_memory == 0 as ::core::ffi::c_int {
        a_size = 0 as size_t;
        v_size = a_size;
        u_size = v_size;
        luma_size = u_size;
        a_stride = 0 as ::core::ffi::c_int;
        v_stride = a_stride;
        u_stride = v_stride;
        luma_stride = u_stride;
        a = ::core::ptr::null_mut::<uint8_t>();
        v = a;
        u = v;
        colorspace = MODE_YUVA;
    } else {
        if u.is_null() || v.is_null() {
            return ::core::ptr::null_mut::<WebPIDecoder>();
        }
        if luma_size == 0 as size_t || u_size == 0 as size_t || v_size == 0 as size_t {
            return ::core::ptr::null_mut::<WebPIDecoder>();
        }
        if luma_stride == 0 as ::core::ffi::c_int
            || u_stride == 0 as ::core::ffi::c_int
            || v_stride == 0 as ::core::ffi::c_int
        {
            return ::core::ptr::null_mut::<WebPIDecoder>();
        }
        if !a.is_null() {
            if a_size == 0 as size_t || a_stride == 0 as ::core::ffi::c_int {
                return ::core::ptr::null_mut::<WebPIDecoder>();
            }
        }
        colorspace = (if a.is_null() {
            MODE_YUV as ::core::ffi::c_int
        } else {
            MODE_YUVA as ::core::ffi::c_int
        }) as WEBP_CSP_MODE;
    }
    idec = WebPINewDecoder(::core::ptr::null_mut::<WebPDecBuffer>());
    if idec.is_null() {
        return ::core::ptr::null_mut::<WebPIDecoder>();
    }
    (*idec).output_.colorspace = colorspace;
    (*idec).output_.is_external_memory = is_external_memory;
    (*idec).output_.u.YUVA.y = luma;
    (*idec).output_.u.YUVA.y_stride = luma_stride;
    (*idec).output_.u.YUVA.y_size = luma_size;
    (*idec).output_.u.YUVA.u = u;
    (*idec).output_.u.YUVA.u_stride = u_stride;
    (*idec).output_.u.YUVA.u_size = u_size;
    (*idec).output_.u.YUVA.v = v;
    (*idec).output_.u.YUVA.v_stride = v_stride;
    (*idec).output_.u.YUVA.v_size = v_size;
    (*idec).output_.u.YUVA.a = a;
    (*idec).output_.u.YUVA.a_stride = a_stride;
    (*idec).output_.u.YUVA.a_size = a_size;
    return idec;
}
#[no_mangle]
pub unsafe extern "C" fn WebPINewYUV(
    mut luma: *mut uint8_t,
    mut luma_size: size_t,
    mut luma_stride: ::core::ffi::c_int,
    mut u: *mut uint8_t,
    mut u_size: size_t,
    mut u_stride: ::core::ffi::c_int,
    mut v: *mut uint8_t,
    mut v_size: size_t,
    mut v_stride: ::core::ffi::c_int,
) -> *mut WebPIDecoder {
    return WebPINewYUVA(
        luma,
        luma_size,
        luma_stride,
        u,
        u_size,
        u_stride,
        v,
        v_size,
        v_stride,
        ::core::ptr::null_mut::<uint8_t>(),
        0 as size_t,
        0 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn IDecCheckStatus(idec: *const WebPIDecoder) -> VP8StatusCode {
    '_c2rust_label: {
        if !idec.is_null() {
        } else {
            __assert_fail(
                b"idec\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/idec_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                771 as ::core::ffi::c_uint,
                b"VP8StatusCode IDecCheckStatus(const WebPIDecoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*idec).state_ as ::core::ffi::c_uint
        == STATE_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return VP8_STATUS_BITSTREAM_ERROR;
    }
    if (*idec).state_ as ::core::ffi::c_uint
        == STATE_DONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return VP8_STATUS_OK;
    }
    return VP8_STATUS_SUSPENDED;
}
#[no_mangle]
pub unsafe extern "C" fn WebPIAppend(
    mut idec: *mut WebPIDecoder,
    mut data: *const uint8_t,
    mut data_size: size_t,
) -> VP8StatusCode {
    let mut status: VP8StatusCode = VP8_STATUS_OK;
    if idec.is_null() || data.is_null() {
        return VP8_STATUS_INVALID_PARAM;
    }
    status = IDecCheckStatus(idec);
    if status as ::core::ffi::c_uint
        != VP8_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return status;
    }
    if CheckMemBufferMode(&raw mut (*idec).mem_, MEM_MODE_APPEND) == 0 {
        return VP8_STATUS_INVALID_PARAM;
    }
    if AppendToMemBuffer(idec, data, data_size) == 0 {
        return VP8_STATUS_OUT_OF_MEMORY;
    }
    return IDecode(idec);
}
#[no_mangle]
pub unsafe extern "C" fn WebPIUpdate(
    mut idec: *mut WebPIDecoder,
    mut data: *const uint8_t,
    mut data_size: size_t,
) -> VP8StatusCode {
    let mut status: VP8StatusCode = VP8_STATUS_OK;
    if idec.is_null() || data.is_null() {
        return VP8_STATUS_INVALID_PARAM;
    }
    status = IDecCheckStatus(idec);
    if status as ::core::ffi::c_uint
        != VP8_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return status;
    }
    if CheckMemBufferMode(&raw mut (*idec).mem_, MEM_MODE_MAP) == 0 {
        return VP8_STATUS_INVALID_PARAM;
    }
    if RemapMemBuffer(idec, data, data_size) == 0 {
        return VP8_STATUS_INVALID_PARAM;
    }
    return IDecode(idec);
}
unsafe extern "C" fn GetOutputBuffer(idec: *const WebPIDecoder) -> *const WebPDecBuffer {
    if idec.is_null() || (*idec).dec_.is_null() {
        return ::core::ptr::null::<WebPDecBuffer>();
    }
    if (*idec).state_ as ::core::ffi::c_uint
        <= STATE_VP8_PARTS0 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null::<WebPDecBuffer>();
    }
    if !(*idec).final_output_.is_null() {
        return ::core::ptr::null::<WebPDecBuffer>();
    }
    return (*idec).params_.output;
}
#[no_mangle]
pub unsafe extern "C" fn WebPIDecodedArea(
    mut idec: *const WebPIDecoder,
    mut left: *mut ::core::ffi::c_int,
    mut top: *mut ::core::ffi::c_int,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
) -> *const WebPDecBuffer {
    let src: *const WebPDecBuffer = GetOutputBuffer(idec) as *const WebPDecBuffer;
    if !left.is_null() {
        *left = 0 as ::core::ffi::c_int;
    }
    if !top.is_null() {
        *top = 0 as ::core::ffi::c_int;
    }
    if !src.is_null() {
        if !width.is_null() {
            *width = (*src).width;
        }
        if !height.is_null() {
            *height = (*idec).params_.last_y;
        }
    } else {
        if !width.is_null() {
            *width = 0 as ::core::ffi::c_int;
        }
        if !height.is_null() {
            *height = 0 as ::core::ffi::c_int;
        }
    }
    return src;
}
#[no_mangle]
pub unsafe extern "C" fn WebPIDecGetRGB(
    mut idec: *const WebPIDecoder,
    mut last_y: *mut ::core::ffi::c_int,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
    mut stride: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let src: *const WebPDecBuffer = GetOutputBuffer(idec) as *const WebPDecBuffer;
    if src.is_null() {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    if (*src).colorspace as ::core::ffi::c_uint
        >= MODE_YUV as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    if !last_y.is_null() {
        *last_y = (*idec).params_.last_y;
    }
    if !width.is_null() {
        *width = (*src).width;
    }
    if !height.is_null() {
        *height = (*src).height;
    }
    if !stride.is_null() {
        *stride = (*src).u.RGBA.stride;
    }
    return (*src).u.RGBA.rgba;
}
#[no_mangle]
pub unsafe extern "C" fn WebPIDecGetYUVA(
    mut idec: *const WebPIDecoder,
    mut last_y: *mut ::core::ffi::c_int,
    mut u: *mut *mut uint8_t,
    mut v: *mut *mut uint8_t,
    mut a: *mut *mut uint8_t,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
    mut stride: *mut ::core::ffi::c_int,
    mut uv_stride: *mut ::core::ffi::c_int,
    mut a_stride: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let src: *const WebPDecBuffer = GetOutputBuffer(idec) as *const WebPDecBuffer;
    if src.is_null() {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    if ((*src).colorspace as ::core::ffi::c_uint)
        < MODE_YUV as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    if !last_y.is_null() {
        *last_y = (*idec).params_.last_y;
    }
    if !u.is_null() {
        *u = (*src).u.YUVA.u;
    }
    if !v.is_null() {
        *v = (*src).u.YUVA.v;
    }
    if !a.is_null() {
        *a = (*src).u.YUVA.a;
    }
    if !width.is_null() {
        *width = (*src).width;
    }
    if !height.is_null() {
        *height = (*src).height;
    }
    if !stride.is_null() {
        *stride = (*src).u.YUVA.y_stride;
    }
    if !uv_stride.is_null() {
        *uv_stride = (*src).u.YUVA.u_stride;
    }
    if !a_stride.is_null() {
        *a_stride = (*src).u.YUVA.a_stride;
    }
    return (*src).u.YUVA.y;
}
#[no_mangle]
pub unsafe extern "C" fn WebPISetIOHooks(
    idec: *mut WebPIDecoder,
    mut put: VP8IoPutHook,
    mut setup: VP8IoSetupHook,
    mut teardown: VP8IoTeardownHook,
    mut user_data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if idec.is_null()
        || (*idec).state_ as ::core::ffi::c_uint
            > STATE_WEBP_HEADER as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    (*idec).io_.put = put;
    (*idec).io_.setup = setup;
    (*idec).io_.teardown = teardown;
    (*idec).io_.opaque = user_data;
    return 1 as ::core::ffi::c_int;
}
