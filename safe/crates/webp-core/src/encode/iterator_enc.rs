use c2rust_bitfields::BitfieldStruct;

#[repr(C)]
pub struct VP8Tokens {
    _unused: [u8; 0],
}

extern "C" {
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
    static VP8Scan: [uint16_t; 16];
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
pub const WEBP_ALIGN_CST: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const YUV_SIZE_ENC: ::core::ffi::c_int = BPS * 16 as ::core::ffi::c_int;
pub const Y_OFF_ENC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const U_OFF_ENC: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const V_OFF_ENC: ::core::ffi::c_int = 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int;
unsafe extern "C" fn InitLeft(it: *mut VP8EncIterator) {
    let ref mut fresh0 = *(*it).v_left_.offset(-(1 as ::core::ffi::c_int) as isize);
    *fresh0 = (if (*it).y_ > 0 as ::core::ffi::c_int {
        129 as ::core::ffi::c_int
    } else {
        127 as ::core::ffi::c_int
    }) as uint8_t;
    let ref mut fresh1 = *(*it).u_left_.offset(-(1 as ::core::ffi::c_int) as isize);
    *fresh1 = *fresh0;
    *(*it).y_left_.offset(-(1 as ::core::ffi::c_int) as isize) = *fresh1;
    memset(
        (*it).y_left_ as *mut ::core::ffi::c_void,
        129 as ::core::ffi::c_int,
        16 as size_t,
    );
    memset(
        (*it).u_left_ as *mut ::core::ffi::c_void,
        129 as ::core::ffi::c_int,
        8 as size_t,
    );
    memset(
        (*it).v_left_ as *mut ::core::ffi::c_void,
        129 as ::core::ffi::c_int,
        8 as size_t,
    );
    (*it).left_nz_[8 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
    if !(*it).top_derr_.is_null() {
        memset(
            &raw mut (*it).left_derr_ as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<DError>() as size_t,
        );
    }
}
unsafe extern "C" fn InitTop(it: *mut VP8EncIterator) {
    let enc: *const VP8Encoder = (*it).enc_;
    let top_size: size_t = ((*enc).mb_w_ * 16 as ::core::ffi::c_int) as size_t;
    memset(
        (*enc).y_top_ as *mut ::core::ffi::c_void,
        127 as ::core::ffi::c_int,
        (2 as size_t).wrapping_mul(top_size),
    );
    memset(
        (*enc).nz_ as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ((*enc).mb_w_ as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
    );
    if !(*enc).top_derr_.is_null() {
        memset(
            (*enc).top_derr_ as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*enc).mb_w_ as size_t).wrapping_mul(::core::mem::size_of::<DError>() as size_t),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorSetRow(it: *mut VP8EncIterator, mut y: ::core::ffi::c_int) {
    let enc: *mut VP8Encoder = (*it).enc_;
    (*it).x_ = 0 as ::core::ffi::c_int;
    (*it).y_ = y;
    (*it).bw_ = (&raw mut (*enc).parts_ as *mut VP8BitWriter)
        .offset((y & (*enc).num_parts_ - 1 as ::core::ffi::c_int) as isize)
        as *mut VP8BitWriter;
    (*it).preds_ = (*enc)
        .preds_
        .offset((y * 4 as ::core::ffi::c_int * (*enc).preds_w_) as isize);
    (*it).nz_ = (*enc).nz_;
    (*it).mb_ = (*enc).mb_info_.offset((y * (*enc).mb_w_) as isize);
    (*it).y_top_ = (*enc).y_top_;
    (*it).uv_top_ = (*enc).uv_top_;
    InitLeft(it);
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorReset(it: *mut VP8EncIterator) {
    let enc: *mut VP8Encoder = (*it).enc_;
    VP8IteratorSetRow(it, 0 as ::core::ffi::c_int);
    VP8IteratorSetCountDown(it, (*enc).mb_w_ * (*enc).mb_h_);
    InitTop(it);
    memset(
        &raw mut (*it).bit_count_ as *mut [uint64_t; 3] as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[[uint64_t; 3]; 4]>() as size_t,
    );
    (*it).do_trellis_ = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorSetCountDown(
    it: *mut VP8EncIterator,
    mut count_down: ::core::ffi::c_int,
) {
    (*it).count_down0_ = count_down;
    (*it).count_down_ = (*it).count_down0_;
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorIsDone(it: *const VP8EncIterator) -> ::core::ffi::c_int {
    return ((*it).count_down_ <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorInit(enc: *mut VP8Encoder, it: *mut VP8EncIterator) {
    (*it).enc_ = enc;
    (*it).yuv_in_ = ((&raw mut (*it).yuv_mem_ as *mut uint8_t as uintptr_t)
        .wrapping_add(WEBP_ALIGN_CST as uintptr_t)
        & !(WEBP_ALIGN_CST as uintptr_t)) as *mut uint8_t;
    (*it).yuv_out_ = (*it).yuv_in_.offset(YUV_SIZE_ENC as isize);
    (*it).yuv_out2_ = (*it).yuv_out_.offset(YUV_SIZE_ENC as isize);
    (*it).yuv_p_ = (*it).yuv_out2_.offset(YUV_SIZE_ENC as isize);
    (*it).lf_stats_ = (*enc).lf_stats_;
    (*it).percent0_ = (*enc).percent_;
    (*it).y_left_ = (((&raw mut (*it).yuv_left_mem_ as *mut uint8_t)
        .offset(1 as ::core::ffi::c_int as isize) as uintptr_t)
        .wrapping_add(WEBP_ALIGN_CST as uintptr_t)
        & !(WEBP_ALIGN_CST as uintptr_t)) as *mut uint8_t;
    (*it).u_left_ = (*it)
        .y_left_
        .offset(16 as ::core::ffi::c_int as isize)
        .offset(16 as ::core::ffi::c_int as isize);
    (*it).v_left_ = (*it).u_left_.offset(16 as ::core::ffi::c_int as isize);
    (*it).top_derr_ = (*enc).top_derr_;
    VP8IteratorReset(it);
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorProgress(
    it: *const VP8EncIterator,
    mut delta: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let enc: *mut VP8Encoder = (*it).enc_;
    if delta != 0 && (*(*enc).pic_).progress_hook.is_some() {
        let done: ::core::ffi::c_int = (*it).count_down0_ - (*it).count_down_;
        let percent: ::core::ffi::c_int = if (*it).count_down0_ <= 0 as ::core::ffi::c_int {
            (*it).percent0_
        } else {
            (*it).percent0_ + delta * done / (*it).count_down0_
        };
        return WebPReportProgress((*enc).pic_, percent, &raw mut (*enc).percent_);
    }
    return 1 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn MinSize(
    mut a: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if a < b { a } else { b };
}
unsafe extern "C" fn ImportBlock(
    mut src: *const uint8_t,
    mut src_stride: ::core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < h {
        memcpy(
            dst as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            w as size_t,
        );
        if w < size {
            memset(
                dst.offset(w as isize) as *mut ::core::ffi::c_void,
                *dst.offset((w - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int,
                (size - w) as size_t,
            );
        }
        dst = dst.offset(BPS as isize);
        src = src.offset(src_stride as isize);
        i += 1;
    }
    i = h;
    while i < size {
        memcpy(
            dst as *mut ::core::ffi::c_void,
            dst.offset(-(BPS as isize)) as *const ::core::ffi::c_void,
            size as size_t,
        );
        dst = dst.offset(BPS as isize);
        i += 1;
    }
}
unsafe extern "C" fn ImportLine(
    mut src: *const uint8_t,
    mut src_stride: ::core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut len: ::core::ffi::c_int,
    mut total_len: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < len {
        *dst.offset(i as isize) = *src;
        i += 1;
        src = src.offset(src_stride as isize);
    }
    while i < total_len {
        *dst.offset(i as isize) = *dst.offset((len - 1 as ::core::ffi::c_int) as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorImport(it: *mut VP8EncIterator, tmp_32: *mut uint8_t) {
    let enc: *const VP8Encoder = (*it).enc_;
    let x: ::core::ffi::c_int = (*it).x_;
    let y: ::core::ffi::c_int = (*it).y_;
    let pic: *const WebPPicture = (*enc).pic_;
    let ysrc: *const uint8_t = (*pic)
        .y
        .offset(((y * (*pic).y_stride + x) * 16 as ::core::ffi::c_int) as isize);
    let usrc: *const uint8_t = (*pic)
        .u
        .offset(((y * (*pic).uv_stride + x) * 8 as ::core::ffi::c_int) as isize);
    let vsrc: *const uint8_t = (*pic)
        .v
        .offset(((y * (*pic).uv_stride + x) * 8 as ::core::ffi::c_int) as isize);
    let w: ::core::ffi::c_int = MinSize(
        (*pic).width - x * 16 as ::core::ffi::c_int,
        16 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    let h: ::core::ffi::c_int = MinSize(
        (*pic).height - y * 16 as ::core::ffi::c_int,
        16 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    let uv_w: ::core::ffi::c_int = w + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
    let uv_h: ::core::ffi::c_int = h + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
    ImportBlock(
        ysrc,
        (*pic).y_stride,
        (*it).yuv_in_.offset(Y_OFF_ENC as isize),
        w,
        h,
        16 as ::core::ffi::c_int,
    );
    ImportBlock(
        usrc,
        (*pic).uv_stride,
        (*it).yuv_in_.offset(U_OFF_ENC as isize),
        uv_w,
        uv_h,
        8 as ::core::ffi::c_int,
    );
    ImportBlock(
        vsrc,
        (*pic).uv_stride,
        (*it).yuv_in_.offset(V_OFF_ENC as isize),
        uv_w,
        uv_h,
        8 as ::core::ffi::c_int,
    );
    if tmp_32.is_null() {
        return;
    }
    if x == 0 as ::core::ffi::c_int {
        InitLeft(it);
    } else {
        if y == 0 as ::core::ffi::c_int {
            let ref mut fresh2 = *(*it).v_left_.offset(-(1 as ::core::ffi::c_int) as isize);
            *fresh2 = 127 as uint8_t;
            let ref mut fresh3 = *(*it).u_left_.offset(-(1 as ::core::ffi::c_int) as isize);
            *fresh3 = *fresh2;
            *(*it).y_left_.offset(-(1 as ::core::ffi::c_int) as isize) = *fresh3;
        } else {
            *(*it).y_left_.offset(-(1 as ::core::ffi::c_int) as isize) =
                *ysrc.offset((-(1 as ::core::ffi::c_int) - (*pic).y_stride) as isize);
            *(*it).u_left_.offset(-(1 as ::core::ffi::c_int) as isize) =
                *usrc.offset((-(1 as ::core::ffi::c_int) - (*pic).uv_stride) as isize);
            *(*it).v_left_.offset(-(1 as ::core::ffi::c_int) as isize) =
                *vsrc.offset((-(1 as ::core::ffi::c_int) - (*pic).uv_stride) as isize);
        }
        ImportLine(
            ysrc.offset(-(1 as ::core::ffi::c_int as isize)),
            (*pic).y_stride,
            (*it).y_left_,
            h,
            16 as ::core::ffi::c_int,
        );
        ImportLine(
            usrc.offset(-(1 as ::core::ffi::c_int as isize)),
            (*pic).uv_stride,
            (*it).u_left_,
            uv_h,
            8 as ::core::ffi::c_int,
        );
        ImportLine(
            vsrc.offset(-(1 as ::core::ffi::c_int as isize)),
            (*pic).uv_stride,
            (*it).v_left_,
            uv_h,
            8 as ::core::ffi::c_int,
        );
    }
    (*it).y_top_ = tmp_32.offset(0 as ::core::ffi::c_int as isize);
    (*it).uv_top_ = tmp_32.offset(16 as ::core::ffi::c_int as isize);
    if y == 0 as ::core::ffi::c_int {
        memset(
            tmp_32 as *mut ::core::ffi::c_void,
            127 as ::core::ffi::c_int,
            (32 as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        );
    } else {
        ImportLine(
            ysrc.offset(-((*pic).y_stride as isize)),
            1 as ::core::ffi::c_int,
            tmp_32,
            w,
            16 as ::core::ffi::c_int,
        );
        ImportLine(
            usrc.offset(-((*pic).uv_stride as isize)),
            1 as ::core::ffi::c_int,
            tmp_32.offset(16 as ::core::ffi::c_int as isize),
            uv_w,
            8 as ::core::ffi::c_int,
        );
        ImportLine(
            vsrc.offset(-((*pic).uv_stride as isize)),
            1 as ::core::ffi::c_int,
            tmp_32
                .offset(16 as ::core::ffi::c_int as isize)
                .offset(8 as ::core::ffi::c_int as isize),
            uv_w,
            8 as ::core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn ExportBlock(
    mut src: *const uint8_t,
    mut dst: *mut uint8_t,
    mut dst_stride: ::core::ffi::c_int,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    loop {
        let fresh4 = h;
        h = h - 1;
        if !(fresh4 > 0 as ::core::ffi::c_int) {
            break;
        }
        memcpy(
            dst as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            w as size_t,
        );
        dst = dst.offset(dst_stride as isize);
        src = src.offset(BPS as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorExport(it: *const VP8EncIterator) {
    let enc: *const VP8Encoder = (*it).enc_;
    if (*(*enc).config_).show_compressed != 0 {
        let x: ::core::ffi::c_int = (*it).x_;
        let y: ::core::ffi::c_int = (*it).y_;
        let ysrc: *const uint8_t = (*it).yuv_out_.offset(Y_OFF_ENC as isize);
        let usrc: *const uint8_t = (*it).yuv_out_.offset(U_OFF_ENC as isize);
        let vsrc: *const uint8_t = (*it).yuv_out_.offset(V_OFF_ENC as isize);
        let pic: *const WebPPicture = (*enc).pic_;
        let ydst: *mut uint8_t = (*pic)
            .y
            .offset(((y * (*pic).y_stride + x) * 16 as ::core::ffi::c_int) as isize);
        let udst: *mut uint8_t = (*pic)
            .u
            .offset(((y * (*pic).uv_stride + x) * 8 as ::core::ffi::c_int) as isize);
        let vdst: *mut uint8_t = (*pic)
            .v
            .offset(((y * (*pic).uv_stride + x) * 8 as ::core::ffi::c_int) as isize);
        let mut w: ::core::ffi::c_int = (*pic).width - x * 16 as ::core::ffi::c_int;
        let mut h: ::core::ffi::c_int = (*pic).height - y * 16 as ::core::ffi::c_int;
        if w > 16 as ::core::ffi::c_int {
            w = 16 as ::core::ffi::c_int;
        }
        if h > 16 as ::core::ffi::c_int {
            h = 16 as ::core::ffi::c_int;
        }
        ExportBlock(ysrc, ydst, (*pic).y_stride, w, h);
        let uv_w: ::core::ffi::c_int = w + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
        let uv_h: ::core::ffi::c_int = h + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
        ExportBlock(usrc, udst, (*pic).uv_stride, uv_w, uv_h);
        ExportBlock(vsrc, vdst, (*pic).uv_stride, uv_w, uv_h);
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorNzToBytes(it: *mut VP8EncIterator) {
    let tnz: ::core::ffi::c_int =
        *(*it).nz_.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    let lnz: ::core::ffi::c_int =
        *(*it).nz_.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int;
    let top_nz: *mut ::core::ffi::c_int = &raw mut (*it).top_nz_ as *mut ::core::ffi::c_int;
    let left_nz: *mut ::core::ffi::c_int = &raw mut (*it).left_nz_ as *mut ::core::ffi::c_int;
    *top_nz.offset(0 as ::core::ffi::c_int as isize) =
        (tnz & (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *top_nz.offset(1 as ::core::ffi::c_int as isize) =
        (tnz & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *top_nz.offset(2 as ::core::ffi::c_int as isize) =
        (tnz & (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *top_nz.offset(3 as ::core::ffi::c_int as isize) =
        (tnz & (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *top_nz.offset(4 as ::core::ffi::c_int as isize) =
        (tnz & (1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *top_nz.offset(5 as ::core::ffi::c_int as isize) =
        (tnz & (1 as ::core::ffi::c_int) << 19 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *top_nz.offset(6 as ::core::ffi::c_int as isize) =
        (tnz & (1 as ::core::ffi::c_int) << 22 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *top_nz.offset(7 as ::core::ffi::c_int as isize) =
        (tnz & (1 as ::core::ffi::c_int) << 23 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *top_nz.offset(8 as ::core::ffi::c_int as isize) =
        (tnz & (1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *left_nz.offset(0 as ::core::ffi::c_int as isize) =
        (lnz & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *left_nz.offset(1 as ::core::ffi::c_int as isize) =
        (lnz & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *left_nz.offset(2 as ::core::ffi::c_int as isize) =
        (lnz & (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *left_nz.offset(3 as ::core::ffi::c_int as isize) =
        (lnz & (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *left_nz.offset(4 as ::core::ffi::c_int as isize) =
        (lnz & (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *left_nz.offset(5 as ::core::ffi::c_int as isize) =
        (lnz & (1 as ::core::ffi::c_int) << 19 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *left_nz.offset(6 as ::core::ffi::c_int as isize) =
        (lnz & (1 as ::core::ffi::c_int) << 21 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    *left_nz.offset(7 as ::core::ffi::c_int as isize) =
        (lnz & (1 as ::core::ffi::c_int) << 23 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorBytesToNz(it: *mut VP8EncIterator) {
    let mut nz: uint32_t = 0 as uint32_t;
    let top_nz: *const ::core::ffi::c_int = &raw mut (*it).top_nz_ as *mut ::core::ffi::c_int;
    let left_nz: *const ::core::ffi::c_int = &raw mut (*it).left_nz_ as *mut ::core::ffi::c_int;
    nz |= (*top_nz.offset(0 as ::core::ffi::c_int as isize) << 12 as ::core::ffi::c_int
        | *top_nz.offset(1 as ::core::ffi::c_int as isize) << 13 as ::core::ffi::c_int)
        as uint32_t;
    nz |= (*top_nz.offset(2 as ::core::ffi::c_int as isize) << 14 as ::core::ffi::c_int
        | *top_nz.offset(3 as ::core::ffi::c_int as isize) << 15 as ::core::ffi::c_int)
        as uint32_t;
    nz |= (*top_nz.offset(4 as ::core::ffi::c_int as isize) << 18 as ::core::ffi::c_int
        | *top_nz.offset(5 as ::core::ffi::c_int as isize) << 19 as ::core::ffi::c_int)
        as uint32_t;
    nz |= (*top_nz.offset(6 as ::core::ffi::c_int as isize) << 22 as ::core::ffi::c_int
        | *top_nz.offset(7 as ::core::ffi::c_int as isize) << 23 as ::core::ffi::c_int)
        as uint32_t;
    nz |=
        (*top_nz.offset(8 as ::core::ffi::c_int as isize) << 24 as ::core::ffi::c_int) as uint32_t;
    nz |= (*left_nz.offset(0 as ::core::ffi::c_int as isize) << 3 as ::core::ffi::c_int
        | *left_nz.offset(1 as ::core::ffi::c_int as isize) << 7 as ::core::ffi::c_int)
        as uint32_t;
    nz |=
        (*left_nz.offset(2 as ::core::ffi::c_int as isize) << 11 as ::core::ffi::c_int) as uint32_t;
    nz |= (*left_nz.offset(4 as ::core::ffi::c_int as isize) << 17 as ::core::ffi::c_int
        | *left_nz.offset(6 as ::core::ffi::c_int as isize) << 21 as ::core::ffi::c_int)
        as uint32_t;
    *(*it).nz_ = nz;
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorSaveBoundary(it: *mut VP8EncIterator) {
    let enc: *mut VP8Encoder = (*it).enc_;
    let x: ::core::ffi::c_int = (*it).x_;
    let y: ::core::ffi::c_int = (*it).y_;
    let ysrc: *const uint8_t = (*it).yuv_out_.offset(Y_OFF_ENC as isize);
    let uvsrc: *const uint8_t = (*it).yuv_out_.offset(U_OFF_ENC as isize);
    if x < (*enc).mb_w_ - 1 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            *(*it).y_left_.offset(i as isize) =
                *ysrc.offset((15 as ::core::ffi::c_int + i * BPS) as isize);
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            *(*it).u_left_.offset(i as isize) =
                *uvsrc.offset((7 as ::core::ffi::c_int + i * BPS) as isize);
            *(*it).v_left_.offset(i as isize) =
                *uvsrc.offset((15 as ::core::ffi::c_int + i * BPS) as isize);
            i += 1;
        }
        *(*it).y_left_.offset(-(1 as ::core::ffi::c_int) as isize) =
            *(*it).y_top_.offset(15 as ::core::ffi::c_int as isize);
        *(*it).u_left_.offset(-(1 as ::core::ffi::c_int) as isize) = *(*it)
            .uv_top_
            .offset((0 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize);
        *(*it).v_left_.offset(-(1 as ::core::ffi::c_int) as isize) = *(*it)
            .uv_top_
            .offset((8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize);
    }
    if y < (*enc).mb_h_ - 1 as ::core::ffi::c_int {
        memcpy(
            (*it).y_top_ as *mut ::core::ffi::c_void,
            ysrc.offset((15 as ::core::ffi::c_int * BPS) as isize) as *const ::core::ffi::c_void,
            16 as size_t,
        );
        memcpy(
            (*it).uv_top_ as *mut ::core::ffi::c_void,
            uvsrc.offset((7 as ::core::ffi::c_int * BPS) as isize) as *const ::core::ffi::c_void,
            (8 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorNext(it: *mut VP8EncIterator) -> ::core::ffi::c_int {
    (*it).x_ += 1;
    if (*it).x_ == (*(*it).enc_).mb_w_ {
        (*it).y_ += 1;
        VP8IteratorSetRow(it, (*it).y_);
    } else {
        (*it).preds_ = (*it).preds_.offset(4 as ::core::ffi::c_int as isize);
        (*it).mb_ = (*it).mb_.offset(1 as ::core::ffi::c_int as isize);
        (*it).nz_ = (*it).nz_.offset(1 as ::core::ffi::c_int as isize);
        (*it).y_top_ = (*it).y_top_.offset(16 as ::core::ffi::c_int as isize);
        (*it).uv_top_ = (*it).uv_top_.offset(16 as ::core::ffi::c_int as isize);
    }
    (*it).count_down_ -= 1;
    return ((0 as ::core::ffi::c_int) < (*it).count_down_) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8SetIntra16Mode(
    it: *const VP8EncIterator,
    mut mode: ::core::ffi::c_int,
) {
    let mut preds: *mut uint8_t = (*it).preds_;
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < 4 as ::core::ffi::c_int {
        memset(preds as *mut ::core::ffi::c_void, mode, 4 as size_t);
        preds = preds.offset((*(*it).enc_).preds_w_ as isize);
        y += 1;
    }
    (*(*it).mb_).set_type_(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn VP8SetIntra4Mode(it: *const VP8EncIterator, mut modes: *const uint8_t) {
    let mut preds: *mut uint8_t = (*it).preds_;
    let mut y: ::core::ffi::c_int = 0;
    y = 4 as ::core::ffi::c_int;
    while y > 0 as ::core::ffi::c_int {
        memcpy(
            preds as *mut ::core::ffi::c_void,
            modes as *const ::core::ffi::c_void,
            (4 as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        );
        preds = preds.offset((*(*it).enc_).preds_w_ as isize);
        modes = modes.offset(4 as ::core::ffi::c_int as isize);
        y -= 1;
    }
    (*(*it).mb_).set_type_(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn VP8SetIntraUVMode(
    it: *const VP8EncIterator,
    mut mode: ::core::ffi::c_int,
) {
    (*(*it).mb_).set_uv_mode_(mode as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn VP8SetSkip(it: *const VP8EncIterator, mut skip: ::core::ffi::c_int) {
    (*(*it).mb_).set_skip_(skip as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn VP8SetSegment(it: *const VP8EncIterator, mut segment: ::core::ffi::c_int) {
    (*(*it).mb_).set_segment_(segment as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
static mut VP8TopLeftI4: [uint8_t; 16] = [
    17 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    29 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    17 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    17 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    17 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorStartI4(it: *mut VP8EncIterator) {
    let enc: *const VP8Encoder = (*it).enc_;
    let mut i: ::core::ffi::c_int = 0;
    (*it).i4_ = 0 as ::core::ffi::c_int;
    (*it).i4_top_ = (&raw mut (*it).i4_boundary_ as *mut uint8_t)
        .offset(VP8TopLeftI4[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int as isize);
    i = 0 as ::core::ffi::c_int;
    while i < 17 as ::core::ffi::c_int {
        (*it).i4_boundary_[i as usize] = *(*it)
            .y_left_
            .offset((15 as ::core::ffi::c_int - i) as isize);
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*it).i4_boundary_[(17 as ::core::ffi::c_int + i) as usize] =
            *(*it).y_top_.offset(i as isize);
        i += 1;
    }
    if (*it).x_ < (*enc).mb_w_ - 1 as ::core::ffi::c_int {
        i = 16 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int {
            (*it).i4_boundary_[(17 as ::core::ffi::c_int + i) as usize] =
                *(*it).y_top_.offset(i as isize);
            i += 1;
        }
    } else {
        i = 16 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int {
            (*it).i4_boundary_[(17 as ::core::ffi::c_int + i) as usize] =
                (*it).i4_boundary_[(17 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as usize];
            i += 1;
        }
    }
    VP8IteratorNzToBytes(it);
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorRotateI4(
    it: *mut VP8EncIterator,
    yuv_out: *const uint8_t,
) -> ::core::ffi::c_int {
    let blk: *const uint8_t =
        yuv_out.offset(VP8Scan[(*it).i4_ as usize] as ::core::ffi::c_int as isize);
    let top: *mut uint8_t = (*it).i4_top_;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i <= 3 as ::core::ffi::c_int {
        *top.offset((-(4 as ::core::ffi::c_int) + i) as isize) =
            *blk.offset((i + 3 as ::core::ffi::c_int * BPS) as isize);
        i += 1;
    }
    if (*it).i4_ & 3 as ::core::ffi::c_int != 3 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i <= 2 as ::core::ffi::c_int {
            *top.offset(i as isize) = *blk
                .offset((3 as ::core::ffi::c_int + (2 as ::core::ffi::c_int - i) * BPS) as isize);
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        while i <= 3 as ::core::ffi::c_int {
            *top.offset(i as isize) = *top.offset((i + 4 as ::core::ffi::c_int) as isize);
            i += 1;
        }
    }
    (*it).i4_ += 1;
    if (*it).i4_ == 16 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    (*it).i4_top_ = (&raw mut (*it).i4_boundary_ as *mut uint8_t)
        .offset(VP8TopLeftI4[(*it).i4_ as usize] as ::core::ffi::c_int as isize);
    return 1 as ::core::ffi::c_int;
}
