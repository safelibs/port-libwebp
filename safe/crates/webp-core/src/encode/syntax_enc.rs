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
    fn VP8BitWriterInit(bw: *mut VP8BitWriter, expected_size: size_t) -> ::core::ffi::c_int;
    fn VP8BitWriterFinish(bw: *mut VP8BitWriter) -> *mut uint8_t;
    fn VP8BitWriterWipeOut(bw: *mut VP8BitWriter);
    fn VP8PutBitUniform(bw: *mut VP8BitWriter, bit: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn VP8PutBits(bw: *mut VP8BitWriter, value: uint32_t, nb_bits: ::core::ffi::c_int);
    fn VP8PutSignedBits(
        bw: *mut VP8BitWriter,
        value: ::core::ffi::c_int,
        nb_bits: ::core::ffi::c_int,
    );
    fn VP8WriteProbas(bw: *mut VP8BitWriter, probas: *const VP8EncProba);
    fn VP8CodeIntraModes(enc: *mut VP8Encoder);
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
pub type WebPFeatureFlags = ::core::ffi::c_uint;
pub const ALL_VALID_FLAGS: WebPFeatureFlags = 62;
pub const ICCP_FLAG: WebPFeatureFlags = 32;
pub const ALPHA_FLAG: WebPFeatureFlags = 16;
pub const EXIF_FLAG: WebPFeatureFlags = 8;
pub const XMP_FLAG: WebPFeatureFlags = 4;
pub const ANIMATION_FLAG: WebPFeatureFlags = 2;
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
#[inline]
unsafe extern "C" fn PutLE16(data: *mut uint8_t, mut val: ::core::ffi::c_int) {
    '_c2rust_label: {
        if val < (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"val < (1 << 16)\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/utils/utils.h\0" as *const u8 as *const ::core::ffi::c_char,
                108 as ::core::ffi::c_uint,
                b"void PutLE16(uint8_t *const, int)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    *data.offset(0 as ::core::ffi::c_int as isize) =
        (val >> 0 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
    *data.offset(1 as ::core::ffi::c_int as isize) =
        (val >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn PutLE24(data: *mut uint8_t, mut val: ::core::ffi::c_int) {
    '_c2rust_label: {
        if val < (1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"val < (1 << 24)\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/utils/utils.h\0" as *const u8 as *const ::core::ffi::c_char,
                114 as ::core::ffi::c_uint,
                b"void PutLE24(uint8_t *const, int)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    PutLE16(data, val & 0xffff as ::core::ffi::c_int);
    *data.offset(2 as ::core::ffi::c_int as isize) =
        (val >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn PutLE32(data: *mut uint8_t, mut val: uint32_t) {
    PutLE16(data, (val & 0xffff as uint32_t) as ::core::ffi::c_int);
    PutLE16(
        data.offset(2 as ::core::ffi::c_int as isize),
        (val >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
}
pub const VP8_SIGNATURE: ::core::ffi::c_int = 0x9d012a as ::core::ffi::c_int;
pub const VP8_MAX_PARTITION0_SIZE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 19 as ::core::ffi::c_int;
pub const VP8_MAX_PARTITION_SIZE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int;
pub const VP8_FRAME_HEADER_SIZE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const TAG_SIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CHUNK_HEADER_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const VP8X_CHUNK_SIZE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn VP8BitWriterPos(bw: *const VP8BitWriter) -> uint64_t {
    let nb_bits: uint64_t = (8 as ::core::ffi::c_int + (*bw).nb_bits_) as uint64_t;
    return ((*bw).pos_ as uint64_t)
        .wrapping_add((*bw).run_ as uint64_t)
        .wrapping_mul(8 as uint64_t)
        .wrapping_add(nb_bits);
}
#[inline]
unsafe extern "C" fn VP8BitWriterBuf(bw: *const VP8BitWriter) -> *mut uint8_t {
    return (*bw).buf_;
}
#[inline]
unsafe extern "C" fn VP8BitWriterSize(bw: *const VP8BitWriter) -> size_t {
    return (*bw).pos_;
}
unsafe extern "C" fn IsVP8XNeeded(enc: *const VP8Encoder) -> ::core::ffi::c_int {
    return ((*enc).has_alpha_ != 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn PutPaddingByte(pic: *const WebPPicture) -> ::core::ffi::c_int {
    let pad_byte: [uint8_t; 1] = [0 as ::core::ffi::c_int as uint8_t];
    return ((*pic).writer.expect("non-null function pointer")(
        &raw const pad_byte as *const uint8_t,
        1 as size_t,
        pic,
    ) != 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn PutRIFFHeader(
    enc: *const VP8Encoder,
    mut riff_size: size_t,
) -> WebPEncodingError {
    let pic: *const WebPPicture = (*enc).pic_;
    let mut riff: [uint8_t; 12] = [
        'R' as i32 as uint8_t,
        'I' as i32 as uint8_t,
        'F' as i32 as uint8_t,
        'F' as i32 as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        'W' as i32 as uint8_t,
        'E' as i32 as uint8_t,
        'B' as i32 as uint8_t,
        'P' as i32 as uint8_t,
    ];
    '_c2rust_label: {
        if riff_size == riff_size as uint32_t as size_t {
        } else {
            __assert_fail(
                b"riff_size == (uint32_t)riff_size\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/syntax_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                43 as ::core::ffi::c_uint,
                b"WebPEncodingError PutRIFFHeader(const VP8Encoder *const, size_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    PutLE32(
        (&raw mut riff as *mut uint8_t).offset(TAG_SIZE as isize),
        riff_size as uint32_t,
    );
    if (*pic).writer.expect("non-null function pointer")(
        &raw mut riff as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 12]>() as size_t,
        pic,
    ) == 0
    {
        return VP8_ENC_ERROR_BAD_WRITE;
    }
    return VP8_ENC_OK;
}
unsafe extern "C" fn PutVP8XHeader(enc: *const VP8Encoder) -> WebPEncodingError {
    let pic: *const WebPPicture = (*enc).pic_;
    let mut vp8x: [uint8_t; 18] = [
        'V' as i32 as uint8_t,
        'P' as i32 as uint8_t,
        '8' as i32 as uint8_t,
        'X' as i32 as uint8_t,
        0,
        0,
        0,
        0,
        0,
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
    let mut flags: uint32_t = 0 as uint32_t;
    '_c2rust_label: {
        if IsVP8XNeeded(enc) != 0 {
        } else {
            __assert_fail(
                b"IsVP8XNeeded(enc)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/syntax_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                58 as ::core::ffi::c_uint,
                b"WebPEncodingError PutVP8XHeader(const VP8Encoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*pic).width >= 1 as ::core::ffi::c_int && (*pic).height >= 1 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"pic->width >= 1 && pic->height >= 1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/syntax_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                59 as ::core::ffi::c_uint,
                b"WebPEncodingError PutVP8XHeader(const VP8Encoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*pic).width <= (1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
            && (*pic).height <= (1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"pic->width <= MAX_CANVAS_SIZE && pic->height <= MAX_CANVAS_SIZE\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/syntax_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                60 as ::core::ffi::c_uint,
                b"WebPEncodingError PutVP8XHeader(const VP8Encoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*enc).has_alpha_ != 0 {
        flags |= ALPHA_FLAG as ::core::ffi::c_int as uint32_t;
    }
    PutLE32(
        (&raw mut vp8x as *mut uint8_t).offset(TAG_SIZE as isize),
        VP8X_CHUNK_SIZE as uint32_t,
    );
    PutLE32(
        (&raw mut vp8x as *mut uint8_t).offset(CHUNK_HEADER_SIZE as isize),
        flags,
    );
    PutLE24(
        (&raw mut vp8x as *mut uint8_t)
            .offset(CHUNK_HEADER_SIZE as isize)
            .offset(4 as ::core::ffi::c_int as isize),
        (*pic).width - 1 as ::core::ffi::c_int,
    );
    PutLE24(
        (&raw mut vp8x as *mut uint8_t)
            .offset(CHUNK_HEADER_SIZE as isize)
            .offset(7 as ::core::ffi::c_int as isize),
        (*pic).height - 1 as ::core::ffi::c_int,
    );
    if (*pic).writer.expect("non-null function pointer")(
        &raw mut vp8x as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 18]>() as size_t,
        pic,
    ) == 0
    {
        return VP8_ENC_ERROR_BAD_WRITE;
    }
    return VP8_ENC_OK;
}
unsafe extern "C" fn PutAlphaChunk(enc: *const VP8Encoder) -> WebPEncodingError {
    let pic: *const WebPPicture = (*enc).pic_;
    let mut alpha_chunk_hdr: [uint8_t; 8] = [
        'A' as i32 as uint8_t,
        'L' as i32 as uint8_t,
        'P' as i32 as uint8_t,
        'H' as i32 as uint8_t,
        0,
        0,
        0,
        0,
    ];
    '_c2rust_label: {
        if (*enc).has_alpha_ != 0 {
        } else {
            __assert_fail(
                b"enc->has_alpha_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/syntax_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                82 as ::core::ffi::c_uint,
                b"WebPEncodingError PutAlphaChunk(const VP8Encoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    PutLE32(
        (&raw mut alpha_chunk_hdr as *mut uint8_t).offset(TAG_SIZE as isize),
        (*enc).alpha_data_size_,
    );
    if (*pic).writer.expect("non-null function pointer")(
        &raw mut alpha_chunk_hdr as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 8]>() as size_t,
        pic,
    ) == 0
    {
        return VP8_ENC_ERROR_BAD_WRITE;
    }
    if (*pic).writer.expect("non-null function pointer")(
        (*enc).alpha_data_,
        (*enc).alpha_data_size_ as size_t,
        pic,
    ) == 0
    {
        return VP8_ENC_ERROR_BAD_WRITE;
    }
    if (*enc).alpha_data_size_ & 1 as uint32_t != 0 && PutPaddingByte(pic) == 0 {
        return VP8_ENC_ERROR_BAD_WRITE;
    }
    return VP8_ENC_OK;
}
unsafe extern "C" fn PutVP8Header(
    pic: *const WebPPicture,
    mut vp8_size: size_t,
) -> WebPEncodingError {
    let mut vp8_chunk_hdr: [uint8_t; 8] = [
        'V' as i32 as uint8_t,
        'P' as i32 as uint8_t,
        '8' as i32 as uint8_t,
        ' ' as i32 as uint8_t,
        0,
        0,
        0,
        0,
    ];
    '_c2rust_label: {
        if vp8_size == vp8_size as uint32_t as size_t {
        } else {
            __assert_fail(
                b"vp8_size == (uint32_t)vp8_size\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/syntax_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                107 as ::core::ffi::c_uint,
                b"WebPEncodingError PutVP8Header(const WebPPicture *const, size_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    PutLE32(
        (&raw mut vp8_chunk_hdr as *mut uint8_t).offset(TAG_SIZE as isize),
        vp8_size as uint32_t,
    );
    if (*pic).writer.expect("non-null function pointer")(
        &raw mut vp8_chunk_hdr as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 8]>() as size_t,
        pic,
    ) == 0
    {
        return VP8_ENC_ERROR_BAD_WRITE;
    }
    return VP8_ENC_OK;
}
unsafe extern "C" fn PutVP8FrameHeader(
    pic: *const WebPPicture,
    mut profile: ::core::ffi::c_int,
    mut size0: size_t,
) -> WebPEncodingError {
    let mut vp8_frm_hdr: [uint8_t; 10] = [0; 10];
    let mut bits: uint32_t = 0;
    if size0 >= VP8_MAX_PARTITION0_SIZE as size_t {
        return VP8_ENC_ERROR_PARTITION0_OVERFLOW;
    }
    bits = (0 as ::core::ffi::c_int
        | profile << 1 as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint32_t
        | (size0 as uint32_t) << 5 as ::core::ffi::c_int;
    vp8_frm_hdr[0 as ::core::ffi::c_int as usize] =
        (bits >> 0 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    vp8_frm_hdr[1 as ::core::ffi::c_int as usize] =
        (bits >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    vp8_frm_hdr[2 as ::core::ffi::c_int as usize] =
        (bits >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    vp8_frm_hdr[3 as ::core::ffi::c_int as usize] =
        (VP8_SIGNATURE >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
    vp8_frm_hdr[4 as ::core::ffi::c_int as usize] =
        (VP8_SIGNATURE >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
    vp8_frm_hdr[5 as ::core::ffi::c_int as usize] =
        (VP8_SIGNATURE >> 0 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
    vp8_frm_hdr[6 as ::core::ffi::c_int as usize] =
        ((*pic).width & 0xff as ::core::ffi::c_int) as uint8_t;
    vp8_frm_hdr[7 as ::core::ffi::c_int as usize] =
        ((*pic).width >> 8 as ::core::ffi::c_int) as uint8_t;
    vp8_frm_hdr[8 as ::core::ffi::c_int as usize] =
        ((*pic).height & 0xff as ::core::ffi::c_int) as uint8_t;
    vp8_frm_hdr[9 as ::core::ffi::c_int as usize] =
        ((*pic).height >> 8 as ::core::ffi::c_int) as uint8_t;
    if (*pic).writer.expect("non-null function pointer")(
        &raw mut vp8_frm_hdr as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 10]>() as size_t,
        pic,
    ) == 0
    {
        return VP8_ENC_ERROR_BAD_WRITE;
    }
    return VP8_ENC_OK;
}
unsafe extern "C" fn PutWebPHeaders(
    enc: *const VP8Encoder,
    mut size0: size_t,
    mut vp8_size: size_t,
    mut riff_size: size_t,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let pic: *mut WebPPicture = (*enc).pic_;
    let mut err: WebPEncodingError = VP8_ENC_OK;
    err = PutRIFFHeader(enc, riff_size);
    if !(err as ::core::ffi::c_uint != VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint) {
        if IsVP8XNeeded(enc) != 0 {
            err = PutVP8XHeader(enc);
            if err as ::core::ffi::c_uint != VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                current_block = 7849242405005690367;
            } else {
                current_block = 7095457783677275021;
            }
        } else {
            current_block = 7095457783677275021;
        }
        match current_block {
            7849242405005690367 => {}
            _ => {
                if (*enc).has_alpha_ != 0 {
                    err = PutAlphaChunk(enc);
                    if err as ::core::ffi::c_uint
                        != VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        current_block = 7849242405005690367;
                    } else {
                        current_block = 14523784380283086299;
                    }
                } else {
                    current_block = 14523784380283086299;
                }
                match current_block {
                    7849242405005690367 => {}
                    _ => {
                        err = PutVP8Header(pic, vp8_size);
                        if !(err as ::core::ffi::c_uint
                            != VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint)
                        {
                            err = PutVP8FrameHeader(pic, (*enc).profile_, size0);
                            if !(err as ::core::ffi::c_uint
                                != VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint)
                            {
                                return 1 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    return WebPEncodingSetError(pic, err);
}
unsafe extern "C" fn PutSegmentHeader(bw: *mut VP8BitWriter, enc: *const VP8Encoder) {
    let hdr: *const VP8EncSegmentHeader = &raw const (*enc).segment_hdr_;
    let proba: *const VP8EncProba = &raw const (*enc).proba_;
    if VP8PutBitUniform(
        bw,
        ((*hdr).num_segments_ > 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
    ) != 0
    {
        let update_data: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut s: ::core::ffi::c_int = 0;
        VP8PutBitUniform(bw, (*hdr).update_map_);
        if VP8PutBitUniform(bw, update_data) != 0 {
            VP8PutBitUniform(bw, 1 as ::core::ffi::c_int);
            s = 0 as ::core::ffi::c_int;
            while s < NUM_MB_SEGMENTS as ::core::ffi::c_int {
                VP8PutSignedBits(bw, (*enc).dqm_[s as usize].quant_, 7 as ::core::ffi::c_int);
                s += 1;
            }
            s = 0 as ::core::ffi::c_int;
            while s < NUM_MB_SEGMENTS as ::core::ffi::c_int {
                VP8PutSignedBits(
                    bw,
                    (*enc).dqm_[s as usize].fstrength_,
                    6 as ::core::ffi::c_int,
                );
                s += 1;
            }
        }
        if (*hdr).update_map_ != 0 {
            s = 0 as ::core::ffi::c_int;
            while s < 3 as ::core::ffi::c_int {
                if VP8PutBitUniform(
                    bw,
                    ((*proba).segments_[s as usize] as ::core::ffi::c_uint
                        != 255 as ::core::ffi::c_uint) as ::core::ffi::c_int,
                ) != 0
                {
                    VP8PutBits(
                        bw,
                        (*proba).segments_[s as usize] as uint32_t,
                        8 as ::core::ffi::c_int,
                    );
                }
                s += 1;
            }
        }
    }
}
unsafe extern "C" fn PutFilterHeader(bw: *mut VP8BitWriter, hdr: *const VP8EncFilterHeader) {
    let use_lf_delta: ::core::ffi::c_int =
        ((*hdr).i4x4_lf_delta_ != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    VP8PutBitUniform(bw, (*hdr).simple_);
    VP8PutBits(bw, (*hdr).level_ as uint32_t, 6 as ::core::ffi::c_int);
    VP8PutBits(bw, (*hdr).sharpness_ as uint32_t, 3 as ::core::ffi::c_int);
    if VP8PutBitUniform(bw, use_lf_delta) != 0 {
        let need_update: ::core::ffi::c_int =
            ((*hdr).i4x4_lf_delta_ != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if VP8PutBitUniform(bw, need_update) != 0 {
            VP8PutBits(bw, 0 as uint32_t, 4 as ::core::ffi::c_int);
            VP8PutSignedBits(bw, (*hdr).i4x4_lf_delta_, 6 as ::core::ffi::c_int);
            VP8PutBits(bw, 0 as uint32_t, 3 as ::core::ffi::c_int);
        }
    }
}
unsafe extern "C" fn PutQuant(bw: *mut VP8BitWriter, enc: *const VP8Encoder) {
    VP8PutBits(bw, (*enc).base_quant_ as uint32_t, 7 as ::core::ffi::c_int);
    VP8PutSignedBits(bw, (*enc).dq_y1_dc_, 4 as ::core::ffi::c_int);
    VP8PutSignedBits(bw, (*enc).dq_y2_dc_, 4 as ::core::ffi::c_int);
    VP8PutSignedBits(bw, (*enc).dq_y2_ac_, 4 as ::core::ffi::c_int);
    VP8PutSignedBits(bw, (*enc).dq_uv_dc_, 4 as ::core::ffi::c_int);
    VP8PutSignedBits(bw, (*enc).dq_uv_ac_, 4 as ::core::ffi::c_int);
}
unsafe extern "C" fn EmitPartitionsSize(
    enc: *const VP8Encoder,
    pic: *mut WebPPicture,
) -> ::core::ffi::c_int {
    let mut buf: [uint8_t; 21] = [0; 21];
    let mut p: ::core::ffi::c_int = 0;
    p = 0 as ::core::ffi::c_int;
    while p < (*enc).num_parts_ - 1 as ::core::ffi::c_int {
        let part_size: size_t =
            VP8BitWriterSize((&raw const (*enc).parts_ as *const VP8BitWriter).offset(p as isize))
                as size_t;
        if part_size >= VP8_MAX_PARTITION_SIZE as size_t {
            return WebPEncodingSetError(pic, VP8_ENC_ERROR_PARTITION_OVERFLOW);
        }
        buf[(3 as ::core::ffi::c_int * p + 0 as ::core::ffi::c_int) as usize] =
            (part_size >> 0 as ::core::ffi::c_int & 0xff as size_t) as uint8_t;
        buf[(3 as ::core::ffi::c_int * p + 1 as ::core::ffi::c_int) as usize] =
            (part_size >> 8 as ::core::ffi::c_int & 0xff as size_t) as uint8_t;
        buf[(3 as ::core::ffi::c_int * p + 2 as ::core::ffi::c_int) as usize] =
            (part_size >> 16 as ::core::ffi::c_int & 0xff as size_t) as uint8_t;
        p += 1;
    }
    if p != 0
        && (*pic).writer.expect("non-null function pointer")(
            &raw mut buf as *mut uint8_t,
            (3 as ::core::ffi::c_int * p) as size_t,
            pic,
        ) == 0
    {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_BAD_WRITE);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn GeneratePartition0(enc: *mut VP8Encoder) -> ::core::ffi::c_int {
    let bw: *mut VP8BitWriter = &raw mut (*enc).bw_;
    let mb_size: ::core::ffi::c_int = (*enc).mb_w_ * (*enc).mb_h_;
    let mut pos1: uint64_t = 0;
    let mut pos2: uint64_t = 0;
    let mut pos3: uint64_t = 0;
    pos1 = VP8BitWriterPos(bw);
    if VP8BitWriterInit(
        bw,
        (mb_size * 7 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as size_t,
    ) == 0
    {
        return WebPEncodingSetError((*enc).pic_, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    VP8PutBitUniform(bw, 0 as ::core::ffi::c_int);
    VP8PutBitUniform(bw, 0 as ::core::ffi::c_int);
    PutSegmentHeader(bw, enc);
    PutFilterHeader(bw, &raw mut (*enc).filter_hdr_);
    VP8PutBits(
        bw,
        (if (*enc).num_parts_ == 8 as ::core::ffi::c_int {
            3 as ::core::ffi::c_int
        } else if (*enc).num_parts_ == 4 as ::core::ffi::c_int {
            2 as ::core::ffi::c_int
        } else if (*enc).num_parts_ == 2 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        2 as ::core::ffi::c_int,
    );
    PutQuant(bw, enc);
    VP8PutBitUniform(bw, 0 as ::core::ffi::c_int);
    VP8WriteProbas(bw, &raw mut (*enc).proba_);
    pos2 = VP8BitWriterPos(bw);
    VP8CodeIntraModes(enc);
    VP8BitWriterFinish(bw);
    pos3 = VP8BitWriterPos(bw);
    if !(*(*enc).pic_).stats.is_null() {
        (*(*(*enc).pic_).stats).header_bytes[0 as ::core::ffi::c_int as usize] =
            (pos2.wrapping_sub(pos1).wrapping_add(7 as uint64_t) >> 3 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        (*(*(*enc).pic_).stats).header_bytes[1 as ::core::ffi::c_int as usize] =
            (pos3.wrapping_sub(pos2).wrapping_add(7 as uint64_t) >> 3 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        (*(*(*enc).pic_).stats).alpha_data_size = (*enc).alpha_data_size_ as ::core::ffi::c_int;
    }
    if (*bw).error_ != 0 {
        return WebPEncodingSetError((*enc).pic_, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8EncFreeBitWriters(enc: *mut VP8Encoder) {
    let mut p: ::core::ffi::c_int = 0;
    VP8BitWriterWipeOut(&raw mut (*enc).bw_);
    p = 0 as ::core::ffi::c_int;
    while p < (*enc).num_parts_ {
        VP8BitWriterWipeOut((&raw mut (*enc).parts_ as *mut VP8BitWriter).offset(p as isize));
        p += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8EncWrite(enc: *mut VP8Encoder) -> ::core::ffi::c_int {
    let pic: *mut WebPPicture = (*enc).pic_;
    let bw: *mut VP8BitWriter = &raw mut (*enc).bw_;
    let task_percent: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
    let percent_per_part: ::core::ffi::c_int = task_percent / (*enc).num_parts_;
    let final_percent: ::core::ffi::c_int = (*enc).percent_ + task_percent;
    let mut ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut vp8_size: size_t = 0;
    let mut pad: size_t = 0;
    let mut riff_size: size_t = 0;
    let mut p: ::core::ffi::c_int = 0;
    ok = GeneratePartition0(enc);
    if ok == 0 {
        return 0 as ::core::ffi::c_int;
    }
    vp8_size = (VP8_FRAME_HEADER_SIZE as size_t)
        .wrapping_add(VP8BitWriterSize(bw))
        .wrapping_add(
            (3 as ::core::ffi::c_int * ((*enc).num_parts_ - 1 as ::core::ffi::c_int)) as size_t,
        );
    p = 0 as ::core::ffi::c_int;
    while p < (*enc).num_parts_ {
        vp8_size = vp8_size.wrapping_add(VP8BitWriterSize(
            (&raw mut (*enc).parts_ as *mut VP8BitWriter).offset(p as isize),
        ));
        p += 1;
    }
    pad = vp8_size & 1 as size_t;
    vp8_size = vp8_size.wrapping_add(pad);
    riff_size = ((TAG_SIZE + CHUNK_HEADER_SIZE) as size_t).wrapping_add(vp8_size);
    if IsVP8XNeeded(enc) != 0 {
        riff_size = riff_size.wrapping_add((CHUNK_HEADER_SIZE + VP8X_CHUNK_SIZE) as size_t);
    }
    if (*enc).has_alpha_ != 0 {
        let padded_alpha_size: uint32_t = (*enc)
            .alpha_data_size_
            .wrapping_add((*enc).alpha_data_size_ & 1 as uint32_t);
        riff_size = riff_size.wrapping_add(
            (CHUNK_HEADER_SIZE as uint32_t).wrapping_add(padded_alpha_size) as size_t,
        );
    }
    if riff_size > 0xfffffffe as ::core::ffi::c_uint as size_t {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_FILE_TOO_BIG);
    }
    let part0: *const uint8_t = VP8BitWriterBuf(bw);
    let size0: size_t = VP8BitWriterSize(bw) as size_t;
    ok = (ok != 0
        && PutWebPHeaders(enc, size0, vp8_size, riff_size) != 0
        && (*pic).writer.expect("non-null function pointer")(part0, size0, pic) != 0
        && EmitPartitionsSize(enc, pic) != 0) as ::core::ffi::c_int;
    VP8BitWriterWipeOut(bw);
    p = 0 as ::core::ffi::c_int;
    while p < (*enc).num_parts_ {
        let buf: *const uint8_t =
            VP8BitWriterBuf((&raw mut (*enc).parts_ as *mut VP8BitWriter).offset(p as isize));
        let size: size_t =
            VP8BitWriterSize((&raw mut (*enc).parts_ as *mut VP8BitWriter).offset(p as isize))
                as size_t;
        if size != 0 {
            ok = (ok != 0 && (*pic).writer.expect("non-null function pointer")(buf, size, pic) != 0)
                as ::core::ffi::c_int;
        }
        VP8BitWriterWipeOut((&raw mut (*enc).parts_ as *mut VP8BitWriter).offset(p as isize));
        ok = (ok != 0
            && WebPReportProgress(
                pic,
                (*enc).percent_ + percent_per_part,
                &raw mut (*enc).percent_,
            ) != 0) as ::core::ffi::c_int;
        p += 1;
    }
    if ok != 0 && pad != 0 {
        ok = PutPaddingByte(pic);
    }
    (*enc).coded_size_ =
        (CHUNK_HEADER_SIZE as size_t).wrapping_add(riff_size) as ::core::ffi::c_int;
    ok = (ok != 0 && WebPReportProgress(pic, final_percent, &raw mut (*enc).percent_) != 0)
        as ::core::ffi::c_int;
    if ok == 0 {
        WebPEncodingSetError(pic, VP8_ENC_ERROR_BAD_WRITE);
    }
    return ok;
}
