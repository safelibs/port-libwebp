use c2rust_bitfields::BitfieldStruct;

#[repr(C)]
pub struct VP8Tokens {
    _unused: [u8; 0],
}

extern "C" {
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static VP8EntropyCost: [uint16_t; 256];
    static VP8EncBands: [uint8_t; 17];
    static mut VP8SetResidualCoeffs: VP8SetResidualCoeffsFunc;
    static mut VP8GetResidualCost: VP8GetResidualCostFunc;
    fn VP8IteratorNzToBytes(it: *mut VP8EncIterator);
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Residual {
    pub first: ::core::ffi::c_int,
    pub last: ::core::ffi::c_int,
    pub coeffs: *const int16_t,
    pub coeff_type: ::core::ffi::c_int,
    pub prob: *mut ProbaArray,
    pub stats: *mut StatsArray,
    pub costs: CostArrayPtr,
}
pub type CostArrayPtr = *mut [*const uint16_t; 3];
pub type StatsArray = [[proba_t; 11]; 3];
pub type proba_t = uint32_t;
pub type ProbaArray = [[uint8_t; 11]; 3];
pub type VP8SetResidualCoeffsFunc =
    Option<unsafe extern "C" fn(*const int16_t, *mut VP8Residual) -> ()>;
pub type VP8GetResidualCostFunc =
    Option<unsafe extern "C" fn(::core::ffi::c_int, *const VP8Residual) -> ::core::ffi::c_int>;
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
#[inline]
unsafe extern "C" fn VP8RecordStats(
    mut bit: ::core::ffi::c_int,
    stats: *mut proba_t,
) -> ::core::ffi::c_int {
    let mut p: proba_t = *stats;
    if p >= 0xfffe0000 as proba_t {
        p = p.wrapping_add(1 as proba_t) >> 1 as ::core::ffi::c_int & 0x7fff7fff as proba_t;
    }
    p = (p as ::core::ffi::c_uint)
        .wrapping_add((0x10000 as ::core::ffi::c_uint).wrapping_add(bit as ::core::ffi::c_uint))
        as proba_t as proba_t;
    *stats = p;
    return bit;
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
#[no_mangle]
pub static mut VP8LevelCodes: [[uint16_t; 2]; 67] = [
    [
        0x1 as ::core::ffi::c_int as uint16_t,
        0 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x7 as ::core::ffi::c_int as uint16_t,
        0x1 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xf as ::core::ffi::c_int as uint16_t,
        0x5 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xf as ::core::ffi::c_int as uint16_t,
        0xd as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x33 as ::core::ffi::c_int as uint16_t,
        0x3 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x33 as ::core::ffi::c_int as uint16_t,
        0x3 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x33 as ::core::ffi::c_int as uint16_t,
        0x23 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x33 as ::core::ffi::c_int as uint16_t,
        0x23 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x33 as ::core::ffi::c_int as uint16_t,
        0x23 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x33 as ::core::ffi::c_int as uint16_t,
        0x23 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x13 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x13 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x13 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x13 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x13 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x13 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x13 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x13 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x93 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x93 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x93 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x93 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x93 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x93 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x93 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x93 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x93 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x93 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x93 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x93 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x93 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x93 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x93 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0xd3 as ::core::ffi::c_int as uint16_t,
        0x93 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x53 as ::core::ffi::c_int as uint16_t,
    ],
    [
        0x153 as ::core::ffi::c_int as uint16_t,
        0x153 as ::core::ffi::c_int as uint16_t,
    ],
];
unsafe extern "C" fn VariableLevelCost(
    mut level: ::core::ffi::c_int,
    mut probas: *const uint8_t,
) -> ::core::ffi::c_int {
    let mut pattern: ::core::ffi::c_int = VP8LevelCodes[(level - 1 as ::core::ffi::c_int) as usize]
        [0 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int;
    let mut bits: ::core::ffi::c_int = VP8LevelCodes[(level - 1 as ::core::ffi::c_int) as usize]
        [1 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int;
    let mut cost: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    i = 2 as ::core::ffi::c_int;
    while pattern != 0 {
        if pattern & 1 as ::core::ffi::c_int != 0 {
            cost += VP8BitCost(bits & 1 as ::core::ffi::c_int, *probas.offset(i as isize));
        }
        bits >>= 1 as ::core::ffi::c_int;
        pattern >>= 1 as ::core::ffi::c_int;
        i += 1;
    }
    return cost;
}
#[no_mangle]
pub unsafe extern "C" fn VP8CalculateLevelCosts(proba: *mut VP8EncProba) {
    let mut ctype: ::core::ffi::c_int = 0;
    let mut band: ::core::ffi::c_int = 0;
    let mut ctx: ::core::ffi::c_int = 0;
    if (*proba).dirty_ == 0 {
        return;
    }
    ctype = 0 as ::core::ffi::c_int;
    while ctype < NUM_TYPES as ::core::ffi::c_int {
        let mut n: ::core::ffi::c_int = 0;
        band = 0 as ::core::ffi::c_int;
        while band < NUM_BANDS as ::core::ffi::c_int {
            ctx = 0 as ::core::ffi::c_int;
            while ctx < NUM_CTX as ::core::ffi::c_int {
                let p: *const uint8_t = &raw mut *(&raw mut *(&raw mut *(&raw mut (*proba).coeffs_
                    as *mut [ProbaArray; 8])
                    .offset(ctype as isize)
                    as *mut ProbaArray)
                    .offset(band as isize)
                    as *mut [uint8_t; 11])
                    .offset(ctx as isize) as *mut uint8_t;
                let table: *mut uint16_t =
                    &raw mut *(&raw mut *(&raw mut *(&raw mut (*proba).level_cost_
                        as *mut [CostArray; 8])
                        .offset(ctype as isize)
                        as *mut CostArray)
                        .offset(band as isize)
                        as *mut [uint16_t; 68])
                        .offset(ctx as isize) as *mut uint16_t;
                let cost0: ::core::ffi::c_int = if ctx > 0 as ::core::ffi::c_int {
                    VP8BitCost(
                        1 as ::core::ffi::c_int,
                        *p.offset(0 as ::core::ffi::c_int as isize),
                    ) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                };
                let cost_base: ::core::ffi::c_int = VP8BitCost(
                    1 as ::core::ffi::c_int,
                    *p.offset(1 as ::core::ffi::c_int as isize),
                ) as ::core::ffi::c_int
                    + cost0;
                let mut v: ::core::ffi::c_int = 0;
                *table.offset(0 as ::core::ffi::c_int as isize) = (VP8BitCost(
                    0 as ::core::ffi::c_int,
                    *p.offset(1 as ::core::ffi::c_int as isize),
                ) + cost0)
                    as uint16_t;
                v = 1 as ::core::ffi::c_int;
                while v <= MAX_VARIABLE_LEVEL as ::core::ffi::c_int {
                    *table.offset(v as isize) =
                        (cost_base + VariableLevelCost(v, p as *const uint8_t)) as uint16_t;
                    v += 1;
                }
                ctx += 1;
            }
            band += 1;
        }
        n = 0 as ::core::ffi::c_int;
        while n < 16 as ::core::ffi::c_int {
            ctx = 0 as ::core::ffi::c_int;
            while ctx < NUM_CTX as ::core::ffi::c_int {
                (*proba).remapped_costs_[ctype as usize][n as usize][ctx as usize] =
                    &raw mut *(&raw mut *(&raw mut *(&raw mut (*proba).level_cost_
                        as *mut [CostArray; 8])
                        .offset(ctype as isize)
                        as *mut CostArray)
                        .offset(
                            *(&raw const VP8EncBands as *const uint8_t).offset(n as isize) as isize,
                        ) as *mut [uint16_t; 68])
                        .offset(ctx as isize) as *mut uint16_t;
                ctx += 1;
            }
            n += 1;
        }
        ctype += 1;
    }
    (*proba).dirty_ = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub static mut VP8FixedCostsUV: [uint16_t; 4] = [
    302 as ::core::ffi::c_int as uint16_t,
    984 as ::core::ffi::c_int as uint16_t,
    439 as ::core::ffi::c_int as uint16_t,
    642 as ::core::ffi::c_int as uint16_t,
];
#[no_mangle]
pub static mut VP8FixedCostsI16: [uint16_t; 4] = [
    663 as ::core::ffi::c_int as uint16_t,
    919 as ::core::ffi::c_int as uint16_t,
    872 as ::core::ffi::c_int as uint16_t,
    919 as ::core::ffi::c_int as uint16_t,
];
#[no_mangle]
pub static mut VP8FixedCostsI4: [[[uint16_t; 10]; 10]; 10] = [
    [
        [
            40 as ::core::ffi::c_int as uint16_t,
            1151 as ::core::ffi::c_int as uint16_t,
            1723 as ::core::ffi::c_int as uint16_t,
            1874 as ::core::ffi::c_int as uint16_t,
            2103 as ::core::ffi::c_int as uint16_t,
            2019 as ::core::ffi::c_int as uint16_t,
            1628 as ::core::ffi::c_int as uint16_t,
            1777 as ::core::ffi::c_int as uint16_t,
            2226 as ::core::ffi::c_int as uint16_t,
            2137 as ::core::ffi::c_int as uint16_t,
        ],
        [
            192 as ::core::ffi::c_int as uint16_t,
            469 as ::core::ffi::c_int as uint16_t,
            1296 as ::core::ffi::c_int as uint16_t,
            1308 as ::core::ffi::c_int as uint16_t,
            1849 as ::core::ffi::c_int as uint16_t,
            1794 as ::core::ffi::c_int as uint16_t,
            1781 as ::core::ffi::c_int as uint16_t,
            1703 as ::core::ffi::c_int as uint16_t,
            1713 as ::core::ffi::c_int as uint16_t,
            1522 as ::core::ffi::c_int as uint16_t,
        ],
        [
            142 as ::core::ffi::c_int as uint16_t,
            910 as ::core::ffi::c_int as uint16_t,
            762 as ::core::ffi::c_int as uint16_t,
            1684 as ::core::ffi::c_int as uint16_t,
            1849 as ::core::ffi::c_int as uint16_t,
            1576 as ::core::ffi::c_int as uint16_t,
            1460 as ::core::ffi::c_int as uint16_t,
            1305 as ::core::ffi::c_int as uint16_t,
            1801 as ::core::ffi::c_int as uint16_t,
            1657 as ::core::ffi::c_int as uint16_t,
        ],
        [
            559 as ::core::ffi::c_int as uint16_t,
            641 as ::core::ffi::c_int as uint16_t,
            1370 as ::core::ffi::c_int as uint16_t,
            421 as ::core::ffi::c_int as uint16_t,
            1182 as ::core::ffi::c_int as uint16_t,
            1569 as ::core::ffi::c_int as uint16_t,
            1612 as ::core::ffi::c_int as uint16_t,
            1725 as ::core::ffi::c_int as uint16_t,
            863 as ::core::ffi::c_int as uint16_t,
            1007 as ::core::ffi::c_int as uint16_t,
        ],
        [
            299 as ::core::ffi::c_int as uint16_t,
            1059 as ::core::ffi::c_int as uint16_t,
            1256 as ::core::ffi::c_int as uint16_t,
            1108 as ::core::ffi::c_int as uint16_t,
            636 as ::core::ffi::c_int as uint16_t,
            1068 as ::core::ffi::c_int as uint16_t,
            1581 as ::core::ffi::c_int as uint16_t,
            1883 as ::core::ffi::c_int as uint16_t,
            869 as ::core::ffi::c_int as uint16_t,
            1142 as ::core::ffi::c_int as uint16_t,
        ],
        [
            277 as ::core::ffi::c_int as uint16_t,
            1111 as ::core::ffi::c_int as uint16_t,
            707 as ::core::ffi::c_int as uint16_t,
            1362 as ::core::ffi::c_int as uint16_t,
            1089 as ::core::ffi::c_int as uint16_t,
            672 as ::core::ffi::c_int as uint16_t,
            1603 as ::core::ffi::c_int as uint16_t,
            1541 as ::core::ffi::c_int as uint16_t,
            1545 as ::core::ffi::c_int as uint16_t,
            1291 as ::core::ffi::c_int as uint16_t,
        ],
        [
            214 as ::core::ffi::c_int as uint16_t,
            781 as ::core::ffi::c_int as uint16_t,
            1609 as ::core::ffi::c_int as uint16_t,
            1303 as ::core::ffi::c_int as uint16_t,
            1632 as ::core::ffi::c_int as uint16_t,
            2229 as ::core::ffi::c_int as uint16_t,
            726 as ::core::ffi::c_int as uint16_t,
            1560 as ::core::ffi::c_int as uint16_t,
            1713 as ::core::ffi::c_int as uint16_t,
            918 as ::core::ffi::c_int as uint16_t,
        ],
        [
            152 as ::core::ffi::c_int as uint16_t,
            1037 as ::core::ffi::c_int as uint16_t,
            1046 as ::core::ffi::c_int as uint16_t,
            1759 as ::core::ffi::c_int as uint16_t,
            1983 as ::core::ffi::c_int as uint16_t,
            2174 as ::core::ffi::c_int as uint16_t,
            1358 as ::core::ffi::c_int as uint16_t,
            742 as ::core::ffi::c_int as uint16_t,
            1740 as ::core::ffi::c_int as uint16_t,
            1390 as ::core::ffi::c_int as uint16_t,
        ],
        [
            512 as ::core::ffi::c_int as uint16_t,
            1046 as ::core::ffi::c_int as uint16_t,
            1420 as ::core::ffi::c_int as uint16_t,
            753 as ::core::ffi::c_int as uint16_t,
            752 as ::core::ffi::c_int as uint16_t,
            1297 as ::core::ffi::c_int as uint16_t,
            1486 as ::core::ffi::c_int as uint16_t,
            1613 as ::core::ffi::c_int as uint16_t,
            460 as ::core::ffi::c_int as uint16_t,
            1207 as ::core::ffi::c_int as uint16_t,
        ],
        [
            424 as ::core::ffi::c_int as uint16_t,
            827 as ::core::ffi::c_int as uint16_t,
            1362 as ::core::ffi::c_int as uint16_t,
            719 as ::core::ffi::c_int as uint16_t,
            1462 as ::core::ffi::c_int as uint16_t,
            1202 as ::core::ffi::c_int as uint16_t,
            1199 as ::core::ffi::c_int as uint16_t,
            1476 as ::core::ffi::c_int as uint16_t,
            1199 as ::core::ffi::c_int as uint16_t,
            538 as ::core::ffi::c_int as uint16_t,
        ],
    ],
    [
        [
            240 as ::core::ffi::c_int as uint16_t,
            402 as ::core::ffi::c_int as uint16_t,
            1134 as ::core::ffi::c_int as uint16_t,
            1491 as ::core::ffi::c_int as uint16_t,
            1659 as ::core::ffi::c_int as uint16_t,
            1505 as ::core::ffi::c_int as uint16_t,
            1517 as ::core::ffi::c_int as uint16_t,
            1555 as ::core::ffi::c_int as uint16_t,
            1979 as ::core::ffi::c_int as uint16_t,
            2099 as ::core::ffi::c_int as uint16_t,
        ],
        [
            467 as ::core::ffi::c_int as uint16_t,
            242 as ::core::ffi::c_int as uint16_t,
            960 as ::core::ffi::c_int as uint16_t,
            1232 as ::core::ffi::c_int as uint16_t,
            1714 as ::core::ffi::c_int as uint16_t,
            1620 as ::core::ffi::c_int as uint16_t,
            1834 as ::core::ffi::c_int as uint16_t,
            1570 as ::core::ffi::c_int as uint16_t,
            1676 as ::core::ffi::c_int as uint16_t,
            1391 as ::core::ffi::c_int as uint16_t,
        ],
        [
            500 as ::core::ffi::c_int as uint16_t,
            455 as ::core::ffi::c_int as uint16_t,
            463 as ::core::ffi::c_int as uint16_t,
            1507 as ::core::ffi::c_int as uint16_t,
            1699 as ::core::ffi::c_int as uint16_t,
            1282 as ::core::ffi::c_int as uint16_t,
            1564 as ::core::ffi::c_int as uint16_t,
            982 as ::core::ffi::c_int as uint16_t,
            2114 as ::core::ffi::c_int as uint16_t,
            2114 as ::core::ffi::c_int as uint16_t,
        ],
        [
            672 as ::core::ffi::c_int as uint16_t,
            643 as ::core::ffi::c_int as uint16_t,
            1372 as ::core::ffi::c_int as uint16_t,
            331 as ::core::ffi::c_int as uint16_t,
            1589 as ::core::ffi::c_int as uint16_t,
            1667 as ::core::ffi::c_int as uint16_t,
            1453 as ::core::ffi::c_int as uint16_t,
            1938 as ::core::ffi::c_int as uint16_t,
            996 as ::core::ffi::c_int as uint16_t,
            876 as ::core::ffi::c_int as uint16_t,
        ],
        [
            458 as ::core::ffi::c_int as uint16_t,
            783 as ::core::ffi::c_int as uint16_t,
            1037 as ::core::ffi::c_int as uint16_t,
            911 as ::core::ffi::c_int as uint16_t,
            738 as ::core::ffi::c_int as uint16_t,
            968 as ::core::ffi::c_int as uint16_t,
            1165 as ::core::ffi::c_int as uint16_t,
            1518 as ::core::ffi::c_int as uint16_t,
            859 as ::core::ffi::c_int as uint16_t,
            1033 as ::core::ffi::c_int as uint16_t,
        ],
        [
            504 as ::core::ffi::c_int as uint16_t,
            815 as ::core::ffi::c_int as uint16_t,
            504 as ::core::ffi::c_int as uint16_t,
            1139 as ::core::ffi::c_int as uint16_t,
            1219 as ::core::ffi::c_int as uint16_t,
            719 as ::core::ffi::c_int as uint16_t,
            1506 as ::core::ffi::c_int as uint16_t,
            1085 as ::core::ffi::c_int as uint16_t,
            1268 as ::core::ffi::c_int as uint16_t,
            1268 as ::core::ffi::c_int as uint16_t,
        ],
        [
            333 as ::core::ffi::c_int as uint16_t,
            630 as ::core::ffi::c_int as uint16_t,
            1445 as ::core::ffi::c_int as uint16_t,
            1239 as ::core::ffi::c_int as uint16_t,
            1883 as ::core::ffi::c_int as uint16_t,
            3672 as ::core::ffi::c_int as uint16_t,
            799 as ::core::ffi::c_int as uint16_t,
            1548 as ::core::ffi::c_int as uint16_t,
            1865 as ::core::ffi::c_int as uint16_t,
            598 as ::core::ffi::c_int as uint16_t,
        ],
        [
            399 as ::core::ffi::c_int as uint16_t,
            644 as ::core::ffi::c_int as uint16_t,
            746 as ::core::ffi::c_int as uint16_t,
            1342 as ::core::ffi::c_int as uint16_t,
            1856 as ::core::ffi::c_int as uint16_t,
            1350 as ::core::ffi::c_int as uint16_t,
            1493 as ::core::ffi::c_int as uint16_t,
            613 as ::core::ffi::c_int as uint16_t,
            1855 as ::core::ffi::c_int as uint16_t,
            1015 as ::core::ffi::c_int as uint16_t,
        ],
        [
            622 as ::core::ffi::c_int as uint16_t,
            749 as ::core::ffi::c_int as uint16_t,
            1205 as ::core::ffi::c_int as uint16_t,
            608 as ::core::ffi::c_int as uint16_t,
            1066 as ::core::ffi::c_int as uint16_t,
            1408 as ::core::ffi::c_int as uint16_t,
            1290 as ::core::ffi::c_int as uint16_t,
            1406 as ::core::ffi::c_int as uint16_t,
            546 as ::core::ffi::c_int as uint16_t,
            971 as ::core::ffi::c_int as uint16_t,
        ],
        [
            500 as ::core::ffi::c_int as uint16_t,
            753 as ::core::ffi::c_int as uint16_t,
            1041 as ::core::ffi::c_int as uint16_t,
            668 as ::core::ffi::c_int as uint16_t,
            1230 as ::core::ffi::c_int as uint16_t,
            1617 as ::core::ffi::c_int as uint16_t,
            1297 as ::core::ffi::c_int as uint16_t,
            1425 as ::core::ffi::c_int as uint16_t,
            1383 as ::core::ffi::c_int as uint16_t,
            523 as ::core::ffi::c_int as uint16_t,
        ],
    ],
    [
        [
            394 as ::core::ffi::c_int as uint16_t,
            553 as ::core::ffi::c_int as uint16_t,
            523 as ::core::ffi::c_int as uint16_t,
            1502 as ::core::ffi::c_int as uint16_t,
            1536 as ::core::ffi::c_int as uint16_t,
            981 as ::core::ffi::c_int as uint16_t,
            1608 as ::core::ffi::c_int as uint16_t,
            1142 as ::core::ffi::c_int as uint16_t,
            1666 as ::core::ffi::c_int as uint16_t,
            2181 as ::core::ffi::c_int as uint16_t,
        ],
        [
            655 as ::core::ffi::c_int as uint16_t,
            430 as ::core::ffi::c_int as uint16_t,
            375 as ::core::ffi::c_int as uint16_t,
            1411 as ::core::ffi::c_int as uint16_t,
            1861 as ::core::ffi::c_int as uint16_t,
            1220 as ::core::ffi::c_int as uint16_t,
            1677 as ::core::ffi::c_int as uint16_t,
            1135 as ::core::ffi::c_int as uint16_t,
            1978 as ::core::ffi::c_int as uint16_t,
            1553 as ::core::ffi::c_int as uint16_t,
        ],
        [
            690 as ::core::ffi::c_int as uint16_t,
            640 as ::core::ffi::c_int as uint16_t,
            245 as ::core::ffi::c_int as uint16_t,
            1954 as ::core::ffi::c_int as uint16_t,
            2070 as ::core::ffi::c_int as uint16_t,
            1194 as ::core::ffi::c_int as uint16_t,
            1528 as ::core::ffi::c_int as uint16_t,
            982 as ::core::ffi::c_int as uint16_t,
            1972 as ::core::ffi::c_int as uint16_t,
            2232 as ::core::ffi::c_int as uint16_t,
        ],
        [
            559 as ::core::ffi::c_int as uint16_t,
            834 as ::core::ffi::c_int as uint16_t,
            741 as ::core::ffi::c_int as uint16_t,
            867 as ::core::ffi::c_int as uint16_t,
            1131 as ::core::ffi::c_int as uint16_t,
            980 as ::core::ffi::c_int as uint16_t,
            1225 as ::core::ffi::c_int as uint16_t,
            852 as ::core::ffi::c_int as uint16_t,
            1092 as ::core::ffi::c_int as uint16_t,
            784 as ::core::ffi::c_int as uint16_t,
        ],
        [
            690 as ::core::ffi::c_int as uint16_t,
            875 as ::core::ffi::c_int as uint16_t,
            516 as ::core::ffi::c_int as uint16_t,
            959 as ::core::ffi::c_int as uint16_t,
            673 as ::core::ffi::c_int as uint16_t,
            894 as ::core::ffi::c_int as uint16_t,
            1056 as ::core::ffi::c_int as uint16_t,
            1190 as ::core::ffi::c_int as uint16_t,
            1528 as ::core::ffi::c_int as uint16_t,
            1126 as ::core::ffi::c_int as uint16_t,
        ],
        [
            740 as ::core::ffi::c_int as uint16_t,
            951 as ::core::ffi::c_int as uint16_t,
            384 as ::core::ffi::c_int as uint16_t,
            1277 as ::core::ffi::c_int as uint16_t,
            1177 as ::core::ffi::c_int as uint16_t,
            492 as ::core::ffi::c_int as uint16_t,
            1579 as ::core::ffi::c_int as uint16_t,
            1155 as ::core::ffi::c_int as uint16_t,
            1846 as ::core::ffi::c_int as uint16_t,
            1513 as ::core::ffi::c_int as uint16_t,
        ],
        [
            323 as ::core::ffi::c_int as uint16_t,
            775 as ::core::ffi::c_int as uint16_t,
            1062 as ::core::ffi::c_int as uint16_t,
            1776 as ::core::ffi::c_int as uint16_t,
            3062 as ::core::ffi::c_int as uint16_t,
            1274 as ::core::ffi::c_int as uint16_t,
            813 as ::core::ffi::c_int as uint16_t,
            1188 as ::core::ffi::c_int as uint16_t,
            1372 as ::core::ffi::c_int as uint16_t,
            655 as ::core::ffi::c_int as uint16_t,
        ],
        [
            488 as ::core::ffi::c_int as uint16_t,
            971 as ::core::ffi::c_int as uint16_t,
            484 as ::core::ffi::c_int as uint16_t,
            1767 as ::core::ffi::c_int as uint16_t,
            1515 as ::core::ffi::c_int as uint16_t,
            1775 as ::core::ffi::c_int as uint16_t,
            1115 as ::core::ffi::c_int as uint16_t,
            503 as ::core::ffi::c_int as uint16_t,
            1539 as ::core::ffi::c_int as uint16_t,
            1461 as ::core::ffi::c_int as uint16_t,
        ],
        [
            740 as ::core::ffi::c_int as uint16_t,
            1006 as ::core::ffi::c_int as uint16_t,
            998 as ::core::ffi::c_int as uint16_t,
            709 as ::core::ffi::c_int as uint16_t,
            851 as ::core::ffi::c_int as uint16_t,
            1230 as ::core::ffi::c_int as uint16_t,
            1337 as ::core::ffi::c_int as uint16_t,
            788 as ::core::ffi::c_int as uint16_t,
            741 as ::core::ffi::c_int as uint16_t,
            721 as ::core::ffi::c_int as uint16_t,
        ],
        [
            522 as ::core::ffi::c_int as uint16_t,
            1073 as ::core::ffi::c_int as uint16_t,
            573 as ::core::ffi::c_int as uint16_t,
            1045 as ::core::ffi::c_int as uint16_t,
            1346 as ::core::ffi::c_int as uint16_t,
            887 as ::core::ffi::c_int as uint16_t,
            1046 as ::core::ffi::c_int as uint16_t,
            1146 as ::core::ffi::c_int as uint16_t,
            1203 as ::core::ffi::c_int as uint16_t,
            697 as ::core::ffi::c_int as uint16_t,
        ],
    ],
    [
        [
            105 as ::core::ffi::c_int as uint16_t,
            864 as ::core::ffi::c_int as uint16_t,
            1442 as ::core::ffi::c_int as uint16_t,
            1009 as ::core::ffi::c_int as uint16_t,
            1934 as ::core::ffi::c_int as uint16_t,
            1840 as ::core::ffi::c_int as uint16_t,
            1519 as ::core::ffi::c_int as uint16_t,
            1920 as ::core::ffi::c_int as uint16_t,
            1673 as ::core::ffi::c_int as uint16_t,
            1579 as ::core::ffi::c_int as uint16_t,
        ],
        [
            534 as ::core::ffi::c_int as uint16_t,
            305 as ::core::ffi::c_int as uint16_t,
            1193 as ::core::ffi::c_int as uint16_t,
            683 as ::core::ffi::c_int as uint16_t,
            1388 as ::core::ffi::c_int as uint16_t,
            2164 as ::core::ffi::c_int as uint16_t,
            1802 as ::core::ffi::c_int as uint16_t,
            1894 as ::core::ffi::c_int as uint16_t,
            1264 as ::core::ffi::c_int as uint16_t,
            1170 as ::core::ffi::c_int as uint16_t,
        ],
        [
            305 as ::core::ffi::c_int as uint16_t,
            518 as ::core::ffi::c_int as uint16_t,
            877 as ::core::ffi::c_int as uint16_t,
            1108 as ::core::ffi::c_int as uint16_t,
            1426 as ::core::ffi::c_int as uint16_t,
            3215 as ::core::ffi::c_int as uint16_t,
            1425 as ::core::ffi::c_int as uint16_t,
            1064 as ::core::ffi::c_int as uint16_t,
            1320 as ::core::ffi::c_int as uint16_t,
            1242 as ::core::ffi::c_int as uint16_t,
        ],
        [
            683 as ::core::ffi::c_int as uint16_t,
            732 as ::core::ffi::c_int as uint16_t,
            1927 as ::core::ffi::c_int as uint16_t,
            257 as ::core::ffi::c_int as uint16_t,
            1493 as ::core::ffi::c_int as uint16_t,
            2048 as ::core::ffi::c_int as uint16_t,
            1858 as ::core::ffi::c_int as uint16_t,
            1552 as ::core::ffi::c_int as uint16_t,
            1055 as ::core::ffi::c_int as uint16_t,
            947 as ::core::ffi::c_int as uint16_t,
        ],
        [
            394 as ::core::ffi::c_int as uint16_t,
            814 as ::core::ffi::c_int as uint16_t,
            1024 as ::core::ffi::c_int as uint16_t,
            660 as ::core::ffi::c_int as uint16_t,
            959 as ::core::ffi::c_int as uint16_t,
            1556 as ::core::ffi::c_int as uint16_t,
            1282 as ::core::ffi::c_int as uint16_t,
            1289 as ::core::ffi::c_int as uint16_t,
            893 as ::core::ffi::c_int as uint16_t,
            1047 as ::core::ffi::c_int as uint16_t,
        ],
        [
            528 as ::core::ffi::c_int as uint16_t,
            615 as ::core::ffi::c_int as uint16_t,
            996 as ::core::ffi::c_int as uint16_t,
            940 as ::core::ffi::c_int as uint16_t,
            1201 as ::core::ffi::c_int as uint16_t,
            635 as ::core::ffi::c_int as uint16_t,
            1094 as ::core::ffi::c_int as uint16_t,
            2515 as ::core::ffi::c_int as uint16_t,
            803 as ::core::ffi::c_int as uint16_t,
            1358 as ::core::ffi::c_int as uint16_t,
        ],
        [
            347 as ::core::ffi::c_int as uint16_t,
            614 as ::core::ffi::c_int as uint16_t,
            1609 as ::core::ffi::c_int as uint16_t,
            1187 as ::core::ffi::c_int as uint16_t,
            3133 as ::core::ffi::c_int as uint16_t,
            1345 as ::core::ffi::c_int as uint16_t,
            1007 as ::core::ffi::c_int as uint16_t,
            1339 as ::core::ffi::c_int as uint16_t,
            1017 as ::core::ffi::c_int as uint16_t,
            667 as ::core::ffi::c_int as uint16_t,
        ],
        [
            218 as ::core::ffi::c_int as uint16_t,
            740 as ::core::ffi::c_int as uint16_t,
            878 as ::core::ffi::c_int as uint16_t,
            1605 as ::core::ffi::c_int as uint16_t,
            3650 as ::core::ffi::c_int as uint16_t,
            3650 as ::core::ffi::c_int as uint16_t,
            1345 as ::core::ffi::c_int as uint16_t,
            758 as ::core::ffi::c_int as uint16_t,
            1357 as ::core::ffi::c_int as uint16_t,
            1617 as ::core::ffi::c_int as uint16_t,
        ],
        [
            672 as ::core::ffi::c_int as uint16_t,
            750 as ::core::ffi::c_int as uint16_t,
            1541 as ::core::ffi::c_int as uint16_t,
            558 as ::core::ffi::c_int as uint16_t,
            1257 as ::core::ffi::c_int as uint16_t,
            1599 as ::core::ffi::c_int as uint16_t,
            1870 as ::core::ffi::c_int as uint16_t,
            2135 as ::core::ffi::c_int as uint16_t,
            402 as ::core::ffi::c_int as uint16_t,
            1087 as ::core::ffi::c_int as uint16_t,
        ],
        [
            592 as ::core::ffi::c_int as uint16_t,
            684 as ::core::ffi::c_int as uint16_t,
            1161 as ::core::ffi::c_int as uint16_t,
            430 as ::core::ffi::c_int as uint16_t,
            1092 as ::core::ffi::c_int as uint16_t,
            1497 as ::core::ffi::c_int as uint16_t,
            1475 as ::core::ffi::c_int as uint16_t,
            1489 as ::core::ffi::c_int as uint16_t,
            1095 as ::core::ffi::c_int as uint16_t,
            822 as ::core::ffi::c_int as uint16_t,
        ],
    ],
    [
        [
            228 as ::core::ffi::c_int as uint16_t,
            1056 as ::core::ffi::c_int as uint16_t,
            1059 as ::core::ffi::c_int as uint16_t,
            1368 as ::core::ffi::c_int as uint16_t,
            752 as ::core::ffi::c_int as uint16_t,
            982 as ::core::ffi::c_int as uint16_t,
            1512 as ::core::ffi::c_int as uint16_t,
            1518 as ::core::ffi::c_int as uint16_t,
            987 as ::core::ffi::c_int as uint16_t,
            1782 as ::core::ffi::c_int as uint16_t,
        ],
        [
            494 as ::core::ffi::c_int as uint16_t,
            514 as ::core::ffi::c_int as uint16_t,
            818 as ::core::ffi::c_int as uint16_t,
            942 as ::core::ffi::c_int as uint16_t,
            965 as ::core::ffi::c_int as uint16_t,
            892 as ::core::ffi::c_int as uint16_t,
            1610 as ::core::ffi::c_int as uint16_t,
            1356 as ::core::ffi::c_int as uint16_t,
            1048 as ::core::ffi::c_int as uint16_t,
            1363 as ::core::ffi::c_int as uint16_t,
        ],
        [
            512 as ::core::ffi::c_int as uint16_t,
            648 as ::core::ffi::c_int as uint16_t,
            591 as ::core::ffi::c_int as uint16_t,
            1042 as ::core::ffi::c_int as uint16_t,
            761 as ::core::ffi::c_int as uint16_t,
            991 as ::core::ffi::c_int as uint16_t,
            1196 as ::core::ffi::c_int as uint16_t,
            1454 as ::core::ffi::c_int as uint16_t,
            1309 as ::core::ffi::c_int as uint16_t,
            1463 as ::core::ffi::c_int as uint16_t,
        ],
        [
            683 as ::core::ffi::c_int as uint16_t,
            749 as ::core::ffi::c_int as uint16_t,
            1043 as ::core::ffi::c_int as uint16_t,
            676 as ::core::ffi::c_int as uint16_t,
            841 as ::core::ffi::c_int as uint16_t,
            1396 as ::core::ffi::c_int as uint16_t,
            1133 as ::core::ffi::c_int as uint16_t,
            1138 as ::core::ffi::c_int as uint16_t,
            654 as ::core::ffi::c_int as uint16_t,
            939 as ::core::ffi::c_int as uint16_t,
        ],
        [
            622 as ::core::ffi::c_int as uint16_t,
            1101 as ::core::ffi::c_int as uint16_t,
            1126 as ::core::ffi::c_int as uint16_t,
            994 as ::core::ffi::c_int as uint16_t,
            361 as ::core::ffi::c_int as uint16_t,
            1077 as ::core::ffi::c_int as uint16_t,
            1203 as ::core::ffi::c_int as uint16_t,
            1318 as ::core::ffi::c_int as uint16_t,
            877 as ::core::ffi::c_int as uint16_t,
            1219 as ::core::ffi::c_int as uint16_t,
        ],
        [
            631 as ::core::ffi::c_int as uint16_t,
            1068 as ::core::ffi::c_int as uint16_t,
            857 as ::core::ffi::c_int as uint16_t,
            1650 as ::core::ffi::c_int as uint16_t,
            651 as ::core::ffi::c_int as uint16_t,
            477 as ::core::ffi::c_int as uint16_t,
            1650 as ::core::ffi::c_int as uint16_t,
            1419 as ::core::ffi::c_int as uint16_t,
            828 as ::core::ffi::c_int as uint16_t,
            1170 as ::core::ffi::c_int as uint16_t,
        ],
        [
            555 as ::core::ffi::c_int as uint16_t,
            727 as ::core::ffi::c_int as uint16_t,
            1068 as ::core::ffi::c_int as uint16_t,
            1335 as ::core::ffi::c_int as uint16_t,
            3127 as ::core::ffi::c_int as uint16_t,
            1339 as ::core::ffi::c_int as uint16_t,
            820 as ::core::ffi::c_int as uint16_t,
            1331 as ::core::ffi::c_int as uint16_t,
            1077 as ::core::ffi::c_int as uint16_t,
            429 as ::core::ffi::c_int as uint16_t,
        ],
        [
            504 as ::core::ffi::c_int as uint16_t,
            879 as ::core::ffi::c_int as uint16_t,
            624 as ::core::ffi::c_int as uint16_t,
            1398 as ::core::ffi::c_int as uint16_t,
            889 as ::core::ffi::c_int as uint16_t,
            889 as ::core::ffi::c_int as uint16_t,
            1392 as ::core::ffi::c_int as uint16_t,
            808 as ::core::ffi::c_int as uint16_t,
            891 as ::core::ffi::c_int as uint16_t,
            1406 as ::core::ffi::c_int as uint16_t,
        ],
        [
            683 as ::core::ffi::c_int as uint16_t,
            1602 as ::core::ffi::c_int as uint16_t,
            1289 as ::core::ffi::c_int as uint16_t,
            977 as ::core::ffi::c_int as uint16_t,
            578 as ::core::ffi::c_int as uint16_t,
            983 as ::core::ffi::c_int as uint16_t,
            1280 as ::core::ffi::c_int as uint16_t,
            1708 as ::core::ffi::c_int as uint16_t,
            406 as ::core::ffi::c_int as uint16_t,
            1122 as ::core::ffi::c_int as uint16_t,
        ],
        [
            399 as ::core::ffi::c_int as uint16_t,
            865 as ::core::ffi::c_int as uint16_t,
            1433 as ::core::ffi::c_int as uint16_t,
            1070 as ::core::ffi::c_int as uint16_t,
            1072 as ::core::ffi::c_int as uint16_t,
            764 as ::core::ffi::c_int as uint16_t,
            968 as ::core::ffi::c_int as uint16_t,
            1477 as ::core::ffi::c_int as uint16_t,
            1223 as ::core::ffi::c_int as uint16_t,
            678 as ::core::ffi::c_int as uint16_t,
        ],
    ],
    [
        [
            333 as ::core::ffi::c_int as uint16_t,
            760 as ::core::ffi::c_int as uint16_t,
            935 as ::core::ffi::c_int as uint16_t,
            1638 as ::core::ffi::c_int as uint16_t,
            1010 as ::core::ffi::c_int as uint16_t,
            529 as ::core::ffi::c_int as uint16_t,
            1646 as ::core::ffi::c_int as uint16_t,
            1410 as ::core::ffi::c_int as uint16_t,
            1472 as ::core::ffi::c_int as uint16_t,
            2219 as ::core::ffi::c_int as uint16_t,
        ],
        [
            512 as ::core::ffi::c_int as uint16_t,
            494 as ::core::ffi::c_int as uint16_t,
            750 as ::core::ffi::c_int as uint16_t,
            1160 as ::core::ffi::c_int as uint16_t,
            1215 as ::core::ffi::c_int as uint16_t,
            610 as ::core::ffi::c_int as uint16_t,
            1870 as ::core::ffi::c_int as uint16_t,
            1868 as ::core::ffi::c_int as uint16_t,
            1628 as ::core::ffi::c_int as uint16_t,
            1169 as ::core::ffi::c_int as uint16_t,
        ],
        [
            572 as ::core::ffi::c_int as uint16_t,
            646 as ::core::ffi::c_int as uint16_t,
            492 as ::core::ffi::c_int as uint16_t,
            1934 as ::core::ffi::c_int as uint16_t,
            1208 as ::core::ffi::c_int as uint16_t,
            603 as ::core::ffi::c_int as uint16_t,
            1580 as ::core::ffi::c_int as uint16_t,
            1099 as ::core::ffi::c_int as uint16_t,
            1398 as ::core::ffi::c_int as uint16_t,
            1995 as ::core::ffi::c_int as uint16_t,
        ],
        [
            786 as ::core::ffi::c_int as uint16_t,
            789 as ::core::ffi::c_int as uint16_t,
            942 as ::core::ffi::c_int as uint16_t,
            581 as ::core::ffi::c_int as uint16_t,
            1018 as ::core::ffi::c_int as uint16_t,
            951 as ::core::ffi::c_int as uint16_t,
            1599 as ::core::ffi::c_int as uint16_t,
            1207 as ::core::ffi::c_int as uint16_t,
            731 as ::core::ffi::c_int as uint16_t,
            768 as ::core::ffi::c_int as uint16_t,
        ],
        [
            690 as ::core::ffi::c_int as uint16_t,
            1015 as ::core::ffi::c_int as uint16_t,
            672 as ::core::ffi::c_int as uint16_t,
            1078 as ::core::ffi::c_int as uint16_t,
            582 as ::core::ffi::c_int as uint16_t,
            504 as ::core::ffi::c_int as uint16_t,
            1693 as ::core::ffi::c_int as uint16_t,
            1438 as ::core::ffi::c_int as uint16_t,
            1108 as ::core::ffi::c_int as uint16_t,
            2897 as ::core::ffi::c_int as uint16_t,
        ],
        [
            768 as ::core::ffi::c_int as uint16_t,
            1267 as ::core::ffi::c_int as uint16_t,
            571 as ::core::ffi::c_int as uint16_t,
            2005 as ::core::ffi::c_int as uint16_t,
            1243 as ::core::ffi::c_int as uint16_t,
            244 as ::core::ffi::c_int as uint16_t,
            2881 as ::core::ffi::c_int as uint16_t,
            1380 as ::core::ffi::c_int as uint16_t,
            1786 as ::core::ffi::c_int as uint16_t,
            1453 as ::core::ffi::c_int as uint16_t,
        ],
        [
            452 as ::core::ffi::c_int as uint16_t,
            899 as ::core::ffi::c_int as uint16_t,
            1293 as ::core::ffi::c_int as uint16_t,
            903 as ::core::ffi::c_int as uint16_t,
            1311 as ::core::ffi::c_int as uint16_t,
            3100 as ::core::ffi::c_int as uint16_t,
            465 as ::core::ffi::c_int as uint16_t,
            1311 as ::core::ffi::c_int as uint16_t,
            1319 as ::core::ffi::c_int as uint16_t,
            813 as ::core::ffi::c_int as uint16_t,
        ],
        [
            394 as ::core::ffi::c_int as uint16_t,
            927 as ::core::ffi::c_int as uint16_t,
            942 as ::core::ffi::c_int as uint16_t,
            1103 as ::core::ffi::c_int as uint16_t,
            1358 as ::core::ffi::c_int as uint16_t,
            1104 as ::core::ffi::c_int as uint16_t,
            946 as ::core::ffi::c_int as uint16_t,
            593 as ::core::ffi::c_int as uint16_t,
            1363 as ::core::ffi::c_int as uint16_t,
            1109 as ::core::ffi::c_int as uint16_t,
        ],
        [
            559 as ::core::ffi::c_int as uint16_t,
            1005 as ::core::ffi::c_int as uint16_t,
            1007 as ::core::ffi::c_int as uint16_t,
            1016 as ::core::ffi::c_int as uint16_t,
            658 as ::core::ffi::c_int as uint16_t,
            1173 as ::core::ffi::c_int as uint16_t,
            1021 as ::core::ffi::c_int as uint16_t,
            1164 as ::core::ffi::c_int as uint16_t,
            623 as ::core::ffi::c_int as uint16_t,
            1028 as ::core::ffi::c_int as uint16_t,
        ],
        [
            564 as ::core::ffi::c_int as uint16_t,
            796 as ::core::ffi::c_int as uint16_t,
            632 as ::core::ffi::c_int as uint16_t,
            1005 as ::core::ffi::c_int as uint16_t,
            1014 as ::core::ffi::c_int as uint16_t,
            863 as ::core::ffi::c_int as uint16_t,
            2316 as ::core::ffi::c_int as uint16_t,
            1268 as ::core::ffi::c_int as uint16_t,
            938 as ::core::ffi::c_int as uint16_t,
            764 as ::core::ffi::c_int as uint16_t,
        ],
    ],
    [
        [
            266 as ::core::ffi::c_int as uint16_t,
            606 as ::core::ffi::c_int as uint16_t,
            1098 as ::core::ffi::c_int as uint16_t,
            1228 as ::core::ffi::c_int as uint16_t,
            1497 as ::core::ffi::c_int as uint16_t,
            1243 as ::core::ffi::c_int as uint16_t,
            948 as ::core::ffi::c_int as uint16_t,
            1030 as ::core::ffi::c_int as uint16_t,
            1734 as ::core::ffi::c_int as uint16_t,
            1461 as ::core::ffi::c_int as uint16_t,
        ],
        [
            366 as ::core::ffi::c_int as uint16_t,
            585 as ::core::ffi::c_int as uint16_t,
            901 as ::core::ffi::c_int as uint16_t,
            1060 as ::core::ffi::c_int as uint16_t,
            1407 as ::core::ffi::c_int as uint16_t,
            1247 as ::core::ffi::c_int as uint16_t,
            876 as ::core::ffi::c_int as uint16_t,
            1134 as ::core::ffi::c_int as uint16_t,
            1620 as ::core::ffi::c_int as uint16_t,
            1054 as ::core::ffi::c_int as uint16_t,
        ],
        [
            452 as ::core::ffi::c_int as uint16_t,
            565 as ::core::ffi::c_int as uint16_t,
            542 as ::core::ffi::c_int as uint16_t,
            1729 as ::core::ffi::c_int as uint16_t,
            1479 as ::core::ffi::c_int as uint16_t,
            1479 as ::core::ffi::c_int as uint16_t,
            1016 as ::core::ffi::c_int as uint16_t,
            886 as ::core::ffi::c_int as uint16_t,
            2938 as ::core::ffi::c_int as uint16_t,
            1150 as ::core::ffi::c_int as uint16_t,
        ],
        [
            555 as ::core::ffi::c_int as uint16_t,
            1088 as ::core::ffi::c_int as uint16_t,
            1533 as ::core::ffi::c_int as uint16_t,
            950 as ::core::ffi::c_int as uint16_t,
            1354 as ::core::ffi::c_int as uint16_t,
            895 as ::core::ffi::c_int as uint16_t,
            834 as ::core::ffi::c_int as uint16_t,
            1019 as ::core::ffi::c_int as uint16_t,
            1021 as ::core::ffi::c_int as uint16_t,
            496 as ::core::ffi::c_int as uint16_t,
        ],
        [
            704 as ::core::ffi::c_int as uint16_t,
            815 as ::core::ffi::c_int as uint16_t,
            1193 as ::core::ffi::c_int as uint16_t,
            971 as ::core::ffi::c_int as uint16_t,
            973 as ::core::ffi::c_int as uint16_t,
            640 as ::core::ffi::c_int as uint16_t,
            1217 as ::core::ffi::c_int as uint16_t,
            2214 as ::core::ffi::c_int as uint16_t,
            832 as ::core::ffi::c_int as uint16_t,
            578 as ::core::ffi::c_int as uint16_t,
        ],
        [
            672 as ::core::ffi::c_int as uint16_t,
            1245 as ::core::ffi::c_int as uint16_t,
            579 as ::core::ffi::c_int as uint16_t,
            871 as ::core::ffi::c_int as uint16_t,
            875 as ::core::ffi::c_int as uint16_t,
            774 as ::core::ffi::c_int as uint16_t,
            872 as ::core::ffi::c_int as uint16_t,
            1273 as ::core::ffi::c_int as uint16_t,
            1027 as ::core::ffi::c_int as uint16_t,
            949 as ::core::ffi::c_int as uint16_t,
        ],
        [
            296 as ::core::ffi::c_int as uint16_t,
            1134 as ::core::ffi::c_int as uint16_t,
            2050 as ::core::ffi::c_int as uint16_t,
            1784 as ::core::ffi::c_int as uint16_t,
            1636 as ::core::ffi::c_int as uint16_t,
            3425 as ::core::ffi::c_int as uint16_t,
            442 as ::core::ffi::c_int as uint16_t,
            1550 as ::core::ffi::c_int as uint16_t,
            2076 as ::core::ffi::c_int as uint16_t,
            722 as ::core::ffi::c_int as uint16_t,
        ],
        [
            342 as ::core::ffi::c_int as uint16_t,
            982 as ::core::ffi::c_int as uint16_t,
            1259 as ::core::ffi::c_int as uint16_t,
            1846 as ::core::ffi::c_int as uint16_t,
            1848 as ::core::ffi::c_int as uint16_t,
            1848 as ::core::ffi::c_int as uint16_t,
            622 as ::core::ffi::c_int as uint16_t,
            568 as ::core::ffi::c_int as uint16_t,
            1847 as ::core::ffi::c_int as uint16_t,
            1052 as ::core::ffi::c_int as uint16_t,
        ],
        [
            555 as ::core::ffi::c_int as uint16_t,
            1064 as ::core::ffi::c_int as uint16_t,
            1304 as ::core::ffi::c_int as uint16_t,
            828 as ::core::ffi::c_int as uint16_t,
            746 as ::core::ffi::c_int as uint16_t,
            1343 as ::core::ffi::c_int as uint16_t,
            1075 as ::core::ffi::c_int as uint16_t,
            1329 as ::core::ffi::c_int as uint16_t,
            1078 as ::core::ffi::c_int as uint16_t,
            494 as ::core::ffi::c_int as uint16_t,
        ],
        [
            288 as ::core::ffi::c_int as uint16_t,
            1167 as ::core::ffi::c_int as uint16_t,
            1285 as ::core::ffi::c_int as uint16_t,
            1174 as ::core::ffi::c_int as uint16_t,
            1639 as ::core::ffi::c_int as uint16_t,
            1639 as ::core::ffi::c_int as uint16_t,
            833 as ::core::ffi::c_int as uint16_t,
            2254 as ::core::ffi::c_int as uint16_t,
            1304 as ::core::ffi::c_int as uint16_t,
            509 as ::core::ffi::c_int as uint16_t,
        ],
    ],
    [
        [
            342 as ::core::ffi::c_int as uint16_t,
            719 as ::core::ffi::c_int as uint16_t,
            767 as ::core::ffi::c_int as uint16_t,
            1866 as ::core::ffi::c_int as uint16_t,
            1757 as ::core::ffi::c_int as uint16_t,
            1270 as ::core::ffi::c_int as uint16_t,
            1246 as ::core::ffi::c_int as uint16_t,
            550 as ::core::ffi::c_int as uint16_t,
            1746 as ::core::ffi::c_int as uint16_t,
            2151 as ::core::ffi::c_int as uint16_t,
        ],
        [
            483 as ::core::ffi::c_int as uint16_t,
            653 as ::core::ffi::c_int as uint16_t,
            694 as ::core::ffi::c_int as uint16_t,
            1509 as ::core::ffi::c_int as uint16_t,
            1459 as ::core::ffi::c_int as uint16_t,
            1410 as ::core::ffi::c_int as uint16_t,
            1218 as ::core::ffi::c_int as uint16_t,
            507 as ::core::ffi::c_int as uint16_t,
            1914 as ::core::ffi::c_int as uint16_t,
            1266 as ::core::ffi::c_int as uint16_t,
        ],
        [
            488 as ::core::ffi::c_int as uint16_t,
            757 as ::core::ffi::c_int as uint16_t,
            447 as ::core::ffi::c_int as uint16_t,
            2979 as ::core::ffi::c_int as uint16_t,
            1813 as ::core::ffi::c_int as uint16_t,
            1268 as ::core::ffi::c_int as uint16_t,
            1654 as ::core::ffi::c_int as uint16_t,
            539 as ::core::ffi::c_int as uint16_t,
            1849 as ::core::ffi::c_int as uint16_t,
            2109 as ::core::ffi::c_int as uint16_t,
        ],
        [
            522 as ::core::ffi::c_int as uint16_t,
            1097 as ::core::ffi::c_int as uint16_t,
            1085 as ::core::ffi::c_int as uint16_t,
            851 as ::core::ffi::c_int as uint16_t,
            1365 as ::core::ffi::c_int as uint16_t,
            1111 as ::core::ffi::c_int as uint16_t,
            851 as ::core::ffi::c_int as uint16_t,
            901 as ::core::ffi::c_int as uint16_t,
            961 as ::core::ffi::c_int as uint16_t,
            605 as ::core::ffi::c_int as uint16_t,
        ],
        [
            709 as ::core::ffi::c_int as uint16_t,
            716 as ::core::ffi::c_int as uint16_t,
            841 as ::core::ffi::c_int as uint16_t,
            728 as ::core::ffi::c_int as uint16_t,
            736 as ::core::ffi::c_int as uint16_t,
            945 as ::core::ffi::c_int as uint16_t,
            941 as ::core::ffi::c_int as uint16_t,
            862 as ::core::ffi::c_int as uint16_t,
            2845 as ::core::ffi::c_int as uint16_t,
            1057 as ::core::ffi::c_int as uint16_t,
        ],
        [
            512 as ::core::ffi::c_int as uint16_t,
            1323 as ::core::ffi::c_int as uint16_t,
            500 as ::core::ffi::c_int as uint16_t,
            1336 as ::core::ffi::c_int as uint16_t,
            1083 as ::core::ffi::c_int as uint16_t,
            681 as ::core::ffi::c_int as uint16_t,
            1342 as ::core::ffi::c_int as uint16_t,
            717 as ::core::ffi::c_int as uint16_t,
            1604 as ::core::ffi::c_int as uint16_t,
            1350 as ::core::ffi::c_int as uint16_t,
        ],
        [
            452 as ::core::ffi::c_int as uint16_t,
            1155 as ::core::ffi::c_int as uint16_t,
            1372 as ::core::ffi::c_int as uint16_t,
            1900 as ::core::ffi::c_int as uint16_t,
            1501 as ::core::ffi::c_int as uint16_t,
            3290 as ::core::ffi::c_int as uint16_t,
            311 as ::core::ffi::c_int as uint16_t,
            944 as ::core::ffi::c_int as uint16_t,
            1919 as ::core::ffi::c_int as uint16_t,
            922 as ::core::ffi::c_int as uint16_t,
        ],
        [
            403 as ::core::ffi::c_int as uint16_t,
            1520 as ::core::ffi::c_int as uint16_t,
            977 as ::core::ffi::c_int as uint16_t,
            2132 as ::core::ffi::c_int as uint16_t,
            1733 as ::core::ffi::c_int as uint16_t,
            3522 as ::core::ffi::c_int as uint16_t,
            1076 as ::core::ffi::c_int as uint16_t,
            276 as ::core::ffi::c_int as uint16_t,
            3335 as ::core::ffi::c_int as uint16_t,
            1547 as ::core::ffi::c_int as uint16_t,
        ],
        [
            559 as ::core::ffi::c_int as uint16_t,
            1374 as ::core::ffi::c_int as uint16_t,
            1101 as ::core::ffi::c_int as uint16_t,
            615 as ::core::ffi::c_int as uint16_t,
            673 as ::core::ffi::c_int as uint16_t,
            2462 as ::core::ffi::c_int as uint16_t,
            974 as ::core::ffi::c_int as uint16_t,
            795 as ::core::ffi::c_int as uint16_t,
            984 as ::core::ffi::c_int as uint16_t,
            984 as ::core::ffi::c_int as uint16_t,
        ],
        [
            547 as ::core::ffi::c_int as uint16_t,
            1122 as ::core::ffi::c_int as uint16_t,
            1062 as ::core::ffi::c_int as uint16_t,
            812 as ::core::ffi::c_int as uint16_t,
            1410 as ::core::ffi::c_int as uint16_t,
            951 as ::core::ffi::c_int as uint16_t,
            1140 as ::core::ffi::c_int as uint16_t,
            622 as ::core::ffi::c_int as uint16_t,
            1268 as ::core::ffi::c_int as uint16_t,
            651 as ::core::ffi::c_int as uint16_t,
        ],
    ],
    [
        [
            165 as ::core::ffi::c_int as uint16_t,
            982 as ::core::ffi::c_int as uint16_t,
            1235 as ::core::ffi::c_int as uint16_t,
            938 as ::core::ffi::c_int as uint16_t,
            1334 as ::core::ffi::c_int as uint16_t,
            1366 as ::core::ffi::c_int as uint16_t,
            1659 as ::core::ffi::c_int as uint16_t,
            1578 as ::core::ffi::c_int as uint16_t,
            964 as ::core::ffi::c_int as uint16_t,
            1612 as ::core::ffi::c_int as uint16_t,
        ],
        [
            592 as ::core::ffi::c_int as uint16_t,
            422 as ::core::ffi::c_int as uint16_t,
            925 as ::core::ffi::c_int as uint16_t,
            847 as ::core::ffi::c_int as uint16_t,
            1139 as ::core::ffi::c_int as uint16_t,
            1112 as ::core::ffi::c_int as uint16_t,
            1387 as ::core::ffi::c_int as uint16_t,
            2036 as ::core::ffi::c_int as uint16_t,
            861 as ::core::ffi::c_int as uint16_t,
            1041 as ::core::ffi::c_int as uint16_t,
        ],
        [
            403 as ::core::ffi::c_int as uint16_t,
            837 as ::core::ffi::c_int as uint16_t,
            732 as ::core::ffi::c_int as uint16_t,
            770 as ::core::ffi::c_int as uint16_t,
            941 as ::core::ffi::c_int as uint16_t,
            1658 as ::core::ffi::c_int as uint16_t,
            1250 as ::core::ffi::c_int as uint16_t,
            809 as ::core::ffi::c_int as uint16_t,
            1407 as ::core::ffi::c_int as uint16_t,
            1407 as ::core::ffi::c_int as uint16_t,
        ],
        [
            896 as ::core::ffi::c_int as uint16_t,
            874 as ::core::ffi::c_int as uint16_t,
            1071 as ::core::ffi::c_int as uint16_t,
            381 as ::core::ffi::c_int as uint16_t,
            1568 as ::core::ffi::c_int as uint16_t,
            1722 as ::core::ffi::c_int as uint16_t,
            1437 as ::core::ffi::c_int as uint16_t,
            2192 as ::core::ffi::c_int as uint16_t,
            480 as ::core::ffi::c_int as uint16_t,
            1035 as ::core::ffi::c_int as uint16_t,
        ],
        [
            640 as ::core::ffi::c_int as uint16_t,
            1098 as ::core::ffi::c_int as uint16_t,
            1012 as ::core::ffi::c_int as uint16_t,
            1032 as ::core::ffi::c_int as uint16_t,
            684 as ::core::ffi::c_int as uint16_t,
            1382 as ::core::ffi::c_int as uint16_t,
            1581 as ::core::ffi::c_int as uint16_t,
            2106 as ::core::ffi::c_int as uint16_t,
            416 as ::core::ffi::c_int as uint16_t,
            865 as ::core::ffi::c_int as uint16_t,
        ],
        [
            559 as ::core::ffi::c_int as uint16_t,
            1005 as ::core::ffi::c_int as uint16_t,
            819 as ::core::ffi::c_int as uint16_t,
            914 as ::core::ffi::c_int as uint16_t,
            710 as ::core::ffi::c_int as uint16_t,
            770 as ::core::ffi::c_int as uint16_t,
            1418 as ::core::ffi::c_int as uint16_t,
            920 as ::core::ffi::c_int as uint16_t,
            838 as ::core::ffi::c_int as uint16_t,
            1435 as ::core::ffi::c_int as uint16_t,
        ],
        [
            415 as ::core::ffi::c_int as uint16_t,
            1258 as ::core::ffi::c_int as uint16_t,
            1245 as ::core::ffi::c_int as uint16_t,
            870 as ::core::ffi::c_int as uint16_t,
            1278 as ::core::ffi::c_int as uint16_t,
            3067 as ::core::ffi::c_int as uint16_t,
            770 as ::core::ffi::c_int as uint16_t,
            1021 as ::core::ffi::c_int as uint16_t,
            1287 as ::core::ffi::c_int as uint16_t,
            522 as ::core::ffi::c_int as uint16_t,
        ],
        [
            406 as ::core::ffi::c_int as uint16_t,
            990 as ::core::ffi::c_int as uint16_t,
            601 as ::core::ffi::c_int as uint16_t,
            1009 as ::core::ffi::c_int as uint16_t,
            1265 as ::core::ffi::c_int as uint16_t,
            1265 as ::core::ffi::c_int as uint16_t,
            1267 as ::core::ffi::c_int as uint16_t,
            759 as ::core::ffi::c_int as uint16_t,
            1017 as ::core::ffi::c_int as uint16_t,
            1277 as ::core::ffi::c_int as uint16_t,
        ],
        [
            968 as ::core::ffi::c_int as uint16_t,
            1182 as ::core::ffi::c_int as uint16_t,
            1329 as ::core::ffi::c_int as uint16_t,
            788 as ::core::ffi::c_int as uint16_t,
            1032 as ::core::ffi::c_int as uint16_t,
            1292 as ::core::ffi::c_int as uint16_t,
            1705 as ::core::ffi::c_int as uint16_t,
            1714 as ::core::ffi::c_int as uint16_t,
            203 as ::core::ffi::c_int as uint16_t,
            1403 as ::core::ffi::c_int as uint16_t,
        ],
        [
            732 as ::core::ffi::c_int as uint16_t,
            877 as ::core::ffi::c_int as uint16_t,
            1279 as ::core::ffi::c_int as uint16_t,
            471 as ::core::ffi::c_int as uint16_t,
            901 as ::core::ffi::c_int as uint16_t,
            1161 as ::core::ffi::c_int as uint16_t,
            1545 as ::core::ffi::c_int as uint16_t,
            1294 as ::core::ffi::c_int as uint16_t,
            755 as ::core::ffi::c_int as uint16_t,
            755 as ::core::ffi::c_int as uint16_t,
        ],
    ],
    [
        [
            111 as ::core::ffi::c_int as uint16_t,
            931 as ::core::ffi::c_int as uint16_t,
            1378 as ::core::ffi::c_int as uint16_t,
            1185 as ::core::ffi::c_int as uint16_t,
            1933 as ::core::ffi::c_int as uint16_t,
            1648 as ::core::ffi::c_int as uint16_t,
            1148 as ::core::ffi::c_int as uint16_t,
            1714 as ::core::ffi::c_int as uint16_t,
            1873 as ::core::ffi::c_int as uint16_t,
            1307 as ::core::ffi::c_int as uint16_t,
        ],
        [
            406 as ::core::ffi::c_int as uint16_t,
            414 as ::core::ffi::c_int as uint16_t,
            1030 as ::core::ffi::c_int as uint16_t,
            1023 as ::core::ffi::c_int as uint16_t,
            1910 as ::core::ffi::c_int as uint16_t,
            1404 as ::core::ffi::c_int as uint16_t,
            1313 as ::core::ffi::c_int as uint16_t,
            1647 as ::core::ffi::c_int as uint16_t,
            1509 as ::core::ffi::c_int as uint16_t,
            793 as ::core::ffi::c_int as uint16_t,
        ],
        [
            342 as ::core::ffi::c_int as uint16_t,
            640 as ::core::ffi::c_int as uint16_t,
            575 as ::core::ffi::c_int as uint16_t,
            1088 as ::core::ffi::c_int as uint16_t,
            1241 as ::core::ffi::c_int as uint16_t,
            1349 as ::core::ffi::c_int as uint16_t,
            1161 as ::core::ffi::c_int as uint16_t,
            1350 as ::core::ffi::c_int as uint16_t,
            1756 as ::core::ffi::c_int as uint16_t,
            1502 as ::core::ffi::c_int as uint16_t,
        ],
        [
            559 as ::core::ffi::c_int as uint16_t,
            766 as ::core::ffi::c_int as uint16_t,
            1185 as ::core::ffi::c_int as uint16_t,
            357 as ::core::ffi::c_int as uint16_t,
            1682 as ::core::ffi::c_int as uint16_t,
            1428 as ::core::ffi::c_int as uint16_t,
            1329 as ::core::ffi::c_int as uint16_t,
            1897 as ::core::ffi::c_int as uint16_t,
            1219 as ::core::ffi::c_int as uint16_t,
            802 as ::core::ffi::c_int as uint16_t,
        ],
        [
            473 as ::core::ffi::c_int as uint16_t,
            909 as ::core::ffi::c_int as uint16_t,
            1164 as ::core::ffi::c_int as uint16_t,
            771 as ::core::ffi::c_int as uint16_t,
            719 as ::core::ffi::c_int as uint16_t,
            2508 as ::core::ffi::c_int as uint16_t,
            1427 as ::core::ffi::c_int as uint16_t,
            1432 as ::core::ffi::c_int as uint16_t,
            722 as ::core::ffi::c_int as uint16_t,
            782 as ::core::ffi::c_int as uint16_t,
        ],
        [
            342 as ::core::ffi::c_int as uint16_t,
            892 as ::core::ffi::c_int as uint16_t,
            785 as ::core::ffi::c_int as uint16_t,
            1145 as ::core::ffi::c_int as uint16_t,
            1150 as ::core::ffi::c_int as uint16_t,
            794 as ::core::ffi::c_int as uint16_t,
            1296 as ::core::ffi::c_int as uint16_t,
            1550 as ::core::ffi::c_int as uint16_t,
            973 as ::core::ffi::c_int as uint16_t,
            1057 as ::core::ffi::c_int as uint16_t,
        ],
        [
            208 as ::core::ffi::c_int as uint16_t,
            1036 as ::core::ffi::c_int as uint16_t,
            1326 as ::core::ffi::c_int as uint16_t,
            1343 as ::core::ffi::c_int as uint16_t,
            1606 as ::core::ffi::c_int as uint16_t,
            3395 as ::core::ffi::c_int as uint16_t,
            815 as ::core::ffi::c_int as uint16_t,
            1455 as ::core::ffi::c_int as uint16_t,
            1618 as ::core::ffi::c_int as uint16_t,
            712 as ::core::ffi::c_int as uint16_t,
        ],
        [
            228 as ::core::ffi::c_int as uint16_t,
            928 as ::core::ffi::c_int as uint16_t,
            890 as ::core::ffi::c_int as uint16_t,
            1046 as ::core::ffi::c_int as uint16_t,
            3499 as ::core::ffi::c_int as uint16_t,
            1711 as ::core::ffi::c_int as uint16_t,
            994 as ::core::ffi::c_int as uint16_t,
            829 as ::core::ffi::c_int as uint16_t,
            1720 as ::core::ffi::c_int as uint16_t,
            1318 as ::core::ffi::c_int as uint16_t,
        ],
        [
            768 as ::core::ffi::c_int as uint16_t,
            724 as ::core::ffi::c_int as uint16_t,
            1058 as ::core::ffi::c_int as uint16_t,
            636 as ::core::ffi::c_int as uint16_t,
            991 as ::core::ffi::c_int as uint16_t,
            1075 as ::core::ffi::c_int as uint16_t,
            1319 as ::core::ffi::c_int as uint16_t,
            1324 as ::core::ffi::c_int as uint16_t,
            616 as ::core::ffi::c_int as uint16_t,
            825 as ::core::ffi::c_int as uint16_t,
        ],
        [
            305 as ::core::ffi::c_int as uint16_t,
            1167 as ::core::ffi::c_int as uint16_t,
            1358 as ::core::ffi::c_int as uint16_t,
            899 as ::core::ffi::c_int as uint16_t,
            1587 as ::core::ffi::c_int as uint16_t,
            1587 as ::core::ffi::c_int as uint16_t,
            987 as ::core::ffi::c_int as uint16_t,
            1988 as ::core::ffi::c_int as uint16_t,
            1332 as ::core::ffi::c_int as uint16_t,
            501 as ::core::ffi::c_int as uint16_t,
        ],
    ],
];
#[no_mangle]
pub unsafe extern "C" fn VP8InitResidual(
    mut first: ::core::ffi::c_int,
    mut coeff_type: ::core::ffi::c_int,
    enc: *mut VP8Encoder,
    res: *mut VP8Residual,
) {
    (*res).coeff_type = coeff_type;
    (*res).prob = &raw mut *(&raw mut (*enc).proba_.coeffs_ as *mut [ProbaArray; 8])
        .offset(coeff_type as isize) as *mut ProbaArray;
    (*res).stats = &raw mut *(&raw mut (*enc).proba_.stats_ as *mut [StatsArray; 8])
        .offset(coeff_type as isize) as *mut StatsArray;
    (*res).costs = &raw mut *(&raw mut (*enc).proba_.remapped_costs_ as *mut CostArrayMap)
        .offset(coeff_type as isize) as *mut [*const uint16_t; 3]
        as CostArrayPtr;
    (*res).first = first;
}
#[no_mangle]
pub unsafe extern "C" fn VP8GetCostLuma4(
    it: *mut VP8EncIterator,
    mut levels: *const int16_t,
) -> ::core::ffi::c_int {
    let x: ::core::ffi::c_int = (*it).i4_ & 3 as ::core::ffi::c_int;
    let y: ::core::ffi::c_int = (*it).i4_ >> 2 as ::core::ffi::c_int;
    let mut res: VP8Residual = VP8Residual {
        first: 0,
        last: 0,
        coeffs: ::core::ptr::null::<int16_t>(),
        coeff_type: 0,
        prob: ::core::ptr::null_mut::<ProbaArray>(),
        stats: ::core::ptr::null_mut::<StatsArray>(),
        costs: ::core::ptr::null_mut::<[*const uint16_t; 3]>(),
    };
    let enc: *mut VP8Encoder = (*it).enc_;
    let mut R: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ctx: ::core::ffi::c_int = 0;
    VP8InitResidual(
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        enc,
        &raw mut res,
    );
    ctx = (*it).top_nz_[x as usize] + (*it).left_nz_[y as usize];
    VP8SetResidualCoeffs.expect("non-null function pointer")(
        levels as *const int16_t,
        &raw mut res,
    );
    R += VP8GetResidualCost.expect("non-null function pointer")(ctx, &raw mut res);
    return R;
}
#[no_mangle]
pub unsafe extern "C" fn VP8GetCostLuma16(
    it: *mut VP8EncIterator,
    rd: *const VP8ModeScore,
) -> ::core::ffi::c_int {
    let mut res: VP8Residual = VP8Residual {
        first: 0,
        last: 0,
        coeffs: ::core::ptr::null::<int16_t>(),
        coeff_type: 0,
        prob: ::core::ptr::null_mut::<ProbaArray>(),
        stats: ::core::ptr::null_mut::<StatsArray>(),
        costs: ::core::ptr::null_mut::<[*const uint16_t; 3]>(),
    };
    let enc: *mut VP8Encoder = (*it).enc_;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut R: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    VP8IteratorNzToBytes(it);
    VP8InitResidual(
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        enc,
        &raw mut res,
    );
    VP8SetResidualCoeffs.expect("non-null function pointer")(
        &raw const (*rd).y_dc_levels as *const int16_t,
        &raw mut res,
    );
    R += VP8GetResidualCost.expect("non-null function pointer")(
        (*it).top_nz_[8 as ::core::ffi::c_int as usize]
            + (*it).left_nz_[8 as ::core::ffi::c_int as usize],
        &raw mut res,
    );
    VP8InitResidual(
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        enc,
        &raw mut res,
    );
    y = 0 as ::core::ffi::c_int;
    while y < 4 as ::core::ffi::c_int {
        x = 0 as ::core::ffi::c_int;
        while x < 4 as ::core::ffi::c_int {
            let ctx: ::core::ffi::c_int = (*it).top_nz_[x as usize] + (*it).left_nz_[y as usize];
            VP8SetResidualCoeffs.expect("non-null function pointer")(
                &raw const *(&raw const (*rd).y_ac_levels as *const [int16_t; 16])
                    .offset((x + y * 4 as ::core::ffi::c_int) as isize)
                    as *const int16_t,
                &raw mut res,
            );
            R += VP8GetResidualCost.expect("non-null function pointer")(ctx, &raw mut res);
            (*it).left_nz_[y as usize] =
                (res.last >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            (*it).top_nz_[x as usize] = (*it).left_nz_[y as usize];
            x += 1;
        }
        y += 1;
    }
    return R;
}
#[no_mangle]
pub unsafe extern "C" fn VP8GetCostUV(
    it: *mut VP8EncIterator,
    rd: *const VP8ModeScore,
) -> ::core::ffi::c_int {
    let mut res: VP8Residual = VP8Residual {
        first: 0,
        last: 0,
        coeffs: ::core::ptr::null::<int16_t>(),
        coeff_type: 0,
        prob: ::core::ptr::null_mut::<ProbaArray>(),
        stats: ::core::ptr::null_mut::<StatsArray>(),
        costs: ::core::ptr::null_mut::<[*const uint16_t; 3]>(),
    };
    let enc: *mut VP8Encoder = (*it).enc_;
    let mut ch: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut R: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    VP8IteratorNzToBytes(it);
    VP8InitResidual(
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        enc,
        &raw mut res,
    );
    ch = 0 as ::core::ffi::c_int;
    while ch <= 2 as ::core::ffi::c_int {
        y = 0 as ::core::ffi::c_int;
        while y < 2 as ::core::ffi::c_int {
            x = 0 as ::core::ffi::c_int;
            while x < 2 as ::core::ffi::c_int {
                let ctx: ::core::ffi::c_int = (*it).top_nz_
                    [(4 as ::core::ffi::c_int + ch + x) as usize]
                    + (*it).left_nz_[(4 as ::core::ffi::c_int + ch + y) as usize];
                VP8SetResidualCoeffs.expect("non-null function pointer")(
                    &raw const *(&raw const (*rd).uv_levels as *const [int16_t; 16]).offset(
                        (ch * 2 as ::core::ffi::c_int + x + y * 2 as ::core::ffi::c_int) as isize,
                    ) as *const int16_t,
                    &raw mut res,
                );
                R += VP8GetResidualCost.expect("non-null function pointer")(ctx, &raw mut res);
                (*it).left_nz_[(4 as ::core::ffi::c_int + ch + y) as usize] =
                    (res.last >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                (*it).top_nz_[(4 as ::core::ffi::c_int + ch + x) as usize] =
                    (*it).left_nz_[(4 as ::core::ffi::c_int + ch + y) as usize];
                x += 1;
            }
            y += 1;
        }
        ch += 2 as ::core::ffi::c_int;
    }
    return R;
}
#[no_mangle]
pub unsafe extern "C" fn VP8RecordCoeffs(
    mut ctx: ::core::ffi::c_int,
    res: *const VP8Residual,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = (*res).first;
    let mut s: *mut proba_t = &raw mut *(&raw mut *(*res).stats.offset(n as isize)
        as *mut [proba_t; 11])
        .offset(ctx as isize) as *mut proba_t;
    if (*res).last < 0 as ::core::ffi::c_int {
        VP8RecordStats(
            0 as ::core::ffi::c_int,
            s.offset(0 as ::core::ffi::c_int as isize),
        );
        return 0 as ::core::ffi::c_int;
    }
    while n <= (*res).last {
        let mut v: ::core::ffi::c_int = 0;
        VP8RecordStats(
            1 as ::core::ffi::c_int,
            s.offset(0 as ::core::ffi::c_int as isize),
        );
        loop {
            let fresh0 = n;
            n = n + 1;
            v = *(*res).coeffs.offset(fresh0 as isize) as ::core::ffi::c_int;
            if !(v == 0 as ::core::ffi::c_int) {
                break;
            }
            VP8RecordStats(
                0 as ::core::ffi::c_int,
                s.offset(1 as ::core::ffi::c_int as isize),
            );
            s = &raw mut *(&raw mut *(*res)
                .stats
                .offset(*(&raw const VP8EncBands as *const uint8_t).offset(n as isize) as isize)
                as *mut [proba_t; 11])
                .offset(0 as ::core::ffi::c_int as isize) as *mut proba_t;
        }
        VP8RecordStats(
            1 as ::core::ffi::c_int,
            s.offset(1 as ::core::ffi::c_int as isize),
        );
        if VP8RecordStats(
            ((2 as ::core::ffi::c_uint) < (v + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                as ::core::ffi::c_int,
            s.offset(2 as ::core::ffi::c_int as isize),
        ) == 0
        {
            s = &raw mut *(&raw mut *(*res)
                .stats
                .offset(*(&raw const VP8EncBands as *const uint8_t).offset(n as isize) as isize)
                as *mut [proba_t; 11])
                .offset(1 as ::core::ffi::c_int as isize) as *mut proba_t;
        } else {
            v = abs(v);
            if v > MAX_VARIABLE_LEVEL as ::core::ffi::c_int {
                v = MAX_VARIABLE_LEVEL as ::core::ffi::c_int;
            }
            let bits: ::core::ffi::c_int = VP8LevelCodes[(v - 1 as ::core::ffi::c_int) as usize]
                [1 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int;
            let mut pattern: ::core::ffi::c_int = VP8LevelCodes
                [(v - 1 as ::core::ffi::c_int) as usize][0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int;
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            loop {
                pattern >>= 1 as ::core::ffi::c_int;
                if !(pattern != 0 as ::core::ffi::c_int) {
                    break;
                }
                let mask: ::core::ffi::c_int = (2 as ::core::ffi::c_int) << i;
                if pattern & 1 as ::core::ffi::c_int != 0 {
                    VP8RecordStats(
                        (bits & mask != 0) as ::core::ffi::c_int,
                        s.offset(3 as ::core::ffi::c_int as isize)
                            .offset(i as isize),
                    );
                }
                i += 1;
            }
            s = &raw mut *(&raw mut *(*res)
                .stats
                .offset(*(&raw const VP8EncBands as *const uint8_t).offset(n as isize) as isize)
                as *mut [proba_t; 11])
                .offset(2 as ::core::ffi::c_int as isize) as *mut proba_t;
        }
    }
    if n < 16 as ::core::ffi::c_int {
        VP8RecordStats(
            0 as ::core::ffi::c_int,
            s.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    return 1 as ::core::ffi::c_int;
}
