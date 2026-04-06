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
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut VP8SSIMGetClipped: VP8SSIMGetClippedFunc;
    fn VP8SSIMDspInit();
    static mut VP8SimpleVFilter16i: VP8SimpleFilterFunc;
    static mut VP8SimpleHFilter16i: VP8SimpleFilterFunc;
    static mut VP8VFilter16i: VP8LumaFilterFunc;
    static mut VP8HFilter16i: VP8LumaFilterFunc;
    static mut VP8VFilter8i: VP8ChromaFilterFunc;
    static mut VP8HFilter8i: VP8ChromaFilterFunc;
}
pub type size_t = usize;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const NUM_PROBAS: C2RustUnnamed = 11;
pub const NUM_CTX: C2RustUnnamed = 3;
pub const NUM_BANDS: C2RustUnnamed = 8;
pub const NUM_TYPES: C2RustUnnamed = 4;
pub const MAX_NUM_PARTITIONS: C2RustUnnamed = 8;
pub const NUM_MODE_LF_DELTAS: C2RustUnnamed = 4;
pub const NUM_REF_LF_DELTAS: C2RustUnnamed = 4;
pub const NUM_MB_SEGMENTS: C2RustUnnamed = 4;
pub const MB_FEATURE_TREE_PROBS: C2RustUnnamed = 3;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Matrix {
    pub q_: [uint16_t; 16],
    pub iq_: [uint16_t; 16],
    pub bias_: [uint32_t; 16],
    pub zthresh_: [uint32_t; 16],
    pub sharpen_: [uint16_t; 16],
}
pub type VP8SSIMGetClippedFunc = Option<
    unsafe extern "C" fn(
        *const uint8_t,
        ::core::ffi::c_int,
        *const uint8_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_double,
>;
pub type VP8SimpleFilterFunc =
    Option<unsafe extern "C" fn(*mut uint8_t, ::core::ffi::c_int, ::core::ffi::c_int) -> ()>;
pub type VP8LumaFilterFunc = Option<
    unsafe extern "C" fn(
        *mut uint8_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type VP8ChromaFilterFunc = Option<
    unsafe extern "C" fn(
        *mut uint8_t,
        *mut uint8_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
>;
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const MAX_LEVEL: C2RustUnnamed_0 = 2047;
pub const MAX_VARIABLE_LEVEL: C2RustUnnamed_0 = 67;
pub const MAX_LF_LEVELS: C2RustUnnamed_0 = 64;
pub type VP8RDLevel = ::core::ffi::c_uint;
pub const RD_OPT_TRELLIS_ALL: VP8RDLevel = 3;
pub const RD_OPT_TRELLIS: VP8RDLevel = 2;
pub const RD_OPT_BASIC: VP8RDLevel = 1;
pub const RD_OPT_NONE: VP8RDLevel = 0;
pub type score_t = int64_t;
pub type proba_t = uint32_t;
pub type ProbaArray = [[uint8_t; 11]; 3];
pub type StatsArray = [[proba_t; 11]; 3];
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BPS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const VP8_SSIM_KERNEL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const YUV_SIZE_ENC: ::core::ffi::c_int = BPS * 16 as ::core::ffi::c_int;
pub const Y_OFF_ENC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const U_OFF_ENC: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const V_OFF_ENC: ::core::ffi::c_int = 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int;
pub const MAX_DELTA_SIZE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
static mut kLevelsFromDelta: [[uint8_t; 64]; 8] = [
    [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
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
        38 as ::core::ffi::c_int as uint8_t,
        39 as ::core::ffi::c_int as uint8_t,
        40 as ::core::ffi::c_int as uint8_t,
        41 as ::core::ffi::c_int as uint8_t,
        42 as ::core::ffi::c_int as uint8_t,
        43 as ::core::ffi::c_int as uint8_t,
        44 as ::core::ffi::c_int as uint8_t,
        45 as ::core::ffi::c_int as uint8_t,
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
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        26 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        29 as ::core::ffi::c_int as uint8_t,
        30 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
        35 as ::core::ffi::c_int as uint8_t,
        36 as ::core::ffi::c_int as uint8_t,
        38 as ::core::ffi::c_int as uint8_t,
        39 as ::core::ffi::c_int as uint8_t,
        41 as ::core::ffi::c_int as uint8_t,
        42 as ::core::ffi::c_int as uint8_t,
        44 as ::core::ffi::c_int as uint8_t,
        45 as ::core::ffi::c_int as uint8_t,
        47 as ::core::ffi::c_int as uint8_t,
        48 as ::core::ffi::c_int as uint8_t,
        50 as ::core::ffi::c_int as uint8_t,
        51 as ::core::ffi::c_int as uint8_t,
        53 as ::core::ffi::c_int as uint8_t,
        54 as ::core::ffi::c_int as uint8_t,
        56 as ::core::ffi::c_int as uint8_t,
        57 as ::core::ffi::c_int as uint8_t,
        59 as ::core::ffi::c_int as uint8_t,
        60 as ::core::ffi::c_int as uint8_t,
        62 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        26 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        29 as ::core::ffi::c_int as uint8_t,
        31 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        34 as ::core::ffi::c_int as uint8_t,
        35 as ::core::ffi::c_int as uint8_t,
        37 as ::core::ffi::c_int as uint8_t,
        38 as ::core::ffi::c_int as uint8_t,
        40 as ::core::ffi::c_int as uint8_t,
        41 as ::core::ffi::c_int as uint8_t,
        43 as ::core::ffi::c_int as uint8_t,
        44 as ::core::ffi::c_int as uint8_t,
        46 as ::core::ffi::c_int as uint8_t,
        47 as ::core::ffi::c_int as uint8_t,
        49 as ::core::ffi::c_int as uint8_t,
        50 as ::core::ffi::c_int as uint8_t,
        52 as ::core::ffi::c_int as uint8_t,
        53 as ::core::ffi::c_int as uint8_t,
        55 as ::core::ffi::c_int as uint8_t,
        56 as ::core::ffi::c_int as uint8_t,
        58 as ::core::ffi::c_int as uint8_t,
        59 as ::core::ffi::c_int as uint8_t,
        61 as ::core::ffi::c_int as uint8_t,
        62 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        30 as ::core::ffi::c_int as uint8_t,
        31 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
        34 as ::core::ffi::c_int as uint8_t,
        36 as ::core::ffi::c_int as uint8_t,
        37 as ::core::ffi::c_int as uint8_t,
        39 as ::core::ffi::c_int as uint8_t,
        40 as ::core::ffi::c_int as uint8_t,
        42 as ::core::ffi::c_int as uint8_t,
        43 as ::core::ffi::c_int as uint8_t,
        45 as ::core::ffi::c_int as uint8_t,
        46 as ::core::ffi::c_int as uint8_t,
        48 as ::core::ffi::c_int as uint8_t,
        49 as ::core::ffi::c_int as uint8_t,
        51 as ::core::ffi::c_int as uint8_t,
        52 as ::core::ffi::c_int as uint8_t,
        54 as ::core::ffi::c_int as uint8_t,
        55 as ::core::ffi::c_int as uint8_t,
        57 as ::core::ffi::c_int as uint8_t,
        58 as ::core::ffi::c_int as uint8_t,
        60 as ::core::ffi::c_int as uint8_t,
        61 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        26 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        29 as ::core::ffi::c_int as uint8_t,
        30 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
        35 as ::core::ffi::c_int as uint8_t,
        36 as ::core::ffi::c_int as uint8_t,
        38 as ::core::ffi::c_int as uint8_t,
        39 as ::core::ffi::c_int as uint8_t,
        41 as ::core::ffi::c_int as uint8_t,
        42 as ::core::ffi::c_int as uint8_t,
        44 as ::core::ffi::c_int as uint8_t,
        45 as ::core::ffi::c_int as uint8_t,
        47 as ::core::ffi::c_int as uint8_t,
        48 as ::core::ffi::c_int as uint8_t,
        50 as ::core::ffi::c_int as uint8_t,
        51 as ::core::ffi::c_int as uint8_t,
        53 as ::core::ffi::c_int as uint8_t,
        54 as ::core::ffi::c_int as uint8_t,
        56 as ::core::ffi::c_int as uint8_t,
        57 as ::core::ffi::c_int as uint8_t,
        59 as ::core::ffi::c_int as uint8_t,
        60 as ::core::ffi::c_int as uint8_t,
        62 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        26 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        29 as ::core::ffi::c_int as uint8_t,
        31 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        34 as ::core::ffi::c_int as uint8_t,
        35 as ::core::ffi::c_int as uint8_t,
        37 as ::core::ffi::c_int as uint8_t,
        38 as ::core::ffi::c_int as uint8_t,
        40 as ::core::ffi::c_int as uint8_t,
        41 as ::core::ffi::c_int as uint8_t,
        43 as ::core::ffi::c_int as uint8_t,
        44 as ::core::ffi::c_int as uint8_t,
        46 as ::core::ffi::c_int as uint8_t,
        47 as ::core::ffi::c_int as uint8_t,
        49 as ::core::ffi::c_int as uint8_t,
        50 as ::core::ffi::c_int as uint8_t,
        52 as ::core::ffi::c_int as uint8_t,
        53 as ::core::ffi::c_int as uint8_t,
        55 as ::core::ffi::c_int as uint8_t,
        56 as ::core::ffi::c_int as uint8_t,
        58 as ::core::ffi::c_int as uint8_t,
        59 as ::core::ffi::c_int as uint8_t,
        61 as ::core::ffi::c_int as uint8_t,
        62 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        30 as ::core::ffi::c_int as uint8_t,
        31 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
        34 as ::core::ffi::c_int as uint8_t,
        36 as ::core::ffi::c_int as uint8_t,
        37 as ::core::ffi::c_int as uint8_t,
        39 as ::core::ffi::c_int as uint8_t,
        40 as ::core::ffi::c_int as uint8_t,
        42 as ::core::ffi::c_int as uint8_t,
        43 as ::core::ffi::c_int as uint8_t,
        45 as ::core::ffi::c_int as uint8_t,
        46 as ::core::ffi::c_int as uint8_t,
        48 as ::core::ffi::c_int as uint8_t,
        49 as ::core::ffi::c_int as uint8_t,
        51 as ::core::ffi::c_int as uint8_t,
        52 as ::core::ffi::c_int as uint8_t,
        54 as ::core::ffi::c_int as uint8_t,
        55 as ::core::ffi::c_int as uint8_t,
        57 as ::core::ffi::c_int as uint8_t,
        58 as ::core::ffi::c_int as uint8_t,
        60 as ::core::ffi::c_int as uint8_t,
        61 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        26 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        29 as ::core::ffi::c_int as uint8_t,
        30 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
        35 as ::core::ffi::c_int as uint8_t,
        36 as ::core::ffi::c_int as uint8_t,
        38 as ::core::ffi::c_int as uint8_t,
        39 as ::core::ffi::c_int as uint8_t,
        41 as ::core::ffi::c_int as uint8_t,
        42 as ::core::ffi::c_int as uint8_t,
        44 as ::core::ffi::c_int as uint8_t,
        45 as ::core::ffi::c_int as uint8_t,
        47 as ::core::ffi::c_int as uint8_t,
        48 as ::core::ffi::c_int as uint8_t,
        50 as ::core::ffi::c_int as uint8_t,
        51 as ::core::ffi::c_int as uint8_t,
        53 as ::core::ffi::c_int as uint8_t,
        54 as ::core::ffi::c_int as uint8_t,
        56 as ::core::ffi::c_int as uint8_t,
        57 as ::core::ffi::c_int as uint8_t,
        59 as ::core::ffi::c_int as uint8_t,
        60 as ::core::ffi::c_int as uint8_t,
        62 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn VP8FilterStrengthFromDelta(
    mut sharpness: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pos: ::core::ffi::c_int = if delta < MAX_DELTA_SIZE {
        delta
    } else {
        MAX_DELTA_SIZE - 1 as ::core::ffi::c_int
    };
    '_c2rust_label: {
        if sharpness >= 0 as ::core::ffi::c_int && sharpness <= 7 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"sharpness >= 0 && sharpness <= 7\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/filter_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                61 as ::core::ffi::c_uint,
                b"int VP8FilterStrengthFromDelta(int, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return kLevelsFromDelta[sharpness as usize][pos as usize] as ::core::ffi::c_int;
}
unsafe extern "C" fn GetILevel(
    mut sharpness: ::core::ffi::c_int,
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if sharpness > 0 as ::core::ffi::c_int {
        if sharpness > 4 as ::core::ffi::c_int {
            level >>= 2 as ::core::ffi::c_int;
        } else {
            level >>= 1 as ::core::ffi::c_int;
        }
        if level > 9 as ::core::ffi::c_int - sharpness {
            level = 9 as ::core::ffi::c_int - sharpness;
        }
    }
    if level < 1 as ::core::ffi::c_int {
        level = 1 as ::core::ffi::c_int;
    }
    return level;
}
unsafe extern "C" fn DoFilter(it: *const VP8EncIterator, mut level: ::core::ffi::c_int) {
    let enc: *const VP8Encoder = (*it).enc_;
    let ilevel: ::core::ffi::c_int =
        GetILevel((*(*enc).config_).filter_sharpness, level) as ::core::ffi::c_int;
    let limit: ::core::ffi::c_int = 2 as ::core::ffi::c_int * level + ilevel;
    let y_dst: *mut uint8_t = (*it).yuv_out2_.offset(Y_OFF_ENC as isize);
    let u_dst: *mut uint8_t = (*it).yuv_out2_.offset(U_OFF_ENC as isize);
    let v_dst: *mut uint8_t = (*it).yuv_out2_.offset(V_OFF_ENC as isize);
    memcpy(
        y_dst as *mut ::core::ffi::c_void,
        (*it).yuv_out_ as *const ::core::ffi::c_void,
        (YUV_SIZE_ENC as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    );
    if (*enc).filter_hdr_.simple_ == 1 as ::core::ffi::c_int {
        VP8SimpleHFilter16i.expect("non-null function pointer")(y_dst, BPS, limit);
        VP8SimpleVFilter16i.expect("non-null function pointer")(y_dst, BPS, limit);
    } else {
        let hev_thresh: ::core::ffi::c_int = if level >= 40 as ::core::ffi::c_int {
            2 as ::core::ffi::c_int
        } else if level >= 15 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        VP8HFilter16i.expect("non-null function pointer")(y_dst, BPS, limit, ilevel, hev_thresh);
        VP8HFilter8i.expect("non-null function pointer")(
            u_dst, v_dst, BPS, limit, ilevel, hev_thresh,
        );
        VP8VFilter16i.expect("non-null function pointer")(y_dst, BPS, limit, ilevel, hev_thresh);
        VP8VFilter8i.expect("non-null function pointer")(
            u_dst, v_dst, BPS, limit, ilevel, hev_thresh,
        );
    };
}
unsafe extern "C" fn GetMBSSIM(
    mut yuv1: *const uint8_t,
    mut yuv2: *const uint8_t,
) -> ::core::ffi::c_double {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut sum: ::core::ffi::c_double = 0.0f64;
    y = VP8_SSIM_KERNEL;
    while y < 16 as ::core::ffi::c_int - VP8_SSIM_KERNEL {
        x = VP8_SSIM_KERNEL;
        while x < 16 as ::core::ffi::c_int - VP8_SSIM_KERNEL {
            sum += VP8SSIMGetClipped.expect("non-null function pointer")(
                yuv1.offset(Y_OFF_ENC as isize),
                BPS,
                yuv2.offset(Y_OFF_ENC as isize),
                BPS,
                x,
                y,
                16 as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
            );
            x += 1;
        }
        y += 1;
    }
    x = 1 as ::core::ffi::c_int;
    while x < 7 as ::core::ffi::c_int {
        y = 1 as ::core::ffi::c_int;
        while y < 7 as ::core::ffi::c_int {
            sum += VP8SSIMGetClipped.expect("non-null function pointer")(
                yuv1.offset(U_OFF_ENC as isize),
                BPS,
                yuv2.offset(U_OFF_ENC as isize),
                BPS,
                x,
                y,
                8 as ::core::ffi::c_int,
                8 as ::core::ffi::c_int,
            );
            sum += VP8SSIMGetClipped.expect("non-null function pointer")(
                yuv1.offset(V_OFF_ENC as isize),
                BPS,
                yuv2.offset(V_OFF_ENC as isize),
                BPS,
                x,
                y,
                8 as ::core::ffi::c_int,
                8 as ::core::ffi::c_int,
            );
            y += 1;
        }
        x += 1;
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn VP8InitFilter(it: *mut VP8EncIterator) {
    if !(*it).lf_stats_.is_null() {
        let mut s: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        s = 0 as ::core::ffi::c_int;
        while s < NUM_MB_SEGMENTS as ::core::ffi::c_int {
            i = 0 as ::core::ffi::c_int;
            while i < MAX_LF_LEVELS as ::core::ffi::c_int {
                (*(*it).lf_stats_)[s as usize][i as usize] =
                    0 as ::core::ffi::c_int as ::core::ffi::c_double;
                i += 1;
            }
            s += 1;
        }
        VP8SSIMDspInit();
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8StoreFilterStats(it: *mut VP8EncIterator) {
    let mut d: ::core::ffi::c_int = 0;
    let enc: *mut VP8Encoder = (*it).enc_;
    let s: ::core::ffi::c_int = (*(*it).mb_).segment_() as ::core::ffi::c_int;
    let level0: ::core::ffi::c_int = (*enc).dqm_[s as usize].fstrength_;
    let delta_min: ::core::ffi::c_int = -(*enc).dqm_[s as usize].quant_;
    let delta_max: ::core::ffi::c_int = (*enc).dqm_[s as usize].quant_;
    let step_size: ::core::ffi::c_int = if delta_max - delta_min >= 4 as ::core::ffi::c_int {
        4 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    if (*it).lf_stats_.is_null() {
        return;
    }
    if (*(*it).mb_).type_() as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        && (*(*it).mb_).skip_() as ::core::ffi::c_int != 0
    {
        return;
    }
    (*(*it).lf_stats_)[s as usize][0 as ::core::ffi::c_int as usize] +=
        GetMBSSIM((*it).yuv_in_, (*it).yuv_out_);
    d = delta_min;
    while d <= delta_max {
        let level: ::core::ffi::c_int = level0 + d;
        if !(level <= 0 as ::core::ffi::c_int || level >= MAX_LF_LEVELS as ::core::ffi::c_int) {
            DoFilter(it, level);
            (*(*it).lf_stats_)[s as usize][level as usize] +=
                GetMBSSIM((*it).yuv_in_, (*it).yuv_out2_);
        }
        d += step_size;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8AdjustFilterStrength(it: *mut VP8EncIterator) {
    let enc: *mut VP8Encoder = (*it).enc_;
    if !(*it).lf_stats_.is_null() {
        let mut s: ::core::ffi::c_int = 0;
        s = 0 as ::core::ffi::c_int;
        while s < NUM_MB_SEGMENTS as ::core::ffi::c_int {
            let mut i: ::core::ffi::c_int = 0;
            let mut best_level: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut best_v: ::core::ffi::c_double =
                1.00001f64 * (*(*it).lf_stats_)[s as usize][0 as ::core::ffi::c_int as usize];
            i = 1 as ::core::ffi::c_int;
            while i < MAX_LF_LEVELS as ::core::ffi::c_int {
                let v: ::core::ffi::c_double = (*(*it).lf_stats_)[s as usize][i as usize];
                if v > best_v {
                    best_v = v;
                    best_level = i;
                }
                i += 1;
            }
            (*enc).dqm_[s as usize].fstrength_ = best_level;
            s += 1;
        }
        return;
    }
    if (*(*enc).config_).filter_strength > 0 as ::core::ffi::c_int {
        let mut max_level: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut s_0: ::core::ffi::c_int = 0;
        s_0 = 0 as ::core::ffi::c_int;
        while s_0 < NUM_MB_SEGMENTS as ::core::ffi::c_int {
            let dqm: *mut VP8SegmentInfo = (&raw mut (*enc).dqm_ as *mut VP8SegmentInfo)
                .offset(s_0 as isize)
                as *mut VP8SegmentInfo;
            let delta: ::core::ffi::c_int = (*dqm).max_edge_
                * (*dqm).y2_.q_[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                >> 3 as ::core::ffi::c_int;
            let level: ::core::ffi::c_int =
                VP8FilterStrengthFromDelta((*enc).filter_hdr_.sharpness_, delta)
                    as ::core::ffi::c_int;
            if level > (*dqm).fstrength_ {
                (*dqm).fstrength_ = level;
            }
            if max_level < (*dqm).fstrength_ {
                max_level = (*dqm).fstrength_;
            }
            s_0 += 1;
        }
        (*enc).filter_hdr_.level_ = max_level;
    }
}
