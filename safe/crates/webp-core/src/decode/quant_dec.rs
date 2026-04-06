#[repr(C)]
pub struct ALPHDecoder {
    _unused: [u8; 0],
}

extern "C" {
    fn VP8GetValue(br: *mut VP8BitReader, num_bits: ::core::ffi::c_int) -> uint32_t;
    fn VP8GetSignedValue(br: *mut VP8BitReader, num_bits: ::core::ffi::c_int) -> int32_t;
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
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
#[inline]
unsafe extern "C" fn clip(
    mut v: ::core::ffi::c_int,
    mut M: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if v < 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else if v > M {
        M
    } else {
        v
    };
}
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
#[no_mangle]
pub unsafe extern "C" fn VP8ParseQuant(dec: *mut VP8Decoder) {
    let br: *mut VP8BitReader = &raw mut (*dec).br_;
    let base_q0: ::core::ffi::c_int =
        VP8GetValue(br, 7 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let dqy1_dc: ::core::ffi::c_int = if VP8GetValue(br, 1 as ::core::ffi::c_int) != 0 {
        VP8GetSignedValue(br, 4 as ::core::ffi::c_int) as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    let dqy2_dc: ::core::ffi::c_int = if VP8GetValue(br, 1 as ::core::ffi::c_int) != 0 {
        VP8GetSignedValue(br, 4 as ::core::ffi::c_int) as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    let dqy2_ac: ::core::ffi::c_int = if VP8GetValue(br, 1 as ::core::ffi::c_int) != 0 {
        VP8GetSignedValue(br, 4 as ::core::ffi::c_int) as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    let dquv_dc: ::core::ffi::c_int = if VP8GetValue(br, 1 as ::core::ffi::c_int) != 0 {
        VP8GetSignedValue(br, 4 as ::core::ffi::c_int) as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    let dquv_ac: ::core::ffi::c_int = if VP8GetValue(br, 1 as ::core::ffi::c_int) != 0 {
        VP8GetSignedValue(br, 4 as ::core::ffi::c_int) as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    let hdr: *const VP8SegmentHeader = &raw mut (*dec).segment_hdr_;
    let mut i: ::core::ffi::c_int = 0;
    let mut current_block_18: u64;
    i = 0 as ::core::ffi::c_int;
    while i < NUM_MB_SEGMENTS as ::core::ffi::c_int {
        let mut q: ::core::ffi::c_int = 0;
        if (*hdr).use_segment_ != 0 {
            q = (*hdr).quantizer_[i as usize] as ::core::ffi::c_int;
            if (*hdr).absolute_delta_ == 0 {
                q += base_q0;
            }
            current_block_18 = 10048703153582371463;
        } else if i > 0 as ::core::ffi::c_int {
            (*dec).dqm_[i as usize] = (*dec).dqm_[0 as ::core::ffi::c_int as usize];
            current_block_18 = 17179679302217393232;
        } else {
            q = base_q0;
            current_block_18 = 10048703153582371463;
        }
        match current_block_18 {
            10048703153582371463 => {
                let m: *mut VP8QuantMatrix = (&raw mut (*dec).dqm_ as *mut VP8QuantMatrix)
                    .offset(i as isize)
                    as *mut VP8QuantMatrix;
                (*m).y1_mat_[0 as ::core::ffi::c_int as usize] = kDcTable
                    [clip(q + dqy1_dc, 127 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
                (*m).y1_mat_[1 as ::core::ffi::c_int as usize] = kAcTable
                    [clip(q + 0 as ::core::ffi::c_int, 127 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
                (*m).y2_mat_[0 as ::core::ffi::c_int as usize] = kDcTable
                    [clip(q + dqy2_dc, 127 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    * 2 as ::core::ffi::c_int;
                (*m).y2_mat_[1 as ::core::ffi::c_int as usize] = kAcTable
                    [clip(q + dqy2_ac, 127 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    * 101581 as ::core::ffi::c_int
                    >> 16 as ::core::ffi::c_int;
                if (*m).y2_mat_[1 as ::core::ffi::c_int as usize] < 8 as ::core::ffi::c_int {
                    (*m).y2_mat_[1 as ::core::ffi::c_int as usize] = 8 as ::core::ffi::c_int;
                }
                (*m).uv_mat_[0 as ::core::ffi::c_int as usize] = kDcTable
                    [clip(q + dquv_dc, 117 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
                (*m).uv_mat_[1 as ::core::ffi::c_int as usize] = kAcTable
                    [clip(q + dquv_ac, 127 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
                (*m).uv_quant_ = q + dquv_ac;
            }
            _ => {}
        }
        i += 1;
    }
}
