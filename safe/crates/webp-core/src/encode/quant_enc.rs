use c2rust_bitfields::BitfieldStruct;

#[repr(C)]
pub struct VP8Tokens {
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    static mut VP8ITransform: VP8Idct;
    static mut VP8FTransform: VP8Fdct;
    static mut VP8FTransform2: VP8Fdct;
    static mut VP8FTransformWHT: VP8WHT;
    static mut VP8EncPredLuma4: VP8Intra4Preds;
    static mut VP8EncPredLuma16: VP8IntraPreds;
    static mut VP8EncPredChroma8: VP8IntraPreds;
    static mut VP8SSE16x16: VP8Metric;
    static mut VP8SSE16x8: VP8Metric;
    static mut VP8SSE4x4: VP8Metric;
    static mut VP8TDisto16x16: VP8WMetric;
    static mut VP8TDisto4x4: VP8WMetric;
    static mut VP8Copy4x4: VP8BlockCopy;
    static mut VP8Copy16x8: VP8BlockCopy;
    static mut VP8EncQuantizeBlock: VP8QuantizeBlock;
    static mut VP8EncQuantize2Blocks: VP8Quantize2Blocks;
    static mut VP8EncQuantizeBlockWHT: VP8QuantizeBlockWHT;
    static VP8EntropyCost: [uint16_t; 256];
    static VP8LevelFixedCosts: [uint16_t; 2048];
    static VP8EncBands: [uint8_t; 17];
    static mut VP8TransformWHT: VP8WHT;
    fn VP8IteratorStartI4(it: *mut VP8EncIterator);
    fn VP8IteratorRotateI4(it: *mut VP8EncIterator, yuv_out: *const uint8_t) -> ::core::ffi::c_int;
    fn VP8IteratorNzToBytes(it: *mut VP8EncIterator);
    fn VP8SetIntra16Mode(it: *const VP8EncIterator, mode: ::core::ffi::c_int);
    fn VP8SetIntra4Mode(it: *const VP8EncIterator, modes: *const uint8_t);
    fn VP8SetIntraUVMode(it: *const VP8EncIterator, mode: ::core::ffi::c_int);
    fn VP8SetSkip(it: *const VP8EncIterator, skip: ::core::ffi::c_int);
    fn VP8GetCostLuma16(it: *mut VP8EncIterator, rd: *const VP8ModeScore) -> ::core::ffi::c_int;
    fn VP8GetCostLuma4(it: *mut VP8EncIterator, levels: *const int16_t) -> ::core::ffi::c_int;
    fn VP8GetCostUV(it: *mut VP8EncIterator, rd: *const VP8ModeScore) -> ::core::ffi::c_int;
    fn VP8FilterStrengthFromDelta(
        sharpness: ::core::ffi::c_int,
        delta: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static VP8FixedCostsUV: [uint16_t; 4];
    static VP8FixedCostsI16: [uint16_t; 4];
    static VP8FixedCostsI4: [[[uint16_t; 10]; 10]; 10];
}
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type size_t = usize;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type VP8Idct = Option<
    unsafe extern "C" fn(*const uint8_t, *const int16_t, *mut uint8_t, ::core::ffi::c_int) -> (),
>;
pub type VP8Fdct = Option<unsafe extern "C" fn(*const uint8_t, *const uint8_t, *mut int16_t) -> ()>;
pub type VP8WHT = Option<unsafe extern "C" fn(*const int16_t, *mut int16_t) -> ()>;
pub type VP8IntraPreds =
    Option<unsafe extern "C" fn(*mut uint8_t, *const uint8_t, *const uint8_t) -> ()>;
pub type VP8Intra4Preds = Option<unsafe extern "C" fn(*mut uint8_t, *const uint8_t) -> ()>;
pub type VP8Metric =
    Option<unsafe extern "C" fn(*const uint8_t, *const uint8_t) -> ::core::ffi::c_int>;
pub type VP8WMetric = Option<
    unsafe extern "C" fn(*const uint8_t, *const uint8_t, *const uint16_t) -> ::core::ffi::c_int,
>;
pub type VP8BlockCopy = Option<unsafe extern "C" fn(*const uint8_t, *mut uint8_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Matrix {
    pub q_: [uint16_t; 16],
    pub iq_: [uint16_t; 16],
    pub bias_: [uint32_t; 16],
    pub zthresh_: [uint32_t; 16],
    pub sharpen_: [uint16_t; 16],
}
pub type VP8QuantizeBlock = Option<
    unsafe extern "C" fn(*mut int16_t, *mut int16_t, *const VP8Matrix) -> ::core::ffi::c_int,
>;
pub type VP8Quantize2Blocks = Option<
    unsafe extern "C" fn(*mut int16_t, *mut int16_t, *const VP8Matrix) -> ::core::ffi::c_int,
>;
pub type VP8QuantizeBlockWHT = Option<
    unsafe extern "C" fn(*mut int16_t, *mut int16_t, *const VP8Matrix) -> ::core::ffi::c_int,
>;
pub type CostArrayPtr = *mut [*const uint16_t; 3];
pub type StatsArray = [[proba_t; 11]; 3];
pub type proba_t = uint32_t;
pub type ProbaArray = [[uint8_t; 11]; 3];
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const NUM_B_DC_MODES: C2RustUnnamed = 7;
pub const B_DC_PRED_NOTOPLEFT: C2RustUnnamed = 6;
pub const B_DC_PRED_NOLEFT: C2RustUnnamed = 5;
pub const B_DC_PRED_NOTOP: C2RustUnnamed = 4;
pub const NUM_PRED_MODES: C2RustUnnamed = 4;
pub const B_PRED: C2RustUnnamed = 10;
pub const TM_PRED: C2RustUnnamed = 1;
pub const H_PRED: C2RustUnnamed = 3;
pub const V_PRED: C2RustUnnamed = 2;
pub const DC_PRED: C2RustUnnamed = 0;
pub const NUM_BMODES: C2RustUnnamed = 10;
pub const B_HU_PRED: C2RustUnnamed = 9;
pub const B_HD_PRED: C2RustUnnamed = 8;
pub const B_VL_PRED: C2RustUnnamed = 7;
pub const B_LD_PRED: C2RustUnnamed = 6;
pub const B_VR_PRED: C2RustUnnamed = 5;
pub const B_RD_PRED: C2RustUnnamed = 4;
pub const B_HE_PRED: C2RustUnnamed = 3;
pub const B_VE_PRED: C2RustUnnamed = 2;
pub const B_TM_PRED: C2RustUnnamed = 1;
pub const B_DC_PRED: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const NUM_PROBAS: C2RustUnnamed_0 = 11;
pub const NUM_CTX: C2RustUnnamed_0 = 3;
pub const NUM_BANDS: C2RustUnnamed_0 = 8;
pub const NUM_TYPES: C2RustUnnamed_0 = 4;
pub const MAX_NUM_PARTITIONS: C2RustUnnamed_0 = 8;
pub const NUM_MODE_LF_DELTAS: C2RustUnnamed_0 = 4;
pub const NUM_REF_LF_DELTAS: C2RustUnnamed_0 = 4;
pub const NUM_MB_SEGMENTS: C2RustUnnamed_0 = 4;
pub const MB_FEATURE_TREE_PROBS: C2RustUnnamed_0 = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8BitWriter {
    pub range_: int32_t,
    pub value_: int32_t,
    pub run_: ::core::ffi::c_int,
    pub nb_bits_: ::core::ffi::c_int,
    pub buf_: *mut uint8_t,
    pub pos_: size_t,
    pub max_pos_: size_t,
    pub error_: ::core::ffi::c_int,
}
pub type WebPWorkerStatus = ::core::ffi::c_uint;
pub const WORK: WebPWorkerStatus = 2;
pub const OK: WebPWorkerStatus = 1;
pub const NOT_OK: WebPWorkerStatus = 0;
pub type WebPWorkerHook = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::core::ffi::c_int,
>;
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
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const MAX_LEVEL: C2RustUnnamed_1 = 2047;
pub const MAX_VARIABLE_LEVEL: C2RustUnnamed_1 = 67;
pub const MAX_LF_LEVELS: C2RustUnnamed_1 = 64;
pub type VP8RDLevel = ::core::ffi::c_uint;
pub const RD_OPT_TRELLIS_ALL: VP8RDLevel = 3;
pub const RD_OPT_TRELLIS: VP8RDLevel = 2;
pub const RD_OPT_BASIC: VP8RDLevel = 1;
pub const RD_OPT_NONE: VP8RDLevel = 0;
pub type score_t = int64_t;
pub type CostArray = [[uint16_t; 68]; 3];
pub type CostArrayMap = [[*const uint16_t; 3]; 16];
pub type LFStats = [[::core::ffi::c_double; 64]; 4];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Encoder {
    pub config_: *const WebPConfig,
    pub pic_: *mut WebPPicture,
    pub filter_hdr_: VP8EncFilterHeader,
    pub segment_hdr_: VP8EncSegmentHeader,
    pub profile_: ::core::ffi::c_int,
    pub mb_w_: ::core::ffi::c_int,
    pub mb_h_: ::core::ffi::c_int,
    pub preds_w_: ::core::ffi::c_int,
    pub num_parts_: ::core::ffi::c_int,
    pub bw_: VP8BitWriter,
    pub parts_: [VP8BitWriter; 8],
    pub tokens_: VP8TBuffer,
    pub percent_: ::core::ffi::c_int,
    pub has_alpha_: ::core::ffi::c_int,
    pub alpha_data_: *mut uint8_t,
    pub alpha_data_size_: uint32_t,
    pub alpha_worker_: WebPWorker,
    pub dqm_: [VP8SegmentInfo; 4],
    pub base_quant_: ::core::ffi::c_int,
    pub alpha_: ::core::ffi::c_int,
    pub uv_alpha_: ::core::ffi::c_int,
    pub dq_y1_dc_: ::core::ffi::c_int,
    pub dq_y2_dc_: ::core::ffi::c_int,
    pub dq_y2_ac_: ::core::ffi::c_int,
    pub dq_uv_dc_: ::core::ffi::c_int,
    pub dq_uv_ac_: ::core::ffi::c_int,
    pub proba_: VP8EncProba,
    pub sse_: [uint64_t; 4],
    pub sse_count_: uint64_t,
    pub coded_size_: ::core::ffi::c_int,
    pub residual_bytes_: [[::core::ffi::c_int; 4]; 3],
    pub block_count_: [::core::ffi::c_int; 3],
    pub method_: ::core::ffi::c_int,
    pub rd_opt_level_: VP8RDLevel,
    pub max_i4_header_bits_: ::core::ffi::c_int,
    pub mb_header_limit_: ::core::ffi::c_int,
    pub thread_level_: ::core::ffi::c_int,
    pub do_search_: ::core::ffi::c_int,
    pub use_tokens_: ::core::ffi::c_int,
    pub mb_info_: *mut VP8MBInfo,
    pub preds_: *mut uint8_t,
    pub nz_: *mut uint32_t,
    pub y_top_: *mut uint8_t,
    pub uv_top_: *mut uint8_t,
    pub lf_stats_: *mut LFStats,
    pub top_derr_: *mut DError,
}
pub type DError = [[int8_t; 2]; 2];
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct VP8MBInfo {
    #[bitfield(name = "type_", ty = "::core::ffi::c_uint", bits = "0..=1")]
    #[bitfield(name = "uv_mode_", ty = "::core::ffi::c_uint", bits = "2..=3")]
    #[bitfield(name = "skip_", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "segment_", ty = "::core::ffi::c_uint", bits = "5..=6")]
    pub type__uv_mode__skip__segment_: [u8; 1],
    pub alpha_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8EncProba {
    pub segments_: [uint8_t; 3],
    pub skip_proba_: uint8_t,
    pub coeffs_: [[ProbaArray; 8]; 4],
    pub stats_: [[StatsArray; 8]; 4],
    pub level_cost_: [[CostArray; 8]; 4],
    pub remapped_costs_: [CostArrayMap; 4],
    pub dirty_: ::core::ffi::c_int,
    pub use_skip_proba_: ::core::ffi::c_int,
    pub nb_skip_: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8SegmentInfo {
    pub y1_: VP8Matrix,
    pub y2_: VP8Matrix,
    pub uv_: VP8Matrix,
    pub alpha_: ::core::ffi::c_int,
    pub beta_: ::core::ffi::c_int,
    pub quant_: ::core::ffi::c_int,
    pub fstrength_: ::core::ffi::c_int,
    pub max_edge_: ::core::ffi::c_int,
    pub min_disto_: ::core::ffi::c_int,
    pub lambda_i16_: ::core::ffi::c_int,
    pub lambda_i4_: ::core::ffi::c_int,
    pub lambda_uv_: ::core::ffi::c_int,
    pub lambda_mode_: ::core::ffi::c_int,
    pub lambda_trellis_: ::core::ffi::c_int,
    pub tlambda_: ::core::ffi::c_int,
    pub lambda_trellis_i16_: ::core::ffi::c_int,
    pub lambda_trellis_i4_: ::core::ffi::c_int,
    pub lambda_trellis_uv_: ::core::ffi::c_int,
    pub i4_penalty_: score_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8TBuffer {
    pub pages_: *mut VP8Tokens,
    pub last_page_: *mut *mut VP8Tokens,
    pub tokens_: *mut uint16_t,
    pub left_: ::core::ffi::c_int,
    pub page_size_: ::core::ffi::c_int,
    pub error_: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8EncSegmentHeader {
    pub num_segments_: ::core::ffi::c_int,
    pub update_map_: ::core::ffi::c_int,
    pub size_: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8EncFilterHeader {
    pub simple_: ::core::ffi::c_int,
    pub level_: ::core::ffi::c_int,
    pub sharpness_: ::core::ffi::c_int,
    pub i4x4_lf_delta_: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8ModeScore {
    pub D: score_t,
    pub SD: score_t,
    pub H: score_t,
    pub R: score_t,
    pub score: score_t,
    pub y_dc_levels: [int16_t; 16],
    pub y_ac_levels: [[int16_t; 16]; 16],
    pub uv_levels: [[int16_t; 16]; 8],
    pub mode_i16: ::core::ffi::c_int,
    pub modes_i4: [uint8_t; 16],
    pub mode_uv: ::core::ffi::c_int,
    pub nz: uint32_t,
    pub derr: [[int8_t; 3]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8EncIterator {
    pub x_: ::core::ffi::c_int,
    pub y_: ::core::ffi::c_int,
    pub yuv_in_: *mut uint8_t,
    pub yuv_out_: *mut uint8_t,
    pub yuv_out2_: *mut uint8_t,
    pub yuv_p_: *mut uint8_t,
    pub enc_: *mut VP8Encoder,
    pub mb_: *mut VP8MBInfo,
    pub bw_: *mut VP8BitWriter,
    pub preds_: *mut uint8_t,
    pub nz_: *mut uint32_t,
    pub i4_boundary_: [uint8_t; 37],
    pub i4_top_: *mut uint8_t,
    pub i4_: ::core::ffi::c_int,
    pub top_nz_: [::core::ffi::c_int; 9],
    pub left_nz_: [::core::ffi::c_int; 9],
    pub bit_count_: [[uint64_t; 3]; 4],
    pub luma_bits_: uint64_t,
    pub uv_bits_: uint64_t,
    pub lf_stats_: *mut LFStats,
    pub do_trellis_: ::core::ffi::c_int,
    pub count_down_: ::core::ffi::c_int,
    pub count_down0_: ::core::ffi::c_int,
    pub percent0_: ::core::ffi::c_int,
    pub left_derr_: DError,
    pub top_derr_: *mut DError,
    pub y_left_: *mut uint8_t,
    pub u_left_: *mut uint8_t,
    pub v_left_: *mut uint8_t,
    pub y_top_: *mut uint8_t,
    pub uv_top_: *mut uint8_t,
    pub yuv_left_mem_: [uint8_t; 88],
    pub yuv_mem_: [uint8_t; 3359],
}
pub const TYPE_CHROMA_A: C2RustUnnamed_2 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub prev: int8_t,
    pub sign: int8_t,
    pub level: int16_t,
}
pub const TYPE_I16_AC: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScoreState {
    pub score: score_t,
    pub costs: *const uint16_t,
}
pub const TYPE_I4_AC: C2RustUnnamed_2 = 3;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const TYPE_I16_DC: C2RustUnnamed_2 = 1;
pub const BPS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const Y_OFF_ENC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const U_OFF_ENC: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const I16DC16: ::core::ffi::c_int = 0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * BPS;
pub const I16TM16: ::core::ffi::c_int = I16DC16 + 16 as ::core::ffi::c_int;
pub const I16VE16: ::core::ffi::c_int = 1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * BPS;
pub const I16HE16: ::core::ffi::c_int = I16VE16 + 16 as ::core::ffi::c_int;
pub const C8DC8: ::core::ffi::c_int = 2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * BPS;
pub const C8TM8: ::core::ffi::c_int = C8DC8 + 1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int;
pub const C8VE8: ::core::ffi::c_int =
    2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * BPS + 8 as ::core::ffi::c_int * BPS;
pub const C8HE8: ::core::ffi::c_int = C8VE8 + 1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int;
pub const I4DC4: ::core::ffi::c_int =
    3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * BPS + 0 as ::core::ffi::c_int;
pub const I4TM4: ::core::ffi::c_int = I4DC4 + 4 as ::core::ffi::c_int;
pub const I4VE4: ::core::ffi::c_int = I4DC4 + 8 as ::core::ffi::c_int;
pub const I4HE4: ::core::ffi::c_int = I4DC4 + 12 as ::core::ffi::c_int;
pub const I4RD4: ::core::ffi::c_int = I4DC4 + 16 as ::core::ffi::c_int;
pub const I4VR4: ::core::ffi::c_int = I4DC4 + 20 as ::core::ffi::c_int;
pub const I4LD4: ::core::ffi::c_int = I4DC4 + 24 as ::core::ffi::c_int;
pub const I4VL4: ::core::ffi::c_int = I4DC4 + 28 as ::core::ffi::c_int;
pub const I4HD4: ::core::ffi::c_int =
    3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * BPS + 4 as ::core::ffi::c_int * BPS;
pub const I4HU4: ::core::ffi::c_int = I4HD4 + 4 as ::core::ffi::c_int;
pub const I4TMP: ::core::ffi::c_int = I4HD4 + 8 as ::core::ffi::c_int;
pub const MAX_COST: score_t = 0x7fffffffffffff as ::core::ffi::c_longlong as score_t;
pub const QFIX: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn QUANTDIV(
    mut n: uint32_t,
    mut iQ: uint32_t,
    mut B: uint32_t,
) -> ::core::ffi::c_int {
    return (n.wrapping_mul(iQ).wrapping_add(B) >> QFIX) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn VP8BitCost(
    mut bit: ::core::ffi::c_int,
    mut proba: uint8_t,
) -> ::core::ffi::c_int {
    return if bit == 0 {
        VP8EntropyCost[proba as usize] as ::core::ffi::c_int
    } else {
        VP8EntropyCost[(255 as ::core::ffi::c_int - proba as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn VP8LevelCost(
    table: *const uint16_t,
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return VP8LevelFixedCosts[level as usize] as ::core::ffi::c_int
        + *table.offset(
            (if level > MAX_VARIABLE_LEVEL as ::core::ffi::c_int {
                MAX_VARIABLE_LEVEL as ::core::ffi::c_int
            } else {
                level
            }) as isize,
        ) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn IsFlat_C(
    mut levels: *const int16_t,
    mut num_blocks: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut score: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let fresh0 = num_blocks;
        num_blocks = num_blocks - 1;
        if !(fresh0 > 0 as ::core::ffi::c_int) {
            break;
        }
        let mut i: ::core::ffi::c_int = 0;
        i = 1 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            score += (*levels.offset(i as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            if score > thresh {
                return 0 as ::core::ffi::c_int;
            }
            i += 1;
        }
        levels = levels.offset(16 as ::core::ffi::c_int as isize);
    }
    return 1 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn IsFlatSource16(mut src: *const uint8_t) -> ::core::ffi::c_int {
    let v: uint32_t = (*src.offset(0 as ::core::ffi::c_int as isize) as uint32_t)
        .wrapping_mul(0x1010101 as uint32_t);
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        if memcmp(
            src.offset(0 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            &raw const v as *const ::core::ffi::c_void,
            4 as size_t,
        ) != 0
            || memcmp(
                src.offset(4 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                &raw const v as *const ::core::ffi::c_void,
                4 as size_t,
            ) != 0
            || memcmp(
                src.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                &raw const v as *const ::core::ffi::c_void,
                4 as size_t,
            ) != 0
            || memcmp(
                src.offset(12 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                &raw const v as *const ::core::ffi::c_void,
                4 as size_t,
            ) != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        src = src.offset(BPS as isize);
        i += 1;
    }
    return 1 as ::core::ffi::c_int;
}
pub const DO_TRELLIS_I4: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DO_TRELLIS_I16: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DO_TRELLIS_UV: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MID_ALPHA: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MIN_ALPHA: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const MAX_ALPHA: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SNS_TO_DQ: ::core::ffi::c_double = 0.9f64;
pub const FLATNESS_LIMIT_I16: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FLATNESS_LIMIT_I4: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const FLATNESS_LIMIT_UV: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FLATNESS_PENALTY: ::core::ffi::c_int = 140 as ::core::ffi::c_int;
pub const RD_DISTO_MULT: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn clip(
    mut v: ::core::ffi::c_int,
    mut m: ::core::ffi::c_int,
    mut M: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if v < m {
        m
    } else if v > M {
        M
    } else {
        v
    };
}
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
static mut kDcTable: [uint8_t; 128] = [
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    12 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    15 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    17 as ::core::ffi::c_int as uint8_t,
    17 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    28 as ::core::ffi::c_int as uint8_t,
    29 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    31 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    34 as ::core::ffi::c_int as uint8_t,
    35 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    37 as ::core::ffi::c_int as uint8_t,
    37 as ::core::ffi::c_int as uint8_t,
    38 as ::core::ffi::c_int as uint8_t,
    39 as ::core::ffi::c_int as uint8_t,
    40 as ::core::ffi::c_int as uint8_t,
    41 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    43 as ::core::ffi::c_int as uint8_t,
    44 as ::core::ffi::c_int as uint8_t,
    45 as ::core::ffi::c_int as uint8_t,
    46 as ::core::ffi::c_int as uint8_t,
    46 as ::core::ffi::c_int as uint8_t,
    47 as ::core::ffi::c_int as uint8_t,
    48 as ::core::ffi::c_int as uint8_t,
    49 as ::core::ffi::c_int as uint8_t,
    50 as ::core::ffi::c_int as uint8_t,
    51 as ::core::ffi::c_int as uint8_t,
    52 as ::core::ffi::c_int as uint8_t,
    53 as ::core::ffi::c_int as uint8_t,
    54 as ::core::ffi::c_int as uint8_t,
    55 as ::core::ffi::c_int as uint8_t,
    56 as ::core::ffi::c_int as uint8_t,
    57 as ::core::ffi::c_int as uint8_t,
    58 as ::core::ffi::c_int as uint8_t,
    59 as ::core::ffi::c_int as uint8_t,
    60 as ::core::ffi::c_int as uint8_t,
    61 as ::core::ffi::c_int as uint8_t,
    62 as ::core::ffi::c_int as uint8_t,
    63 as ::core::ffi::c_int as uint8_t,
    64 as ::core::ffi::c_int as uint8_t,
    65 as ::core::ffi::c_int as uint8_t,
    66 as ::core::ffi::c_int as uint8_t,
    67 as ::core::ffi::c_int as uint8_t,
    68 as ::core::ffi::c_int as uint8_t,
    69 as ::core::ffi::c_int as uint8_t,
    70 as ::core::ffi::c_int as uint8_t,
    71 as ::core::ffi::c_int as uint8_t,
    72 as ::core::ffi::c_int as uint8_t,
    73 as ::core::ffi::c_int as uint8_t,
    74 as ::core::ffi::c_int as uint8_t,
    75 as ::core::ffi::c_int as uint8_t,
    76 as ::core::ffi::c_int as uint8_t,
    76 as ::core::ffi::c_int as uint8_t,
    77 as ::core::ffi::c_int as uint8_t,
    78 as ::core::ffi::c_int as uint8_t,
    79 as ::core::ffi::c_int as uint8_t,
    80 as ::core::ffi::c_int as uint8_t,
    81 as ::core::ffi::c_int as uint8_t,
    82 as ::core::ffi::c_int as uint8_t,
    83 as ::core::ffi::c_int as uint8_t,
    84 as ::core::ffi::c_int as uint8_t,
    85 as ::core::ffi::c_int as uint8_t,
    86 as ::core::ffi::c_int as uint8_t,
    87 as ::core::ffi::c_int as uint8_t,
    88 as ::core::ffi::c_int as uint8_t,
    89 as ::core::ffi::c_int as uint8_t,
    91 as ::core::ffi::c_int as uint8_t,
    93 as ::core::ffi::c_int as uint8_t,
    95 as ::core::ffi::c_int as uint8_t,
    96 as ::core::ffi::c_int as uint8_t,
    98 as ::core::ffi::c_int as uint8_t,
    100 as ::core::ffi::c_int as uint8_t,
    101 as ::core::ffi::c_int as uint8_t,
    102 as ::core::ffi::c_int as uint8_t,
    104 as ::core::ffi::c_int as uint8_t,
    106 as ::core::ffi::c_int as uint8_t,
    108 as ::core::ffi::c_int as uint8_t,
    110 as ::core::ffi::c_int as uint8_t,
    112 as ::core::ffi::c_int as uint8_t,
    114 as ::core::ffi::c_int as uint8_t,
    116 as ::core::ffi::c_int as uint8_t,
    118 as ::core::ffi::c_int as uint8_t,
    122 as ::core::ffi::c_int as uint8_t,
    124 as ::core::ffi::c_int as uint8_t,
    126 as ::core::ffi::c_int as uint8_t,
    128 as ::core::ffi::c_int as uint8_t,
    130 as ::core::ffi::c_int as uint8_t,
    132 as ::core::ffi::c_int as uint8_t,
    134 as ::core::ffi::c_int as uint8_t,
    136 as ::core::ffi::c_int as uint8_t,
    138 as ::core::ffi::c_int as uint8_t,
    140 as ::core::ffi::c_int as uint8_t,
    143 as ::core::ffi::c_int as uint8_t,
    145 as ::core::ffi::c_int as uint8_t,
    148 as ::core::ffi::c_int as uint8_t,
    151 as ::core::ffi::c_int as uint8_t,
    154 as ::core::ffi::c_int as uint8_t,
    157 as ::core::ffi::c_int as uint8_t,
];
static mut kAcTable: [uint16_t; 128] = [
    4 as ::core::ffi::c_int as uint16_t,
    5 as ::core::ffi::c_int as uint16_t,
    6 as ::core::ffi::c_int as uint16_t,
    7 as ::core::ffi::c_int as uint16_t,
    8 as ::core::ffi::c_int as uint16_t,
    9 as ::core::ffi::c_int as uint16_t,
    10 as ::core::ffi::c_int as uint16_t,
    11 as ::core::ffi::c_int as uint16_t,
    12 as ::core::ffi::c_int as uint16_t,
    13 as ::core::ffi::c_int as uint16_t,
    14 as ::core::ffi::c_int as uint16_t,
    15 as ::core::ffi::c_int as uint16_t,
    16 as ::core::ffi::c_int as uint16_t,
    17 as ::core::ffi::c_int as uint16_t,
    18 as ::core::ffi::c_int as uint16_t,
    19 as ::core::ffi::c_int as uint16_t,
    20 as ::core::ffi::c_int as uint16_t,
    21 as ::core::ffi::c_int as uint16_t,
    22 as ::core::ffi::c_int as uint16_t,
    23 as ::core::ffi::c_int as uint16_t,
    24 as ::core::ffi::c_int as uint16_t,
    25 as ::core::ffi::c_int as uint16_t,
    26 as ::core::ffi::c_int as uint16_t,
    27 as ::core::ffi::c_int as uint16_t,
    28 as ::core::ffi::c_int as uint16_t,
    29 as ::core::ffi::c_int as uint16_t,
    30 as ::core::ffi::c_int as uint16_t,
    31 as ::core::ffi::c_int as uint16_t,
    32 as ::core::ffi::c_int as uint16_t,
    33 as ::core::ffi::c_int as uint16_t,
    34 as ::core::ffi::c_int as uint16_t,
    35 as ::core::ffi::c_int as uint16_t,
    36 as ::core::ffi::c_int as uint16_t,
    37 as ::core::ffi::c_int as uint16_t,
    38 as ::core::ffi::c_int as uint16_t,
    39 as ::core::ffi::c_int as uint16_t,
    40 as ::core::ffi::c_int as uint16_t,
    41 as ::core::ffi::c_int as uint16_t,
    42 as ::core::ffi::c_int as uint16_t,
    43 as ::core::ffi::c_int as uint16_t,
    44 as ::core::ffi::c_int as uint16_t,
    45 as ::core::ffi::c_int as uint16_t,
    46 as ::core::ffi::c_int as uint16_t,
    47 as ::core::ffi::c_int as uint16_t,
    48 as ::core::ffi::c_int as uint16_t,
    49 as ::core::ffi::c_int as uint16_t,
    50 as ::core::ffi::c_int as uint16_t,
    51 as ::core::ffi::c_int as uint16_t,
    52 as ::core::ffi::c_int as uint16_t,
    53 as ::core::ffi::c_int as uint16_t,
    54 as ::core::ffi::c_int as uint16_t,
    55 as ::core::ffi::c_int as uint16_t,
    56 as ::core::ffi::c_int as uint16_t,
    57 as ::core::ffi::c_int as uint16_t,
    58 as ::core::ffi::c_int as uint16_t,
    60 as ::core::ffi::c_int as uint16_t,
    62 as ::core::ffi::c_int as uint16_t,
    64 as ::core::ffi::c_int as uint16_t,
    66 as ::core::ffi::c_int as uint16_t,
    68 as ::core::ffi::c_int as uint16_t,
    70 as ::core::ffi::c_int as uint16_t,
    72 as ::core::ffi::c_int as uint16_t,
    74 as ::core::ffi::c_int as uint16_t,
    76 as ::core::ffi::c_int as uint16_t,
    78 as ::core::ffi::c_int as uint16_t,
    80 as ::core::ffi::c_int as uint16_t,
    82 as ::core::ffi::c_int as uint16_t,
    84 as ::core::ffi::c_int as uint16_t,
    86 as ::core::ffi::c_int as uint16_t,
    88 as ::core::ffi::c_int as uint16_t,
    90 as ::core::ffi::c_int as uint16_t,
    92 as ::core::ffi::c_int as uint16_t,
    94 as ::core::ffi::c_int as uint16_t,
    96 as ::core::ffi::c_int as uint16_t,
    98 as ::core::ffi::c_int as uint16_t,
    100 as ::core::ffi::c_int as uint16_t,
    102 as ::core::ffi::c_int as uint16_t,
    104 as ::core::ffi::c_int as uint16_t,
    106 as ::core::ffi::c_int as uint16_t,
    108 as ::core::ffi::c_int as uint16_t,
    110 as ::core::ffi::c_int as uint16_t,
    112 as ::core::ffi::c_int as uint16_t,
    114 as ::core::ffi::c_int as uint16_t,
    116 as ::core::ffi::c_int as uint16_t,
    119 as ::core::ffi::c_int as uint16_t,
    122 as ::core::ffi::c_int as uint16_t,
    125 as ::core::ffi::c_int as uint16_t,
    128 as ::core::ffi::c_int as uint16_t,
    131 as ::core::ffi::c_int as uint16_t,
    134 as ::core::ffi::c_int as uint16_t,
    137 as ::core::ffi::c_int as uint16_t,
    140 as ::core::ffi::c_int as uint16_t,
    143 as ::core::ffi::c_int as uint16_t,
    146 as ::core::ffi::c_int as uint16_t,
    149 as ::core::ffi::c_int as uint16_t,
    152 as ::core::ffi::c_int as uint16_t,
    155 as ::core::ffi::c_int as uint16_t,
    158 as ::core::ffi::c_int as uint16_t,
    161 as ::core::ffi::c_int as uint16_t,
    164 as ::core::ffi::c_int as uint16_t,
    167 as ::core::ffi::c_int as uint16_t,
    170 as ::core::ffi::c_int as uint16_t,
    173 as ::core::ffi::c_int as uint16_t,
    177 as ::core::ffi::c_int as uint16_t,
    181 as ::core::ffi::c_int as uint16_t,
    185 as ::core::ffi::c_int as uint16_t,
    189 as ::core::ffi::c_int as uint16_t,
    193 as ::core::ffi::c_int as uint16_t,
    197 as ::core::ffi::c_int as uint16_t,
    201 as ::core::ffi::c_int as uint16_t,
    205 as ::core::ffi::c_int as uint16_t,
    209 as ::core::ffi::c_int as uint16_t,
    213 as ::core::ffi::c_int as uint16_t,
    217 as ::core::ffi::c_int as uint16_t,
    221 as ::core::ffi::c_int as uint16_t,
    225 as ::core::ffi::c_int as uint16_t,
    229 as ::core::ffi::c_int as uint16_t,
    234 as ::core::ffi::c_int as uint16_t,
    239 as ::core::ffi::c_int as uint16_t,
    245 as ::core::ffi::c_int as uint16_t,
    249 as ::core::ffi::c_int as uint16_t,
    254 as ::core::ffi::c_int as uint16_t,
    259 as ::core::ffi::c_int as uint16_t,
    264 as ::core::ffi::c_int as uint16_t,
    269 as ::core::ffi::c_int as uint16_t,
    274 as ::core::ffi::c_int as uint16_t,
    279 as ::core::ffi::c_int as uint16_t,
    284 as ::core::ffi::c_int as uint16_t,
];
static mut kAcTable2: [uint16_t; 128] = [
    8 as ::core::ffi::c_int as uint16_t,
    8 as ::core::ffi::c_int as uint16_t,
    9 as ::core::ffi::c_int as uint16_t,
    10 as ::core::ffi::c_int as uint16_t,
    12 as ::core::ffi::c_int as uint16_t,
    13 as ::core::ffi::c_int as uint16_t,
    15 as ::core::ffi::c_int as uint16_t,
    17 as ::core::ffi::c_int as uint16_t,
    18 as ::core::ffi::c_int as uint16_t,
    20 as ::core::ffi::c_int as uint16_t,
    21 as ::core::ffi::c_int as uint16_t,
    23 as ::core::ffi::c_int as uint16_t,
    24 as ::core::ffi::c_int as uint16_t,
    26 as ::core::ffi::c_int as uint16_t,
    27 as ::core::ffi::c_int as uint16_t,
    29 as ::core::ffi::c_int as uint16_t,
    31 as ::core::ffi::c_int as uint16_t,
    32 as ::core::ffi::c_int as uint16_t,
    34 as ::core::ffi::c_int as uint16_t,
    35 as ::core::ffi::c_int as uint16_t,
    37 as ::core::ffi::c_int as uint16_t,
    38 as ::core::ffi::c_int as uint16_t,
    40 as ::core::ffi::c_int as uint16_t,
    41 as ::core::ffi::c_int as uint16_t,
    43 as ::core::ffi::c_int as uint16_t,
    44 as ::core::ffi::c_int as uint16_t,
    46 as ::core::ffi::c_int as uint16_t,
    48 as ::core::ffi::c_int as uint16_t,
    49 as ::core::ffi::c_int as uint16_t,
    51 as ::core::ffi::c_int as uint16_t,
    52 as ::core::ffi::c_int as uint16_t,
    54 as ::core::ffi::c_int as uint16_t,
    55 as ::core::ffi::c_int as uint16_t,
    57 as ::core::ffi::c_int as uint16_t,
    58 as ::core::ffi::c_int as uint16_t,
    60 as ::core::ffi::c_int as uint16_t,
    62 as ::core::ffi::c_int as uint16_t,
    63 as ::core::ffi::c_int as uint16_t,
    65 as ::core::ffi::c_int as uint16_t,
    66 as ::core::ffi::c_int as uint16_t,
    68 as ::core::ffi::c_int as uint16_t,
    69 as ::core::ffi::c_int as uint16_t,
    71 as ::core::ffi::c_int as uint16_t,
    72 as ::core::ffi::c_int as uint16_t,
    74 as ::core::ffi::c_int as uint16_t,
    75 as ::core::ffi::c_int as uint16_t,
    77 as ::core::ffi::c_int as uint16_t,
    79 as ::core::ffi::c_int as uint16_t,
    80 as ::core::ffi::c_int as uint16_t,
    82 as ::core::ffi::c_int as uint16_t,
    83 as ::core::ffi::c_int as uint16_t,
    85 as ::core::ffi::c_int as uint16_t,
    86 as ::core::ffi::c_int as uint16_t,
    88 as ::core::ffi::c_int as uint16_t,
    89 as ::core::ffi::c_int as uint16_t,
    93 as ::core::ffi::c_int as uint16_t,
    96 as ::core::ffi::c_int as uint16_t,
    99 as ::core::ffi::c_int as uint16_t,
    102 as ::core::ffi::c_int as uint16_t,
    105 as ::core::ffi::c_int as uint16_t,
    108 as ::core::ffi::c_int as uint16_t,
    111 as ::core::ffi::c_int as uint16_t,
    114 as ::core::ffi::c_int as uint16_t,
    117 as ::core::ffi::c_int as uint16_t,
    120 as ::core::ffi::c_int as uint16_t,
    124 as ::core::ffi::c_int as uint16_t,
    127 as ::core::ffi::c_int as uint16_t,
    130 as ::core::ffi::c_int as uint16_t,
    133 as ::core::ffi::c_int as uint16_t,
    136 as ::core::ffi::c_int as uint16_t,
    139 as ::core::ffi::c_int as uint16_t,
    142 as ::core::ffi::c_int as uint16_t,
    145 as ::core::ffi::c_int as uint16_t,
    148 as ::core::ffi::c_int as uint16_t,
    151 as ::core::ffi::c_int as uint16_t,
    155 as ::core::ffi::c_int as uint16_t,
    158 as ::core::ffi::c_int as uint16_t,
    161 as ::core::ffi::c_int as uint16_t,
    164 as ::core::ffi::c_int as uint16_t,
    167 as ::core::ffi::c_int as uint16_t,
    170 as ::core::ffi::c_int as uint16_t,
    173 as ::core::ffi::c_int as uint16_t,
    176 as ::core::ffi::c_int as uint16_t,
    179 as ::core::ffi::c_int as uint16_t,
    184 as ::core::ffi::c_int as uint16_t,
    189 as ::core::ffi::c_int as uint16_t,
    193 as ::core::ffi::c_int as uint16_t,
    198 as ::core::ffi::c_int as uint16_t,
    203 as ::core::ffi::c_int as uint16_t,
    207 as ::core::ffi::c_int as uint16_t,
    212 as ::core::ffi::c_int as uint16_t,
    217 as ::core::ffi::c_int as uint16_t,
    221 as ::core::ffi::c_int as uint16_t,
    226 as ::core::ffi::c_int as uint16_t,
    230 as ::core::ffi::c_int as uint16_t,
    235 as ::core::ffi::c_int as uint16_t,
    240 as ::core::ffi::c_int as uint16_t,
    244 as ::core::ffi::c_int as uint16_t,
    249 as ::core::ffi::c_int as uint16_t,
    254 as ::core::ffi::c_int as uint16_t,
    258 as ::core::ffi::c_int as uint16_t,
    263 as ::core::ffi::c_int as uint16_t,
    268 as ::core::ffi::c_int as uint16_t,
    274 as ::core::ffi::c_int as uint16_t,
    280 as ::core::ffi::c_int as uint16_t,
    286 as ::core::ffi::c_int as uint16_t,
    292 as ::core::ffi::c_int as uint16_t,
    299 as ::core::ffi::c_int as uint16_t,
    305 as ::core::ffi::c_int as uint16_t,
    311 as ::core::ffi::c_int as uint16_t,
    317 as ::core::ffi::c_int as uint16_t,
    323 as ::core::ffi::c_int as uint16_t,
    330 as ::core::ffi::c_int as uint16_t,
    336 as ::core::ffi::c_int as uint16_t,
    342 as ::core::ffi::c_int as uint16_t,
    348 as ::core::ffi::c_int as uint16_t,
    354 as ::core::ffi::c_int as uint16_t,
    362 as ::core::ffi::c_int as uint16_t,
    370 as ::core::ffi::c_int as uint16_t,
    379 as ::core::ffi::c_int as uint16_t,
    385 as ::core::ffi::c_int as uint16_t,
    393 as ::core::ffi::c_int as uint16_t,
    401 as ::core::ffi::c_int as uint16_t,
    409 as ::core::ffi::c_int as uint16_t,
    416 as ::core::ffi::c_int as uint16_t,
    424 as ::core::ffi::c_int as uint16_t,
    432 as ::core::ffi::c_int as uint16_t,
    440 as ::core::ffi::c_int as uint16_t,
];
static mut kBiasMatrices: [[uint8_t; 2]; 3] = [
    [
        96 as ::core::ffi::c_int as uint8_t,
        110 as ::core::ffi::c_int as uint8_t,
    ],
    [
        96 as ::core::ffi::c_int as uint8_t,
        108 as ::core::ffi::c_int as uint8_t,
    ],
    [
        110 as ::core::ffi::c_int as uint8_t,
        115 as ::core::ffi::c_int as uint8_t,
    ],
];
pub const SHARPEN_BITS: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
static mut kFreqSharpening: [uint8_t; 16] = [
    0 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    60 as ::core::ffi::c_int as uint8_t,
    90 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    60 as ::core::ffi::c_int as uint8_t,
    90 as ::core::ffi::c_int as uint8_t,
    90 as ::core::ffi::c_int as uint8_t,
    60 as ::core::ffi::c_int as uint8_t,
    90 as ::core::ffi::c_int as uint8_t,
    90 as ::core::ffi::c_int as uint8_t,
    90 as ::core::ffi::c_int as uint8_t,
    90 as ::core::ffi::c_int as uint8_t,
    90 as ::core::ffi::c_int as uint8_t,
    90 as ::core::ffi::c_int as uint8_t,
    90 as ::core::ffi::c_int as uint8_t,
];
unsafe extern "C" fn ExpandMatrix(
    m: *mut VP8Matrix,
    mut type_0: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut sum: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        let is_ac_coeff: ::core::ffi::c_int = (i > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        let bias: ::core::ffi::c_int =
            kBiasMatrices[type_0 as usize][is_ac_coeff as usize] as ::core::ffi::c_int;
        (*m).iq_[i as usize] = (((1 as ::core::ffi::c_int) << QFIX)
            / (*m).q_[i as usize] as ::core::ffi::c_int) as uint16_t;
        (*m).bias_[i as usize] = (bias << QFIX - 8 as ::core::ffi::c_int) as uint32_t;
        (*m).zthresh_[i as usize] =
            ((((1 as ::core::ffi::c_int) << QFIX) - 1 as ::core::ffi::c_int) as uint32_t)
                .wrapping_sub((*m).bias_[i as usize])
                .wrapping_div((*m).iq_[i as usize] as uint32_t);
        i += 1;
    }
    i = 2 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*m).q_[i as usize] = (*m).q_[1 as ::core::ffi::c_int as usize];
        (*m).iq_[i as usize] = (*m).iq_[1 as ::core::ffi::c_int as usize];
        (*m).bias_[i as usize] = (*m).bias_[1 as ::core::ffi::c_int as usize];
        (*m).zthresh_[i as usize] = (*m).zthresh_[1 as ::core::ffi::c_int as usize];
        i += 1;
    }
    sum = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        if type_0 == 0 as ::core::ffi::c_int {
            (*m).sharpen_[i as usize] = (kFreqSharpening[i as usize] as ::core::ffi::c_int
                * (*m).q_[i as usize] as ::core::ffi::c_int
                >> SHARPEN_BITS) as uint16_t;
        } else {
            (*m).sharpen_[i as usize] = 0 as uint16_t;
        }
        sum += (*m).q_[i as usize] as ::core::ffi::c_int;
        i += 1;
    }
    return sum + 8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int;
}
unsafe extern "C" fn CheckLambdaValue(v: *mut ::core::ffi::c_int) {
    if *v < 1 as ::core::ffi::c_int {
        *v = 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn SetupMatrices(mut enc: *mut VP8Encoder) {
    let mut i: ::core::ffi::c_int = 0;
    let tlambda_scale: ::core::ffi::c_int = if (*enc).method_ >= 4 as ::core::ffi::c_int {
        (*(*enc).config_).sns_strength
    } else {
        0 as ::core::ffi::c_int
    };
    let num_segments: ::core::ffi::c_int = (*enc).segment_hdr_.num_segments_;
    i = 0 as ::core::ffi::c_int;
    while i < num_segments {
        let m: *mut VP8SegmentInfo =
            (&raw mut (*enc).dqm_ as *mut VP8SegmentInfo).offset(i as isize) as *mut VP8SegmentInfo;
        let q: ::core::ffi::c_int = (*m).quant_;
        let mut q_i4: ::core::ffi::c_int = 0;
        let mut q_i16: ::core::ffi::c_int = 0;
        let mut q_uv: ::core::ffi::c_int = 0;
        (*m).y1_.q_[0 as ::core::ffi::c_int as usize] = kDcTable[clip(
            q + (*enc).dq_y1_dc_,
            0 as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
        ) as usize] as uint16_t;
        (*m).y1_.q_[1 as ::core::ffi::c_int as usize] =
            kAcTable[clip(q, 0 as ::core::ffi::c_int, 127 as ::core::ffi::c_int) as usize];
        (*m).y2_.q_[0 as ::core::ffi::c_int as usize] = (kDcTable[clip(
            q + (*enc).dq_y2_dc_,
            0 as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
        ) as usize] as ::core::ffi::c_int
            * 2 as ::core::ffi::c_int)
            as uint16_t;
        (*m).y2_.q_[1 as ::core::ffi::c_int as usize] = kAcTable2[clip(
            q + (*enc).dq_y2_ac_,
            0 as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
        ) as usize];
        (*m).uv_.q_[0 as ::core::ffi::c_int as usize] = kDcTable[clip(
            q + (*enc).dq_uv_dc_,
            0 as ::core::ffi::c_int,
            117 as ::core::ffi::c_int,
        ) as usize] as uint16_t;
        (*m).uv_.q_[1 as ::core::ffi::c_int as usize] = kAcTable[clip(
            q + (*enc).dq_uv_ac_,
            0 as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
        ) as usize];
        q_i4 = ExpandMatrix(&raw mut (*m).y1_, 0 as ::core::ffi::c_int);
        q_i16 = ExpandMatrix(&raw mut (*m).y2_, 1 as ::core::ffi::c_int);
        q_uv = ExpandMatrix(&raw mut (*m).uv_, 2 as ::core::ffi::c_int);
        (*m).lambda_i4_ = 3 as ::core::ffi::c_int * q_i4 * q_i4 >> 7 as ::core::ffi::c_int;
        (*m).lambda_i16_ = 3 as ::core::ffi::c_int * q_i16 * q_i16;
        (*m).lambda_uv_ = 3 as ::core::ffi::c_int * q_uv * q_uv >> 6 as ::core::ffi::c_int;
        (*m).lambda_mode_ = 1 as ::core::ffi::c_int * q_i4 * q_i4 >> 7 as ::core::ffi::c_int;
        (*m).lambda_trellis_i4_ = 7 as ::core::ffi::c_int * q_i4 * q_i4 >> 3 as ::core::ffi::c_int;
        (*m).lambda_trellis_i16_ = q_i16 * q_i16 >> 2 as ::core::ffi::c_int;
        (*m).lambda_trellis_uv_ = q_uv * q_uv << 1 as ::core::ffi::c_int;
        (*m).tlambda_ = tlambda_scale * q_i4 >> 5 as ::core::ffi::c_int;
        CheckLambdaValue(&raw mut (*m).lambda_i4_);
        CheckLambdaValue(&raw mut (*m).lambda_i16_);
        CheckLambdaValue(&raw mut (*m).lambda_uv_);
        CheckLambdaValue(&raw mut (*m).lambda_mode_);
        CheckLambdaValue(&raw mut (*m).lambda_trellis_i4_);
        CheckLambdaValue(&raw mut (*m).lambda_trellis_i16_);
        CheckLambdaValue(&raw mut (*m).lambda_trellis_uv_);
        CheckLambdaValue(&raw mut (*m).tlambda_);
        (*m).min_disto_ = 20 as ::core::ffi::c_int
            * (*m).y1_.q_[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        (*m).max_edge_ = 0 as ::core::ffi::c_int;
        (*m).i4_penalty_ = (1000 as ::core::ffi::c_int * q_i4 * q_i4) as score_t;
        i += 1;
    }
}
pub const FSTRENGTH_CUTOFF: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn SetupFilterStrength(enc: *mut VP8Encoder) {
    let mut i: ::core::ffi::c_int = 0;
    let level0: ::core::ffi::c_int = 5 as ::core::ffi::c_int * (*(*enc).config_).filter_strength;
    i = 0 as ::core::ffi::c_int;
    while i < NUM_MB_SEGMENTS as ::core::ffi::c_int {
        let m: *mut VP8SegmentInfo =
            (&raw mut (*enc).dqm_ as *mut VP8SegmentInfo).offset(i as isize) as *mut VP8SegmentInfo;
        let qstep: ::core::ffi::c_int = kAcTable[clip(
            (*m).quant_,
            0 as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
        ) as usize] as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int;
        let base_strength: ::core::ffi::c_int =
            VP8FilterStrengthFromDelta((*enc).filter_hdr_.sharpness_, qstep) as ::core::ffi::c_int;
        let f: ::core::ffi::c_int =
            base_strength * level0 / (256 as ::core::ffi::c_int + (*m).beta_);
        (*m).fstrength_ = if f < FSTRENGTH_CUTOFF {
            0 as ::core::ffi::c_int
        } else if f > 63 as ::core::ffi::c_int {
            63 as ::core::ffi::c_int
        } else {
            f
        };
        i += 1;
    }
    (*enc).filter_hdr_.level_ = (*enc).dqm_[0 as ::core::ffi::c_int as usize].fstrength_;
    (*enc).filter_hdr_.simple_ =
        ((*(*enc).config_).filter_type == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*enc).filter_hdr_.sharpness_ = (*(*enc).config_).filter_sharpness;
}
pub const MAX_DQ_UV: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MIN_DQ_UV: ::core::ffi::c_int = -(4 as ::core::ffi::c_int);
unsafe extern "C" fn QualityToCompression(mut c: ::core::ffi::c_double) -> ::core::ffi::c_double {
    let linear_c: ::core::ffi::c_double = if c < 0.75f64 {
        c * (2.0f64 / 3.0f64)
    } else {
        2.0f64 * c - 1.0f64
    };
    let v: ::core::ffi::c_double = pow(
        linear_c,
        1 as ::core::ffi::c_int as ::core::ffi::c_double / 3.0f64,
    ) as ::core::ffi::c_double;
    return v;
}
unsafe extern "C" fn QualityToJPEGCompression(
    mut c: ::core::ffi::c_double,
    mut alpha: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let amin: ::core::ffi::c_double = 0.30f64;
    let amax: ::core::ffi::c_double = 0.85f64;
    let exp_min: ::core::ffi::c_double = 0.4f64;
    let exp_max: ::core::ffi::c_double = 0.9f64;
    let slope: ::core::ffi::c_double = (exp_min - exp_max) / (amax - amin);
    let expn: ::core::ffi::c_double = if alpha > amax {
        exp_min
    } else if alpha < amin {
        exp_max
    } else {
        exp_max + slope * (alpha - amin)
    };
    let v: ::core::ffi::c_double = pow(c, expn) as ::core::ffi::c_double;
    return v;
}
unsafe extern "C" fn SegmentsAreEquivalent(
    S1: *const VP8SegmentInfo,
    S2: *const VP8SegmentInfo,
) -> ::core::ffi::c_int {
    return ((*S1).quant_ == (*S2).quant_ && (*S1).fstrength_ == (*S2).fstrength_)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn SimplifySegments(enc: *mut VP8Encoder) {
    let mut map: [::core::ffi::c_int; 4] = [
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let num_segments: ::core::ffi::c_int =
        if (*enc).segment_hdr_.num_segments_ < NUM_MB_SEGMENTS as ::core::ffi::c_int {
            (*enc).segment_hdr_.num_segments_
        } else {
            NUM_MB_SEGMENTS as ::core::ffi::c_int as ::core::ffi::c_int
        };
    let mut num_final_segments: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut s1: ::core::ffi::c_int = 0;
    let mut s2: ::core::ffi::c_int = 0;
    s1 = 1 as ::core::ffi::c_int;
    while s1 < num_segments {
        let S1: *const VP8SegmentInfo = (&raw mut (*enc).dqm_ as *mut VP8SegmentInfo)
            .offset(s1 as isize) as *mut VP8SegmentInfo;
        let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        s2 = 0 as ::core::ffi::c_int;
        while s2 < num_final_segments {
            let S2: *const VP8SegmentInfo = (&raw mut (*enc).dqm_ as *mut VP8SegmentInfo)
                .offset(s2 as isize)
                as *mut VP8SegmentInfo;
            if SegmentsAreEquivalent(S1, S2) != 0 {
                found = 1 as ::core::ffi::c_int;
                break;
            } else {
                s2 += 1;
            }
        }
        map[s1 as usize] = s2;
        if found == 0 {
            if num_final_segments != s1 {
                (*enc).dqm_[num_final_segments as usize] = (*enc).dqm_[s1 as usize];
            }
            num_final_segments += 1;
        }
        s1 += 1;
    }
    if num_final_segments < num_segments {
        let mut i: ::core::ffi::c_int = (*enc).mb_w_ * (*enc).mb_h_;
        loop {
            let fresh1 = i;
            i = i - 1;
            if !(fresh1 > 0 as ::core::ffi::c_int) {
                break;
            }
            let ref mut fresh2 = *(*enc).mb_info_.offset(i as isize);
            (*fresh2).set_segment_(
                map[(*(*enc).mb_info_.offset(i as isize)).segment_() as usize]
                    as ::core::ffi::c_uint as ::core::ffi::c_uint,
            );
        }
        (*enc).segment_hdr_.num_segments_ = num_final_segments;
        i = num_final_segments;
        while i < num_segments {
            (*enc).dqm_[i as usize] =
                (*enc).dqm_[(num_final_segments - 1 as ::core::ffi::c_int) as usize];
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8SetSegmentParams(
    enc: *mut VP8Encoder,
    mut quality: ::core::ffi::c_float,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut dq_uv_ac: ::core::ffi::c_int = 0;
    let mut dq_uv_dc: ::core::ffi::c_int = 0;
    let num_segments: ::core::ffi::c_int = (*enc).segment_hdr_.num_segments_;
    let amp: ::core::ffi::c_double =
        SNS_TO_DQ * (*(*enc).config_).sns_strength as ::core::ffi::c_double / 100.0f64 / 128.0f64;
    let Q: ::core::ffi::c_double = quality as ::core::ffi::c_double / 100.0f64;
    let c_base: ::core::ffi::c_double = if (*(*enc).config_).emulate_jpeg_size != 0 {
        QualityToJPEGCompression(Q, (*enc).alpha_ as ::core::ffi::c_double / 255.0f64)
            as ::core::ffi::c_double
    } else {
        QualityToCompression(Q) as ::core::ffi::c_double
    };
    i = 0 as ::core::ffi::c_int;
    while i < num_segments {
        let expn: ::core::ffi::c_double =
            1.0f64 - amp * (*enc).dqm_[i as usize].alpha_ as ::core::ffi::c_double;
        let c: ::core::ffi::c_double = pow(c_base, expn) as ::core::ffi::c_double;
        let q: ::core::ffi::c_int = (127.0f64 * (1.0f64 - c)) as ::core::ffi::c_int;
        '_c2rust_label: {
            if expn > 0.0f64 {
            } else {
                __assert_fail(
                    b"expn > 0.\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/quant_enc.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    417 as ::core::ffi::c_uint,
                    b"void VP8SetSegmentParams(VP8Encoder *const, float)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        (*enc).dqm_[i as usize].quant_ =
            clip(q, 0 as ::core::ffi::c_int, 127 as ::core::ffi::c_int);
        i += 1;
    }
    (*enc).base_quant_ = (*enc).dqm_[0 as ::core::ffi::c_int as usize].quant_;
    i = num_segments;
    while i < NUM_MB_SEGMENTS as ::core::ffi::c_int {
        (*enc).dqm_[i as usize].quant_ = (*enc).base_quant_;
        i += 1;
    }
    dq_uv_ac = ((*enc).uv_alpha_ - MID_ALPHA) * (MAX_DQ_UV - MIN_DQ_UV) / (MAX_ALPHA - MIN_ALPHA);
    dq_uv_ac = dq_uv_ac * (*(*enc).config_).sns_strength / 100 as ::core::ffi::c_int;
    dq_uv_ac = clip(dq_uv_ac, MIN_DQ_UV, MAX_DQ_UV);
    dq_uv_dc =
        -(4 as ::core::ffi::c_int) * (*(*enc).config_).sns_strength / 100 as ::core::ffi::c_int;
    dq_uv_dc = clip(
        dq_uv_dc,
        -(15 as ::core::ffi::c_int),
        15 as ::core::ffi::c_int,
    );
    (*enc).dq_y1_dc_ = 0 as ::core::ffi::c_int;
    (*enc).dq_y2_dc_ = 0 as ::core::ffi::c_int;
    (*enc).dq_y2_ac_ = 0 as ::core::ffi::c_int;
    (*enc).dq_uv_dc_ = dq_uv_dc;
    (*enc).dq_uv_ac_ = dq_uv_ac;
    SetupFilterStrength(enc);
    if num_segments > 1 as ::core::ffi::c_int {
        SimplifySegments(enc);
    }
    SetupMatrices(enc);
}
#[no_mangle]
pub static mut VP8I16ModeOffsets: [uint16_t; 4] = [
    I16DC16 as uint16_t,
    I16TM16 as uint16_t,
    I16VE16 as uint16_t,
    I16HE16 as uint16_t,
];
#[no_mangle]
pub static mut VP8UVModeOffsets: [uint16_t; 4] = [
    C8DC8 as uint16_t,
    C8TM8 as uint16_t,
    C8VE8 as uint16_t,
    C8HE8 as uint16_t,
];
#[no_mangle]
pub static mut VP8I4ModeOffsets: [uint16_t; 10] = [
    I4DC4 as uint16_t,
    I4TM4 as uint16_t,
    I4VE4 as uint16_t,
    I4HE4 as uint16_t,
    I4RD4 as uint16_t,
    I4VR4 as uint16_t,
    I4LD4 as uint16_t,
    I4VL4 as uint16_t,
    I4HD4 as uint16_t,
    I4HU4 as uint16_t,
];
#[no_mangle]
pub unsafe extern "C" fn VP8MakeLuma16Preds(it: *const VP8EncIterator) {
    let left: *const uint8_t = if (*it).x_ != 0 {
        (*it).y_left_
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    let top: *const uint8_t = if (*it).y_ != 0 {
        (*it).y_top_
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    VP8EncPredLuma16.expect("non-null function pointer")((*it).yuv_p_, left, top);
}
#[no_mangle]
pub unsafe extern "C" fn VP8MakeChroma8Preds(it: *const VP8EncIterator) {
    let left: *const uint8_t = if (*it).x_ != 0 {
        (*it).u_left_
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    let top: *const uint8_t = if (*it).y_ != 0 {
        (*it).uv_top_
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    VP8EncPredChroma8.expect("non-null function pointer")((*it).yuv_p_, left, top);
}
#[no_mangle]
pub unsafe extern "C" fn VP8MakeIntra4Preds(it: *const VP8EncIterator) {
    VP8EncPredLuma4.expect("non-null function pointer")((*it).yuv_p_, (*it).i4_top_);
}
#[no_mangle]
pub static mut VP8Scan: [uint16_t; 16] = [
    (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as uint16_t,
    (4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as uint16_t,
    (8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as uint16_t,
    (12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as uint16_t,
    (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * BPS) as uint16_t,
    (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * BPS) as uint16_t,
    (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * BPS) as uint16_t,
    (12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * BPS) as uint16_t,
    (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * BPS) as uint16_t,
    (4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * BPS) as uint16_t,
    (8 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * BPS) as uint16_t,
    (12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * BPS) as uint16_t,
    (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * BPS) as uint16_t,
    (4 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * BPS) as uint16_t,
    (8 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * BPS) as uint16_t,
    (12 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * BPS) as uint16_t,
];
static mut VP8ScanUV: [uint16_t; 8] = [
    (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as uint16_t,
    (4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as uint16_t,
    (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * BPS) as uint16_t,
    (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * BPS) as uint16_t,
    (8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as uint16_t,
    (12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as uint16_t,
    (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * BPS) as uint16_t,
    (12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * BPS) as uint16_t,
];
static mut kWeightY: [uint16_t; 16] = [
    38 as ::core::ffi::c_int as uint16_t,
    32 as ::core::ffi::c_int as uint16_t,
    20 as ::core::ffi::c_int as uint16_t,
    9 as ::core::ffi::c_int as uint16_t,
    32 as ::core::ffi::c_int as uint16_t,
    28 as ::core::ffi::c_int as uint16_t,
    17 as ::core::ffi::c_int as uint16_t,
    7 as ::core::ffi::c_int as uint16_t,
    20 as ::core::ffi::c_int as uint16_t,
    17 as ::core::ffi::c_int as uint16_t,
    10 as ::core::ffi::c_int as uint16_t,
    4 as ::core::ffi::c_int as uint16_t,
    9 as ::core::ffi::c_int as uint16_t,
    7 as ::core::ffi::c_int as uint16_t,
    4 as ::core::ffi::c_int as uint16_t,
    2 as ::core::ffi::c_int as uint16_t,
];
static mut kWeightTrellis: [uint16_t; 16] = [
    30 as ::core::ffi::c_int as uint16_t,
    27 as ::core::ffi::c_int as uint16_t,
    19 as ::core::ffi::c_int as uint16_t,
    11 as ::core::ffi::c_int as uint16_t,
    27 as ::core::ffi::c_int as uint16_t,
    24 as ::core::ffi::c_int as uint16_t,
    17 as ::core::ffi::c_int as uint16_t,
    10 as ::core::ffi::c_int as uint16_t,
    19 as ::core::ffi::c_int as uint16_t,
    17 as ::core::ffi::c_int as uint16_t,
    12 as ::core::ffi::c_int as uint16_t,
    8 as ::core::ffi::c_int as uint16_t,
    11 as ::core::ffi::c_int as uint16_t,
    10 as ::core::ffi::c_int as uint16_t,
    8 as ::core::ffi::c_int as uint16_t,
    6 as ::core::ffi::c_int as uint16_t,
];
unsafe extern "C" fn InitScore(rd: *mut VP8ModeScore) {
    (*rd).D = 0 as score_t;
    (*rd).SD = 0 as score_t;
    (*rd).R = 0 as score_t;
    (*rd).H = 0 as score_t;
    (*rd).nz = 0 as uint32_t;
    (*rd).score = MAX_COST;
}
unsafe extern "C" fn CopyScore(dst: *mut VP8ModeScore, src: *const VP8ModeScore) {
    (*dst).D = (*src).D;
    (*dst).SD = (*src).SD;
    (*dst).R = (*src).R;
    (*dst).H = (*src).H;
    (*dst).nz = (*src).nz;
    (*dst).score = (*src).score;
}
unsafe extern "C" fn AddScore(dst: *mut VP8ModeScore, src: *const VP8ModeScore) {
    (*dst).D += (*src).D;
    (*dst).SD += (*src).SD;
    (*dst).R += (*src).R;
    (*dst).H += (*src).H;
    (*dst).nz |= (*src).nz;
    (*dst).score += (*src).score;
}
pub const MIN_DELTA: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MAX_DELTA: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn SetRDScore(mut lambda: ::core::ffi::c_int, rd: *mut VP8ModeScore) {
    (*rd).score =
        ((*rd).R + (*rd).H) * lambda as score_t + RD_DISTO_MULT as score_t * ((*rd).D + (*rd).SD);
}
#[inline]
unsafe extern "C" fn RDScoreTrellis(
    mut lambda: ::core::ffi::c_int,
    mut rate: score_t,
    mut distortion: score_t,
) -> score_t {
    return rate * lambda as score_t + RD_DISTO_MULT as score_t * distortion;
}
unsafe extern "C" fn TrellisQuantizeBlock(
    enc: *const VP8Encoder,
    mut in_0: *mut int16_t,
    mut out: *mut int16_t,
    mut ctx0: ::core::ffi::c_int,
    mut coeff_type: ::core::ffi::c_int,
    mtx: *const VP8Matrix,
    mut lambda: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let probas: *const ProbaArray = &raw const *(&raw const (*enc).proba_.coeffs_
        as *const [ProbaArray; 8])
        .offset(coeff_type as isize) as *const ProbaArray;
    let costs: CostArrayPtr =
        &raw const *(&raw const (*enc).proba_.remapped_costs_ as *const CostArrayMap)
            .offset(coeff_type as isize) as *const [*const uint16_t; 3] as CostArrayPtr;
    let first: ::core::ffi::c_int = if coeff_type == TYPE_I16_AC as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    let mut nodes: [[Node; 2]; 16] = [[Node {
        prev: 0,
        sign: 0,
        level: 0,
    }; 2]; 16];
    let mut score_states: [[ScoreState; 2]; 2] = [[ScoreState {
        score: 0,
        costs: ::core::ptr::null::<uint16_t>(),
    }; 2]; 2];
    let mut ss_cur: *mut ScoreState =
        (&raw mut *(&raw mut score_states as *mut [ScoreState; 2])
            .offset(0 as ::core::ffi::c_int as isize) as *mut ScoreState)
            .offset((0 as ::core::ffi::c_int + MIN_DELTA) as isize) as *mut ScoreState;
    let mut ss_prev: *mut ScoreState =
        (&raw mut *(&raw mut score_states as *mut [ScoreState; 2])
            .offset(1 as ::core::ffi::c_int as isize) as *mut ScoreState)
            .offset((0 as ::core::ffi::c_int + MIN_DELTA) as isize) as *mut ScoreState;
    let mut best_path: [::core::ffi::c_int; 3] = [
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
    ];
    let mut best_score: score_t = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut last: ::core::ffi::c_int = 0;
    let mut cost: score_t = 0;
    let thresh: ::core::ffi::c_int = (*mtx).q_[1 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int
        * (*mtx).q_[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        / 4 as ::core::ffi::c_int;
    let last_proba: ::core::ffi::c_int = (*probas.offset(VP8EncBands[first as usize] as isize))
        [ctx0 as usize][0 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int;
    last = first - 1 as ::core::ffi::c_int;
    n = 15 as ::core::ffi::c_int;
    while n >= first {
        let j: ::core::ffi::c_int = kZigzag[n as usize] as ::core::ffi::c_int;
        let err: ::core::ffi::c_int = *in_0.offset(j as isize) as ::core::ffi::c_int
            * *in_0.offset(j as isize) as ::core::ffi::c_int;
        if err > thresh {
            last = n;
            break;
        } else {
            n -= 1;
        }
    }
    if last < 15 as ::core::ffi::c_int {
        last += 1;
    }
    cost = VP8BitCost(0 as ::core::ffi::c_int, last_proba as uint8_t) as score_t;
    best_score = RDScoreTrellis(lambda, cost, 0 as score_t);
    m = -MIN_DELTA;
    while m <= MAX_DELTA {
        let rate: score_t = (if ctx0 == 0 as ::core::ffi::c_int {
            VP8BitCost(1 as ::core::ffi::c_int, last_proba as uint8_t)
        } else {
            0 as ::core::ffi::c_int
        }) as score_t;
        (*ss_cur.offset(m as isize)).score = RDScoreTrellis(lambda, rate, 0 as score_t);
        let ref mut fresh3 = (*ss_cur.offset(m as isize)).costs;
        *fresh3 = (*costs.offset(first as isize))[ctx0 as usize];
        m += 1;
    }
    n = first;
    while n <= last {
        let j_0: ::core::ffi::c_int = kZigzag[n as usize] as ::core::ffi::c_int;
        let Q: uint32_t = (*mtx).q_[j_0 as usize] as uint32_t;
        let iQ: uint32_t = (*mtx).iq_[j_0 as usize] as uint32_t;
        let B: uint32_t = ((0 as ::core::ffi::c_int) << QFIX - 8 as ::core::ffi::c_int) as uint32_t;
        let sign: ::core::ffi::c_int = ((*in_0.offset(j_0 as isize) as ::core::ffi::c_int)
            < 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        let coeff0: uint32_t = ((if sign != 0 {
            -(*in_0.offset(j_0 as isize) as ::core::ffi::c_int)
        } else {
            *in_0.offset(j_0 as isize) as ::core::ffi::c_int
        }) + (*mtx).sharpen_[j_0 as usize] as ::core::ffi::c_int)
            as uint32_t;
        let mut level0: ::core::ffi::c_int = QUANTDIV(coeff0, iQ, B);
        let mut thresh_level: ::core::ffi::c_int = QUANTDIV(
            coeff0,
            iQ,
            ((0x80 as ::core::ffi::c_int) << QFIX - 8 as ::core::ffi::c_int) as uint32_t,
        );
        if thresh_level > MAX_LEVEL as ::core::ffi::c_int {
            thresh_level = MAX_LEVEL as ::core::ffi::c_int;
        }
        if level0 > MAX_LEVEL as ::core::ffi::c_int {
            level0 = MAX_LEVEL as ::core::ffi::c_int;
        }
        let tmp: *mut ScoreState = ss_cur;
        ss_cur = ss_prev;
        ss_prev = tmp;
        m = -MIN_DELTA;
        while m <= MAX_DELTA {
            let cur: *mut Node = (&raw mut *(&raw mut nodes as *mut [Node; 2]).offset(n as isize)
                as *mut Node)
                .offset((m + MIN_DELTA) as isize) as *mut Node;
            let level: ::core::ffi::c_int = level0 + m;
            let ctx: ::core::ffi::c_int = if level > 2 as ::core::ffi::c_int {
                2 as ::core::ffi::c_int
            } else {
                level
            };
            let band: ::core::ffi::c_int =
                VP8EncBands[(n + 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
            let mut base_score: score_t = 0;
            let mut best_cur_score: score_t = 0;
            let mut best_prev: ::core::ffi::c_int = 0;
            let mut cost_0: score_t = 0;
            let mut score: score_t = 0;
            let ref mut fresh4 = (*ss_cur.offset(m as isize)).costs;
            *fresh4 = (*costs.offset((n + 1 as ::core::ffi::c_int) as isize))[ctx as usize];
            if level < 0 as ::core::ffi::c_int || level > thresh_level {
                (*ss_cur.offset(m as isize)).score = MAX_COST;
            } else {
                let new_error: ::core::ffi::c_int =
                    coeff0.wrapping_sub((level as uint32_t).wrapping_mul(Q)) as ::core::ffi::c_int;
                let delta_error: ::core::ffi::c_int = (kWeightTrellis[j_0 as usize] as uint32_t)
                    .wrapping_mul(
                        ((new_error * new_error) as uint32_t)
                            .wrapping_sub(coeff0.wrapping_mul(coeff0)),
                    ) as ::core::ffi::c_int;
                base_score = RDScoreTrellis(lambda, 0 as score_t, delta_error as score_t);
                cost_0 =
                    VP8LevelCost((*ss_prev.offset(-MIN_DELTA as isize)).costs, level) as score_t;
                best_cur_score = (*ss_prev.offset(-MIN_DELTA as isize)).score
                    + RDScoreTrellis(lambda, cost_0, 0 as score_t);
                best_prev = -MIN_DELTA;
                p = -MIN_DELTA + 1 as ::core::ffi::c_int;
                while p <= MAX_DELTA {
                    cost_0 = VP8LevelCost((*ss_prev.offset(p as isize)).costs, level) as score_t;
                    score = (*ss_prev.offset(p as isize)).score
                        + RDScoreTrellis(lambda, cost_0, 0 as score_t);
                    if score < best_cur_score {
                        best_cur_score = score;
                        best_prev = p;
                    }
                    p += 1;
                }
                best_cur_score += base_score;
                (*cur).sign = sign as int8_t;
                (*cur).level = level as int16_t;
                (*cur).prev = best_prev as int8_t;
                (*ss_cur.offset(m as isize)).score = best_cur_score;
                if level != 0 as ::core::ffi::c_int && best_cur_score < best_score {
                    let last_pos_cost: score_t = (if n < 15 as ::core::ffi::c_int {
                        VP8BitCost(
                            0 as ::core::ffi::c_int,
                            (*probas.offset(band as isize))[ctx as usize]
                                [0 as ::core::ffi::c_int as usize],
                        )
                    } else {
                        0 as ::core::ffi::c_int
                    }) as score_t;
                    let last_pos_score: score_t =
                        RDScoreTrellis(lambda, last_pos_cost, 0 as score_t) as score_t;
                    score = best_cur_score + last_pos_score;
                    if score < best_score {
                        best_score = score;
                        best_path[0 as ::core::ffi::c_int as usize] = n;
                        best_path[1 as ::core::ffi::c_int as usize] = m;
                        best_path[2 as ::core::ffi::c_int as usize] = best_prev;
                    }
                }
            }
            m += 1;
        }
        n += 1;
    }
    if coeff_type == TYPE_I16_AC as ::core::ffi::c_int {
        memset(
            in_0.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (15 as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
        );
        memset(
            out.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (15 as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
        );
    } else {
        memset(
            in_0 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (16 as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
        );
        memset(
            out as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (16 as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
        );
    }
    if best_path[0 as ::core::ffi::c_int as usize] == -(1 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
    let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut best_node: ::core::ffi::c_int = best_path[1 as ::core::ffi::c_int as usize];
    n = best_path[0 as ::core::ffi::c_int as usize];
    nodes[n as usize][(best_node + MIN_DELTA) as usize].prev =
        best_path[2 as ::core::ffi::c_int as usize] as int8_t;
    while n >= first {
        let node: *const Node = (&raw mut *(&raw mut nodes as *mut [Node; 2]).offset(n as isize)
            as *mut Node)
            .offset((best_node + MIN_DELTA) as isize) as *mut Node;
        let j_1: ::core::ffi::c_int = kZigzag[n as usize] as ::core::ffi::c_int;
        *out.offset(n as isize) = (if (*node).sign as ::core::ffi::c_int != 0 {
            -((*node).level as ::core::ffi::c_int)
        } else {
            (*node).level as ::core::ffi::c_int
        }) as int16_t;
        nz |= (*node).level as ::core::ffi::c_int;
        *in_0.offset(j_1 as isize) = (*out.offset(n as isize) as ::core::ffi::c_int
            * (*mtx).q_[j_1 as usize] as ::core::ffi::c_int)
            as int16_t;
        best_node = (*node).prev as ::core::ffi::c_int;
        n -= 1;
    }
    return (nz != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn ReconstructIntra16(
    it: *mut VP8EncIterator,
    rd: *mut VP8ModeScore,
    yuv_out: *mut uint8_t,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let enc: *const VP8Encoder = (*it).enc_;
    let ref_0: *const uint8_t = (*it)
        .yuv_p_
        .offset(VP8I16ModeOffsets[mode as usize] as ::core::ffi::c_int as isize);
    let src: *const uint8_t = (*it).yuv_in_.offset(Y_OFF_ENC as isize);
    let dqm: *const VP8SegmentInfo = (&raw const (*enc).dqm_ as *const VP8SegmentInfo)
        .offset((*(*it).mb_).segment_() as isize)
        as *const VP8SegmentInfo;
    let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut n: ::core::ffi::c_int = 0;
    let mut tmp: [[int16_t; 16]; 16] = [[0; 16]; 16];
    let mut dc_tmp: [int16_t; 16] = [0; 16];
    n = 0 as ::core::ffi::c_int;
    while n < 16 as ::core::ffi::c_int {
        VP8FTransform2.expect("non-null function pointer")(
            src.offset(VP8Scan[n as usize] as ::core::ffi::c_int as isize),
            ref_0.offset(VP8Scan[n as usize] as ::core::ffi::c_int as isize),
            &raw mut *(&raw mut tmp as *mut [int16_t; 16]).offset(n as isize) as *mut int16_t,
        );
        n += 2 as ::core::ffi::c_int;
    }
    VP8FTransformWHT.expect("non-null function pointer")(
        &raw mut *(&raw mut tmp as *mut [int16_t; 16]).offset(0 as ::core::ffi::c_int as isize)
            as *mut int16_t,
        &raw mut dc_tmp as *mut int16_t,
    );
    nz |= VP8EncQuantizeBlockWHT.expect("non-null function pointer")(
        &raw mut dc_tmp as *mut int16_t,
        &raw mut (*rd).y_dc_levels as *mut int16_t,
        &raw const (*dqm).y2_,
    ) << 24 as ::core::ffi::c_int;
    if DO_TRELLIS_I16 != 0 && (*it).do_trellis_ != 0 {
        let mut x: ::core::ffi::c_int = 0;
        let mut y: ::core::ffi::c_int = 0;
        VP8IteratorNzToBytes(it);
        y = 0 as ::core::ffi::c_int;
        n = 0 as ::core::ffi::c_int;
        while y < 4 as ::core::ffi::c_int {
            x = 0 as ::core::ffi::c_int;
            while x < 4 as ::core::ffi::c_int {
                let ctx: ::core::ffi::c_int =
                    (*it).top_nz_[x as usize] + (*it).left_nz_[y as usize];
                let non_zero: ::core::ffi::c_int = TrellisQuantizeBlock(
                    enc,
                    &raw mut *(&raw mut tmp as *mut [int16_t; 16]).offset(n as isize)
                        as *mut int16_t,
                    &raw mut *(&raw mut (*rd).y_ac_levels as *mut [int16_t; 16]).offset(n as isize)
                        as *mut int16_t,
                    ctx,
                    TYPE_I16_AC as ::core::ffi::c_int,
                    &raw const (*dqm).y1_,
                    (*dqm).lambda_trellis_i16_,
                ) as ::core::ffi::c_int;
                (*it).left_nz_[y as usize] = non_zero;
                (*it).top_nz_[x as usize] = (*it).left_nz_[y as usize];
                (*rd).y_ac_levels[n as usize][0 as ::core::ffi::c_int as usize] = 0 as int16_t;
                nz |= non_zero << n;
                x += 1;
                n += 1;
            }
            y += 1;
        }
    } else {
        n = 0 as ::core::ffi::c_int;
        while n < 16 as ::core::ffi::c_int {
            tmp[(n + 1 as ::core::ffi::c_int) as usize][0 as ::core::ffi::c_int as usize] =
                0 as int16_t;
            tmp[n as usize][0 as ::core::ffi::c_int as usize] =
                tmp[(n + 1 as ::core::ffi::c_int) as usize][0 as ::core::ffi::c_int as usize];
            nz |= VP8EncQuantize2Blocks.expect("non-null function pointer")(
                &raw mut *(&raw mut tmp as *mut [int16_t; 16]).offset(n as isize) as *mut int16_t,
                &raw mut *(&raw mut (*rd).y_ac_levels as *mut [int16_t; 16]).offset(n as isize)
                    as *mut int16_t,
                &raw const (*dqm).y1_,
            ) << n;
            '_c2rust_label: {
                if (*rd).y_ac_levels[(n + 0 as ::core::ffi::c_int) as usize]
                    [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                } else {
                    __assert_fail(
                        b"rd->y_ac_levels[n + 0][0] == 0\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/quant_enc.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        810 as ::core::ffi::c_uint,
                        b"int ReconstructIntra16(VP8EncIterator *const restrict, VP8ModeScore *const restrict, uint8_t *const restrict, int)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            '_c2rust_label_0: {
                if (*rd).y_ac_levels[(n + 1 as ::core::ffi::c_int) as usize]
                    [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                } else {
                    __assert_fail(
                        b"rd->y_ac_levels[n + 1][0] == 0\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/quant_enc.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        811 as ::core::ffi::c_uint,
                        b"int ReconstructIntra16(VP8EncIterator *const restrict, VP8ModeScore *const restrict, uint8_t *const restrict, int)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            n += 2 as ::core::ffi::c_int;
        }
    }
    VP8TransformWHT.expect("non-null function pointer")(
        &raw mut dc_tmp as *mut int16_t,
        &raw mut *(&raw mut tmp as *mut [int16_t; 16]).offset(0 as ::core::ffi::c_int as isize)
            as *mut int16_t,
    );
    n = 0 as ::core::ffi::c_int;
    while n < 16 as ::core::ffi::c_int {
        VP8ITransform.expect("non-null function pointer")(
            ref_0.offset(VP8Scan[n as usize] as ::core::ffi::c_int as isize),
            &raw mut *(&raw mut tmp as *mut [int16_t; 16]).offset(n as isize) as *mut int16_t,
            yuv_out.offset(VP8Scan[n as usize] as ::core::ffi::c_int as isize),
            1 as ::core::ffi::c_int,
        );
        n += 2 as ::core::ffi::c_int;
    }
    return nz;
}
unsafe extern "C" fn ReconstructIntra4(
    it: *mut VP8EncIterator,
    mut levels: *mut int16_t,
    src: *const uint8_t,
    yuv_out: *mut uint8_t,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let enc: *const VP8Encoder = (*it).enc_;
    let ref_0: *const uint8_t = (*it)
        .yuv_p_
        .offset(VP8I4ModeOffsets[mode as usize] as ::core::ffi::c_int as isize);
    let dqm: *const VP8SegmentInfo = (&raw const (*enc).dqm_ as *const VP8SegmentInfo)
        .offset((*(*it).mb_).segment_() as isize)
        as *const VP8SegmentInfo;
    let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tmp: [int16_t; 16] = [0; 16];
    VP8FTransform.expect("non-null function pointer")(src, ref_0, &raw mut tmp as *mut int16_t);
    if DO_TRELLIS_I4 != 0 && (*it).do_trellis_ != 0 {
        let x: ::core::ffi::c_int = (*it).i4_ & 3 as ::core::ffi::c_int;
        let y: ::core::ffi::c_int = (*it).i4_ >> 2 as ::core::ffi::c_int;
        let ctx: ::core::ffi::c_int = (*it).top_nz_[x as usize] + (*it).left_nz_[y as usize];
        nz = TrellisQuantizeBlock(
            enc,
            &raw mut tmp as *mut int16_t,
            levels,
            ctx,
            TYPE_I4_AC as ::core::ffi::c_int,
            &raw const (*dqm).y1_,
            (*dqm).lambda_trellis_i4_,
        );
    } else {
        nz = VP8EncQuantizeBlock.expect("non-null function pointer")(
            &raw mut tmp as *mut int16_t,
            levels,
            &raw const (*dqm).y1_,
        );
    }
    VP8ITransform.expect("non-null function pointer")(
        ref_0,
        &raw mut tmp as *mut int16_t,
        yuv_out,
        0 as ::core::ffi::c_int,
    );
    return nz;
}
pub const C1: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const C2: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const DSHIFT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DSCALE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn QuantizeSingle(v: *mut int16_t, mtx: *const VP8Matrix) -> ::core::ffi::c_int {
    let mut V: ::core::ffi::c_int = *v as ::core::ffi::c_int;
    let sign: ::core::ffi::c_int = (V < 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if sign != 0 {
        V = -V;
    }
    if V > (*mtx).zthresh_[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int {
        let qV: ::core::ffi::c_int = QUANTDIV(
            V as uint32_t,
            (*mtx).iq_[0 as ::core::ffi::c_int as usize] as uint32_t,
            (*mtx).bias_[0 as ::core::ffi::c_int as usize],
        ) as ::core::ffi::c_int
            * (*mtx).q_[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        let err: ::core::ffi::c_int = V - qV;
        *v = (if sign != 0 { -qV } else { qV }) as int16_t;
        return (if sign != 0 { -err } else { err }) >> DSCALE;
    }
    *v = 0 as int16_t;
    return (if sign != 0 { -V } else { V }) >> DSCALE;
}
unsafe extern "C" fn CorrectDCValues(
    it: *const VP8EncIterator,
    mtx: *const VP8Matrix,
    mut tmp: *mut [int16_t; 16],
    rd: *mut VP8ModeScore,
) {
    let mut ch: ::core::ffi::c_int = 0;
    ch = 0 as ::core::ffi::c_int;
    while ch <= 1 as ::core::ffi::c_int {
        let top: *const int8_t = &raw mut *(&raw mut *(*it).top_derr_.offset((*it).x_ as isize)
            as *mut [int8_t; 2])
            .offset(ch as isize) as *mut int8_t;
        let left: *const int8_t = &raw const *(&raw const (*it).left_derr_ as *const [int8_t; 2])
            .offset(ch as isize) as *const int8_t;
        let c: *mut [int16_t; 16] =
            tmp.offset((ch * 4 as ::core::ffi::c_int) as isize) as *mut [int16_t; 16];
        let mut err0: ::core::ffi::c_int = 0;
        let mut err1: ::core::ffi::c_int = 0;
        let mut err2: ::core::ffi::c_int = 0;
        let mut err3: ::core::ffi::c_int = 0;
        let ref mut fresh5 =
            (*c.offset(0 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize];
        *fresh5 = (*fresh5 as ::core::ffi::c_int
            + (C1 * *top.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + C2 * *left.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >> DSHIFT - DSCALE)) as int16_t;
        err0 = QuantizeSingle(
            (&raw mut *c.offset(0 as ::core::ffi::c_int as isize) as *mut int16_t)
                .offset(0 as ::core::ffi::c_int as isize) as *mut int16_t,
            mtx,
        );
        let ref mut fresh6 =
            (*c.offset(1 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize];
        *fresh6 = (*fresh6 as ::core::ffi::c_int
            + (C1 * *top.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + C2 * err0
                >> DSHIFT - DSCALE)) as int16_t;
        err1 = QuantizeSingle(
            (&raw mut *c.offset(1 as ::core::ffi::c_int as isize) as *mut int16_t)
                .offset(0 as ::core::ffi::c_int as isize) as *mut int16_t,
            mtx,
        );
        let ref mut fresh7 =
            (*c.offset(2 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize];
        *fresh7 = (*fresh7 as ::core::ffi::c_int
            + (C1 * err0
                + C2 * *left.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >> DSHIFT - DSCALE)) as int16_t;
        err2 = QuantizeSingle(
            (&raw mut *c.offset(2 as ::core::ffi::c_int as isize) as *mut int16_t)
                .offset(0 as ::core::ffi::c_int as isize) as *mut int16_t,
            mtx,
        );
        let ref mut fresh8 =
            (*c.offset(3 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize];
        *fresh8 =
            (*fresh8 as ::core::ffi::c_int + (C1 * err1 + C2 * err2 >> DSHIFT - DSCALE)) as int16_t;
        err3 = QuantizeSingle(
            (&raw mut *c.offset(3 as ::core::ffi::c_int as isize) as *mut int16_t)
                .offset(0 as ::core::ffi::c_int as isize) as *mut int16_t,
            mtx,
        );
        '_c2rust_label: {
            if abs(err1) <= 127 as ::core::ffi::c_int
                && abs(err2) <= 127 as ::core::ffi::c_int
                && abs(err3) <= 127 as ::core::ffi::c_int
            {
            } else {
                __assert_fail(
                    b"abs(err1) <= 127 && abs(err2) <= 127 && abs(err3) <= 127\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/quant_enc.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    902 as ::core::ffi::c_uint,
                    b"void CorrectDCValues(const VP8EncIterator *const restrict, const VP8Matrix *const restrict, int16_t (*)[16], VP8ModeScore *const restrict)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        (*rd).derr[ch as usize][0 as ::core::ffi::c_int as usize] = err1 as int8_t;
        (*rd).derr[ch as usize][1 as ::core::ffi::c_int as usize] = err2 as int8_t;
        (*rd).derr[ch as usize][2 as ::core::ffi::c_int as usize] = err3 as int8_t;
        ch += 1;
    }
}
unsafe extern "C" fn StoreDiffusionErrors(it: *mut VP8EncIterator, rd: *const VP8ModeScore) {
    let mut ch: ::core::ffi::c_int = 0;
    ch = 0 as ::core::ffi::c_int;
    while ch <= 1 as ::core::ffi::c_int {
        let top: *mut int8_t = &raw mut *(&raw mut *(*it).top_derr_.offset((*it).x_ as isize)
            as *mut [int8_t; 2])
            .offset(ch as isize) as *mut int8_t;
        let left: *mut int8_t = &raw mut *(&raw mut (*it).left_derr_ as *mut [int8_t; 2])
            .offset(ch as isize) as *mut int8_t;
        *left.offset(0 as ::core::ffi::c_int as isize) =
            (*rd).derr[ch as usize][0 as ::core::ffi::c_int as usize];
        *left.offset(1 as ::core::ffi::c_int as isize) = (3 as ::core::ffi::c_int
            * (*rd).derr[ch as usize][2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as int8_t;
        *top.offset(0 as ::core::ffi::c_int as isize) =
            (*rd).derr[ch as usize][1 as ::core::ffi::c_int as usize];
        *top.offset(1 as ::core::ffi::c_int as isize) =
            ((*rd).derr[ch as usize][2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                - *left.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as int8_t;
        ch += 1;
    }
}
unsafe extern "C" fn ReconstructUV(
    it: *mut VP8EncIterator,
    rd: *mut VP8ModeScore,
    yuv_out: *mut uint8_t,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let enc: *const VP8Encoder = (*it).enc_;
    let ref_0: *const uint8_t = (*it)
        .yuv_p_
        .offset(VP8UVModeOffsets[mode as usize] as ::core::ffi::c_int as isize);
    let src: *const uint8_t = (*it).yuv_in_.offset(U_OFF_ENC as isize);
    let dqm: *const VP8SegmentInfo = (&raw const (*enc).dqm_ as *const VP8SegmentInfo)
        .offset((*(*it).mb_).segment_() as isize)
        as *const VP8SegmentInfo;
    let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut n: ::core::ffi::c_int = 0;
    let mut tmp: [[int16_t; 16]; 8] = [[0; 16]; 8];
    n = 0 as ::core::ffi::c_int;
    while n < 8 as ::core::ffi::c_int {
        VP8FTransform2.expect("non-null function pointer")(
            src.offset(VP8ScanUV[n as usize] as ::core::ffi::c_int as isize),
            ref_0.offset(VP8ScanUV[n as usize] as ::core::ffi::c_int as isize),
            &raw mut *(&raw mut tmp as *mut [int16_t; 16]).offset(n as isize) as *mut int16_t,
        );
        n += 2 as ::core::ffi::c_int;
    }
    if !(*it).top_derr_.is_null() {
        CorrectDCValues(
            it,
            &raw const (*dqm).uv_,
            &raw mut tmp as *mut [int16_t; 16],
            rd,
        );
    }
    if DO_TRELLIS_UV != 0 && (*it).do_trellis_ != 0 {
        let mut ch: ::core::ffi::c_int = 0;
        let mut x: ::core::ffi::c_int = 0;
        let mut y: ::core::ffi::c_int = 0;
        ch = 0 as ::core::ffi::c_int;
        n = 0 as ::core::ffi::c_int;
        while ch <= 2 as ::core::ffi::c_int {
            y = 0 as ::core::ffi::c_int;
            while y < 2 as ::core::ffi::c_int {
                x = 0 as ::core::ffi::c_int;
                while x < 2 as ::core::ffi::c_int {
                    let ctx: ::core::ffi::c_int = (*it).top_nz_
                        [(4 as ::core::ffi::c_int + ch + x) as usize]
                        + (*it).left_nz_[(4 as ::core::ffi::c_int + ch + y) as usize];
                    let non_zero: ::core::ffi::c_int = TrellisQuantizeBlock(
                        enc,
                        &raw mut *(&raw mut tmp as *mut [int16_t; 16]).offset(n as isize)
                            as *mut int16_t,
                        &raw mut *(&raw mut (*rd).uv_levels as *mut [int16_t; 16])
                            .offset(n as isize) as *mut int16_t,
                        ctx,
                        TYPE_CHROMA_A as ::core::ffi::c_int,
                        &raw const (*dqm).uv_,
                        (*dqm).lambda_trellis_uv_,
                    ) as ::core::ffi::c_int;
                    (*it).left_nz_[(4 as ::core::ffi::c_int + ch + y) as usize] = non_zero;
                    (*it).top_nz_[(4 as ::core::ffi::c_int + ch + x) as usize] =
                        (*it).left_nz_[(4 as ::core::ffi::c_int + ch + y) as usize];
                    nz |= non_zero << n;
                    x += 1;
                    n += 1;
                }
                y += 1;
            }
            ch += 2 as ::core::ffi::c_int;
        }
    } else {
        n = 0 as ::core::ffi::c_int;
        while n < 8 as ::core::ffi::c_int {
            nz |= VP8EncQuantize2Blocks.expect("non-null function pointer")(
                &raw mut *(&raw mut tmp as *mut [int16_t; 16]).offset(n as isize) as *mut int16_t,
                &raw mut *(&raw mut (*rd).uv_levels as *mut [int16_t; 16]).offset(n as isize)
                    as *mut int16_t,
                &raw const (*dqm).uv_,
            ) << n;
            n += 2 as ::core::ffi::c_int;
        }
    }
    n = 0 as ::core::ffi::c_int;
    while n < 8 as ::core::ffi::c_int {
        VP8ITransform.expect("non-null function pointer")(
            ref_0.offset(VP8ScanUV[n as usize] as ::core::ffi::c_int as isize),
            &raw mut *(&raw mut tmp as *mut [int16_t; 16]).offset(n as isize) as *mut int16_t,
            yuv_out.offset(VP8ScanUV[n as usize] as ::core::ffi::c_int as isize),
            1 as ::core::ffi::c_int,
        );
        n += 2 as ::core::ffi::c_int;
    }
    return nz << 16 as ::core::ffi::c_int;
}
unsafe extern "C" fn StoreMaxDelta(dqm: *mut VP8SegmentInfo, mut DCs: *const int16_t) {
    let v0: ::core::ffi::c_int =
        abs(*DCs.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    let v1: ::core::ffi::c_int =
        abs(*DCs.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    let v2: ::core::ffi::c_int =
        abs(*DCs.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    let mut max_v: ::core::ffi::c_int = if v1 > v0 { v1 } else { v0 };
    max_v = if v2 > max_v { v2 } else { max_v };
    if max_v > (*dqm).max_edge_ {
        (*dqm).max_edge_ = max_v;
    }
}
unsafe extern "C" fn SwapModeScore(mut a: *mut *mut VP8ModeScore, mut b: *mut *mut VP8ModeScore) {
    let tmp: *mut VP8ModeScore = *a;
    *a = *b;
    *b = tmp;
}
unsafe extern "C" fn SwapPtr(mut a: *mut *mut uint8_t, mut b: *mut *mut uint8_t) {
    let tmp: *mut uint8_t = *a;
    *a = *b;
    *b = tmp;
}
unsafe extern "C" fn SwapOut(it: *mut VP8EncIterator) {
    SwapPtr(&raw mut (*it).yuv_out_, &raw mut (*it).yuv_out2_);
}
unsafe extern "C" fn PickBestIntra16(it: *mut VP8EncIterator, mut rd: *mut VP8ModeScore) {
    let kNumBlocks: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    let dqm: *mut VP8SegmentInfo = (&raw mut (*(*it).enc_).dqm_ as *mut VP8SegmentInfo)
        .offset((*(*it).mb_).segment_() as isize)
        as *mut VP8SegmentInfo;
    let lambda: ::core::ffi::c_int = (*dqm).lambda_i16_;
    let tlambda: ::core::ffi::c_int = (*dqm).tlambda_;
    let src: *const uint8_t = (*it).yuv_in_.offset(Y_OFF_ENC as isize);
    let mut rd_tmp: VP8ModeScore = VP8ModeScore {
        D: 0,
        SD: 0,
        H: 0,
        R: 0,
        score: 0,
        y_dc_levels: [0; 16],
        y_ac_levels: [[0; 16]; 16],
        uv_levels: [[0; 16]; 8],
        mode_i16: 0,
        modes_i4: [0; 16],
        mode_uv: 0,
        nz: 0,
        derr: [[0; 3]; 2],
    };
    let mut rd_cur: *mut VP8ModeScore = &raw mut rd_tmp;
    let mut rd_best: *mut VP8ModeScore = rd;
    let mut mode: ::core::ffi::c_int = 0;
    let mut is_flat: ::core::ffi::c_int = IsFlatSource16((*it).yuv_in_.offset(Y_OFF_ENC as isize));
    (*rd).mode_i16 = -(1 as ::core::ffi::c_int);
    mode = 0 as ::core::ffi::c_int;
    while mode < NUM_PRED_MODES as ::core::ffi::c_int {
        let tmp_dst: *mut uint8_t = (*it).yuv_out2_.offset(Y_OFF_ENC as isize);
        (*rd_cur).mode_i16 = mode;
        (*rd_cur).nz = ReconstructIntra16(it, rd_cur, tmp_dst, mode) as uint32_t;
        (*rd_cur).D = VP8SSE16x16.expect("non-null function pointer")(src, tmp_dst) as score_t;
        (*rd_cur).SD = (if tlambda != 0 {
            tlambda
                * VP8TDisto16x16.expect("non-null function pointer")(
                    src,
                    tmp_dst,
                    &raw const kWeightY as *const uint16_t,
                )
                + 128 as ::core::ffi::c_int
                >> 8 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as score_t;
        (*rd_cur).H = VP8FixedCostsI16[mode as usize] as score_t;
        (*rd_cur).R = VP8GetCostLuma16(it, rd_cur) as score_t;
        if is_flat != 0 {
            is_flat = IsFlat_C(
                &raw mut *(&raw mut (*rd_cur).y_ac_levels as *mut [int16_t; 16])
                    .offset(0 as ::core::ffi::c_int as isize) as *mut int16_t,
                kNumBlocks,
                FLATNESS_LIMIT_I16,
            );
            if is_flat != 0 {
                (*rd_cur).D *= 2 as score_t;
                (*rd_cur).SD *= 2 as score_t;
            }
        }
        SetRDScore(lambda, rd_cur);
        if mode == 0 as ::core::ffi::c_int || (*rd_cur).score < (*rd_best).score {
            SwapModeScore(&raw mut rd_cur, &raw mut rd_best);
            SwapOut(it);
        }
        mode += 1;
    }
    if rd_best != rd {
        memcpy(
            rd as *mut ::core::ffi::c_void,
            rd_best as *const ::core::ffi::c_void,
            ::core::mem::size_of::<VP8ModeScore>() as size_t,
        );
    }
    SetRDScore((*dqm).lambda_mode_, rd);
    VP8SetIntra16Mode(it, (*rd).mode_i16);
    if (*rd).nz & 0x100ffff as uint32_t == 0x1000000 as uint32_t
        && (*rd).D > (*dqm).min_disto_ as score_t
    {
        StoreMaxDelta(
            dqm,
            &raw mut (*rd).y_dc_levels as *mut int16_t as *const int16_t,
        );
    }
}
unsafe extern "C" fn GetCostModeI4(
    it: *mut VP8EncIterator,
    mut modes: *const uint8_t,
) -> *const uint16_t {
    let preds_w: ::core::ffi::c_int = (*(*it).enc_).preds_w_;
    let x: ::core::ffi::c_int = (*it).i4_ & 3 as ::core::ffi::c_int;
    let y: ::core::ffi::c_int = (*it).i4_ >> 2 as ::core::ffi::c_int;
    let left: ::core::ffi::c_int = if x == 0 as ::core::ffi::c_int {
        *(*it)
            .preds_
            .offset((y * preds_w - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
    } else {
        *modes.offset(((*it).i4_ - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
    };
    let top: ::core::ffi::c_int = if y == 0 as ::core::ffi::c_int {
        *(*it).preds_.offset((-preds_w + x) as isize) as ::core::ffi::c_int
    } else {
        *modes.offset(((*it).i4_ - 4 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
    };
    return &raw const *(&raw const *(&raw const VP8FixedCostsI4 as *const [[uint16_t; 10]; 10])
        .offset(top as isize) as *const [uint16_t; 10])
        .offset(left as isize) as *const uint16_t;
}
unsafe extern "C" fn PickBestIntra4(
    it: *mut VP8EncIterator,
    rd: *mut VP8ModeScore,
) -> ::core::ffi::c_int {
    let enc: *const VP8Encoder = (*it).enc_;
    let dqm: *const VP8SegmentInfo = (&raw const (*enc).dqm_ as *const VP8SegmentInfo)
        .offset((*(*it).mb_).segment_() as isize)
        as *const VP8SegmentInfo;
    let lambda: ::core::ffi::c_int = (*dqm).lambda_i4_;
    let tlambda: ::core::ffi::c_int = (*dqm).tlambda_;
    let src0: *const uint8_t = (*it).yuv_in_.offset(Y_OFF_ENC as isize);
    let best_blocks: *mut uint8_t = (*it).yuv_out2_.offset(Y_OFF_ENC as isize);
    let mut total_header_bits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rd_best: VP8ModeScore = VP8ModeScore {
        D: 0,
        SD: 0,
        H: 0,
        R: 0,
        score: 0,
        y_dc_levels: [0; 16],
        y_ac_levels: [[0; 16]; 16],
        uv_levels: [[0; 16]; 8],
        mode_i16: 0,
        modes_i4: [0; 16],
        mode_uv: 0,
        nz: 0,
        derr: [[0; 3]; 2],
    };
    if (*enc).max_i4_header_bits_ == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    InitScore(&raw mut rd_best);
    rd_best.H = 211 as score_t;
    SetRDScore((*dqm).lambda_mode_, &raw mut rd_best);
    VP8IteratorStartI4(it);
    loop {
        let kNumBlocks: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut rd_i4: VP8ModeScore = VP8ModeScore {
            D: 0,
            SD: 0,
            H: 0,
            R: 0,
            score: 0,
            y_dc_levels: [0; 16],
            y_ac_levels: [[0; 16]; 16],
            uv_levels: [[0; 16]; 8],
            mode_i16: 0,
            modes_i4: [0; 16],
            mode_uv: 0,
            nz: 0,
            derr: [[0; 3]; 2],
        };
        let mut mode: ::core::ffi::c_int = 0;
        let mut best_mode: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let src: *const uint8_t =
            src0.offset(VP8Scan[(*it).i4_ as usize] as ::core::ffi::c_int as isize);
        let mode_costs: *const uint16_t = GetCostModeI4(
            it,
            &raw mut (*rd).modes_i4 as *mut uint8_t as *const uint8_t,
        ) as *const uint16_t;
        let mut best_block: *mut uint8_t =
            best_blocks.offset(VP8Scan[(*it).i4_ as usize] as ::core::ffi::c_int as isize);
        let mut tmp_dst: *mut uint8_t = (*it).yuv_p_.offset(I4TMP as isize);
        InitScore(&raw mut rd_i4);
        VP8MakeIntra4Preds(it);
        mode = 0 as ::core::ffi::c_int;
        while mode < NUM_BMODES as ::core::ffi::c_int {
            let mut rd_tmp: VP8ModeScore = VP8ModeScore {
                D: 0,
                SD: 0,
                H: 0,
                R: 0,
                score: 0,
                y_dc_levels: [0; 16],
                y_ac_levels: [[0; 16]; 16],
                uv_levels: [[0; 16]; 8],
                mode_i16: 0,
                modes_i4: [0; 16],
                mode_uv: 0,
                nz: 0,
                derr: [[0; 3]; 2],
            };
            let mut tmp_levels: [int16_t; 16] = [0; 16];
            rd_tmp.nz =
                (ReconstructIntra4(it, &raw mut tmp_levels as *mut int16_t, src, tmp_dst, mode)
                    << (*it).i4_) as uint32_t;
            rd_tmp.D = VP8SSE4x4.expect("non-null function pointer")(src, tmp_dst) as score_t;
            rd_tmp.SD = (if tlambda != 0 {
                tlambda
                    * VP8TDisto4x4.expect("non-null function pointer")(
                        src,
                        tmp_dst,
                        &raw const kWeightY as *const uint16_t,
                    )
                    + 128 as ::core::ffi::c_int
                    >> 8 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as score_t;
            rd_tmp.H = *mode_costs.offset(mode as isize) as score_t;
            if mode > 0 as ::core::ffi::c_int
                && IsFlat_C(
                    &raw mut tmp_levels as *mut int16_t,
                    kNumBlocks,
                    FLATNESS_LIMIT_I4,
                ) != 0
            {
                rd_tmp.R = (FLATNESS_PENALTY * kNumBlocks) as score_t;
            } else {
                rd_tmp.R = 0 as score_t;
            }
            SetRDScore(lambda, &raw mut rd_tmp);
            if !(best_mode >= 0 as ::core::ffi::c_int && rd_tmp.score >= rd_i4.score) {
                rd_tmp.R +=
                    VP8GetCostLuma4(it, &raw mut tmp_levels as *mut int16_t as *const int16_t)
                        as score_t;
                SetRDScore(lambda, &raw mut rd_tmp);
                if best_mode < 0 as ::core::ffi::c_int || rd_tmp.score < rd_i4.score {
                    CopyScore(&raw mut rd_i4, &raw mut rd_tmp);
                    best_mode = mode;
                    SwapPtr(&raw mut tmp_dst, &raw mut best_block);
                    memcpy(
                        &raw mut *(&raw mut rd_best.y_ac_levels as *mut [int16_t; 16])
                            .offset((*it).i4_ as isize) as *mut int16_t
                            as *mut ::core::ffi::c_void,
                        &raw mut tmp_levels as *mut int16_t as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<[int16_t; 16]>() as size_t,
                    );
                }
            }
            mode += 1;
        }
        SetRDScore((*dqm).lambda_mode_, &raw mut rd_i4);
        AddScore(&raw mut rd_best, &raw mut rd_i4);
        if rd_best.score >= (*rd).score {
            return 0 as ::core::ffi::c_int;
        }
        total_header_bits += rd_i4.H as ::core::ffi::c_int;
        if total_header_bits > (*enc).max_i4_header_bits_ {
            return 0 as ::core::ffi::c_int;
        }
        if best_block
            != best_blocks.offset(VP8Scan[(*it).i4_ as usize] as ::core::ffi::c_int as isize)
        {
            VP8Copy4x4.expect("non-null function pointer")(
                best_block,
                best_blocks.offset(VP8Scan[(*it).i4_ as usize] as ::core::ffi::c_int as isize),
            );
        }
        (*rd).modes_i4[(*it).i4_ as usize] = best_mode as uint8_t;
        (*it).left_nz_[((*it).i4_ >> 2 as ::core::ffi::c_int) as usize] = if rd_i4.nz != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        (*it).top_nz_[((*it).i4_ & 3 as ::core::ffi::c_int) as usize] =
            (*it).left_nz_[((*it).i4_ >> 2 as ::core::ffi::c_int) as usize];
        if !(VP8IteratorRotateI4(it, best_blocks) != 0) {
            break;
        }
    }
    CopyScore(rd, &raw mut rd_best);
    VP8SetIntra4Mode(it, &raw mut (*rd).modes_i4 as *mut uint8_t);
    SwapOut(it);
    memcpy(
        &raw mut (*rd).y_ac_levels as *mut [int16_t; 16] as *mut ::core::ffi::c_void,
        &raw mut rd_best.y_ac_levels as *mut [int16_t; 16] as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[[int16_t; 16]; 16]>() as size_t,
    );
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn PickBestUV(it: *mut VP8EncIterator, rd: *mut VP8ModeScore) {
    let kNumBlocks: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let dqm: *const VP8SegmentInfo = (&raw mut (*(*it).enc_).dqm_ as *mut VP8SegmentInfo)
        .offset((*(*it).mb_).segment_() as isize)
        as *mut VP8SegmentInfo;
    let lambda: ::core::ffi::c_int = (*dqm).lambda_uv_;
    let src: *const uint8_t = (*it).yuv_in_.offset(U_OFF_ENC as isize);
    let mut tmp_dst: *mut uint8_t = (*it).yuv_out2_.offset(U_OFF_ENC as isize);
    let mut dst0: *mut uint8_t = (*it).yuv_out_.offset(U_OFF_ENC as isize);
    let mut dst: *mut uint8_t = dst0;
    let mut rd_best: VP8ModeScore = VP8ModeScore {
        D: 0,
        SD: 0,
        H: 0,
        R: 0,
        score: 0,
        y_dc_levels: [0; 16],
        y_ac_levels: [[0; 16]; 16],
        uv_levels: [[0; 16]; 8],
        mode_i16: 0,
        modes_i4: [0; 16],
        mode_uv: 0,
        nz: 0,
        derr: [[0; 3]; 2],
    };
    let mut mode: ::core::ffi::c_int = 0;
    (*rd).mode_uv = -(1 as ::core::ffi::c_int);
    InitScore(&raw mut rd_best);
    mode = 0 as ::core::ffi::c_int;
    while mode < NUM_PRED_MODES as ::core::ffi::c_int {
        let mut rd_uv: VP8ModeScore = VP8ModeScore {
            D: 0,
            SD: 0,
            H: 0,
            R: 0,
            score: 0,
            y_dc_levels: [0; 16],
            y_ac_levels: [[0; 16]; 16],
            uv_levels: [[0; 16]; 8],
            mode_i16: 0,
            modes_i4: [0; 16],
            mode_uv: 0,
            nz: 0,
            derr: [[0; 3]; 2],
        };
        rd_uv.nz = ReconstructUV(it, &raw mut rd_uv, tmp_dst, mode) as uint32_t;
        rd_uv.D = VP8SSE16x8.expect("non-null function pointer")(src, tmp_dst) as score_t;
        rd_uv.SD = 0 as score_t;
        rd_uv.H = VP8FixedCostsUV[mode as usize] as score_t;
        rd_uv.R = VP8GetCostUV(it, &raw mut rd_uv) as score_t;
        if mode > 0 as ::core::ffi::c_int
            && IsFlat_C(
                &raw mut *(&raw mut rd_uv.uv_levels as *mut [int16_t; 16])
                    .offset(0 as ::core::ffi::c_int as isize) as *mut int16_t,
                kNumBlocks,
                FLATNESS_LIMIT_UV,
            ) != 0
        {
            rd_uv.R += (FLATNESS_PENALTY * kNumBlocks) as score_t;
        }
        SetRDScore(lambda, &raw mut rd_uv);
        if mode == 0 as ::core::ffi::c_int || rd_uv.score < rd_best.score {
            CopyScore(&raw mut rd_best, &raw mut rd_uv);
            (*rd).mode_uv = mode;
            memcpy(
                &raw mut (*rd).uv_levels as *mut [int16_t; 16] as *mut ::core::ffi::c_void,
                &raw mut rd_uv.uv_levels as *mut [int16_t; 16] as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[[int16_t; 16]; 8]>() as size_t,
            );
            if !(*it).top_derr_.is_null() {
                memcpy(
                    &raw mut (*rd).derr as *mut [int8_t; 3] as *mut ::core::ffi::c_void,
                    &raw mut rd_uv.derr as *mut [int8_t; 3] as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[[int8_t; 3]; 2]>() as size_t,
                );
            }
            SwapPtr(&raw mut dst, &raw mut tmp_dst);
        }
        mode += 1;
    }
    VP8SetIntraUVMode(it, (*rd).mode_uv);
    AddScore(rd, &raw mut rd_best);
    if dst != dst0 {
        VP8Copy16x8.expect("non-null function pointer")(dst, dst0);
    }
    if !(*it).top_derr_.is_null() {
        StoreDiffusionErrors(it, rd);
    }
}
unsafe extern "C" fn SimpleQuantize(it: *mut VP8EncIterator, rd: *mut VP8ModeScore) {
    let enc: *const VP8Encoder = (*it).enc_;
    let is_i16: ::core::ffi::c_int = ((*(*it).mb_).type_() as ::core::ffi::c_int
        == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if is_i16 != 0 {
        nz = ReconstructIntra16(
            it,
            rd,
            (*it).yuv_out_.offset(Y_OFF_ENC as isize),
            *(*it).preds_.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        );
    } else {
        VP8IteratorStartI4(it);
        loop {
            let mode: ::core::ffi::c_int = *(*it).preds_.offset(
                (((*it).i4_ & 3 as ::core::ffi::c_int)
                    + ((*it).i4_ >> 2 as ::core::ffi::c_int) * (*enc).preds_w_)
                    as isize,
            ) as ::core::ffi::c_int;
            let src: *const uint8_t = (*it)
                .yuv_in_
                .offset(Y_OFF_ENC as isize)
                .offset(VP8Scan[(*it).i4_ as usize] as ::core::ffi::c_int as isize);
            let dst: *mut uint8_t = (*it)
                .yuv_out_
                .offset(Y_OFF_ENC as isize)
                .offset(VP8Scan[(*it).i4_ as usize] as ::core::ffi::c_int as isize);
            VP8MakeIntra4Preds(it);
            nz |= ReconstructIntra4(
                it,
                &raw mut *(&raw mut (*rd).y_ac_levels as *mut [int16_t; 16])
                    .offset((*it).i4_ as isize) as *mut int16_t,
                src,
                dst,
                mode,
            ) << (*it).i4_;
            if !(VP8IteratorRotateI4(it, (*it).yuv_out_.offset(Y_OFF_ENC as isize)) != 0) {
                break;
            }
        }
    }
    nz |= ReconstructUV(
        it,
        rd,
        (*it).yuv_out_.offset(U_OFF_ENC as isize),
        (*(*it).mb_).uv_mode_() as ::core::ffi::c_int,
    );
    (*rd).nz = nz as uint32_t;
}
unsafe extern "C" fn RefineUsingDistortion(
    it: *mut VP8EncIterator,
    mut try_both_modes: ::core::ffi::c_int,
    mut refine_uv_mode: ::core::ffi::c_int,
    rd: *mut VP8ModeScore,
) {
    let mut best_score: score_t = MAX_COST;
    let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mode: ::core::ffi::c_int = 0;
    let mut is_i16: ::core::ffi::c_int = (try_both_modes != 0
        || (*(*it).mb_).type_() as ::core::ffi::c_int == 1 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    let dqm: *const VP8SegmentInfo = (&raw mut (*(*it).enc_).dqm_ as *mut VP8SegmentInfo)
        .offset((*(*it).mb_).segment_() as isize)
        as *mut VP8SegmentInfo;
    let lambda_d_i16: ::core::ffi::c_int = 106 as ::core::ffi::c_int;
    let lambda_d_i4: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
    let lambda_d_uv: ::core::ffi::c_int = 120 as ::core::ffi::c_int;
    let mut score_i4: score_t = (*dqm).i4_penalty_;
    let mut i4_bit_sum: score_t = 0 as score_t;
    let bit_limit: score_t = if try_both_modes != 0 {
        (*(*it).enc_).mb_header_limit_ as score_t
    } else {
        MAX_COST
    };
    if is_i16 != 0 {
        let mut best_mode: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let src: *const uint8_t = (*it).yuv_in_.offset(Y_OFF_ENC as isize);
        mode = 0 as ::core::ffi::c_int;
        while mode < NUM_PRED_MODES as ::core::ffi::c_int {
            let ref_0: *const uint8_t = (*it)
                .yuv_p_
                .offset(VP8I16ModeOffsets[mode as usize] as ::core::ffi::c_int as isize);
            let score: score_t = VP8SSE16x16.expect("non-null function pointer")(src, ref_0)
                as score_t
                * RD_DISTO_MULT as score_t
                + (VP8FixedCostsI16[mode as usize] as ::core::ffi::c_int * lambda_d_i16) as score_t;
            if !(mode > 0 as ::core::ffi::c_int
                && VP8FixedCostsI16[mode as usize] as score_t > bit_limit)
            {
                if score < best_score {
                    best_mode = mode;
                    best_score = score;
                }
            }
            mode += 1;
        }
        if (*it).x_ == 0 as ::core::ffi::c_int || (*it).y_ == 0 as ::core::ffi::c_int {
            if IsFlatSource16(src) != 0 {
                best_mode = if (*it).x_ == 0 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                };
                try_both_modes = 0 as ::core::ffi::c_int;
            }
        }
        VP8SetIntra16Mode(it, best_mode);
    }
    if try_both_modes != 0 || is_i16 == 0 {
        is_i16 = 0 as ::core::ffi::c_int;
        VP8IteratorStartI4(it);
        loop {
            let mut best_i4_mode: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
            let mut best_i4_score: score_t = MAX_COST;
            let src_0: *const uint8_t = (*it)
                .yuv_in_
                .offset(Y_OFF_ENC as isize)
                .offset(VP8Scan[(*it).i4_ as usize] as ::core::ffi::c_int as isize);
            let mode_costs: *const uint16_t = GetCostModeI4(
                it,
                &raw mut (*rd).modes_i4 as *mut uint8_t as *const uint8_t,
            ) as *const uint16_t;
            VP8MakeIntra4Preds(it);
            mode = 0 as ::core::ffi::c_int;
            while mode < NUM_BMODES as ::core::ffi::c_int {
                let ref_1: *const uint8_t = (*it)
                    .yuv_p_
                    .offset(VP8I4ModeOffsets[mode as usize] as ::core::ffi::c_int as isize);
                let score_0: score_t = (VP8SSE4x4.expect("non-null function pointer")(src_0, ref_1)
                    * RD_DISTO_MULT
                    + *mode_costs.offset(mode as isize) as ::core::ffi::c_int * lambda_d_i4)
                    as score_t;
                if score_0 < best_i4_score {
                    best_i4_mode = mode;
                    best_i4_score = score_0;
                }
                mode += 1;
            }
            i4_bit_sum += *mode_costs.offset(best_i4_mode as isize) as score_t;
            (*rd).modes_i4[(*it).i4_ as usize] = best_i4_mode as uint8_t;
            score_i4 += best_i4_score;
            if score_i4 >= best_score || i4_bit_sum > bit_limit {
                is_i16 = 1 as ::core::ffi::c_int;
                break;
            } else {
                let tmp_dst: *mut uint8_t = (*it)
                    .yuv_out2_
                    .offset(Y_OFF_ENC as isize)
                    .offset(VP8Scan[(*it).i4_ as usize] as ::core::ffi::c_int as isize);
                nz |= ReconstructIntra4(
                    it,
                    &raw mut *(&raw mut (*rd).y_ac_levels as *mut [int16_t; 16])
                        .offset((*it).i4_ as isize) as *mut int16_t,
                    src_0,
                    tmp_dst,
                    best_i4_mode,
                ) << (*it).i4_;
                if !(VP8IteratorRotateI4(it, (*it).yuv_out2_.offset(Y_OFF_ENC as isize)) != 0) {
                    break;
                }
            }
        }
    }
    if is_i16 == 0 {
        VP8SetIntra4Mode(it, &raw mut (*rd).modes_i4 as *mut uint8_t);
        SwapOut(it);
        best_score = score_i4;
    } else {
        nz = ReconstructIntra16(
            it,
            rd,
            (*it).yuv_out_.offset(Y_OFF_ENC as isize),
            *(*it).preds_.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        );
    }
    if refine_uv_mode != 0 {
        let mut best_mode_0: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut best_uv_score: score_t = MAX_COST;
        let src_1: *const uint8_t = (*it).yuv_in_.offset(U_OFF_ENC as isize);
        mode = 0 as ::core::ffi::c_int;
        while mode < NUM_PRED_MODES as ::core::ffi::c_int {
            let ref_2: *const uint8_t = (*it)
                .yuv_p_
                .offset(VP8UVModeOffsets[mode as usize] as ::core::ffi::c_int as isize);
            let score_1: score_t = (VP8SSE16x8.expect("non-null function pointer")(src_1, ref_2)
                * RD_DISTO_MULT
                + VP8FixedCostsUV[mode as usize] as ::core::ffi::c_int * lambda_d_uv)
                as score_t;
            if score_1 < best_uv_score {
                best_mode_0 = mode;
                best_uv_score = score_1;
            }
            mode += 1;
        }
        VP8SetIntraUVMode(it, best_mode_0);
    }
    nz |= ReconstructUV(
        it,
        rd,
        (*it).yuv_out_.offset(U_OFF_ENC as isize),
        (*(*it).mb_).uv_mode_() as ::core::ffi::c_int,
    );
    (*rd).nz = nz as uint32_t;
    (*rd).score = best_score;
}
#[no_mangle]
pub unsafe extern "C" fn VP8Decimate(
    it: *mut VP8EncIterator,
    rd: *mut VP8ModeScore,
    mut rd_opt: VP8RDLevel,
) -> ::core::ffi::c_int {
    let mut is_skipped: ::core::ffi::c_int = 0;
    let method: ::core::ffi::c_int = (*(*it).enc_).method_;
    InitScore(rd);
    VP8MakeLuma16Preds(it);
    VP8MakeChroma8Preds(it);
    if rd_opt as ::core::ffi::c_uint > RD_OPT_NONE as ::core::ffi::c_int as ::core::ffi::c_uint {
        (*it).do_trellis_ = (rd_opt as ::core::ffi::c_uint
            >= RD_OPT_TRELLIS_ALL as ::core::ffi::c_int as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        PickBestIntra16(it, rd);
        if method >= 2 as ::core::ffi::c_int {
            PickBestIntra4(it, rd);
        }
        PickBestUV(it, rd);
        if rd_opt as ::core::ffi::c_uint
            == RD_OPT_TRELLIS as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*it).do_trellis_ = 1 as ::core::ffi::c_int;
            SimpleQuantize(it, rd);
        }
    } else {
        RefineUsingDistortion(
            it,
            (method >= 2 as ::core::ffi::c_int) as ::core::ffi::c_int,
            (method >= 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
            rd,
        );
    }
    is_skipped = ((*rd).nz == 0 as uint32_t) as ::core::ffi::c_int;
    VP8SetSkip(it, is_skipped);
    return is_skipped;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
