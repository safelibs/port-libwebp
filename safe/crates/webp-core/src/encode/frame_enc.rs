use c2rust_bitfields::BitfieldStruct;

#[repr(C)]
pub struct VP8Tokens {
    _unused: [u8; 0],
}

extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn log10(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    static mut VP8SSE16x16: VP8Metric;
    static mut VP8SSE8x8: VP8Metric;
    static VP8EntropyCost: [uint16_t; 256];
    static VP8EncBands: [uint8_t; 17];
    static mut VP8SetResidualCoeffs: VP8SetResidualCoeffsFunc;
    fn VP8IteratorInit(enc: *mut VP8Encoder, it: *mut VP8EncIterator);
    fn VP8IteratorImport(it: *mut VP8EncIterator, tmp_32: *mut uint8_t);
    fn VP8IteratorExport(it: *const VP8EncIterator);
    fn VP8IteratorNext(it: *mut VP8EncIterator) -> ::core::ffi::c_int;
    fn VP8IteratorSaveBoundary(it: *mut VP8EncIterator);
    fn VP8IteratorProgress(
        it: *const VP8EncIterator,
        delta: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VP8IteratorNzToBytes(it: *mut VP8EncIterator);
    fn VP8IteratorBytesToNz(it: *mut VP8EncIterator);
    fn VP8TBufferClear(b: *mut VP8TBuffer);
    fn VP8EmitTokens(
        b: *mut VP8TBuffer,
        bw: *mut VP8BitWriter,
        probas: *const uint8_t,
        final_pass: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VP8RecordCoeffTokens(
        ctx: ::core::ffi::c_int,
        res: *const VP8Residual,
        tokens: *mut VP8TBuffer,
    ) -> ::core::ffi::c_int;
    fn VP8EstimateTokenSize(b: *mut VP8TBuffer, probas: *const uint8_t) -> size_t;
    static VP8CoeffsProba0: [[[[uint8_t; 11]; 3]; 8]; 4];
    static VP8CoeffsUpdateProba: [[[[uint8_t; 11]; 3]; 8]; 4];
    fn VP8EncFreeBitWriters(enc: *mut VP8Encoder);
    fn WebPEncodingSetError(
        pic: *const WebPPicture,
        error: WebPEncodingError,
    ) -> ::core::ffi::c_int;
    fn WebPReportProgress(
        pic: *const WebPPicture,
        percent: ::core::ffi::c_int,
        percent_store: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VP8SetSegmentParams(enc: *mut VP8Encoder, quality: ::core::ffi::c_float);
    fn VP8Decimate(
        it: *mut VP8EncIterator,
        rd: *mut VP8ModeScore,
        rd_opt: VP8RDLevel,
    ) -> ::core::ffi::c_int;
    fn VP8InitFilter(it: *mut VP8EncIterator);
    fn VP8StoreFilterStats(it: *mut VP8EncIterator);
    fn VP8AdjustFilterStrength(it: *mut VP8EncIterator);
    fn VP8InitResidual(
        first: ::core::ffi::c_int,
        coeff_type: ::core::ffi::c_int,
        enc: *mut VP8Encoder,
        res: *mut VP8Residual,
    );
    fn VP8RecordCoeffs(ctx: ::core::ffi::c_int, res: *const VP8Residual) -> ::core::ffi::c_int;
    fn VP8CalculateLevelCosts(proba: *mut VP8EncProba);
    fn VP8BitWriterInit(bw: *mut VP8BitWriter, expected_size: size_t) -> ::core::ffi::c_int;
    fn VP8BitWriterFinish(bw: *mut VP8BitWriter) -> *mut uint8_t;
    fn VP8PutBit(
        bw: *mut VP8BitWriter,
        bit: ::core::ffi::c_int,
        prob: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VP8PutBitUniform(bw: *mut VP8BitWriter, bit: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
pub type VP8Metric =
    Option<unsafe extern "C" fn(*const uint8_t, *const uint8_t) -> ::core::ffi::c_int>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PassStats {
    pub is_first: ::core::ffi::c_int,
    pub dq: ::core::ffi::c_float,
    pub q: ::core::ffi::c_float,
    pub last_q: ::core::ffi::c_float,
    pub qmin: ::core::ffi::c_float,
    pub qmax: ::core::ffi::c_float,
    pub value: ::core::ffi::c_double,
    pub last_value: ::core::ffi::c_double,
    pub target: ::core::ffi::c_double,
    pub do_size_search: ::core::ffi::c_int,
}
pub const Y_OFF_ENC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const U_OFF_ENC: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const V_OFF_ENC: ::core::ffi::c_int = 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int;
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
unsafe extern "C" fn VP8BitWriterPos(bw: *const VP8BitWriter) -> uint64_t {
    let nb_bits: uint64_t = (8 as ::core::ffi::c_int + (*bw).nb_bits_) as uint64_t;
    return ((*bw).pos_ as uint64_t)
        .wrapping_add((*bw).run_ as uint64_t)
        .wrapping_mul(8 as uint64_t)
        .wrapping_add(nb_bits);
}
pub const HEADER_SIZE_ESTIMATE: ::core::ffi::c_int =
    RIFF_HEADER_SIZE + CHUNK_HEADER_SIZE + VP8_FRAME_HEADER_SIZE;
pub const DQ_LIMIT: ::core::ffi::c_double = 0.4f64;
pub const PARTITION0_SIZE_LIMIT: ::core::ffi::c_ulonglong = (VP8_MAX_PARTITION0_SIZE
    as ::core::ffi::c_ulonglong)
    .wrapping_sub(2048 as ::core::ffi::c_ulonglong)
    << 11 as ::core::ffi::c_int;
unsafe extern "C" fn Clamp(
    mut v: ::core::ffi::c_float,
    mut min: ::core::ffi::c_float,
    mut max: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    return if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    };
}
unsafe extern "C" fn InitPassStats(
    enc: *const VP8Encoder,
    s: *mut PassStats,
) -> ::core::ffi::c_int {
    let target_size: uint64_t = (*(*enc).config_).target_size as uint64_t;
    let do_size_search: ::core::ffi::c_int = (target_size != 0 as uint64_t) as ::core::ffi::c_int;
    let target_PSNR: ::core::ffi::c_float = (*(*enc).config_).target_PSNR;
    (*s).is_first = 1 as ::core::ffi::c_int;
    (*s).dq = 10.0f32;
    (*s).qmin = 1.0f32 * (*(*enc).config_).qmin as ::core::ffi::c_float;
    (*s).qmax = 1.0f32 * (*(*enc).config_).qmax as ::core::ffi::c_float;
    (*s).last_q = Clamp((*(*enc).config_).quality, (*s).qmin, (*s).qmax);
    (*s).q = (*s).last_q;
    (*s).target = if do_size_search != 0 {
        target_size as ::core::ffi::c_double
    } else if target_PSNR as ::core::ffi::c_double > 0.0f64 {
        target_PSNR as ::core::ffi::c_double
    } else {
        40.0f64
    };
    (*s).last_value = 0.0f64;
    (*s).value = (*s).last_value;
    (*s).do_size_search = do_size_search;
    return do_size_search;
}
unsafe extern "C" fn ComputeNextQ(s: *mut PassStats) -> ::core::ffi::c_float {
    let mut dq: ::core::ffi::c_float = 0.;
    if (*s).is_first != 0 {
        dq = if (*s).value > (*s).target {
            -(*s).dq
        } else {
            (*s).dq
        };
        (*s).is_first = 0 as ::core::ffi::c_int;
    } else if (*s).value != (*s).last_value {
        let slope: ::core::ffi::c_double =
            ((*s).target - (*s).value) / ((*s).last_value - (*s).value);
        dq = (slope * ((*s).last_q - (*s).q) as ::core::ffi::c_double) as ::core::ffi::c_float;
    } else {
        dq = 0.0f32;
    }
    (*s).dq = Clamp(dq, -30.0f32, 30.0f32);
    (*s).last_q = (*s).q;
    (*s).last_value = (*s).value;
    (*s).q = Clamp((*s).q + (*s).dq, (*s).qmin, (*s).qmax);
    return (*s).q;
}
#[no_mangle]
pub static mut VP8Cat3: [uint8_t; 3] = [
    173 as ::core::ffi::c_int as uint8_t,
    148 as ::core::ffi::c_int as uint8_t,
    140 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub static mut VP8Cat4: [uint8_t; 4] = [
    176 as ::core::ffi::c_int as uint8_t,
    155 as ::core::ffi::c_int as uint8_t,
    140 as ::core::ffi::c_int as uint8_t,
    135 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub static mut VP8Cat5: [uint8_t; 5] = [
    180 as ::core::ffi::c_int as uint8_t,
    157 as ::core::ffi::c_int as uint8_t,
    141 as ::core::ffi::c_int as uint8_t,
    134 as ::core::ffi::c_int as uint8_t,
    130 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub static mut VP8Cat6: [uint8_t; 11] = [
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
];
unsafe extern "C" fn ResetStats(enc: *mut VP8Encoder) {
    let proba: *mut VP8EncProba = &raw mut (*enc).proba_;
    VP8CalculateLevelCosts(proba);
    (*proba).nb_skip_ = 0 as ::core::ffi::c_int;
}
pub const SKIP_PROBA_THRESHOLD: ::core::ffi::c_int = 250 as ::core::ffi::c_int;
unsafe extern "C" fn CalcSkipProba(mut nb: uint64_t, mut total: uint64_t) -> ::core::ffi::c_int {
    return (if total != 0 {
        total
            .wrapping_sub(nb)
            .wrapping_mul(255 as uint64_t)
            .wrapping_div(total)
    } else {
        255 as uint64_t
    }) as ::core::ffi::c_int;
}
unsafe extern "C" fn FinalizeSkipProba(enc: *mut VP8Encoder) -> ::core::ffi::c_int {
    let proba: *mut VP8EncProba = &raw mut (*enc).proba_;
    let nb_mbs: ::core::ffi::c_int = (*enc).mb_w_ * (*enc).mb_h_;
    let nb_events: ::core::ffi::c_int = (*proba).nb_skip_;
    let mut size: ::core::ffi::c_int = 0;
    (*proba).skip_proba_ = CalcSkipProba(nb_events as uint64_t, nb_mbs as uint64_t) as uint8_t;
    (*proba).use_skip_proba_ =
        (((*proba).skip_proba_ as ::core::ffi::c_int) < SKIP_PROBA_THRESHOLD) as ::core::ffi::c_int;
    size = 256 as ::core::ffi::c_int;
    if (*proba).use_skip_proba_ != 0 {
        size += nb_events * VP8BitCost(1 as ::core::ffi::c_int, (*proba).skip_proba_)
            + (nb_mbs - nb_events) * VP8BitCost(0 as ::core::ffi::c_int, (*proba).skip_proba_);
        size += 8 as ::core::ffi::c_int * 256 as ::core::ffi::c_int;
    }
    return size;
}
unsafe extern "C" fn CalcTokenProba(
    mut nb: ::core::ffi::c_int,
    mut total: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if nb <= total {
        } else {
            __assert_fail(
                b"nb <= total\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/frame_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                132 as ::core::ffi::c_uint,
                b"int CalcTokenProba(int, int)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    return if nb != 0 {
        255 as ::core::ffi::c_int - nb * 255 as ::core::ffi::c_int / total
    } else {
        255 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn BranchCost(
    mut nb: ::core::ffi::c_int,
    mut total: ::core::ffi::c_int,
    mut proba: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return nb * VP8BitCost(1 as ::core::ffi::c_int, proba as uint8_t)
        + (total - nb) * VP8BitCost(0 as ::core::ffi::c_int, proba as uint8_t);
}
unsafe extern "C" fn ResetTokenStats(enc: *mut VP8Encoder) {
    let proba: *mut VP8EncProba = &raw mut (*enc).proba_;
    memset(
        &raw mut (*proba).stats_ as *mut [StatsArray; 8] as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[[StatsArray; 8]; 4]>() as size_t,
    );
}
unsafe extern "C" fn FinalizeTokenProbas(proba: *mut VP8EncProba) -> ::core::ffi::c_int {
    let mut has_changed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut t: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    t = 0 as ::core::ffi::c_int;
    while t < NUM_TYPES as ::core::ffi::c_int {
        b = 0 as ::core::ffi::c_int;
        while b < NUM_BANDS as ::core::ffi::c_int {
            c = 0 as ::core::ffi::c_int;
            while c < NUM_CTX as ::core::ffi::c_int {
                p = 0 as ::core::ffi::c_int;
                while p < NUM_PROBAS as ::core::ffi::c_int {
                    let stats: proba_t =
                        (*proba).stats_[t as usize][b as usize][c as usize][p as usize];
                    let nb: ::core::ffi::c_int = (stats >> 0 as ::core::ffi::c_int
                        & 0xffff as proba_t)
                        as ::core::ffi::c_int;
                    let total: ::core::ffi::c_int = (stats >> 16 as ::core::ffi::c_int
                        & 0xffff as proba_t)
                        as ::core::ffi::c_int;
                    let update_proba: ::core::ffi::c_int = VP8CoeffsUpdateProba[t as usize]
                        [b as usize][c as usize][p as usize]
                        as ::core::ffi::c_int;
                    let old_p: ::core::ffi::c_int = VP8CoeffsProba0[t as usize][b as usize]
                        [c as usize][p as usize]
                        as ::core::ffi::c_int;
                    let new_p: ::core::ffi::c_int = CalcTokenProba(nb, total) as ::core::ffi::c_int;
                    let old_cost: ::core::ffi::c_int = BranchCost(nb, total, old_p)
                        as ::core::ffi::c_int
                        + VP8BitCost(0 as ::core::ffi::c_int, update_proba as uint8_t)
                            as ::core::ffi::c_int;
                    let new_cost: ::core::ffi::c_int = BranchCost(nb, total, new_p)
                        as ::core::ffi::c_int
                        + VP8BitCost(1 as ::core::ffi::c_int, update_proba as uint8_t)
                            as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 256 as ::core::ffi::c_int;
                    let use_new_p: ::core::ffi::c_int = (old_cost > new_cost) as ::core::ffi::c_int;
                    size += VP8BitCost(use_new_p, update_proba as uint8_t);
                    if use_new_p != 0 {
                        (*proba).coeffs_[t as usize][b as usize][c as usize][p as usize] =
                            new_p as uint8_t;
                        has_changed |= (new_p != old_p) as ::core::ffi::c_int;
                        size += 8 as ::core::ffi::c_int * 256 as ::core::ffi::c_int;
                    } else {
                        (*proba).coeffs_[t as usize][b as usize][c as usize][p as usize] =
                            old_p as uint8_t;
                    }
                    p += 1;
                }
                c += 1;
            }
            b += 1;
        }
        t += 1;
    }
    (*proba).dirty_ = has_changed;
    return size;
}
unsafe extern "C" fn GetProba(
    mut a: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let total: ::core::ffi::c_int = a + b;
    return if total == 0 as ::core::ffi::c_int {
        255 as ::core::ffi::c_int
    } else {
        (255 as ::core::ffi::c_int * a + total / 2 as ::core::ffi::c_int) / total
    };
}
unsafe extern "C" fn ResetSegments(enc: *mut VP8Encoder) {
    let mut n: ::core::ffi::c_int = 0;
    n = 0 as ::core::ffi::c_int;
    while n < (*enc).mb_w_ * (*enc).mb_h_ {
        let ref mut fresh3 = *(*enc).mb_info_.offset(n as isize);
        (*fresh3).set_segment_(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        n += 1;
    }
}
unsafe extern "C" fn SetSegmentProbas(enc: *mut VP8Encoder) {
    let mut p: [::core::ffi::c_int; 4] = [0 as ::core::ffi::c_int; 4];
    let mut n: ::core::ffi::c_int = 0;
    n = 0 as ::core::ffi::c_int;
    while n < (*enc).mb_w_ * (*enc).mb_h_ {
        let mb: *const VP8MBInfo = (*enc).mb_info_.offset(n as isize) as *mut VP8MBInfo;
        p[(*mb).segment_() as usize] += 1;
        n += 1;
    }
    if !(*(*enc).pic_).stats.is_null() {
        n = 0 as ::core::ffi::c_int;
        while n < NUM_MB_SEGMENTS as ::core::ffi::c_int {
            (*(*(*enc).pic_).stats).segment_size[n as usize] = p[n as usize];
            n += 1;
        }
    }
    if (*enc).segment_hdr_.num_segments_ > 1 as ::core::ffi::c_int {
        let probas: *mut uint8_t = &raw mut (*enc).proba_.segments_ as *mut uint8_t;
        *probas.offset(0 as ::core::ffi::c_int as isize) = GetProba(
            p[0 as ::core::ffi::c_int as usize] + p[1 as ::core::ffi::c_int as usize],
            p[2 as ::core::ffi::c_int as usize] + p[3 as ::core::ffi::c_int as usize],
        ) as uint8_t;
        *probas.offset(1 as ::core::ffi::c_int as isize) = GetProba(
            p[0 as ::core::ffi::c_int as usize],
            p[1 as ::core::ffi::c_int as usize],
        ) as uint8_t;
        *probas.offset(2 as ::core::ffi::c_int as isize) = GetProba(
            p[2 as ::core::ffi::c_int as usize],
            p[3 as ::core::ffi::c_int as usize],
        ) as uint8_t;
        (*enc).segment_hdr_.update_map_ =
            (*probas.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 255 as ::core::ffi::c_int
                || *probas.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != 255 as ::core::ffi::c_int
                || *probas.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != 255 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if (*enc).segment_hdr_.update_map_ == 0 {
            ResetSegments(enc);
        }
        (*enc).segment_hdr_.size_ = p[0 as ::core::ffi::c_int as usize]
            * (VP8BitCost(
                0 as ::core::ffi::c_int,
                *probas.offset(0 as ::core::ffi::c_int as isize),
            ) + VP8BitCost(
                0 as ::core::ffi::c_int,
                *probas.offset(1 as ::core::ffi::c_int as isize),
            ))
            + p[1 as ::core::ffi::c_int as usize]
                * (VP8BitCost(
                    0 as ::core::ffi::c_int,
                    *probas.offset(0 as ::core::ffi::c_int as isize),
                ) + VP8BitCost(
                    1 as ::core::ffi::c_int,
                    *probas.offset(1 as ::core::ffi::c_int as isize),
                ))
            + p[2 as ::core::ffi::c_int as usize]
                * (VP8BitCost(
                    1 as ::core::ffi::c_int,
                    *probas.offset(0 as ::core::ffi::c_int as isize),
                ) + VP8BitCost(
                    0 as ::core::ffi::c_int,
                    *probas.offset(2 as ::core::ffi::c_int as isize),
                ))
            + p[3 as ::core::ffi::c_int as usize]
                * (VP8BitCost(
                    1 as ::core::ffi::c_int,
                    *probas.offset(0 as ::core::ffi::c_int as isize),
                ) + VP8BitCost(
                    1 as ::core::ffi::c_int,
                    *probas.offset(2 as ::core::ffi::c_int as isize),
                ));
    } else {
        (*enc).segment_hdr_.update_map_ = 0 as ::core::ffi::c_int;
        (*enc).segment_hdr_.size_ = 0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn PutCoeffs(
    bw: *mut VP8BitWriter,
    mut ctx: ::core::ffi::c_int,
    mut res: *const VP8Residual,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = (*res).first;
    let mut p: *const uint8_t = &raw mut *(&raw mut *(*res).prob.offset(n as isize)
        as *mut [uint8_t; 11])
        .offset(ctx as isize) as *mut uint8_t;
    if VP8PutBit(
        bw,
        ((*res).last >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    while n < 16 as ::core::ffi::c_int {
        let fresh0 = n;
        n = n + 1;
        let c: ::core::ffi::c_int = *(*res).coeffs.offset(fresh0 as isize) as ::core::ffi::c_int;
        let sign: ::core::ffi::c_int = (c < 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        let mut v: ::core::ffi::c_int = if sign != 0 { -c } else { c };
        if VP8PutBit(
            bw,
            (v != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        ) == 0
        {
            p = &raw mut *(&raw mut *(*res)
                .prob
                .offset(*(&raw const VP8EncBands as *const uint8_t).offset(n as isize) as isize)
                as *mut [uint8_t; 11])
                .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t;
        } else {
            if VP8PutBit(
                bw,
                (v > 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                *p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            ) == 0
            {
                p = &raw mut *(&raw mut *(*res)
                    .prob
                    .offset(*(&raw const VP8EncBands as *const uint8_t).offset(n as isize) as isize)
                    as *mut [uint8_t; 11])
                    .offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t;
            } else {
                if VP8PutBit(
                    bw,
                    (v > 4 as ::core::ffi::c_int) as ::core::ffi::c_int,
                    *p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                ) == 0
                {
                    if VP8PutBit(
                        bw,
                        (v != 2 as ::core::ffi::c_int) as ::core::ffi::c_int,
                        *p.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                    ) != 0
                    {
                        VP8PutBit(
                            bw,
                            (v == 4 as ::core::ffi::c_int) as ::core::ffi::c_int,
                            *p.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                        );
                    }
                } else if VP8PutBit(
                    bw,
                    (v > 10 as ::core::ffi::c_int) as ::core::ffi::c_int,
                    *p.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                ) == 0
                {
                    if VP8PutBit(
                        bw,
                        (v > 6 as ::core::ffi::c_int) as ::core::ffi::c_int,
                        *p.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                    ) == 0
                    {
                        VP8PutBit(
                            bw,
                            (v == 6 as ::core::ffi::c_int) as ::core::ffi::c_int,
                            159 as ::core::ffi::c_int,
                        );
                    } else {
                        VP8PutBit(
                            bw,
                            (v >= 9 as ::core::ffi::c_int) as ::core::ffi::c_int,
                            165 as ::core::ffi::c_int,
                        );
                        VP8PutBit(
                            bw,
                            (v & 1 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                            145 as ::core::ffi::c_int,
                        );
                    }
                } else {
                    let mut mask: ::core::ffi::c_int = 0;
                    let mut tab: *const uint8_t = ::core::ptr::null::<uint8_t>();
                    if v < 3 as ::core::ffi::c_int
                        + ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    {
                        VP8PutBit(
                            bw,
                            0 as ::core::ffi::c_int,
                            *p.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                        );
                        VP8PutBit(
                            bw,
                            0 as ::core::ffi::c_int,
                            *p.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                        );
                        v -= 3 as ::core::ffi::c_int
                            + ((8 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int);
                        mask = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
                        tab = &raw const VP8Cat3 as *const uint8_t;
                    } else if v < 3 as ::core::ffi::c_int
                        + ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    {
                        VP8PutBit(
                            bw,
                            0 as ::core::ffi::c_int,
                            *p.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                        );
                        VP8PutBit(
                            bw,
                            1 as ::core::ffi::c_int,
                            *p.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                        );
                        v -= 3 as ::core::ffi::c_int
                            + ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int);
                        mask = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
                        tab = &raw const VP8Cat4 as *const uint8_t;
                    } else if v < 3 as ::core::ffi::c_int
                        + ((8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    {
                        VP8PutBit(
                            bw,
                            1 as ::core::ffi::c_int,
                            *p.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                        );
                        VP8PutBit(
                            bw,
                            0 as ::core::ffi::c_int,
                            *p.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                        );
                        v -= 3 as ::core::ffi::c_int
                            + ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int);
                        mask = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
                        tab = &raw const VP8Cat5 as *const uint8_t;
                    } else {
                        VP8PutBit(
                            bw,
                            1 as ::core::ffi::c_int,
                            *p.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                        );
                        VP8PutBit(
                            bw,
                            1 as ::core::ffi::c_int,
                            *p.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                        );
                        v -= 3 as ::core::ffi::c_int
                            + ((8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int);
                        mask = (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int;
                        tab = &raw const VP8Cat6 as *const uint8_t;
                    }
                    while mask != 0 {
                        let fresh1 = tab;
                        tab = tab.offset(1);
                        VP8PutBit(
                            bw,
                            (v & mask != 0) as ::core::ffi::c_int,
                            *fresh1 as ::core::ffi::c_int,
                        );
                        mask >>= 1 as ::core::ffi::c_int;
                    }
                }
                p = &raw mut *(&raw mut *(*res)
                    .prob
                    .offset(*(&raw const VP8EncBands as *const uint8_t).offset(n as isize) as isize)
                    as *mut [uint8_t; 11])
                    .offset(2 as ::core::ffi::c_int as isize) as *mut uint8_t;
            }
            VP8PutBitUniform(bw, sign);
            if n == 16 as ::core::ffi::c_int
                || VP8PutBit(
                    bw,
                    (n <= (*res).last) as ::core::ffi::c_int,
                    *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                ) == 0
            {
                return 1 as ::core::ffi::c_int;
            }
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn CodeResiduals(
    bw: *mut VP8BitWriter,
    it: *mut VP8EncIterator,
    rd: *const VP8ModeScore,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut ch: ::core::ffi::c_int = 0;
    let mut res: VP8Residual = VP8Residual {
        first: 0,
        last: 0,
        coeffs: ::core::ptr::null::<int16_t>(),
        coeff_type: 0,
        prob: ::core::ptr::null_mut::<ProbaArray>(),
        stats: ::core::ptr::null_mut::<StatsArray>(),
        costs: ::core::ptr::null_mut::<[*const uint16_t; 3]>(),
    };
    let mut pos1: uint64_t = 0;
    let mut pos2: uint64_t = 0;
    let mut pos3: uint64_t = 0;
    let i16: ::core::ffi::c_int = ((*(*it).mb_).type_() as ::core::ffi::c_int
        == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let segment: ::core::ffi::c_int = (*(*it).mb_).segment_() as ::core::ffi::c_int;
    let enc: *mut VP8Encoder = (*it).enc_;
    VP8IteratorNzToBytes(it);
    pos1 = VP8BitWriterPos(bw);
    if i16 != 0 {
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
        (*it).left_nz_[8 as ::core::ffi::c_int as usize] = PutCoeffs(
            bw,
            (*it).top_nz_[8 as ::core::ffi::c_int as usize]
                + (*it).left_nz_[8 as ::core::ffi::c_int as usize],
            &raw mut res,
        );
        (*it).top_nz_[8 as ::core::ffi::c_int as usize] =
            (*it).left_nz_[8 as ::core::ffi::c_int as usize];
        VP8InitResidual(
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            enc,
            &raw mut res,
        );
    } else {
        VP8InitResidual(
            0 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int,
            enc,
            &raw mut res,
        );
    }
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
            (*it).left_nz_[y as usize] = PutCoeffs(bw, ctx, &raw mut res);
            (*it).top_nz_[x as usize] = (*it).left_nz_[y as usize];
            x += 1;
        }
        y += 1;
    }
    pos2 = VP8BitWriterPos(bw);
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
                let ctx_0: ::core::ffi::c_int = (*it).top_nz_
                    [(4 as ::core::ffi::c_int + ch + x) as usize]
                    + (*it).left_nz_[(4 as ::core::ffi::c_int + ch + y) as usize];
                VP8SetResidualCoeffs.expect("non-null function pointer")(
                    &raw const *(&raw const (*rd).uv_levels as *const [int16_t; 16]).offset(
                        (ch * 2 as ::core::ffi::c_int + x + y * 2 as ::core::ffi::c_int) as isize,
                    ) as *const int16_t,
                    &raw mut res,
                );
                (*it).left_nz_[(4 as ::core::ffi::c_int + ch + y) as usize] =
                    PutCoeffs(bw, ctx_0, &raw mut res);
                (*it).top_nz_[(4 as ::core::ffi::c_int + ch + x) as usize] =
                    (*it).left_nz_[(4 as ::core::ffi::c_int + ch + y) as usize];
                x += 1;
            }
            y += 1;
        }
        ch += 2 as ::core::ffi::c_int;
    }
    pos3 = VP8BitWriterPos(bw);
    (*it).luma_bits_ = pos2.wrapping_sub(pos1);
    (*it).uv_bits_ = pos3.wrapping_sub(pos2);
    (*it).bit_count_[segment as usize][i16 as usize] =
        (*it).bit_count_[segment as usize][i16 as usize].wrapping_add((*it).luma_bits_);
    (*it).bit_count_[segment as usize][2 as ::core::ffi::c_int as usize] = (*it).bit_count_
        [segment as usize][2 as ::core::ffi::c_int as usize]
        .wrapping_add((*it).uv_bits_);
    VP8IteratorBytesToNz(it);
}
unsafe extern "C" fn RecordResiduals(it: *mut VP8EncIterator, rd: *const VP8ModeScore) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut ch: ::core::ffi::c_int = 0;
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
    VP8IteratorNzToBytes(it);
    if (*(*it).mb_).type_() as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
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
        (*it).left_nz_[8 as ::core::ffi::c_int as usize] = VP8RecordCoeffs(
            (*it).top_nz_[8 as ::core::ffi::c_int as usize]
                + (*it).left_nz_[8 as ::core::ffi::c_int as usize],
            &raw mut res,
        );
        (*it).top_nz_[8 as ::core::ffi::c_int as usize] =
            (*it).left_nz_[8 as ::core::ffi::c_int as usize];
        VP8InitResidual(
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            enc,
            &raw mut res,
        );
    } else {
        VP8InitResidual(
            0 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int,
            enc,
            &raw mut res,
        );
    }
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
            (*it).left_nz_[y as usize] = VP8RecordCoeffs(ctx, &raw mut res);
            (*it).top_nz_[x as usize] = (*it).left_nz_[y as usize];
            x += 1;
        }
        y += 1;
    }
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
                let ctx_0: ::core::ffi::c_int = (*it).top_nz_
                    [(4 as ::core::ffi::c_int + ch + x) as usize]
                    + (*it).left_nz_[(4 as ::core::ffi::c_int + ch + y) as usize];
                VP8SetResidualCoeffs.expect("non-null function pointer")(
                    &raw const *(&raw const (*rd).uv_levels as *const [int16_t; 16]).offset(
                        (ch * 2 as ::core::ffi::c_int + x + y * 2 as ::core::ffi::c_int) as isize,
                    ) as *const int16_t,
                    &raw mut res,
                );
                (*it).left_nz_[(4 as ::core::ffi::c_int + ch + y) as usize] =
                    VP8RecordCoeffs(ctx_0, &raw mut res);
                (*it).top_nz_[(4 as ::core::ffi::c_int + ch + x) as usize] =
                    (*it).left_nz_[(4 as ::core::ffi::c_int + ch + y) as usize];
                x += 1;
            }
            y += 1;
        }
        ch += 2 as ::core::ffi::c_int;
    }
    VP8IteratorBytesToNz(it);
}
unsafe extern "C" fn RecordTokens(
    it: *mut VP8EncIterator,
    rd: *const VP8ModeScore,
    tokens: *mut VP8TBuffer,
) -> ::core::ffi::c_int {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut ch: ::core::ffi::c_int = 0;
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
    VP8IteratorNzToBytes(it);
    if (*(*it).mb_).type_() as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        let ctx: ::core::ffi::c_int = (*it).top_nz_[8 as ::core::ffi::c_int as usize]
            + (*it).left_nz_[8 as ::core::ffi::c_int as usize];
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
        (*it).left_nz_[8 as ::core::ffi::c_int as usize] =
            VP8RecordCoeffTokens(ctx, &raw mut res, tokens);
        (*it).top_nz_[8 as ::core::ffi::c_int as usize] =
            (*it).left_nz_[8 as ::core::ffi::c_int as usize];
        VP8InitResidual(
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            enc,
            &raw mut res,
        );
    } else {
        VP8InitResidual(
            0 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int,
            enc,
            &raw mut res,
        );
    }
    y = 0 as ::core::ffi::c_int;
    while y < 4 as ::core::ffi::c_int {
        x = 0 as ::core::ffi::c_int;
        while x < 4 as ::core::ffi::c_int {
            let ctx_0: ::core::ffi::c_int = (*it).top_nz_[x as usize] + (*it).left_nz_[y as usize];
            VP8SetResidualCoeffs.expect("non-null function pointer")(
                &raw const *(&raw const (*rd).y_ac_levels as *const [int16_t; 16])
                    .offset((x + y * 4 as ::core::ffi::c_int) as isize)
                    as *const int16_t,
                &raw mut res,
            );
            (*it).left_nz_[y as usize] = VP8RecordCoeffTokens(ctx_0, &raw mut res, tokens);
            (*it).top_nz_[x as usize] = (*it).left_nz_[y as usize];
            x += 1;
        }
        y += 1;
    }
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
                let ctx_1: ::core::ffi::c_int = (*it).top_nz_
                    [(4 as ::core::ffi::c_int + ch + x) as usize]
                    + (*it).left_nz_[(4 as ::core::ffi::c_int + ch + y) as usize];
                VP8SetResidualCoeffs.expect("non-null function pointer")(
                    &raw const *(&raw const (*rd).uv_levels as *const [int16_t; 16]).offset(
                        (ch * 2 as ::core::ffi::c_int + x + y * 2 as ::core::ffi::c_int) as isize,
                    ) as *const int16_t,
                    &raw mut res,
                );
                (*it).left_nz_[(4 as ::core::ffi::c_int + ch + y) as usize] =
                    VP8RecordCoeffTokens(ctx_1, &raw mut res, tokens);
                (*it).top_nz_[(4 as ::core::ffi::c_int + ch + x) as usize] =
                    (*it).left_nz_[(4 as ::core::ffi::c_int + ch + y) as usize];
                x += 1;
            }
            y += 1;
        }
        ch += 2 as ::core::ffi::c_int;
    }
    VP8IteratorBytesToNz(it);
    return ((*tokens).error_ == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn ResetSSE(enc: *mut VP8Encoder) {
    (*enc).sse_[0 as ::core::ffi::c_int as usize] = 0 as uint64_t;
    (*enc).sse_[1 as ::core::ffi::c_int as usize] = 0 as uint64_t;
    (*enc).sse_[2 as ::core::ffi::c_int as usize] = 0 as uint64_t;
    (*enc).sse_count_ = 0 as uint64_t;
}
unsafe extern "C" fn StoreSSE(it: *const VP8EncIterator) {
    let enc: *mut VP8Encoder = (*it).enc_;
    let in_0: *const uint8_t = (*it).yuv_in_;
    let out: *const uint8_t = (*it).yuv_out_;
    (*enc).sse_[0 as ::core::ffi::c_int as usize] = (*enc).sse_[0 as ::core::ffi::c_int as usize]
        .wrapping_add(VP8SSE16x16.expect("non-null function pointer")(
            in_0.offset(Y_OFF_ENC as isize),
            out.offset(Y_OFF_ENC as isize),
        ) as uint64_t);
    (*enc).sse_[1 as ::core::ffi::c_int as usize] = (*enc).sse_[1 as ::core::ffi::c_int as usize]
        .wrapping_add(VP8SSE8x8.expect("non-null function pointer")(
            in_0.offset(U_OFF_ENC as isize),
            out.offset(U_OFF_ENC as isize),
        ) as uint64_t);
    (*enc).sse_[2 as ::core::ffi::c_int as usize] = (*enc).sse_[2 as ::core::ffi::c_int as usize]
        .wrapping_add(VP8SSE8x8.expect("non-null function pointer")(
            in_0.offset(V_OFF_ENC as isize),
            out.offset(V_OFF_ENC as isize),
        ) as uint64_t);
    (*enc).sse_count_ = (*enc)
        .sse_count_
        .wrapping_add((16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint64_t);
}
unsafe extern "C" fn StoreSideInfo(it: *const VP8EncIterator) {
    let enc: *mut VP8Encoder = (*it).enc_;
    let mb: *const VP8MBInfo = (*it).mb_;
    let pic: *mut WebPPicture = (*enc).pic_;
    if !(*pic).stats.is_null() {
        StoreSSE(it);
        (*enc).block_count_[0 as ::core::ffi::c_int as usize] +=
            ((*mb).type_() as ::core::ffi::c_int == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        (*enc).block_count_[1 as ::core::ffi::c_int as usize] +=
            ((*mb).type_() as ::core::ffi::c_int == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
        (*enc).block_count_[2 as ::core::ffi::c_int as usize] +=
            ((*mb).skip_() as ::core::ffi::c_int != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    if !(*pic).extra_info.is_null() {
        let info: *mut uint8_t = (*pic)
            .extra_info
            .offset(((*it).x_ + (*it).y_ * (*enc).mb_w_) as isize)
            as *mut uint8_t;
        match (*pic).extra_info_type {
            1 => {
                *info = (*mb).type_() as uint8_t;
            }
            2 => {
                *info = (*mb).segment_() as uint8_t;
            }
            3 => {
                *info = (*enc).dqm_[(*mb).segment_() as usize].quant_ as uint8_t;
            }
            4 => {
                *info = (if (*mb).type_() as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    *(*it).preds_.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                } else {
                    0xff as ::core::ffi::c_int
                }) as uint8_t;
            }
            5 => {
                *info = (*mb).uv_mode_() as uint8_t;
            }
            6 => {
                let b: ::core::ffi::c_int = ((*it)
                    .luma_bits_
                    .wrapping_add((*it).uv_bits_)
                    .wrapping_add(7 as uint64_t)
                    >> 3 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                *info = (if b > 255 as ::core::ffi::c_int {
                    255 as ::core::ffi::c_int
                } else {
                    b
                }) as uint8_t;
            }
            7 => {
                *info = (*mb).alpha_;
            }
            _ => {
                *info = 0 as uint8_t;
            }
        }
    }
}
unsafe extern "C" fn ResetSideInfo(it: *const VP8EncIterator) {
    let enc: *mut VP8Encoder = (*it).enc_;
    let pic: *mut WebPPicture = (*enc).pic_;
    if !(*pic).stats.is_null() {
        memset(
            &raw mut (*enc).block_count_ as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_int; 3]>() as size_t,
        );
    }
    ResetSSE(enc);
}
unsafe extern "C" fn GetPSNR(mut mse: uint64_t, mut size: uint64_t) -> ::core::ffi::c_double {
    return if mse > 0 as uint64_t && size > 0 as uint64_t {
        10.0f64
            * log10(
                255.0f64 * 255.0f64 * size as ::core::ffi::c_double / mse as ::core::ffi::c_double,
            )
    } else {
        99 as ::core::ffi::c_int as ::core::ffi::c_double
    };
}
unsafe extern "C" fn SetLoopParams(enc: *mut VP8Encoder, mut q: ::core::ffi::c_float) {
    q = Clamp(q, 0.0f32, 100.0f32);
    VP8SetSegmentParams(enc, q);
    SetSegmentProbas(enc);
    ResetStats(enc);
    ResetSSE(enc);
}
unsafe extern "C" fn OneStatPass(
    enc: *mut VP8Encoder,
    mut rd_opt: VP8RDLevel,
    mut nb_mbs: ::core::ffi::c_int,
    mut percent_delta: ::core::ffi::c_int,
    s: *mut PassStats,
) -> uint64_t {
    let mut it: VP8EncIterator = VP8EncIterator {
        x_: 0,
        y_: 0,
        yuv_in_: ::core::ptr::null_mut::<uint8_t>(),
        yuv_out_: ::core::ptr::null_mut::<uint8_t>(),
        yuv_out2_: ::core::ptr::null_mut::<uint8_t>(),
        yuv_p_: ::core::ptr::null_mut::<uint8_t>(),
        enc_: ::core::ptr::null_mut::<VP8Encoder>(),
        mb_: ::core::ptr::null_mut::<VP8MBInfo>(),
        bw_: ::core::ptr::null_mut::<VP8BitWriter>(),
        preds_: ::core::ptr::null_mut::<uint8_t>(),
        nz_: ::core::ptr::null_mut::<uint32_t>(),
        i4_boundary_: [0; 37],
        i4_top_: ::core::ptr::null_mut::<uint8_t>(),
        i4_: 0,
        top_nz_: [0; 9],
        left_nz_: [0; 9],
        bit_count_: [[0; 3]; 4],
        luma_bits_: 0,
        uv_bits_: 0,
        lf_stats_: ::core::ptr::null_mut::<LFStats>(),
        do_trellis_: 0,
        count_down_: 0,
        count_down0_: 0,
        percent0_: 0,
        left_derr_: [[0; 2]; 2],
        top_derr_: ::core::ptr::null_mut::<DError>(),
        y_left_: ::core::ptr::null_mut::<uint8_t>(),
        u_left_: ::core::ptr::null_mut::<uint8_t>(),
        v_left_: ::core::ptr::null_mut::<uint8_t>(),
        y_top_: ::core::ptr::null_mut::<uint8_t>(),
        uv_top_: ::core::ptr::null_mut::<uint8_t>(),
        yuv_left_mem_: [0; 88],
        yuv_mem_: [0; 3359],
    };
    let mut size: uint64_t = 0 as uint64_t;
    let mut size_p0: uint64_t = 0 as uint64_t;
    let mut distortion: uint64_t = 0 as uint64_t;
    let pixel_count: uint64_t = (nb_mbs as uint64_t).wrapping_mul(384 as uint64_t);
    VP8IteratorInit(enc, &raw mut it);
    SetLoopParams(enc, (*s).q);
    loop {
        let mut info: VP8ModeScore = VP8ModeScore {
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
        VP8IteratorImport(&raw mut it, ::core::ptr::null_mut::<uint8_t>());
        if VP8Decimate(&raw mut it, &raw mut info, rd_opt) != 0 {
            (*enc).proba_.nb_skip_ += 1;
        }
        RecordResiduals(&raw mut it, &raw mut info);
        size = size.wrapping_add((info.R + info.H) as uint64_t);
        size_p0 = size_p0.wrapping_add(info.H as uint64_t);
        distortion = distortion.wrapping_add(info.D as uint64_t);
        if percent_delta != 0 && VP8IteratorProgress(&raw mut it, percent_delta) == 0 {
            return 0 as uint64_t;
        }
        VP8IteratorSaveBoundary(&raw mut it);
        if !(VP8IteratorNext(&raw mut it) != 0 && {
            nb_mbs -= 1;
            nb_mbs > 0 as ::core::ffi::c_int
        }) {
            break;
        }
    }
    size_p0 = size_p0.wrapping_add((*enc).segment_hdr_.size_ as uint64_t);
    if (*s).do_size_search != 0 {
        size = size.wrapping_add(FinalizeSkipProba(enc) as uint64_t);
        size = size.wrapping_add(FinalizeTokenProbas(&raw mut (*enc).proba_) as uint64_t);
        size = (size.wrapping_add(size_p0).wrapping_add(1024 as uint64_t)
            >> 11 as ::core::ffi::c_int)
            .wrapping_add(HEADER_SIZE_ESTIMATE as uint64_t);
        (*s).value = size as ::core::ffi::c_double;
    } else {
        (*s).value = GetPSNR(distortion, pixel_count);
    }
    return size_p0;
}
unsafe extern "C" fn StatLoop(enc: *mut VP8Encoder) -> ::core::ffi::c_int {
    let method: ::core::ffi::c_int = (*enc).method_;
    let do_search: ::core::ffi::c_int = (*enc).do_search_;
    let fast_probe: ::core::ffi::c_int = ((method == 0 as ::core::ffi::c_int
        || method == 3 as ::core::ffi::c_int)
        && do_search == 0) as ::core::ffi::c_int;
    let mut num_pass_left: ::core::ffi::c_int = (*(*enc).config_).pass;
    let task_percent: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
    let percent_per_pass: ::core::ffi::c_int =
        (task_percent + num_pass_left / 2 as ::core::ffi::c_int) / num_pass_left;
    let final_percent: ::core::ffi::c_int = (*enc).percent_ + task_percent;
    let rd_opt: VP8RDLevel = (if method >= 3 as ::core::ffi::c_int || do_search != 0 {
        RD_OPT_BASIC as ::core::ffi::c_int
    } else {
        RD_OPT_NONE as ::core::ffi::c_int
    }) as VP8RDLevel;
    let mut nb_mbs: ::core::ffi::c_int = (*enc).mb_w_ * (*enc).mb_h_;
    let mut stats: PassStats = PassStats {
        is_first: 0,
        dq: 0.,
        q: 0.,
        last_q: 0.,
        qmin: 0.,
        qmax: 0.,
        value: 0.,
        last_value: 0.,
        target: 0.,
        do_size_search: 0,
    };
    InitPassStats(enc, &raw mut stats);
    ResetTokenStats(enc);
    if fast_probe != 0 {
        if method == 3 as ::core::ffi::c_int {
            nb_mbs = if nb_mbs > 200 as ::core::ffi::c_int {
                nb_mbs >> 1 as ::core::ffi::c_int
            } else {
                100 as ::core::ffi::c_int
            };
        } else {
            nb_mbs = if nb_mbs > 200 as ::core::ffi::c_int {
                nb_mbs >> 2 as ::core::ffi::c_int
            } else {
                50 as ::core::ffi::c_int
            };
        }
    }
    loop {
        let fresh2 = num_pass_left;
        num_pass_left = num_pass_left - 1;
        if !(fresh2 > 0 as ::core::ffi::c_int) {
            break;
        }
        let is_last_pass: ::core::ffi::c_int = (fabs(stats.dq as ::core::ffi::c_double) <= DQ_LIMIT
            || num_pass_left == 0 as ::core::ffi::c_int
            || (*enc).max_i4_header_bits_ == 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        let size_p0: uint64_t =
            OneStatPass(enc, rd_opt, nb_mbs, percent_per_pass, &raw mut stats) as uint64_t;
        if size_p0 == 0 as uint64_t {
            return 0 as ::core::ffi::c_int;
        }
        if (*enc).max_i4_header_bits_ > 0 as ::core::ffi::c_int
            && size_p0 as ::core::ffi::c_ulonglong > PARTITION0_SIZE_LIMIT
        {
            num_pass_left += 1;
            (*enc).max_i4_header_bits_ >>= 1 as ::core::ffi::c_int;
        } else {
            if is_last_pass != 0 {
                break;
            }
            if !(do_search != 0) {
                continue;
            }
            ComputeNextQ(&raw mut stats);
            if fabs(stats.dq as ::core::ffi::c_double) <= DQ_LIMIT {
                break;
            }
        }
    }
    if do_search == 0 || stats.do_size_search == 0 {
        FinalizeSkipProba(enc);
        FinalizeTokenProbas(&raw mut (*enc).proba_);
    }
    VP8CalculateLevelCosts(&raw mut (*enc).proba_);
    return WebPReportProgress((*enc).pic_, final_percent, &raw mut (*enc).percent_);
}
static mut kAverageBytesPerMB: [uint8_t; 8] = [
    50 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
];
unsafe extern "C" fn PreLoopInitialize(enc: *mut VP8Encoder) -> ::core::ffi::c_int {
    let mut p: ::core::ffi::c_int = 0;
    let mut ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let average_bytes_per_MB: ::core::ffi::c_int = kAverageBytesPerMB
        [((*enc).base_quant_ >> 4 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int;
    let bytes_per_parts: ::core::ffi::c_int =
        (*enc).mb_w_ * (*enc).mb_h_ * average_bytes_per_MB / (*enc).num_parts_;
    p = 0 as ::core::ffi::c_int;
    while ok != 0 && p < (*enc).num_parts_ {
        ok = VP8BitWriterInit(
            (&raw mut (*enc).parts_ as *mut VP8BitWriter).offset(p as isize),
            bytes_per_parts as size_t,
        );
        p += 1;
    }
    if ok == 0 {
        VP8EncFreeBitWriters(enc);
        return WebPEncodingSetError((*enc).pic_, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    return ok;
}
unsafe extern "C" fn PostLoopFinalize(
    it: *mut VP8EncIterator,
    mut ok: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let enc: *mut VP8Encoder = (*it).enc_;
    if ok != 0 {
        let mut p: ::core::ffi::c_int = 0;
        p = 0 as ::core::ffi::c_int;
        while p < (*enc).num_parts_ {
            VP8BitWriterFinish((&raw mut (*enc).parts_ as *mut VP8BitWriter).offset(p as isize));
            ok &= ((*enc).parts_[p as usize].error_ == 0) as ::core::ffi::c_int;
            p += 1;
        }
    }
    if ok != 0 {
        if !(*(*enc).pic_).stats.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            let mut s: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i <= 2 as ::core::ffi::c_int {
                s = 0 as ::core::ffi::c_int;
                while s < NUM_MB_SEGMENTS as ::core::ffi::c_int {
                    (*enc).residual_bytes_[i as usize][s as usize] =
                        ((*it).bit_count_[s as usize][i as usize].wrapping_add(7 as uint64_t)
                            >> 3 as ::core::ffi::c_int)
                            as ::core::ffi::c_int;
                    s += 1;
                }
                i += 1;
            }
        }
        VP8AdjustFilterStrength(it);
    } else {
        VP8EncFreeBitWriters(enc);
        return WebPEncodingSetError((*enc).pic_, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    return ok;
}
unsafe extern "C" fn ResetAfterSkip(it: *mut VP8EncIterator) {
    if (*(*it).mb_).type_() as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        *(*it).nz_ = 0 as uint32_t;
        (*it).left_nz_[8 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
    } else {
        *(*it).nz_ &= ((1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as uint32_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8EncLoop(enc: *mut VP8Encoder) -> ::core::ffi::c_int {
    let mut it: VP8EncIterator = VP8EncIterator {
        x_: 0,
        y_: 0,
        yuv_in_: ::core::ptr::null_mut::<uint8_t>(),
        yuv_out_: ::core::ptr::null_mut::<uint8_t>(),
        yuv_out2_: ::core::ptr::null_mut::<uint8_t>(),
        yuv_p_: ::core::ptr::null_mut::<uint8_t>(),
        enc_: ::core::ptr::null_mut::<VP8Encoder>(),
        mb_: ::core::ptr::null_mut::<VP8MBInfo>(),
        bw_: ::core::ptr::null_mut::<VP8BitWriter>(),
        preds_: ::core::ptr::null_mut::<uint8_t>(),
        nz_: ::core::ptr::null_mut::<uint32_t>(),
        i4_boundary_: [0; 37],
        i4_top_: ::core::ptr::null_mut::<uint8_t>(),
        i4_: 0,
        top_nz_: [0; 9],
        left_nz_: [0; 9],
        bit_count_: [[0; 3]; 4],
        luma_bits_: 0,
        uv_bits_: 0,
        lf_stats_: ::core::ptr::null_mut::<LFStats>(),
        do_trellis_: 0,
        count_down_: 0,
        count_down0_: 0,
        percent0_: 0,
        left_derr_: [[0; 2]; 2],
        top_derr_: ::core::ptr::null_mut::<DError>(),
        y_left_: ::core::ptr::null_mut::<uint8_t>(),
        u_left_: ::core::ptr::null_mut::<uint8_t>(),
        v_left_: ::core::ptr::null_mut::<uint8_t>(),
        y_top_: ::core::ptr::null_mut::<uint8_t>(),
        uv_top_: ::core::ptr::null_mut::<uint8_t>(),
        yuv_left_mem_: [0; 88],
        yuv_mem_: [0; 3359],
    };
    let mut ok: ::core::ffi::c_int = PreLoopInitialize(enc);
    if ok == 0 {
        return 0 as ::core::ffi::c_int;
    }
    StatLoop(enc);
    VP8IteratorInit(enc, &raw mut it);
    VP8InitFilter(&raw mut it);
    loop {
        let mut info: VP8ModeScore = VP8ModeScore {
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
        let dont_use_skip: ::core::ffi::c_int =
            ((*enc).proba_.use_skip_proba_ == 0) as ::core::ffi::c_int;
        let rd_opt: VP8RDLevel = (*enc).rd_opt_level_;
        VP8IteratorImport(&raw mut it, ::core::ptr::null_mut::<uint8_t>());
        if VP8Decimate(&raw mut it, &raw mut info, rd_opt) == 0 || dont_use_skip != 0 {
            CodeResiduals(it.bw_, &raw mut it, &raw mut info);
            if (*it.bw_).error_ != 0 {
                ok = 0 as ::core::ffi::c_int;
                break;
            }
        } else {
            ResetAfterSkip(&raw mut it);
        }
        StoreSideInfo(&raw mut it);
        VP8StoreFilterStats(&raw mut it);
        VP8IteratorExport(&raw mut it);
        ok = VP8IteratorProgress(&raw mut it, 20 as ::core::ffi::c_int);
        VP8IteratorSaveBoundary(&raw mut it);
        if !(ok != 0 && VP8IteratorNext(&raw mut it) != 0) {
            break;
        }
    }
    return PostLoopFinalize(&raw mut it, ok);
}
pub const MIN_COUNT: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn VP8EncTokenLoop(enc: *mut VP8Encoder) -> ::core::ffi::c_int {
    let mut max_count: ::core::ffi::c_int = (*enc).mb_w_ * (*enc).mb_h_ >> 3 as ::core::ffi::c_int;
    let mut num_pass_left: ::core::ffi::c_int = (*(*enc).config_).pass;
    let mut remaining_progress: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
    let do_search: ::core::ffi::c_int = (*enc).do_search_;
    let mut it: VP8EncIterator = VP8EncIterator {
        x_: 0,
        y_: 0,
        yuv_in_: ::core::ptr::null_mut::<uint8_t>(),
        yuv_out_: ::core::ptr::null_mut::<uint8_t>(),
        yuv_out2_: ::core::ptr::null_mut::<uint8_t>(),
        yuv_p_: ::core::ptr::null_mut::<uint8_t>(),
        enc_: ::core::ptr::null_mut::<VP8Encoder>(),
        mb_: ::core::ptr::null_mut::<VP8MBInfo>(),
        bw_: ::core::ptr::null_mut::<VP8BitWriter>(),
        preds_: ::core::ptr::null_mut::<uint8_t>(),
        nz_: ::core::ptr::null_mut::<uint32_t>(),
        i4_boundary_: [0; 37],
        i4_top_: ::core::ptr::null_mut::<uint8_t>(),
        i4_: 0,
        top_nz_: [0; 9],
        left_nz_: [0; 9],
        bit_count_: [[0; 3]; 4],
        luma_bits_: 0,
        uv_bits_: 0,
        lf_stats_: ::core::ptr::null_mut::<LFStats>(),
        do_trellis_: 0,
        count_down_: 0,
        count_down0_: 0,
        percent0_: 0,
        left_derr_: [[0; 2]; 2],
        top_derr_: ::core::ptr::null_mut::<DError>(),
        y_left_: ::core::ptr::null_mut::<uint8_t>(),
        u_left_: ::core::ptr::null_mut::<uint8_t>(),
        v_left_: ::core::ptr::null_mut::<uint8_t>(),
        y_top_: ::core::ptr::null_mut::<uint8_t>(),
        uv_top_: ::core::ptr::null_mut::<uint8_t>(),
        yuv_left_mem_: [0; 88],
        yuv_mem_: [0; 3359],
    };
    let proba: *mut VP8EncProba = &raw mut (*enc).proba_;
    let rd_opt: VP8RDLevel = (*enc).rd_opt_level_;
    let pixel_count: uint64_t = ((*enc).mb_w_ as uint64_t)
        .wrapping_mul((*enc).mb_h_ as uint64_t)
        .wrapping_mul(384 as uint64_t);
    let mut stats: PassStats = PassStats {
        is_first: 0,
        dq: 0.,
        q: 0.,
        last_q: 0.,
        qmin: 0.,
        qmax: 0.,
        value: 0.,
        last_value: 0.,
        target: 0.,
        do_size_search: 0,
    };
    let mut ok: ::core::ffi::c_int = 0;
    InitPassStats(enc, &raw mut stats);
    ok = PreLoopInitialize(enc);
    if ok == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if max_count < MIN_COUNT {
        max_count = MIN_COUNT;
    }
    '_c2rust_label: {
        if (*enc).num_parts_ == 1 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"enc->num_parts_ == 1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/frame_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                802 as ::core::ffi::c_uint,
                b"int VP8EncTokenLoop(VP8Encoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*enc).use_tokens_ != 0 {
        } else {
            __assert_fail(
                b"enc->use_tokens_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/frame_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                803 as ::core::ffi::c_uint,
                b"int VP8EncTokenLoop(VP8Encoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*proba).use_skip_proba_ == 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"proba->use_skip_proba_ == 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/frame_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                804 as ::core::ffi::c_uint,
                b"int VP8EncTokenLoop(VP8Encoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if rd_opt as ::core::ffi::c_uint
            >= RD_OPT_BASIC as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"rd_opt >= RD_OPT_BASIC\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/frame_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                805 as ::core::ffi::c_uint,
                b"int VP8EncTokenLoop(VP8Encoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if num_pass_left > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"num_pass_left > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/frame_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                806 as ::core::ffi::c_uint,
                b"int VP8EncTokenLoop(VP8Encoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    while ok != 0 && {
        let fresh4 = num_pass_left;
        num_pass_left = num_pass_left - 1;
        fresh4 > 0 as ::core::ffi::c_int
    } {
        let is_last_pass: ::core::ffi::c_int = (fabs(stats.dq as ::core::ffi::c_double) <= DQ_LIMIT
            || num_pass_left == 0 as ::core::ffi::c_int
            || (*enc).max_i4_header_bits_ == 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        let mut size_p0: uint64_t = 0 as uint64_t;
        let mut distortion: uint64_t = 0 as uint64_t;
        let mut cnt: ::core::ffi::c_int = max_count;
        let pass_progress: ::core::ffi::c_int =
            remaining_progress / (2 as ::core::ffi::c_int + num_pass_left);
        remaining_progress -= pass_progress;
        VP8IteratorInit(enc, &raw mut it);
        SetLoopParams(enc, stats.q);
        if is_last_pass != 0 {
            ResetTokenStats(enc);
            VP8InitFilter(&raw mut it);
        }
        VP8TBufferClear(&raw mut (*enc).tokens_);
        loop {
            let mut info: VP8ModeScore = VP8ModeScore {
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
            VP8IteratorImport(&raw mut it, ::core::ptr::null_mut::<uint8_t>());
            cnt -= 1;
            if cnt < 0 as ::core::ffi::c_int {
                FinalizeTokenProbas(proba);
                VP8CalculateLevelCosts(proba);
                cnt = max_count;
            }
            VP8Decimate(&raw mut it, &raw mut info, rd_opt);
            ok = RecordTokens(&raw mut it, &raw mut info, &raw mut (*enc).tokens_);
            if ok == 0 {
                WebPEncodingSetError((*enc).pic_, VP8_ENC_ERROR_OUT_OF_MEMORY);
                break;
            } else {
                size_p0 = size_p0.wrapping_add(info.H as uint64_t);
                distortion = distortion.wrapping_add(info.D as uint64_t);
                if is_last_pass != 0 {
                    StoreSideInfo(&raw mut it);
                    VP8StoreFilterStats(&raw mut it);
                    VP8IteratorExport(&raw mut it);
                    ok = VP8IteratorProgress(&raw mut it, pass_progress);
                }
                VP8IteratorSaveBoundary(&raw mut it);
                if !(ok != 0 && VP8IteratorNext(&raw mut it) != 0) {
                    break;
                }
            }
        }
        if ok == 0 {
            break;
        }
        size_p0 = size_p0.wrapping_add((*enc).segment_hdr_.size_ as uint64_t);
        if stats.do_size_search != 0 {
            let mut size: uint64_t = FinalizeTokenProbas(&raw mut (*enc).proba_) as uint64_t;
            size = (size as ::core::ffi::c_ulong).wrapping_add(VP8EstimateTokenSize(
                &raw mut (*enc).tokens_,
                &raw mut (*proba).coeffs_ as *mut [ProbaArray; 8] as *const uint8_t,
            )
                as ::core::ffi::c_ulong) as uint64_t as uint64_t;
            size = size.wrapping_add(size_p0).wrapping_add(1024 as uint64_t)
                >> 11 as ::core::ffi::c_int;
            size = size.wrapping_add(HEADER_SIZE_ESTIMATE as uint64_t);
            stats.value = size as ::core::ffi::c_double;
        } else {
            stats.value = GetPSNR(distortion, pixel_count);
        }
        if (*enc).max_i4_header_bits_ > 0 as ::core::ffi::c_int
            && size_p0 as ::core::ffi::c_ulonglong > PARTITION0_SIZE_LIMIT
        {
            num_pass_left += 1;
            (*enc).max_i4_header_bits_ >>= 1 as ::core::ffi::c_int;
            if is_last_pass != 0 {
                ResetSideInfo(&raw mut it);
            }
        } else {
            if is_last_pass != 0 {
                break;
            }
            if do_search != 0 {
                ComputeNextQ(&raw mut stats);
            }
        }
    }
    if ok != 0 {
        if stats.do_size_search == 0 {
            FinalizeTokenProbas(&raw mut (*enc).proba_);
        }
        ok = VP8EmitTokens(
            &raw mut (*enc).tokens_,
            (&raw mut (*enc).parts_ as *mut VP8BitWriter).offset(0 as ::core::ffi::c_int as isize),
            &raw mut (*proba).coeffs_ as *mut [ProbaArray; 8] as *const uint8_t,
            1 as ::core::ffi::c_int,
        );
    }
    ok = (ok != 0
        && WebPReportProgress(
            (*enc).pic_,
            (*enc).percent_ + remaining_progress,
            &raw mut (*enc).percent_,
        ) != 0) as ::core::ffi::c_int;
    return PostLoopFinalize(&raw mut it, ok);
}
pub const VP8_MAX_PARTITION0_SIZE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 19 as ::core::ffi::c_int;
pub const VP8_FRAME_HEADER_SIZE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const CHUNK_HEADER_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const RIFF_HEADER_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
