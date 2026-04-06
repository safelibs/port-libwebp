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
    static mut WebPDispatchAlphaToGreen: Option<
        unsafe extern "C" fn(
            *const uint8_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut uint32_t,
            ::core::ffi::c_int,
        ) -> (),
    >;
    fn WebPInitAlphaProcessing();
    static mut WebPFilters: [WebPFilterFunc; 4];
    fn VP8FiltersInit();
    fn VP8BitWriterInit(bw: *mut VP8BitWriter, expected_size: size_t) -> ::core::ffi::c_int;
    fn VP8BitWriterWipeOut(bw: *mut VP8BitWriter);
    fn VP8BitWriterAppend(
        bw: *mut VP8BitWriter,
        data: *const uint8_t,
        size: size_t,
    ) -> ::core::ffi::c_int;
    fn VP8LBitWriterInit(bw: *mut VP8LBitWriter, expected_size: size_t) -> ::core::ffi::c_int;
    fn VP8LBitWriterFinish(bw: *mut VP8LBitWriter) -> *mut uint8_t;
    fn VP8LBitWriterWipeOut(bw: *mut VP8LBitWriter);
    fn WebPGetWorkerInterface() -> *const WebPWorkerInterface;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
    fn WebPCopyPlane(
        src: *const uint8_t,
        src_stride: ::core::ffi::c_int,
        dst: *mut uint8_t,
        dst_stride: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    fn WebPConfigInitInternal(
        _: *mut WebPConfig,
        _: WebPPreset,
        _: ::core::ffi::c_float,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPPictureInitInternal(_: *mut WebPPicture, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn WebPPictureAlloc(picture: *mut WebPPicture) -> ::core::ffi::c_int;
    fn WebPPictureFree(picture: *mut WebPPicture);
    fn WebPPictureHasTransparency(picture: *const WebPPicture) -> ::core::ffi::c_int;
    fn WebPEncodingSetError(
        pic: *const WebPPicture,
        error: WebPEncodingError,
    ) -> ::core::ffi::c_int;
    fn WebPReportProgress(
        pic: *const WebPPicture,
        percent: ::core::ffi::c_int,
        percent_store: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPEstimateBestFilter(
        data: *const uint8_t,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        stride: ::core::ffi::c_int,
    ) -> WEBP_FILTER_TYPE;
    fn QuantizeLevels(
        data: *mut uint8_t,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        num_levels: ::core::ffi::c_int,
        sse: *mut uint64_t,
    ) -> ::core::ffi::c_int;
    fn VP8LEncodeStream(
        config: *const WebPConfig,
        picture: *const WebPPicture,
        bw: *mut VP8LBitWriter,
        use_cache: ::core::ffi::c_int,
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
pub type WEBP_FILTER_TYPE = ::core::ffi::c_uint;
pub const WEBP_FILTER_FAST: WEBP_FILTER_TYPE = 6;
pub const WEBP_FILTER_BEST: WEBP_FILTER_TYPE = 5;
pub const WEBP_FILTER_LAST: WEBP_FILTER_TYPE = 4;
pub const WEBP_FILTER_GRADIENT: WEBP_FILTER_TYPE = 3;
pub const WEBP_FILTER_VERTICAL: WEBP_FILTER_TYPE = 2;
pub const WEBP_FILTER_HORIZONTAL: WEBP_FILTER_TYPE = 1;
pub const WEBP_FILTER_NONE: WEBP_FILTER_TYPE = 0;
pub type WebPFilterFunc = Option<
    unsafe extern "C" fn(
        *const uint8_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut uint8_t,
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
pub type vp8l_atype_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LBitWriter {
    pub bits_: vp8l_atype_t,
    pub used_: ::core::ffi::c_int,
    pub buf_: *mut uint8_t,
    pub cur_: *mut uint8_t,
    pub end_: *mut uint8_t,
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
pub type WebPPreset = ::core::ffi::c_uint;
pub const WEBP_PRESET_TEXT: WebPPreset = 5;
pub const WEBP_PRESET_ICON: WebPPreset = 4;
pub const WEBP_PRESET_DRAWING: WebPPreset = 3;
pub const WEBP_PRESET_PHOTO: WebPPreset = 2;
pub const WEBP_PRESET_PICTURE: WebPPreset = 1;
pub const WEBP_PRESET_DEFAULT: WebPPreset = 0;
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
pub struct FilterTrial {
    pub score: size_t,
    pub bw: VP8BitWriter,
    pub stats: WebPAuxStats,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn VP8BitWriterBuf(bw: *const VP8BitWriter) -> *mut uint8_t {
    return (*bw).buf_;
}
#[inline]
unsafe extern "C" fn VP8BitWriterSize(bw: *const VP8BitWriter) -> size_t {
    return (*bw).pos_;
}
#[inline]
unsafe extern "C" fn VP8LBitWriterNumBytes(bw: *const VP8LBitWriter) -> size_t {
    return ((*bw).cur_.offset_from((*bw).buf_) as ::core::ffi::c_long
        + ((*bw).used_ + 7 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as size_t;
}
pub const WEBP_ENCODER_ABI_VERSION: ::core::ffi::c_int = 0x20f as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn WebPConfigInit(mut config: *mut WebPConfig) -> ::core::ffi::c_int {
    return WebPConfigInitInternal(
        config,
        WEBP_PRESET_DEFAULT,
        75.0f32,
        WEBP_ENCODER_ABI_VERSION,
    );
}
#[inline]
unsafe extern "C" fn WebPPictureInit(mut picture: *mut WebPPicture) -> ::core::ffi::c_int {
    return WebPPictureInitInternal(picture, WEBP_ENCODER_ABI_VERSION);
}
pub const ALPHA_HEADER_LEN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ALPHA_NO_COMPRESSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ALPHA_LOSSLESS_COMPRESSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ALPHA_PREPROCESSED_LEVELS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn EncodeLossless(
    data: *const uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut effort_level: ::core::ffi::c_int,
    mut use_quality_100: ::core::ffi::c_int,
    bw: *mut VP8LBitWriter,
    stats: *mut WebPAuxStats,
) -> ::core::ffi::c_int {
    let mut ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut config: WebPConfig = WebPConfig {
        lossless: 0,
        quality: 0.,
        method: 0,
        image_hint: WEBP_HINT_DEFAULT,
        target_size: 0,
        target_PSNR: 0.,
        segments: 0,
        sns_strength: 0,
        filter_strength: 0,
        filter_sharpness: 0,
        filter_type: 0,
        autofilter: 0,
        alpha_compression: 0,
        alpha_filtering: 0,
        alpha_quality: 0,
        pass: 0,
        show_compressed: 0,
        preprocessing: 0,
        partitions: 0,
        partition_limit: 0,
        emulate_jpeg_size: 0,
        thread_level: 0,
        low_memory: 0,
        near_lossless: 0,
        exact: 0,
        use_delta_palette: 0,
        use_sharp_yuv: 0,
        qmin: 0,
        qmax: 0,
    };
    let mut picture: WebPPicture = WebPPicture {
        use_argb: 0,
        colorspace: WEBP_YUV420,
        width: 0,
        height: 0,
        y: ::core::ptr::null_mut::<uint8_t>(),
        u: ::core::ptr::null_mut::<uint8_t>(),
        v: ::core::ptr::null_mut::<uint8_t>(),
        y_stride: 0,
        uv_stride: 0,
        a: ::core::ptr::null_mut::<uint8_t>(),
        a_stride: 0,
        pad1: [0; 2],
        argb: ::core::ptr::null_mut::<uint32_t>(),
        argb_stride: 0,
        pad2: [0; 3],
        writer: None,
        custom_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        extra_info_type: 0,
        extra_info: ::core::ptr::null_mut::<uint8_t>(),
        stats: ::core::ptr::null_mut::<WebPAuxStats>(),
        error_code: VP8_ENC_OK,
        progress_hook: None,
        user_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pad3: [0; 3],
        pad4: ::core::ptr::null_mut::<uint8_t>(),
        pad5: ::core::ptr::null_mut::<uint8_t>(),
        pad6: [0; 8],
        memory_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        memory_argb_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pad7: [::core::ptr::null_mut::<::core::ffi::c_void>(); 2],
    };
    if WebPPictureInit(&raw mut picture) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    picture.width = width;
    picture.height = height;
    picture.use_argb = 1 as ::core::ffi::c_int;
    picture.stats = stats;
    if WebPPictureAlloc(&raw mut picture) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    WebPDispatchAlphaToGreen.expect("non-null function pointer")(
        data,
        width,
        picture.width,
        picture.height,
        picture.argb,
        picture.argb_stride,
    );
    WebPConfigInit(&raw mut config);
    config.lossless = 1 as ::core::ffi::c_int;
    config.exact = 1 as ::core::ffi::c_int;
    config.method = effort_level;
    config.quality = if use_quality_100 != 0 && effort_level == 6 as ::core::ffi::c_int {
        100 as ::core::ffi::c_int as ::core::ffi::c_float
    } else {
        8.0f32 * effort_level as ::core::ffi::c_float
    };
    '_c2rust_label: {
        if config.quality >= 0 as ::core::ffi::c_int as ::core::ffi::c_float
            && config.quality <= 100.0f32
        {
        } else {
            __assert_fail(
                b"config.quality >= 0 && config.quality <= 100.f\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/alpha_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                84 as ::core::ffi::c_uint,
                b"int EncodeLossless(const uint8_t *const, int, int, int, int, VP8LBitWriter *const, WebPAuxStats *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    ok = VP8LEncodeStream(
        &raw mut config,
        &raw mut picture,
        bw,
        0 as ::core::ffi::c_int,
    );
    WebPPictureFree(&raw mut picture);
    ok = (ok != 0 && (*bw).error_ == 0) as ::core::ffi::c_int;
    if ok == 0 {
        VP8LBitWriterWipeOut(bw);
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn EncodeAlphaInternal(
    data: *const uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut method: ::core::ffi::c_int,
    mut filter: ::core::ffi::c_int,
    mut reduce_levels: ::core::ffi::c_int,
    mut effort_level: ::core::ffi::c_int,
    tmp_alpha: *mut uint8_t,
    mut result: *mut FilterTrial,
) -> ::core::ffi::c_int {
    let mut ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut alpha_src: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut filter_func: WebPFilterFunc = None;
    let mut header: uint8_t = 0;
    let data_size: size_t = (width * height) as size_t;
    let mut output: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut output_size: size_t = 0 as size_t;
    let mut tmp_bw: VP8LBitWriter = VP8LBitWriter {
        bits_: 0,
        used_: 0,
        buf_: ::core::ptr::null_mut::<uint8_t>(),
        cur_: ::core::ptr::null_mut::<uint8_t>(),
        end_: ::core::ptr::null_mut::<uint8_t>(),
        error_: 0,
    };
    '_c2rust_label: {
        if data_size as uint64_t == (width as uint64_t).wrapping_mul(height as uint64_t) {
        } else {
            __assert_fail(
                b"(uint64_t)data_size == (uint64_t)width * height\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/alpha_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                124 as ::core::ffi::c_uint,
                b"int EncodeAlphaInternal(const uint8_t *const, int, int, int, int, int, int, uint8_t *const, FilterTrial *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if filter >= 0 as ::core::ffi::c_int && filter < WEBP_FILTER_LAST as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"filter >= 0 && filter < WEBP_FILTER_LAST\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/alpha_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                125 as ::core::ffi::c_uint,
                b"int EncodeAlphaInternal(const uint8_t *const, int, int, int, int, int, int, uint8_t *const, FilterTrial *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if method >= 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"method >= ALPHA_NO_COMPRESSION\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/alpha_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                126 as ::core::ffi::c_uint,
                b"int EncodeAlphaInternal(const uint8_t *const, int, int, int, int, int, int, uint8_t *const, FilterTrial *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if method <= 1 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"method <= ALPHA_LOSSLESS_COMPRESSION\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/alpha_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                127 as ::core::ffi::c_uint,
                b"int EncodeAlphaInternal(const uint8_t *const, int, int, int, int, int, int, uint8_t *const, FilterTrial *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if ::core::mem::size_of::<uint8_t>() as usize == 1 as usize {
        } else {
            __assert_fail(
                b"sizeof(header) == ALPHA_HEADER_LEN\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/alpha_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                128 as ::core::ffi::c_uint,
                b"int EncodeAlphaInternal(const uint8_t *const, int, int, int, int, int, int, uint8_t *const, FilterTrial *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    filter_func = WebPFilters[filter as usize];
    if filter_func.is_some() {
        filter_func.expect("non-null function pointer")(data, width, height, width, tmp_alpha);
        alpha_src = tmp_alpha;
    } else {
        alpha_src = data;
    }
    if method != ALPHA_NO_COMPRESSION {
        ok = VP8LBitWriterInit(&raw mut tmp_bw, data_size >> 3 as ::core::ffi::c_int);
        ok = (ok != 0
            && EncodeLossless(
                alpha_src,
                width,
                height,
                effort_level,
                (reduce_levels == 0) as ::core::ffi::c_int,
                &raw mut tmp_bw,
                &raw mut (*result).stats,
            ) != 0) as ::core::ffi::c_int;
        if ok != 0 {
            output = VP8LBitWriterFinish(&raw mut tmp_bw);
            if tmp_bw.error_ != 0 {
                VP8LBitWriterWipeOut(&raw mut tmp_bw);
                memset(
                    &raw mut (*result).bw as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<VP8BitWriter>() as size_t,
                );
                return 0 as ::core::ffi::c_int;
            }
            output_size = VP8LBitWriterNumBytes(&raw mut tmp_bw);
            if output_size > data_size {
                method = ALPHA_NO_COMPRESSION;
                VP8LBitWriterWipeOut(&raw mut tmp_bw);
            }
        } else {
            VP8LBitWriterWipeOut(&raw mut tmp_bw);
            memset(
                &raw mut (*result).bw as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<VP8BitWriter>() as size_t,
            );
            return 0 as ::core::ffi::c_int;
        }
    }
    if method == ALPHA_NO_COMPRESSION {
        output = alpha_src;
        output_size = data_size;
        ok = 1 as ::core::ffi::c_int;
    }
    header = (method | filter << 2 as ::core::ffi::c_int) as uint8_t;
    if reduce_levels != 0 {
        header = (header as ::core::ffi::c_int
            | ALPHA_PREPROCESSED_LEVELS << 4 as ::core::ffi::c_int) as uint8_t;
    }
    if VP8BitWriterInit(
        &raw mut (*result).bw,
        (ALPHA_HEADER_LEN as size_t).wrapping_add(output_size),
    ) == 0
    {
        ok = 0 as ::core::ffi::c_int;
    }
    ok = (ok != 0
        && VP8BitWriterAppend(
            &raw mut (*result).bw,
            &raw mut header,
            ALPHA_HEADER_LEN as size_t,
        ) != 0) as ::core::ffi::c_int;
    ok = (ok != 0 && VP8BitWriterAppend(&raw mut (*result).bw, output, output_size) != 0)
        as ::core::ffi::c_int;
    if method != ALPHA_NO_COMPRESSION {
        VP8LBitWriterWipeOut(&raw mut tmp_bw);
    }
    ok = (ok != 0 && (*result).bw.error_ == 0) as ::core::ffi::c_int;
    (*result).score = VP8BitWriterSize(&raw mut (*result).bw);
    return ok;
}
unsafe extern "C" fn GetNumColors(
    mut data: *const uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut colors: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut color: [uint8_t; 256] = [
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
    j = 0 as ::core::ffi::c_int;
    while j < height {
        let mut i: ::core::ffi::c_int = 0;
        let p: *const uint8_t = data.offset((j * stride) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < width {
            color[*p.offset(i as isize) as usize] = 1 as uint8_t;
            i += 1;
        }
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < 256 as ::core::ffi::c_int {
        if color[j as usize] as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            colors += 1;
        }
        j += 1;
    }
    return colors;
}
unsafe extern "C" fn GetFilterMap(
    mut alpha: *const uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut filter: ::core::ffi::c_int,
    mut effort_level: ::core::ffi::c_int,
) -> uint32_t {
    let mut bit_map: uint32_t = 0 as uint32_t;
    if filter == WEBP_FILTER_FAST as ::core::ffi::c_int {
        let mut try_filter_none: ::core::ffi::c_int =
            (effort_level > 3 as ::core::ffi::c_int) as ::core::ffi::c_int;
        let kMinColorsForFilterNone: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
        let kMaxColorsForFilterNone: ::core::ffi::c_int = 192 as ::core::ffi::c_int;
        let num_colors: ::core::ffi::c_int =
            GetNumColors(alpha, width, height, width) as ::core::ffi::c_int;
        filter = (if num_colors <= kMinColorsForFilterNone {
            WEBP_FILTER_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        } else {
            WebPEstimateBestFilter(alpha, width, height, width) as ::core::ffi::c_uint
        }) as ::core::ffi::c_int;
        bit_map |= ((1 as ::core::ffi::c_int) << filter) as uint32_t;
        if try_filter_none != 0 || num_colors > kMaxColorsForFilterNone {
            bit_map |=
                ((1 as ::core::ffi::c_int) << WEBP_FILTER_NONE as ::core::ffi::c_int) as uint32_t;
        }
    } else if filter == WEBP_FILTER_NONE as ::core::ffi::c_int {
        bit_map = ((1 as ::core::ffi::c_int) << WEBP_FILTER_NONE as ::core::ffi::c_int) as uint32_t;
    } else {
        bit_map = (((1 as ::core::ffi::c_int) << WEBP_FILTER_LAST as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as uint32_t;
    }
    return bit_map;
}
unsafe extern "C" fn InitFilterTrial(score: *mut FilterTrial) {
    (*score).score = !(0 as ::core::ffi::c_uint) as size_t;
    VP8BitWriterInit(&raw mut (*score).bw, 0 as size_t);
}
unsafe extern "C" fn ApplyFiltersAndEncode(
    mut alpha: *const uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut data_size: size_t,
    mut method: ::core::ffi::c_int,
    mut filter: ::core::ffi::c_int,
    mut reduce_levels: ::core::ffi::c_int,
    mut effort_level: ::core::ffi::c_int,
    output: *mut *mut uint8_t,
    output_size: *mut size_t,
    stats: *mut WebPAuxStats,
) -> ::core::ffi::c_int {
    let mut ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut best: FilterTrial = FilterTrial {
        score: 0,
        bw: VP8BitWriter {
            range_: 0,
            value_: 0,
            run_: 0,
            nb_bits_: 0,
            buf_: ::core::ptr::null_mut::<uint8_t>(),
            pos_: 0,
            max_pos_: 0,
            error_: 0,
        },
        stats: WebPAuxStats {
            coded_size: 0,
            PSNR: [0.; 5],
            block_count: [0; 3],
            header_bytes: [0; 2],
            residual_bytes: [[0; 4]; 3],
            segment_size: [0; 4],
            segment_quant: [0; 4],
            segment_level: [0; 4],
            alpha_data_size: 0,
            layer_data_size: 0,
            lossless_features: 0,
            histogram_bits: 0,
            transform_bits: 0,
            cache_bits: 0,
            palette_size: 0,
            lossless_size: 0,
            lossless_hdr_size: 0,
            lossless_data_size: 0,
            pad: [0; 2],
        },
    };
    let mut try_map: uint32_t = GetFilterMap(alpha, width, height, filter, effort_level);
    InitFilterTrial(&raw mut best);
    if try_map != ((1 as ::core::ffi::c_int) << WEBP_FILTER_NONE as ::core::ffi::c_int) as uint32_t
    {
        let mut filtered_alpha: *mut uint8_t =
            WebPSafeMalloc(1 as uint64_t, data_size) as *mut uint8_t;
        if filtered_alpha.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        filter = WEBP_FILTER_NONE as ::core::ffi::c_int;
        while ok != 0 && try_map != 0 {
            if try_map & 1 as uint32_t != 0 {
                let mut trial: FilterTrial = FilterTrial {
                    score: 0,
                    bw: VP8BitWriter {
                        range_: 0,
                        value_: 0,
                        run_: 0,
                        nb_bits_: 0,
                        buf_: ::core::ptr::null_mut::<uint8_t>(),
                        pos_: 0,
                        max_pos_: 0,
                        error_: 0,
                    },
                    stats: WebPAuxStats {
                        coded_size: 0,
                        PSNR: [0.; 5],
                        block_count: [0; 3],
                        header_bytes: [0; 2],
                        residual_bytes: [[0; 4]; 3],
                        segment_size: [0; 4],
                        segment_quant: [0; 4],
                        segment_level: [0; 4],
                        alpha_data_size: 0,
                        layer_data_size: 0,
                        lossless_features: 0,
                        histogram_bits: 0,
                        transform_bits: 0,
                        cache_bits: 0,
                        palette_size: 0,
                        lossless_size: 0,
                        lossless_hdr_size: 0,
                        lossless_data_size: 0,
                        pad: [0; 2],
                    },
                };
                ok = EncodeAlphaInternal(
                    alpha,
                    width,
                    height,
                    method,
                    filter,
                    reduce_levels,
                    effort_level,
                    filtered_alpha,
                    &raw mut trial,
                );
                if ok != 0 && trial.score < best.score {
                    VP8BitWriterWipeOut(&raw mut best.bw);
                    best = trial;
                } else {
                    VP8BitWriterWipeOut(&raw mut trial.bw);
                }
            }
            filter += 1;
            try_map >>= 1 as ::core::ffi::c_int;
        }
        WebPSafeFree(filtered_alpha as *mut ::core::ffi::c_void);
    } else {
        ok = EncodeAlphaInternal(
            alpha,
            width,
            height,
            method,
            WEBP_FILTER_NONE as ::core::ffi::c_int,
            reduce_levels,
            effort_level,
            ::core::ptr::null_mut::<uint8_t>(),
            &raw mut best,
        );
    }
    if ok != 0 {
        if !stats.is_null() {
            (*stats).lossless_features = best.stats.lossless_features;
            (*stats).histogram_bits = best.stats.histogram_bits;
            (*stats).transform_bits = best.stats.transform_bits;
            (*stats).cache_bits = best.stats.cache_bits;
            (*stats).palette_size = best.stats.palette_size;
            (*stats).lossless_size = best.stats.lossless_size;
            (*stats).lossless_hdr_size = best.stats.lossless_hdr_size;
            (*stats).lossless_data_size = best.stats.lossless_data_size;
        }
        *output_size = VP8BitWriterSize(&raw mut best.bw);
        *output = VP8BitWriterBuf(&raw mut best.bw);
    } else {
        VP8BitWriterWipeOut(&raw mut best.bw);
    }
    return ok;
}
unsafe extern "C" fn EncodeAlpha(
    enc: *mut VP8Encoder,
    mut quality: ::core::ffi::c_int,
    mut method: ::core::ffi::c_int,
    mut filter: ::core::ffi::c_int,
    mut effort_level: ::core::ffi::c_int,
    output: *mut *mut uint8_t,
    output_size: *mut size_t,
) -> ::core::ffi::c_int {
    let pic: *const WebPPicture = (*enc).pic_;
    let width: ::core::ffi::c_int = (*pic).width;
    let height: ::core::ffi::c_int = (*pic).height;
    let mut quant_alpha: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let data_size: size_t = (width * height) as size_t;
    let mut sse: uint64_t = 0 as uint64_t;
    let mut ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let reduce_levels: ::core::ffi::c_int =
        (quality < 100 as ::core::ffi::c_int) as ::core::ffi::c_int;
    '_c2rust_label: {
        if data_size as uint64_t == (width as uint64_t).wrapping_mul(height as uint64_t) {
        } else {
            __assert_fail(
                b"(uint64_t)data_size == (uint64_t)width * height\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/alpha_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                314 as ::core::ffi::c_uint,
                b"int EncodeAlpha(VP8Encoder *const, int, int, int, int, uint8_t **const, size_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !enc.is_null() && !pic.is_null() && !(*pic).a.is_null() {
        } else {
            __assert_fail(
                b"enc != NULL && pic != NULL && pic->a != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/alpha_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                315 as ::core::ffi::c_uint,
                b"int EncodeAlpha(VP8Encoder *const, int, int, int, int, uint8_t **const, size_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !output.is_null() && !output_size.is_null() {
        } else {
            __assert_fail(
                b"output != NULL && output_size != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/alpha_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                316 as ::core::ffi::c_uint,
                b"int EncodeAlpha(VP8Encoder *const, int, int, int, int, uint8_t **const, size_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if width > 0 as ::core::ffi::c_int && height > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"width > 0 && height > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/alpha_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                317 as ::core::ffi::c_uint,
                b"int EncodeAlpha(VP8Encoder *const, int, int, int, int, uint8_t **const, size_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if (*pic).a_stride >= width {
        } else {
            __assert_fail(
                b"pic->a_stride >= width\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/alpha_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                318 as ::core::ffi::c_uint,
                b"int EncodeAlpha(VP8Encoder *const, int, int, int, int, uint8_t **const, size_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if filter >= WEBP_FILTER_NONE as ::core::ffi::c_int
            && filter <= WEBP_FILTER_FAST as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"filter >= WEBP_FILTER_NONE && filter <= WEBP_FILTER_FAST\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/alpha_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                319 as ::core::ffi::c_uint,
                b"int EncodeAlpha(VP8Encoder *const, int, int, int, int, uint8_t **const, size_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if quality < 0 as ::core::ffi::c_int || quality > 100 as ::core::ffi::c_int {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_INVALID_CONFIGURATION);
    }
    if method < ALPHA_NO_COMPRESSION || method > ALPHA_LOSSLESS_COMPRESSION {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_INVALID_CONFIGURATION);
    }
    if method == ALPHA_NO_COMPRESSION {
        filter = WEBP_FILTER_NONE as ::core::ffi::c_int;
    }
    quant_alpha = WebPSafeMalloc(1 as uint64_t, data_size) as *mut uint8_t;
    if quant_alpha.is_null() {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    WebPCopyPlane((*pic).a, (*pic).a_stride, quant_alpha, width, width, height);
    if reduce_levels != 0 {
        let alpha_levels: ::core::ffi::c_int = if quality <= 70 as ::core::ffi::c_int {
            2 as ::core::ffi::c_int + quality / 5 as ::core::ffi::c_int
        } else {
            16 as ::core::ffi::c_int
                + (quality - 70 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int
        };
        ok = QuantizeLevels(quant_alpha, width, height, alpha_levels, &raw mut sse);
    }
    if ok != 0 {
        VP8FiltersInit();
        ok = ApplyFiltersAndEncode(
            quant_alpha,
            width,
            height,
            data_size,
            method,
            filter,
            reduce_levels,
            effort_level,
            output,
            output_size,
            (*pic).stats,
        );
        if ok == 0 {
            WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
        }
        if !(*pic).stats.is_null() {
            (*(*pic).stats).coded_size += *output_size as ::core::ffi::c_int;
            (*enc).sse_[3 as ::core::ffi::c_int as usize] = sse;
        }
    }
    WebPSafeFree(quant_alpha as *mut ::core::ffi::c_void);
    return ok;
}
unsafe extern "C" fn CompressAlphaJob(
    mut arg1: *mut ::core::ffi::c_void,
    mut unused: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let enc: *mut VP8Encoder = arg1 as *mut VP8Encoder;
    let mut config: *const WebPConfig = (*enc).config_;
    let mut alpha_data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut alpha_size: size_t = 0 as size_t;
    let effort_level: ::core::ffi::c_int = (*config).method;
    let filter: WEBP_FILTER_TYPE = (if (*config).alpha_filtering == 0 as ::core::ffi::c_int {
        WEBP_FILTER_NONE as ::core::ffi::c_int
    } else if (*config).alpha_filtering == 1 as ::core::ffi::c_int {
        WEBP_FILTER_FAST as ::core::ffi::c_int
    } else {
        WEBP_FILTER_BEST as ::core::ffi::c_int
    }) as WEBP_FILTER_TYPE;
    if EncodeAlpha(
        enc,
        (*config).alpha_quality,
        (*config).alpha_compression,
        filter as ::core::ffi::c_int,
        effort_level,
        &raw mut alpha_data,
        &raw mut alpha_size,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if alpha_size != alpha_size as uint32_t as size_t {
        WebPSafeFree(alpha_data as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    (*enc).alpha_data_size_ = alpha_size as uint32_t;
    (*enc).alpha_data_ = alpha_data;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8EncInitAlpha(enc: *mut VP8Encoder) {
    WebPInitAlphaProcessing();
    (*enc).has_alpha_ = WebPPictureHasTransparency((*enc).pic_);
    (*enc).alpha_data_ = ::core::ptr::null_mut::<uint8_t>();
    (*enc).alpha_data_size_ = 0 as uint32_t;
    if (*enc).thread_level_ > 0 as ::core::ffi::c_int {
        let worker: *mut WebPWorker = &raw mut (*enc).alpha_worker_;
        (*WebPGetWorkerInterface())
            .Init
            .expect("non-null function pointer")(worker);
        (*worker).data1 = enc as *mut ::core::ffi::c_void;
        (*worker).data2 = NULL;
        (*worker).hook = Some(
            CompressAlphaJob
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ) as WebPWorkerHook;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8EncStartAlpha(enc: *mut VP8Encoder) -> ::core::ffi::c_int {
    if (*enc).has_alpha_ != 0 {
        if (*enc).thread_level_ > 0 as ::core::ffi::c_int {
            let worker: *mut WebPWorker = &raw mut (*enc).alpha_worker_;
            if (*WebPGetWorkerInterface())
                .Reset
                .expect("non-null function pointer")(worker)
                == 0
            {
                return WebPEncodingSetError((*enc).pic_, VP8_ENC_ERROR_OUT_OF_MEMORY);
            }
            (*WebPGetWorkerInterface())
                .Launch
                .expect("non-null function pointer")(worker);
            return 1 as ::core::ffi::c_int;
        } else {
            return CompressAlphaJob(enc as *mut ::core::ffi::c_void, NULL);
        }
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8EncFinishAlpha(enc: *mut VP8Encoder) -> ::core::ffi::c_int {
    if (*enc).has_alpha_ != 0 {
        if (*enc).thread_level_ > 0 as ::core::ffi::c_int {
            let worker: *mut WebPWorker = &raw mut (*enc).alpha_worker_;
            if (*WebPGetWorkerInterface())
                .Sync_0
                .expect("non-null function pointer")(worker)
                == 0
            {
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    return WebPReportProgress(
        (*enc).pic_,
        (*enc).percent_ + 20 as ::core::ffi::c_int,
        &raw mut (*enc).percent_,
    );
}
#[no_mangle]
pub unsafe extern "C" fn VP8EncDeleteAlpha(enc: *mut VP8Encoder) -> ::core::ffi::c_int {
    let mut ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if (*enc).thread_level_ > 0 as ::core::ffi::c_int {
        let worker: *mut WebPWorker = &raw mut (*enc).alpha_worker_;
        ok = (*WebPGetWorkerInterface())
            .Sync_0
            .expect("non-null function pointer")(worker);
        (*WebPGetWorkerInterface())
            .End
            .expect("non-null function pointer")(worker);
    }
    WebPSafeFree((*enc).alpha_data_ as *mut ::core::ffi::c_void);
    (*enc).alpha_data_ = ::core::ptr::null_mut::<uint8_t>();
    (*enc).alpha_data_size_ = 0 as uint32_t;
    (*enc).has_alpha_ = 0 as ::core::ffi::c_int;
    return ok;
}
