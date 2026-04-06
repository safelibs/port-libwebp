extern "C" {
    fn WebPRescalerInit(
        rescaler: *mut WebPRescaler,
        src_width: ::core::ffi::c_int,
        src_height: ::core::ffi::c_int,
        dst: *mut uint8_t,
        dst_width: ::core::ffi::c_int,
        dst_height: ::core::ffi::c_int,
        dst_stride: ::core::ffi::c_int,
        num_channels: ::core::ffi::c_int,
        work: *mut rescaler_t,
    ) -> ::core::ffi::c_int;
    fn WebPRescaleNeededLines(
        rescaler: *const WebPRescaler,
        max_num_lines: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPRescalerImport(
        rescaler: *mut WebPRescaler,
        num_rows: ::core::ffi::c_int,
        src: *const uint8_t,
        src_stride: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPIoInitFromOptions(
        options: *const WebPDecoderOptions,
        io: *mut VP8Io,
        src_colorspace: WEBP_CSP_MODE,
    ) -> ::core::ffi::c_int;
    static mut WebPConvertARGBToY:
        Option<unsafe extern "C" fn(*const uint32_t, *mut uint8_t, ::core::ffi::c_int) -> ()>;
    static mut WebPConvertARGBToUV: Option<
        unsafe extern "C" fn(
            *const uint32_t,
            *mut uint8_t,
            *mut uint8_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >;
    fn WebPInitConvertARGBToYUV();
    fn WebPRescalerExportRow(wrk: *mut WebPRescaler);
    static mut WebPExtractAlpha: Option<
        unsafe extern "C" fn(
            *const uint8_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut uint8_t,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >;
    static mut WebPExtractGreen:
        Option<unsafe extern "C" fn(*const uint32_t, *mut uint8_t, ::core::ffi::c_int) -> ()>;
    static mut WebPMultARGBRow:
        Option<unsafe extern "C" fn(*mut uint32_t, ::core::ffi::c_int, ::core::ffi::c_int) -> ()>;
    fn WebPMultARGBRows(
        ptr: *mut uint8_t,
        stride: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        num_rows: ::core::ffi::c_int,
        inverse: ::core::ffi::c_int,
    );
    fn WebPInitAlphaProcessing();
    static mut WebPUnfilters: [WebPUnfilterFunc; 4];
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
    fn VP8LInitBitReader(br: *mut VP8LBitReader, start: *const uint8_t, length: size_t);
    fn VP8LReadBits(br: *mut VP8LBitReader, n_bits: ::core::ffi::c_int) -> uint32_t;
    fn VP8LDoFillBitWindow(br: *mut VP8LBitReader);
    fn VP8LColorCacheInit(
        color_cache: *mut VP8LColorCache,
        hash_bits: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VP8LColorCacheCopy(src: *const VP8LColorCache, dst: *mut VP8LColorCache);
    fn VP8LColorCacheClear(color_cache: *mut VP8LColorCache);
    fn VP8LHuffmanTablesAllocate(
        size: ::core::ffi::c_int,
        huffman_tables: *mut HuffmanTables,
    ) -> ::core::ffi::c_int;
    fn VP8LHuffmanTablesDeallocate(huffman_tables: *mut HuffmanTables);
    fn VP8LHtreeGroupsNew(num_htree_groups: ::core::ffi::c_int) -> *mut HTreeGroup;
    fn VP8LHtreeGroupsFree(htree_groups: *mut HTreeGroup);
    fn VP8LBuildHuffmanTable(
        root_table: *mut HuffmanTables,
        root_bits: ::core::ffi::c_int,
        code_lengths: *const ::core::ffi::c_int,
        code_lengths_size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeCalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
    fn VP8LInverseTransform(
        transform: *const VP8LTransform,
        row_start: ::core::ffi::c_int,
        row_end: ::core::ffi::c_int,
        in_0: *const uint32_t,
        out: *mut uint32_t,
    );
    fn VP8LConvertFromBGRA(
        in_data: *const uint32_t,
        num_pixels: ::core::ffi::c_int,
        out_colorspace: WEBP_CSP_MODE,
        rgba: *mut uint8_t,
    );
    fn VP8LColorIndexInverseTransformAlpha(
        transform: *const VP8LTransform,
        y_start: ::core::ffi::c_int,
        y_end: ::core::ffi::c_int,
        src: *const uint8_t,
        dst: *mut uint8_t,
    );
    fn VP8LDspInit();
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int64_t = __int64_t;
pub type ptrdiff_t = isize;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = usize;
pub type rescaler_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPRescaler {
    pub x_expand: ::core::ffi::c_int,
    pub y_expand: ::core::ffi::c_int,
    pub num_channels: ::core::ffi::c_int,
    pub fx_scale: uint32_t,
    pub fy_scale: uint32_t,
    pub fxy_scale: uint32_t,
    pub y_accum: ::core::ffi::c_int,
    pub y_add: ::core::ffi::c_int,
    pub y_sub: ::core::ffi::c_int,
    pub x_add: ::core::ffi::c_int,
    pub x_sub: ::core::ffi::c_int,
    pub src_width: ::core::ffi::c_int,
    pub src_height: ::core::ffi::c_int,
    pub dst_width: ::core::ffi::c_int,
    pub dst_height: ::core::ffi::c_int,
    pub src_y: ::core::ffi::c_int,
    pub dst_y: ::core::ffi::c_int,
    pub dst: *mut uint8_t,
    pub dst_stride: ::core::ffi::c_int,
    pub irow: *mut rescaler_t,
    pub frow: *mut rescaler_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPRGBABuffer {
    pub rgba: *mut uint8_t,
    pub stride: ::core::ffi::c_int,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPYUVABuffer {
    pub y: *mut uint8_t,
    pub u: *mut uint8_t,
    pub v: *mut uint8_t,
    pub a: *mut uint8_t,
    pub y_stride: ::core::ffi::c_int,
    pub u_stride: ::core::ffi::c_int,
    pub v_stride: ::core::ffi::c_int,
    pub a_stride: ::core::ffi::c_int,
    pub y_size: size_t,
    pub u_size: size_t,
    pub v_size: size_t,
    pub a_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPDecBuffer {
    pub colorspace: WEBP_CSP_MODE,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub is_external_memory: ::core::ffi::c_int,
    pub u: C2RustUnnamed,
    pub pad: [uint32_t; 4],
    pub private_memory: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub RGBA: WebPRGBABuffer,
    pub YUVA: WebPYUVABuffer,
}
pub type WEBP_CSP_MODE = ::core::ffi::c_uint;
pub const MODE_LAST: WEBP_CSP_MODE = 13;
pub const MODE_YUVA: WEBP_CSP_MODE = 12;
pub const MODE_YUV: WEBP_CSP_MODE = 11;
pub const MODE_rgbA_4444: WEBP_CSP_MODE = 10;
pub const MODE_Argb: WEBP_CSP_MODE = 9;
pub const MODE_bgrA: WEBP_CSP_MODE = 8;
pub const MODE_rgbA: WEBP_CSP_MODE = 7;
pub const MODE_RGB_565: WEBP_CSP_MODE = 6;
pub const MODE_RGBA_4444: WEBP_CSP_MODE = 5;
pub const MODE_ARGB: WEBP_CSP_MODE = 4;
pub const MODE_BGRA: WEBP_CSP_MODE = 3;
pub const MODE_BGR: WEBP_CSP_MODE = 2;
pub const MODE_RGBA: WEBP_CSP_MODE = 1;
pub const MODE_RGB: WEBP_CSP_MODE = 0;
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
pub struct VP8LBitReader {
    pub val_: vp8l_val_t,
    pub buf_: *const uint8_t,
    pub len_: size_t,
    pub pos_: size_t,
    pub bit_pos_: ::core::ffi::c_int,
    pub eos_: ::core::ffi::c_int,
}
pub type vp8l_val_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPDecParams {
    pub output: *mut WebPDecBuffer,
    pub tmp_y: *mut uint8_t,
    pub tmp_u: *mut uint8_t,
    pub tmp_v: *mut uint8_t,
    pub last_y: ::core::ffi::c_int,
    pub options: *const WebPDecoderOptions,
    pub scaler_y: *mut WebPRescaler,
    pub scaler_u: *mut WebPRescaler,
    pub scaler_v: *mut WebPRescaler,
    pub scaler_a: *mut WebPRescaler,
    pub memory: *mut ::core::ffi::c_void,
    pub emit: OutputFunc,
    pub emit_alpha: OutputAlphaFunc,
    pub emit_alpha_row: OutputRowFunc,
}
pub type OutputRowFunc = Option<
    unsafe extern "C" fn(
        *mut WebPDecParams,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type OutputAlphaFunc = Option<
    unsafe extern "C" fn(
        *const VP8Io,
        *mut WebPDecParams,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type OutputFunc =
    Option<unsafe extern "C" fn(*const VP8Io, *mut WebPDecParams) -> ::core::ffi::c_int>;
pub type WEBP_FILTER_TYPE = ::core::ffi::c_uint;
pub const WEBP_FILTER_FAST: WEBP_FILTER_TYPE = 6;
pub const WEBP_FILTER_BEST: WEBP_FILTER_TYPE = 5;
pub const WEBP_FILTER_LAST: WEBP_FILTER_TYPE = 4;
pub const WEBP_FILTER_GRADIENT: WEBP_FILTER_TYPE = 3;
pub const WEBP_FILTER_VERTICAL: WEBP_FILTER_TYPE = 2;
pub const WEBP_FILTER_HORIZONTAL: WEBP_FILTER_TYPE = 1;
pub const WEBP_FILTER_NONE: WEBP_FILTER_TYPE = 0;
pub type WebPUnfilterFunc = Option<
    unsafe extern "C" fn(*const uint8_t, *const uint8_t, *mut uint8_t, ::core::ffi::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LDecoder {
    pub status_: VP8StatusCode,
    pub state_: VP8LDecodeState,
    pub io_: *mut VP8Io,
    pub output_: *const WebPDecBuffer,
    pub pixels_: *mut uint32_t,
    pub argb_cache_: *mut uint32_t,
    pub br_: VP8LBitReader,
    pub incremental_: ::core::ffi::c_int,
    pub saved_br_: VP8LBitReader,
    pub saved_last_pixel_: ::core::ffi::c_int,
    pub width_: ::core::ffi::c_int,
    pub height_: ::core::ffi::c_int,
    pub last_row_: ::core::ffi::c_int,
    pub last_pixel_: ::core::ffi::c_int,
    pub last_out_row_: ::core::ffi::c_int,
    pub hdr_: VP8LMetadata,
    pub next_transform_: ::core::ffi::c_int,
    pub transforms_: [VP8LTransform; 4],
    pub transforms_seen_: uint32_t,
    pub rescaler_memory: *mut uint8_t,
    pub rescaler: *mut WebPRescaler,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LTransform {
    pub type_: VP8LImageTransformType,
    pub bits_: ::core::ffi::c_int,
    pub xsize_: ::core::ffi::c_int,
    pub ysize_: ::core::ffi::c_int,
    pub data_: *mut uint32_t,
}
pub type VP8LImageTransformType = ::core::ffi::c_uint;
pub const COLOR_INDEXING_TRANSFORM: VP8LImageTransformType = 3;
pub const SUBTRACT_GREEN_TRANSFORM: VP8LImageTransformType = 2;
pub const CROSS_COLOR_TRANSFORM: VP8LImageTransformType = 1;
pub const PREDICTOR_TRANSFORM: VP8LImageTransformType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LMetadata {
    pub color_cache_size_: ::core::ffi::c_int,
    pub color_cache_: VP8LColorCache,
    pub saved_color_cache_: VP8LColorCache,
    pub huffman_mask_: ::core::ffi::c_int,
    pub huffman_subsample_bits_: ::core::ffi::c_int,
    pub huffman_xsize_: ::core::ffi::c_int,
    pub huffman_image_: *mut uint32_t,
    pub num_htree_groups_: ::core::ffi::c_int,
    pub htree_groups_: *mut HTreeGroup,
    pub huffman_tables_: HuffmanTables,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTables {
    pub root: HuffmanTablesSegment,
    pub curr_segment: *mut HuffmanTablesSegment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTablesSegment {
    pub start: *mut HuffmanCode,
    pub curr_table: *mut HuffmanCode,
    pub next: *mut HuffmanTablesSegment,
    pub size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanCode {
    pub bits: uint8_t,
    pub value: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HTreeGroup {
    pub htrees: [*mut HuffmanCode; 5],
    pub is_trivial_literal: ::core::ffi::c_int,
    pub literal_arb: uint32_t,
    pub is_trivial_code: ::core::ffi::c_int,
    pub use_packed_table: ::core::ffi::c_int,
    pub packed_table: [HuffmanCode32; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanCode32 {
    pub bits: ::core::ffi::c_int,
    pub value: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LColorCache {
    pub colors_: *mut uint32_t,
    pub hash_shift_: ::core::ffi::c_int,
    pub hash_bits_: ::core::ffi::c_int,
}
pub type VP8LDecodeState = ::core::ffi::c_uint;
pub const READ_DIM: VP8LDecodeState = 2;
pub const READ_HDR: VP8LDecodeState = 1;
pub const READ_DATA: VP8LDecodeState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ALPHDecoder {
    pub width_: ::core::ffi::c_int,
    pub height_: ::core::ffi::c_int,
    pub method_: ::core::ffi::c_int,
    pub filter_: WEBP_FILTER_TYPE,
    pub pre_processing_: ::core::ffi::c_int,
    pub vp8l_dec_: *mut VP8LDecoder,
    pub io_: VP8Io,
    pub use_8b_decode_: ::core::ffi::c_int,
    pub output_: *mut uint8_t,
    pub prev_line_: *const uint8_t,
}
pub const ALPHA: C2RustUnnamed_0 = 3;
pub const BLUE: C2RustUnnamed_0 = 2;
pub const RED: C2RustUnnamed_0 = 1;
pub type ProcessRowsFunc = Option<unsafe extern "C" fn(*mut VP8LDecoder, ::core::ffi::c_int) -> ()>;
pub const DIST: C2RustUnnamed_0 = 4;
pub const GREEN: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn WebPRescalerOutputDone(rescaler: *const WebPRescaler) -> ::core::ffi::c_int {
    return ((*rescaler).dst_y >= (*rescaler).dst_height) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn WebPRescalerHasPendingOutput(
    rescaler: *const WebPRescaler,
) -> ::core::ffi::c_int {
    return (WebPRescalerOutputDone(rescaler) == 0 && (*rescaler).y_accum <= 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn WebPIsPremultipliedMode(mut mode: WEBP_CSP_MODE) -> ::core::ffi::c_int {
    return (mode as ::core::ffi::c_uint == MODE_rgbA as ::core::ffi::c_int as ::core::ffi::c_uint
        || mode as ::core::ffi::c_uint == MODE_bgrA as ::core::ffi::c_int as ::core::ffi::c_uint
        || mode as ::core::ffi::c_uint == MODE_Argb as ::core::ffi::c_int as ::core::ffi::c_uint
        || mode as ::core::ffi::c_uint
            == MODE_rgbA_4444 as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn WebPIsRGBMode(mut mode: WEBP_CSP_MODE) -> ::core::ffi::c_int {
    return ((mode as ::core::ffi::c_uint) < MODE_YUV as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 50] = unsafe {
    ::core::mem::transmute::<[u8; 50], [::core::ffi::c_char; 50]>(
        *b"int VP8LIsEndOfStream(const VP8LBitReader *const)\0",
    )
};
pub const VP8L_LBITS: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const VP8L_WBITS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn VP8LPrefetchBits(br: *mut VP8LBitReader) -> uint32_t {
    return ((*br).val_ >> ((*br).bit_pos_ & VP8L_LBITS - 1 as ::core::ffi::c_int)) as uint32_t;
}
#[inline]
unsafe extern "C" fn VP8LIsEndOfStream(br: *const VP8LBitReader) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if (*br).pos_ <= (*br).len_ {
        } else {
            __assert_fail(
                b"br->pos_ <= br->len_\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/utils/bit_reader_utils.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                172 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    return ((*br).eos_ != 0 || (*br).pos_ == (*br).len_ && (*br).bit_pos_ > VP8L_LBITS)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn VP8LSetBitPos(br: *mut VP8LBitReader, mut val: ::core::ffi::c_int) {
    (*br).bit_pos_ = val;
}
#[inline]
unsafe extern "C" fn VP8LFillBitWindow(br: *mut VP8LBitReader) {
    if (*br).bit_pos_ >= VP8L_WBITS {
        VP8LDoFillBitWindow(br);
    }
}
static mut kHashMul: uint32_t = 0x1e35a7bd as uint32_t;
#[inline]
unsafe extern "C" fn VP8LHashPix(
    mut argb: uint32_t,
    mut shift: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (argb.wrapping_mul(kHashMul) >> shift) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn VP8LColorCacheLookup(
    cc: *const VP8LColorCache,
    mut key: uint32_t,
) -> uint32_t {
    '_c2rust_label: {
        if key >> (*cc).hash_bits_ == 0 as uint32_t {
        } else {
            __assert_fail(
                b"(key >> cc->hash_bits_) == 0u\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/utils/color_cache_utils.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                43 as ::core::ffi::c_uint,
                b"uint32_t VP8LColorCacheLookup(const VP8LColorCache *const, uint32_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    return *(*cc).colors_.offset(key as isize);
}
#[inline]
unsafe extern "C" fn VP8LColorCacheInsert(cc: *const VP8LColorCache, mut argb: uint32_t) {
    let key: ::core::ffi::c_int = VP8LHashPix(argb, (*cc).hash_shift_) as ::core::ffi::c_int;
    *(*cc).colors_.offset(key as isize) = argb;
}
pub const VP8L_MAGIC_BYTE: ::core::ffi::c_int = 0x2f as ::core::ffi::c_int;
pub const VP8L_IMAGE_SIZE_BITS: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const VP8L_VERSION_BITS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const VP8L_FRAME_HEADER_SIZE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MAX_CACHE_BITS: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const HUFFMAN_CODES_PER_META_CODE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const DEFAULT_CODE_LENGTH: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const NUM_LITERAL_CODES: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const NUM_LENGTH_CODES: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const NUM_DISTANCE_CODES: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const HUFFMAN_TABLE_BITS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const HUFFMAN_TABLE_MASK: ::core::ffi::c_int =
    ((1 as ::core::ffi::c_int) << HUFFMAN_TABLE_BITS) - 1 as ::core::ffi::c_int;
pub const LENGTHS_TABLE_BITS: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const LENGTHS_TABLE_MASK: ::core::ffi::c_int =
    ((1 as ::core::ffi::c_int) << LENGTHS_TABLE_BITS) - 1 as ::core::ffi::c_int;
pub const HUFFMAN_PACKED_BITS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const HUFFMAN_PACKED_TABLE_SIZE: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << HUFFMAN_PACKED_BITS;
#[inline]
unsafe extern "C" fn VP8LSubSampleSize(
    mut size: uint32_t,
    mut sampling_bits: uint32_t,
) -> uint32_t {
    return size
        .wrapping_add(((1 as ::core::ffi::c_int) << sampling_bits) as uint32_t)
        .wrapping_sub(1 as uint32_t)
        >> sampling_bits;
}
pub const NUM_ARGB_CACHE_ROWS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
static mut kCodeLengthLiterals: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
static mut kCodeLengthRepeatCode: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
static mut kCodeLengthExtraBits: [uint8_t; 3] = [
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
];
static mut kCodeLengthRepeatOffsets: [uint8_t; 3] = [
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
];
static mut kAlphabetSize: [uint16_t; 5] = [
    (NUM_LITERAL_CODES + NUM_LENGTH_CODES) as uint16_t,
    NUM_LITERAL_CODES as uint16_t,
    NUM_LITERAL_CODES as uint16_t,
    NUM_LITERAL_CODES as uint16_t,
    NUM_DISTANCE_CODES as uint16_t,
];
static mut kLiteralMap: [uint8_t; 5] = [
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
pub const NUM_CODE_LENGTH_CODES: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
static mut kCodeLengthCodeOrder: [uint8_t; 19] = [
    17 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
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
];
pub const CODE_TO_PLANE_CODES: ::core::ffi::c_int = 120 as ::core::ffi::c_int;
static mut kCodeToPlane: [uint8_t; 120] = [
    0x18 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0x17 as ::core::ffi::c_int as uint8_t,
    0x19 as ::core::ffi::c_int as uint8_t,
    0x28 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x27 as ::core::ffi::c_int as uint8_t,
    0x29 as ::core::ffi::c_int as uint8_t,
    0x16 as ::core::ffi::c_int as uint8_t,
    0x1a as ::core::ffi::c_int as uint8_t,
    0x26 as ::core::ffi::c_int as uint8_t,
    0x2a as ::core::ffi::c_int as uint8_t,
    0x38 as ::core::ffi::c_int as uint8_t,
    0x5 as ::core::ffi::c_int as uint8_t,
    0x37 as ::core::ffi::c_int as uint8_t,
    0x39 as ::core::ffi::c_int as uint8_t,
    0x15 as ::core::ffi::c_int as uint8_t,
    0x1b as ::core::ffi::c_int as uint8_t,
    0x36 as ::core::ffi::c_int as uint8_t,
    0x3a as ::core::ffi::c_int as uint8_t,
    0x25 as ::core::ffi::c_int as uint8_t,
    0x2b as ::core::ffi::c_int as uint8_t,
    0x48 as ::core::ffi::c_int as uint8_t,
    0x4 as ::core::ffi::c_int as uint8_t,
    0x47 as ::core::ffi::c_int as uint8_t,
    0x49 as ::core::ffi::c_int as uint8_t,
    0x14 as ::core::ffi::c_int as uint8_t,
    0x1c as ::core::ffi::c_int as uint8_t,
    0x35 as ::core::ffi::c_int as uint8_t,
    0x3b as ::core::ffi::c_int as uint8_t,
    0x46 as ::core::ffi::c_int as uint8_t,
    0x4a as ::core::ffi::c_int as uint8_t,
    0x24 as ::core::ffi::c_int as uint8_t,
    0x2c as ::core::ffi::c_int as uint8_t,
    0x58 as ::core::ffi::c_int as uint8_t,
    0x45 as ::core::ffi::c_int as uint8_t,
    0x4b as ::core::ffi::c_int as uint8_t,
    0x34 as ::core::ffi::c_int as uint8_t,
    0x3c as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0x57 as ::core::ffi::c_int as uint8_t,
    0x59 as ::core::ffi::c_int as uint8_t,
    0x13 as ::core::ffi::c_int as uint8_t,
    0x1d as ::core::ffi::c_int as uint8_t,
    0x56 as ::core::ffi::c_int as uint8_t,
    0x5a as ::core::ffi::c_int as uint8_t,
    0x23 as ::core::ffi::c_int as uint8_t,
    0x2d as ::core::ffi::c_int as uint8_t,
    0x44 as ::core::ffi::c_int as uint8_t,
    0x4c as ::core::ffi::c_int as uint8_t,
    0x55 as ::core::ffi::c_int as uint8_t,
    0x5b as ::core::ffi::c_int as uint8_t,
    0x33 as ::core::ffi::c_int as uint8_t,
    0x3d as ::core::ffi::c_int as uint8_t,
    0x68 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x67 as ::core::ffi::c_int as uint8_t,
    0x69 as ::core::ffi::c_int as uint8_t,
    0x12 as ::core::ffi::c_int as uint8_t,
    0x1e as ::core::ffi::c_int as uint8_t,
    0x66 as ::core::ffi::c_int as uint8_t,
    0x6a as ::core::ffi::c_int as uint8_t,
    0x22 as ::core::ffi::c_int as uint8_t,
    0x2e as ::core::ffi::c_int as uint8_t,
    0x54 as ::core::ffi::c_int as uint8_t,
    0x5c as ::core::ffi::c_int as uint8_t,
    0x43 as ::core::ffi::c_int as uint8_t,
    0x4d as ::core::ffi::c_int as uint8_t,
    0x65 as ::core::ffi::c_int as uint8_t,
    0x6b as ::core::ffi::c_int as uint8_t,
    0x32 as ::core::ffi::c_int as uint8_t,
    0x3e as ::core::ffi::c_int as uint8_t,
    0x78 as ::core::ffi::c_int as uint8_t,
    0x1 as ::core::ffi::c_int as uint8_t,
    0x77 as ::core::ffi::c_int as uint8_t,
    0x79 as ::core::ffi::c_int as uint8_t,
    0x53 as ::core::ffi::c_int as uint8_t,
    0x5d as ::core::ffi::c_int as uint8_t,
    0x11 as ::core::ffi::c_int as uint8_t,
    0x1f as ::core::ffi::c_int as uint8_t,
    0x64 as ::core::ffi::c_int as uint8_t,
    0x6c as ::core::ffi::c_int as uint8_t,
    0x42 as ::core::ffi::c_int as uint8_t,
    0x4e as ::core::ffi::c_int as uint8_t,
    0x76 as ::core::ffi::c_int as uint8_t,
    0x7a as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
    0x2f as ::core::ffi::c_int as uint8_t,
    0x75 as ::core::ffi::c_int as uint8_t,
    0x7b as ::core::ffi::c_int as uint8_t,
    0x31 as ::core::ffi::c_int as uint8_t,
    0x3f as ::core::ffi::c_int as uint8_t,
    0x63 as ::core::ffi::c_int as uint8_t,
    0x6d as ::core::ffi::c_int as uint8_t,
    0x52 as ::core::ffi::c_int as uint8_t,
    0x5e as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0x74 as ::core::ffi::c_int as uint8_t,
    0x7c as ::core::ffi::c_int as uint8_t,
    0x41 as ::core::ffi::c_int as uint8_t,
    0x4f as ::core::ffi::c_int as uint8_t,
    0x10 as ::core::ffi::c_int as uint8_t,
    0x20 as ::core::ffi::c_int as uint8_t,
    0x62 as ::core::ffi::c_int as uint8_t,
    0x6e as ::core::ffi::c_int as uint8_t,
    0x30 as ::core::ffi::c_int as uint8_t,
    0x73 as ::core::ffi::c_int as uint8_t,
    0x7d as ::core::ffi::c_int as uint8_t,
    0x51 as ::core::ffi::c_int as uint8_t,
    0x5f as ::core::ffi::c_int as uint8_t,
    0x40 as ::core::ffi::c_int as uint8_t,
    0x72 as ::core::ffi::c_int as uint8_t,
    0x7e as ::core::ffi::c_int as uint8_t,
    0x61 as ::core::ffi::c_int as uint8_t,
    0x6f as ::core::ffi::c_int as uint8_t,
    0x50 as ::core::ffi::c_int as uint8_t,
    0x71 as ::core::ffi::c_int as uint8_t,
    0x7f as ::core::ffi::c_int as uint8_t,
    0x60 as ::core::ffi::c_int as uint8_t,
    0x70 as ::core::ffi::c_int as uint8_t,
];
pub const FIXED_TABLE_SIZE: ::core::ffi::c_int =
    630 as ::core::ffi::c_int * 3 as ::core::ffi::c_int + 410 as ::core::ffi::c_int;
static mut kTableSize: [uint16_t; 12] = [
    (FIXED_TABLE_SIZE + 654 as ::core::ffi::c_int) as uint16_t,
    (FIXED_TABLE_SIZE + 656 as ::core::ffi::c_int) as uint16_t,
    (FIXED_TABLE_SIZE + 658 as ::core::ffi::c_int) as uint16_t,
    (FIXED_TABLE_SIZE + 662 as ::core::ffi::c_int) as uint16_t,
    (FIXED_TABLE_SIZE + 670 as ::core::ffi::c_int) as uint16_t,
    (FIXED_TABLE_SIZE + 686 as ::core::ffi::c_int) as uint16_t,
    (FIXED_TABLE_SIZE + 718 as ::core::ffi::c_int) as uint16_t,
    (FIXED_TABLE_SIZE + 782 as ::core::ffi::c_int) as uint16_t,
    (FIXED_TABLE_SIZE + 912 as ::core::ffi::c_int) as uint16_t,
    (FIXED_TABLE_SIZE + 1168 as ::core::ffi::c_int) as uint16_t,
    (FIXED_TABLE_SIZE + 1680 as ::core::ffi::c_int) as uint16_t,
    (FIXED_TABLE_SIZE + 2704 as ::core::ffi::c_int) as uint16_t,
];
const MAX_HTREE_GROUPS: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;

#[inline]
fn checked_pixel_count(
    xsize: ::core::ffi::c_int,
    ysize: ::core::ffi::c_int,
) -> Option<::core::ffi::c_int> {
    if xsize <= 0 || ysize <= 0 {
        return None;
    }
    xsize.checked_mul(ysize)
}

pub fn checked_huffman_table_allocation_size(
    num_htree_groups: ::core::ffi::c_int,
    table_size: ::core::ffi::c_int,
) -> Option<::core::ffi::c_int> {
    if num_htree_groups <= 0 || table_size <= 0 {
        return None;
    }
    num_htree_groups.checked_mul(table_size)
}

pub fn compact_huffman_group_indices(
    groups: &mut [uint32_t],
) -> Option<(
    crate::rust_alloc::vec::Vec<::core::ffi::c_int>,
    ::core::ffi::c_int,
)> {
    let max_group_plus_one = groups.iter().try_fold(1usize, |max_group, &group| {
        let group = usize::try_from(group).ok()?;
        let next = group.checked_add(1)?;
        Some(max_group.max(next))
    })?;
    if max_group_plus_one > MAX_HTREE_GROUPS as usize {
        return None;
    }

    let mut mapping = crate::rust_alloc::vec::Vec::new();
    mapping.try_reserve_exact(max_group_plus_one).ok()?;
    mapping.resize(max_group_plus_one, -(1 as ::core::ffi::c_int));

    let mut unique_groups = 0 as ::core::ffi::c_int;
    for group in groups {
        let idx = usize::try_from(*group).ok()?;
        let entry = &mut mapping[idx];
        if *entry == -(1 as ::core::ffi::c_int) {
            *entry = unique_groups;
            unique_groups = unique_groups.checked_add(1)?;
        }
        *group = *entry as uint32_t;
    }
    Some((mapping, unique_groups))
}

#[no_mangle]
pub unsafe extern "C" fn VP8LCheckSignature(
    data: *const uint8_t,
    mut size: size_t,
) -> ::core::ffi::c_int {
    return (size >= VP8L_FRAME_HEADER_SIZE as size_t
        && *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == VP8L_MAGIC_BYTE
        && *data.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >> 5 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn ReadImageInfo(
    br: *mut VP8LBitReader,
    width: *mut ::core::ffi::c_int,
    height: *mut ::core::ffi::c_int,
    has_alpha: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if VP8LReadBits(br, 8 as ::core::ffi::c_int) != VP8L_MAGIC_BYTE as uint32_t {
        return 0 as ::core::ffi::c_int;
    }
    *width =
        VP8LReadBits(br, VP8L_IMAGE_SIZE_BITS).wrapping_add(1 as uint32_t) as ::core::ffi::c_int;
    *height =
        VP8LReadBits(br, VP8L_IMAGE_SIZE_BITS).wrapping_add(1 as uint32_t) as ::core::ffi::c_int;
    *has_alpha = VP8LReadBits(br, 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if VP8LReadBits(br, VP8L_VERSION_BITS) != 0 as uint32_t {
        return 0 as ::core::ffi::c_int;
    }
    return ((*br).eos_ == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LGetInfo(
    mut data: *const uint8_t,
    mut data_size: size_t,
    width: *mut ::core::ffi::c_int,
    height: *mut ::core::ffi::c_int,
    has_alpha: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if data.is_null() || data_size < VP8L_FRAME_HEADER_SIZE as size_t {
        return 0 as ::core::ffi::c_int;
    } else if VP8LCheckSignature(data, data_size) == 0 {
        return 0 as ::core::ffi::c_int;
    } else {
        let mut w: ::core::ffi::c_int = 0;
        let mut h: ::core::ffi::c_int = 0;
        let mut a: ::core::ffi::c_int = 0;
        let mut br: VP8LBitReader = VP8LBitReader {
            val_: 0,
            buf_: ::core::ptr::null::<uint8_t>(),
            len_: 0,
            pos_: 0,
            bit_pos_: 0,
            eos_: 0,
        };
        VP8LInitBitReader(&raw mut br, data, data_size);
        if ReadImageInfo(&raw mut br, &raw mut w, &raw mut h, &raw mut a) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        if !width.is_null() {
            *width = w;
        }
        if !height.is_null() {
            *height = h;
        }
        if !has_alpha.is_null() {
            *has_alpha = a;
        }
        return 1 as ::core::ffi::c_int;
    };
}
#[inline]
unsafe extern "C" fn GetCopyDistance(
    mut distance_symbol: ::core::ffi::c_int,
    br: *mut VP8LBitReader,
) -> ::core::ffi::c_int {
    let mut extra_bits: ::core::ffi::c_int = 0;
    let mut offset: ::core::ffi::c_int = 0;
    if distance_symbol < 4 as ::core::ffi::c_int {
        return distance_symbol + 1 as ::core::ffi::c_int;
    }
    extra_bits = distance_symbol - 2 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
    offset = 2 as ::core::ffi::c_int + (distance_symbol & 1 as ::core::ffi::c_int) << extra_bits;
    return (offset as uint32_t)
        .wrapping_add(VP8LReadBits(br, extra_bits))
        .wrapping_add(1 as uint32_t) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn GetCopyLength(
    mut length_symbol: ::core::ffi::c_int,
    br: *mut VP8LBitReader,
) -> ::core::ffi::c_int {
    return GetCopyDistance(length_symbol, br);
}
#[inline]
unsafe extern "C" fn PlaneCodeToDistance(
    mut xsize: ::core::ffi::c_int,
    mut plane_code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if plane_code > CODE_TO_PLANE_CODES {
        return plane_code - CODE_TO_PLANE_CODES;
    } else {
        let dist_code: ::core::ffi::c_int =
            kCodeToPlane[(plane_code - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
        let yoffset: ::core::ffi::c_int = dist_code >> 4 as ::core::ffi::c_int;
        let xoffset: ::core::ffi::c_int =
            8 as ::core::ffi::c_int - (dist_code & 0xf as ::core::ffi::c_int);
        let dist: ::core::ffi::c_int = yoffset * xsize + xoffset;
        return if dist >= 1 as ::core::ffi::c_int {
            dist
        } else {
            1 as ::core::ffi::c_int
        };
    };
}
#[inline]
unsafe extern "C" fn ReadSymbol(
    mut table: *const HuffmanCode,
    br: *mut VP8LBitReader,
) -> ::core::ffi::c_int {
    let mut nbits: ::core::ffi::c_int = 0;
    let mut val: uint32_t = VP8LPrefetchBits(br);
    table = table.offset((val & HUFFMAN_TABLE_MASK as uint32_t) as isize);
    nbits = (*table).bits as ::core::ffi::c_int - HUFFMAN_TABLE_BITS;
    if nbits > 0 as ::core::ffi::c_int {
        VP8LSetBitPos(br, (*br).bit_pos_ + HUFFMAN_TABLE_BITS);
        val = VP8LPrefetchBits(br);
        table = table.offset((*table).value as ::core::ffi::c_int as isize);
        table = table.offset(
            (val & (((1 as ::core::ffi::c_int) << nbits) - 1 as ::core::ffi::c_int) as uint32_t)
                as isize,
        );
    }
    VP8LSetBitPos(br, (*br).bit_pos_ + (*table).bits as ::core::ffi::c_int);
    return (*table).value as ::core::ffi::c_int;
}
pub const BITS_SPECIAL_MARKER: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const PACKED_NON_LITERAL_CODE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn ReadPackedSymbols(
    mut group: *const HTreeGroup,
    br: *mut VP8LBitReader,
    dst: *mut uint32_t,
) -> ::core::ffi::c_int {
    let val: uint32_t = VP8LPrefetchBits(br) as uint32_t
        & (HUFFMAN_PACKED_TABLE_SIZE as uint32_t).wrapping_sub(1 as uint32_t);
    let code: HuffmanCode32 = (*group).packed_table[val as usize];
    '_c2rust_label: {
        if (*group).use_packed_table != 0 {
        } else {
            __assert_fail(
                b"group->use_packed_table\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                207 as ::core::ffi::c_uint,
                b"int ReadPackedSymbols(const HTreeGroup *, VP8LBitReader *const, uint32_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if code.bits < BITS_SPECIAL_MARKER {
        VP8LSetBitPos(br, (*br).bit_pos_ + code.bits);
        *dst = code.value;
        return PACKED_NON_LITERAL_CODE;
    } else {
        VP8LSetBitPos(br, (*br).bit_pos_ + code.bits - BITS_SPECIAL_MARKER);
        '_c2rust_label_0: {
            if code.value >= 256 as uint32_t {
            } else {
                __assert_fail(
                    b"code.value >= NUM_LITERAL_CODES\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    214 as ::core::ffi::c_uint,
                    b"int ReadPackedSymbols(const HTreeGroup *, VP8LBitReader *const, uint32_t *const)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        return code.value as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn AccumulateHCode(
    mut hcode: HuffmanCode,
    mut shift: ::core::ffi::c_int,
    huff: *mut HuffmanCode32,
) -> ::core::ffi::c_int {
    (*huff).bits += hcode.bits as ::core::ffi::c_int;
    (*huff).value |= (hcode.value as uint32_t) << shift;
    '_c2rust_label: {
        if (*huff).bits <= 8 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"huff->bits <= HUFFMAN_TABLE_BITS\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                223 as ::core::ffi::c_uint,
                b"int AccumulateHCode(HuffmanCode, int, HuffmanCode32 *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return hcode.bits as ::core::ffi::c_int;
}
unsafe extern "C" fn BuildPackedTable(htree_group: *mut HTreeGroup) {
    let mut code: uint32_t = 0;
    code = 0 as uint32_t;
    while code < HUFFMAN_PACKED_TABLE_SIZE as uint32_t {
        let mut bits: uint32_t = code;
        let huff: *mut HuffmanCode32 = (&raw mut (*htree_group).packed_table as *mut HuffmanCode32)
            .offset(bits as isize) as *mut HuffmanCode32;
        let mut hcode: HuffmanCode =
            *(*htree_group).htrees[GREEN as ::core::ffi::c_int as usize].offset(bits as isize);
        if hcode.value as ::core::ffi::c_int >= NUM_LITERAL_CODES {
            (*huff).bits = hcode.bits as ::core::ffi::c_int + BITS_SPECIAL_MARKER;
            (*huff).value = hcode.value as uint32_t;
        } else {
            (*huff).bits = 0 as ::core::ffi::c_int;
            (*huff).value = 0 as uint32_t;
            bits >>= AccumulateHCode(hcode, 8 as ::core::ffi::c_int, huff);
            bits >>= AccumulateHCode(
                *(*htree_group).htrees[RED as ::core::ffi::c_int as usize].offset(bits as isize),
                16 as ::core::ffi::c_int,
                huff,
            );
            bits >>= AccumulateHCode(
                *(*htree_group).htrees[BLUE as ::core::ffi::c_int as usize].offset(bits as isize),
                0 as ::core::ffi::c_int,
                huff,
            );
            bits >>= AccumulateHCode(
                *(*htree_group).htrees[ALPHA as ::core::ffi::c_int as usize].offset(bits as isize),
                24 as ::core::ffi::c_int,
                huff,
            );
        }
        code = code.wrapping_add(1);
    }
}
unsafe extern "C" fn ReadHuffmanCodeLengths(
    dec: *mut VP8LDecoder,
    code_length_code_lengths: *const ::core::ffi::c_int,
    mut num_symbols: ::core::ffi::c_int,
    code_lengths: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let br: *mut VP8LBitReader = &raw mut (*dec).br_;
    let mut symbol: ::core::ffi::c_int = 0;
    let mut max_symbol: ::core::ffi::c_int = 0;
    let mut prev_code_len: ::core::ffi::c_int = DEFAULT_CODE_LENGTH;
    let mut tables: HuffmanTables = HuffmanTables {
        root: HuffmanTablesSegment {
            start: ::core::ptr::null_mut::<HuffmanCode>(),
            curr_table: ::core::ptr::null_mut::<HuffmanCode>(),
            next: ::core::ptr::null_mut::<HuffmanTablesSegment>(),
            size: 0,
        },
        curr_segment: ::core::ptr::null_mut::<HuffmanTablesSegment>(),
    };
    if !(VP8LHuffmanTablesAllocate(
        (1 as ::core::ffi::c_int) << LENGTHS_TABLE_BITS,
        &raw mut tables,
    ) == 0
        || VP8LBuildHuffmanTable(
            &raw mut tables,
            LENGTHS_TABLE_BITS,
            code_length_code_lengths as *const ::core::ffi::c_int,
            NUM_CODE_LENGTH_CODES,
        ) == 0)
    {
        if VP8LReadBits(br, 1 as ::core::ffi::c_int) != 0 {
            let length_nbits: ::core::ffi::c_int = (2 as uint32_t).wrapping_add(
                (2 as uint32_t).wrapping_mul(VP8LReadBits(br, 3 as ::core::ffi::c_int)),
            ) as ::core::ffi::c_int;
            max_symbol =
                (2 as uint32_t).wrapping_add(VP8LReadBits(br, length_nbits)) as ::core::ffi::c_int;
            if max_symbol > num_symbols {
                current_block = 6936477503727301328;
            } else {
                current_block = 5720623009719927633;
            }
        } else {
            max_symbol = num_symbols;
            current_block = 5720623009719927633;
        }
        match current_block {
            6936477503727301328 => {}
            _ => {
                symbol = 0 as ::core::ffi::c_int;
                loop {
                    if !(symbol < num_symbols) {
                        current_block = 7056779235015430508;
                        break;
                    }
                    let mut p: *const HuffmanCode = ::core::ptr::null::<HuffmanCode>();
                    let mut code_len: ::core::ffi::c_int = 0;
                    let fresh7 = max_symbol;
                    max_symbol = max_symbol - 1;
                    if fresh7 == 0 as ::core::ffi::c_int {
                        current_block = 7056779235015430508;
                        break;
                    }
                    VP8LFillBitWindow(br);
                    p = (*tables.curr_segment).start.offset(
                        ((VP8LPrefetchBits as unsafe extern "C" fn(*mut VP8LBitReader) -> uint32_t)(
                            br,
                        ) & LENGTHS_TABLE_MASK as uint32_t) as isize,
                    ) as *mut HuffmanCode;
                    VP8LSetBitPos(br, (*br).bit_pos_ + (*p).bits as ::core::ffi::c_int);
                    code_len = (*p).value as ::core::ffi::c_int;
                    if code_len < kCodeLengthLiterals {
                        let fresh8 = symbol;
                        symbol = symbol + 1;
                        *code_lengths.offset(fresh8 as isize) = code_len;
                        if code_len != 0 as ::core::ffi::c_int {
                            prev_code_len = code_len;
                        }
                    } else {
                        let use_prev: ::core::ffi::c_int =
                            (code_len == kCodeLengthRepeatCode) as ::core::ffi::c_int;
                        let slot: ::core::ffi::c_int = code_len - kCodeLengthLiterals;
                        let extra_bits: ::core::ffi::c_int =
                            kCodeLengthExtraBits[slot as usize] as ::core::ffi::c_int;
                        let repeat_offset: ::core::ffi::c_int =
                            kCodeLengthRepeatOffsets[slot as usize] as ::core::ffi::c_int;
                        let mut repeat: ::core::ffi::c_int = VP8LReadBits(br, extra_bits)
                            .wrapping_add(repeat_offset as uint32_t)
                            as ::core::ffi::c_int;
                        if symbol + repeat > num_symbols {
                            current_block = 6936477503727301328;
                            break;
                        }
                        let length: ::core::ffi::c_int = if use_prev != 0 {
                            prev_code_len
                        } else {
                            0 as ::core::ffi::c_int
                        };
                        loop {
                            let fresh9 = repeat;
                            repeat = repeat - 1;
                            if !(fresh9 > 0 as ::core::ffi::c_int) {
                                break;
                            }
                            let fresh10 = symbol;
                            symbol = symbol + 1;
                            *code_lengths.offset(fresh10 as isize) = length;
                        }
                    }
                }
                match current_block {
                    6936477503727301328 => {}
                    _ => {
                        ok = 1 as ::core::ffi::c_int;
                    }
                }
            }
        }
    }
    VP8LHuffmanTablesDeallocate(&raw mut tables);
    if ok == 0 {
        (*dec).status_ = VP8_STATUS_BITSTREAM_ERROR;
    }
    return ok;
}
unsafe extern "C" fn ReadHuffmanCode(
    mut alphabet_size: ::core::ffi::c_int,
    dec: *mut VP8LDecoder,
    code_lengths: *mut ::core::ffi::c_int,
    table: *mut HuffmanTables,
) -> ::core::ffi::c_int {
    let mut ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let br: *mut VP8LBitReader = &raw mut (*dec).br_;
    let simple_code: ::core::ffi::c_int =
        VP8LReadBits(br, 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    memset(
        code_lengths as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (alphabet_size as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    );
    if simple_code != 0 {
        let num_symbols: ::core::ffi::c_int = VP8LReadBits(br, 1 as ::core::ffi::c_int)
            .wrapping_add(1 as uint32_t)
            as ::core::ffi::c_int;
        let first_symbol_len_code: ::core::ffi::c_int =
            VP8LReadBits(br, 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
        let mut symbol: ::core::ffi::c_int = VP8LReadBits(
            br,
            if first_symbol_len_code == 0 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                8 as ::core::ffi::c_int
            },
        ) as ::core::ffi::c_int;
        *code_lengths.offset(symbol as isize) = 1 as ::core::ffi::c_int;
        if num_symbols == 2 as ::core::ffi::c_int {
            symbol = VP8LReadBits(br, 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
            *code_lengths.offset(symbol as isize) = 1 as ::core::ffi::c_int;
        }
        ok = 1 as ::core::ffi::c_int;
    } else {
        let mut i: ::core::ffi::c_int = 0;
        let mut code_length_code_lengths: [::core::ffi::c_int; 19] = [0 as ::core::ffi::c_int; 19];
        let num_codes: ::core::ffi::c_int = VP8LReadBits(br, 4 as ::core::ffi::c_int)
            .wrapping_add(4 as uint32_t)
            as ::core::ffi::c_int;
        if num_codes > NUM_CODE_LENGTH_CODES {
            (*dec).status_ = VP8_STATUS_BITSTREAM_ERROR;
            return 0 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while i < num_codes {
            code_length_code_lengths[kCodeLengthCodeOrder[i as usize] as usize] =
                VP8LReadBits(br, 3 as ::core::ffi::c_int) as ::core::ffi::c_int;
            i += 1;
        }
        ok = ReadHuffmanCodeLengths(
            dec,
            &raw mut code_length_code_lengths as *mut ::core::ffi::c_int,
            alphabet_size,
            code_lengths,
        );
    }
    ok = (ok != 0 && (*br).eos_ == 0) as ::core::ffi::c_int;
    if ok != 0 {
        size = VP8LBuildHuffmanTable(
            table,
            HUFFMAN_TABLE_BITS,
            code_lengths as *const ::core::ffi::c_int,
            alphabet_size,
        );
    }
    if ok == 0 || size == 0 as ::core::ffi::c_int {
        (*dec).status_ = VP8_STATUS_BITSTREAM_ERROR;
        return 0 as ::core::ffi::c_int;
    }
    return size;
}
unsafe extern "C" fn ReadHuffmanCodes(
    dec: *mut VP8LDecoder,
    mut xsize: ::core::ffi::c_int,
    mut ysize: ::core::ffi::c_int,
    mut color_cache_bits: ::core::ffi::c_int,
    mut allow_recursion: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let br: *mut VP8LBitReader = &raw mut (*dec).br_;
    let hdr: *mut VP8LMetadata = &raw mut (*dec).hdr_;
    let image_pixel_count = match checked_pixel_count(xsize, ysize) {
        Some(pixel_count) => pixel_count,
        None => {
            (*dec).status_ = VP8_STATUS_BITSTREAM_ERROR;
            return 0 as ::core::ffi::c_int;
        }
    };
    let mut huffman_image: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut htree_groups: *mut HTreeGroup = ::core::ptr::null_mut::<HTreeGroup>();
    let mut huffman_tables: *mut HuffmanTables = &raw mut (*hdr).huffman_tables_;
    let mut num_htree_groups: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut num_htree_groups_max: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut max_alphabet_size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut code_lengths: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let table_size: ::core::ffi::c_int =
        kTableSize[color_cache_bits as usize] as ::core::ffi::c_int;
    let mut mapping: Option<crate::rust_alloc::vec::Vec<::core::ffi::c_int>> = None;
    let mut ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    '_c2rust_label: {
        if (*huffman_tables).root.start.is_null() {
        } else {
            __assert_fail(
                b"huffman_tables->root.start == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                377 as ::core::ffi::c_uint,
                b"int ReadHuffmanCodes(VP8LDecoder *const, int, int, int, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*huffman_tables).curr_segment.is_null() {
        } else {
            __assert_fail(
                b"huffman_tables->curr_segment == NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                378 as ::core::ffi::c_uint,
                b"int ReadHuffmanCodes(VP8LDecoder *const, int, int, int, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if allow_recursion != 0 && VP8LReadBits(br, 1 as ::core::ffi::c_int) != 0 {
        let huffman_precision: ::core::ffi::c_int = VP8LReadBits(br, 3 as ::core::ffi::c_int)
            .wrapping_add(2 as uint32_t)
            as ::core::ffi::c_int;
        let huffman_xsize: ::core::ffi::c_int =
            VP8LSubSampleSize(xsize as uint32_t, huffman_precision as uint32_t)
                as ::core::ffi::c_int;
        let huffman_ysize: ::core::ffi::c_int =
            VP8LSubSampleSize(ysize as uint32_t, huffman_precision as uint32_t)
                as ::core::ffi::c_int;
        let huffman_pixs = match checked_pixel_count(huffman_xsize, huffman_ysize) {
            Some(pixel_count) => pixel_count,
            None => {
                (*dec).status_ = VP8_STATUS_BITSTREAM_ERROR;
                return 0 as ::core::ffi::c_int;
            }
        };
        if DecodeImageStream(
            huffman_xsize,
            huffman_ysize,
            0 as ::core::ffi::c_int,
            dec,
            &raw mut huffman_image,
        ) == 0
        {
            current_block = 3785228886137225493;
        } else {
            (*hdr).huffman_subsample_bits_ = huffman_precision;
            i = 0 as ::core::ffi::c_int;
            while i < huffman_pixs {
                let group: ::core::ffi::c_int =
                    (*huffman_image.offset(i as isize) >> 8 as ::core::ffi::c_int
                        & 0xffff as uint32_t) as ::core::ffi::c_int;
                *huffman_image.offset(i as isize) = group as uint32_t;
                if group >= num_htree_groups_max {
                    num_htree_groups_max = group + 1 as ::core::ffi::c_int;
                }
                i += 1;
            }
            let huffman_groups =
                ::core::slice::from_raw_parts_mut(huffman_image, huffman_pixs as usize);
            match compact_huffman_group_indices(huffman_groups) {
                Some((mapping_vec, unique_groups)) => {
                    if unique_groups <= 0 || unique_groups > image_pixel_count {
                        (*dec).status_ = VP8_STATUS_BITSTREAM_ERROR;
                        current_block = 3785228886137225493;
                    } else {
                        mapping = Some(mapping_vec);
                        num_htree_groups = unique_groups;
                        current_block = 4761528863920922185;
                    }
                }
                None => {
                    (*dec).status_ = VP8_STATUS_BITSTREAM_ERROR;
                    current_block = 3785228886137225493;
                }
            }
        }
    } else {
        current_block = 4761528863920922185;
    }
    match current_block {
        4761528863920922185 => {
            if !((*br).eos_ != 0) {
                j = 0 as ::core::ffi::c_int;
                while j < HUFFMAN_CODES_PER_META_CODE {
                    let mut alphabet_size: ::core::ffi::c_int =
                        kAlphabetSize[j as usize] as ::core::ffi::c_int;
                    if j == 0 as ::core::ffi::c_int && color_cache_bits > 0 as ::core::ffi::c_int {
                        alphabet_size += (1 as ::core::ffi::c_int) << color_cache_bits;
                    }
                    if max_alphabet_size < alphabet_size {
                        max_alphabet_size = alphabet_size;
                    }
                    j += 1;
                }
                code_lengths = WebPSafeCalloc(
                    max_alphabet_size as uint64_t,
                    ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                ) as *mut ::core::ffi::c_int;
                htree_groups = VP8LHtreeGroupsNew(num_htree_groups);
                let table_allocation_size =
                    checked_huffman_table_allocation_size(num_htree_groups, table_size)
                        .unwrap_or(0 as ::core::ffi::c_int);
                if htree_groups.is_null()
                    || code_lengths.is_null()
                    || table_allocation_size == 0 as ::core::ffi::c_int
                    || VP8LHuffmanTablesAllocate(table_allocation_size, huffman_tables) == 0
                {
                    (*dec).status_ = if table_allocation_size == 0 as ::core::ffi::c_int {
                        VP8_STATUS_BITSTREAM_ERROR
                    } else {
                        VP8_STATUS_OUT_OF_MEMORY
                    };
                } else {
                    i = 0 as ::core::ffi::c_int;
                    's_212: loop {
                        if !(i < num_htree_groups_max) {
                            current_block = 14329534724295951598;
                            break;
                        }
                        let mapped_group = mapping
                            .as_ref()
                            .and_then(|mapping_vec| mapping_vec.get(i as usize))
                            .copied();
                        if matches!(mapped_group, Some(value) if value == -(1 as ::core::ffi::c_int))
                        {
                            j = 0 as ::core::ffi::c_int;
                            while j < HUFFMAN_CODES_PER_META_CODE {
                                let mut alphabet_size_0: ::core::ffi::c_int =
                                    kAlphabetSize[j as usize] as ::core::ffi::c_int;
                                if j == 0 as ::core::ffi::c_int
                                    && color_cache_bits > 0 as ::core::ffi::c_int
                                {
                                    alphabet_size_0 +=
                                        (1 as ::core::ffi::c_int) << color_cache_bits;
                                }
                                if ReadHuffmanCode(
                                    alphabet_size_0,
                                    dec,
                                    code_lengths,
                                    ::core::ptr::null_mut::<HuffmanTables>(),
                                ) == 0
                                {
                                    current_block = 3785228886137225493;
                                    break 's_212;
                                }
                                j += 1;
                            }
                        } else {
                            let htree_group: *mut HTreeGroup = htree_groups
                                .offset(mapped_group.unwrap_or(i) as isize)
                                as *mut HTreeGroup;
                            let htrees: *mut *mut HuffmanCode =
                                &raw mut (*htree_group).htrees as *mut *mut HuffmanCode;
                            let mut size: ::core::ffi::c_int = 0;
                            let mut total_size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            let mut is_trivial_literal: ::core::ffi::c_int =
                                1 as ::core::ffi::c_int;
                            let mut max_bits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            j = 0 as ::core::ffi::c_int;
                            while j < HUFFMAN_CODES_PER_META_CODE {
                                let mut alphabet_size_1: ::core::ffi::c_int =
                                    kAlphabetSize[j as usize] as ::core::ffi::c_int;
                                if j == 0 as ::core::ffi::c_int
                                    && color_cache_bits > 0 as ::core::ffi::c_int
                                {
                                    alphabet_size_1 +=
                                        (1 as ::core::ffi::c_int) << color_cache_bits;
                                }
                                size = ReadHuffmanCode(
                                    alphabet_size_1,
                                    dec,
                                    code_lengths,
                                    huffman_tables,
                                );
                                let ref mut fresh6 = *htrees.offset(j as isize);
                                *fresh6 = (*(*huffman_tables).curr_segment).curr_table;
                                if size == 0 as ::core::ffi::c_int {
                                    current_block = 3785228886137225493;
                                    break 's_212;
                                }
                                if is_trivial_literal != 0
                                    && kLiteralMap[j as usize] as ::core::ffi::c_int
                                        == 1 as ::core::ffi::c_int
                                {
                                    is_trivial_literal = ((**htrees.offset(j as isize)).bits
                                        as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int)
                                        as ::core::ffi::c_int;
                                }
                                total_size +=
                                    (**htrees.offset(j as isize)).bits as ::core::ffi::c_int;
                                (*(*huffman_tables).curr_segment).curr_table = (*(*huffman_tables)
                                    .curr_segment)
                                    .curr_table
                                    .offset(size as isize);
                                if j <= ALPHA as ::core::ffi::c_int {
                                    let mut local_max_bits: ::core::ffi::c_int =
                                        *code_lengths.offset(0 as ::core::ffi::c_int as isize);
                                    let mut k: ::core::ffi::c_int = 0;
                                    k = 1 as ::core::ffi::c_int;
                                    while k < alphabet_size_1 {
                                        if *code_lengths.offset(k as isize) > local_max_bits {
                                            local_max_bits = *code_lengths.offset(k as isize);
                                        }
                                        k += 1;
                                    }
                                    max_bits += local_max_bits;
                                }
                                j += 1;
                            }
                            (*htree_group).is_trivial_literal = is_trivial_literal;
                            (*htree_group).is_trivial_code = 0 as ::core::ffi::c_int;
                            if is_trivial_literal != 0 {
                                let red: ::core::ffi::c_int = (*(*htrees
                                    .offset(RED as ::core::ffi::c_int as isize))
                                .offset(0 as ::core::ffi::c_int as isize))
                                .value
                                    as ::core::ffi::c_int;
                                let blue: ::core::ffi::c_int = (*(*htrees
                                    .offset(BLUE as ::core::ffi::c_int as isize))
                                .offset(0 as ::core::ffi::c_int as isize))
                                .value
                                    as ::core::ffi::c_int;
                                let alpha: ::core::ffi::c_int = (*(*htrees
                                    .offset(ALPHA as ::core::ffi::c_int as isize))
                                .offset(0 as ::core::ffi::c_int as isize))
                                .value
                                    as ::core::ffi::c_int;
                                (*htree_group).literal_arb = (alpha as uint32_t)
                                    << 24 as ::core::ffi::c_int
                                    | (red << 16 as ::core::ffi::c_int) as uint32_t
                                    | blue as uint32_t;
                                if total_size == 0 as ::core::ffi::c_int
                                    && ((*(*htrees.offset(GREEN as ::core::ffi::c_int as isize))
                                        .offset(0 as ::core::ffi::c_int as isize))
                                    .value
                                        as ::core::ffi::c_int)
                                        < NUM_LITERAL_CODES
                                {
                                    (*htree_group).is_trivial_code = 1 as ::core::ffi::c_int;
                                    (*htree_group).literal_arb |= (((*(*htrees
                                        .offset(GREEN as ::core::ffi::c_int as isize))
                                    .offset(0 as ::core::ffi::c_int as isize))
                                    .value
                                        as ::core::ffi::c_int)
                                        << 8 as ::core::ffi::c_int)
                                        as uint32_t;
                                }
                            }
                            (*htree_group).use_packed_table = ((*htree_group).is_trivial_code == 0
                                && max_bits < HUFFMAN_PACKED_BITS)
                                as ::core::ffi::c_int;
                            if (*htree_group).use_packed_table != 0 {
                                BuildPackedTable(htree_group);
                            }
                        }
                        i += 1;
                    }
                    match current_block {
                        3785228886137225493 => {}
                        _ => {
                            ok = 1 as ::core::ffi::c_int;
                            (*hdr).huffman_image_ = huffman_image;
                            (*hdr).num_htree_groups_ = num_htree_groups;
                            (*hdr).htree_groups_ = htree_groups;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    WebPSafeFree(code_lengths as *mut ::core::ffi::c_void);
    if ok == 0 {
        WebPSafeFree(huffman_image as *mut ::core::ffi::c_void);
        VP8LHuffmanTablesDeallocate(huffman_tables);
        VP8LHtreeGroupsFree(htree_groups);
    }
    return ok;
}
unsafe extern "C" fn AllocateAndInitRescaler(
    dec: *mut VP8LDecoder,
    io: *mut VP8Io,
) -> ::core::ffi::c_int {
    let num_channels: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    let in_width: ::core::ffi::c_int = (*io).mb_w;
    let out_width: ::core::ffi::c_int = (*io).scaled_width;
    let in_height: ::core::ffi::c_int = (*io).mb_h;
    let out_height: ::core::ffi::c_int = (*io).scaled_height;
    let work_size: uint64_t =
        ((2 as ::core::ffi::c_int * num_channels) as uint64_t).wrapping_mul(out_width as uint64_t);
    let mut work: *mut rescaler_t = ::core::ptr::null_mut::<rescaler_t>();
    let scaled_data_size: uint64_t = out_width as uint64_t;
    let mut scaled_data: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let memory_size: uint64_t = (::core::mem::size_of::<WebPRescaler>() as uint64_t)
        .wrapping_add(work_size.wrapping_mul(::core::mem::size_of::<rescaler_t>() as uint64_t))
        .wrapping_add(
            scaled_data_size.wrapping_mul(::core::mem::size_of::<uint32_t>() as uint64_t),
        );
    let mut memory: *mut uint8_t =
        WebPSafeMalloc(memory_size, ::core::mem::size_of::<uint8_t>() as size_t) as *mut uint8_t;
    if memory.is_null() {
        (*dec).status_ = VP8_STATUS_OUT_OF_MEMORY;
        return 0 as ::core::ffi::c_int;
    }
    '_c2rust_label: {
        if (*dec).rescaler_memory.is_null() {
        } else {
            __assert_fail(
                b"dec->rescaler_memory == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                557 as ::core::ffi::c_uint,
                b"int AllocateAndInitRescaler(VP8LDecoder *const, VP8Io *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*dec).rescaler_memory = memory;
    (*dec).rescaler = memory as *mut WebPRescaler;
    memory = memory.offset(::core::mem::size_of::<WebPRescaler>() as usize as isize);
    work = memory as *mut rescaler_t;
    memory = memory
        .offset(work_size.wrapping_mul(::core::mem::size_of::<rescaler_t>() as uint64_t) as isize);
    scaled_data = memory as *mut uint32_t;
    if WebPRescalerInit(
        (*dec).rescaler,
        in_width,
        in_height,
        scaled_data as *mut uint8_t,
        out_width,
        out_height,
        0 as ::core::ffi::c_int,
        num_channels,
        work,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn Export(
    rescaler: *mut WebPRescaler,
    mut colorspace: WEBP_CSP_MODE,
    mut rgba_stride: ::core::ffi::c_int,
    rgba: *mut uint8_t,
) -> ::core::ffi::c_int {
    let src: *mut uint32_t = (*rescaler).dst as *mut uint32_t;
    let mut dst: *mut uint8_t = rgba;
    let dst_width: ::core::ffi::c_int = (*rescaler).dst_width;
    let mut num_lines_out: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while WebPRescalerHasPendingOutput(rescaler) != 0 {
        WebPRescalerExportRow(rescaler as *mut WebPRescaler);
        WebPMultARGBRow.expect("non-null function pointer")(
            src,
            dst_width,
            1 as ::core::ffi::c_int,
        );
        VP8LConvertFromBGRA(src, dst_width, colorspace, dst);
        dst = dst.offset(rgba_stride as isize);
        num_lines_out += 1;
    }
    return num_lines_out;
}
unsafe extern "C" fn EmitRescaledRowsRGBA(
    dec: *const VP8LDecoder,
    mut in_0: *mut uint8_t,
    mut in_stride: ::core::ffi::c_int,
    mut mb_h: ::core::ffi::c_int,
    out: *mut uint8_t,
    mut out_stride: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let colorspace: WEBP_CSP_MODE = (*(*dec).output_).colorspace;
    let mut num_lines_in: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut num_lines_out: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while num_lines_in < mb_h {
        let row_in: *mut uint8_t =
            in_0.offset((num_lines_in as uint64_t).wrapping_mul(in_stride as uint64_t) as isize);
        let row_out: *mut uint8_t =
            out.offset((num_lines_out as uint64_t).wrapping_mul(out_stride as uint64_t) as isize);
        let lines_left: ::core::ffi::c_int = mb_h - num_lines_in;
        let needed_lines: ::core::ffi::c_int =
            WebPRescaleNeededLines((*dec).rescaler, lines_left) as ::core::ffi::c_int;
        let mut lines_imported: ::core::ffi::c_int = 0;
        '_c2rust_label: {
            if needed_lines > 0 as ::core::ffi::c_int && needed_lines <= lines_left {
            } else {
                __assert_fail(
                    b"needed_lines > 0 && needed_lines <= lines_left\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    610 as ::core::ffi::c_uint,
                    b"int EmitRescaledRowsRGBA(const VP8LDecoder *const, uint8_t *, int, int, uint8_t *const, int)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        WebPMultARGBRows(
            row_in,
            in_stride,
            (*(*dec).rescaler).src_width,
            needed_lines,
            0 as ::core::ffi::c_int,
        );
        lines_imported = WebPRescalerImport((*dec).rescaler, lines_left, row_in, in_stride);
        '_c2rust_label_0: {
            if lines_imported == needed_lines {
            } else {
                __assert_fail(
                    b"lines_imported == needed_lines\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    615 as ::core::ffi::c_uint,
                    b"int EmitRescaledRowsRGBA(const VP8LDecoder *const, uint8_t *, int, int, uint8_t *const, int)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        num_lines_in += lines_imported;
        num_lines_out += Export((*dec).rescaler, colorspace, out_stride, row_out);
    }
    return num_lines_out;
}
unsafe extern "C" fn EmitRows(
    mut colorspace: WEBP_CSP_MODE,
    mut row_in: *const uint8_t,
    mut in_stride: ::core::ffi::c_int,
    mut mb_w: ::core::ffi::c_int,
    mut mb_h: ::core::ffi::c_int,
    out: *mut uint8_t,
    mut out_stride: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lines: ::core::ffi::c_int = mb_h;
    let mut row_out: *mut uint8_t = out;
    loop {
        let fresh15 = lines;
        lines = lines - 1;
        if !(fresh15 > 0 as ::core::ffi::c_int) {
            break;
        }
        VP8LConvertFromBGRA(row_in as *const uint32_t, mb_w, colorspace, row_out);
        row_in = row_in.offset(in_stride as isize);
        row_out = row_out.offset(out_stride as isize);
    }
    return mb_h;
}
unsafe extern "C" fn ConvertToYUVA(
    src: *const uint32_t,
    mut width: ::core::ffi::c_int,
    mut y_pos: ::core::ffi::c_int,
    output: *const WebPDecBuffer,
) {
    let buf: *const WebPYUVABuffer = &raw const (*output).u.YUVA;
    WebPConvertARGBToY.expect("non-null function pointer")(
        src,
        (*buf).y.offset((y_pos * (*buf).y_stride) as isize),
        width,
    );
    let u: *mut uint8_t = (*buf)
        .u
        .offset(((y_pos >> 1 as ::core::ffi::c_int) * (*buf).u_stride) as isize);
    let v: *mut uint8_t = (*buf)
        .v
        .offset(((y_pos >> 1 as ::core::ffi::c_int) * (*buf).v_stride) as isize);
    WebPConvertARGBToUV.expect("non-null function pointer")(
        src,
        u,
        v,
        width,
        (y_pos & 1 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    );
    if !(*buf).a.is_null() {
        let a: *mut uint8_t = (*buf).a.offset((y_pos * (*buf).a_stride) as isize);
        WebPExtractAlpha.expect("non-null function pointer")(
            (src as *mut uint8_t).offset(3 as ::core::ffi::c_int as isize),
            0 as ::core::ffi::c_int,
            width,
            1 as ::core::ffi::c_int,
            a,
            0 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn ExportYUVA(
    dec: *const VP8LDecoder,
    mut y_pos: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let rescaler: *mut WebPRescaler = (*dec).rescaler;
    let src: *mut uint32_t = (*rescaler).dst as *mut uint32_t;
    let dst_width: ::core::ffi::c_int = (*rescaler).dst_width;
    let mut num_lines_out: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while WebPRescalerHasPendingOutput(rescaler) != 0 {
        WebPRescalerExportRow(rescaler as *mut WebPRescaler);
        WebPMultARGBRow.expect("non-null function pointer")(
            src,
            dst_width,
            1 as ::core::ffi::c_int,
        );
        ConvertToYUVA(src, dst_width, y_pos, (*dec).output_);
        y_pos += 1;
        num_lines_out += 1;
    }
    return num_lines_out;
}
unsafe extern "C" fn EmitRescaledRowsYUVA(
    dec: *const VP8LDecoder,
    mut in_0: *mut uint8_t,
    mut in_stride: ::core::ffi::c_int,
    mut mb_h: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut num_lines_in: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y_pos: ::core::ffi::c_int = (*dec).last_out_row_;
    while num_lines_in < mb_h {
        let lines_left: ::core::ffi::c_int = mb_h - num_lines_in;
        let needed_lines: ::core::ffi::c_int =
            WebPRescaleNeededLines((*dec).rescaler, lines_left) as ::core::ffi::c_int;
        let mut lines_imported: ::core::ffi::c_int = 0;
        WebPMultARGBRows(
            in_0,
            in_stride,
            (*(*dec).rescaler).src_width,
            needed_lines,
            0 as ::core::ffi::c_int,
        );
        lines_imported = WebPRescalerImport((*dec).rescaler, lines_left, in_0, in_stride);
        '_c2rust_label: {
            if lines_imported == needed_lines {
            } else {
                __assert_fail(
                    b"lines_imported == needed_lines\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    694 as ::core::ffi::c_uint,
                    b"int EmitRescaledRowsYUVA(const VP8LDecoder *const, uint8_t *, int, int)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        num_lines_in += lines_imported;
        in_0 = in_0.offset((needed_lines * in_stride) as isize);
        y_pos += ExportYUVA(dec, y_pos);
    }
    return y_pos;
}
unsafe extern "C" fn EmitRowsYUVA(
    dec: *const VP8LDecoder,
    mut in_0: *const uint8_t,
    mut in_stride: ::core::ffi::c_int,
    mut mb_w: ::core::ffi::c_int,
    mut num_rows: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut y_pos: ::core::ffi::c_int = (*dec).last_out_row_;
    loop {
        let fresh14 = num_rows;
        num_rows = num_rows - 1;
        if !(fresh14 > 0 as ::core::ffi::c_int) {
            break;
        }
        ConvertToYUVA(in_0 as *const uint32_t, mb_w, y_pos, (*dec).output_);
        in_0 = in_0.offset(in_stride as isize);
        y_pos += 1;
    }
    return y_pos;
}
unsafe extern "C" fn SetCropWindow(
    io: *mut VP8Io,
    mut y_start: ::core::ffi::c_int,
    mut y_end: ::core::ffi::c_int,
    in_data: *mut *mut uint8_t,
    mut pixel_stride: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if y_start < y_end {
        } else {
            __assert_fail(
                b"y_start < y_end\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                724 as ::core::ffi::c_uint,
                b"int SetCropWindow(VP8Io *const, int, int, uint8_t **const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*io).crop_left < (*io).crop_right {
        } else {
            __assert_fail(
                b"io->crop_left < io->crop_right\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                725 as ::core::ffi::c_uint,
                b"int SetCropWindow(VP8Io *const, int, int, uint8_t **const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if y_end > (*io).crop_bottom {
        y_end = (*io).crop_bottom;
    }
    if y_start < (*io).crop_top {
        let delta: ::core::ffi::c_int = (*io).crop_top - y_start;
        y_start = (*io).crop_top;
        *in_data = (*in_data).offset((delta * pixel_stride) as isize);
    }
    if y_start >= y_end {
        return 0 as ::core::ffi::c_int;
    }
    *in_data = (*in_data).offset(
        ((*io).crop_left as usize).wrapping_mul(::core::mem::size_of::<uint32_t>() as usize)
            as isize,
    );
    (*io).mb_y = y_start - (*io).crop_top;
    (*io).mb_w = (*io).crop_right - (*io).crop_left;
    (*io).mb_h = y_end - y_start;
    return 1 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn GetMetaIndex(
    image: *const uint32_t,
    mut xsize: ::core::ffi::c_int,
    mut bits: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if bits == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    return *image.offset((xsize * (y >> bits) + (x >> bits)) as isize) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn GetHtreeGroupForPos(
    hdr: *mut VP8LMetadata,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) -> *mut HTreeGroup {
    let meta_index: ::core::ffi::c_int = GetMetaIndex(
        (*hdr).huffman_image_,
        (*hdr).huffman_xsize_,
        (*hdr).huffman_subsample_bits_,
        x,
        y,
    ) as ::core::ffi::c_int;
    '_c2rust_label: {
        if meta_index < (*hdr).num_htree_groups_ {
        } else {
            __assert_fail(
                b"meta_index < hdr->num_htree_groups_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                756 as ::core::ffi::c_uint,
                b"HTreeGroup *GetHtreeGroupForPos(VP8LMetadata *const, int, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return (*hdr).htree_groups_.offset(meta_index as isize);
}
unsafe extern "C" fn ApplyInverseTransforms(
    dec: *mut VP8LDecoder,
    mut start_row: ::core::ffi::c_int,
    mut num_rows: ::core::ffi::c_int,
    rows: *const uint32_t,
) {
    let mut n: ::core::ffi::c_int = (*dec).next_transform_;
    let cache_pixs: ::core::ffi::c_int = (*dec).width_ * num_rows;
    let end_row: ::core::ffi::c_int = start_row + num_rows;
    let mut rows_in: *const uint32_t = rows;
    let rows_out: *mut uint32_t = (*dec).argb_cache_;
    loop {
        let fresh11 = n;
        n = n - 1;
        if !(fresh11 > 0 as ::core::ffi::c_int) {
            break;
        }
        let transform: *mut VP8LTransform = (&raw mut (*dec).transforms_ as *mut VP8LTransform)
            .offset(n as isize) as *mut VP8LTransform;
        VP8LInverseTransform(transform, start_row, end_row, rows_in, rows_out);
        rows_in = rows_out;
    }
    if rows_in != rows_out as *const uint32_t {
        memcpy(
            rows_out as *mut ::core::ffi::c_void,
            rows_in as *const ::core::ffi::c_void,
            (cache_pixs as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        );
    }
}
unsafe extern "C" fn ProcessRows(dec: *mut VP8LDecoder, mut row: ::core::ffi::c_int) {
    let rows: *const uint32_t = (*dec)
        .pixels_
        .offset(((*dec).width_ * (*dec).last_row_) as isize);
    let num_rows: ::core::ffi::c_int = row - (*dec).last_row_;
    '_c2rust_label: {
        if row <= (*(*dec).io_).crop_bottom {
        } else {
            __assert_fail(
                b"row <= dec->io_->crop_bottom\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                792 as ::core::ffi::c_uint,
                b"void ProcessRows(VP8LDecoder *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if num_rows <= 16 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"num_rows <= NUM_ARGB_CACHE_ROWS\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                795 as ::core::ffi::c_uint,
                b"void ProcessRows(VP8LDecoder *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if num_rows > 0 as ::core::ffi::c_int {
        let io: *mut VP8Io = (*dec).io_;
        let mut rows_data: *mut uint8_t = (*dec).argb_cache_ as *mut uint8_t;
        let in_stride: ::core::ffi::c_int = ((*io).width as usize)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as usize)
            as ::core::ffi::c_int;
        ApplyInverseTransforms(dec, (*dec).last_row_, num_rows, rows);
        if !(SetCropWindow(io, (*dec).last_row_, row, &raw mut rows_data, in_stride) == 0) {
            let output: *const WebPDecBuffer = (*dec).output_;
            if WebPIsRGBMode((*output).colorspace) != 0 {
                let buf: *const WebPRGBABuffer = &raw const (*output).u.RGBA;
                let rgba: *mut uint8_t = (*buf)
                    .rgba
                    .offset(((*dec).last_out_row_ as int64_t * (*buf).stride as int64_t) as isize);
                let num_rows_out: ::core::ffi::c_int = if (*io).use_scaling != 0 {
                    EmitRescaledRowsRGBA(dec, rows_data, in_stride, (*io).mb_h, rgba, (*buf).stride)
                        as ::core::ffi::c_int
                } else {
                    EmitRows(
                        (*output).colorspace,
                        rows_data,
                        in_stride,
                        (*io).mb_w,
                        (*io).mb_h,
                        rgba,
                        (*buf).stride,
                    ) as ::core::ffi::c_int
                };
                (*dec).last_out_row_ += num_rows_out;
            } else {
                (*dec).last_out_row_ = if (*io).use_scaling != 0 {
                    EmitRescaledRowsYUVA(dec, rows_data, in_stride, (*io).mb_h)
                } else {
                    EmitRowsYUVA(dec, rows_data, in_stride, (*io).mb_w, (*io).mb_h)
                };
            }
            '_c2rust_label_1: {
                if (*dec).last_out_row_ <= (*output).height {
                } else {
                    __assert_fail(
                        b"dec->last_out_row_ <= output->height\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        824 as ::core::ffi::c_uint,
                        b"void ProcessRows(VP8LDecoder *const, int)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
        }
    }
    (*dec).last_row_ = row;
    '_c2rust_label_2: {
        if (*dec).last_row_ <= (*dec).height_ {
        } else {
            __assert_fail(
                b"dec->last_row_ <= dec->height_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                830 as ::core::ffi::c_uint,
                b"void ProcessRows(VP8LDecoder *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
}
unsafe extern "C" fn Is8bOptimizable(hdr: *const VP8LMetadata) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if (*hdr).color_cache_size_ > 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*hdr).num_htree_groups_ {
        let htrees: *mut *mut HuffmanCode =
            &raw mut (*(*hdr).htree_groups_.offset(i as isize)).htrees as *mut *mut HuffmanCode;
        if (*(*htrees.offset(RED as ::core::ffi::c_int as isize))
            .offset(0 as ::core::ffi::c_int as isize))
        .bits as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*(*htrees.offset(BLUE as ::core::ffi::c_int as isize))
            .offset(0 as ::core::ffi::c_int as isize))
        .bits as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*(*htrees.offset(ALPHA as ::core::ffi::c_int as isize))
            .offset(0 as ::core::ffi::c_int as isize))
        .bits as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn AlphaApplyFilter(
    alph_dec: *mut ALPHDecoder,
    mut first_row: ::core::ffi::c_int,
    mut last_row: ::core::ffi::c_int,
    mut out: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
) {
    if (*alph_dec).filter_ as ::core::ffi::c_uint
        != WEBP_FILTER_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut y: ::core::ffi::c_int = 0;
        let mut prev_line: *const uint8_t = (*alph_dec).prev_line_;
        '_c2rust_label: {
            if WebPUnfilters[(*alph_dec).filter_ as usize].is_some() {
            } else {
                __assert_fail(
                    b"WebPUnfilters[alph_dec->filter_] != NULL\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    855 as ::core::ffi::c_uint,
                    b"void AlphaApplyFilter(ALPHDecoder *const, int, int, uint8_t *, int)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        y = first_row;
        while y < last_row {
            WebPUnfilters[(*alph_dec).filter_ as usize].expect("non-null function pointer")(
                prev_line, out, out, stride,
            );
            prev_line = out;
            out = out.offset(stride as isize);
            y += 1;
        }
        (*alph_dec).prev_line_ = prev_line;
    }
}
unsafe extern "C" fn ExtractPalettedAlphaRows(
    dec: *mut VP8LDecoder,
    mut last_row: ::core::ffi::c_int,
) {
    let alph_dec: *mut ALPHDecoder = (*(*dec).io_).opaque as *mut ALPHDecoder;
    let top_row: ::core::ffi::c_int = if (*alph_dec).filter_ as ::core::ffi::c_uint
        == WEBP_FILTER_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*alph_dec).filter_ as ::core::ffi::c_uint
            == WEBP_FILTER_HORIZONTAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*(*dec).io_).crop_top
    } else {
        (*dec).last_row_
    };
    let first_row: ::core::ffi::c_int = if (*dec).last_row_ < top_row {
        top_row
    } else {
        (*dec).last_row_
    };
    '_c2rust_label: {
        if last_row <= (*(*dec).io_).crop_bottom {
        } else {
            __assert_fail(
                b"last_row <= dec->io_->crop_bottom\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                874 as ::core::ffi::c_uint,
                b"void ExtractPalettedAlphaRows(VP8LDecoder *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if last_row > first_row {
        let width: ::core::ffi::c_int = (*(*dec).io_).width;
        let mut out: *mut uint8_t = (*alph_dec).output_.offset((width * first_row) as isize);
        let in_0: *const uint8_t =
            ((*dec).pixels_ as *mut uint8_t).offset(((*dec).width_ * first_row) as isize);
        let transform: *mut VP8LTransform = (&raw mut (*dec).transforms_ as *mut VP8LTransform)
            .offset(0 as ::core::ffi::c_int as isize)
            as *mut VP8LTransform;
        '_c2rust_label_0: {
            if (*dec).next_transform_ == 1 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"dec->next_transform_ == 1\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    882 as ::core::ffi::c_uint,
                    b"void ExtractPalettedAlphaRows(VP8LDecoder *const, int)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_1: {
            if (*transform).type_ as ::core::ffi::c_uint
                == COLOR_INDEXING_TRANSFORM as ::core::ffi::c_int as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"transform->type_ == COLOR_INDEXING_TRANSFORM\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    883 as ::core::ffi::c_uint,
                    b"void ExtractPalettedAlphaRows(VP8LDecoder *const, int)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        VP8LColorIndexInverseTransformAlpha(transform, first_row, last_row, in_0, out);
        AlphaApplyFilter(alph_dec, first_row, last_row, out, width);
    }
    (*dec).last_out_row_ = last_row;
    (*dec).last_row_ = (*dec).last_out_row_;
}
#[inline]
unsafe extern "C" fn Rotate8b(mut V: uint32_t) -> uint32_t {
    return (V & 0xff as uint32_t) << 24 as ::core::ffi::c_int | V >> 8 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn CopySmallPattern8b(
    mut src: *const uint8_t,
    mut dst: *mut uint8_t,
    mut length: ::core::ffi::c_int,
    mut pattern: uint32_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    while dst as uintptr_t & 3 as ::core::ffi::c_int as uintptr_t != 0 {
        let fresh12 = src;
        src = src.offset(1);
        let fresh13 = dst;
        dst = dst.offset(1);
        *fresh13 = *fresh12;
        pattern = Rotate8b(pattern);
        length -= 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < length >> 2 as ::core::ffi::c_int {
        *(dst as *mut uint32_t).offset(i as isize) = pattern;
        i += 1;
    }
    i <<= 2 as ::core::ffi::c_int;
    while i < length {
        *dst.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn CopyBlock8b(
    dst: *mut uint8_t,
    mut dist: ::core::ffi::c_int,
    mut length: ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut src: *const uint8_t = dst.offset(-(dist as isize));
    if length >= 8 as ::core::ffi::c_int {
        let mut pattern: uint32_t = 0 as uint32_t;
        match dist {
            1 => {
                current_block = 13351617936709262981;
                match current_block {
                    5604031265259680791 => {
                        memcpy(
                            &raw mut pattern as *mut ::core::ffi::c_void,
                            src as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<uint32_t>() as size_t,
                        );
                    }
                    13351617936709262981 => {
                        pattern = *src.offset(0 as ::core::ffi::c_int as isize) as uint32_t;
                        pattern = (0x1010101 as uint32_t).wrapping_mul(pattern);
                    }
                    _ => {
                        memcpy(
                            &raw mut pattern as *mut ::core::ffi::c_void,
                            src as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<uint16_t>() as size_t,
                        );
                        pattern = (0x10001 as uint32_t).wrapping_mul(pattern);
                    }
                }
                CopySmallPattern8b(src, dst, length, pattern);
                return;
            }
            2 => {
                current_block = 6313240952847421736;
                match current_block {
                    5604031265259680791 => {
                        memcpy(
                            &raw mut pattern as *mut ::core::ffi::c_void,
                            src as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<uint32_t>() as size_t,
                        );
                    }
                    13351617936709262981 => {
                        pattern = *src.offset(0 as ::core::ffi::c_int as isize) as uint32_t;
                        pattern = (0x1010101 as uint32_t).wrapping_mul(pattern);
                    }
                    _ => {
                        memcpy(
                            &raw mut pattern as *mut ::core::ffi::c_void,
                            src as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<uint16_t>() as size_t,
                        );
                        pattern = (0x10001 as uint32_t).wrapping_mul(pattern);
                    }
                }
                CopySmallPattern8b(src, dst, length, pattern);
                return;
            }
            4 => {
                current_block = 5604031265259680791;
                match current_block {
                    5604031265259680791 => {
                        memcpy(
                            &raw mut pattern as *mut ::core::ffi::c_void,
                            src as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<uint32_t>() as size_t,
                        );
                    }
                    13351617936709262981 => {
                        pattern = *src.offset(0 as ::core::ffi::c_int as isize) as uint32_t;
                        pattern = (0x1010101 as uint32_t).wrapping_mul(pattern);
                    }
                    _ => {
                        memcpy(
                            &raw mut pattern as *mut ::core::ffi::c_void,
                            src as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<uint16_t>() as size_t,
                        );
                        pattern = (0x10001 as uint32_t).wrapping_mul(pattern);
                    }
                }
                CopySmallPattern8b(src, dst, length, pattern);
                return;
            }
            _ => {}
        }
    }
    if dist >= length {
        memcpy(
            dst as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            (length as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        );
    } else {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < length {
            *dst.offset(i as isize) = *src.offset(i as isize);
            i += 1;
        }
    };
}
#[inline]
unsafe extern "C" fn CopySmallPattern32b(
    mut src: *const uint32_t,
    mut dst: *mut uint32_t,
    mut length: ::core::ffi::c_int,
    mut pattern: uint64_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    if dst as uintptr_t & 4 as ::core::ffi::c_int as uintptr_t != 0 {
        let fresh3 = src;
        src = src.offset(1);
        let fresh4 = dst;
        dst = dst.offset(1);
        *fresh4 = *fresh3;
        pattern = pattern >> 32 as ::core::ffi::c_int | pattern << 32 as ::core::ffi::c_int;
        length -= 1;
    }
    '_c2rust_label: {
        if 0 as uintptr_t == dst as uintptr_t & 7 as ::core::ffi::c_int as uintptr_t {
        } else {
            __assert_fail(
                b"0 == ((uintptr_t)dst & 7)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                982 as ::core::ffi::c_uint,
                b"void CopySmallPattern32b(const uint32_t *, uint32_t *, int, uint64_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    i = 0 as ::core::ffi::c_int;
    while i < length >> 1 as ::core::ffi::c_int {
        *(dst as *mut uint64_t).offset(i as isize) = pattern;
        i += 1;
    }
    if length & 1 as ::core::ffi::c_int != 0 {
        *dst.offset((i << 1 as ::core::ffi::c_int) as isize) =
            *src.offset((i << 1 as ::core::ffi::c_int) as isize);
    }
}
#[inline]
unsafe extern "C" fn CopyBlock32b(
    dst: *mut uint32_t,
    mut dist: ::core::ffi::c_int,
    mut length: ::core::ffi::c_int,
) {
    let src: *const uint32_t = dst.offset(-(dist as isize));
    if dist <= 2 as ::core::ffi::c_int
        && length >= 4 as ::core::ffi::c_int
        && dst as uintptr_t & 3 as ::core::ffi::c_int as uintptr_t == 0 as uintptr_t
    {
        let mut pattern: uint64_t = 0;
        if dist == 1 as ::core::ffi::c_int {
            pattern = *src.offset(0 as ::core::ffi::c_int as isize) as uint64_t;
            pattern |= pattern << 32 as ::core::ffi::c_int;
        } else {
            memcpy(
                &raw mut pattern as *mut ::core::ffi::c_void,
                src as *const ::core::ffi::c_void,
                ::core::mem::size_of::<uint64_t>() as size_t,
            );
        }
        CopySmallPattern32b(src, dst, length, pattern);
    } else if dist >= length {
        memcpy(
            dst as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            (length as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        );
    } else {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < length {
            *dst.offset(i as isize) = *src.offset(i as isize);
            i += 1;
        }
    };
}
unsafe extern "C" fn DecodeAlphaData(
    dec: *mut VP8LDecoder,
    data: *mut uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut last_row: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut row: ::core::ffi::c_int = (*dec).last_pixel_ / width;
    let mut col: ::core::ffi::c_int = (*dec).last_pixel_ % width;
    let br: *mut VP8LBitReader = &raw mut (*dec).br_;
    let hdr: *mut VP8LMetadata = &raw mut (*dec).hdr_;
    let mut pos: ::core::ffi::c_int = (*dec).last_pixel_;
    let end: ::core::ffi::c_int = width * height;
    let last: ::core::ffi::c_int = width * last_row;
    let len_code_limit: ::core::ffi::c_int = NUM_LITERAL_CODES + NUM_LENGTH_CODES;
    let mask: ::core::ffi::c_int = (*hdr).huffman_mask_;
    let mut htree_group: *const HTreeGroup = if pos < last {
        GetHtreeGroupForPos(hdr, col, row)
    } else {
        ::core::ptr::null_mut::<HTreeGroup>()
    };
    '_c2rust_label: {
        if pos <= end {
        } else {
            __assert_fail(
                b"pos <= end\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1027 as ::core::ffi::c_uint,
                b"int DecodeAlphaData(VP8LDecoder *const, uint8_t *const, int, int, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if last_row <= height {
        } else {
            __assert_fail(
                b"last_row <= height\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1028 as ::core::ffi::c_uint,
                b"int DecodeAlphaData(VP8LDecoder *const, uint8_t *const, int, int, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if Is8bOptimizable(hdr) != 0 {
        } else {
            __assert_fail(
                b"Is8bOptimizable(hdr)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1029 as ::core::ffi::c_uint,
                b"int DecodeAlphaData(VP8LDecoder *const, uint8_t *const, int, int, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    loop {
        if !((*br).eos_ == 0 && pos < last) {
            current_block = 7427571413727699167;
            break;
        }
        let mut code: ::core::ffi::c_int = 0;
        if col & mask == 0 as ::core::ffi::c_int {
            htree_group = GetHtreeGroupForPos(hdr, col, row);
        }
        '_c2rust_label_2: {
            if !htree_group.is_null() {
            } else {
                __assert_fail(
                    b"htree_group != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1037 as ::core::ffi::c_uint,
                    b"int DecodeAlphaData(VP8LDecoder *const, uint8_t *const, int, int, int)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        VP8LFillBitWindow(br);
        code = ReadSymbol(
            (*htree_group).htrees[GREEN as ::core::ffi::c_int as usize],
            br,
        );
        if code < NUM_LITERAL_CODES {
            *data.offset(pos as isize) = code as uint8_t;
            pos += 1;
            col += 1;
            if col >= width {
                col = 0 as ::core::ffi::c_int;
                row += 1;
                if row <= last_row && row % NUM_ARGB_CACHE_ROWS == 0 as ::core::ffi::c_int {
                    ExtractPalettedAlphaRows(dec, row);
                }
            }
        } else if code < len_code_limit {
            let mut dist_code: ::core::ffi::c_int = 0;
            let mut dist: ::core::ffi::c_int = 0;
            let length_sym: ::core::ffi::c_int = code - NUM_LITERAL_CODES;
            let length: ::core::ffi::c_int = GetCopyLength(length_sym, br) as ::core::ffi::c_int;
            let dist_symbol: ::core::ffi::c_int = ReadSymbol(
                (*htree_group).htrees[DIST as ::core::ffi::c_int as usize],
                br,
            ) as ::core::ffi::c_int;
            VP8LFillBitWindow(br);
            dist_code = GetCopyDistance(dist_symbol, br);
            dist = PlaneCodeToDistance(width, dist_code);
            if pos >= dist && end - pos >= length {
                CopyBlock8b(data.offset(pos as isize), dist, length);
                pos += length;
                col += length;
                while col >= width {
                    col -= width;
                    row += 1;
                    if row <= last_row && row % NUM_ARGB_CACHE_ROWS == 0 as ::core::ffi::c_int {
                        ExtractPalettedAlphaRows(dec, row);
                    }
                }
                if pos < last && col & mask != 0 {
                    htree_group = GetHtreeGroupForPos(hdr, col, row);
                }
            } else {
                ok = 0 as ::core::ffi::c_int;
                current_block = 16251394943426114332;
                break;
            }
        } else {
            ok = 0 as ::core::ffi::c_int;
            current_block = 16251394943426114332;
            break;
        }
        (*br).eos_ = VP8LIsEndOfStream(br);
    }
    match current_block {
        7427571413727699167 => {
            ExtractPalettedAlphaRows(dec, if row > last_row { last_row } else { row });
        }
        _ => {}
    }
    (*br).eos_ = VP8LIsEndOfStream(br);
    if ok == 0 || (*br).eos_ != 0 && pos < end {
        ok = 0 as ::core::ffi::c_int;
        (*dec).status_ = (if (*br).eos_ != 0 {
            VP8_STATUS_SUSPENDED as ::core::ffi::c_int
        } else {
            VP8_STATUS_BITSTREAM_ERROR as ::core::ffi::c_int
        }) as VP8StatusCode;
    } else {
        (*dec).last_pixel_ = pos;
    }
    return ok;
}
unsafe extern "C" fn SaveState(dec: *mut VP8LDecoder, mut last_pixel: ::core::ffi::c_int) {
    '_c2rust_label: {
        if (*dec).incremental_ != 0 {
        } else {
            __assert_fail(
                b"dec->incremental_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1099 as ::core::ffi::c_uint,
                b"void SaveState(VP8LDecoder *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*dec).saved_br_ = (*dec).br_;
    (*dec).saved_last_pixel_ = last_pixel;
    if (*dec).hdr_.color_cache_size_ > 0 as ::core::ffi::c_int {
        VP8LColorCacheCopy(
            &raw mut (*dec).hdr_.color_cache_,
            &raw mut (*dec).hdr_.saved_color_cache_,
        );
    }
}
unsafe extern "C" fn RestoreState(dec: *mut VP8LDecoder) {
    '_c2rust_label: {
        if (*dec).br_.eos_ != 0 {
        } else {
            __assert_fail(
                b"dec->br_.eos_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1108 as ::core::ffi::c_uint,
                b"void RestoreState(VP8LDecoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*dec).status_ = VP8_STATUS_SUSPENDED;
    (*dec).br_ = (*dec).saved_br_;
    (*dec).last_pixel_ = (*dec).saved_last_pixel_;
    if (*dec).hdr_.color_cache_size_ > 0 as ::core::ffi::c_int {
        VP8LColorCacheCopy(
            &raw mut (*dec).hdr_.saved_color_cache_,
            &raw mut (*dec).hdr_.color_cache_,
        );
    }
}
pub const SYNC_EVERY_N_ROWS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
unsafe extern "C" fn DecodeImageData(
    dec: *mut VP8LDecoder,
    data: *mut uint32_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut last_row: ::core::ffi::c_int,
    mut process_func: ProcessRowsFunc,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut row: ::core::ffi::c_int = (*dec).last_pixel_ / width;
    let mut col: ::core::ffi::c_int = (*dec).last_pixel_ % width;
    let br: *mut VP8LBitReader = &raw mut (*dec).br_;
    let hdr: *mut VP8LMetadata = &raw mut (*dec).hdr_;
    let mut src: *mut uint32_t = data.offset((*dec).last_pixel_ as isize);
    let mut last_cached: *mut uint32_t = src;
    let src_end: *mut uint32_t = data.offset((width * height) as isize);
    let src_last: *mut uint32_t = data.offset((width * last_row) as isize);
    let len_code_limit: ::core::ffi::c_int = NUM_LITERAL_CODES + NUM_LENGTH_CODES;
    let color_cache_limit: ::core::ffi::c_int = len_code_limit + (*hdr).color_cache_size_;
    let mut next_sync_row: ::core::ffi::c_int = if (*dec).incremental_ != 0 {
        row
    } else {
        (1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
    };
    let color_cache: *mut VP8LColorCache = if (*hdr).color_cache_size_ > 0 as ::core::ffi::c_int {
        &raw mut (*hdr).color_cache_
    } else {
        ::core::ptr::null_mut::<VP8LColorCache>()
    };
    let mask: ::core::ffi::c_int = (*hdr).huffman_mask_;
    let mut htree_group: *const HTreeGroup = if src < src_last {
        GetHtreeGroupForPos(hdr, col, row)
    } else {
        ::core::ptr::null_mut::<HTreeGroup>()
    };
    '_c2rust_label: {
        if (*dec).last_row_ < last_row {
        } else {
            __assert_fail(
                b"dec->last_row_ < last_row\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1137 as ::core::ffi::c_uint,
                b"int DecodeImageData(VP8LDecoder *const, uint32_t *const, int, int, int, ProcessRowsFunc)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if src_last <= src_end {
        } else {
            __assert_fail(
                b"src_last <= src_end\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1138 as ::core::ffi::c_uint,
                b"int DecodeImageData(VP8LDecoder *const, uint32_t *const, int, int, int, ProcessRowsFunc)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    loop {
        if !(src < src_last) {
            current_block = 10435735846551762309;
            break;
        }
        let mut code: ::core::ffi::c_int = 0;
        if row >= next_sync_row {
            SaveState(
                dec,
                src.offset_from(data) as ::core::ffi::c_long as ::core::ffi::c_int,
            );
            next_sync_row = row + SYNC_EVERY_N_ROWS;
        }
        if col & mask == 0 as ::core::ffi::c_int {
            htree_group = GetHtreeGroupForPos(hdr, col, row);
        }
        '_c2rust_label_1: {
            if !htree_group.is_null() {
            } else {
                __assert_fail(
                    b"htree_group != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1152 as ::core::ffi::c_uint,
                    b"int DecodeImageData(VP8LDecoder *const, uint32_t *const, int, int, int, ProcessRowsFunc)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        if (*htree_group).is_trivial_code != 0 {
            *src = (*htree_group).literal_arb;
        } else {
            VP8LFillBitWindow(br);
            if (*htree_group).use_packed_table != 0 {
                code = ReadPackedSymbols(htree_group, br, src);
                if VP8LIsEndOfStream(br) != 0 {
                    current_block = 10435735846551762309;
                    break;
                }
                if code == PACKED_NON_LITERAL_CODE {
                    current_block = 3852799436481433328;
                } else {
                    current_block = 2668756484064249700;
                }
            } else {
                code = ReadSymbol(
                    (*htree_group).htrees[GREEN as ::core::ffi::c_int as usize],
                    br,
                );
                current_block = 2668756484064249700;
            }
            match current_block {
                3852799436481433328 => {}
                _ => {
                    if VP8LIsEndOfStream(br) != 0 {
                        current_block = 10435735846551762309;
                        break;
                    }
                    if code < NUM_LITERAL_CODES {
                        if (*htree_group).is_trivial_literal != 0 {
                            *src = (*htree_group).literal_arb
                                | (code << 8 as ::core::ffi::c_int) as uint32_t;
                        } else {
                            let mut red: ::core::ffi::c_int = 0;
                            let mut blue: ::core::ffi::c_int = 0;
                            let mut alpha: ::core::ffi::c_int = 0;
                            red = ReadSymbol(
                                (*htree_group).htrees[RED as ::core::ffi::c_int as usize],
                                br,
                            );
                            VP8LFillBitWindow(br);
                            blue = ReadSymbol(
                                (*htree_group).htrees[BLUE as ::core::ffi::c_int as usize],
                                br,
                            );
                            alpha = ReadSymbol(
                                (*htree_group).htrees[ALPHA as ::core::ffi::c_int as usize],
                                br,
                            );
                            if VP8LIsEndOfStream(br) != 0 {
                                current_block = 10435735846551762309;
                                break;
                            }
                            *src = (alpha as uint32_t) << 24 as ::core::ffi::c_int
                                | (red << 16 as ::core::ffi::c_int) as uint32_t
                                | (code << 8 as ::core::ffi::c_int) as uint32_t
                                | blue as uint32_t;
                        }
                    } else if code < len_code_limit {
                        let mut dist_code: ::core::ffi::c_int = 0;
                        let mut dist: ::core::ffi::c_int = 0;
                        let length_sym: ::core::ffi::c_int = code - NUM_LITERAL_CODES;
                        let length: ::core::ffi::c_int =
                            GetCopyLength(length_sym, br) as ::core::ffi::c_int;
                        let dist_symbol: ::core::ffi::c_int = ReadSymbol(
                            (*htree_group).htrees[DIST as ::core::ffi::c_int as usize],
                            br,
                        )
                            as ::core::ffi::c_int;
                        VP8LFillBitWindow(br);
                        dist_code = GetCopyDistance(dist_symbol, br);
                        dist = PlaneCodeToDistance(width, dist_code);
                        if VP8LIsEndOfStream(br) != 0 {
                            current_block = 10435735846551762309;
                            break;
                        }
                        if (src.offset_from(data) as ptrdiff_t) < dist as ptrdiff_t
                            || (src_end.offset_from(src) as ptrdiff_t) < length as ptrdiff_t
                        {
                            current_block = 8191066059507257934;
                            break;
                        }
                        CopyBlock32b(src, dist, length);
                        src = src.offset(length as isize);
                        col += length;
                        while col >= width {
                            col -= width;
                            row += 1;
                            if process_func.is_some() {
                                if row <= last_row
                                    && row % NUM_ARGB_CACHE_ROWS == 0 as ::core::ffi::c_int
                                {
                                    process_func.expect("non-null function pointer")(dec, row);
                                }
                            }
                        }
                        '_c2rust_label_2: {
                            if src <= src_end {
                            } else {
                                __assert_fail(
                                    b"src <= src_end\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    1223 as ::core::ffi::c_uint,
                                    b"int DecodeImageData(VP8LDecoder *const, uint32_t *const, int, int, int, ProcessRowsFunc)\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                );
                            }
                        };
                        if col & mask != 0 {
                            htree_group = GetHtreeGroupForPos(hdr, col, row);
                        }
                        if !color_cache.is_null() {
                            while last_cached < src {
                                let fresh1 = last_cached;
                                last_cached = last_cached.offset(1);
                                VP8LColorCacheInsert(color_cache, *fresh1);
                            }
                        }
                        continue;
                    } else {
                        if !(code < color_cache_limit) {
                            current_block = 8191066059507257934;
                            break;
                        }
                        let key: ::core::ffi::c_int = code - len_code_limit;
                        '_c2rust_label_3: {
                            if !color_cache.is_null() {
                            } else {
                                __assert_fail(
                                    b"color_cache != NULL\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    1232 as ::core::ffi::c_uint,
                                    b"int DecodeImageData(VP8LDecoder *const, uint32_t *const, int, int, int, ProcessRowsFunc)\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                );
                            }
                        };
                        while last_cached < src {
                            let fresh2 = last_cached;
                            last_cached = last_cached.offset(1);
                            VP8LColorCacheInsert(color_cache, *fresh2);
                        }
                        *src = VP8LColorCacheLookup(color_cache, key as uint32_t);
                    }
                }
            }
        }
        src = src.offset(1);
        col += 1;
        if col >= width {
            col = 0 as ::core::ffi::c_int;
            row += 1;
            if process_func.is_some() {
                if row <= last_row && row % NUM_ARGB_CACHE_ROWS == 0 as ::core::ffi::c_int {
                    process_func.expect("non-null function pointer")(dec, row);
                }
            }
            if !color_cache.is_null() {
                while last_cached < src {
                    let fresh0 = last_cached;
                    last_cached = last_cached.offset(1);
                    VP8LColorCacheInsert(color_cache, *fresh0);
                }
            }
        }
    }
    match current_block {
        10435735846551762309 => {
            (*br).eos_ = VP8LIsEndOfStream(br);
            '_c2rust_label_4: {
                if (*dec).incremental_ == 0 || (*br).eos_ != 0 && src < src_last || src >= src_last
                {
                } else {
                    __assert_fail(
                        b"!dec->incremental_ || (br->eos_ && src < src_last) || src >= src_last\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        1254 as ::core::ffi::c_uint,
                        b"int DecodeImageData(VP8LDecoder *const, uint32_t *const, int, int, int, ProcessRowsFunc)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            if (*dec).incremental_ != 0 && (*br).eos_ != 0 && src < src_last {
                RestoreState(dec);
                current_block = 8062065914618164218;
            } else if (*dec).incremental_ != 0 && src >= src_last || (*br).eos_ == 0 {
                if process_func.is_some() {
                    process_func.expect("non-null function pointer")(
                        dec,
                        if row > last_row { last_row } else { row },
                    );
                }
                (*dec).status_ = VP8_STATUS_OK;
                (*dec).last_pixel_ =
                    src.offset_from(data) as ::core::ffi::c_long as ::core::ffi::c_int;
                current_block = 8062065914618164218;
            } else {
                current_block = 8191066059507257934;
            }
            match current_block {
                8191066059507257934 => {}
                _ => return 1 as ::core::ffi::c_int,
            }
        }
        _ => {}
    }
    (*dec).status_ = VP8_STATUS_BITSTREAM_ERROR;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ClearTransform(transform: *mut VP8LTransform) {
    WebPSafeFree((*transform).data_ as *mut ::core::ffi::c_void);
    (*transform).data_ = ::core::ptr::null_mut::<uint32_t>();
}
unsafe extern "C" fn ExpandColorMap(
    mut num_colors: ::core::ffi::c_int,
    transform: *mut VP8LTransform,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let final_num_colors: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << (8 as ::core::ffi::c_int >> (*transform).bits_);
    let new_color_map: *mut uint32_t = WebPSafeMalloc(
        final_num_colors as uint64_t,
        ::core::mem::size_of::<uint32_t>() as size_t,
    ) as *mut uint32_t;
    if new_color_map.is_null() {
        return 0 as ::core::ffi::c_int;
    } else {
        let data: *mut uint8_t = (*transform).data_ as *mut uint8_t;
        let new_data: *mut uint8_t = new_color_map as *mut uint8_t;
        *new_color_map.offset(0 as ::core::ffi::c_int as isize) =
            *(*transform).data_.offset(0 as ::core::ffi::c_int as isize);
        i = 4 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int * num_colors {
            *new_data.offset(i as isize) = (*data.offset(i as isize) as ::core::ffi::c_int
                + *new_data.offset((i - 4 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int) as uint8_t;
            i += 1;
        }
        while i < 4 as ::core::ffi::c_int * final_num_colors {
            *new_data.offset(i as isize) = 0 as uint8_t;
            i += 1;
        }
        WebPSafeFree((*transform).data_ as *mut ::core::ffi::c_void);
        (*transform).data_ = new_color_map;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn ReadTransform(
    xsize: *mut ::core::ffi::c_int,
    mut ysize: *const ::core::ffi::c_int,
    dec: *mut VP8LDecoder,
) -> ::core::ffi::c_int {
    let mut ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let br: *mut VP8LBitReader = &raw mut (*dec).br_;
    let mut transform: *mut VP8LTransform = (&raw mut (*dec).transforms_ as *mut VP8LTransform)
        .offset((*dec).next_transform_ as isize)
        as *mut VP8LTransform;
    let type_0: VP8LImageTransformType =
        VP8LReadBits(br, 2 as ::core::ffi::c_int) as VP8LImageTransformType;
    if (*dec).transforms_seen_ & (1 as uint32_t) << type_0 as ::core::ffi::c_uint != 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*dec).transforms_seen_ = ((*dec).transforms_seen_ as ::core::ffi::c_uint
        | (1 as ::core::ffi::c_uint) << type_0 as ::core::ffi::c_uint)
        as uint32_t;
    (*transform).type_ = type_0;
    (*transform).xsize_ = *xsize;
    (*transform).ysize_ = *ysize;
    (*transform).data_ = ::core::ptr::null_mut::<uint32_t>();
    (*dec).next_transform_ += 1;
    '_c2rust_label: {
        if (*dec).next_transform_ <= 4 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"dec->next_transform_ <= NUM_TRANSFORMS\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1330 as ::core::ffi::c_uint,
                b"int ReadTransform(int *const, const int *, VP8LDecoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    match type_0 as ::core::ffi::c_uint {
        0 | 1 => {
            (*transform).bits_ = VP8LReadBits(br, 3 as ::core::ffi::c_int)
                .wrapping_add(2 as uint32_t) as ::core::ffi::c_int;
            ok = DecodeImageStream(
                VP8LSubSampleSize(
                    (*transform).xsize_ as uint32_t,
                    (*transform).bits_ as uint32_t,
                ) as ::core::ffi::c_int,
                VP8LSubSampleSize(
                    (*transform).ysize_ as uint32_t,
                    (*transform).bits_ as uint32_t,
                ) as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dec,
                &raw mut (*transform).data_,
            );
        }
        3 => {
            let num_colors: ::core::ffi::c_int = VP8LReadBits(br, 8 as ::core::ffi::c_int)
                .wrapping_add(1 as uint32_t)
                as ::core::ffi::c_int;
            let bits: ::core::ffi::c_int = if num_colors > 16 as ::core::ffi::c_int {
                0 as ::core::ffi::c_int
            } else if num_colors > 4 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else if num_colors > 2 as ::core::ffi::c_int {
                2 as ::core::ffi::c_int
            } else {
                3 as ::core::ffi::c_int
            };
            *xsize = VP8LSubSampleSize((*transform).xsize_ as uint32_t, bits as uint32_t)
                as ::core::ffi::c_int;
            (*transform).bits_ = bits;
            ok = DecodeImageStream(
                num_colors,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dec,
                &raw mut (*transform).data_,
            );
            ok = (ok != 0 && ExpandColorMap(num_colors, transform) != 0) as ::core::ffi::c_int;
        }
        2 => {}
        _ => {
            '_c2rust_label_0: {
                __assert_fail(
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1357 as ::core::ffi::c_uint,
                    b"int ReadTransform(int *const, const int *, VP8LDecoder *const)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            };
        }
    }
    return ok;
}
unsafe extern "C" fn InitMetadata(hdr: *mut VP8LMetadata) {
    '_c2rust_label: {
        if !hdr.is_null() {
        } else {
            __assert_fail(
                b"hdr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1368 as ::core::ffi::c_uint,
                b"void InitMetadata(VP8LMetadata *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    memset(
        hdr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VP8LMetadata>() as size_t,
    );
}
unsafe extern "C" fn ClearMetadata(hdr: *mut VP8LMetadata) {
    '_c2rust_label: {
        if !hdr.is_null() {
        } else {
            __assert_fail(
                b"hdr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1373 as ::core::ffi::c_uint,
                b"void ClearMetadata(VP8LMetadata *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    WebPSafeFree((*hdr).huffman_image_ as *mut ::core::ffi::c_void);
    VP8LHuffmanTablesDeallocate(&raw mut (*hdr).huffman_tables_);
    VP8LHtreeGroupsFree((*hdr).htree_groups_);
    VP8LColorCacheClear(&raw mut (*hdr).color_cache_);
    VP8LColorCacheClear(&raw mut (*hdr).saved_color_cache_);
    InitMetadata(hdr);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LNew() -> *mut VP8LDecoder {
    let dec: *mut VP8LDecoder = WebPSafeCalloc(
        1 as uint64_t,
        ::core::mem::size_of::<VP8LDecoder>() as size_t,
    ) as *mut VP8LDecoder;
    if dec.is_null() {
        return ::core::ptr::null_mut::<VP8LDecoder>();
    }
    (*dec).status_ = VP8_STATUS_OK;
    (*dec).state_ = READ_DIM;
    VP8LDspInit();
    return dec;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LClear(dec: *mut VP8LDecoder) {
    let mut i: ::core::ffi::c_int = 0;
    if dec.is_null() {
        return;
    }
    ClearMetadata(&raw mut (*dec).hdr_);
    WebPSafeFree((*dec).pixels_ as *mut ::core::ffi::c_void);
    (*dec).pixels_ = ::core::ptr::null_mut::<uint32_t>();
    i = 0 as ::core::ffi::c_int;
    while i < (*dec).next_transform_ {
        ClearTransform(
            (&raw mut (*dec).transforms_ as *mut VP8LTransform).offset(i as isize)
                as *mut VP8LTransform,
        );
        i += 1;
    }
    (*dec).next_transform_ = 0 as ::core::ffi::c_int;
    (*dec).transforms_seen_ = 0 as uint32_t;
    WebPSafeFree((*dec).rescaler_memory as *mut ::core::ffi::c_void);
    (*dec).rescaler_memory = ::core::ptr::null_mut::<uint8_t>();
    (*dec).output_ = ::core::ptr::null::<WebPDecBuffer>();
}
#[no_mangle]
pub unsafe extern "C" fn VP8LDelete(dec: *mut VP8LDecoder) {
    if !dec.is_null() {
        VP8LClear(dec);
        WebPSafeFree(dec as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn UpdateDecoder(
    dec: *mut VP8LDecoder,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    let hdr: *mut VP8LMetadata = &raw mut (*dec).hdr_;
    let num_bits: ::core::ffi::c_int = (*hdr).huffman_subsample_bits_;
    (*dec).width_ = width;
    (*dec).height_ = height;
    (*hdr).huffman_xsize_ =
        VP8LSubSampleSize(width as uint32_t, num_bits as uint32_t) as ::core::ffi::c_int;
    (*hdr).huffman_mask_ = if num_bits == 0 as ::core::ffi::c_int {
        !(0 as ::core::ffi::c_int)
    } else {
        ((1 as ::core::ffi::c_int) << num_bits) - 1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn DecodeImageStream(
    mut xsize: ::core::ffi::c_int,
    mut ysize: ::core::ffi::c_int,
    mut is_level0: ::core::ffi::c_int,
    dec: *mut VP8LDecoder,
    decoded_data: *mut *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut transform_xsize: ::core::ffi::c_int = xsize;
    let mut transform_ysize: ::core::ffi::c_int = ysize;
    let br: *mut VP8LBitReader = &raw mut (*dec).br_;
    let hdr: *mut VP8LMetadata = &raw mut (*dec).hdr_;
    let mut data: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut color_cache_bits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if is_level0 != 0 {
        while ok != 0 && VP8LReadBits(br, 1 as ::core::ffi::c_int) != 0 {
            ok = ReadTransform(&raw mut transform_xsize, &raw mut transform_ysize, dec);
        }
    }
    if ok != 0 && VP8LReadBits(br, 1 as ::core::ffi::c_int) != 0 {
        color_cache_bits = VP8LReadBits(br, 4 as ::core::ffi::c_int) as ::core::ffi::c_int;
        ok = (color_cache_bits >= 1 as ::core::ffi::c_int && color_cache_bits <= MAX_CACHE_BITS)
            as ::core::ffi::c_int;
        if ok == 0 {
            (*dec).status_ = VP8_STATUS_BITSTREAM_ERROR;
            current_block = 10706502177519779249;
        } else {
            current_block = 2968425633554183086;
        }
    } else {
        current_block = 2968425633554183086;
    }
    match current_block {
        2968425633554183086 => {
            ok = (ok != 0
                && ReadHuffmanCodes(
                    dec,
                    transform_xsize,
                    transform_ysize,
                    color_cache_bits,
                    is_level0,
                ) != 0) as ::core::ffi::c_int;
            if ok == 0 {
                (*dec).status_ = VP8_STATUS_BITSTREAM_ERROR;
            } else {
                if color_cache_bits > 0 as ::core::ffi::c_int {
                    (*hdr).color_cache_size_ = (1 as ::core::ffi::c_int) << color_cache_bits;
                    if VP8LColorCacheInit(&raw mut (*hdr).color_cache_, color_cache_bits) == 0 {
                        (*dec).status_ = VP8_STATUS_OUT_OF_MEMORY;
                        ok = 0 as ::core::ffi::c_int;
                        current_block = 10706502177519779249;
                    } else {
                        current_block = 26972500619410423;
                    }
                } else {
                    (*hdr).color_cache_size_ = 0 as ::core::ffi::c_int;
                    current_block = 26972500619410423;
                }
                match current_block {
                    10706502177519779249 => {}
                    _ => {
                        UpdateDecoder(dec, transform_xsize, transform_ysize);
                        if is_level0 != 0 {
                            (*dec).state_ = READ_HDR;
                        } else {
                            let total_size: uint64_t = (transform_xsize as uint64_t)
                                .wrapping_mul(transform_ysize as uint64_t);
                            data = WebPSafeMalloc(
                                total_size,
                                ::core::mem::size_of::<uint32_t>() as size_t,
                            ) as *mut uint32_t;
                            if data.is_null() {
                                (*dec).status_ = VP8_STATUS_OUT_OF_MEMORY;
                                ok = 0 as ::core::ffi::c_int;
                            } else {
                                ok = DecodeImageData(
                                    dec,
                                    data,
                                    transform_xsize,
                                    transform_ysize,
                                    transform_ysize,
                                    None,
                                );
                                ok = (ok != 0 && (*br).eos_ == 0) as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if ok == 0 {
        WebPSafeFree(data as *mut ::core::ffi::c_void);
        ClearMetadata(hdr);
    } else {
        if !decoded_data.is_null() {
            *decoded_data = data;
        } else {
            '_c2rust_label: {
                if data.is_null() {
                } else {
                    __assert_fail(
                        b"data == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        1513 as ::core::ffi::c_uint,
                        b"int DecodeImageStream(int, int, int, VP8LDecoder *const, uint32_t **const)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            '_c2rust_label_0: {
                if is_level0 != 0 {
                } else {
                    __assert_fail(
                        b"is_level0\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        1514 as ::core::ffi::c_uint,
                        b"int DecodeImageStream(int, int, int, VP8LDecoder *const, uint32_t **const)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
        }
        (*dec).last_pixel_ = 0 as ::core::ffi::c_int;
        if is_level0 == 0 {
            ClearMetadata(hdr);
        }
    }
    return ok;
}
unsafe extern "C" fn AllocateInternalBuffers32b(
    dec: *mut VP8LDecoder,
    mut final_width: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let num_pixels: uint64_t = ((*dec).width_ as uint64_t).wrapping_mul((*dec).height_ as uint64_t);
    let cache_top_pixels: uint64_t = final_width as uint16_t as uint64_t;
    let cache_pixels: uint64_t =
        (final_width as uint64_t).wrapping_mul(NUM_ARGB_CACHE_ROWS as uint64_t);
    let total_num_pixels: uint64_t = num_pixels
        .wrapping_add(cache_top_pixels)
        .wrapping_add(cache_pixels);
    '_c2rust_label: {
        if (*dec).width_ <= final_width {
        } else {
            __assert_fail(
                b"dec->width_ <= final_width\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1534 as ::core::ffi::c_uint,
                b"int AllocateInternalBuffers32b(VP8LDecoder *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*dec).pixels_ = WebPSafeMalloc(
        total_num_pixels,
        ::core::mem::size_of::<uint32_t>() as size_t,
    ) as *mut uint32_t;
    if (*dec).pixels_.is_null() {
        (*dec).argb_cache_ = ::core::ptr::null_mut::<uint32_t>();
        (*dec).status_ = VP8_STATUS_OUT_OF_MEMORY;
        return 0 as ::core::ffi::c_int;
    }
    (*dec).argb_cache_ = (*dec)
        .pixels_
        .offset(num_pixels as isize)
        .offset(cache_top_pixels as isize);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn AllocateInternalBuffers8b(dec: *mut VP8LDecoder) -> ::core::ffi::c_int {
    let total_num_pixels: uint64_t =
        ((*dec).width_ as uint64_t).wrapping_mul((*dec).height_ as uint64_t);
    (*dec).argb_cache_ = ::core::ptr::null_mut::<uint32_t>();
    (*dec).pixels_ = WebPSafeMalloc(
        total_num_pixels,
        ::core::mem::size_of::<uint8_t>() as size_t,
    ) as *mut uint32_t;
    if (*dec).pixels_.is_null() {
        (*dec).status_ = VP8_STATUS_OUT_OF_MEMORY;
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn ExtractAlphaRows(dec: *mut VP8LDecoder, mut last_row: ::core::ffi::c_int) {
    let mut cur_row: ::core::ffi::c_int = (*dec).last_row_;
    let mut num_rows: ::core::ffi::c_int = last_row - cur_row;
    let mut in_0: *const uint32_t = (*dec).pixels_.offset(((*dec).width_ * cur_row) as isize);
    '_c2rust_label: {
        if last_row <= (*(*dec).io_).crop_bottom {
        } else {
            __assert_fail(
                b"last_row <= dec->io_->crop_bottom\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1564 as ::core::ffi::c_uint,
                b"void ExtractAlphaRows(VP8LDecoder *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    while num_rows > 0 as ::core::ffi::c_int {
        let num_rows_to_process: ::core::ffi::c_int = if num_rows > NUM_ARGB_CACHE_ROWS {
            NUM_ARGB_CACHE_ROWS
        } else {
            num_rows
        };
        let alph_dec: *mut ALPHDecoder = (*(*dec).io_).opaque as *mut ALPHDecoder;
        let output: *mut uint8_t = (*alph_dec).output_;
        let width: ::core::ffi::c_int = (*(*dec).io_).width;
        let cache_pixs: ::core::ffi::c_int = width * num_rows_to_process;
        let dst: *mut uint8_t = output.offset((width * cur_row) as isize);
        let src: *const uint32_t = (*dec).argb_cache_;
        ApplyInverseTransforms(dec, cur_row, num_rows_to_process, in_0);
        WebPExtractGreen.expect("non-null function pointer")(src, dst, cache_pixs);
        AlphaApplyFilter(alph_dec, cur_row, cur_row + num_rows_to_process, dst, width);
        num_rows -= num_rows_to_process;
        in_0 = in_0.offset((num_rows_to_process * (*dec).width_) as isize);
        cur_row += num_rows_to_process;
    }
    '_c2rust_label_0: {
        if cur_row == last_row {
        } else {
            __assert_fail(
                b"cur_row == last_row\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1583 as ::core::ffi::c_uint,
                b"void ExtractAlphaRows(VP8LDecoder *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*dec).last_out_row_ = last_row;
    (*dec).last_row_ = (*dec).last_out_row_;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LDecodeAlphaHeader(
    alph_dec: *mut ALPHDecoder,
    data: *const uint8_t,
    mut data_size: size_t,
) -> ::core::ffi::c_int {
    let mut ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut dec: *mut VP8LDecoder = VP8LNew();
    if dec.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    '_c2rust_label: {
        if !alph_dec.is_null() {
        } else {
            __assert_fail(
                b"alph_dec != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1594 as ::core::ffi::c_uint,
                b"int VP8LDecodeAlphaHeader(ALPHDecoder *const, const uint8_t *const, size_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    (*dec).width_ = (*alph_dec).width_;
    (*dec).height_ = (*alph_dec).height_;
    (*dec).io_ = &raw mut (*alph_dec).io_;
    (*(*dec).io_).opaque = alph_dec as *mut ::core::ffi::c_void;
    (*(*dec).io_).width = (*alph_dec).width_;
    (*(*dec).io_).height = (*alph_dec).height_;
    (*dec).status_ = VP8_STATUS_OK;
    VP8LInitBitReader(&raw mut (*dec).br_, data, data_size);
    if !(DecodeImageStream(
        (*alph_dec).width_,
        (*alph_dec).height_,
        1 as ::core::ffi::c_int,
        dec,
        ::core::ptr::null_mut::<*mut uint32_t>(),
    ) == 0)
    {
        if (*dec).next_transform_ == 1 as ::core::ffi::c_int
            && (*dec).transforms_[0 as ::core::ffi::c_int as usize].type_ as ::core::ffi::c_uint
                == COLOR_INDEXING_TRANSFORM as ::core::ffi::c_int as ::core::ffi::c_uint
            && Is8bOptimizable(&raw mut (*dec).hdr_) != 0
        {
            (*alph_dec).use_8b_decode_ = 1 as ::core::ffi::c_int;
            ok = AllocateInternalBuffers8b(dec);
        } else {
            (*alph_dec).use_8b_decode_ = 0 as ::core::ffi::c_int;
            ok = AllocateInternalBuffers32b(dec, (*alph_dec).width_);
        }
        if !(ok == 0) {
            (*alph_dec).vp8l_dec_ = dec as *mut VP8LDecoder;
            return 1 as ::core::ffi::c_int;
        }
    }
    VP8LDelete(dec);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LDecodeAlphaImageStream(
    alph_dec: *mut ALPHDecoder,
    mut last_row: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let dec: *mut VP8LDecoder = (*alph_dec).vp8l_dec_ as *mut VP8LDecoder;
    '_c2rust_label: {
        if !dec.is_null() {
        } else {
            __assert_fail(
                b"dec != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1637 as ::core::ffi::c_uint,
                b"int VP8LDecodeAlphaImageStream(ALPHDecoder *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if last_row <= (*dec).height_ {
        } else {
            __assert_fail(
                b"last_row <= dec->height_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1638 as ::core::ffi::c_uint,
                b"int VP8LDecodeAlphaImageStream(ALPHDecoder *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*dec).last_row_ >= last_row {
        return 1 as ::core::ffi::c_int;
    }
    if (*alph_dec).use_8b_decode_ == 0 {
        WebPInitAlphaProcessing();
    }
    return if (*alph_dec).use_8b_decode_ != 0 {
        DecodeAlphaData(
            dec,
            (*dec).pixels_ as *mut uint8_t,
            (*dec).width_,
            (*dec).height_,
            last_row,
        )
    } else {
        DecodeImageData(
            dec,
            (*dec).pixels_,
            (*dec).width_,
            (*dec).height_,
            last_row,
            Some(
                ExtractAlphaRows
                    as unsafe extern "C" fn(*mut VP8LDecoder, ::core::ffi::c_int) -> (),
            ),
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LDecodeHeader(
    dec: *mut VP8LDecoder,
    io: *mut VP8Io,
) -> ::core::ffi::c_int {
    let mut width: ::core::ffi::c_int = 0;
    let mut height: ::core::ffi::c_int = 0;
    let mut has_alpha: ::core::ffi::c_int = 0;
    if dec.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if io.is_null() {
        (*dec).status_ = VP8_STATUS_INVALID_PARAM;
        return 0 as ::core::ffi::c_int;
    }
    (*dec).io_ = io;
    (*dec).status_ = VP8_STATUS_OK;
    VP8LInitBitReader(&raw mut (*dec).br_, (*io).data, (*io).data_size);
    if ReadImageInfo(
        &raw mut (*dec).br_,
        &raw mut width,
        &raw mut height,
        &raw mut has_alpha,
    ) == 0
    {
        (*dec).status_ = VP8_STATUS_BITSTREAM_ERROR;
    } else {
        (*dec).state_ = READ_DIM;
        (*io).width = width;
        (*io).height = height;
        if !(DecodeImageStream(
            width,
            height,
            1 as ::core::ffi::c_int,
            dec,
            ::core::ptr::null_mut::<*mut uint32_t>(),
        ) == 0)
        {
            return 1 as ::core::ffi::c_int;
        }
    }
    VP8LClear(dec);
    '_c2rust_label: {
        if (*dec).status_ as ::core::ffi::c_uint
            != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"dec->status_ != VP8_STATUS_OK\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1681 as ::core::ffi::c_uint,
                b"int VP8LDecodeHeader(VP8LDecoder *const, VP8Io *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LDecodeImage(dec: *mut VP8LDecoder) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut io: *mut VP8Io = ::core::ptr::null_mut::<VP8Io>();
    let mut params: *mut WebPDecParams = ::core::ptr::null_mut::<WebPDecParams>();
    if dec.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    '_c2rust_label: {
        if !(*dec).hdr_.huffman_tables_.root.start.is_null() {
        } else {
            __assert_fail(
                b"dec->hdr_.huffman_tables_.root.start != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1691 as ::core::ffi::c_uint,
                b"int VP8LDecodeImage(VP8LDecoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !(*dec).hdr_.htree_groups_.is_null() {
        } else {
            __assert_fail(
                b"dec->hdr_.htree_groups_ != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1692 as ::core::ffi::c_uint,
                b"int VP8LDecodeImage(VP8LDecoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*dec).hdr_.num_htree_groups_ > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"dec->hdr_.num_htree_groups_ > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1693 as ::core::ffi::c_uint,
                b"int VP8LDecodeImage(VP8LDecoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    io = (*dec).io_;
    '_c2rust_label_2: {
        if !io.is_null() {
        } else {
            __assert_fail(
                b"io != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1696 as ::core::ffi::c_uint,
                b"int VP8LDecodeImage(VP8LDecoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    params = (*io).opaque as *mut WebPDecParams;
    '_c2rust_label_3: {
        if !params.is_null() {
        } else {
            __assert_fail(
                b"params != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1698 as ::core::ffi::c_uint,
                b"int VP8LDecodeImage(VP8LDecoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*dec).state_ as ::core::ffi::c_uint
        != READ_DATA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*dec).output_ = (*params).output;
        '_c2rust_label_4: {
            if !(*dec).output_.is_null() {
            } else {
                __assert_fail(
                    b"dec->output_ != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1703 as ::core::ffi::c_uint,
                    b"int VP8LDecodeImage(VP8LDecoder *const)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        if WebPIoInitFromOptions((*params).options, io, MODE_BGRA) == 0 {
            (*dec).status_ = VP8_STATUS_INVALID_PARAM;
            current_block = 11311682206942702048;
        } else if AllocateInternalBuffers32b(dec, (*io).width) == 0 {
            current_block = 11311682206942702048;
        } else if (*io).use_scaling != 0 && AllocateAndInitRescaler(dec, io) == 0 {
            current_block = 11311682206942702048;
        } else {
            if (*io).use_scaling != 0 || WebPIsPremultipliedMode((*(*dec).output_).colorspace) != 0
            {
                WebPInitAlphaProcessing();
            }
            if WebPIsRGBMode((*(*dec).output_).colorspace) == 0 {
                WebPInitConvertARGBToYUV();
                if !(*(*dec).output_).u.YUVA.a.is_null() {
                    WebPInitAlphaProcessing();
                }
            }
            if (*dec).incremental_ != 0 {
                if (*dec).hdr_.color_cache_size_ > 0 as ::core::ffi::c_int
                    && (*dec).hdr_.saved_color_cache_.colors_.is_null()
                {
                    if VP8LColorCacheInit(
                        &raw mut (*dec).hdr_.saved_color_cache_,
                        (*dec).hdr_.color_cache_.hash_bits_,
                    ) == 0
                    {
                        (*dec).status_ = VP8_STATUS_OUT_OF_MEMORY;
                        current_block = 11311682206942702048;
                    } else {
                        current_block = 5634871135123216486;
                    }
                } else {
                    current_block = 5634871135123216486;
                }
            } else {
                current_block = 5634871135123216486;
            }
            match current_block {
                11311682206942702048 => {}
                _ => {
                    (*dec).state_ = READ_DATA;
                    current_block = 17478428563724192186;
                }
            }
        }
    } else {
        current_block = 17478428563724192186;
    }
    match current_block {
        17478428563724192186 => {
            if !(DecodeImageData(
                dec,
                (*dec).pixels_,
                (*dec).width_,
                (*dec).height_,
                (*io).crop_bottom,
                Some(
                    ProcessRows as unsafe extern "C" fn(*mut VP8LDecoder, ::core::ffi::c_int) -> (),
                ),
            ) == 0)
            {
                (*params).last_y = (*dec).last_out_row_;
                return 1 as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    VP8LClear(dec);
    '_c2rust_label_5: {
        if (*dec).status_ as ::core::ffi::c_uint
            != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"dec->status_ != VP8_STATUS_OK\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/vp8l_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1753 as ::core::ffi::c_uint,
                b"int VP8LDecodeImage(VP8LDecoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return 0 as ::core::ffi::c_int;
}
