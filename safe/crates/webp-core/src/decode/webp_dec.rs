#[repr(C)]
pub struct ALPHDecoder {
    _unused: [u8; 0],
}

extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn WebPRescalerGetScaledDimensions(
        src_width: ::core::ffi::c_int,
        src_height: ::core::ffi::c_int,
        scaled_width: *mut ::core::ffi::c_int,
        scaled_height: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPInitDecBufferInternal(
        _: *mut WebPDecBuffer,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPFreeDecBuffer(buffer: *mut WebPDecBuffer);
    fn VP8InitIoInternal(_: *mut VP8Io, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn VP8New() -> *mut VP8Decoder;
    fn VP8GetHeaders(dec: *mut VP8Decoder, io: *mut VP8Io) -> ::core::ffi::c_int;
    fn VP8Decode(dec: *mut VP8Decoder, io: *mut VP8Io) -> ::core::ffi::c_int;
    fn VP8Delete(dec: *mut VP8Decoder);
    fn VP8GetInfo(
        data: *const uint8_t,
        data_size: size_t,
        chunk_size: size_t,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VP8LCheckSignature(data: *const uint8_t, size: size_t) -> ::core::ffi::c_int;
    fn VP8LGetInfo(
        data: *const uint8_t,
        data_size: size_t,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
        has_alpha: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPInitCustomIo(params: *mut WebPDecParams, io: *mut VP8Io);
    fn WebPAllocateDecBuffer(
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        options: *const WebPDecoderOptions,
        buffer: *mut WebPDecBuffer,
    ) -> VP8StatusCode;
    fn WebPFlipBuffer(buffer: *mut WebPDecBuffer) -> VP8StatusCode;
    fn WebPCopyDecBuffer(src: *const WebPDecBuffer, dst: *mut WebPDecBuffer);
    fn WebPCopyDecBufferPixels(src: *const WebPDecBuffer, dst: *mut WebPDecBuffer)
        -> VP8StatusCode;
    fn WebPAvoidSlowMemory(
        output: *const WebPDecBuffer,
        features: *const WebPBitstreamFeatures,
    ) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn VP8LNew() -> *mut VP8LDecoder;
    fn VP8LDecodeHeader(dec: *mut VP8LDecoder, io: *mut VP8Io) -> ::core::ffi::c_int;
    fn VP8LDecodeImage(dec: *mut VP8LDecoder) -> ::core::ffi::c_int;
    fn VP8LDelete(dec: *mut VP8LDecoder);
    fn VP8GetThreadMethod(
        options: *const WebPDecoderOptions,
        headers: *const WebPHeaderStructure,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VP8InitDithering(options: *const WebPDecoderOptions, dec: *mut VP8Decoder);
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
pub struct WebPDecoderConfig {
    pub input: WebPBitstreamFeatures,
    pub output: WebPDecBuffer,
    pub options: WebPDecoderOptions,
}
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
pub const ALPHA_FLAG: WebPFeatureFlags = 16;
pub const ANIMATION_FLAG: WebPFeatureFlags = 2;
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
pub type OutputFunc =
    Option<unsafe extern "C" fn(*const VP8Io, *mut WebPDecParams) -> ::core::ffi::c_int>;
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
pub type WebPFeatureFlags = ::core::ffi::c_uint;
pub const ALL_VALID_FLAGS: WebPFeatureFlags = 62;
pub const ICCP_FLAG: WebPFeatureFlags = 32;
pub const EXIF_FLAG: WebPFeatureFlags = 8;
pub const XMP_FLAG: WebPFeatureFlags = 4;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const WEBP_DECODER_ABI_VERSION: ::core::ffi::c_int = 0x209 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn WebPIsRGBMode(mut mode: WEBP_CSP_MODE) -> ::core::ffi::c_int {
    return ((mode as ::core::ffi::c_uint) < MODE_YUV as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn WebPInitDecBuffer(mut buffer: *mut WebPDecBuffer) -> ::core::ffi::c_int {
    return WebPInitDecBufferInternal(buffer, WEBP_DECODER_ABI_VERSION);
}
#[inline]
unsafe extern "C" fn VP8InitIo(io: *mut VP8Io) -> ::core::ffi::c_int {
    return VP8InitIoInternal(io, WEBP_DECODER_ABI_VERSION);
}
pub const VP8_FRAME_HEADER_SIZE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const VP8L_FRAME_HEADER_SIZE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const TAG_SIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CHUNK_HEADER_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const RIFF_HEADER_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const VP8X_CHUNK_SIZE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const MAX_IMAGE_AREA: ::core::ffi::c_ulonglong =
    (1 as ::core::ffi::c_ulonglong) << 32 as ::core::ffi::c_int;
pub const MAX_CHUNK_PAYLOAD: ::core::ffi::c_uint = (!(0 as ::core::ffi::c_uint))
    .wrapping_sub(CHUNK_HEADER_SIZE as ::core::ffi::c_uint)
    .wrapping_sub(1 as ::core::ffi::c_uint);
#[inline]
unsafe extern "C" fn GetLE16(data: *const uint8_t) -> ::core::ffi::c_int {
    return (*data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        << 0 as ::core::ffi::c_int
        | (*data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn GetLE24(data: *const uint8_t) -> ::core::ffi::c_int {
    return GetLE16(data)
        | (*data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn GetLE32(data: *const uint8_t) -> uint32_t {
    return GetLE16(data) as uint32_t
        | (GetLE16(data.offset(2 as ::core::ffi::c_int as isize)) as uint32_t)
            << 16 as ::core::ffi::c_int;
}
unsafe extern "C" fn ParseRIFF(
    data: *mut *const uint8_t,
    data_size: *mut size_t,
    mut have_all_data: ::core::ffi::c_int,
    riff_size: *mut size_t,
) -> VP8StatusCode {
    '_c2rust_label: {
        if !data.is_null() {
        } else {
            __assert_fail(
                b"data != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                57 as ::core::ffi::c_uint,
                b"VP8StatusCode ParseRIFF(const uint8_t **const, size_t *const, int, size_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !data_size.is_null() {
        } else {
            __assert_fail(
                b"data_size != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                58 as ::core::ffi::c_uint,
                b"VP8StatusCode ParseRIFF(const uint8_t **const, size_t *const, int, size_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !riff_size.is_null() {
        } else {
            __assert_fail(
                b"riff_size != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                59 as ::core::ffi::c_uint,
                b"VP8StatusCode ParseRIFF(const uint8_t **const, size_t *const, int, size_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    *riff_size = 0 as size_t;
    if *data_size >= RIFF_HEADER_SIZE as size_t
        && memcmp(
            *data as *const ::core::ffi::c_void,
            b"RIFF\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            TAG_SIZE as size_t,
        ) == 0
    {
        if memcmp(
            (*data).offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            b"WEBP\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            TAG_SIZE as size_t,
        ) != 0
        {
            return VP8_STATUS_BITSTREAM_ERROR;
        } else {
            let size: uint32_t = GetLE32((*data).offset(TAG_SIZE as isize)) as uint32_t;
            if size < (TAG_SIZE + CHUNK_HEADER_SIZE) as uint32_t {
                return VP8_STATUS_BITSTREAM_ERROR;
            }
            if size > MAX_CHUNK_PAYLOAD as uint32_t {
                return VP8_STATUS_BITSTREAM_ERROR;
            }
            if have_all_data != 0
                && size as size_t > (*data_size).wrapping_sub(CHUNK_HEADER_SIZE as size_t)
            {
                return VP8_STATUS_NOT_ENOUGH_DATA;
            }
            *riff_size = size as size_t;
            *data = (*data).offset(RIFF_HEADER_SIZE as isize);
            *data_size = (*data_size).wrapping_sub(RIFF_HEADER_SIZE as size_t);
        }
    }
    return VP8_STATUS_OK;
}
unsafe extern "C" fn ParseVP8X(
    data: *mut *const uint8_t,
    data_size: *mut size_t,
    found_vp8x: *mut ::core::ffi::c_int,
    width_ptr: *mut ::core::ffi::c_int,
    height_ptr: *mut ::core::ffi::c_int,
    flags_ptr: *mut uint32_t,
) -> VP8StatusCode {
    let vp8x_size: uint32_t = (CHUNK_HEADER_SIZE + VP8X_CHUNK_SIZE) as uint32_t;
    '_c2rust_label: {
        if !data.is_null() {
        } else {
            __assert_fail(
                b"data != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                99 as ::core::ffi::c_uint,
                b"VP8StatusCode ParseVP8X(const uint8_t **const, size_t *const, int *const, int *const, int *const, uint32_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !data_size.is_null() {
        } else {
            __assert_fail(
                b"data_size != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                100 as ::core::ffi::c_uint,
                b"VP8StatusCode ParseVP8X(const uint8_t **const, size_t *const, int *const, int *const, int *const, uint32_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !found_vp8x.is_null() {
        } else {
            __assert_fail(
                b"found_vp8x != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                101 as ::core::ffi::c_uint,
                b"VP8StatusCode ParseVP8X(const uint8_t **const, size_t *const, int *const, int *const, int *const, uint32_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    *found_vp8x = 0 as ::core::ffi::c_int;
    if *data_size < CHUNK_HEADER_SIZE as size_t {
        return VP8_STATUS_NOT_ENOUGH_DATA;
    }
    if memcmp(
        *data as *const ::core::ffi::c_void,
        b"VP8X\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        TAG_SIZE as size_t,
    ) == 0
    {
        let mut width: ::core::ffi::c_int = 0;
        let mut height: ::core::ffi::c_int = 0;
        let mut flags: uint32_t = 0;
        let chunk_size: uint32_t = GetLE32((*data).offset(TAG_SIZE as isize)) as uint32_t;
        if chunk_size != VP8X_CHUNK_SIZE as uint32_t {
            return VP8_STATUS_BITSTREAM_ERROR;
        }
        if *data_size < vp8x_size as size_t {
            return VP8_STATUS_NOT_ENOUGH_DATA;
        }
        flags = GetLE32((*data).offset(8 as ::core::ffi::c_int as isize));
        width =
            1 as ::core::ffi::c_int + GetLE24((*data).offset(12 as ::core::ffi::c_int as isize));
        height =
            1 as ::core::ffi::c_int + GetLE24((*data).offset(15 as ::core::ffi::c_int as isize));
        if (width as uint64_t).wrapping_mul(height as uint64_t) as ::core::ffi::c_ulonglong
            >= MAX_IMAGE_AREA
        {
            return VP8_STATUS_BITSTREAM_ERROR;
        }
        if !flags_ptr.is_null() {
            *flags_ptr = flags;
        }
        if !width_ptr.is_null() {
            *width_ptr = width;
        }
        if !height_ptr.is_null() {
            *height_ptr = height;
        }
        *data = (*data).offset(vp8x_size as isize);
        *data_size = (*data_size).wrapping_sub(vp8x_size as size_t);
        *found_vp8x = 1 as ::core::ffi::c_int;
    }
    return VP8_STATUS_OK;
}
unsafe extern "C" fn ParseOptionalChunks(
    data: *mut *const uint8_t,
    data_size: *mut size_t,
    riff_size: size_t,
    alpha_data: *mut *const uint8_t,
    alpha_size: *mut size_t,
) -> VP8StatusCode {
    let mut buf: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut buf_size: size_t = 0;
    let mut total_size: uint32_t = (TAG_SIZE + CHUNK_HEADER_SIZE + VP8X_CHUNK_SIZE) as uint32_t;
    '_c2rust_label: {
        if !data.is_null() {
        } else {
            __assert_fail(
                b"data != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                156 as ::core::ffi::c_uint,
                b"VP8StatusCode ParseOptionalChunks(const uint8_t **const, size_t *const, const size_t, const uint8_t **const, size_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !data_size.is_null() {
        } else {
            __assert_fail(
                b"data_size != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                157 as ::core::ffi::c_uint,
                b"VP8StatusCode ParseOptionalChunks(const uint8_t **const, size_t *const, const size_t, const uint8_t **const, size_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    buf = *data;
    buf_size = *data_size;
    '_c2rust_label_1: {
        if !alpha_data.is_null() {
        } else {
            __assert_fail(
                b"alpha_data != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                161 as ::core::ffi::c_uint,
                b"VP8StatusCode ParseOptionalChunks(const uint8_t **const, size_t *const, const size_t, const uint8_t **const, size_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if !alpha_size.is_null() {
        } else {
            __assert_fail(
                b"alpha_size != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                162 as ::core::ffi::c_uint,
                b"VP8StatusCode ParseOptionalChunks(const uint8_t **const, size_t *const, const size_t, const uint8_t **const, size_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    *alpha_data = ::core::ptr::null::<uint8_t>();
    *alpha_size = 0 as size_t;
    loop {
        let mut chunk_size: uint32_t = 0;
        let mut disk_chunk_size: uint32_t = 0;
        *data = buf;
        *data_size = buf_size;
        if buf_size < CHUNK_HEADER_SIZE as size_t {
            return VP8_STATUS_NOT_ENOUGH_DATA;
        }
        chunk_size = GetLE32(buf.offset(TAG_SIZE as isize));
        if chunk_size > MAX_CHUNK_PAYLOAD as uint32_t {
            return VP8_STATUS_BITSTREAM_ERROR;
        }
        disk_chunk_size = (CHUNK_HEADER_SIZE as uint32_t)
            .wrapping_add(chunk_size)
            .wrapping_add(1 as uint32_t)
            & !(1 as uint32_t);
        total_size = total_size.wrapping_add(disk_chunk_size);
        if riff_size > 0 as size_t && total_size as size_t > riff_size {
            return VP8_STATUS_BITSTREAM_ERROR;
        }
        if memcmp(
            buf as *const ::core::ffi::c_void,
            b"VP8 \0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            TAG_SIZE as size_t,
        ) == 0
            || memcmp(
                buf as *const ::core::ffi::c_void,
                b"VP8L\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                TAG_SIZE as size_t,
            ) == 0
        {
            return VP8_STATUS_OK;
        }
        if buf_size < disk_chunk_size as size_t {
            return VP8_STATUS_NOT_ENOUGH_DATA;
        }
        if memcmp(
            buf as *const ::core::ffi::c_void,
            b"ALPH\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            TAG_SIZE as size_t,
        ) == 0
        {
            *alpha_data = buf.offset(CHUNK_HEADER_SIZE as isize);
            *alpha_size = chunk_size as size_t;
        }
        buf = buf.offset(disk_chunk_size as isize);
        buf_size = buf_size.wrapping_sub(disk_chunk_size as size_t);
    }
}
unsafe extern "C" fn ParseVP8Header(
    data_ptr: *mut *const uint8_t,
    data_size: *mut size_t,
    mut have_all_data: ::core::ffi::c_int,
    mut riff_size: size_t,
    chunk_size: *mut size_t,
    is_lossless: *mut ::core::ffi::c_int,
) -> VP8StatusCode {
    let data: *const uint8_t = *data_ptr;
    let is_vp8: ::core::ffi::c_int = (memcmp(
        data as *const ::core::ffi::c_void,
        b"VP8 \0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        TAG_SIZE as size_t,
    ) == 0) as ::core::ffi::c_int;
    let is_vp8l: ::core::ffi::c_int = (memcmp(
        data as *const ::core::ffi::c_void,
        b"VP8L\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        TAG_SIZE as size_t,
    ) == 0) as ::core::ffi::c_int;
    let minimal_size: uint32_t = (TAG_SIZE + CHUNK_HEADER_SIZE) as uint32_t;
    '_c2rust_label: {
        if !data.is_null() {
        } else {
            __assert_fail(
                b"data != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                232 as ::core::ffi::c_uint,
                b"VP8StatusCode ParseVP8Header(const uint8_t **const, size_t *const, int, size_t, size_t *const, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !data_size.is_null() {
        } else {
            __assert_fail(
                b"data_size != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                233 as ::core::ffi::c_uint,
                b"VP8StatusCode ParseVP8Header(const uint8_t **const, size_t *const, int, size_t, size_t *const, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !chunk_size.is_null() {
        } else {
            __assert_fail(
                b"chunk_size != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                234 as ::core::ffi::c_uint,
                b"VP8StatusCode ParseVP8Header(const uint8_t **const, size_t *const, int, size_t, size_t *const, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if !is_lossless.is_null() {
        } else {
            __assert_fail(
                b"is_lossless != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                235 as ::core::ffi::c_uint,
                b"VP8StatusCode ParseVP8Header(const uint8_t **const, size_t *const, int, size_t, size_t *const, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if *data_size < CHUNK_HEADER_SIZE as size_t {
        return VP8_STATUS_NOT_ENOUGH_DATA;
    }
    if is_vp8 != 0 || is_vp8l != 0 {
        let size: uint32_t = GetLE32(data.offset(TAG_SIZE as isize)) as uint32_t;
        if riff_size >= minimal_size as size_t
            && size as size_t > riff_size.wrapping_sub(minimal_size as size_t)
        {
            return VP8_STATUS_BITSTREAM_ERROR;
        }
        if have_all_data != 0
            && size as size_t > (*data_size).wrapping_sub(CHUNK_HEADER_SIZE as size_t)
        {
            return VP8_STATUS_NOT_ENOUGH_DATA;
        }
        *chunk_size = size as size_t;
        *data_ptr = (*data_ptr).offset(CHUNK_HEADER_SIZE as isize);
        *data_size = (*data_size).wrapping_sub(CHUNK_HEADER_SIZE as size_t);
        *is_lossless = is_vp8l;
    } else {
        *is_lossless = VP8LCheckSignature(data, *data_size);
        *chunk_size = *data_size;
    }
    return VP8_STATUS_OK;
}
unsafe extern "C" fn ParseHeadersInternal(
    mut data: *const uint8_t,
    mut data_size: size_t,
    width: *mut ::core::ffi::c_int,
    height: *mut ::core::ffi::c_int,
    has_alpha: *mut ::core::ffi::c_int,
    has_animation: *mut ::core::ffi::c_int,
    format: *mut ::core::ffi::c_int,
    headers: *mut WebPHeaderStructure,
) -> VP8StatusCode {
    let mut current_block: u64;
    let mut canvas_width: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut canvas_height: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut image_width: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut image_height: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut found_riff: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut found_vp8x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut animation_present: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let have_all_data: ::core::ffi::c_int = if !headers.is_null() {
        (*headers).have_all_data
    } else {
        0 as ::core::ffi::c_int
    };
    let mut status: VP8StatusCode = VP8_STATUS_OK;
    let mut hdrs: WebPHeaderStructure = WebPHeaderStructure {
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
    if data.is_null() || data_size < RIFF_HEADER_SIZE as size_t {
        return VP8_STATUS_NOT_ENOUGH_DATA;
    }
    memset(
        &raw mut hdrs as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<WebPHeaderStructure>() as size_t,
    );
    hdrs.data = data;
    hdrs.data_size = data_size;
    status = ParseRIFF(
        &raw mut data,
        &raw mut data_size,
        have_all_data,
        &raw mut hdrs.riff_size,
    );
    if status as ::core::ffi::c_uint != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        return status;
    }
    found_riff = (hdrs.riff_size > 0 as size_t) as ::core::ffi::c_int;
    let mut flags: uint32_t = 0 as uint32_t;
    status = ParseVP8X(
        &raw mut data,
        &raw mut data_size,
        &raw mut found_vp8x,
        &raw mut canvas_width,
        &raw mut canvas_height,
        &raw mut flags,
    );
    if status as ::core::ffi::c_uint != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        return status;
    }
    animation_present =
        (flags & ANIMATION_FLAG as ::core::ffi::c_int as uint32_t != 0) as ::core::ffi::c_int;
    if found_riff == 0 && found_vp8x != 0 {
        return VP8_STATUS_BITSTREAM_ERROR;
    }
    if !has_alpha.is_null() {
        *has_alpha =
            (flags & ALPHA_FLAG as ::core::ffi::c_int as uint32_t != 0) as ::core::ffi::c_int;
    }
    if !has_animation.is_null() {
        *has_animation = animation_present;
    }
    if !format.is_null() {
        *format = 0 as ::core::ffi::c_int;
    }
    image_width = canvas_width;
    image_height = canvas_height;
    if found_vp8x != 0 && animation_present != 0 && headers.is_null() {
        status = VP8_STATUS_OK;
    } else if data_size < TAG_SIZE as size_t {
        status = VP8_STATUS_NOT_ENOUGH_DATA;
    } else {
        if found_riff != 0 && found_vp8x != 0
            || found_riff == 0
                && found_vp8x == 0
                && memcmp(
                    data as *const ::core::ffi::c_void,
                    b"ALPH\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    TAG_SIZE as size_t,
                ) == 0
        {
            status = ParseOptionalChunks(
                &raw mut data,
                &raw mut data_size,
                hdrs.riff_size,
                &raw mut hdrs.alpha_data,
                &raw mut hdrs.alpha_data_size,
            );
            if status as ::core::ffi::c_uint
                != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                current_block = 17309821763151109009;
            } else {
                current_block = 17788412896529399552;
            }
        } else {
            current_block = 17788412896529399552;
        }
        match current_block {
            17309821763151109009 => {}
            _ => {
                status = ParseVP8Header(
                    &raw mut data,
                    &raw mut data_size,
                    have_all_data,
                    hdrs.riff_size,
                    &raw mut hdrs.compressed_size,
                    &raw mut hdrs.is_lossless,
                );
                if !(status as ::core::ffi::c_uint
                    != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint)
                {
                    if hdrs.compressed_size > MAX_CHUNK_PAYLOAD as size_t {
                        return VP8_STATUS_BITSTREAM_ERROR;
                    }
                    if !format.is_null() && animation_present == 0 {
                        *format = if hdrs.is_lossless != 0 {
                            2 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        };
                    }
                    if hdrs.is_lossless == 0 {
                        if data_size < VP8_FRAME_HEADER_SIZE as size_t {
                            status = VP8_STATUS_NOT_ENOUGH_DATA;
                            current_block = 17309821763151109009;
                        } else {
                            if VP8GetInfo(
                                data,
                                data_size,
                                hdrs.compressed_size as uint32_t as size_t,
                                &raw mut image_width,
                                &raw mut image_height,
                            ) == 0
                            {
                                return VP8_STATUS_BITSTREAM_ERROR;
                            }
                            current_block = 6174974146017752131;
                        }
                    } else if data_size < VP8L_FRAME_HEADER_SIZE as size_t {
                        status = VP8_STATUS_NOT_ENOUGH_DATA;
                        current_block = 17309821763151109009;
                    } else {
                        if VP8LGetInfo(
                            data,
                            data_size,
                            &raw mut image_width,
                            &raw mut image_height,
                            has_alpha,
                        ) == 0
                        {
                            return VP8_STATUS_BITSTREAM_ERROR;
                        }
                        current_block = 6174974146017752131;
                    }
                    match current_block {
                        17309821763151109009 => {}
                        _ => {
                            if found_vp8x != 0 {
                                if canvas_width != image_width || canvas_height != image_height {
                                    return VP8_STATUS_BITSTREAM_ERROR;
                                }
                            }
                            if !headers.is_null() {
                                *headers = hdrs;
                                (*headers).offset = data.offset_from((*headers).data)
                                    as ::core::ffi::c_long
                                    as size_t;
                                '_c2rust_label: {
                                    if (data.offset_from((*headers).data) as ::core::ffi::c_long
                                        as uint64_t)
                                        < (!(0 as ::core::ffi::c_uint))
                                            .wrapping_sub(8 as ::core::ffi::c_uint)
                                            .wrapping_sub(1 as ::core::ffi::c_uint)
                                            as uint64_t
                                    {
                                    } else {
                                        __assert_fail(
                                            b"(uint64_t)(data - headers->data) < MAX_CHUNK_PAYLOAD\0"
                                                as *const u8 as *const ::core::ffi::c_char,
                                            b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                                                as *const u8 as *const ::core::ffi::c_char,
                                            395 as ::core::ffi::c_uint,
                                            b"VP8StatusCode ParseHeadersInternal(const uint8_t *, size_t, int *const, int *const, int *const, int *const, int *const, WebPHeaderStructure *const)\0"
                                                as *const u8 as *const ::core::ffi::c_char,
                                        );
                                    }
                                };
                                '_c2rust_label_0: {
                                    if (*headers).offset
                                        == (*headers).data_size.wrapping_sub(data_size)
                                    {
                                    } else {
                                        __assert_fail(
                                            b"headers->offset == headers->data_size - data_size\0"
                                                as *const u8 as *const ::core::ffi::c_char,
                                            b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                                                as *const u8 as *const ::core::ffi::c_char,
                                            396 as ::core::ffi::c_uint,
                                            b"VP8StatusCode ParseHeadersInternal(const uint8_t *, size_t, int *const, int *const, int *const, int *const, int *const, WebPHeaderStructure *const)\0"
                                                as *const u8 as *const ::core::ffi::c_char,
                                        );
                                    }
                                };
                            }
                        }
                    }
                }
            }
        }
    }
    if status as ::core::ffi::c_uint == VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        || status as ::core::ffi::c_uint
            == VP8_STATUS_NOT_ENOUGH_DATA as ::core::ffi::c_int as ::core::ffi::c_uint
            && found_vp8x != 0
            && headers.is_null()
    {
        if !has_alpha.is_null() {
            *has_alpha |= (hdrs.alpha_data != NULL as *const uint8_t) as ::core::ffi::c_int;
        }
        if !width.is_null() {
            *width = image_width;
        }
        if !height.is_null() {
            *height = image_height;
        }
        return VP8_STATUS_OK;
    } else {
        return status;
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPParseHeaders(headers: *mut WebPHeaderStructure) -> VP8StatusCode {
    let mut status: VP8StatusCode = VP8_STATUS_OK;
    let mut has_animation: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    '_c2rust_label: {
        if !headers.is_null() {
        } else {
            __assert_fail(
                b"headers != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                418 as ::core::ffi::c_uint,
                b"VP8StatusCode WebPParseHeaders(WebPHeaderStructure *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    ::core::ptr::write_volatile(
        &mut status as *mut VP8StatusCode,
        ParseHeadersInternal(
            (*headers).data,
            (*headers).data_size,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            &raw mut has_animation,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            headers,
        ),
    );
    if status as ::core::ffi::c_uint == VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        || status as ::core::ffi::c_uint
            == VP8_STATUS_NOT_ENOUGH_DATA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if has_animation != 0 {
            ::core::ptr::write_volatile(
                &mut status as *mut VP8StatusCode,
                VP8_STATUS_UNSUPPORTED_FEATURE,
            );
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn WebPResetDecParams(params: *mut WebPDecParams) {
    if !params.is_null() {
        memset(
            params as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<WebPDecParams>() as size_t,
        );
    }
}
unsafe extern "C" fn DecodeInto(
    data: *const uint8_t,
    mut data_size: size_t,
    params: *mut WebPDecParams,
) -> VP8StatusCode {
    let mut status: VP8StatusCode = VP8_STATUS_OK;
    let mut io: VP8Io = VP8Io {
        width: 0,
        height: 0,
        mb_y: 0,
        mb_w: 0,
        mb_h: 0,
        y: ::core::ptr::null::<uint8_t>(),
        u: ::core::ptr::null::<uint8_t>(),
        v: ::core::ptr::null::<uint8_t>(),
        y_stride: 0,
        uv_stride: 0,
        opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        put: None,
        setup: None,
        teardown: None,
        fancy_upsampling: 0,
        data_size: 0,
        data: ::core::ptr::null::<uint8_t>(),
        bypass_filtering: 0,
        use_cropping: 0,
        crop_left: 0,
        crop_right: 0,
        crop_top: 0,
        crop_bottom: 0,
        use_scaling: 0,
        scaled_width: 0,
        scaled_height: 0,
        a: ::core::ptr::null::<uint8_t>(),
    };
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
    headers.data_size = data_size;
    headers.have_all_data = 1 as ::core::ffi::c_int;
    status = WebPParseHeaders(&raw mut headers);
    if status as ::core::ffi::c_uint != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        return status;
    }
    '_c2rust_label: {
        if !params.is_null() {
        } else {
            __assert_fail(
                b"params != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                461 as ::core::ffi::c_uint,
                b"VP8StatusCode DecodeInto(const uint8_t *const, size_t, WebPDecParams *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    VP8InitIo(&raw mut io);
    io.data = headers.data.offset(headers.offset as isize);
    io.data_size = headers.data_size.wrapping_sub(headers.offset);
    WebPInitCustomIo(params, &raw mut io);
    if headers.is_lossless == 0 {
        let dec: *mut VP8Decoder = VP8New() as *mut VP8Decoder;
        if dec.is_null() {
            return VP8_STATUS_OUT_OF_MEMORY;
        }
        (*dec).alpha_data_ = headers.alpha_data;
        (*dec).alpha_data_size_ = headers.alpha_data_size;
        if VP8GetHeaders(dec, &raw mut io) == 0 {
            status = (*dec).status_;
        } else {
            status =
                WebPAllocateDecBuffer(io.width, io.height, (*params).options, (*params).output);
            if status as ::core::ffi::c_uint
                == VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*dec).mt_method_ =
                    VP8GetThreadMethod((*params).options, &raw mut headers, io.width, io.height);
                VP8InitDithering((*params).options, dec);
                if VP8Decode(dec, &raw mut io) == 0 {
                    status = (*dec).status_;
                }
            }
        }
        VP8Delete(dec);
    } else {
        let dec_0: *mut VP8LDecoder = VP8LNew() as *mut VP8LDecoder;
        if dec_0.is_null() {
            return VP8_STATUS_OUT_OF_MEMORY;
        }
        if VP8LDecodeHeader(dec_0, &raw mut io) == 0 {
            status = (*dec_0).status_;
        } else {
            status =
                WebPAllocateDecBuffer(io.width, io.height, (*params).options, (*params).output);
            if status as ::core::ffi::c_uint
                == VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if VP8LDecodeImage(dec_0) == 0 {
                    status = (*dec_0).status_;
                }
            }
        }
        VP8LDelete(dec_0);
    }
    if status as ::core::ffi::c_uint != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        WebPFreeDecBuffer((*params).output);
    } else if !(*params).options.is_null() && (*(*params).options).flip != 0 {
        status = WebPFlipBuffer((*params).output);
    }
    return status;
}
unsafe extern "C" fn DecodeIntoRGBABuffer(
    mut colorspace: WEBP_CSP_MODE,
    data: *const uint8_t,
    mut data_size: size_t,
    rgba: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
    mut size: size_t,
) -> *mut uint8_t {
    let mut params: WebPDecParams = WebPDecParams {
        output: ::core::ptr::null_mut::<WebPDecBuffer>(),
        tmp_y: ::core::ptr::null_mut::<uint8_t>(),
        tmp_u: ::core::ptr::null_mut::<uint8_t>(),
        tmp_v: ::core::ptr::null_mut::<uint8_t>(),
        last_y: 0,
        options: ::core::ptr::null::<WebPDecoderOptions>(),
        scaler_y: ::core::ptr::null_mut::<WebPRescaler>(),
        scaler_u: ::core::ptr::null_mut::<WebPRescaler>(),
        scaler_v: ::core::ptr::null_mut::<WebPRescaler>(),
        scaler_a: ::core::ptr::null_mut::<WebPRescaler>(),
        memory: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        emit: None,
        emit_alpha: None,
        emit_alpha_row: None,
    };
    let mut buf: WebPDecBuffer = WebPDecBuffer {
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
    };
    if rgba.is_null() {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    WebPInitDecBuffer(&raw mut buf);
    WebPResetDecParams(&raw mut params);
    params.output = &raw mut buf;
    buf.colorspace = colorspace;
    buf.u.RGBA.rgba = rgba;
    buf.u.RGBA.stride = stride;
    buf.u.RGBA.size = size;
    buf.is_external_memory = 1 as ::core::ffi::c_int;
    if DecodeInto(data, data_size, &raw mut params) as ::core::ffi::c_uint
        != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    return rgba;
}
#[no_mangle]
pub unsafe extern "C" fn WebPDecodeRGBInto(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut output: *mut uint8_t,
    mut size: size_t,
    mut stride: ::core::ffi::c_int,
) -> *mut uint8_t {
    return DecodeIntoRGBABuffer(MODE_RGB, data, data_size, output, stride, size);
}
#[no_mangle]
pub unsafe extern "C" fn WebPDecodeRGBAInto(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut output: *mut uint8_t,
    mut size: size_t,
    mut stride: ::core::ffi::c_int,
) -> *mut uint8_t {
    return DecodeIntoRGBABuffer(MODE_RGBA, data, data_size, output, stride, size);
}
#[no_mangle]
pub unsafe extern "C" fn WebPDecodeARGBInto(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut output: *mut uint8_t,
    mut size: size_t,
    mut stride: ::core::ffi::c_int,
) -> *mut uint8_t {
    return DecodeIntoRGBABuffer(MODE_ARGB, data, data_size, output, stride, size);
}
#[no_mangle]
pub unsafe extern "C" fn WebPDecodeBGRInto(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut output: *mut uint8_t,
    mut size: size_t,
    mut stride: ::core::ffi::c_int,
) -> *mut uint8_t {
    return DecodeIntoRGBABuffer(MODE_BGR, data, data_size, output, stride, size);
}
#[no_mangle]
pub unsafe extern "C" fn WebPDecodeBGRAInto(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut output: *mut uint8_t,
    mut size: size_t,
    mut stride: ::core::ffi::c_int,
) -> *mut uint8_t {
    return DecodeIntoRGBABuffer(MODE_BGRA, data, data_size, output, stride, size);
}
#[no_mangle]
pub unsafe extern "C" fn WebPDecodeYUVInto(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut luma: *mut uint8_t,
    mut luma_size: size_t,
    mut luma_stride: ::core::ffi::c_int,
    mut u: *mut uint8_t,
    mut u_size: size_t,
    mut u_stride: ::core::ffi::c_int,
    mut v: *mut uint8_t,
    mut v_size: size_t,
    mut v_stride: ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut params: WebPDecParams = WebPDecParams {
        output: ::core::ptr::null_mut::<WebPDecBuffer>(),
        tmp_y: ::core::ptr::null_mut::<uint8_t>(),
        tmp_u: ::core::ptr::null_mut::<uint8_t>(),
        tmp_v: ::core::ptr::null_mut::<uint8_t>(),
        last_y: 0,
        options: ::core::ptr::null::<WebPDecoderOptions>(),
        scaler_y: ::core::ptr::null_mut::<WebPRescaler>(),
        scaler_u: ::core::ptr::null_mut::<WebPRescaler>(),
        scaler_v: ::core::ptr::null_mut::<WebPRescaler>(),
        scaler_a: ::core::ptr::null_mut::<WebPRescaler>(),
        memory: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        emit: None,
        emit_alpha: None,
        emit_alpha_row: None,
    };
    let mut output: WebPDecBuffer = WebPDecBuffer {
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
    };
    if luma.is_null() {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    WebPInitDecBuffer(&raw mut output);
    WebPResetDecParams(&raw mut params);
    params.output = &raw mut output;
    output.colorspace = MODE_YUV;
    output.u.YUVA.y = luma;
    output.u.YUVA.y_stride = luma_stride;
    output.u.YUVA.y_size = luma_size;
    output.u.YUVA.u = u;
    output.u.YUVA.u_stride = u_stride;
    output.u.YUVA.u_size = u_size;
    output.u.YUVA.v = v;
    output.u.YUVA.v_stride = v_stride;
    output.u.YUVA.v_size = v_size;
    output.is_external_memory = 1 as ::core::ffi::c_int;
    if DecodeInto(data, data_size, &raw mut params) as ::core::ffi::c_uint
        != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    return luma;
}
unsafe extern "C" fn Decode(
    mut mode: WEBP_CSP_MODE,
    data: *const uint8_t,
    mut data_size: size_t,
    width: *mut ::core::ffi::c_int,
    height: *mut ::core::ffi::c_int,
    keep_info: *mut WebPDecBuffer,
) -> *mut uint8_t {
    let mut params: WebPDecParams = WebPDecParams {
        output: ::core::ptr::null_mut::<WebPDecBuffer>(),
        tmp_y: ::core::ptr::null_mut::<uint8_t>(),
        tmp_u: ::core::ptr::null_mut::<uint8_t>(),
        tmp_v: ::core::ptr::null_mut::<uint8_t>(),
        last_y: 0,
        options: ::core::ptr::null::<WebPDecoderOptions>(),
        scaler_y: ::core::ptr::null_mut::<WebPRescaler>(),
        scaler_u: ::core::ptr::null_mut::<WebPRescaler>(),
        scaler_v: ::core::ptr::null_mut::<WebPRescaler>(),
        scaler_a: ::core::ptr::null_mut::<WebPRescaler>(),
        memory: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        emit: None,
        emit_alpha: None,
        emit_alpha_row: None,
    };
    let mut output: WebPDecBuffer = WebPDecBuffer {
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
    };
    WebPInitDecBuffer(&raw mut output);
    WebPResetDecParams(&raw mut params);
    params.output = &raw mut output;
    output.colorspace = mode;
    if WebPGetInfo(
        data,
        data_size,
        &raw mut output.width,
        &raw mut output.height,
    ) == 0
    {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    if !width.is_null() {
        *width = output.width;
    }
    if !height.is_null() {
        *height = output.height;
    }
    if DecodeInto(data, data_size, &raw mut params) as ::core::ffi::c_uint
        != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    if !keep_info.is_null() {
        WebPCopyDecBuffer(&raw mut output, keep_info);
    }
    return if WebPIsRGBMode(mode) != 0 {
        output.u.RGBA.rgba
    } else {
        output.u.YUVA.y
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPDecodeRGB(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    return Decode(
        MODE_RGB,
        data,
        data_size,
        width,
        height,
        ::core::ptr::null_mut::<WebPDecBuffer>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPDecodeRGBA(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    return Decode(
        MODE_RGBA,
        data,
        data_size,
        width,
        height,
        ::core::ptr::null_mut::<WebPDecBuffer>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPDecodeARGB(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    return Decode(
        MODE_ARGB,
        data,
        data_size,
        width,
        height,
        ::core::ptr::null_mut::<WebPDecBuffer>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPDecodeBGR(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    return Decode(
        MODE_BGR,
        data,
        data_size,
        width,
        height,
        ::core::ptr::null_mut::<WebPDecBuffer>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPDecodeBGRA(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    return Decode(
        MODE_BGRA,
        data,
        data_size,
        width,
        height,
        ::core::ptr::null_mut::<WebPDecBuffer>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPDecodeYUV(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
    mut u: *mut *mut uint8_t,
    mut v: *mut *mut uint8_t,
    mut stride: *mut ::core::ffi::c_int,
    mut uv_stride: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    if u.is_null() || v.is_null() || stride.is_null() || uv_stride.is_null() {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    let mut output: WebPDecBuffer = WebPDecBuffer {
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
    };
    let out: *mut uint8_t =
        Decode(MODE_YUV, data, data_size, width, height, &raw mut output) as *mut uint8_t;
    if !out.is_null() {
        let buf: *const WebPYUVABuffer = &raw mut output.u.YUVA;
        *u = (*buf).u;
        *v = (*buf).v;
        *stride = (*buf).y_stride;
        *uv_stride = (*buf).u_stride;
        '_c2rust_label: {
            if (*buf).u_stride == (*buf).v_stride {
            } else {
                __assert_fail(
                    b"buf->u_stride == buf->v_stride\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    677 as ::core::ffi::c_uint,
                    b"uint8_t *WebPDecodeYUV(const uint8_t *, size_t, int *, int *, uint8_t **, uint8_t **, int *, int *)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
    }
    return out;
}
unsafe extern "C" fn DefaultFeatures(features: *mut WebPBitstreamFeatures) {
    '_c2rust_label: {
        if !features.is_null() {
        } else {
            __assert_fail(
                b"features != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/webp_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                684 as ::core::ffi::c_uint,
                b"void DefaultFeatures(WebPBitstreamFeatures *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    memset(
        features as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<WebPBitstreamFeatures>() as size_t,
    );
}
unsafe extern "C" fn GetFeatures(
    data: *const uint8_t,
    mut data_size: size_t,
    features: *mut WebPBitstreamFeatures,
) -> VP8StatusCode {
    if features.is_null() || data.is_null() {
        return VP8_STATUS_INVALID_PARAM;
    }
    DefaultFeatures(features);
    return ParseHeadersInternal(
        data,
        data_size,
        &raw mut (*features).width,
        &raw mut (*features).height,
        &raw mut (*features).has_alpha,
        &raw mut (*features).has_animation,
        &raw mut (*features).format,
        ::core::ptr::null_mut::<WebPHeaderStructure>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPGetInfo(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut features: WebPBitstreamFeatures = WebPBitstreamFeatures {
        width: 0,
        height: 0,
        has_alpha: 0,
        has_animation: 0,
        format: 0,
        pad: [0; 5],
    };
    if GetFeatures(data, data_size, &raw mut features) as ::core::ffi::c_uint
        != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if !width.is_null() {
        *width = features.width;
    }
    if !height.is_null() {
        *height = features.height;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPInitDecoderConfigInternal(
    mut config: *mut WebPDecoderConfig,
    mut version: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if version >> 8 as ::core::ffi::c_int != 0x209 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if config.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    memset(
        config as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<WebPDecoderConfig>() as size_t,
    );
    DefaultFeatures(&raw mut (*config).input);
    WebPInitDecBuffer(&raw mut (*config).output);
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPGetFeaturesInternal(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut features: *mut WebPBitstreamFeatures,
    mut version: ::core::ffi::c_int,
) -> VP8StatusCode {
    if version >> 8 as ::core::ffi::c_int != 0x209 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
    {
        return VP8_STATUS_INVALID_PARAM;
    }
    if features.is_null() {
        return VP8_STATUS_INVALID_PARAM;
    }
    return GetFeatures(data, data_size, features);
}
#[no_mangle]
pub unsafe extern "C" fn WebPDecode(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut config: *mut WebPDecoderConfig,
) -> VP8StatusCode {
    let mut params: WebPDecParams = WebPDecParams {
        output: ::core::ptr::null_mut::<WebPDecBuffer>(),
        tmp_y: ::core::ptr::null_mut::<uint8_t>(),
        tmp_u: ::core::ptr::null_mut::<uint8_t>(),
        tmp_v: ::core::ptr::null_mut::<uint8_t>(),
        last_y: 0,
        options: ::core::ptr::null::<WebPDecoderOptions>(),
        scaler_y: ::core::ptr::null_mut::<WebPRescaler>(),
        scaler_u: ::core::ptr::null_mut::<WebPRescaler>(),
        scaler_v: ::core::ptr::null_mut::<WebPRescaler>(),
        scaler_a: ::core::ptr::null_mut::<WebPRescaler>(),
        memory: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        emit: None,
        emit_alpha: None,
        emit_alpha_row: None,
    };
    let mut status: VP8StatusCode = VP8_STATUS_OK;
    if config.is_null() {
        return VP8_STATUS_INVALID_PARAM;
    }
    status = GetFeatures(data, data_size, &raw mut (*config).input);
    if status as ::core::ffi::c_uint != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        if status as ::core::ffi::c_uint
            == VP8_STATUS_NOT_ENOUGH_DATA as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return VP8_STATUS_BITSTREAM_ERROR;
        }
        return status;
    }
    WebPResetDecParams(&raw mut params);
    params.options = &raw mut (*config).options;
    params.output = &raw mut (*config).output;
    if WebPAvoidSlowMemory(params.output, &raw mut (*config).input) != 0 {
        let mut in_mem_buffer: WebPDecBuffer = WebPDecBuffer {
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
        };
        WebPInitDecBuffer(&raw mut in_mem_buffer);
        in_mem_buffer.colorspace = (*config).output.colorspace;
        in_mem_buffer.width = (*config).input.width;
        in_mem_buffer.height = (*config).input.height;
        params.output = &raw mut in_mem_buffer;
        status = DecodeInto(data, data_size, &raw mut params);
        if status as ::core::ffi::c_uint
            == VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            status = WebPCopyDecBufferPixels(&raw mut in_mem_buffer, &raw mut (*config).output);
        }
        WebPFreeDecBuffer(&raw mut in_mem_buffer);
    } else {
        status = DecodeInto(data, data_size, &raw mut params);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn WebPCheckCropDimensions(
    mut image_width: ::core::ffi::c_int,
    mut image_height: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return !(x < 0 as ::core::ffi::c_int
        || y < 0 as ::core::ffi::c_int
        || w <= 0 as ::core::ffi::c_int
        || h <= 0 as ::core::ffi::c_int
        || x >= image_width
        || w > image_width
        || w > image_width - x
        || y >= image_height
        || h > image_height
        || h > image_height - y) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPIoInitFromOptions(
    options: *const WebPDecoderOptions,
    io: *mut VP8Io,
    mut src_colorspace: WEBP_CSP_MODE,
) -> ::core::ffi::c_int {
    let W: ::core::ffi::c_int = (*io).width;
    let H: ::core::ffi::c_int = (*io).height;
    let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut w: ::core::ffi::c_int = W;
    let mut h: ::core::ffi::c_int = H;
    (*io).use_cropping = (!options.is_null() && (*options).use_cropping != 0) as ::core::ffi::c_int;
    if (*io).use_cropping != 0 {
        w = (*options).crop_width;
        h = (*options).crop_height;
        x = (*options).crop_left;
        y = (*options).crop_top;
        if WebPIsRGBMode(src_colorspace) == 0 {
            x &= !(1 as ::core::ffi::c_int);
            y &= !(1 as ::core::ffi::c_int);
        }
        if WebPCheckCropDimensions(W, H, x, y, w, h) == 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    (*io).crop_left = x;
    (*io).crop_top = y;
    (*io).crop_right = x + w;
    (*io).crop_bottom = y + h;
    (*io).mb_w = w;
    (*io).mb_h = h;
    (*io).use_scaling = (!options.is_null() && (*options).use_scaling != 0) as ::core::ffi::c_int;
    if (*io).use_scaling != 0 {
        let mut scaled_width: ::core::ffi::c_int = (*options).scaled_width;
        let mut scaled_height: ::core::ffi::c_int = (*options).scaled_height;
        if WebPRescalerGetScaledDimensions(w, h, &raw mut scaled_width, &raw mut scaled_height) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        (*io).scaled_width = scaled_width;
        (*io).scaled_height = scaled_height;
    }
    (*io).bypass_filtering =
        (!options.is_null() && (*options).bypass_filtering != 0) as ::core::ffi::c_int;
    (*io).fancy_upsampling =
        (options.is_null() || (*options).no_fancy_upsampling == 0) as ::core::ffi::c_int;
    if (*io).use_scaling != 0 {
        (*io).bypass_filtering |= ((*io).scaled_width
            < W * 3 as ::core::ffi::c_int / 4 as ::core::ffi::c_int
            && (*io).scaled_height < H * 3 as ::core::ffi::c_int / 4 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        (*io).fancy_upsampling = 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
