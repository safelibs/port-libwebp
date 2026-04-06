extern "C" {
    static mut VP8TransformWHT: VP8WHT;
    fn WebPDeallocateAlphaMemory(dec: *mut VP8Decoder);
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
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn VP8InitBitReader(br: *mut VP8BitReader, start: *const uint8_t, size: size_t);
    fn VP8GetValue(br: *mut VP8BitReader, num_bits: ::core::ffi::c_int) -> uint32_t;
    fn VP8GetSignedValue(br: *mut VP8BitReader, num_bits: ::core::ffi::c_int) -> int32_t;
    fn WebPGetWorkerInterface() -> *const WebPWorkerInterface;
    fn VP8ResetProba(proba: *mut VP8Proba);
    fn VP8ParseProba(br: *mut VP8BitReader, dec: *mut VP8Decoder);
    fn VP8ParseIntraModeRow(br: *mut VP8BitReader, dec: *mut VP8Decoder) -> ::core::ffi::c_int;
    fn VP8ParseQuant(dec: *mut VP8Decoder);
    fn VP8InitFrame(dec: *mut VP8Decoder, io: *mut VP8Io) -> ::core::ffi::c_int;
    fn VP8EnterCritical(dec: *mut VP8Decoder, io: *mut VP8Io) -> VP8StatusCode;
    fn VP8ExitCritical(dec: *mut VP8Decoder, io: *mut VP8Io) -> ::core::ffi::c_int;
    fn VP8ProcessRow(dec: *mut VP8Decoder, io: *mut VP8Io) -> ::core::ffi::c_int;
    fn WebPSafeCalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
    static kVP8Log2Range: [uint8_t; 128];
    static kVP8NewRange: [uint8_t; 128];
    fn VP8LoadFinalBytes(br: *mut VP8BitReader);
    static mut VP8GetCPUInfo: VP8CPUInfo;
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
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
pub type VP8CPUInfo = Option<unsafe extern "C" fn(CPUFeature) -> ::core::ffi::c_int>;
pub type CPUFeature = ::core::ffi::c_uint;
pub const kMSA: CPUFeature = 9;
pub const kMIPSdspR2: CPUFeature = 8;
pub const kMIPS32: CPUFeature = 7;
pub const kNEON: CPUFeature = 6;
pub const kAVX2: CPUFeature = 5;
pub const kAVX: CPUFeature = 4;
pub const kSSE4_1: CPUFeature = 3;
pub const kSlowSSSE3: CPUFeature = 2;
pub const kSSE3: CPUFeature = 1;
pub const kSSE2: CPUFeature = 0;
pub type GetCoeffsFunc = Option<
    unsafe extern "C" fn(
        *mut VP8BitReader,
        *const *const VP8BandProbas,
        ::core::ffi::c_int,
        *const ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut int16_t,
    ) -> ::core::ffi::c_int,
>;
pub type lbit_t = uint64_t;
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
pub const NUM_MODE_LF_DELTAS: C2RustUnnamed_1 = 4;
pub const NUM_REF_LF_DELTAS: C2RustUnnamed_1 = 4;
pub const MB_FEATURE_TREE_PROBS: C2RustUnnamed_1 = 3;
pub const NUM_MB_SEGMENTS: C2RustUnnamed_1 = 4;
pub const B_DC_PRED: C2RustUnnamed_0 = 0;
pub type VP8WHT = Option<unsafe extern "C" fn(*const int16_t, *mut int16_t) -> ()>;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const NUM_B_DC_MODES: C2RustUnnamed_0 = 7;
pub const B_DC_PRED_NOTOPLEFT: C2RustUnnamed_0 = 6;
pub const B_DC_PRED_NOLEFT: C2RustUnnamed_0 = 5;
pub const B_DC_PRED_NOTOP: C2RustUnnamed_0 = 4;
pub const NUM_PRED_MODES: C2RustUnnamed_0 = 4;
pub const B_PRED: C2RustUnnamed_0 = 10;
pub const TM_PRED: C2RustUnnamed_0 = 1;
pub const H_PRED: C2RustUnnamed_0 = 3;
pub const V_PRED: C2RustUnnamed_0 = 2;
pub const DC_PRED: C2RustUnnamed_0 = 0;
pub const NUM_BMODES: C2RustUnnamed_0 = 10;
pub const B_HU_PRED: C2RustUnnamed_0 = 9;
pub const B_HD_PRED: C2RustUnnamed_0 = 8;
pub const B_VL_PRED: C2RustUnnamed_0 = 7;
pub const B_LD_PRED: C2RustUnnamed_0 = 6;
pub const B_VR_PRED: C2RustUnnamed_0 = 5;
pub const B_RD_PRED: C2RustUnnamed_0 = 4;
pub const B_HE_PRED: C2RustUnnamed_0 = 3;
pub const B_VE_PRED: C2RustUnnamed_0 = 2;
pub const B_TM_PRED: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const NUM_PROBAS: C2RustUnnamed_1 = 11;
pub const NUM_CTX: C2RustUnnamed_1 = 3;
pub const NUM_BANDS: C2RustUnnamed_1 = 8;
pub const NUM_TYPES: C2RustUnnamed_1 = 4;
pub const MAX_NUM_PARTITIONS: C2RustUnnamed_1 = 8;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BITS: ::core::ffi::c_int = 56 as ::core::ffi::c_int;
pub const VP8_FRAME_HEADER_SIZE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const DEC_MAJ_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DEC_MIN_VERSION: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const DEC_REV_VERSION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn BSwap64(mut x: uint64_t) -> uint64_t {
    return x.swap_bytes();
}
#[inline]
unsafe extern "C" fn BitsLog2Floor(mut n: uint32_t) -> ::core::ffi::c_int {
    return 31 as ::core::ffi::c_int ^ n.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn VP8LoadNewBytes(br: *mut VP8BitReader) {
    '_c2rust_label: {
        if !br.is_null() && !(*br).buf_.is_null() {
        } else {
            __assert_fail(
                b"br != NULL && br->buf_ != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/utils/bit_reader_inl_utils.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                59 as ::core::ffi::c_uint,
                b"void VP8LoadNewBytes(VP8BitReader *const restrict)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*br).buf_ < (*br).buf_max_ {
        let mut bits: bit_t = 0;
        let mut in_bits: lbit_t = 0;
        memcpy(
            &raw mut in_bits as *mut ::core::ffi::c_void,
            (*br).buf_ as *const ::core::ffi::c_void,
            ::core::mem::size_of::<lbit_t>() as size_t,
        );
        (*br).buf_ = (*br)
            .buf_
            .offset((BITS >> 3 as ::core::ffi::c_int) as isize);
        bits = BSwap64(in_bits as uint64_t) as bit_t;
        bits >>= 64 as ::core::ffi::c_int - BITS;
        (*br).value_ = bits | (*br).value_ << BITS;
        (*br).bits_ += BITS;
    } else {
        VP8LoadFinalBytes(br);
    };
}
#[inline]
unsafe extern "C" fn VP8GetBit(
    br: *mut VP8BitReader,
    mut prob: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut range: range_t = (*br).range_;
    if (*br).bits_ < 0 as ::core::ffi::c_int {
        VP8LoadNewBytes(br);
    }
    let pos: ::core::ffi::c_int = (*br).bits_;
    let split: range_t = range.wrapping_mul(prob as range_t) >> 8 as ::core::ffi::c_int;
    let value: range_t = ((*br).value_ >> pos) as range_t;
    let bit: ::core::ffi::c_int = (value > split) as ::core::ffi::c_int;
    if bit != 0 {
        range = range.wrapping_sub(split);
        (*br).value_ = (*br)
            .value_
            .wrapping_sub((split.wrapping_add(1 as range_t) as bit_t) << pos);
    } else {
        range = split.wrapping_add(1 as range_t);
    }
    let shift: ::core::ffi::c_int =
        7 as ::core::ffi::c_int ^ BitsLog2Floor(range as uint32_t) as ::core::ffi::c_int;
    range <<= shift;
    (*br).bits_ -= shift;
    (*br).range_ = range.wrapping_sub(1 as range_t);
    return bit;
}
#[inline]
unsafe extern "C" fn VP8GetSigned(
    br: *mut VP8BitReader,
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*br).bits_ < 0 as ::core::ffi::c_int {
        VP8LoadNewBytes(br);
    }
    let pos: ::core::ffi::c_int = (*br).bits_;
    let split: range_t = (*br).range_ >> 1 as ::core::ffi::c_int;
    let value: range_t = ((*br).value_ >> pos) as range_t;
    let mask: int32_t = split.wrapping_sub(value) as int32_t >> 31 as ::core::ffi::c_int;
    (*br).bits_ -= 1 as ::core::ffi::c_int;
    (*br).range_ = (*br).range_.wrapping_add(mask as range_t);
    (*br).range_ |= 1 as range_t;
    (*br).value_ = (*br)
        .value_
        .wrapping_sub(((split.wrapping_add(1 as range_t) & mask as range_t) as bit_t) << pos);
    return (v ^ mask as ::core::ffi::c_int) - mask as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn VP8GetBitAlt(
    br: *mut VP8BitReader,
    mut prob: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut range: range_t = (*br).range_;
    if (*br).bits_ < 0 as ::core::ffi::c_int {
        VP8LoadNewBytes(br);
    }
    let pos: ::core::ffi::c_int = (*br).bits_;
    let split: range_t = range.wrapping_mul(prob as range_t) >> 8 as ::core::ffi::c_int;
    let value: range_t = ((*br).value_ >> pos) as range_t;
    let mut bit: ::core::ffi::c_int = 0;
    if value > split {
        range = range.wrapping_sub(split.wrapping_add(1 as range_t));
        (*br).value_ = (*br)
            .value_
            .wrapping_sub((split.wrapping_add(1 as range_t) as bit_t) << pos);
        bit = 1 as ::core::ffi::c_int;
    } else {
        range = split;
        bit = 0 as ::core::ffi::c_int;
    }
    if range <= 0x7e as ::core::ffi::c_int as range_t {
        let shift: ::core::ffi::c_int = kVP8Log2Range[range as usize] as ::core::ffi::c_int;
        range = kVP8NewRange[range as usize] as range_t;
        (*br).bits_ -= shift;
    }
    (*br).range_ = range;
    return bit;
}
#[no_mangle]
pub unsafe extern "C" fn WebPGetDecoderVersion() -> ::core::ffi::c_int {
    return DEC_MAJ_VERSION << 16 as ::core::ffi::c_int
        | DEC_MIN_VERSION << 8 as ::core::ffi::c_int
        | DEC_REV_VERSION;
}
static mut GetCoeffs: GetCoeffsFunc = None;
unsafe extern "C" fn SetOk(dec: *mut VP8Decoder) {
    (*dec).status_ = VP8_STATUS_OK;
    (*dec).error_msg_ = b"OK\0" as *const u8 as *const ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn VP8InitIoInternal(
    io: *mut VP8Io,
    mut version: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if version >> 8 as ::core::ffi::c_int != 0x209 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if !io.is_null() {
        memset(
            io as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<VP8Io>() as size_t,
        );
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8New() -> *mut VP8Decoder {
    let dec: *mut VP8Decoder = WebPSafeCalloc(
        1 as uint64_t,
        ::core::mem::size_of::<VP8Decoder>() as size_t,
    ) as *mut VP8Decoder;
    if !dec.is_null() {
        SetOk(dec);
        (*WebPGetWorkerInterface())
            .Init
            .expect("non-null function pointer")(&raw mut (*dec).worker_);
        (*dec).ready_ = 0 as ::core::ffi::c_int;
        (*dec).num_parts_minus_one_ = 0 as uint32_t;
        InitGetCoeffs();
    }
    return dec;
}
#[no_mangle]
pub unsafe extern "C" fn VP8Status(dec: *mut VP8Decoder) -> VP8StatusCode {
    if dec.is_null() {
        return VP8_STATUS_INVALID_PARAM;
    }
    return (*dec).status_;
}
#[no_mangle]
pub unsafe extern "C" fn VP8StatusMessage(dec: *mut VP8Decoder) -> *const ::core::ffi::c_char {
    if dec.is_null() {
        return b"no object\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if (*dec).error_msg_.is_null() {
        return b"OK\0" as *const u8 as *const ::core::ffi::c_char;
    }
    return (*dec).error_msg_;
}
#[no_mangle]
pub unsafe extern "C" fn VP8Delete(dec: *mut VP8Decoder) {
    if !dec.is_null() {
        VP8Clear(dec);
        WebPSafeFree(dec as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8SetError(
    dec: *mut VP8Decoder,
    mut error: VP8StatusCode,
    msg: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if (*dec).status_ as ::core::ffi::c_uint
        == VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*dec).status_ = error;
        (*dec).error_msg_ = msg;
        (*dec).ready_ = 0 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8CheckSignature(
    data: *const uint8_t,
    mut data_size: size_t,
) -> ::core::ffi::c_int {
    return (data_size >= 3 as size_t
        && *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x9d as ::core::ffi::c_int
        && *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x1 as ::core::ffi::c_int
        && *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x2a as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8GetInfo(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut chunk_size: size_t,
    width: *mut ::core::ffi::c_int,
    height: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if data.is_null() || data_size < VP8_FRAME_HEADER_SIZE as size_t {
        return 0 as ::core::ffi::c_int;
    }
    if VP8CheckSignature(
        data.offset(3 as ::core::ffi::c_int as isize),
        data_size.wrapping_sub(3 as size_t),
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    } else {
        let bits: uint32_t = (*data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            | (*data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
            | (*data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as uint32_t;
        let key_frame: ::core::ffi::c_int = (bits & 1 as uint32_t == 0) as ::core::ffi::c_int;
        let w: ::core::ffi::c_int = ((*data.offset(7 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *data.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            & 0x3fff as ::core::ffi::c_int;
        let h: ::core::ffi::c_int = ((*data.offset(9 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *data.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            & 0x3fff as ::core::ffi::c_int;
        if key_frame == 0 {
            return 0 as ::core::ffi::c_int;
        }
        if bits >> 1 as ::core::ffi::c_int & 7 as uint32_t > 3 as uint32_t {
            return 0 as ::core::ffi::c_int;
        }
        if bits >> 4 as ::core::ffi::c_int & 1 as uint32_t == 0 {
            return 0 as ::core::ffi::c_int;
        }
        if (bits >> 5 as ::core::ffi::c_int) as size_t >= chunk_size {
            return 0 as ::core::ffi::c_int;
        }
        if w == 0 as ::core::ffi::c_int || h == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if !width.is_null() {
            *width = w;
        }
        if !height.is_null() {
            *height = h;
        }
        return 1 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn ResetSegmentHeader(hdr: *mut VP8SegmentHeader) {
    '_c2rust_label: {
        if !hdr.is_null() {
        } else {
            __assert_fail(
                b"hdr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8_dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                151 as ::core::ffi::c_uint,
                b"void ResetSegmentHeader(VP8SegmentHeader *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*hdr).use_segment_ = 0 as ::core::ffi::c_int;
    (*hdr).update_map_ = 0 as ::core::ffi::c_int;
    (*hdr).absolute_delta_ = 1 as ::core::ffi::c_int;
    memset(
        &raw mut (*hdr).quantizer_ as *mut int8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[int8_t; 4]>() as size_t,
    );
    memset(
        &raw mut (*hdr).filter_strength_ as *mut int8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[int8_t; 4]>() as size_t,
    );
}
unsafe extern "C" fn ParseSegmentHeader(
    mut br: *mut VP8BitReader,
    mut hdr: *mut VP8SegmentHeader,
    mut proba: *mut VP8Proba,
) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if !br.is_null() {
        } else {
            __assert_fail(
                b"br != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8_dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                162 as ::core::ffi::c_uint,
                b"int ParseSegmentHeader(VP8BitReader *, VP8SegmentHeader *, VP8Proba *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !hdr.is_null() {
        } else {
            __assert_fail(
                b"hdr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8_dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                163 as ::core::ffi::c_uint,
                b"int ParseSegmentHeader(VP8BitReader *, VP8SegmentHeader *, VP8Proba *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    (*hdr).use_segment_ = VP8GetValue(br, 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if (*hdr).use_segment_ != 0 {
        (*hdr).update_map_ = VP8GetValue(br, 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if VP8GetValue(br, 1 as ::core::ffi::c_int) != 0 {
            let mut s: ::core::ffi::c_int = 0;
            (*hdr).absolute_delta_ = VP8GetValue(br, 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
            s = 0 as ::core::ffi::c_int;
            while s < NUM_MB_SEGMENTS as ::core::ffi::c_int {
                (*hdr).quantizer_[s as usize] = (if VP8GetValue(br, 1 as ::core::ffi::c_int) != 0 {
                    VP8GetSignedValue(br, 7 as ::core::ffi::c_int)
                } else {
                    0 as int32_t
                }) as int8_t;
                s += 1;
            }
            s = 0 as ::core::ffi::c_int;
            while s < NUM_MB_SEGMENTS as ::core::ffi::c_int {
                (*hdr).filter_strength_[s as usize] =
                    (if VP8GetValue(br, 1 as ::core::ffi::c_int) != 0 {
                        VP8GetSignedValue(br, 6 as ::core::ffi::c_int)
                    } else {
                        0 as int32_t
                    }) as int8_t;
                s += 1;
            }
        }
        if (*hdr).update_map_ != 0 {
            let mut s_0: ::core::ffi::c_int = 0;
            s_0 = 0 as ::core::ffi::c_int;
            while s_0 < MB_FEATURE_TREE_PROBS as ::core::ffi::c_int {
                (*proba).segments_[s_0 as usize] = (if VP8GetValue(br, 1 as ::core::ffi::c_int) != 0
                {
                    VP8GetValue(br, 8 as ::core::ffi::c_int)
                } else {
                    255 as uint32_t
                }) as uint8_t;
                s_0 += 1;
            }
        }
    } else {
        (*hdr).update_map_ = 0 as ::core::ffi::c_int;
    }
    return ((*br).eof_ == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn ParsePartitions(
    dec: *mut VP8Decoder,
    mut buf: *const uint8_t,
    mut size: size_t,
) -> VP8StatusCode {
    let br: *mut VP8BitReader = &raw mut (*dec).br_;
    let mut sz: *const uint8_t = buf;
    let mut buf_end: *const uint8_t = buf.offset(size as isize);
    let mut part_start: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut size_left: size_t = size;
    let mut last_part: size_t = 0;
    let mut p: size_t = 0;
    (*dec).num_parts_minus_one_ = (((1 as ::core::ffi::c_int)
        << VP8GetValue(br, 2 as ::core::ffi::c_int))
        - 1 as ::core::ffi::c_int) as uint32_t;
    last_part = (*dec).num_parts_minus_one_ as size_t;
    if size < (3 as size_t).wrapping_mul(last_part) {
        return VP8_STATUS_NOT_ENOUGH_DATA;
    }
    part_start = buf.offset(last_part.wrapping_mul(3 as size_t) as isize);
    size_left = size_left.wrapping_sub(last_part.wrapping_mul(3 as size_t));
    p = 0 as size_t;
    while p < last_part {
        let mut psize: size_t = (*sz.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            | (*sz.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
            | (*sz.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as size_t;
        if psize > size_left {
            psize = size_left;
        }
        VP8InitBitReader(
            (&raw mut (*dec).parts_ as *mut VP8BitReader).offset(p as isize),
            part_start,
            psize,
        );
        part_start = part_start.offset(psize as isize);
        size_left = size_left.wrapping_sub(psize);
        sz = sz.offset(3 as ::core::ffi::c_int as isize);
        p = p.wrapping_add(1);
    }
    VP8InitBitReader(
        (&raw mut (*dec).parts_ as *mut VP8BitReader).offset(last_part as isize),
        part_start,
        size_left,
    );
    return (if part_start < buf_end {
        VP8_STATUS_OK as ::core::ffi::c_int
    } else {
        VP8_STATUS_SUSPENDED as ::core::ffi::c_int
    }) as VP8StatusCode;
}
unsafe extern "C" fn ParseFilterHeader(
    mut br: *mut VP8BitReader,
    dec: *mut VP8Decoder,
) -> ::core::ffi::c_int {
    let hdr: *mut VP8FilterHeader = &raw mut (*dec).filter_hdr_;
    (*hdr).simple_ = VP8GetValue(br, 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*hdr).level_ = VP8GetValue(br, 6 as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*hdr).sharpness_ = VP8GetValue(br, 3 as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*hdr).use_lf_delta_ = VP8GetValue(br, 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if (*hdr).use_lf_delta_ != 0 {
        if VP8GetValue(br, 1 as ::core::ffi::c_int) != 0 {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < NUM_REF_LF_DELTAS as ::core::ffi::c_int {
                if VP8GetValue(br, 1 as ::core::ffi::c_int) != 0 {
                    (*hdr).ref_lf_delta_[i as usize] =
                        VP8GetSignedValue(br, 6 as ::core::ffi::c_int) as ::core::ffi::c_int;
                }
                i += 1;
            }
            i = 0 as ::core::ffi::c_int;
            while i < NUM_MODE_LF_DELTAS as ::core::ffi::c_int {
                if VP8GetValue(br, 1 as ::core::ffi::c_int) != 0 {
                    (*hdr).mode_lf_delta_[i as usize] =
                        VP8GetSignedValue(br, 6 as ::core::ffi::c_int) as ::core::ffi::c_int;
                }
                i += 1;
            }
        }
    }
    (*dec).filter_type_ = if (*hdr).level_ == 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else if (*hdr).simple_ != 0 {
        1 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    };
    return ((*br).eof_ == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8GetHeaders(dec: *mut VP8Decoder, io: *mut VP8Io) -> ::core::ffi::c_int {
    let mut buf: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut buf_size: size_t = 0;
    let mut frm_hdr: *mut VP8FrameHeader = ::core::ptr::null_mut::<VP8FrameHeader>();
    let mut pic_hdr: *mut VP8PictureHeader = ::core::ptr::null_mut::<VP8PictureHeader>();
    let mut br: *mut VP8BitReader = ::core::ptr::null_mut::<VP8BitReader>();
    let mut status: VP8StatusCode = VP8_STATUS_OK;
    if dec.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    SetOk(dec);
    if io.is_null() {
        return VP8SetError(
            dec,
            VP8_STATUS_INVALID_PARAM,
            b"null VP8Io passed to VP8GetHeaders()\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    buf = (*io).data;
    buf_size = (*io).data_size;
    if buf_size < 4 as size_t {
        return VP8SetError(
            dec,
            VP8_STATUS_NOT_ENOUGH_DATA,
            b"Truncated header.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let bits: uint32_t = (*buf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        | (*buf.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
        | (*buf.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int) as uint32_t;
    frm_hdr = &raw mut (*dec).frm_hdr_;
    (*frm_hdr).key_frame_ = (bits & 1 as uint32_t == 0) as ::core::ffi::c_int as uint8_t;
    (*frm_hdr).profile_ = (bits >> 1 as ::core::ffi::c_int & 7 as uint32_t) as uint8_t;
    (*frm_hdr).show_ = (bits >> 4 as ::core::ffi::c_int & 1 as uint32_t) as uint8_t;
    (*frm_hdr).partition_length_ = bits >> 5 as ::core::ffi::c_int;
    if (*frm_hdr).profile_ as ::core::ffi::c_int > 3 as ::core::ffi::c_int {
        return VP8SetError(
            dec,
            VP8_STATUS_BITSTREAM_ERROR,
            b"Incorrect keyframe parameters.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*frm_hdr).show_ == 0 {
        return VP8SetError(
            dec,
            VP8_STATUS_UNSUPPORTED_FEATURE,
            b"Frame not displayable.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    buf = buf.offset(3 as ::core::ffi::c_int as isize);
    buf_size = buf_size.wrapping_sub(3 as size_t);
    pic_hdr = &raw mut (*dec).pic_hdr_;
    if (*frm_hdr).key_frame_ != 0 {
        if buf_size < 7 as size_t {
            return VP8SetError(
                dec,
                VP8_STATUS_NOT_ENOUGH_DATA,
                b"cannot parse picture header\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if VP8CheckSignature(buf, buf_size) == 0 {
            return VP8SetError(
                dec,
                VP8_STATUS_BITSTREAM_ERROR,
                b"Bad code word\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*pic_hdr).width_ = (((*buf.offset(4 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *buf.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            & 0x3fff as ::core::ffi::c_int) as uint16_t;
        (*pic_hdr).xscale_ = (*buf.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >> 6 as ::core::ffi::c_int) as uint8_t;
        (*pic_hdr).height_ = (((*buf.offset(6 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *buf.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            & 0x3fff as ::core::ffi::c_int) as uint16_t;
        (*pic_hdr).yscale_ = (*buf.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >> 6 as ::core::ffi::c_int) as uint8_t;
        buf = buf.offset(7 as ::core::ffi::c_int as isize);
        buf_size = buf_size.wrapping_sub(7 as size_t);
        (*dec).mb_w_ = (*pic_hdr).width_ as ::core::ffi::c_int + 15 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int;
        (*dec).mb_h_ = (*pic_hdr).height_ as ::core::ffi::c_int + 15 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int;
        (*io).width = (*pic_hdr).width_ as ::core::ffi::c_int;
        (*io).height = (*pic_hdr).height_ as ::core::ffi::c_int;
        (*io).use_cropping = 0 as ::core::ffi::c_int;
        (*io).crop_top = 0 as ::core::ffi::c_int;
        (*io).crop_left = 0 as ::core::ffi::c_int;
        (*io).crop_right = (*io).width;
        (*io).crop_bottom = (*io).height;
        (*io).use_scaling = 0 as ::core::ffi::c_int;
        (*io).scaled_width = (*io).width;
        (*io).scaled_height = (*io).height;
        (*io).mb_w = (*io).width;
        (*io).mb_h = (*io).height;
        VP8ResetProba(&raw mut (*dec).proba_);
        ResetSegmentHeader(&raw mut (*dec).segment_hdr_);
    }
    if (*frm_hdr).partition_length_ as size_t > buf_size {
        return VP8SetError(
            dec,
            VP8_STATUS_NOT_ENOUGH_DATA,
            b"bad partition length\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    br = &raw mut (*dec).br_;
    VP8InitBitReader(br, buf, (*frm_hdr).partition_length_ as size_t);
    buf = buf.offset((*frm_hdr).partition_length_ as isize);
    buf_size = buf_size.wrapping_sub((*frm_hdr).partition_length_ as size_t);
    if (*frm_hdr).key_frame_ != 0 {
        (*pic_hdr).colorspace_ = VP8GetValue(br, 1 as ::core::ffi::c_int) as uint8_t;
        (*pic_hdr).clamp_type_ = VP8GetValue(br, 1 as ::core::ffi::c_int) as uint8_t;
    }
    if ParseSegmentHeader(br, &raw mut (*dec).segment_hdr_, &raw mut (*dec).proba_) == 0 {
        return VP8SetError(
            dec,
            VP8_STATUS_BITSTREAM_ERROR,
            b"cannot parse segment header\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ParseFilterHeader(br, dec) == 0 {
        return VP8SetError(
            dec,
            VP8_STATUS_BITSTREAM_ERROR,
            b"cannot parse filter header\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    status = ParsePartitions(dec, buf, buf_size);
    if status as ::core::ffi::c_uint != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        return VP8SetError(
            dec,
            status,
            b"cannot parse partitions\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    VP8ParseQuant(dec);
    if (*frm_hdr).key_frame_ == 0 {
        return VP8SetError(
            dec,
            VP8_STATUS_UNSUPPORTED_FEATURE,
            b"Not a key frame.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    VP8GetValue(br, 1 as ::core::ffi::c_int);
    VP8ParseProba(br, dec);
    (*dec).ready_ = 1 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
static mut kCat3: [uint8_t; 4] = [
    173 as ::core::ffi::c_int as uint8_t,
    148 as ::core::ffi::c_int as uint8_t,
    140 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
static mut kCat4: [uint8_t; 5] = [
    176 as ::core::ffi::c_int as uint8_t,
    155 as ::core::ffi::c_int as uint8_t,
    140 as ::core::ffi::c_int as uint8_t,
    135 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
static mut kCat5: [uint8_t; 6] = [
    180 as ::core::ffi::c_int as uint8_t,
    157 as ::core::ffi::c_int as uint8_t,
    141 as ::core::ffi::c_int as uint8_t,
    134 as ::core::ffi::c_int as uint8_t,
    130 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
static mut kCat6: [uint8_t; 12] = [
    254 as ::core::ffi::c_int as uint8_t,
    254 as ::core::ffi::c_int as uint8_t,
    243 as ::core::ffi::c_int as uint8_t,
    230 as ::core::ffi::c_int as uint8_t,
    196 as ::core::ffi::c_int as uint8_t,
    177 as ::core::ffi::c_int as uint8_t,
    153 as ::core::ffi::c_int as uint8_t,
    140 as ::core::ffi::c_int as uint8_t,
    133 as ::core::ffi::c_int as uint8_t,
    130 as ::core::ffi::c_int as uint8_t,
    129 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
static mut kCat3456: [*const uint8_t; 4] = [
    &raw const kCat3 as *const uint8_t,
    &raw const kCat4 as *const uint8_t,
    &raw const kCat5 as *const uint8_t,
    &raw const kCat6 as *const uint8_t,
];
static mut kZigzag: [uint8_t; 16] = [
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    12 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    15 as ::core::ffi::c_int as uint8_t,
];
unsafe extern "C" fn GetLargeValue(br: *mut VP8BitReader, p: *const uint8_t) -> ::core::ffi::c_int {
    let mut v: ::core::ffi::c_int = 0;
    if VP8GetBit(
        br,
        *p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
    ) == 0
    {
        if VP8GetBit(
            br,
            *p.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        ) == 0
        {
            v = 2 as ::core::ffi::c_int;
        } else {
            v = 3 as ::core::ffi::c_int
                + VP8GetBit(
                    br,
                    *p.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                );
        }
    } else if VP8GetBit(
        br,
        *p.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
    ) == 0
    {
        if VP8GetBit(
            br,
            *p.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        ) == 0
        {
            v = 5 as ::core::ffi::c_int + VP8GetBit(br, 159 as ::core::ffi::c_int);
        } else {
            v = 7 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * VP8GetBit(br, 165 as ::core::ffi::c_int);
            v += VP8GetBit(br, 145 as ::core::ffi::c_int);
        }
    } else {
        let mut tab: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let bit1: ::core::ffi::c_int = VP8GetBit(
            br,
            *p.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        let bit0: ::core::ffi::c_int = VP8GetBit(
            br,
            *p.offset((9 as ::core::ffi::c_int + bit1) as isize) as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        let cat: ::core::ffi::c_int = 2 as ::core::ffi::c_int * bit1 + bit0;
        v = 0 as ::core::ffi::c_int;
        tab = kCat3456[cat as usize];
        while *tab != 0 {
            v += v + VP8GetBit(br, *tab as ::core::ffi::c_int);
            tab = tab.offset(1);
        }
        v += 3 as ::core::ffi::c_int + ((8 as ::core::ffi::c_int) << cat);
    }
    return v;
}
unsafe extern "C" fn GetCoeffsFast(
    br: *mut VP8BitReader,
    mut prob: *const *const VP8BandProbas,
    mut ctx: ::core::ffi::c_int,
    mut dq: *const ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut out: *mut int16_t,
) -> ::core::ffi::c_int {
    let mut p: *const uint8_t = &raw const *(&raw const (**prob.offset(n as isize)).probas_
        as *const VP8ProbaArray)
        .offset(ctx as isize) as *const uint8_t;
    while n < 16 as ::core::ffi::c_int {
        if VP8GetBit(
            br,
            *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        ) == 0
        {
            return n;
        }
        while VP8GetBit(
            br,
            *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        ) == 0
        {
            n += 1;
            p = &raw const *(&raw const (**prob.offset(n as isize)).probas_ as *const VP8ProbaArray)
                .offset(0 as ::core::ffi::c_int as isize) as *const uint8_t;
            if n == 16 as ::core::ffi::c_int {
                return 16 as ::core::ffi::c_int;
            }
        }
        let p_ctx: *const VP8ProbaArray =
            (&raw const (**prob.offset((n + 1 as ::core::ffi::c_int) as isize)).probas_
                as *const VP8ProbaArray)
                .offset(0 as ::core::ffi::c_int as isize) as *const VP8ProbaArray;
        let mut v: ::core::ffi::c_int = 0;
        if VP8GetBit(
            br,
            *p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        ) == 0
        {
            v = 1 as ::core::ffi::c_int;
            p = &raw const *p_ctx.offset(1 as ::core::ffi::c_int as isize) as *const uint8_t;
        } else {
            v = GetLargeValue(br, p);
            p = &raw const *p_ctx.offset(2 as ::core::ffi::c_int as isize) as *const uint8_t;
        }
        *out.offset(kZigzag[n as usize] as isize) = (VP8GetSigned(br, v)
            * *dq.offset((n > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as isize))
            as int16_t;
        n += 1;
    }
    return 16 as ::core::ffi::c_int;
}
unsafe extern "C" fn GetCoeffsAlt(
    br: *mut VP8BitReader,
    mut prob: *const *const VP8BandProbas,
    mut ctx: ::core::ffi::c_int,
    mut dq: *const ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut out: *mut int16_t,
) -> ::core::ffi::c_int {
    let mut p: *const uint8_t = &raw const *(&raw const (**prob.offset(n as isize)).probas_
        as *const VP8ProbaArray)
        .offset(ctx as isize) as *const uint8_t;
    while n < 16 as ::core::ffi::c_int {
        if VP8GetBitAlt(
            br,
            *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        ) == 0
        {
            return n;
        }
        while VP8GetBitAlt(
            br,
            *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        ) == 0
        {
            n += 1;
            p = &raw const *(&raw const (**prob.offset(n as isize)).probas_ as *const VP8ProbaArray)
                .offset(0 as ::core::ffi::c_int as isize) as *const uint8_t;
            if n == 16 as ::core::ffi::c_int {
                return 16 as ::core::ffi::c_int;
            }
        }
        let p_ctx: *const VP8ProbaArray =
            (&raw const (**prob.offset((n + 1 as ::core::ffi::c_int) as isize)).probas_
                as *const VP8ProbaArray)
                .offset(0 as ::core::ffi::c_int as isize) as *const VP8ProbaArray;
        let mut v: ::core::ffi::c_int = 0;
        if VP8GetBitAlt(
            br,
            *p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        ) == 0
        {
            v = 1 as ::core::ffi::c_int;
            p = &raw const *p_ctx.offset(1 as ::core::ffi::c_int as isize) as *const uint8_t;
        } else {
            v = GetLargeValue(br, p);
            p = &raw const *p_ctx.offset(2 as ::core::ffi::c_int as isize) as *const uint8_t;
        }
        *out.offset(kZigzag[n as usize] as isize) = (VP8GetSigned(br, v)
            * *dq.offset((n > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as isize))
            as int16_t;
        n += 1;
    }
    return 16 as ::core::ffi::c_int;
}
unsafe extern "C" fn InitGetCoeffs_body() {
    if VP8GetCPUInfo.is_some() && VP8GetCPUInfo.expect("non-null function pointer")(kSlowSSSE3) != 0
    {
        ::core::ptr::write_volatile(
            &mut GetCoeffs as *mut GetCoeffsFunc,
            Some(
                GetCoeffsAlt
                    as unsafe extern "C" fn(
                        *mut VP8BitReader,
                        *const *const VP8BandProbas,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut int16_t,
                    ) -> ::core::ffi::c_int,
            ) as GetCoeffsFunc,
        );
    } else {
        ::core::ptr::write_volatile(
            &mut GetCoeffs as *mut GetCoeffsFunc,
            Some(
                GetCoeffsFast
                    as unsafe extern "C" fn(
                        *mut VP8BitReader,
                        *const *const VP8BandProbas,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut int16_t,
                    ) -> ::core::ffi::c_int,
            ) as GetCoeffsFunc,
        );
    };
}
unsafe extern "C" fn InitGetCoeffs() {
    static mut InitGetCoeffs_body_last_cpuinfo_used: VP8CPUInfo = None;
    if !(InitGetCoeffs_body_last_cpuinfo_used == VP8GetCPUInfo) {
        InitGetCoeffs_body();
        ::core::ptr::write_volatile(
            &mut InitGetCoeffs_body_last_cpuinfo_used as *mut VP8CPUInfo,
            VP8GetCPUInfo,
        );
    }
}
#[inline]
unsafe extern "C" fn NzCodeBits(
    mut nz_coeffs: uint32_t,
    mut nz: ::core::ffi::c_int,
    mut dc_nz: ::core::ffi::c_int,
) -> uint32_t {
    nz_coeffs <<= 2 as ::core::ffi::c_int;
    nz_coeffs |= (if nz > 3 as ::core::ffi::c_int {
        3 as ::core::ffi::c_int
    } else if nz > 1 as ::core::ffi::c_int {
        2 as ::core::ffi::c_int
    } else {
        dc_nz
    }) as uint32_t;
    return nz_coeffs;
}
unsafe extern "C" fn ParseResiduals(
    dec: *mut VP8Decoder,
    mb: *mut VP8MB,
    token_br: *mut VP8BitReader,
) -> ::core::ffi::c_int {
    let bands: *mut [*const VP8BandProbas; 17] =
        &raw mut (*dec).proba_.bands_ptr_ as *mut [*const VP8BandProbas; 17];
    let mut ac_proba: *const *const VP8BandProbas = ::core::ptr::null::<*const VP8BandProbas>();
    let block: *mut VP8MBData = (*dec).mb_data_.offset((*dec).mb_x_ as isize);
    let q: *const VP8QuantMatrix = (&raw mut (*dec).dqm_ as *mut VP8QuantMatrix)
        .offset((*block).segment_ as isize)
        as *mut VP8QuantMatrix;
    let mut dst: *mut int16_t = &raw mut (*block).coeffs_ as *mut int16_t;
    let left_mb: *mut VP8MB = (*dec).mb_info_.offset(-(1 as ::core::ffi::c_int as isize));
    let mut tnz: uint8_t = 0;
    let mut lnz: uint8_t = 0;
    let mut non_zero_y: uint32_t = 0 as uint32_t;
    let mut non_zero_uv: uint32_t = 0 as uint32_t;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut ch: ::core::ffi::c_int = 0;
    let mut out_t_nz: uint32_t = 0;
    let mut out_l_nz: uint32_t = 0;
    let mut first: ::core::ffi::c_int = 0;
    memset(
        dst as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (384 as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
    );
    if (*block).is_i4x4_ == 0 {
        let mut dc: [int16_t; 16] = [
            0 as ::core::ffi::c_int as int16_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let ctx: ::core::ffi::c_int =
            (*mb).nz_dc_ as ::core::ffi::c_int + (*left_mb).nz_dc_ as ::core::ffi::c_int;
        let nz: ::core::ffi::c_int = GetCoeffs.expect("non-null function pointer")(
            token_br,
            &raw mut *bands.offset(1 as ::core::ffi::c_int as isize) as *mut *const VP8BandProbas
                as *const *const VP8BandProbas,
            ctx,
            &raw const (*q).y2_mat_ as *const ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw mut dc as *mut int16_t,
        ) as ::core::ffi::c_int;
        (*left_mb).nz_dc_ = (nz > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as uint8_t;
        (*mb).nz_dc_ = (*left_mb).nz_dc_;
        if nz > 1 as ::core::ffi::c_int {
            VP8TransformWHT.expect("non-null function pointer")(&raw mut dc as *mut int16_t, dst);
        } else {
            let mut i: ::core::ffi::c_int = 0;
            let dc0: ::core::ffi::c_int = dc[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int
                >> 3 as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int {
                *dst.offset(i as isize) = dc0 as int16_t;
                i += 16 as ::core::ffi::c_int;
            }
        }
        first = 1 as ::core::ffi::c_int;
        ac_proba =
            &raw mut *bands.offset(0 as ::core::ffi::c_int as isize) as *mut *const VP8BandProbas;
    } else {
        first = 0 as ::core::ffi::c_int;
        ac_proba =
            &raw mut *bands.offset(3 as ::core::ffi::c_int as isize) as *mut *const VP8BandProbas;
    }
    tnz = ((*mb).nz_ as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as uint8_t;
    lnz = ((*left_mb).nz_ as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as uint8_t;
    y = 0 as ::core::ffi::c_int;
    while y < 4 as ::core::ffi::c_int {
        let mut l: ::core::ffi::c_int = lnz as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
        let mut nz_coeffs: uint32_t = 0 as uint32_t;
        x = 0 as ::core::ffi::c_int;
        while x < 4 as ::core::ffi::c_int {
            let ctx_0: ::core::ffi::c_int =
                l + (tnz as ::core::ffi::c_int & 1 as ::core::ffi::c_int);
            let nz_0: ::core::ffi::c_int = GetCoeffs.expect("non-null function pointer")(
                token_br,
                ac_proba as *const *const VP8BandProbas,
                ctx_0,
                &raw const (*q).y1_mat_ as *const ::core::ffi::c_int,
                first,
                dst,
            ) as ::core::ffi::c_int;
            l = (nz_0 > first) as ::core::ffi::c_int;
            tnz = (tnz as ::core::ffi::c_int >> 1 as ::core::ffi::c_int
                | l << 7 as ::core::ffi::c_int) as uint8_t;
            nz_coeffs = NzCodeBits(
                nz_coeffs,
                nz_0,
                (*dst.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            );
            dst = dst.offset(16 as ::core::ffi::c_int as isize);
            x += 1;
        }
        tnz = (tnz as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as uint8_t;
        lnz = (lnz as ::core::ffi::c_int >> 1 as ::core::ffi::c_int | l << 7 as ::core::ffi::c_int)
            as uint8_t;
        non_zero_y = non_zero_y << 8 as ::core::ffi::c_int | nz_coeffs;
        y += 1;
    }
    out_t_nz = tnz as uint32_t;
    out_l_nz = (lnz as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as uint32_t;
    ch = 0 as ::core::ffi::c_int;
    while ch < 4 as ::core::ffi::c_int {
        let mut nz_coeffs_0: uint32_t = 0 as uint32_t;
        tnz = ((*mb).nz_ as ::core::ffi::c_int >> 4 as ::core::ffi::c_int + ch) as uint8_t;
        lnz = ((*left_mb).nz_ as ::core::ffi::c_int >> 4 as ::core::ffi::c_int + ch) as uint8_t;
        y = 0 as ::core::ffi::c_int;
        while y < 2 as ::core::ffi::c_int {
            let mut l_0: ::core::ffi::c_int = lnz as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
            x = 0 as ::core::ffi::c_int;
            while x < 2 as ::core::ffi::c_int {
                let ctx_1: ::core::ffi::c_int =
                    l_0 + (tnz as ::core::ffi::c_int & 1 as ::core::ffi::c_int);
                let nz_1: ::core::ffi::c_int = GetCoeffs.expect("non-null function pointer")(
                    token_br,
                    &raw mut *bands.offset(2 as ::core::ffi::c_int as isize)
                        as *mut *const VP8BandProbas
                        as *const *const VP8BandProbas,
                    ctx_1,
                    &raw const (*q).uv_mat_ as *const ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dst,
                ) as ::core::ffi::c_int;
                l_0 = (nz_1 > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                tnz = (tnz as ::core::ffi::c_int >> 1 as ::core::ffi::c_int
                    | l_0 << 3 as ::core::ffi::c_int) as uint8_t;
                nz_coeffs_0 = NzCodeBits(
                    nz_coeffs_0,
                    nz_1,
                    (*dst.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                );
                dst = dst.offset(16 as ::core::ffi::c_int as isize);
                x += 1;
            }
            tnz = (tnz as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as uint8_t;
            lnz = (lnz as ::core::ffi::c_int >> 1 as ::core::ffi::c_int
                | l_0 << 5 as ::core::ffi::c_int) as uint8_t;
            y += 1;
        }
        non_zero_uv |= nz_coeffs_0 << 4 as ::core::ffi::c_int * ch;
        out_t_nz |= (((tnz as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) << ch) as uint32_t;
        out_l_nz |= ((lnz as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int) << ch) as uint32_t;
        ch += 2 as ::core::ffi::c_int;
    }
    (*mb).nz_ = out_t_nz as uint8_t;
    (*left_mb).nz_ = out_l_nz as uint8_t;
    (*block).non_zero_y_ = non_zero_y;
    (*block).non_zero_uv_ = non_zero_uv;
    (*block).dither_ = (if non_zero_uv & 0xaaaa as uint32_t != 0 {
        0 as ::core::ffi::c_int
    } else {
        (*q).dither_
    }) as uint8_t;
    return (non_zero_y | non_zero_uv == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8DecodeMB(
    dec: *mut VP8Decoder,
    token_br: *mut VP8BitReader,
) -> ::core::ffi::c_int {
    let left: *mut VP8MB = (*dec).mb_info_.offset(-(1 as ::core::ffi::c_int as isize));
    let mb: *mut VP8MB = (*dec).mb_info_.offset((*dec).mb_x_ as isize);
    let block: *mut VP8MBData = (*dec).mb_data_.offset((*dec).mb_x_ as isize);
    let mut skip: ::core::ffi::c_int = if (*dec).use_skip_proba_ != 0 {
        (*block).skip_ as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    if skip == 0 {
        skip = ParseResiduals(dec, mb, token_br);
    } else {
        (*mb).nz_ = 0 as uint8_t;
        (*left).nz_ = (*mb).nz_;
        if (*block).is_i4x4_ == 0 {
            (*mb).nz_dc_ = 0 as uint8_t;
            (*left).nz_dc_ = (*mb).nz_dc_;
        }
        (*block).non_zero_y_ = 0 as uint32_t;
        (*block).non_zero_uv_ = 0 as uint32_t;
        (*block).dither_ = 0 as uint8_t;
    }
    if (*dec).filter_type_ > 0 as ::core::ffi::c_int {
        let finfo: *mut VP8FInfo = (*dec).f_info_.offset((*dec).mb_x_ as isize);
        *finfo = (*dec).fstrengths_[(*block).segment_ as usize][(*block).is_i4x4_ as usize];
        (*finfo).f_inner_ = ((*finfo).f_inner_ as ::core::ffi::c_int
            | (skip == 0) as ::core::ffi::c_int) as uint8_t;
    }
    return ((*token_br).eof_ == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8InitScanline(dec: *mut VP8Decoder) {
    let left: *mut VP8MB = (*dec).mb_info_.offset(-(1 as ::core::ffi::c_int as isize));
    (*left).nz_ = 0 as uint8_t;
    (*left).nz_dc_ = 0 as uint8_t;
    memset(
        &raw mut (*dec).intra_l_ as *mut uint8_t as *mut ::core::ffi::c_void,
        B_DC_PRED as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
    );
    (*dec).mb_x_ = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ParseFrame(dec: *mut VP8Decoder, mut io: *mut VP8Io) -> ::core::ffi::c_int {
    (*dec).mb_y_ = 0 as ::core::ffi::c_int;
    while (*dec).mb_y_ < (*dec).br_mb_y_ {
        let token_br: *mut VP8BitReader = (&raw mut (*dec).parts_ as *mut VP8BitReader)
            .offset(((*dec).mb_y_ as uint32_t & (*dec).num_parts_minus_one_) as isize)
            as *mut VP8BitReader;
        if VP8ParseIntraModeRow(&raw mut (*dec).br_, dec) == 0 {
            return VP8SetError(
                dec,
                VP8_STATUS_NOT_ENOUGH_DATA,
                b"Premature end-of-partition0 encountered.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        while (*dec).mb_x_ < (*dec).mb_w_ {
            if VP8DecodeMB(dec, token_br) == 0 {
                return VP8SetError(
                    dec,
                    VP8_STATUS_NOT_ENOUGH_DATA,
                    b"Premature end-of-file encountered.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            (*dec).mb_x_ += 1;
        }
        VP8InitScanline(dec);
        if VP8ProcessRow(dec, io) == 0 {
            return VP8SetError(
                dec,
                VP8_STATUS_USER_ABORT,
                b"Output aborted.\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*dec).mb_y_ += 1;
    }
    if (*dec).mt_method_ > 0 as ::core::ffi::c_int {
        if (*WebPGetWorkerInterface())
            .Sync_0
            .expect("non-null function pointer")(&raw mut (*dec).worker_)
            == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8Decode(dec: *mut VP8Decoder, io: *mut VP8Io) -> ::core::ffi::c_int {
    let mut ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if dec.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if io.is_null() {
        return VP8SetError(
            dec,
            VP8_STATUS_INVALID_PARAM,
            b"NULL VP8Io parameter in VP8Decode().\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*dec).ready_ == 0 {
        if VP8GetHeaders(dec, io) == 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    '_c2rust_label: {
        if (*dec).ready_ != 0 {
        } else {
            __assert_fail(
                b"dec->ready_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8_dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                687 as ::core::ffi::c_uint,
                b"int VP8Decode(VP8Decoder *const, VP8Io *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    ok = (VP8EnterCritical(dec, io) as ::core::ffi::c_uint
        == VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
    if ok != 0 {
        if ok != 0 {
            ok = VP8InitFrame(dec, io);
        }
        if ok != 0 {
            ok = ParseFrame(dec, io);
        }
        ok &= VP8ExitCritical(dec, io);
    }
    if ok == 0 {
        VP8Clear(dec);
        return 0 as ::core::ffi::c_int;
    }
    (*dec).ready_ = 0 as ::core::ffi::c_int;
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn VP8Clear(dec: *mut VP8Decoder) {
    if dec.is_null() {
        return;
    }
    (*WebPGetWorkerInterface())
        .End
        .expect("non-null function pointer")(&raw mut (*dec).worker_);
    WebPDeallocateAlphaMemory(dec);
    WebPSafeFree((*dec).mem_);
    (*dec).mem_ = NULL;
    (*dec).mem_size_ = 0 as size_t;
    memset(
        &raw mut (*dec).br_ as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VP8BitReader>() as size_t,
    );
    (*dec).ready_ = 0 as ::core::ffi::c_int;
}
