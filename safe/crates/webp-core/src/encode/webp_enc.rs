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
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn log10(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn VP8EncDspInit();
    fn VP8EncDspCostInit();
    fn VP8TBufferInit(b: *mut VP8TBuffer, page_size: ::core::ffi::c_int);
    fn VP8TBufferClear(b: *mut VP8TBuffer);
    fn VP8DefaultProbas(enc: *mut VP8Encoder);
    fn VP8EncWrite(enc: *mut VP8Encoder) -> ::core::ffi::c_int;
    fn VP8EncFreeBitWriters(enc: *mut VP8Encoder);
    fn VP8EncLoop(enc: *mut VP8Encoder) -> ::core::ffi::c_int;
    fn VP8EncTokenLoop(enc: *mut VP8Encoder) -> ::core::ffi::c_int;
    fn VP8EncAnalyze(enc: *mut VP8Encoder) -> ::core::ffi::c_int;
    fn VP8EncInitAlpha(enc: *mut VP8Encoder);
    fn VP8EncStartAlpha(enc: *mut VP8Encoder) -> ::core::ffi::c_int;
    fn VP8EncFinishAlpha(enc: *mut VP8Encoder) -> ::core::ffi::c_int;
    fn VP8EncDeleteAlpha(enc: *mut VP8Encoder) -> ::core::ffi::c_int;
    fn WebPValidatePicture(picture: *const WebPPicture) -> ::core::ffi::c_int;
    fn WebPReplaceTransparentPixels(pic: *mut WebPPicture, color: uint32_t);
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
    fn WebPValidateConfig(config: *const WebPConfig) -> ::core::ffi::c_int;
    fn WebPPictureARGBToYUVADithered(
        picture: *mut WebPPicture,
        colorspace: WebPEncCSP,
        dithering: ::core::ffi::c_float,
    ) -> ::core::ffi::c_int;
    fn WebPPictureSharpARGBToYUVA(picture: *mut WebPPicture) -> ::core::ffi::c_int;
    fn WebPPictureYUVAToARGB(picture: *mut WebPPicture) -> ::core::ffi::c_int;
    fn WebPCleanupTransparentArea(picture: *mut WebPPicture);
    fn VP8LEncodeImage(
        config: *const WebPConfig,
        picture: *const WebPPicture,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Matrix {
    pub q_: [uint16_t; 16],
    pub iq_: [uint16_t; 16],
    pub bias_: [uint32_t; 16],
    pub zthresh_: [uint32_t; 16],
    pub sharpen_: [uint16_t; 16],
}
pub type StatsArray = [[proba_t; 11]; 3];
pub type proba_t = uint32_t;
pub type ProbaArray = [[uint8_t; 11]; 3];
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
pub type LFStats = [[::core::ffi::c_double; 64]; 4];
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
pub type VP8RDLevel = ::core::ffi::c_uint;
pub const RD_OPT_TRELLIS_ALL: VP8RDLevel = 3;
pub const RD_OPT_TRELLIS: VP8RDLevel = 2;
pub const RD_OPT_BASIC: VP8RDLevel = 1;
pub const RD_OPT_NONE: VP8RDLevel = 0;
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
pub type CostArrayMap = [[*const uint16_t; 3]; 16];
pub type CostArray = [[uint16_t; 68]; 3];
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
pub type score_t = int64_t;
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
pub const ENC_MAJ_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENC_MIN_VERSION: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ENC_REV_VERSION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ERROR_DIFFUSION_QUALITY: ::core::ffi::c_int = 98 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn WebPGetEncoderVersion() -> ::core::ffi::c_int {
    return ENC_MAJ_VERSION << 16 as ::core::ffi::c_int
        | ENC_MIN_VERSION << 8 as ::core::ffi::c_int
        | ENC_REV_VERSION;
}
unsafe extern "C" fn ResetSegmentHeader(enc: *mut VP8Encoder) {
    let hdr: *mut VP8EncSegmentHeader = &raw mut (*enc).segment_hdr_;
    (*hdr).num_segments_ = (*(*enc).config_).segments;
    (*hdr).update_map_ = ((*hdr).num_segments_ > 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*hdr).size_ = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ResetFilterHeader(enc: *mut VP8Encoder) {
    let hdr: *mut VP8EncFilterHeader = &raw mut (*enc).filter_hdr_;
    (*hdr).simple_ = 1 as ::core::ffi::c_int;
    (*hdr).level_ = 0 as ::core::ffi::c_int;
    (*hdr).sharpness_ = 0 as ::core::ffi::c_int;
    (*hdr).i4x4_lf_delta_ = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ResetBoundaryPredictions(enc: *mut VP8Encoder) {
    let mut i: ::core::ffi::c_int = 0;
    let top: *mut uint8_t = (*enc).preds_.offset(-((*enc).preds_w_ as isize));
    let left: *mut uint8_t = (*enc).preds_.offset(-(1 as ::core::ffi::c_int as isize));
    i = -(1 as ::core::ffi::c_int);
    while i < 4 as ::core::ffi::c_int * (*enc).mb_w_ {
        *top.offset(i as isize) = B_DC_PRED as ::core::ffi::c_int as uint8_t;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int * (*enc).mb_h_ {
        *left.offset((i * (*enc).preds_w_) as isize) = B_DC_PRED as ::core::ffi::c_int as uint8_t;
        i += 1;
    }
    *(*enc).nz_.offset(-(1 as ::core::ffi::c_int) as isize) = 0 as uint32_t;
}
unsafe extern "C" fn MapConfigToTools(enc: *mut VP8Encoder) {
    let config: *const WebPConfig = (*enc).config_;
    let method: ::core::ffi::c_int = (*config).method;
    let limit: ::core::ffi::c_int = 100 as ::core::ffi::c_int - (*config).partition_limit;
    (*enc).method_ = method;
    (*enc).rd_opt_level_ = (if method >= 6 as ::core::ffi::c_int {
        RD_OPT_TRELLIS_ALL as ::core::ffi::c_int
    } else if method >= 5 as ::core::ffi::c_int {
        RD_OPT_TRELLIS as ::core::ffi::c_int
    } else if method >= 3 as ::core::ffi::c_int {
        RD_OPT_BASIC as ::core::ffi::c_int
    } else {
        RD_OPT_NONE as ::core::ffi::c_int
    }) as VP8RDLevel;
    (*enc).max_i4_header_bits_ = 256 as ::core::ffi::c_int
        * 16 as ::core::ffi::c_int
        * 16 as ::core::ffi::c_int
        * (limit * limit)
        / (100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int);
    (*enc).mb_header_limit_ =
        (256 as ::core::ffi::c_int as score_t * 510 as score_t * 8 as score_t * 1024 as score_t
            / ((*enc).mb_w_ * (*enc).mb_h_) as score_t) as ::core::ffi::c_int;
    (*enc).thread_level_ = (*config).thread_level;
    (*enc).do_search_ = ((*config).target_size > 0 as ::core::ffi::c_int
        || (*config).target_PSNR > 0 as ::core::ffi::c_int as ::core::ffi::c_float)
        as ::core::ffi::c_int;
    if (*config).low_memory == 0 {
        (*enc).use_tokens_ = ((*enc).rd_opt_level_ as ::core::ffi::c_uint
            >= RD_OPT_BASIC as ::core::ffi::c_int as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        if (*enc).use_tokens_ != 0 {
            (*enc).num_parts_ = 1 as ::core::ffi::c_int;
        }
    }
}
unsafe extern "C" fn InitVP8Encoder(
    config: *const WebPConfig,
    picture: *mut WebPPicture,
) -> *mut VP8Encoder {
    let mut enc: *mut VP8Encoder = ::core::ptr::null_mut::<VP8Encoder>();
    let use_filter: ::core::ffi::c_int = ((*config).filter_strength > 0 as ::core::ffi::c_int
        || (*config).autofilter > 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    let mb_w: ::core::ffi::c_int =
        (*picture).width + 15 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int;
    let mb_h: ::core::ffi::c_int =
        (*picture).height + 15 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int;
    let preds_w: ::core::ffi::c_int = 4 as ::core::ffi::c_int * mb_w + 1 as ::core::ffi::c_int;
    let preds_h: ::core::ffi::c_int = 4 as ::core::ffi::c_int * mb_h + 1 as ::core::ffi::c_int;
    let preds_size: size_t =
        ((preds_w * preds_h) as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t);
    let top_stride: ::core::ffi::c_int = mb_w * 16 as ::core::ffi::c_int;
    let nz_size: size_t = ((mb_w + 1 as ::core::ffi::c_int) as size_t)
        .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t)
        .wrapping_add(WEBP_ALIGN_CST as size_t);
    let info_size: size_t =
        ((mb_w * mb_h) as size_t).wrapping_mul(::core::mem::size_of::<VP8MBInfo>() as size_t);
    let samples_size: size_t = ((2 as ::core::ffi::c_int * top_stride) as size_t)
        .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t)
        .wrapping_add(WEBP_ALIGN_CST as size_t);
    let lf_stats_size: size_t = if (*config).autofilter != 0 {
        (::core::mem::size_of::<LFStats>() as size_t).wrapping_add(WEBP_ALIGN_CST as size_t)
    } else {
        0 as size_t
    };
    let top_derr_size: size_t = if (*config).quality
        <= ERROR_DIFFUSION_QUALITY as ::core::ffi::c_float
        || (*config).pass > 1 as ::core::ffi::c_int
    {
        (mb_w as size_t).wrapping_mul(::core::mem::size_of::<DError>() as size_t)
    } else {
        0 as size_t
    };
    let mut mem: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let size: uint64_t = (::core::mem::size_of::<VP8Encoder>() as uint64_t)
        .wrapping_add(WEBP_ALIGN_CST as uint64_t)
        .wrapping_add(info_size as uint64_t)
        .wrapping_add(preds_size as uint64_t)
        .wrapping_add(samples_size as uint64_t)
        .wrapping_add(top_derr_size as uint64_t)
        .wrapping_add(nz_size as uint64_t)
        .wrapping_add(lf_stats_size as uint64_t);
    mem = WebPSafeMalloc(size, ::core::mem::size_of::<uint8_t>() as size_t) as *mut uint8_t;
    if mem.is_null() {
        WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
        return ::core::ptr::null_mut::<VP8Encoder>();
    }
    enc = mem as *mut VP8Encoder;
    mem = ((mem.offset(::core::mem::size_of::<VP8Encoder>() as usize as isize) as uintptr_t)
        .wrapping_add(WEBP_ALIGN_CST as uintptr_t)
        & !(WEBP_ALIGN_CST as uintptr_t)) as *mut uint8_t;
    memset(
        enc as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VP8Encoder>() as size_t,
    );
    (*enc).num_parts_ = (1 as ::core::ffi::c_int) << (*config).partitions;
    (*enc).mb_w_ = mb_w;
    (*enc).mb_h_ = mb_h;
    (*enc).preds_w_ = preds_w;
    (*enc).mb_info_ = mem as *mut VP8MBInfo;
    mem = mem.offset(info_size as isize);
    (*enc).preds_ = mem
        .offset(1 as ::core::ffi::c_int as isize)
        .offset((*enc).preds_w_ as isize);
    mem = mem.offset(preds_size as isize);
    (*enc).nz_ = (((mem as uintptr_t).wrapping_add(WEBP_ALIGN_CST as uintptr_t)
        & !(WEBP_ALIGN_CST as uintptr_t)) as *mut uint32_t)
        .offset(1 as ::core::ffi::c_int as isize);
    mem = mem.offset(nz_size as isize);
    (*enc).lf_stats_ = if lf_stats_size != 0 {
        ((mem as uintptr_t).wrapping_add(WEBP_ALIGN_CST as uintptr_t)
            & !(WEBP_ALIGN_CST as uintptr_t)) as *mut LFStats
    } else {
        ::core::ptr::null_mut::<LFStats>()
    };
    mem = mem.offset(lf_stats_size as isize);
    mem = ((mem as uintptr_t).wrapping_add(WEBP_ALIGN_CST as uintptr_t)
        & !(WEBP_ALIGN_CST as uintptr_t)) as *mut uint8_t;
    (*enc).y_top_ = mem;
    (*enc).uv_top_ = (*enc).y_top_.offset(top_stride as isize);
    mem = mem.offset((2 as ::core::ffi::c_int * top_stride) as isize);
    (*enc).top_derr_ = if top_derr_size != 0 {
        mem as *mut DError
    } else {
        ::core::ptr::null_mut::<DError>()
    };
    mem = mem.offset(top_derr_size as isize);
    '_c2rust_label: {
        if mem <= (enc as *mut uint8_t).offset(size as isize) {
        } else {
            __assert_fail(
                b"mem <= (uint8_t*)enc + size\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/webp_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                229 as ::core::ffi::c_uint,
                b"VP8Encoder *InitVP8Encoder(const WebPConfig *const, WebPPicture *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    (*enc).config_ = config;
    (*enc).profile_ = if use_filter != 0 {
        if (*config).filter_type == 1 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }
    } else {
        2 as ::core::ffi::c_int
    };
    (*enc).pic_ = picture;
    (*enc).percent_ = 0 as ::core::ffi::c_int;
    MapConfigToTools(enc);
    VP8EncDspInit();
    VP8DefaultProbas(enc);
    ResetSegmentHeader(enc);
    ResetFilterHeader(enc);
    ResetBoundaryPredictions(enc);
    VP8EncDspCostInit();
    VP8EncInitAlpha(enc);
    let scale: ::core::ffi::c_float = 1.0f32 + (*config).quality * 5.0f32 / 100.0f32;
    VP8TBufferInit(
        &raw mut (*enc).tokens_,
        ((mb_w * mb_h * 4 as ::core::ffi::c_int) as ::core::ffi::c_float * scale)
            as ::core::ffi::c_int,
    );
    return enc;
}
unsafe extern "C" fn DeleteVP8Encoder(mut enc: *mut VP8Encoder) -> ::core::ffi::c_int {
    let mut ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if !enc.is_null() {
        ok = VP8EncDeleteAlpha(enc);
        VP8TBufferClear(&raw mut (*enc).tokens_);
        WebPSafeFree(enc as *mut ::core::ffi::c_void);
    }
    return ok;
}
unsafe extern "C" fn GetPSNR(mut err: uint64_t, mut size: uint64_t) -> ::core::ffi::c_double {
    return if err > 0 as uint64_t && size > 0 as uint64_t {
        10.0f64
            * log10(
                255.0f64 * 255.0f64 * size as ::core::ffi::c_double / err as ::core::ffi::c_double,
            )
    } else {
        99.0f64
    };
}
unsafe extern "C" fn FinalizePSNR(enc: *const VP8Encoder) {
    let mut stats: *mut WebPAuxStats = (*(*enc).pic_).stats;
    let size: uint64_t = (*enc).sse_count_;
    let sse: *const uint64_t = &raw const (*enc).sse_ as *const uint64_t;
    (*stats).PSNR[0 as ::core::ffi::c_int as usize] =
        GetPSNR(*sse.offset(0 as ::core::ffi::c_int as isize), size) as ::core::ffi::c_float;
    (*stats).PSNR[1 as ::core::ffi::c_int as usize] = GetPSNR(
        *sse.offset(1 as ::core::ffi::c_int as isize),
        size.wrapping_div(4 as uint64_t),
    ) as ::core::ffi::c_float;
    (*stats).PSNR[2 as ::core::ffi::c_int as usize] = GetPSNR(
        *sse.offset(2 as ::core::ffi::c_int as isize),
        size.wrapping_div(4 as uint64_t),
    ) as ::core::ffi::c_float;
    (*stats).PSNR[3 as ::core::ffi::c_int as usize] = GetPSNR(
        (*sse.offset(0 as ::core::ffi::c_int as isize))
            .wrapping_add(*sse.offset(1 as ::core::ffi::c_int as isize))
            .wrapping_add(*sse.offset(2 as ::core::ffi::c_int as isize)),
        size.wrapping_mul(3 as uint64_t).wrapping_div(2 as uint64_t),
    ) as ::core::ffi::c_float;
    (*stats).PSNR[4 as ::core::ffi::c_int as usize] =
        GetPSNR(*sse.offset(3 as ::core::ffi::c_int as isize), size) as ::core::ffi::c_float;
}
unsafe extern "C" fn StoreStats(enc: *mut VP8Encoder) {
    let stats: *mut WebPAuxStats = (*(*enc).pic_).stats;
    if !stats.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        let mut s: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < NUM_MB_SEGMENTS as ::core::ffi::c_int {
            (*stats).segment_level[i as usize] = (*enc).dqm_[i as usize].fstrength_;
            (*stats).segment_quant[i as usize] = (*enc).dqm_[i as usize].quant_;
            s = 0 as ::core::ffi::c_int;
            while s <= 2 as ::core::ffi::c_int {
                (*stats).residual_bytes[s as usize][i as usize] =
                    (*enc).residual_bytes_[s as usize][i as usize];
                s += 1;
            }
            i += 1;
        }
        FinalizePSNR(enc);
        (*stats).coded_size = (*enc).coded_size_;
        i = 0 as ::core::ffi::c_int;
        while i < 3 as ::core::ffi::c_int {
            (*stats).block_count[i as usize] = (*enc).block_count_[i as usize];
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPEncodingSetError(
    pic: *const WebPPicture,
    mut error: WebPEncodingError,
) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if (error as ::core::ffi::c_int) < VP8_ENC_ERROR_LAST as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"(int)error < VP8_ENC_ERROR_LAST\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/webp_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                308 as ::core::ffi::c_uint,
                b"int WebPEncodingSetError(const WebPPicture *const, WebPEncodingError)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if error as ::core::ffi::c_int >= VP8_ENC_OK as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"(int)error >= VP8_ENC_OK\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/webp_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                309 as ::core::ffi::c_uint,
                b"int WebPEncodingSetError(const WebPPicture *const, WebPEncodingError)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if (*pic).error_code as ::core::ffi::c_uint
        == VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*(pic as *mut WebPPicture)).error_code = error;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPReportProgress(
    pic: *const WebPPicture,
    mut percent: ::core::ffi::c_int,
    percent_store: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !percent_store.is_null() && percent != *percent_store {
        *percent_store = percent;
        if (*pic).progress_hook.is_some()
            && (*pic).progress_hook.expect("non-null function pointer")(percent, pic) == 0
        {
            return WebPEncodingSetError(pic, VP8_ENC_ERROR_USER_ABORT);
        }
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPEncode(
    mut config: *const WebPConfig,
    mut pic: *mut WebPPicture,
) -> ::core::ffi::c_int {
    let mut ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if pic.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*pic).error_code = VP8_ENC_OK;
    if config.is_null() {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_NULL_PARAMETER);
    }
    if WebPValidateConfig(config) == 0 {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_INVALID_CONFIGURATION);
    }
    if WebPValidatePicture(pic) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*pic).width > WEBP_MAX_DIMENSION || (*pic).height > WEBP_MAX_DIMENSION {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_BAD_DIMENSION);
    }
    if !(*pic).stats.is_null() {
        memset(
            (*pic).stats as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<WebPAuxStats>() as size_t,
        );
    }
    if (*config).lossless == 0 {
        let mut enc: *mut VP8Encoder = ::core::ptr::null_mut::<VP8Encoder>();
        if (*pic).use_argb != 0 || (*pic).y.is_null() || (*pic).u.is_null() || (*pic).v.is_null() {
            if (*config).use_sharp_yuv != 0
                || (*config).preprocessing & 4 as ::core::ffi::c_int != 0
            {
                if WebPPictureSharpARGBToYUVA(pic) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
            } else {
                let mut dithering: ::core::ffi::c_float = 0.0f32;
                if (*config).preprocessing & 2 as ::core::ffi::c_int != 0 {
                    let x: ::core::ffi::c_float = (*config).quality / 100.0f32;
                    let x2: ::core::ffi::c_float = x * x;
                    dithering = 1.0f32 + (0.5f32 - 1.0f32) * x2 * x2;
                }
                if WebPPictureARGBToYUVADithered(pic, WEBP_YUV420, dithering) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
        if (*config).exact == 0 {
            WebPCleanupTransparentArea(pic);
        }
        enc = InitVP8Encoder(config, pic);
        if enc.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        ok = VP8EncAnalyze(enc);
        ok = (ok != 0 && VP8EncStartAlpha(enc) != 0) as ::core::ffi::c_int;
        if (*enc).use_tokens_ == 0 {
            ok = (ok != 0 && VP8EncLoop(enc) != 0) as ::core::ffi::c_int;
        } else {
            ok = (ok != 0 && VP8EncTokenLoop(enc) != 0) as ::core::ffi::c_int;
        }
        ok = (ok != 0 && VP8EncFinishAlpha(enc) != 0) as ::core::ffi::c_int;
        ok = (ok != 0 && VP8EncWrite(enc) != 0) as ::core::ffi::c_int;
        StoreStats(enc);
        if ok == 0 {
            VP8EncFreeBitWriters(enc);
        }
        ok &= DeleteVP8Encoder(enc);
    } else {
        if (*pic).argb.is_null() && WebPPictureYUVAToARGB(pic) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        if (*config).exact == 0 {
            WebPReplaceTransparentPixels(pic, 0 as uint32_t);
        }
        ok = VP8LEncodeImage(config, pic);
    }
    return ok;
}
pub const WEBP_ALIGN_CST: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const WEBP_MAX_DIMENSION: ::core::ffi::c_int = 16383 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
