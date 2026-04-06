use c2rust_bitfields::BitfieldStruct;

#[repr(C)]
pub struct VP8Tokens {
    _unused: [u8; 0],
}

extern "C" {
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    static mut VP8Mean16x4: VP8MeanMetric;
    static mut VP8CollectHistogram: VP8CHisto;
    fn WebPGetWorkerInterface() -> *const WebPWorkerInterface;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
    static VP8UVModeOffsets: [uint16_t; 4];
    static VP8I16ModeOffsets: [uint16_t; 4];
    fn VP8IteratorInit(enc: *mut VP8Encoder, it: *mut VP8EncIterator);
    fn VP8IteratorSetRow(it: *mut VP8EncIterator, y: ::core::ffi::c_int);
    fn VP8IteratorSetCountDown(it: *mut VP8EncIterator, count_down: ::core::ffi::c_int);
    fn VP8IteratorIsDone(it: *const VP8EncIterator) -> ::core::ffi::c_int;
    fn VP8IteratorImport(it: *mut VP8EncIterator, tmp_32: *mut uint8_t);
    fn VP8IteratorNext(it: *mut VP8EncIterator) -> ::core::ffi::c_int;
    fn VP8IteratorProgress(
        it: *const VP8EncIterator,
        delta: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VP8SetIntra16Mode(it: *const VP8EncIterator, mode: ::core::ffi::c_int);
    fn VP8SetIntra4Mode(it: *const VP8EncIterator, modes: *const uint8_t);
    fn VP8SetIntraUVMode(it: *const VP8EncIterator, mode: ::core::ffi::c_int);
    fn VP8SetSkip(it: *const VP8EncIterator, skip: ::core::ffi::c_int);
    fn VP8SetSegment(it: *const VP8EncIterator, segment: ::core::ffi::c_int);
    fn VP8MakeLuma16Preds(it: *const VP8EncIterator);
    fn VP8MakeChroma8Preds(it: *const VP8EncIterator);
    fn WebPEncodingSetError(
        pic: *const WebPPicture,
        error: WebPEncodingError,
    ) -> ::core::ffi::c_int;
    fn WebPReportProgress(
        pic: *const WebPPicture,
        percent: ::core::ffi::c_int,
        percent_store: *mut ::core::ffi::c_int,
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
pub type uintptr_t = usize;
pub type VP8MeanMetric = Option<unsafe extern "C" fn(*const uint8_t, *mut uint32_t) -> ()>;
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
pub struct VP8Histogram {
    pub max_value: ::core::ffi::c_int,
    pub last_non_zero: ::core::ffi::c_int,
}
pub type VP8CHisto = Option<
    unsafe extern "C" fn(
        *const uint8_t,
        *const uint8_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut VP8Histogram,
    ) -> (),
>;
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
pub struct SegmentJob {
    pub worker: WebPWorker,
    pub alphas: [::core::ffi::c_int; 256],
    pub alpha: ::core::ffi::c_int,
    pub uv_alpha: ::core::ffi::c_int,
    pub it: VP8EncIterator,
    pub delta_progress: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BPS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const WEBP_ALIGN_CST: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const Y_OFF_ENC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const U_OFF_ENC: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MAX_ITERS_K_MEANS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
unsafe extern "C" fn SmoothSegmentMap(enc: *mut VP8Encoder) {
    let mut n: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let w: ::core::ffi::c_int = (*enc).mb_w_;
    let h: ::core::ffi::c_int = (*enc).mb_h_;
    let majority_cnt_3_x_3_grid: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let tmp: *mut uint8_t = WebPSafeMalloc(
        (w * h) as uint64_t,
        ::core::mem::size_of::<uint8_t>() as size_t,
    ) as *mut uint8_t;
    '_c2rust_label: {
        if (w * h) as uint64_t == (w as uint64_t).wrapping_mul(h as uint64_t) {
        } else {
            __assert_fail(
                b"(uint64_t)(w * h) == (uint64_t)w * h\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/analysis_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                34 as ::core::ffi::c_uint,
                b"void SmoothSegmentMap(VP8Encoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if tmp.is_null() {
        return;
    }
    y = 1 as ::core::ffi::c_int;
    while y < h - 1 as ::core::ffi::c_int {
        x = 1 as ::core::ffi::c_int;
        while x < w - 1 as ::core::ffi::c_int {
            let mut cnt: [::core::ffi::c_int; 4] = [0 as ::core::ffi::c_int; 4];
            let mb: *const VP8MBInfo =
                (*enc).mb_info_.offset((x + w * y) as isize) as *mut VP8MBInfo;
            let mut majority_seg: ::core::ffi::c_int = (*mb).segment_() as ::core::ffi::c_int;
            cnt[(*mb.offset((-w - 1 as ::core::ffi::c_int) as isize)).segment_() as usize] += 1;
            cnt[(*mb.offset((-w + 0 as ::core::ffi::c_int) as isize)).segment_() as usize] += 1;
            cnt[(*mb.offset((-w + 1 as ::core::ffi::c_int) as isize)).segment_() as usize] += 1;
            cnt[(*mb.offset(-(1 as ::core::ffi::c_int) as isize)).segment_() as usize] += 1;
            cnt[(*mb.offset(1 as ::core::ffi::c_int as isize)).segment_() as usize] += 1;
            cnt[(*mb.offset((w - 1 as ::core::ffi::c_int) as isize)).segment_() as usize] += 1;
            cnt[(*mb.offset((w + 0 as ::core::ffi::c_int) as isize)).segment_() as usize] += 1;
            cnt[(*mb.offset((w + 1 as ::core::ffi::c_int) as isize)).segment_() as usize] += 1;
            n = 0 as ::core::ffi::c_int;
            while n < NUM_MB_SEGMENTS as ::core::ffi::c_int {
                if cnt[n as usize] >= majority_cnt_3_x_3_grid {
                    majority_seg = n;
                    break;
                } else {
                    n += 1;
                }
            }
            *tmp.offset((x + y * w) as isize) = majority_seg as uint8_t;
            x += 1;
        }
        y += 1;
    }
    y = 1 as ::core::ffi::c_int;
    while y < h - 1 as ::core::ffi::c_int {
        x = 1 as ::core::ffi::c_int;
        while x < w - 1 as ::core::ffi::c_int {
            let mb_0: *mut VP8MBInfo =
                (*enc).mb_info_.offset((x + w * y) as isize) as *mut VP8MBInfo;
            (*mb_0).set_segment_(
                *tmp.offset((x + y * w) as isize) as ::core::ffi::c_uint as ::core::ffi::c_uint
            );
            x += 1;
        }
        y += 1;
    }
    WebPSafeFree(tmp as *mut ::core::ffi::c_void);
}
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
unsafe extern "C" fn SetSegmentAlphas(
    enc: *mut VP8Encoder,
    mut centers: *const ::core::ffi::c_int,
    mut mid: ::core::ffi::c_int,
) {
    let nb: ::core::ffi::c_int = (*enc).segment_hdr_.num_segments_;
    let mut min: ::core::ffi::c_int = *centers.offset(0 as ::core::ffi::c_int as isize);
    let mut max: ::core::ffi::c_int = *centers.offset(0 as ::core::ffi::c_int as isize);
    let mut n: ::core::ffi::c_int = 0;
    if nb > 1 as ::core::ffi::c_int {
        n = 0 as ::core::ffi::c_int;
        while n < nb {
            if min > *centers.offset(n as isize) {
                min = *centers.offset(n as isize);
            }
            if max < *centers.offset(n as isize) {
                max = *centers.offset(n as isize);
            }
            n += 1;
        }
    }
    if max == min {
        max = min + 1 as ::core::ffi::c_int;
    }
    '_c2rust_label: {
        if mid <= max && mid >= min {
        } else {
            __assert_fail(
                b"mid <= max && mid >= min\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/analysis_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                90 as ::core::ffi::c_uint,
                b"void SetSegmentAlphas(VP8Encoder *const, const int *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    n = 0 as ::core::ffi::c_int;
    while n < nb {
        let alpha: ::core::ffi::c_int =
            255 as ::core::ffi::c_int * (*centers.offset(n as isize) - mid) / (max - min);
        let beta: ::core::ffi::c_int =
            255 as ::core::ffi::c_int * (*centers.offset(n as isize) - min) / (max - min);
        (*enc).dqm_[n as usize].alpha_ = clip(
            alpha,
            -(127 as ::core::ffi::c_int),
            127 as ::core::ffi::c_int,
        );
        (*enc).dqm_[n as usize].beta_ =
            clip(beta, 0 as ::core::ffi::c_int, 255 as ::core::ffi::c_int);
        n += 1;
    }
}
pub const MAX_ALPHA: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const ALPHA_SCALE: ::core::ffi::c_int = 2 as ::core::ffi::c_int * MAX_ALPHA;
pub const DEFAULT_ALPHA: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
unsafe extern "C" fn FinalAlphaValue(mut alpha: ::core::ffi::c_int) -> ::core::ffi::c_int {
    alpha = MAX_ALPHA - alpha;
    return clip(alpha, 0 as ::core::ffi::c_int, MAX_ALPHA);
}
unsafe extern "C" fn GetAlpha(histo: *const VP8Histogram) -> ::core::ffi::c_int {
    let max_value: ::core::ffi::c_int = (*histo).max_value;
    let last_non_zero: ::core::ffi::c_int = (*histo).last_non_zero;
    let alpha: ::core::ffi::c_int = if max_value > 1 as ::core::ffi::c_int {
        ALPHA_SCALE * last_non_zero / max_value
    } else {
        0 as ::core::ffi::c_int
    };
    return alpha;
}
unsafe extern "C" fn InitHistogram(histo: *mut VP8Histogram) {
    (*histo).max_value = 0 as ::core::ffi::c_int;
    (*histo).last_non_zero = 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn AssignSegments(enc: *mut VP8Encoder, mut alphas: *const ::core::ffi::c_int) {
    let nb: ::core::ffi::c_int =
        if (*enc).segment_hdr_.num_segments_ < NUM_MB_SEGMENTS as ::core::ffi::c_int {
            (*enc).segment_hdr_.num_segments_
        } else {
            NUM_MB_SEGMENTS as ::core::ffi::c_int as ::core::ffi::c_int
        };
    let mut centers: [::core::ffi::c_int; 4] = [0; 4];
    let mut weighted_average: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut map: [::core::ffi::c_int; 256] = [0; 256];
    let mut a: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut min_a: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut max_a: ::core::ffi::c_int = MAX_ALPHA;
    let mut range_a: ::core::ffi::c_int = 0;
    let mut accum: [::core::ffi::c_int; 4] = [0; 4];
    let mut dist_accum: [::core::ffi::c_int; 4] = [0; 4];
    '_c2rust_label: {
        if nb >= 1 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"nb >= 1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/analysis_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                147 as ::core::ffi::c_uint,
                b"void AssignSegments(VP8Encoder *const, const int *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if nb <= NUM_MB_SEGMENTS as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"nb <= NUM_MB_SEGMENTS\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/analysis_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                148 as ::core::ffi::c_uint,
                b"void AssignSegments(VP8Encoder *const, const int *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    n = 0 as ::core::ffi::c_int;
    while n <= MAX_ALPHA && *alphas.offset(n as isize) == 0 as ::core::ffi::c_int {
        n += 1;
    }
    min_a = n;
    n = MAX_ALPHA;
    while n > min_a && *alphas.offset(n as isize) == 0 as ::core::ffi::c_int {
        n -= 1;
    }
    max_a = n;
    range_a = max_a - min_a;
    k = 0 as ::core::ffi::c_int;
    n = 1 as ::core::ffi::c_int;
    while k < nb {
        '_c2rust_label_1: {
            if n < 2 as ::core::ffi::c_int * nb {
            } else {
                __assert_fail(
                    b"n < 2 * nb\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/analysis_enc.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    159 as ::core::ffi::c_uint,
                    b"void AssignSegments(VP8Encoder *const, const int *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        centers[k as usize] = min_a + n * range_a / (2 as ::core::ffi::c_int * nb);
        k += 1;
        n += 2 as ::core::ffi::c_int;
    }
    k = 0 as ::core::ffi::c_int;
    while k < MAX_ITERS_K_MEANS {
        let mut total_weight: ::core::ffi::c_int = 0;
        let mut displaced: ::core::ffi::c_int = 0;
        n = 0 as ::core::ffi::c_int;
        while n < nb {
            accum[n as usize] = 0 as ::core::ffi::c_int;
            dist_accum[n as usize] = 0 as ::core::ffi::c_int;
            n += 1;
        }
        n = 0 as ::core::ffi::c_int;
        a = min_a;
        while a <= max_a {
            if *alphas.offset(a as isize) != 0 {
                while (n + 1 as ::core::ffi::c_int) < nb
                    && abs(a - centers[(n + 1 as ::core::ffi::c_int) as usize])
                        < abs(a - centers[n as usize])
                {
                    n += 1;
                }
                map[a as usize] = n;
                dist_accum[n as usize] += a * *alphas.offset(a as isize);
                accum[n as usize] += *alphas.offset(a as isize);
            }
            a += 1;
        }
        displaced = 0 as ::core::ffi::c_int;
        weighted_average = 0 as ::core::ffi::c_int;
        total_weight = 0 as ::core::ffi::c_int;
        n = 0 as ::core::ffi::c_int;
        while n < nb {
            if accum[n as usize] != 0 {
                let new_center: ::core::ffi::c_int = (dist_accum[n as usize]
                    + accum[n as usize] / 2 as ::core::ffi::c_int)
                    / accum[n as usize];
                displaced += abs(centers[n as usize] - new_center);
                centers[n as usize] = new_center;
                weighted_average += new_center * accum[n as usize];
                total_weight += accum[n as usize];
            }
            n += 1;
        }
        weighted_average =
            (weighted_average + total_weight / 2 as ::core::ffi::c_int) / total_weight;
        if displaced < 5 as ::core::ffi::c_int {
            break;
        }
        k += 1;
    }
    n = 0 as ::core::ffi::c_int;
    while n < (*enc).mb_w_ * (*enc).mb_h_ {
        let mb: *mut VP8MBInfo = (*enc).mb_info_.offset(n as isize) as *mut VP8MBInfo;
        let alpha: ::core::ffi::c_int = (*mb).alpha_ as ::core::ffi::c_int;
        (*mb).set_segment_(map[alpha as usize] as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*mb).alpha_ = centers[map[alpha as usize] as usize] as uint8_t;
        n += 1;
    }
    if nb > 1 as ::core::ffi::c_int {
        let smooth: ::core::ffi::c_int = (*(*enc).config_).preprocessing & 1 as ::core::ffi::c_int;
        if smooth != 0 {
            SmoothSegmentMap(enc);
        }
    }
    SetSegmentAlphas(
        enc,
        &raw mut centers as *mut ::core::ffi::c_int as *const ::core::ffi::c_int,
        weighted_average,
    );
}
pub const MAX_INTRA16_MODE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAX_UV_MODE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn MBAnalyzeBestIntra16Mode(it: *mut VP8EncIterator) -> ::core::ffi::c_int {
    let max_mode: ::core::ffi::c_int = MAX_INTRA16_MODE;
    let mut mode: ::core::ffi::c_int = 0;
    let mut best_alpha: ::core::ffi::c_int = DEFAULT_ALPHA;
    let mut best_mode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    VP8MakeLuma16Preds(it);
    mode = 0 as ::core::ffi::c_int;
    while mode < max_mode {
        let mut histo: VP8Histogram = VP8Histogram {
            max_value: 0,
            last_non_zero: 0,
        };
        let mut alpha: ::core::ffi::c_int = 0;
        InitHistogram(&raw mut histo);
        VP8CollectHistogram.expect("non-null function pointer")(
            (*it).yuv_in_.offset(Y_OFF_ENC as isize),
            (*it)
                .yuv_p_
                .offset(VP8I16ModeOffsets[mode as usize] as ::core::ffi::c_int as isize),
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            &raw mut histo,
        );
        alpha = GetAlpha(&raw mut histo);
        if alpha > best_alpha {
            best_alpha = alpha;
            best_mode = mode;
        }
        mode += 1;
    }
    VP8SetIntra16Mode(it, best_mode);
    return best_alpha;
}
unsafe extern "C" fn FastMBAnalyze(it: *mut VP8EncIterator) -> ::core::ffi::c_int {
    let q: ::core::ffi::c_int = (*(*(*it).enc_).config_).quality as ::core::ffi::c_int;
    let kThreshold: uint32_t = (8 as ::core::ffi::c_int
        + (17 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) * q / 100 as ::core::ffi::c_int)
        as uint32_t;
    let mut k: ::core::ffi::c_int = 0;
    let mut dc: [uint32_t; 16] = [0; 16];
    let mut m: uint32_t = 0;
    let mut m2: uint32_t = 0;
    k = 0 as ::core::ffi::c_int;
    while k < 16 as ::core::ffi::c_int {
        VP8Mean16x4.expect("non-null function pointer")(
            (*it)
                .yuv_in_
                .offset(Y_OFF_ENC as isize)
                .offset((k * BPS) as isize),
            (&raw mut dc as *mut uint32_t).offset(k as isize) as *mut uint32_t,
        );
        k += 4 as ::core::ffi::c_int;
    }
    m = 0 as uint32_t;
    m2 = 0 as uint32_t;
    k = 0 as ::core::ffi::c_int;
    while k < 16 as ::core::ffi::c_int {
        m = m.wrapping_add(dc[k as usize]);
        m2 = m2.wrapping_add(dc[k as usize].wrapping_mul(dc[k as usize]));
        k += 1;
    }
    if kThreshold.wrapping_mul(m2) < m.wrapping_mul(m) {
        VP8SetIntra16Mode(it, 0 as ::core::ffi::c_int);
    } else {
        let modes: [uint8_t; 16] = [
            0 as ::core::ffi::c_int as uint8_t,
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
        VP8SetIntra4Mode(it, &raw const modes as *const uint8_t);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn MBAnalyzeBestUVMode(it: *mut VP8EncIterator) -> ::core::ffi::c_int {
    let mut best_alpha: ::core::ffi::c_int = DEFAULT_ALPHA;
    let mut smallest_alpha: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut best_mode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let max_mode: ::core::ffi::c_int = MAX_UV_MODE;
    let mut mode: ::core::ffi::c_int = 0;
    VP8MakeChroma8Preds(it);
    mode = 0 as ::core::ffi::c_int;
    while mode < max_mode {
        let mut histo: VP8Histogram = VP8Histogram {
            max_value: 0,
            last_non_zero: 0,
        };
        let mut alpha: ::core::ffi::c_int = 0;
        InitHistogram(&raw mut histo);
        VP8CollectHistogram.expect("non-null function pointer")(
            (*it).yuv_in_.offset(U_OFF_ENC as isize),
            (*it)
                .yuv_p_
                .offset(VP8UVModeOffsets[mode as usize] as ::core::ffi::c_int as isize),
            16 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int,
            &raw mut histo,
        );
        alpha = GetAlpha(&raw mut histo);
        if alpha > best_alpha {
            best_alpha = alpha;
        }
        if mode == 0 as ::core::ffi::c_int || alpha < smallest_alpha {
            smallest_alpha = alpha;
            best_mode = mode;
        }
        mode += 1;
    }
    VP8SetIntraUVMode(it, best_mode);
    return best_alpha;
}
unsafe extern "C" fn MBAnalyze(
    it: *mut VP8EncIterator,
    mut alphas: *mut ::core::ffi::c_int,
    alpha: *mut ::core::ffi::c_int,
    uv_alpha: *mut ::core::ffi::c_int,
) {
    let enc: *const VP8Encoder = (*it).enc_;
    let mut best_alpha: ::core::ffi::c_int = 0;
    let mut best_uv_alpha: ::core::ffi::c_int = 0;
    VP8SetIntra16Mode(it, 0 as ::core::ffi::c_int);
    VP8SetSkip(it, 0 as ::core::ffi::c_int);
    VP8SetSegment(it, 0 as ::core::ffi::c_int);
    if (*enc).method_ <= 1 as ::core::ffi::c_int {
        best_alpha = FastMBAnalyze(it);
    } else {
        best_alpha = MBAnalyzeBestIntra16Mode(it);
    }
    best_uv_alpha = MBAnalyzeBestUVMode(it);
    best_alpha = 3 as ::core::ffi::c_int * best_alpha + best_uv_alpha + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int;
    best_alpha = FinalAlphaValue(best_alpha);
    let ref mut fresh0 = *alphas.offset(best_alpha as isize);
    *fresh0 += 1;
    (*(*it).mb_).alpha_ = best_alpha as uint8_t;
    *alpha += best_alpha;
    *uv_alpha += best_uv_alpha;
}
unsafe extern "C" fn DefaultMBInfo(mb: *mut VP8MBInfo) {
    (*mb).set_type_(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*mb).set_uv_mode_(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*mb).set_skip_(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*mb).set_segment_(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*mb).alpha_ = 0 as uint8_t;
}
unsafe extern "C" fn ResetAllMBInfo(enc: *mut VP8Encoder) {
    let mut n: ::core::ffi::c_int = 0;
    n = 0 as ::core::ffi::c_int;
    while n < (*enc).mb_w_ * (*enc).mb_h_ {
        DefaultMBInfo((*enc).mb_info_.offset(n as isize) as *mut VP8MBInfo);
        n += 1;
    }
    (*enc).dqm_[0 as ::core::ffi::c_int as usize].alpha_ = 0 as ::core::ffi::c_int;
    (*enc).dqm_[0 as ::core::ffi::c_int as usize].beta_ = 0 as ::core::ffi::c_int;
    (*enc).alpha_ = 0 as ::core::ffi::c_int;
    (*enc).uv_alpha_ = 0 as ::core::ffi::c_int;
    WebPReportProgress(
        (*enc).pic_,
        (*enc).percent_ + 20 as ::core::ffi::c_int,
        &raw mut (*enc).percent_,
    );
}
unsafe extern "C" fn DoSegmentsJob(
    mut arg1: *mut ::core::ffi::c_void,
    mut arg2: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let job: *mut SegmentJob = arg1 as *mut SegmentJob;
    let it: *mut VP8EncIterator = arg2 as *mut VP8EncIterator;
    let mut ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if VP8IteratorIsDone(it) == 0 {
        let mut tmp: [uint8_t; 63] = [0; 63];
        let scratch: *mut uint8_t = ((&raw mut tmp as *mut uint8_t as uintptr_t)
            .wrapping_add(WEBP_ALIGN_CST as uintptr_t)
            & !(WEBP_ALIGN_CST as uintptr_t)) as *mut uint8_t;
        loop {
            VP8IteratorImport(it, scratch);
            MBAnalyze(
                it,
                &raw mut (*job).alphas as *mut ::core::ffi::c_int,
                &raw mut (*job).alpha,
                &raw mut (*job).uv_alpha,
            );
            ok = VP8IteratorProgress(it, (*job).delta_progress);
            if !(ok != 0 && VP8IteratorNext(it) != 0) {
                break;
            }
        }
    }
    return ok;
}
unsafe extern "C" fn InitSegmentJob(
    enc: *mut VP8Encoder,
    job: *mut SegmentJob,
    mut start_row: ::core::ffi::c_int,
    mut end_row: ::core::ffi::c_int,
) {
    (*WebPGetWorkerInterface())
        .Init
        .expect("non-null function pointer")(&raw mut (*job).worker);
    (*job).worker.data1 = job as *mut ::core::ffi::c_void;
    (*job).worker.data2 = &raw mut (*job).it as *mut ::core::ffi::c_void;
    (*job).worker.hook = Some(
        DoSegmentsJob
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
    ) as WebPWorkerHook;
    VP8IteratorInit(enc, &raw mut (*job).it);
    VP8IteratorSetRow(&raw mut (*job).it, start_row);
    VP8IteratorSetCountDown(&raw mut (*job).it, (end_row - start_row) * (*enc).mb_w_);
    memset(
        &raw mut (*job).alphas as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_int; 256]>() as size_t,
    );
    (*job).alpha = 0 as ::core::ffi::c_int;
    (*job).uv_alpha = 0 as ::core::ffi::c_int;
    (*job).delta_progress = if start_row == 0 as ::core::ffi::c_int {
        20 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8EncAnalyze(enc: *mut VP8Encoder) -> ::core::ffi::c_int {
    let mut ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let do_segments: ::core::ffi::c_int = ((*(*enc).config_).emulate_jpeg_size != 0
        || (*enc).segment_hdr_.num_segments_ > 1 as ::core::ffi::c_int
        || (*enc).method_ <= 1 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    if do_segments != 0 {
        let last_row: ::core::ffi::c_int = (*enc).mb_h_;
        let total_mb: ::core::ffi::c_int = last_row * (*enc).mb_w_;
        let do_mt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let worker_interface: *const WebPWorkerInterface =
            WebPGetWorkerInterface() as *const WebPWorkerInterface;
        let mut main_job: SegmentJob = SegmentJob {
            worker: WebPWorker {
                impl_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                status_: NOT_OK,
                hook: None,
                data1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                data2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                had_error: 0,
            },
            alphas: [0; 256],
            alpha: 0,
            uv_alpha: 0,
            it: VP8EncIterator {
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
            },
            delta_progress: 0,
        };
        if !(do_mt != 0) {
            InitSegmentJob(enc, &raw mut main_job, 0 as ::core::ffi::c_int, last_row);
            (*worker_interface)
                .Execute
                .expect("non-null function pointer")(&raw mut main_job.worker);
            ok &= (*worker_interface)
                .Sync_0
                .expect("non-null function pointer")(&raw mut main_job.worker);
        }
        (*worker_interface).End.expect("non-null function pointer")(&raw mut main_job.worker);
        if ok != 0 {
            (*enc).alpha_ = main_job.alpha / total_mb;
            (*enc).uv_alpha_ = main_job.uv_alpha / total_mb;
            AssignSegments(
                enc,
                &raw mut main_job.alphas as *mut ::core::ffi::c_int as *const ::core::ffi::c_int,
            );
        }
    } else {
        ResetAllMBInfo(enc);
    }
    if ok == 0 {
        return WebPEncodingSetError((*enc).pic_, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    return ok;
}
