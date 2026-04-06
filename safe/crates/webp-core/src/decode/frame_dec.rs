#[repr(C)]
pub struct ALPHDecoder {
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
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    static mut VP8Transform: VP8DecIdct2;
    static mut VP8TransformAC3: VP8DecIdct;
    static mut VP8TransformUV: VP8DecIdct;
    static mut VP8TransformDC: VP8DecIdct;
    static mut VP8TransformDCUV: VP8DecIdct;
    static mut VP8PredLuma16: [VP8PredFunc; 0];
    static mut VP8PredChroma8: [VP8PredFunc; 0];
    static mut VP8PredLuma4: [VP8PredFunc; 0];
    static mut VP8SimpleVFilter16: VP8SimpleFilterFunc;
    static mut VP8SimpleHFilter16: VP8SimpleFilterFunc;
    static mut VP8SimpleVFilter16i: VP8SimpleFilterFunc;
    static mut VP8SimpleHFilter16i: VP8SimpleFilterFunc;
    static mut VP8VFilter16: VP8LumaFilterFunc;
    static mut VP8HFilter16: VP8LumaFilterFunc;
    static mut VP8VFilter8: VP8ChromaFilterFunc;
    static mut VP8HFilter8: VP8ChromaFilterFunc;
    static mut VP8VFilter16i: VP8LumaFilterFunc;
    static mut VP8HFilter16i: VP8LumaFilterFunc;
    static mut VP8VFilter8i: VP8ChromaFilterFunc;
    static mut VP8HFilter8i: VP8ChromaFilterFunc;
    static mut VP8DitherCombine8x8:
        Option<unsafe extern "C" fn(*const uint8_t, *mut uint8_t, ::core::ffi::c_int) -> ()>;
    fn VP8DspInit();
    fn VP8InitRandom(rg: *mut VP8Random, dithering: ::core::ffi::c_float);
    fn WebPGetWorkerInterface() -> *const WebPWorkerInterface;
    fn VP8SetError(
        dec: *mut VP8Decoder,
        error: VP8StatusCode,
        msg: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn VP8InitScanline(dec: *mut VP8Decoder);
    fn VP8DecompressAlphaRows(
        dec: *mut VP8Decoder,
        io: *const VP8Io,
        row: ::core::ffi::c_int,
        num_rows: ::core::ffi::c_int,
    ) -> *const uint8_t;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
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
pub struct WebPDecoderOptions {
    pub bypass_filtering: ::core::ffi::c_int,
    pub no_fancy_upsampling: ::core::ffi::c_int,
    pub use_cropping: ::core::ffi::c_int,
    pub crop_left: ::core::ffi::c_int,
    pub crop_top: ::core::ffi::c_int,
    pub crop_width: ::core::ffi::c_int,
    pub crop_height: ::core::ffi::c_int,
    pub use_scaling: ::core::ffi::c_int,
    pub scaled_width: ::core::ffi::c_int,
    pub scaled_height: ::core::ffi::c_int,
    pub use_threads: ::core::ffi::c_int,
    pub dithering_strength: ::core::ffi::c_int,
    pub flip: ::core::ffi::c_int,
    pub alpha_dithering_strength: ::core::ffi::c_int,
    pub pad: [uint32_t; 5],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPHeaderStructure {
    pub data: *const uint8_t,
    pub data_size: size_t,
    pub have_all_data: ::core::ffi::c_int,
    pub offset: size_t,
    pub alpha_data: *const uint8_t,
    pub alpha_data_size: size_t,
    pub compressed_size: size_t,
    pub riff_size: size_t,
    pub is_lossless: ::core::ffi::c_int,
}
pub type VP8DecIdct = Option<unsafe extern "C" fn(*const int16_t, *mut uint8_t) -> ()>;
pub type VP8DecIdct2 =
    Option<unsafe extern "C" fn(*const int16_t, *mut uint8_t, ::core::ffi::c_int) -> ()>;
pub type VP8PredFunc = Option<unsafe extern "C" fn(*mut uint8_t) -> ()>;
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
pub struct WebPWorkerInterface {
    pub Init: Option<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
    pub Reset: Option<unsafe extern "C" fn(*mut WebPWorker) -> ::core::ffi::c_int>,
    pub Sync_0: Option<unsafe extern "C" fn(*mut WebPWorker) -> ::core::ffi::c_int>,
    pub Launch: Option<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
    pub Execute: Option<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
    pub End: Option<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 47] =
    crate::compat::c_char_array(b"int VP8RandomBits2(VP8Random *const, int, int)\0");
pub const BPS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const VP8_DITHER_AMP_BITS: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const VP8_RANDOM_DITHER_FIX: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const VP8_RANDOM_TABLE_SIZE: ::core::ffi::c_int = 55 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn VP8RandomBits2(
    rg: *mut VP8Random,
    mut num_bits: ::core::ffi::c_int,
    mut amp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut diff: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if num_bits + 8 as ::core::ffi::c_int <= 31 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"num_bits + VP8_RANDOM_DITHER_FIX <= 31\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"original/src/utils/random_utils.h\0" as *const u8 as *const ::core::ffi::c_char,
                42 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    diff = (*rg).tab_[(*rg).index1_ as usize].wrapping_sub((*rg).tab_[(*rg).index2_ as usize])
        as ::core::ffi::c_int;
    if diff < 0 as ::core::ffi::c_int {
        diff = (diff as ::core::ffi::c_uint)
            .wrapping_add((1 as ::core::ffi::c_uint) << 31 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    (*rg).tab_[(*rg).index1_ as usize] = diff as uint32_t;
    (*rg).index1_ += 1;
    if (*rg).index1_ == VP8_RANDOM_TABLE_SIZE {
        (*rg).index1_ = 0 as ::core::ffi::c_int;
    }
    (*rg).index2_ += 1;
    if (*rg).index2_ == VP8_RANDOM_TABLE_SIZE {
        (*rg).index2_ = 0 as ::core::ffi::c_int;
    }
    diff = ((diff as uint32_t) << 1 as ::core::ffi::c_int) as ::core::ffi::c_int
        >> 32 as ::core::ffi::c_int - num_bits;
    diff = diff * amp >> VP8_RANDOM_DITHER_FIX;
    diff += (1 as ::core::ffi::c_int) << num_bits - 1 as ::core::ffi::c_int;
    return diff;
}
pub const YUV_SIZE: ::core::ffi::c_int =
    BPS * 17 as ::core::ffi::c_int + BPS * 9 as ::core::ffi::c_int;
pub const Y_OFF: ::core::ffi::c_int = BPS * 1 as ::core::ffi::c_int + 8 as ::core::ffi::c_int;
pub const U_OFF: ::core::ffi::c_int = Y_OFF + BPS * 16 as ::core::ffi::c_int + BPS;
pub const V_OFF: ::core::ffi::c_int = U_OFF + 16 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn CheckSizeOverflow(mut size: uint64_t) -> ::core::ffi::c_int {
    return (size == size) as ::core::ffi::c_int;
}
pub const WEBP_ALIGN_CST: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
static mut kScan: [uint16_t; 16] = [
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
unsafe extern "C" fn CheckMode(
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if mode == B_DC_PRED as ::core::ffi::c_int {
        if mb_x == 0 as ::core::ffi::c_int {
            return if mb_y == 0 as ::core::ffi::c_int {
                B_DC_PRED_NOTOPLEFT as ::core::ffi::c_int
            } else {
                B_DC_PRED_NOLEFT as ::core::ffi::c_int
            };
        } else {
            return if mb_y == 0 as ::core::ffi::c_int {
                B_DC_PRED_NOTOP as ::core::ffi::c_int
            } else {
                B_DC_PRED as ::core::ffi::c_int
            };
        }
    }
    return mode;
}
unsafe extern "C" fn Copy32b(dst: *mut uint8_t, src: *const uint8_t) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        4 as size_t,
    );
}
#[inline]
unsafe extern "C" fn DoTransform(mut bits: uint32_t, src: *const int16_t, dst: *mut uint8_t) {
    match bits >> 30 as ::core::ffi::c_int {
        3 => {
            VP8Transform.expect("non-null function pointer")(src, dst, 0 as ::core::ffi::c_int);
        }
        2 => {
            VP8TransformAC3.expect("non-null function pointer")(src, dst);
        }
        1 => {
            VP8TransformDC.expect("non-null function pointer")(src, dst);
        }
        _ => {}
    };
}
unsafe extern "C" fn DoUVTransform(mut bits: uint32_t, src: *const int16_t, dst: *mut uint8_t) {
    if bits & 0xff as uint32_t != 0 {
        if bits & 0xaa as uint32_t != 0 {
            VP8TransformUV.expect("non-null function pointer")(src, dst);
        } else {
            VP8TransformDCUV.expect("non-null function pointer")(src, dst);
        }
    }
}
unsafe extern "C" fn ReconstructRow(dec: *const VP8Decoder, mut ctx: *const VP8ThreadContext) {
    let mut j: ::core::ffi::c_int = 0;
    let mut mb_x: ::core::ffi::c_int = 0;
    let mb_y: ::core::ffi::c_int = (*ctx).mb_y_;
    let cache_id: ::core::ffi::c_int = (*ctx).id_;
    let y_dst: *mut uint8_t = (*dec).yuv_b_.offset(Y_OFF as isize);
    let u_dst: *mut uint8_t = (*dec).yuv_b_.offset(U_OFF as isize);
    let v_dst: *mut uint8_t = (*dec).yuv_b_.offset(V_OFF as isize);
    j = 0 as ::core::ffi::c_int;
    while j < 16 as ::core::ffi::c_int {
        *y_dst.offset((j * BPS - 1 as ::core::ffi::c_int) as isize) = 129 as uint8_t;
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < 8 as ::core::ffi::c_int {
        *u_dst.offset((j * BPS - 1 as ::core::ffi::c_int) as isize) = 129 as uint8_t;
        *v_dst.offset((j * BPS - 1 as ::core::ffi::c_int) as isize) = 129 as uint8_t;
        j += 1;
    }
    if mb_y > 0 as ::core::ffi::c_int {
        let ref mut fresh0 = *v_dst.offset((-(1 as ::core::ffi::c_int) - BPS) as isize);
        *fresh0 = 129 as uint8_t;
        let ref mut fresh1 = *u_dst.offset((-(1 as ::core::ffi::c_int) - BPS) as isize);
        *fresh1 = *fresh0;
        *y_dst.offset((-(1 as ::core::ffi::c_int) - BPS) as isize) = *fresh1;
    } else {
        memset(
            y_dst
                .offset(-(BPS as isize))
                .offset(-(1 as ::core::ffi::c_int as isize))
                as *mut ::core::ffi::c_void,
            127 as ::core::ffi::c_int,
            (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as size_t,
        );
        memset(
            u_dst
                .offset(-(BPS as isize))
                .offset(-(1 as ::core::ffi::c_int as isize))
                as *mut ::core::ffi::c_void,
            127 as ::core::ffi::c_int,
            (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
        );
        memset(
            v_dst
                .offset(-(BPS as isize))
                .offset(-(1 as ::core::ffi::c_int as isize))
                as *mut ::core::ffi::c_void,
            127 as ::core::ffi::c_int,
            (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
        );
    }
    mb_x = 0 as ::core::ffi::c_int;
    while mb_x < (*dec).mb_w_ {
        let block: *const VP8MBData = (*ctx).mb_data_.offset(mb_x as isize);
        if mb_x > 0 as ::core::ffi::c_int {
            j = -(1 as ::core::ffi::c_int);
            while j < 16 as ::core::ffi::c_int {
                Copy32b(
                    y_dst.offset((j * BPS - 4 as ::core::ffi::c_int) as isize) as *mut uint8_t,
                    y_dst.offset((j * BPS + 12 as ::core::ffi::c_int) as isize) as *mut uint8_t,
                );
                j += 1;
            }
            j = -(1 as ::core::ffi::c_int);
            while j < 8 as ::core::ffi::c_int {
                Copy32b(
                    u_dst.offset((j * BPS - 4 as ::core::ffi::c_int) as isize) as *mut uint8_t,
                    u_dst.offset((j * BPS + 4 as ::core::ffi::c_int) as isize) as *mut uint8_t,
                );
                Copy32b(
                    v_dst.offset((j * BPS - 4 as ::core::ffi::c_int) as isize) as *mut uint8_t,
                    v_dst.offset((j * BPS + 4 as ::core::ffi::c_int) as isize) as *mut uint8_t,
                );
                j += 1;
            }
        }
        let top_yuv: *mut VP8TopSamples = (*dec).yuv_t_.offset(mb_x as isize);
        let coeffs: *const int16_t = &raw const (*block).coeffs_ as *const int16_t;
        let mut bits: uint32_t = (*block).non_zero_y_;
        let mut n: ::core::ffi::c_int = 0;
        if mb_y > 0 as ::core::ffi::c_int {
            memcpy(
                y_dst.offset(-(BPS as isize)) as *mut ::core::ffi::c_void,
                &raw mut (*top_yuv.offset(0 as ::core::ffi::c_int as isize)).y as *mut uint8_t
                    as *const ::core::ffi::c_void,
                16 as size_t,
            );
            memcpy(
                u_dst.offset(-(BPS as isize)) as *mut ::core::ffi::c_void,
                &raw mut (*top_yuv.offset(0 as ::core::ffi::c_int as isize)).u as *mut uint8_t
                    as *const ::core::ffi::c_void,
                8 as size_t,
            );
            memcpy(
                v_dst.offset(-(BPS as isize)) as *mut ::core::ffi::c_void,
                &raw mut (*top_yuv.offset(0 as ::core::ffi::c_int as isize)).v as *mut uint8_t
                    as *const ::core::ffi::c_void,
                8 as size_t,
            );
        }
        if (*block).is_i4x4_ != 0 {
            let top_right: *mut uint32_t = y_dst
                .offset(-(BPS as isize))
                .offset(16 as ::core::ffi::c_int as isize)
                as *mut uint32_t;
            if mb_y > 0 as ::core::ffi::c_int {
                if mb_x >= (*dec).mb_w_ - 1 as ::core::ffi::c_int {
                    memset(
                        top_right as *mut ::core::ffi::c_void,
                        (*top_yuv.offset(0 as ::core::ffi::c_int as isize)).y
                            [15 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int,
                        ::core::mem::size_of::<uint32_t>() as size_t,
                    );
                } else {
                    memcpy(
                        top_right as *mut ::core::ffi::c_void,
                        &raw mut (*top_yuv.offset(1 as ::core::ffi::c_int as isize)).y
                            as *mut uint8_t as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<uint32_t>() as size_t,
                    );
                }
            }
            let ref mut fresh2 = *top_right.offset((3 as ::core::ffi::c_int * BPS) as isize);
            *fresh2 = *top_right.offset(0 as ::core::ffi::c_int as isize);
            let ref mut fresh3 = *top_right.offset((2 as ::core::ffi::c_int * BPS) as isize);
            *fresh3 = *fresh2;
            *top_right.offset(BPS as isize) = *fresh3;
            n = 0 as ::core::ffi::c_int;
            while n < 16 as ::core::ffi::c_int {
                let dst: *mut uint8_t =
                    y_dst.offset(kScan[n as usize] as ::core::ffi::c_int as isize);
                (*(&raw mut VP8PredLuma4 as *mut VP8PredFunc)
                    .offset((*block).imodes_[n as usize] as isize))
                .expect("non-null function pointer")(dst);
                DoTransform(
                    bits,
                    coeffs.offset((n * 16 as ::core::ffi::c_int) as isize),
                    dst,
                );
                n += 1;
                bits <<= 2 as ::core::ffi::c_int;
            }
        } else {
            let pred_func: ::core::ffi::c_int = CheckMode(
                mb_x,
                mb_y,
                (*block).imodes_[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
            (*(&raw mut VP8PredLuma16 as *mut VP8PredFunc).offset(pred_func as isize))
                .expect("non-null function pointer")(y_dst);
            if bits != 0 as uint32_t {
                n = 0 as ::core::ffi::c_int;
                while n < 16 as ::core::ffi::c_int {
                    DoTransform(
                        bits,
                        coeffs.offset((n * 16 as ::core::ffi::c_int) as isize),
                        y_dst.offset(kScan[n as usize] as ::core::ffi::c_int as isize),
                    );
                    n += 1;
                    bits <<= 2 as ::core::ffi::c_int;
                }
            }
        }
        let bits_uv: uint32_t = (*block).non_zero_uv_;
        let pred_func_0: ::core::ffi::c_int =
            CheckMode(mb_x, mb_y, (*block).uvmode_ as ::core::ffi::c_int) as ::core::ffi::c_int;
        (*(&raw mut VP8PredChroma8 as *mut VP8PredFunc).offset(pred_func_0 as isize))
            .expect("non-null function pointer")(u_dst);
        (*(&raw mut VP8PredChroma8 as *mut VP8PredFunc).offset(pred_func_0 as isize))
            .expect("non-null function pointer")(v_dst);
        DoUVTransform(
            bits_uv >> 0 as ::core::ffi::c_int,
            coeffs.offset((16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize),
            u_dst,
        );
        DoUVTransform(
            bits_uv >> 8 as ::core::ffi::c_int,
            coeffs.offset((20 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize),
            v_dst,
        );
        if mb_y < (*dec).mb_h_ - 1 as ::core::ffi::c_int {
            memcpy(
                &raw mut (*top_yuv.offset(0 as ::core::ffi::c_int as isize)).y as *mut uint8_t
                    as *mut ::core::ffi::c_void,
                y_dst.offset((15 as ::core::ffi::c_int * BPS) as isize)
                    as *const ::core::ffi::c_void,
                16 as size_t,
            );
            memcpy(
                &raw mut (*top_yuv.offset(0 as ::core::ffi::c_int as isize)).u as *mut uint8_t
                    as *mut ::core::ffi::c_void,
                u_dst.offset((7 as ::core::ffi::c_int * BPS) as isize)
                    as *const ::core::ffi::c_void,
                8 as size_t,
            );
            memcpy(
                &raw mut (*top_yuv.offset(0 as ::core::ffi::c_int as isize)).v as *mut uint8_t
                    as *mut ::core::ffi::c_void,
                v_dst.offset((7 as ::core::ffi::c_int * BPS) as isize)
                    as *const ::core::ffi::c_void,
                8 as size_t,
            );
        }
        let y_offset: ::core::ffi::c_int =
            cache_id * 16 as ::core::ffi::c_int * (*dec).cache_y_stride_;
        let uv_offset: ::core::ffi::c_int =
            cache_id * 8 as ::core::ffi::c_int * (*dec).cache_uv_stride_;
        let y_out: *mut uint8_t = (*dec)
            .cache_y_
            .offset((mb_x * 16 as ::core::ffi::c_int) as isize)
            .offset(y_offset as isize);
        let u_out: *mut uint8_t = (*dec)
            .cache_u_
            .offset((mb_x * 8 as ::core::ffi::c_int) as isize)
            .offset(uv_offset as isize);
        let v_out: *mut uint8_t = (*dec)
            .cache_v_
            .offset((mb_x * 8 as ::core::ffi::c_int) as isize)
            .offset(uv_offset as isize);
        j = 0 as ::core::ffi::c_int;
        while j < 16 as ::core::ffi::c_int {
            memcpy(
                y_out.offset((j * (*dec).cache_y_stride_) as isize) as *mut ::core::ffi::c_void,
                y_dst.offset((j * BPS) as isize) as *const ::core::ffi::c_void,
                16 as size_t,
            );
            j += 1;
        }
        j = 0 as ::core::ffi::c_int;
        while j < 8 as ::core::ffi::c_int {
            memcpy(
                u_out.offset((j * (*dec).cache_uv_stride_) as isize) as *mut ::core::ffi::c_void,
                u_dst.offset((j * BPS) as isize) as *const ::core::ffi::c_void,
                8 as size_t,
            );
            memcpy(
                v_out.offset((j * (*dec).cache_uv_stride_) as isize) as *mut ::core::ffi::c_void,
                v_dst.offset((j * BPS) as isize) as *const ::core::ffi::c_void,
                8 as size_t,
            );
            j += 1;
        }
        mb_x += 1;
    }
}
static mut kFilterExtraRows: [uint8_t; 3] = [
    0 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
];
unsafe extern "C" fn DoFilter(
    dec: *const VP8Decoder,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
) {
    let ctx: *const VP8ThreadContext = &raw const (*dec).thread_ctx_;
    let cache_id: ::core::ffi::c_int = (*ctx).id_;
    let y_bps: ::core::ffi::c_int = (*dec).cache_y_stride_;
    let f_info: *const VP8FInfo = (*ctx).f_info_.offset(mb_x as isize);
    let y_dst: *mut uint8_t = (*dec)
        .cache_y_
        .offset((cache_id * 16 as ::core::ffi::c_int * y_bps) as isize)
        .offset((mb_x * 16 as ::core::ffi::c_int) as isize);
    let ilevel: ::core::ffi::c_int = (*f_info).f_ilevel_ as ::core::ffi::c_int;
    let limit: ::core::ffi::c_int = (*f_info).f_limit_ as ::core::ffi::c_int;
    if limit == 0 as ::core::ffi::c_int {
        return;
    }
    '_c2rust_label: {
        if limit >= 3 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"limit >= 3\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/frame_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                214 as ::core::ffi::c_uint,
                b"void DoFilter(const VP8Decoder *const, int, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*dec).filter_type_ == 1 as ::core::ffi::c_int {
        if mb_x > 0 as ::core::ffi::c_int {
            VP8SimpleHFilter16.expect("non-null function pointer")(
                y_dst,
                y_bps,
                limit + 4 as ::core::ffi::c_int,
            );
        }
        if (*f_info).f_inner_ != 0 {
            VP8SimpleHFilter16i.expect("non-null function pointer")(y_dst, y_bps, limit);
        }
        if mb_y > 0 as ::core::ffi::c_int {
            VP8SimpleVFilter16.expect("non-null function pointer")(
                y_dst,
                y_bps,
                limit + 4 as ::core::ffi::c_int,
            );
        }
        if (*f_info).f_inner_ != 0 {
            VP8SimpleVFilter16i.expect("non-null function pointer")(y_dst, y_bps, limit);
        }
    } else {
        let uv_bps: ::core::ffi::c_int = (*dec).cache_uv_stride_;
        let u_dst: *mut uint8_t = (*dec)
            .cache_u_
            .offset((cache_id * 8 as ::core::ffi::c_int * uv_bps) as isize)
            .offset((mb_x * 8 as ::core::ffi::c_int) as isize);
        let v_dst: *mut uint8_t = (*dec)
            .cache_v_
            .offset((cache_id * 8 as ::core::ffi::c_int * uv_bps) as isize)
            .offset((mb_x * 8 as ::core::ffi::c_int) as isize);
        let hev_thresh: ::core::ffi::c_int = (*f_info).hev_thresh_ as ::core::ffi::c_int;
        if mb_x > 0 as ::core::ffi::c_int {
            VP8HFilter16.expect("non-null function pointer")(
                y_dst,
                y_bps,
                limit + 4 as ::core::ffi::c_int,
                ilevel,
                hev_thresh,
            );
            VP8HFilter8.expect("non-null function pointer")(
                u_dst,
                v_dst,
                uv_bps,
                limit + 4 as ::core::ffi::c_int,
                ilevel,
                hev_thresh,
            );
        }
        if (*f_info).f_inner_ != 0 {
            VP8HFilter16i.expect("non-null function pointer")(
                y_dst, y_bps, limit, ilevel, hev_thresh,
            );
            VP8HFilter8i.expect("non-null function pointer")(
                u_dst, v_dst, uv_bps, limit, ilevel, hev_thresh,
            );
        }
        if mb_y > 0 as ::core::ffi::c_int {
            VP8VFilter16.expect("non-null function pointer")(
                y_dst,
                y_bps,
                limit + 4 as ::core::ffi::c_int,
                ilevel,
                hev_thresh,
            );
            VP8VFilter8.expect("non-null function pointer")(
                u_dst,
                v_dst,
                uv_bps,
                limit + 4 as ::core::ffi::c_int,
                ilevel,
                hev_thresh,
            );
        }
        if (*f_info).f_inner_ != 0 {
            VP8VFilter16i.expect("non-null function pointer")(
                y_dst, y_bps, limit, ilevel, hev_thresh,
            );
            VP8VFilter8i.expect("non-null function pointer")(
                u_dst, v_dst, uv_bps, limit, ilevel, hev_thresh,
            );
        }
    };
}
unsafe extern "C" fn FilterRow(dec: *const VP8Decoder) {
    let mut mb_x: ::core::ffi::c_int = 0;
    let mb_y: ::core::ffi::c_int = (*dec).thread_ctx_.mb_y_;
    '_c2rust_label: {
        if (*dec).thread_ctx_.filter_row_ != 0 {
        } else {
            __assert_fail(
                b"dec->thread_ctx_.filter_row_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/frame_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                256 as ::core::ffi::c_uint,
                b"void FilterRow(const VP8Decoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    mb_x = (*dec).tl_mb_x_;
    while mb_x < (*dec).br_mb_x_ {
        DoFilter(dec, mb_x, mb_y);
        mb_x += 1;
    }
}
unsafe extern "C" fn PrecomputeFilterStrengths(dec: *mut VP8Decoder) {
    if (*dec).filter_type_ > 0 as ::core::ffi::c_int {
        let mut s: ::core::ffi::c_int = 0;
        let hdr: *const VP8FilterHeader = &raw mut (*dec).filter_hdr_;
        s = 0 as ::core::ffi::c_int;
        while s < NUM_MB_SEGMENTS as ::core::ffi::c_int {
            let mut i4x4: ::core::ffi::c_int = 0;
            let mut base_level: ::core::ffi::c_int = 0;
            if (*dec).segment_hdr_.use_segment_ != 0 {
                base_level = (*dec).segment_hdr_.filter_strength_[s as usize] as ::core::ffi::c_int;
                if (*dec).segment_hdr_.absolute_delta_ == 0 {
                    base_level += (*hdr).level_;
                }
            } else {
                base_level = (*hdr).level_;
            }
            i4x4 = 0 as ::core::ffi::c_int;
            while i4x4 <= 1 as ::core::ffi::c_int {
                let info: *mut VP8FInfo =
                    (&raw mut *(&raw mut (*dec).fstrengths_ as *mut [VP8FInfo; 2])
                        .offset(s as isize) as *mut VP8FInfo)
                        .offset(i4x4 as isize) as *mut VP8FInfo;
                let mut level: ::core::ffi::c_int = base_level;
                if (*hdr).use_lf_delta_ != 0 {
                    level += (*hdr).ref_lf_delta_[0 as ::core::ffi::c_int as usize];
                    if i4x4 != 0 {
                        level += (*hdr).mode_lf_delta_[0 as ::core::ffi::c_int as usize];
                    }
                }
                level = if level < 0 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int
                } else if level > 63 as ::core::ffi::c_int {
                    63 as ::core::ffi::c_int
                } else {
                    level
                };
                if level > 0 as ::core::ffi::c_int {
                    let mut ilevel: ::core::ffi::c_int = level;
                    if (*hdr).sharpness_ > 0 as ::core::ffi::c_int {
                        if (*hdr).sharpness_ > 4 as ::core::ffi::c_int {
                            ilevel >>= 2 as ::core::ffi::c_int;
                        } else {
                            ilevel >>= 1 as ::core::ffi::c_int;
                        }
                        if ilevel > 9 as ::core::ffi::c_int - (*hdr).sharpness_ {
                            ilevel = 9 as ::core::ffi::c_int - (*hdr).sharpness_;
                        }
                    }
                    if ilevel < 1 as ::core::ffi::c_int {
                        ilevel = 1 as ::core::ffi::c_int;
                    }
                    (*info).f_ilevel_ = ilevel as uint8_t;
                    (*info).f_limit_ = (2 as ::core::ffi::c_int * level + ilevel) as uint8_t;
                    (*info).hev_thresh_ = (if level >= 40 as ::core::ffi::c_int {
                        2 as ::core::ffi::c_int
                    } else if level >= 15 as ::core::ffi::c_int {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t;
                } else {
                    (*info).f_limit_ = 0 as uint8_t;
                }
                (*info).f_inner_ = i4x4 as uint8_t;
                i4x4 += 1;
            }
            s += 1;
        }
    }
}
pub const MIN_DITHER_AMP: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DITHER_AMP_TAB_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
static mut kQuantToDitherAmp: [uint8_t; 12] = [
    8 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub unsafe extern "C" fn VP8InitDithering(
    options: *const WebPDecoderOptions,
    dec: *mut VP8Decoder,
) {
    '_c2rust_label: {
        if !dec.is_null() {
        } else {
            __assert_fail(
                b"dec != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/frame_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                330 as ::core::ffi::c_uint,
                b"void VP8InitDithering(const WebPDecoderOptions *const, VP8Decoder *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if !options.is_null() {
        let d: ::core::ffi::c_int = (*options).dithering_strength;
        let max_amp: ::core::ffi::c_int =
            ((1 as ::core::ffi::c_int) << VP8_RANDOM_DITHER_FIX) - 1 as ::core::ffi::c_int;
        let f: ::core::ffi::c_int = if d < 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else if d > 100 as ::core::ffi::c_int {
            max_amp
        } else {
            d * max_amp / 100 as ::core::ffi::c_int
        };
        if f > 0 as ::core::ffi::c_int {
            let mut s: ::core::ffi::c_int = 0;
            let mut all_amp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            s = 0 as ::core::ffi::c_int;
            while s < NUM_MB_SEGMENTS as ::core::ffi::c_int {
                let dqm: *mut VP8QuantMatrix = (&raw mut (*dec).dqm_ as *mut VP8QuantMatrix)
                    .offset(s as isize)
                    as *mut VP8QuantMatrix;
                if (*dqm).uv_quant_ < DITHER_AMP_TAB_SIZE {
                    let idx: ::core::ffi::c_int = if (*dqm).uv_quant_ < 0 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int
                    } else {
                        (*dqm).uv_quant_
                    };
                    (*dqm).dither_ = f * kQuantToDitherAmp[idx as usize] as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int;
                }
                all_amp |= (*dqm).dither_;
                s += 1;
            }
            if all_amp != 0 as ::core::ffi::c_int {
                VP8InitRandom(&raw mut (*dec).dithering_rg_, 1.0f32);
                (*dec).dither_ = 1 as ::core::ffi::c_int;
            }
        }
        (*dec).alpha_dithering_ = (*options).alpha_dithering_strength;
        if (*dec).alpha_dithering_ > 100 as ::core::ffi::c_int {
            (*dec).alpha_dithering_ = 100 as ::core::ffi::c_int;
        } else if (*dec).alpha_dithering_ < 0 as ::core::ffi::c_int {
            (*dec).alpha_dithering_ = 0 as ::core::ffi::c_int;
        }
    }
}
unsafe extern "C" fn Dither8x8(
    rg: *mut VP8Random,
    mut dst: *mut uint8_t,
    mut bps: ::core::ffi::c_int,
    mut amp: ::core::ffi::c_int,
) {
    let mut dither: [uint8_t; 64] = [0; 64];
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int {
        dither[i as usize] =
            VP8RandomBits2(rg, VP8_DITHER_AMP_BITS + 1 as ::core::ffi::c_int, amp) as uint8_t;
        i += 1;
    }
    VP8DitherCombine8x8.expect("non-null function pointer")(
        &raw mut dither as *mut uint8_t,
        dst,
        bps,
    );
}
unsafe extern "C" fn DitherRow(dec: *mut VP8Decoder) {
    let mut mb_x: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if (*dec).dither_ != 0 {
        } else {
            __assert_fail(
                b"dec->dither_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/frame_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                373 as ::core::ffi::c_uint,
                b"void DitherRow(VP8Decoder *const)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    mb_x = (*dec).tl_mb_x_;
    while mb_x < (*dec).br_mb_x_ {
        let ctx: *const VP8ThreadContext = &raw mut (*dec).thread_ctx_;
        let data: *const VP8MBData = (*ctx).mb_data_.offset(mb_x as isize);
        let cache_id: ::core::ffi::c_int = (*ctx).id_;
        let uv_bps: ::core::ffi::c_int = (*dec).cache_uv_stride_;
        if (*data).dither_ as ::core::ffi::c_int >= MIN_DITHER_AMP {
            let u_dst: *mut uint8_t = (*dec)
                .cache_u_
                .offset((cache_id * 8 as ::core::ffi::c_int * uv_bps) as isize)
                .offset((mb_x * 8 as ::core::ffi::c_int) as isize);
            let v_dst: *mut uint8_t = (*dec)
                .cache_v_
                .offset((cache_id * 8 as ::core::ffi::c_int * uv_bps) as isize)
                .offset((mb_x * 8 as ::core::ffi::c_int) as isize);
            Dither8x8(
                &raw mut (*dec).dithering_rg_,
                u_dst,
                uv_bps,
                (*data).dither_ as ::core::ffi::c_int,
            );
            Dither8x8(
                &raw mut (*dec).dithering_rg_,
                v_dst,
                uv_bps,
                (*data).dither_ as ::core::ffi::c_int,
            );
        }
        mb_x += 1;
    }
}
unsafe extern "C" fn FinishRow(
    mut arg1: *mut ::core::ffi::c_void,
    mut arg2: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let dec: *mut VP8Decoder = arg1 as *mut VP8Decoder;
    let io: *mut VP8Io = arg2 as *mut VP8Io;
    let mut ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let ctx: *const VP8ThreadContext = &raw mut (*dec).thread_ctx_;
    let cache_id: ::core::ffi::c_int = (*ctx).id_;
    let extra_y_rows: ::core::ffi::c_int =
        kFilterExtraRows[(*dec).filter_type_ as usize] as ::core::ffi::c_int;
    let ysize: ::core::ffi::c_int = extra_y_rows * (*dec).cache_y_stride_;
    let uvsize: ::core::ffi::c_int =
        extra_y_rows / 2 as ::core::ffi::c_int * (*dec).cache_uv_stride_;
    let y_offset: ::core::ffi::c_int = cache_id * 16 as ::core::ffi::c_int * (*dec).cache_y_stride_;
    let uv_offset: ::core::ffi::c_int =
        cache_id * 8 as ::core::ffi::c_int * (*dec).cache_uv_stride_;
    let ydst: *mut uint8_t = (*dec)
        .cache_y_
        .offset(-(ysize as isize))
        .offset(y_offset as isize);
    let udst: *mut uint8_t = (*dec)
        .cache_u_
        .offset(-(uvsize as isize))
        .offset(uv_offset as isize);
    let vdst: *mut uint8_t = (*dec)
        .cache_v_
        .offset(-(uvsize as isize))
        .offset(uv_offset as isize);
    let mb_y: ::core::ffi::c_int = (*ctx).mb_y_;
    let is_first_row: ::core::ffi::c_int = (mb_y == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let is_last_row: ::core::ffi::c_int =
        (mb_y >= (*dec).br_mb_y_ - 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if (*dec).mt_method_ == 2 as ::core::ffi::c_int {
        ReconstructRow(dec, ctx);
    }
    if (*ctx).filter_row_ != 0 {
        FilterRow(dec);
    }
    if (*dec).dither_ != 0 {
        DitherRow(dec);
    }
    if (*io).put.is_some() {
        let mut y_start: ::core::ffi::c_int = mb_y * 16 as ::core::ffi::c_int;
        let mut y_end: ::core::ffi::c_int =
            (mb_y + 1 as ::core::ffi::c_int) * 16 as ::core::ffi::c_int;
        if is_first_row == 0 {
            y_start -= extra_y_rows;
            (*io).y = ydst;
            (*io).u = udst;
            (*io).v = vdst;
        } else {
            (*io).y = (*dec).cache_y_.offset(y_offset as isize);
            (*io).u = (*dec).cache_u_.offset(uv_offset as isize);
            (*io).v = (*dec).cache_v_.offset(uv_offset as isize);
        }
        if is_last_row == 0 {
            y_end -= extra_y_rows;
        }
        if y_end > (*io).crop_bottom {
            y_end = (*io).crop_bottom;
        }
        (*io).a = ::core::ptr::null::<uint8_t>();
        if !(*dec).alpha_data_.is_null() && y_start < y_end {
            (*io).a = VP8DecompressAlphaRows(dec, io, y_start, y_end - y_start);
            if (*io).a.is_null() {
                return VP8SetError(
                    dec,
                    VP8_STATUS_BITSTREAM_ERROR,
                    b"Could not decode alpha data.\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        if y_start < (*io).crop_top {
            let delta_y: ::core::ffi::c_int = (*io).crop_top - y_start;
            y_start = (*io).crop_top;
            '_c2rust_label: {
                if delta_y & 1 as ::core::ffi::c_int == 0 {
                } else {
                    __assert_fail(
                        b"!(delta_y & 1)\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/frame_dec.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        464 as ::core::ffi::c_uint,
                        b"int FinishRow(void *, void *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            (*io).y = (*io).y.offset(((*dec).cache_y_stride_ * delta_y) as isize);
            (*io).u = (*io)
                .u
                .offset(((*dec).cache_uv_stride_ * (delta_y >> 1 as ::core::ffi::c_int)) as isize);
            (*io).v = (*io)
                .v
                .offset(((*dec).cache_uv_stride_ * (delta_y >> 1 as ::core::ffi::c_int)) as isize);
            if !(*io).a.is_null() {
                (*io).a = (*io).a.offset(((*io).width * delta_y) as isize);
            }
        }
        if y_start < y_end {
            (*io).y = (*io).y.offset((*io).crop_left as isize);
            (*io).u = (*io)
                .u
                .offset(((*io).crop_left >> 1 as ::core::ffi::c_int) as isize);
            (*io).v = (*io)
                .v
                .offset(((*io).crop_left >> 1 as ::core::ffi::c_int) as isize);
            if !(*io).a.is_null() {
                (*io).a = (*io).a.offset((*io).crop_left as isize);
            }
            (*io).mb_y = y_start - (*io).crop_top;
            (*io).mb_w = (*io).crop_right - (*io).crop_left;
            (*io).mb_h = y_end - y_start;
            ok = (*io).put.expect("non-null function pointer")(io);
        }
    }
    if cache_id + 1 as ::core::ffi::c_int == (*dec).num_caches_ {
        if is_last_row == 0 {
            memcpy(
                (*dec).cache_y_.offset(-(ysize as isize)) as *mut ::core::ffi::c_void,
                ydst.offset((16 as ::core::ffi::c_int * (*dec).cache_y_stride_) as isize)
                    as *const ::core::ffi::c_void,
                ysize as size_t,
            );
            memcpy(
                (*dec).cache_u_.offset(-(uvsize as isize)) as *mut ::core::ffi::c_void,
                udst.offset((8 as ::core::ffi::c_int * (*dec).cache_uv_stride_) as isize)
                    as *const ::core::ffi::c_void,
                uvsize as size_t,
            );
            memcpy(
                (*dec).cache_v_.offset(-(uvsize as isize)) as *mut ::core::ffi::c_void,
                vdst.offset((8 as ::core::ffi::c_int * (*dec).cache_uv_stride_) as isize)
                    as *const ::core::ffi::c_void,
                uvsize as size_t,
            );
        }
    }
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn VP8ProcessRow(dec: *mut VP8Decoder, io: *mut VP8Io) -> ::core::ffi::c_int {
    let mut ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let ctx: *mut VP8ThreadContext = &raw mut (*dec).thread_ctx_;
    let filter_row: ::core::ffi::c_int = ((*dec).filter_type_ > 0 as ::core::ffi::c_int
        && (*dec).mb_y_ >= (*dec).tl_mb_y_
        && (*dec).mb_y_ <= (*dec).br_mb_y_)
        as ::core::ffi::c_int;
    if (*dec).mt_method_ == 0 as ::core::ffi::c_int {
        (*ctx).mb_y_ = (*dec).mb_y_;
        (*ctx).filter_row_ = filter_row;
        ReconstructRow(dec, ctx);
        ok = FinishRow(
            dec as *mut ::core::ffi::c_void,
            io as *mut ::core::ffi::c_void,
        );
    } else {
        let worker: *mut WebPWorker = &raw mut (*dec).worker_;
        ok &= (*WebPGetWorkerInterface())
            .Sync_0
            .expect("non-null function pointer")(worker);
        '_c2rust_label: {
            if (*worker).status_ as ::core::ffi::c_uint
                == OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"worker->status_ == OK\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/frame_dec.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    517 as ::core::ffi::c_uint,
                    b"int VP8ProcessRow(VP8Decoder *const, VP8Io *const)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        if ok != 0 {
            (*ctx).io_ = *io;
            (*ctx).id_ = (*dec).cache_id_;
            (*ctx).mb_y_ = (*dec).mb_y_;
            (*ctx).filter_row_ = filter_row;
            if (*dec).mt_method_ == 2 as ::core::ffi::c_int {
                let tmp: *mut VP8MBData = (*ctx).mb_data_;
                (*ctx).mb_data_ = (*dec).mb_data_;
                (*dec).mb_data_ = tmp;
            } else {
                ReconstructRow(dec, ctx);
            }
            if filter_row != 0 {
                let tmp_0: *mut VP8FInfo = (*ctx).f_info_;
                (*ctx).f_info_ = (*dec).f_info_;
                (*dec).f_info_ = tmp_0;
            }
            (*WebPGetWorkerInterface())
                .Launch
                .expect("non-null function pointer")(worker);
            (*dec).cache_id_ += 1;
            if (*dec).cache_id_ == (*dec).num_caches_ {
                (*dec).cache_id_ = 0 as ::core::ffi::c_int;
            }
        }
    }
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn VP8EnterCritical(dec: *mut VP8Decoder, io: *mut VP8Io) -> VP8StatusCode {
    if (*io).setup.is_some() && (*io).setup.expect("non-null function pointer")(io) == 0 {
        VP8SetError(
            dec,
            VP8_STATUS_USER_ABORT,
            b"Frame setup failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (*dec).status_;
    }
    if (*io).bypass_filtering != 0 {
        (*dec).filter_type_ = 0 as ::core::ffi::c_int;
    }
    let extra_pixels: ::core::ffi::c_int =
        kFilterExtraRows[(*dec).filter_type_ as usize] as ::core::ffi::c_int;
    if (*dec).filter_type_ == 2 as ::core::ffi::c_int {
        (*dec).tl_mb_x_ = 0 as ::core::ffi::c_int;
        (*dec).tl_mb_y_ = 0 as ::core::ffi::c_int;
    } else {
        (*dec).tl_mb_x_ = (*io).crop_left - extra_pixels >> 4 as ::core::ffi::c_int;
        (*dec).tl_mb_y_ = (*io).crop_top - extra_pixels >> 4 as ::core::ffi::c_int;
        if (*dec).tl_mb_x_ < 0 as ::core::ffi::c_int {
            (*dec).tl_mb_x_ = 0 as ::core::ffi::c_int;
        }
        if (*dec).tl_mb_y_ < 0 as ::core::ffi::c_int {
            (*dec).tl_mb_y_ = 0 as ::core::ffi::c_int;
        }
    }
    (*dec).br_mb_y_ =
        (*io).crop_bottom + 15 as ::core::ffi::c_int + extra_pixels >> 4 as ::core::ffi::c_int;
    (*dec).br_mb_x_ =
        (*io).crop_right + 15 as ::core::ffi::c_int + extra_pixels >> 4 as ::core::ffi::c_int;
    if (*dec).br_mb_x_ > (*dec).mb_w_ {
        (*dec).br_mb_x_ = (*dec).mb_w_;
    }
    if (*dec).br_mb_y_ > (*dec).mb_h_ {
        (*dec).br_mb_y_ = (*dec).mb_h_;
    }
    PrecomputeFilterStrengths(dec);
    return VP8_STATUS_OK;
}
#[no_mangle]
pub unsafe extern "C" fn VP8ExitCritical(
    dec: *mut VP8Decoder,
    io: *mut VP8Io,
) -> ::core::ffi::c_int {
    let mut ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if (*dec).mt_method_ > 0 as ::core::ffi::c_int {
        ok = (*WebPGetWorkerInterface())
            .Sync_0
            .expect("non-null function pointer")(&raw mut (*dec).worker_);
    }
    if (*io).teardown.is_some() {
        (*io).teardown.expect("non-null function pointer")(io);
    }
    return ok;
}
pub const MT_CACHE_LINES: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ST_CACHE_LINES: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn InitThreadContext(dec: *mut VP8Decoder) -> ::core::ffi::c_int {
    (*dec).cache_id_ = 0 as ::core::ffi::c_int;
    if (*dec).mt_method_ > 0 as ::core::ffi::c_int {
        let worker: *mut WebPWorker = &raw mut (*dec).worker_;
        if (*WebPGetWorkerInterface())
            .Reset
            .expect("non-null function pointer")(worker)
            == 0
        {
            return VP8SetError(
                dec,
                VP8_STATUS_OUT_OF_MEMORY,
                b"thread initialization failed.\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*worker).data1 = dec as *mut ::core::ffi::c_void;
        (*worker).data2 = &raw mut (*dec).thread_ctx_.io_ as *mut ::core::ffi::c_void;
        (*worker).hook = Some(
            FinishRow
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ) as WebPWorkerHook;
        (*dec).num_caches_ = if (*dec).filter_type_ > 0 as ::core::ffi::c_int {
            MT_CACHE_LINES
        } else {
            MT_CACHE_LINES - 1 as ::core::ffi::c_int
        };
    } else {
        (*dec).num_caches_ = ST_CACHE_LINES;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8GetThreadMethod(
    options: *const WebPDecoderOptions,
    headers: *const WebPHeaderStructure,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if options.is_null() || (*options).use_threads == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    '_c2rust_label: {
        if headers.is_null() || (*headers).is_lossless == 0 {
        } else {
            __assert_fail(
                b"headers == NULL || !headers->is_lossless\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/frame_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                669 as ::core::ffi::c_uint,
                b"int VP8GetThreadMethod(const WebPDecoderOptions *const, const WebPHeaderStructure *const, int, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn AllocateMemory(dec: *mut VP8Decoder) -> ::core::ffi::c_int {
    let num_caches: ::core::ffi::c_int = (*dec).num_caches_;
    let mb_w: ::core::ffi::c_int = (*dec).mb_w_;
    let intra_pred_mode_size: size_t = ((4 as ::core::ffi::c_int * mb_w) as size_t)
        .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t);
    let top_size: size_t =
        (::core::mem::size_of::<VP8TopSamples>() as size_t).wrapping_mul(mb_w as size_t);
    let mb_info_size: size_t = ((mb_w + 1 as ::core::ffi::c_int) as size_t)
        .wrapping_mul(::core::mem::size_of::<VP8MB>() as size_t);
    let f_info_size: size_t = if (*dec).filter_type_ > 0 as ::core::ffi::c_int {
        ((mb_w
            * (if (*dec).mt_method_ > 0 as ::core::ffi::c_int {
                2 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            })) as size_t)
            .wrapping_mul(::core::mem::size_of::<VP8FInfo>() as size_t)
    } else {
        0 as size_t
    };
    let yuv_size: size_t =
        (YUV_SIZE as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t);
    let mb_data_size: size_t = (((if (*dec).mt_method_ == 2 as ::core::ffi::c_int {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) * mb_w) as size_t)
        .wrapping_mul(::core::mem::size_of::<VP8MBData>() as size_t);
    let cache_height: size_t = ((16 as ::core::ffi::c_int * num_caches
        + kFilterExtraRows[(*dec).filter_type_ as usize] as ::core::ffi::c_int)
        * 3 as ::core::ffi::c_int
        / 2 as ::core::ffi::c_int) as size_t;
    let cache_size: size_t = top_size.wrapping_mul(cache_height);
    let alpha_size: uint64_t = (if !(*dec).alpha_data_.is_null() {
        ((*dec).pic_hdr_.width_ as uint64_t).wrapping_mul((*dec).pic_hdr_.height_ as uint64_t)
            as ::core::ffi::c_ulonglong
    } else {
        0 as ::core::ffi::c_ulonglong
    }) as uint64_t;
    let needed: uint64_t = (intra_pred_mode_size as uint64_t)
        .wrapping_add(top_size as uint64_t)
        .wrapping_add(mb_info_size as uint64_t)
        .wrapping_add(f_info_size as uint64_t)
        .wrapping_add(yuv_size as uint64_t)
        .wrapping_add(mb_data_size as uint64_t)
        .wrapping_add(cache_size as uint64_t)
        .wrapping_add(alpha_size)
        .wrapping_add(WEBP_ALIGN_CST as uint64_t);
    let mut mem: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if CheckSizeOverflow(needed) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if needed > (*dec).mem_size_ as uint64_t {
        WebPSafeFree((*dec).mem_);
        (*dec).mem_size_ = 0 as size_t;
        (*dec).mem_ = WebPSafeMalloc(needed, ::core::mem::size_of::<uint8_t>() as size_t);
        if (*dec).mem_.is_null() {
            return VP8SetError(
                dec,
                VP8_STATUS_OUT_OF_MEMORY,
                b"no memory during frame initialization.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        (*dec).mem_size_ = needed as size_t;
    }
    mem = (*dec).mem_ as *mut uint8_t;
    (*dec).intra_t_ = mem;
    mem = mem.offset(intra_pred_mode_size as isize);
    (*dec).yuv_t_ = mem as *mut VP8TopSamples;
    mem = mem.offset(top_size as isize);
    (*dec).mb_info_ = (mem as *mut VP8MB).offset(1 as ::core::ffi::c_int as isize);
    mem = mem.offset(mb_info_size as isize);
    (*dec).f_info_ = if f_info_size != 0 {
        mem as *mut VP8FInfo
    } else {
        ::core::ptr::null_mut::<VP8FInfo>()
    };
    mem = mem.offset(f_info_size as isize);
    (*dec).thread_ctx_.id_ = 0 as ::core::ffi::c_int;
    (*dec).thread_ctx_.f_info_ = (*dec).f_info_;
    if (*dec).filter_type_ > 0 as ::core::ffi::c_int && (*dec).mt_method_ > 0 as ::core::ffi::c_int
    {
        (*dec).thread_ctx_.f_info_ = (*dec).thread_ctx_.f_info_.offset(mb_w as isize);
    }
    mem = ((mem as uintptr_t).wrapping_add(WEBP_ALIGN_CST as uintptr_t)
        & !(WEBP_ALIGN_CST as uintptr_t)) as *mut uint8_t;
    '_c2rust_label: {
        if yuv_size & 31 as size_t == 0 as size_t {
        } else {
            __assert_fail(
                b"(yuv_size & WEBP_ALIGN_CST) == 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/frame_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                743 as ::core::ffi::c_uint,
                b"int AllocateMemory(VP8Decoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*dec).yuv_b_ = mem;
    mem = mem.offset(yuv_size as isize);
    (*dec).mb_data_ = mem as *mut VP8MBData;
    (*dec).thread_ctx_.mb_data_ = mem as *mut VP8MBData;
    if (*dec).mt_method_ == 2 as ::core::ffi::c_int {
        (*dec).thread_ctx_.mb_data_ = (*dec).thread_ctx_.mb_data_.offset(mb_w as isize);
    }
    mem = mem.offset(mb_data_size as isize);
    (*dec).cache_y_stride_ = 16 as ::core::ffi::c_int * mb_w;
    (*dec).cache_uv_stride_ = 8 as ::core::ffi::c_int * mb_w;
    let extra_rows: ::core::ffi::c_int =
        kFilterExtraRows[(*dec).filter_type_ as usize] as ::core::ffi::c_int;
    let extra_y: ::core::ffi::c_int = extra_rows * (*dec).cache_y_stride_;
    let extra_uv: ::core::ffi::c_int =
        extra_rows / 2 as ::core::ffi::c_int * (*dec).cache_uv_stride_;
    (*dec).cache_y_ = mem.offset(extra_y as isize);
    (*dec).cache_u_ = (*dec)
        .cache_y_
        .offset((16 as ::core::ffi::c_int * num_caches * (*dec).cache_y_stride_) as isize)
        .offset(extra_uv as isize);
    (*dec).cache_v_ = (*dec)
        .cache_u_
        .offset((8 as ::core::ffi::c_int * num_caches * (*dec).cache_uv_stride_) as isize)
        .offset(extra_uv as isize);
    (*dec).cache_id_ = 0 as ::core::ffi::c_int;
    mem = mem.offset(cache_size as isize);
    (*dec).alpha_plane_ = if alpha_size != 0 {
        mem
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    mem = mem.offset(alpha_size as isize);
    '_c2rust_label_0: {
        if mem <= ((*dec).mem_ as *mut uint8_t).offset((*dec).mem_size_ as isize) {
        } else {
            __assert_fail(
                b"mem <= (uint8_t*)dec->mem_ + dec->mem_size_\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/frame_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                772 as ::core::ffi::c_uint,
                b"int AllocateMemory(VP8Decoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    memset(
        (*dec).mb_info_.offset(-(1 as ::core::ffi::c_int as isize)) as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        mb_info_size,
    );
    VP8InitScanline(dec);
    memset(
        (*dec).intra_t_ as *mut ::core::ffi::c_void,
        B_DC_PRED as ::core::ffi::c_int,
        intra_pred_mode_size,
    );
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn InitIo(dec: *mut VP8Decoder, mut io: *mut VP8Io) {
    (*io).mb_y = 0 as ::core::ffi::c_int;
    (*io).y = (*dec).cache_y_;
    (*io).u = (*dec).cache_u_;
    (*io).v = (*dec).cache_v_;
    (*io).y_stride = (*dec).cache_y_stride_;
    (*io).uv_stride = (*dec).cache_uv_stride_;
    (*io).a = ::core::ptr::null::<uint8_t>();
}
#[no_mangle]
pub unsafe extern "C" fn VP8InitFrame(dec: *mut VP8Decoder, io: *mut VP8Io) -> ::core::ffi::c_int {
    if InitThreadContext(dec) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if AllocateMemory(dec) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    InitIo(dec, io);
    VP8DspInit();
    return 1 as ::core::ffi::c_int;
}
