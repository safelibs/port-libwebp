#[repr(C)]
pub struct PixOrCopyBlock {
    _unused: [u8; 0],
}

extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn bsearch(
        __key: *const ::core::ffi::c_void,
        __base: *const ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    ) -> *mut ::core::ffi::c_void;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn VP8LRefsCursorInit(refs: *const VP8LBackwardRefs) -> VP8LRefsCursor;
    fn VP8LRefsCursorNextBlock(c: *mut VP8LRefsCursor);
    fn WebPEncodingSetError(
        pic: *const WebPPicture,
        error: WebPEncodingError,
    ) -> ::core::ffi::c_int;
    fn WebPReportProgress(
        pic: *const WebPPicture,
        percent: ::core::ffi::c_int,
        percent_store: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut VP8LExtraCost: VP8LCostFunc;
    static mut VP8LExtraCostCombined: VP8LCostCombinedFunc;
    fn VP8LBitEntropyInit(entropy: *mut VP8LBitEntropy);
    static mut VP8LGetCombinedEntropyUnrefined: VP8LGetCombinedEntropyUnrefinedFunc;
    static mut VP8LGetEntropyUnrefined: VP8LGetEntropyUnrefinedFunc;
    fn VP8LBitsEntropyUnrefined(
        array: *const uint32_t,
        n: ::core::ffi::c_int,
        entropy: *mut VP8LBitEntropy,
    );
    fn VP8LHistogramAdd(a: *const VP8LHistogram, b: *const VP8LHistogram, out: *mut VP8LHistogram);
    static kPrefixEncodeCode: [VP8LPrefixCode; 512];
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
}
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type size_t = usize;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = usize;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
pub type Mode = ::core::ffi::c_uint;
pub const kNone: Mode = 3;
pub const kCopy: Mode = 2;
pub const kCacheIdx: Mode = 1;
pub const kLiteral: Mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PixOrCopy {
    pub mode: uint8_t,
    pub len: uint16_t,
    pub argb_or_distance: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LBackwardRefs {
    pub block_size_: ::core::ffi::c_int,
    pub error_: ::core::ffi::c_int,
    pub refs_: *mut PixOrCopyBlock,
    pub tail_: *mut *mut PixOrCopyBlock,
    pub free_blocks_: *mut PixOrCopyBlock,
    pub last_block_: *mut PixOrCopyBlock,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LRefsCursor {
    pub cur_pos: *mut PixOrCopy,
    pub cur_block_: *mut PixOrCopyBlock,
    pub last_pos_: *const PixOrCopy,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LHistogram {
    pub literal_: *mut uint32_t,
    pub red_: [uint32_t; 256],
    pub blue_: [uint32_t; 256],
    pub alpha_: [uint32_t; 256],
    pub distance_: [uint32_t; 40],
    pub palette_code_bits_: ::core::ffi::c_int,
    pub trivial_symbol_: uint32_t,
    pub bit_cost_: ::core::ffi::c_float,
    pub literal_cost_: ::core::ffi::c_float,
    pub red_cost_: ::core::ffi::c_float,
    pub blue_cost_: ::core::ffi::c_float,
    pub is_used_: [uint8_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LHistogramSet {
    pub size: ::core::ffi::c_int,
    pub max_size: ::core::ffi::c_int,
    pub histograms: *mut *mut VP8LHistogram,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LPrefixCode {
    pub code_: int8_t,
    pub extra_bits_: int8_t,
}
pub type VP8LCostCombinedFunc = Option<
    unsafe extern "C" fn(
        *const uint32_t,
        *const uint32_t,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_float,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LStreaks {
    pub counts: [::core::ffi::c_int; 2],
    pub streaks: [[::core::ffi::c_int; 2]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LBitEntropy {
    pub entropy: ::core::ffi::c_float,
    pub sum: uint32_t,
    pub nonzeros: ::core::ffi::c_int,
    pub max_val: uint32_t,
    pub nonzero_code: uint32_t,
}
pub type VP8LGetEntropyUnrefinedFunc = Option<
    unsafe extern "C" fn(
        *const uint32_t,
        ::core::ffi::c_int,
        *mut VP8LBitEntropy,
        *mut VP8LStreaks,
    ) -> (),
>;
pub type VP8LGetCombinedEntropyUnrefinedFunc = Option<
    unsafe extern "C" fn(
        *const uint32_t,
        *const uint32_t,
        ::core::ffi::c_int,
        *mut VP8LBitEntropy,
        *mut VP8LStreaks,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistoQueue {
    pub queue: *mut HistogramPair,
    pub size: ::core::ffi::c_int,
    pub max_size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramPair {
    pub idx1: ::core::ffi::c_int,
    pub idx2: ::core::ffi::c_int,
    pub cost_diff: ::core::ffi::c_float,
    pub cost_combo: ::core::ffi::c_float,
}
pub type VP8LCostFunc =
    Option<unsafe extern "C" fn(*const uint32_t, ::core::ffi::c_int) -> ::core::ffi::c_float>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub first: int16_t,
    pub num_combine_failures: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DominantCostRange {
    pub literal_max_: ::core::ffi::c_float,
    pub literal_min_: ::core::ffi::c_float,
    pub red_max_: ::core::ffi::c_float,
    pub red_min_: ::core::ffi::c_float,
    pub blue_max_: ::core::ffi::c_float,
    pub blue_min_: ::core::ffi::c_float,
}
pub const NUM_LITERAL_CODES: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const NUM_LENGTH_CODES: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const NUM_DISTANCE_CODES: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const CODE_LENGTH_CODES: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn PixOrCopyIsLiteral(p: *const PixOrCopy) -> ::core::ffi::c_int {
    return ((*p).mode as ::core::ffi::c_int == kLiteral as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn PixOrCopyIsCacheIdx(p: *const PixOrCopy) -> ::core::ffi::c_int {
    return ((*p).mode as ::core::ffi::c_int == kCacheIdx as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn PixOrCopyLiteral(
    p: *const PixOrCopy,
    mut component: ::core::ffi::c_int,
) -> uint32_t {
    '_c2rust_label: {
        if (*p).mode as ::core::ffi::c_int == kLiteral as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"p->mode == kLiteral\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/enc/backward_references_enc.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                87 as ::core::ffi::c_uint,
                b"uint32_t PixOrCopyLiteral(const PixOrCopy *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return (*p).argb_or_distance >> component * 8 as ::core::ffi::c_int & 0xff as uint32_t;
}
#[inline]
unsafe extern "C" fn PixOrCopyLength(p: *const PixOrCopy) -> uint32_t {
    return (*p).len as uint32_t;
}
#[inline]
unsafe extern "C" fn PixOrCopyCacheIdx(p: *const PixOrCopy) -> uint32_t {
    '_c2rust_label: {
        if (*p).mode as ::core::ffi::c_int == kCacheIdx as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"p->mode == kCacheIdx\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/enc/backward_references_enc.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_uint,
                b"uint32_t PixOrCopyCacheIdx(const PixOrCopy *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*p).argb_or_distance < (1 as uint32_t) << 10 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"p->argb_or_distance < (1U << MAX_COLOR_CACHE_BITS)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"original/src/enc/backward_references_enc.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                97 as ::core::ffi::c_uint,
                b"uint32_t PixOrCopyCacheIdx(const PixOrCopy *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return (*p).argb_or_distance;
}
#[inline]
unsafe extern "C" fn PixOrCopyDistance(p: *const PixOrCopy) -> uint32_t {
    '_c2rust_label: {
        if (*p).mode as ::core::ffi::c_int == kCopy as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"p->mode == kCopy\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/enc/backward_references_enc.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                102 as ::core::ffi::c_uint,
                b"uint32_t PixOrCopyDistance(const PixOrCopy *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return (*p).argb_or_distance;
}
#[inline]
unsafe extern "C" fn VP8LRefsCursorOk(c: *const VP8LRefsCursor) -> ::core::ffi::c_int {
    return ((*c).cur_pos != NULL as *mut PixOrCopy) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn VP8LRefsCursorNext(c: *mut VP8LRefsCursor) {
    '_c2rust_label: {
        if !c.is_null() {
        } else {
            __assert_fail(
                b"c != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/enc/backward_references_enc.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                206 as ::core::ffi::c_uint,
                b"void VP8LRefsCursorNext(VP8LRefsCursor *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if VP8LRefsCursorOk(c) != 0 {
        } else {
            __assert_fail(
                b"VP8LRefsCursorOk(c)\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/enc/backward_references_enc.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                207 as ::core::ffi::c_uint,
                b"void VP8LRefsCursorNext(VP8LRefsCursor *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*c).cur_pos = (*c).cur_pos.offset(1);
    if (*c).cur_pos == (*c).last_pos_ as *mut PixOrCopy {
        VP8LRefsCursorNextBlock(c);
    }
}
pub const VP8L_NON_TRIVIAL_SYM: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn VP8LHistogramNumCodes(
    mut palette_code_bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return NUM_LITERAL_CODES
        + NUM_LENGTH_CODES
        + (if palette_code_bits > 0 as ::core::ffi::c_int {
            (1 as ::core::ffi::c_int) << palette_code_bits
        } else {
            0 as ::core::ffi::c_int
        });
}
pub const MAX_BIT_COST: ::core::ffi::c_float = FLT_MAX;
pub const NUM_PARTITIONS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const BIN_SIZE: ::core::ffi::c_int = NUM_PARTITIONS * NUM_PARTITIONS * NUM_PARTITIONS;
pub const MAX_HISTO_GREEDY: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
unsafe extern "C" fn HistogramClear(p: *mut VP8LHistogram) {
    let literal: *mut uint32_t = (*p).literal_;
    let cache_bits: ::core::ffi::c_int = (*p).palette_code_bits_;
    let histo_size: ::core::ffi::c_int = VP8LGetHistogramSize(cache_bits) as ::core::ffi::c_int;
    memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        histo_size as size_t,
    );
    (*p).palette_code_bits_ = cache_bits;
    (*p).literal_ = literal;
}
unsafe extern "C" fn HistogramSwap(A: *mut *mut VP8LHistogram, B: *mut *mut VP8LHistogram) {
    let tmp: *mut VP8LHistogram = *A;
    *A = *B;
    *B = tmp;
}
unsafe extern "C" fn HistogramCopy(src: *const VP8LHistogram, dst: *mut VP8LHistogram) {
    let dst_literal: *mut uint32_t = (*dst).literal_;
    let dst_cache_bits: ::core::ffi::c_int = (*dst).palette_code_bits_;
    let literal_size: ::core::ffi::c_int =
        VP8LHistogramNumCodes(dst_cache_bits) as ::core::ffi::c_int;
    let histo_size: ::core::ffi::c_int = VP8LGetHistogramSize(dst_cache_bits) as ::core::ffi::c_int;
    '_c2rust_label: {
        if (*src).palette_code_bits_ == dst_cache_bits {
        } else {
            __assert_fail(
                b"src->palette_code_bits_ == dst_cache_bits\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                58 as ::core::ffi::c_uint,
                b"void HistogramCopy(const VP8LHistogram *const, VP8LHistogram *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        histo_size as size_t,
    );
    (*dst).literal_ = dst_literal;
    memcpy(
        (*dst).literal_ as *mut ::core::ffi::c_void,
        (*src).literal_ as *const ::core::ffi::c_void,
        (literal_size as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn VP8LGetHistogramSize(
    mut cache_bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let literal_size: ::core::ffi::c_int = VP8LHistogramNumCodes(cache_bits) as ::core::ffi::c_int;
    let total_size: size_t = (::core::mem::size_of::<VP8LHistogram>() as size_t).wrapping_add(
        (::core::mem::size_of::<::core::ffi::c_int>() as size_t)
            .wrapping_mul(literal_size as size_t),
    );
    '_c2rust_label: {
        if total_size <= 0x7fffffff as ::core::ffi::c_int as size_t {
        } else {
            __assert_fail(
                b"total_size <= (size_t)0x7fffffff\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                67 as ::core::ffi::c_uint,
                b"int VP8LGetHistogramSize(int)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    return total_size as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LFreeHistogram(histo: *mut VP8LHistogram) {
    WebPSafeFree(histo as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LFreeHistogramSet(histo: *mut VP8LHistogramSet) {
    WebPSafeFree(histo as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHistogramStoreRefs(
    refs: *const VP8LBackwardRefs,
    histo: *mut VP8LHistogram,
) {
    let mut c: VP8LRefsCursor = VP8LRefsCursorInit(refs);
    while VP8LRefsCursorOk(&raw mut c) != 0 {
        VP8LHistogramAddSinglePixOrCopy(histo, c.cur_pos, None, 0 as ::core::ffi::c_int);
        VP8LRefsCursorNext(&raw mut c);
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHistogramCreate(
    p: *mut VP8LHistogram,
    refs: *const VP8LBackwardRefs,
    mut palette_code_bits: ::core::ffi::c_int,
) {
    if palette_code_bits >= 0 as ::core::ffi::c_int {
        (*p).palette_code_bits_ = palette_code_bits;
    }
    HistogramClear(p);
    VP8LHistogramStoreRefs(refs, p);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHistogramInit(
    p: *mut VP8LHistogram,
    mut palette_code_bits: ::core::ffi::c_int,
    mut init_arrays: ::core::ffi::c_int,
) {
    (*p).palette_code_bits_ = palette_code_bits;
    if init_arrays != 0 {
        HistogramClear(p);
    } else {
        (*p).trivial_symbol_ = 0 as uint32_t;
        (*p).bit_cost_ = 0.0f32;
        (*p).literal_cost_ = 0.0f32;
        (*p).red_cost_ = 0.0f32;
        (*p).blue_cost_ = 0.0f32;
        memset(
            &raw mut (*p).is_used_ as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[uint8_t; 5]>() as size_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LAllocateHistogram(
    mut cache_bits: ::core::ffi::c_int,
) -> *mut VP8LHistogram {
    let mut histo: *mut VP8LHistogram = ::core::ptr::null_mut::<VP8LHistogram>();
    let total_size: ::core::ffi::c_int = VP8LGetHistogramSize(cache_bits) as ::core::ffi::c_int;
    let memory: *mut uint8_t = WebPSafeMalloc(
        total_size as uint64_t,
        ::core::mem::size_of::<uint8_t>() as size_t,
    ) as *mut uint8_t;
    if memory.is_null() {
        return ::core::ptr::null_mut::<VP8LHistogram>();
    }
    histo = memory as *mut VP8LHistogram;
    (*histo).literal_ =
        memory.offset(::core::mem::size_of::<VP8LHistogram>() as usize as isize) as *mut uint32_t;
    VP8LHistogramInit(histo, cache_bits, 0 as ::core::ffi::c_int);
    return histo;
}
unsafe extern "C" fn HistogramSetResetPointers(
    set: *mut VP8LHistogramSet,
    mut cache_bits: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let histo_size: ::core::ffi::c_int = VP8LGetHistogramSize(cache_bits) as ::core::ffi::c_int;
    let mut memory: *mut uint8_t = (*set).histograms as *mut uint8_t;
    memory = memory.offset(
        ((*set).max_size as usize)
            .wrapping_mul(::core::mem::size_of::<*mut VP8LHistogram>() as usize) as isize,
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*set).max_size {
        memory = ((memory as uintptr_t).wrapping_add(WEBP_ALIGN_CST as uintptr_t)
            & !(WEBP_ALIGN_CST as uintptr_t)) as *mut uint8_t;
        let ref mut fresh3 = *(*set).histograms.offset(i as isize);
        *fresh3 = memory as *mut VP8LHistogram;
        let ref mut fresh4 = (**(*set).histograms.offset(i as isize)).literal_;
        *fresh4 = memory.offset(::core::mem::size_of::<VP8LHistogram>() as usize as isize)
            as *mut uint32_t;
        memory = memory.offset(histo_size as isize);
        i += 1;
    }
}
unsafe extern "C" fn HistogramSetTotalSize(
    mut size: ::core::ffi::c_int,
    mut cache_bits: ::core::ffi::c_int,
) -> size_t {
    let histo_size: ::core::ffi::c_int = VP8LGetHistogramSize(cache_bits) as ::core::ffi::c_int;
    return (::core::mem::size_of::<VP8LHistogramSet>() as size_t).wrapping_add(
        (size as size_t).wrapping_mul(
            (::core::mem::size_of::<*mut VP8LHistogram>() as size_t)
                .wrapping_add(histo_size as size_t)
                .wrapping_add(WEBP_ALIGN_CST as size_t),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn VP8LAllocateHistogramSet(
    mut size: ::core::ffi::c_int,
    mut cache_bits: ::core::ffi::c_int,
) -> *mut VP8LHistogramSet {
    let mut i: ::core::ffi::c_int = 0;
    let mut set: *mut VP8LHistogramSet = ::core::ptr::null_mut::<VP8LHistogramSet>();
    let total_size: size_t = HistogramSetTotalSize(size, cache_bits) as size_t;
    let mut memory: *mut uint8_t = WebPSafeMalloc(
        total_size as uint64_t,
        ::core::mem::size_of::<uint8_t>() as size_t,
    ) as *mut uint8_t;
    if memory.is_null() {
        return ::core::ptr::null_mut::<VP8LHistogramSet>();
    }
    set = memory as *mut VP8LHistogramSet;
    memory = memory.offset(::core::mem::size_of::<VP8LHistogramSet>() as usize as isize);
    (*set).histograms = memory as *mut *mut VP8LHistogram;
    (*set).max_size = size;
    (*set).size = size;
    HistogramSetResetPointers(set, cache_bits);
    i = 0 as ::core::ffi::c_int;
    while i < size {
        VP8LHistogramInit(
            *(*set).histograms.offset(i as isize),
            cache_bits,
            0 as ::core::ffi::c_int,
        );
        i += 1;
    }
    return set;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHistogramSetClear(set: *mut VP8LHistogramSet) {
    let mut i: ::core::ffi::c_int = 0;
    let cache_bits: ::core::ffi::c_int =
        (**(*set).histograms.offset(0 as ::core::ffi::c_int as isize)).palette_code_bits_;
    let size: ::core::ffi::c_int = (*set).max_size;
    let total_size: size_t = HistogramSetTotalSize(size, cache_bits) as size_t;
    let mut memory: *mut uint8_t = set as *mut uint8_t;
    memset(
        memory as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        total_size,
    );
    memory = memory.offset(::core::mem::size_of::<VP8LHistogramSet>() as usize as isize);
    (*set).histograms = memory as *mut *mut VP8LHistogram;
    (*set).max_size = size;
    (*set).size = size;
    HistogramSetResetPointers(set, cache_bits);
    i = 0 as ::core::ffi::c_int;
    while i < size {
        (**(*set).histograms.offset(i as isize)).palette_code_bits_ = cache_bits;
        i += 1;
    }
}
unsafe extern "C" fn HistogramSetRemoveHistogram(
    set: *mut VP8LHistogramSet,
    mut i: ::core::ffi::c_int,
    num_used: *mut ::core::ffi::c_int,
) {
    '_c2rust_label: {
        if !(*(*set).histograms.offset(i as isize)).is_null() {
        } else {
            __assert_fail(
                b"set->histograms[i] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                188 as ::core::ffi::c_uint,
                b"void HistogramSetRemoveHistogram(VP8LHistogramSet *const, int, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    let ref mut fresh8 = *(*set).histograms.offset(i as isize);
    *fresh8 = ::core::ptr::null_mut::<VP8LHistogram>();
    *num_used -= 1;
    if i == (*set).size - 1 as ::core::ffi::c_int {
        while (*set).size >= 1 as ::core::ffi::c_int
            && (*(*set)
                .histograms
                .offset(((*set).size - 1 as ::core::ffi::c_int) as isize))
            .is_null()
        {
            (*set).size -= 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHistogramAddSinglePixOrCopy(
    histo: *mut VP8LHistogram,
    v: *const PixOrCopy,
    distance_modifier: Option<
        unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    mut distance_modifier_arg0: ::core::ffi::c_int,
) {
    if PixOrCopyIsLiteral(v) != 0 {
        (*histo).alpha_[PixOrCopyLiteral(v, 3 as ::core::ffi::c_int) as usize] =
            (*histo).alpha_[PixOrCopyLiteral(v, 3 as ::core::ffi::c_int) as usize].wrapping_add(1);
        (*histo).red_[PixOrCopyLiteral(v, 2 as ::core::ffi::c_int) as usize] =
            (*histo).red_[PixOrCopyLiteral(v, 2 as ::core::ffi::c_int) as usize].wrapping_add(1);
        let ref mut fresh0 = *(*histo)
            .literal_
            .offset(PixOrCopyLiteral(v, 1 as ::core::ffi::c_int) as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        (*histo).blue_[PixOrCopyLiteral(v, 0 as ::core::ffi::c_int) as usize] =
            (*histo).blue_[PixOrCopyLiteral(v, 0 as ::core::ffi::c_int) as usize].wrapping_add(1);
    } else if PixOrCopyIsCacheIdx(v) != 0 {
        let literal_ix: ::core::ffi::c_int = ((NUM_LITERAL_CODES + NUM_LENGTH_CODES) as uint32_t)
            .wrapping_add(PixOrCopyCacheIdx(v))
            as ::core::ffi::c_int;
        '_c2rust_label: {
            if (*histo).palette_code_bits_ != 0 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"histo->palette_code_bits_ != 0\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    213 as ::core::ffi::c_uint,
                    b"void VP8LHistogramAddSinglePixOrCopy(VP8LHistogram *const, const PixOrCopy *const, int (*const)(int, int), int)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        let ref mut fresh1 = *(*histo).literal_.offset(literal_ix as isize);
        *fresh1 = (*fresh1).wrapping_add(1);
    } else {
        let mut code: ::core::ffi::c_int = 0;
        let mut extra_bits: ::core::ffi::c_int = 0;
        VP8LPrefixEncodeBits(
            PixOrCopyLength(v) as ::core::ffi::c_int,
            &raw mut code,
            &raw mut extra_bits,
        );
        let ref mut fresh2 = *(*histo)
            .literal_
            .offset((NUM_LITERAL_CODES + code) as isize);
        *fresh2 = (*fresh2).wrapping_add(1);
        if distance_modifier.is_none() {
            VP8LPrefixEncodeBits(
                PixOrCopyDistance(v) as ::core::ffi::c_int,
                &raw mut code,
                &raw mut extra_bits,
            );
        } else {
            VP8LPrefixEncodeBits(
                distance_modifier.expect("non-null function pointer")(
                    distance_modifier_arg0,
                    PixOrCopyDistance(v) as ::core::ffi::c_int,
                ),
                &raw mut code,
                &raw mut extra_bits,
            );
        }
        (*histo).distance_[code as usize] = (*histo).distance_[code as usize].wrapping_add(1);
    };
}
#[inline]
unsafe extern "C" fn BitsEntropyRefine(mut entropy: *const VP8LBitEntropy) -> ::core::ffi::c_float {
    let mut mix: ::core::ffi::c_float = 0.;
    if (*entropy).nonzeros < 5 as ::core::ffi::c_int {
        if (*entropy).nonzeros <= 1 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int as ::core::ffi::c_float;
        }
        if (*entropy).nonzeros == 2 as ::core::ffi::c_int {
            return 0.99f32 * (*entropy).sum as ::core::ffi::c_float + 0.01f32 * (*entropy).entropy;
        }
        if (*entropy).nonzeros == 3 as ::core::ffi::c_int {
            mix = 0.95f32;
        } else {
            mix = 0.7f32;
        }
    } else {
        mix = 0.627f32;
    }
    let mut min_limit: ::core::ffi::c_float = 2.0f32 * (*entropy).sum as ::core::ffi::c_float
        - (*entropy).max_val as ::core::ffi::c_float;
    min_limit = mix * min_limit + (1.0f32 - mix) * (*entropy).entropy;
    return if (*entropy).entropy < min_limit {
        min_limit
    } else {
        (*entropy).entropy
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBitsEntropy(
    array: *const uint32_t,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_float {
    let mut entropy: VP8LBitEntropy = VP8LBitEntropy {
        entropy: 0.,
        sum: 0,
        nonzeros: 0,
        max_val: 0,
        nonzero_code: 0,
    };
    VP8LBitsEntropyUnrefined(array, n, &raw mut entropy);
    return BitsEntropyRefine(&raw mut entropy);
}
unsafe extern "C" fn InitialHuffmanCost() -> ::core::ffi::c_float {
    static mut kHuffmanCodeOfHuffmanCodeSize: ::core::ffi::c_int =
        CODE_LENGTH_CODES * 3 as ::core::ffi::c_int;
    static mut kSmallBias: ::core::ffi::c_float = 9.1f32;
    return kHuffmanCodeOfHuffmanCodeSize as ::core::ffi::c_float - kSmallBias;
}
unsafe extern "C" fn FinalHuffmanCost(stats: *const VP8LStreaks) -> ::core::ffi::c_float {
    let mut retval: ::core::ffi::c_float = InitialHuffmanCost();
    retval += (*stats).counts[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_float * 1.5625f32
        + 0.234375f32
            * (*stats).streaks[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_float;
    retval += (*stats).counts[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_float
        * 2.578125f32
        + 0.703125f32
            * (*stats).streaks[1 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_float;
    retval += 1.796875f32
        * (*stats).streaks[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_float;
    retval += 3.28125f32
        * (*stats).streaks[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_float;
    return retval;
}
unsafe extern "C" fn PopulationCost(
    population: *const uint32_t,
    mut length: ::core::ffi::c_int,
    trivial_sym: *mut uint32_t,
    is_used: *mut uint8_t,
) -> ::core::ffi::c_float {
    let mut bit_entropy: VP8LBitEntropy = VP8LBitEntropy {
        entropy: 0.,
        sum: 0,
        nonzeros: 0,
        max_val: 0,
        nonzero_code: 0,
    };
    let mut stats: VP8LStreaks = VP8LStreaks {
        counts: [0; 2],
        streaks: [[0; 2]; 2],
    };
    VP8LGetEntropyUnrefined.expect("non-null function pointer")(
        population as *const uint32_t,
        length,
        &raw mut bit_entropy,
        &raw mut stats,
    );
    if !trivial_sym.is_null() {
        *trivial_sym = if bit_entropy.nonzeros == 1 as ::core::ffi::c_int {
            bit_entropy.nonzero_code
        } else {
            VP8L_NON_TRIVIAL_SYM as uint32_t
        };
    }
    *is_used = (stats.streaks[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
        != 0 as ::core::ffi::c_int
        || stats.streaks[1 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as uint8_t;
    return BitsEntropyRefine(&raw mut bit_entropy) + FinalHuffmanCost(&raw mut stats);
}
#[inline]
unsafe extern "C" fn GetCombinedEntropy(
    X: *const uint32_t,
    Y: *const uint32_t,
    mut length: ::core::ffi::c_int,
    mut is_X_used: ::core::ffi::c_int,
    mut is_Y_used: ::core::ffi::c_int,
    mut trivial_at_end: ::core::ffi::c_int,
) -> ::core::ffi::c_float {
    let mut stats: VP8LStreaks = VP8LStreaks {
        counts: [0; 2],
        streaks: [[0; 2]; 2],
    };
    if trivial_at_end != 0 {
        memset(
            &raw mut stats as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<VP8LStreaks>() as size_t,
        );
        stats.streaks[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize] =
            1 as ::core::ffi::c_int;
        stats.counts[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        stats.streaks[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize] =
            length - 1 as ::core::ffi::c_int;
        return FinalHuffmanCost(&raw mut stats);
    } else {
        let mut bit_entropy: VP8LBitEntropy = VP8LBitEntropy {
            entropy: 0.,
            sum: 0,
            nonzeros: 0,
            max_val: 0,
            nonzero_code: 0,
        };
        if is_X_used != 0 {
            if is_Y_used != 0 {
                VP8LGetCombinedEntropyUnrefined.expect("non-null function pointer")(
                    X as *const uint32_t,
                    Y as *const uint32_t,
                    length,
                    &raw mut bit_entropy,
                    &raw mut stats,
                );
            } else {
                VP8LGetEntropyUnrefined.expect("non-null function pointer")(
                    X as *const uint32_t,
                    length,
                    &raw mut bit_entropy,
                    &raw mut stats,
                );
            }
        } else if is_Y_used != 0 {
            VP8LGetEntropyUnrefined.expect("non-null function pointer")(
                Y as *const uint32_t,
                length,
                &raw mut bit_entropy,
                &raw mut stats,
            );
        } else {
            memset(
                &raw mut stats as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<VP8LStreaks>() as size_t,
            );
            stats.counts[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
            stats.streaks[0 as ::core::ffi::c_int as usize]
                [(length > 3 as ::core::ffi::c_int) as ::core::ffi::c_int as usize] = length;
            VP8LBitEntropyInit(&raw mut bit_entropy);
        }
        return BitsEntropyRefine(&raw mut bit_entropy) + FinalHuffmanCost(&raw mut stats);
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHistogramEstimateBits(p: *mut VP8LHistogram) -> ::core::ffi::c_float {
    return PopulationCost(
        (*p).literal_,
        VP8LHistogramNumCodes((*p).palette_code_bits_),
        ::core::ptr::null_mut::<uint32_t>(),
        (&raw mut (*p).is_used_ as *mut uint8_t).offset(0 as ::core::ffi::c_int as isize)
            as *mut uint8_t,
    ) + PopulationCost(
        &raw mut (*p).red_ as *mut uint32_t,
        NUM_LITERAL_CODES,
        ::core::ptr::null_mut::<uint32_t>(),
        (&raw mut (*p).is_used_ as *mut uint8_t).offset(1 as ::core::ffi::c_int as isize)
            as *mut uint8_t,
    ) + PopulationCost(
        &raw mut (*p).blue_ as *mut uint32_t,
        NUM_LITERAL_CODES,
        ::core::ptr::null_mut::<uint32_t>(),
        (&raw mut (*p).is_used_ as *mut uint8_t).offset(2 as ::core::ffi::c_int as isize)
            as *mut uint8_t,
    ) + PopulationCost(
        &raw mut (*p).alpha_ as *mut uint32_t,
        NUM_LITERAL_CODES,
        ::core::ptr::null_mut::<uint32_t>(),
        (&raw mut (*p).is_used_ as *mut uint8_t).offset(3 as ::core::ffi::c_int as isize)
            as *mut uint8_t,
    ) + PopulationCost(
        &raw mut (*p).distance_ as *mut uint32_t,
        NUM_DISTANCE_CODES,
        ::core::ptr::null_mut::<uint32_t>(),
        (&raw mut (*p).is_used_ as *mut uint8_t).offset(4 as ::core::ffi::c_int as isize)
            as *mut uint8_t,
    ) + VP8LExtraCost.expect("non-null function pointer")(
        (*p).literal_.offset(NUM_LITERAL_CODES as isize),
        NUM_LENGTH_CODES,
    ) + VP8LExtraCost.expect("non-null function pointer")(
        &raw mut (*p).distance_ as *mut uint32_t,
        NUM_DISTANCE_CODES,
    );
}
unsafe extern "C" fn GetCombinedHistogramEntropy(
    a: *const VP8LHistogram,
    b: *const VP8LHistogram,
    mut cost_threshold: ::core::ffi::c_float,
    mut cost: *mut ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    let palette_code_bits: ::core::ffi::c_int = (*a).palette_code_bits_;
    let mut trivial_at_end: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    '_c2rust_label: {
        if (*a).palette_code_bits_ == (*b).palette_code_bits_ {
        } else {
            __assert_fail(
                b"a->palette_code_bits_ == b->palette_code_bits_\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                380 as ::core::ffi::c_uint,
                b"int GetCombinedHistogramEntropy(const VP8LHistogram *const, const VP8LHistogram *const, float, float *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    *cost += GetCombinedEntropy(
        (*a).literal_,
        (*b).literal_,
        VP8LHistogramNumCodes(palette_code_bits),
        (*a).is_used_[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        (*b).is_used_[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    *cost += VP8LExtraCostCombined.expect("non-null function pointer")(
        (*a).literal_.offset(NUM_LITERAL_CODES as isize),
        (*b).literal_.offset(NUM_LITERAL_CODES as isize),
        NUM_LENGTH_CODES,
    );
    if *cost > cost_threshold {
        return 0 as ::core::ffi::c_int;
    }
    if (*a).trivial_symbol_ != VP8L_NON_TRIVIAL_SYM as uint32_t
        && (*a).trivial_symbol_ == (*b).trivial_symbol_
    {
        let color_a: uint32_t = (*a).trivial_symbol_ >> 24 as ::core::ffi::c_int & 0xff as uint32_t;
        let color_r: uint32_t = (*a).trivial_symbol_ >> 16 as ::core::ffi::c_int & 0xff as uint32_t;
        let color_b: uint32_t = (*a).trivial_symbol_ >> 0 as ::core::ffi::c_int & 0xff as uint32_t;
        if (color_a == 0 as uint32_t || color_a == 0xff as uint32_t)
            && (color_r == 0 as uint32_t || color_r == 0xff as uint32_t)
            && (color_b == 0 as uint32_t || color_b == 0xff as uint32_t)
        {
            trivial_at_end = 1 as ::core::ffi::c_int;
        }
    }
    *cost += GetCombinedEntropy(
        &raw const (*a).red_ as *const uint32_t,
        &raw const (*b).red_ as *const uint32_t,
        NUM_LITERAL_CODES,
        (*a).is_used_[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        (*b).is_used_[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        trivial_at_end,
    );
    if *cost > cost_threshold {
        return 0 as ::core::ffi::c_int;
    }
    *cost += GetCombinedEntropy(
        &raw const (*a).blue_ as *const uint32_t,
        &raw const (*b).blue_ as *const uint32_t,
        NUM_LITERAL_CODES,
        (*a).is_used_[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        (*b).is_used_[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        trivial_at_end,
    );
    if *cost > cost_threshold {
        return 0 as ::core::ffi::c_int;
    }
    *cost += GetCombinedEntropy(
        &raw const (*a).alpha_ as *const uint32_t,
        &raw const (*b).alpha_ as *const uint32_t,
        NUM_LITERAL_CODES,
        (*a).is_used_[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        (*b).is_used_[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        trivial_at_end,
    );
    if *cost > cost_threshold {
        return 0 as ::core::ffi::c_int;
    }
    *cost += GetCombinedEntropy(
        &raw const (*a).distance_ as *const uint32_t,
        &raw const (*b).distance_ as *const uint32_t,
        NUM_DISTANCE_CODES,
        (*a).is_used_[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        (*b).is_used_[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    *cost += VP8LExtraCostCombined.expect("non-null function pointer")(
        &raw const (*a).distance_ as *const uint32_t,
        &raw const (*b).distance_ as *const uint32_t,
        NUM_DISTANCE_CODES,
    );
    if *cost > cost_threshold {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn HistogramAdd(
    a: *const VP8LHistogram,
    b: *const VP8LHistogram,
    out: *mut VP8LHistogram,
) {
    VP8LHistogramAdd(a, b, out);
    (*out).trivial_symbol_ = if (*a).trivial_symbol_ == (*b).trivial_symbol_ {
        (*a).trivial_symbol_
    } else {
        VP8L_NON_TRIVIAL_SYM as uint32_t
    };
}
unsafe extern "C" fn HistogramAddEval(
    a: *const VP8LHistogram,
    b: *const VP8LHistogram,
    out: *mut VP8LHistogram,
    mut cost_threshold: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    let mut cost: ::core::ffi::c_float = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
    let sum_cost: ::core::ffi::c_float = (*a).bit_cost_ + (*b).bit_cost_;
    cost_threshold += sum_cost;
    if GetCombinedHistogramEntropy(a, b, cost_threshold, &raw mut cost) != 0 {
        HistogramAdd(a, b, out);
        (*out).bit_cost_ = cost;
        (*out).palette_code_bits_ = (*a).palette_code_bits_;
    }
    return cost - sum_cost;
}
unsafe extern "C" fn HistogramAddThresh(
    a: *const VP8LHistogram,
    b: *const VP8LHistogram,
    mut cost_threshold: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    let mut cost: ::core::ffi::c_float = 0.;
    '_c2rust_label: {
        if !a.is_null() && !b.is_null() {
        } else {
            __assert_fail(
                b"a != NULL && b != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                465 as ::core::ffi::c_uint,
                b"float HistogramAddThresh(const VP8LHistogram *const, const VP8LHistogram *const, float)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    cost = -(*a).bit_cost_;
    GetCombinedHistogramEntropy(a, b, cost_threshold, &raw mut cost);
    return cost;
}
unsafe extern "C" fn DominantCostRangeInit(c: *mut DominantCostRange) {
    (*c).literal_max_ = 0.0f32;
    (*c).literal_min_ = MAX_BIT_COST;
    (*c).red_max_ = 0.0f32;
    (*c).red_min_ = MAX_BIT_COST;
    (*c).blue_max_ = 0.0f32;
    (*c).blue_min_ = MAX_BIT_COST;
}
unsafe extern "C" fn UpdateDominantCostRange(h: *const VP8LHistogram, c: *mut DominantCostRange) {
    if (*c).literal_max_ < (*h).literal_cost_ {
        (*c).literal_max_ = (*h).literal_cost_;
    }
    if (*c).literal_min_ > (*h).literal_cost_ {
        (*c).literal_min_ = (*h).literal_cost_;
    }
    if (*c).red_max_ < (*h).red_cost_ {
        (*c).red_max_ = (*h).red_cost_;
    }
    if (*c).red_min_ > (*h).red_cost_ {
        (*c).red_min_ = (*h).red_cost_;
    }
    if (*c).blue_max_ < (*h).blue_cost_ {
        (*c).blue_max_ = (*h).blue_cost_;
    }
    if (*c).blue_min_ > (*h).blue_cost_ {
        (*c).blue_min_ = (*h).blue_cost_;
    }
}
unsafe extern "C" fn UpdateHistogramCost(h: *mut VP8LHistogram) {
    let mut alpha_sym: uint32_t = 0;
    let mut red_sym: uint32_t = 0;
    let mut blue_sym: uint32_t = 0;
    let alpha_cost: ::core::ffi::c_float = PopulationCost(
        &raw mut (*h).alpha_ as *mut uint32_t,
        NUM_LITERAL_CODES,
        &raw mut alpha_sym,
        (&raw mut (*h).is_used_ as *mut uint8_t).offset(3 as ::core::ffi::c_int as isize)
            as *mut uint8_t,
    ) as ::core::ffi::c_float;
    let distance_cost: ::core::ffi::c_float = PopulationCost(
        &raw mut (*h).distance_ as *mut uint32_t,
        NUM_DISTANCE_CODES,
        ::core::ptr::null_mut::<uint32_t>(),
        (&raw mut (*h).is_used_ as *mut uint8_t).offset(4 as ::core::ffi::c_int as isize)
            as *mut uint8_t,
    ) as ::core::ffi::c_float
        + VP8LExtraCost.expect("non-null function pointer")(
            &raw mut (*h).distance_ as *mut uint32_t,
            NUM_DISTANCE_CODES,
        ) as ::core::ffi::c_float;
    let num_codes: ::core::ffi::c_int =
        VP8LHistogramNumCodes((*h).palette_code_bits_) as ::core::ffi::c_int;
    (*h).literal_cost_ = PopulationCost(
        (*h).literal_,
        num_codes,
        ::core::ptr::null_mut::<uint32_t>(),
        (&raw mut (*h).is_used_ as *mut uint8_t).offset(0 as ::core::ffi::c_int as isize)
            as *mut uint8_t,
    ) + VP8LExtraCost.expect("non-null function pointer")(
        (*h).literal_.offset(NUM_LITERAL_CODES as isize),
        NUM_LENGTH_CODES,
    );
    (*h).red_cost_ = PopulationCost(
        &raw mut (*h).red_ as *mut uint32_t,
        NUM_LITERAL_CODES,
        &raw mut red_sym,
        (&raw mut (*h).is_used_ as *mut uint8_t).offset(1 as ::core::ffi::c_int as isize)
            as *mut uint8_t,
    );
    (*h).blue_cost_ = PopulationCost(
        &raw mut (*h).blue_ as *mut uint32_t,
        NUM_LITERAL_CODES,
        &raw mut blue_sym,
        (&raw mut (*h).is_used_ as *mut uint8_t).offset(2 as ::core::ffi::c_int as isize)
            as *mut uint8_t,
    );
    (*h).bit_cost_ =
        (*h).literal_cost_ + (*h).red_cost_ + (*h).blue_cost_ + alpha_cost + distance_cost;
    if alpha_sym | red_sym | blue_sym == VP8L_NON_TRIVIAL_SYM as uint32_t {
        (*h).trivial_symbol_ = VP8L_NON_TRIVIAL_SYM as uint32_t;
    } else {
        (*h).trivial_symbol_ = alpha_sym << 24 as ::core::ffi::c_int
            | red_sym << 16 as ::core::ffi::c_int
            | blue_sym << 0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn GetBinIdForEntropy(
    mut min: ::core::ffi::c_float,
    mut max: ::core::ffi::c_float,
    mut val: ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    let range: ::core::ffi::c_float = max - min;
    if range as ::core::ffi::c_double > 0.0f64 {
        let delta: ::core::ffi::c_float = val - min;
        return ((NUM_PARTITIONS as ::core::ffi::c_double - 1e-6f64)
            * delta as ::core::ffi::c_double
            / range as ::core::ffi::c_double) as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn GetHistoBinIndex(
    h: *const VP8LHistogram,
    c: *const DominantCostRange,
    mut low_effort: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bin_id: ::core::ffi::c_int =
        GetBinIdForEntropy((*c).literal_min_, (*c).literal_max_, (*h).literal_cost_);
    '_c2rust_label: {
        if bin_id < 4 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"bin_id < NUM_PARTITIONS\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                542 as ::core::ffi::c_uint,
                b"int GetHistoBinIndex(const VP8LHistogram *const, const DominantCostRange *const, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if low_effort == 0 {
        bin_id = bin_id * NUM_PARTITIONS
            + GetBinIdForEntropy((*c).red_min_, (*c).red_max_, (*h).red_cost_);
        bin_id = bin_id * NUM_PARTITIONS
            + GetBinIdForEntropy((*c).blue_min_, (*c).blue_max_, (*h).blue_cost_);
        '_c2rust_label_0: {
            if bin_id < 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            {
            } else {
                __assert_fail(
                    b"bin_id < BIN_SIZE\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    548 as ::core::ffi::c_uint,
                    b"int GetHistoBinIndex(const VP8LHistogram *const, const DominantCostRange *const, int)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
    }
    return bin_id;
}
unsafe extern "C" fn HistogramBuild(
    mut xsize: ::core::ffi::c_int,
    mut histo_bits: ::core::ffi::c_int,
    backward_refs: *const VP8LBackwardRefs,
    image_histo: *mut VP8LHistogramSet,
) {
    let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let histo_xsize: ::core::ffi::c_int =
        VP8LSubSampleSize(xsize as uint32_t, histo_bits as uint32_t) as ::core::ffi::c_int;
    let histograms: *mut *mut VP8LHistogram = (*image_histo).histograms;
    let mut c: VP8LRefsCursor = VP8LRefsCursorInit(backward_refs);
    '_c2rust_label: {
        if histo_bits > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"histo_bits > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                561 as ::core::ffi::c_uint,
                b"void HistogramBuild(int, int, const VP8LBackwardRefs *const, VP8LHistogramSet *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    VP8LHistogramSetClear(image_histo);
    while VP8LRefsCursorOk(&raw mut c) != 0 {
        let v: *const PixOrCopy = c.cur_pos;
        let ix: ::core::ffi::c_int = (y >> histo_bits) * histo_xsize + (x >> histo_bits);
        VP8LHistogramAddSinglePixOrCopy(
            *histograms.offset(ix as isize),
            v,
            None,
            0 as ::core::ffi::c_int,
        );
        x = (x as uint32_t).wrapping_add(PixOrCopyLength(v)) as ::core::ffi::c_int
            as ::core::ffi::c_int;
        while x >= xsize {
            x -= xsize;
            y += 1;
        }
        VP8LRefsCursorNext(&raw mut c);
    }
}
static mut kInvalidHistogramSymbol: uint16_t = -(1 as ::core::ffi::c_int) as uint16_t;
unsafe extern "C" fn HistogramCopyAndAnalyze(
    orig_histo: *mut VP8LHistogramSet,
    image_histo: *mut VP8LHistogramSet,
    num_used: *mut ::core::ffi::c_int,
    histogram_symbols: *mut uint16_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut cluster_id: ::core::ffi::c_int = 0;
    let mut num_used_orig: ::core::ffi::c_int = *num_used;
    let orig_histograms: *mut *mut VP8LHistogram = (*orig_histo).histograms;
    let histograms: *mut *mut VP8LHistogram = (*image_histo).histograms;
    '_c2rust_label: {
        if (*image_histo).max_size == (*orig_histo).max_size {
        } else {
            __assert_fail(
                b"image_histo->max_size == orig_histo->max_size\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                586 as ::core::ffi::c_uint,
                b"void HistogramCopyAndAnalyze(VP8LHistogramSet *const, VP8LHistogramSet *const, int *const, uint16_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    cluster_id = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*orig_histo).max_size {
        let histo: *mut VP8LHistogram = *orig_histograms.offset(i as isize);
        UpdateHistogramCost(histo);
        if (*histo).is_used_[0 as ::core::ffi::c_int as usize] == 0
            && (*histo).is_used_[1 as ::core::ffi::c_int as usize] == 0
            && (*histo).is_used_[2 as ::core::ffi::c_int as usize] == 0
            && (*histo).is_used_[3 as ::core::ffi::c_int as usize] == 0
            && (*histo).is_used_[4 as ::core::ffi::c_int as usize] == 0
        {
            '_c2rust_label_0: {
                if i > 0 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"i > 0\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        598 as ::core::ffi::c_uint,
                        b"void HistogramCopyAndAnalyze(VP8LHistogramSet *const, VP8LHistogramSet *const, int *const, uint16_t *const)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            HistogramSetRemoveHistogram(image_histo, i, num_used);
            HistogramSetRemoveHistogram(orig_histo, i, &raw mut num_used_orig);
            *histogram_symbols.offset(i as isize) = kInvalidHistogramSymbol;
        } else {
            HistogramCopy(histo, *histograms.offset(i as isize));
            let fresh10 = cluster_id;
            cluster_id = cluster_id + 1;
            *histogram_symbols.offset(i as isize) = fresh10 as uint16_t;
            '_c2rust_label_1: {
                if cluster_id <= (*image_histo).max_size {
                } else {
                    __assert_fail(
                        b"cluster_id <= image_histo->max_size\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        606 as ::core::ffi::c_uint,
                        b"void HistogramCopyAndAnalyze(VP8LHistogramSet *const, VP8LHistogramSet *const, int *const, uint16_t *const)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
        }
        i += 1;
    }
}
unsafe extern "C" fn HistogramAnalyzeEntropyBin(
    image_histo: *mut VP8LHistogramSet,
    bin_map: *mut uint16_t,
    mut low_effort: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let histograms: *mut *mut VP8LHistogram = (*image_histo).histograms;
    let histo_size: ::core::ffi::c_int = (*image_histo).size;
    let mut cost_range: DominantCostRange = DominantCostRange {
        literal_max_: 0.,
        literal_min_: 0.,
        red_max_: 0.,
        red_min_: 0.,
        blue_max_: 0.,
        blue_min_: 0.,
    };
    DominantCostRangeInit(&raw mut cost_range);
    i = 0 as ::core::ffi::c_int;
    while i < histo_size {
        if !(*histograms.offset(i as isize)).is_null() {
            UpdateDominantCostRange(*histograms.offset(i as isize), &raw mut cost_range);
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < histo_size {
        if !(*histograms.offset(i as isize)).is_null() {
            *bin_map.offset(i as isize) = GetHistoBinIndex(
                *histograms.offset(i as isize),
                &raw mut cost_range,
                low_effort,
            ) as uint16_t;
        }
        i += 1;
    }
}
unsafe extern "C" fn HistogramCombineEntropyBin(
    image_histo: *mut VP8LHistogramSet,
    mut num_used: *mut ::core::ffi::c_int,
    clusters: *const uint16_t,
    cluster_mappings: *mut uint16_t,
    mut cur_combo: *mut VP8LHistogram,
    bin_map: *const uint16_t,
    mut num_bins: ::core::ffi::c_int,
    mut combine_cost_factor: ::core::ffi::c_float,
    mut low_effort: ::core::ffi::c_int,
) {
    let histograms: *mut *mut VP8LHistogram = (*image_histo).histograms;
    let mut idx: ::core::ffi::c_int = 0;
    let mut bin_info: [C2RustUnnamed; 64] = [C2RustUnnamed {
        first: 0,
        num_combine_failures: 0,
    }; 64];
    '_c2rust_label: {
        if num_bins <= 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"num_bins <= BIN_SIZE\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                653 as ::core::ffi::c_uint,
                b"void HistogramCombineEntropyBin(VP8LHistogramSet *const, int *, const uint16_t *const, uint16_t *const, VP8LHistogram *, const uint16_t *const, int, float, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    idx = 0 as ::core::ffi::c_int;
    while idx < num_bins {
        bin_info[idx as usize].first = -(1 as ::core::ffi::c_int) as int16_t;
        bin_info[idx as usize].num_combine_failures = 0 as uint16_t;
        idx += 1;
    }
    idx = 0 as ::core::ffi::c_int;
    while idx < *num_used {
        *cluster_mappings.offset(idx as isize) = idx as uint16_t;
        idx += 1;
    }
    idx = 0 as ::core::ffi::c_int;
    while idx < (*image_histo).size {
        let mut bin_id: ::core::ffi::c_int = 0;
        let mut first: ::core::ffi::c_int = 0;
        if !(*histograms.offset(idx as isize)).is_null() {
            bin_id = *bin_map.offset(idx as isize) as ::core::ffi::c_int;
            first = bin_info[bin_id as usize].first as ::core::ffi::c_int;
            if first == -(1 as ::core::ffi::c_int) {
                bin_info[bin_id as usize].first = idx as int16_t;
            } else if low_effort != 0 {
                HistogramAdd(
                    *histograms.offset(idx as isize),
                    *histograms.offset(first as isize),
                    *histograms.offset(first as isize),
                );
                HistogramSetRemoveHistogram(image_histo, idx, num_used);
                *cluster_mappings.offset(*clusters.offset(idx as isize) as isize) =
                    *clusters.offset(first as isize);
            } else {
                let bit_cost: ::core::ffi::c_float = (**histograms.offset(idx as isize)).bit_cost_;
                let bit_cost_thresh: ::core::ffi::c_float = -bit_cost * combine_cost_factor;
                let curr_cost_diff: ::core::ffi::c_float = HistogramAddEval(
                    *histograms.offset(first as isize),
                    *histograms.offset(idx as isize),
                    cur_combo,
                    bit_cost_thresh,
                )
                    as ::core::ffi::c_float;
                if curr_cost_diff < bit_cost_thresh {
                    let try_combine: ::core::ffi::c_int = ((*cur_combo).trivial_symbol_
                        != VP8L_NON_TRIVIAL_SYM as uint32_t
                        || (**histograms.offset(idx as isize)).trivial_symbol_
                            == VP8L_NON_TRIVIAL_SYM as uint32_t
                            && (**histograms.offset(first as isize)).trivial_symbol_
                                == VP8L_NON_TRIVIAL_SYM as uint32_t)
                        as ::core::ffi::c_int;
                    let max_combine_failures: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
                    if try_combine != 0
                        || bin_info[bin_id as usize].num_combine_failures as ::core::ffi::c_int
                            >= max_combine_failures
                    {
                        HistogramSwap(
                            &raw mut cur_combo,
                            histograms.offset(first as isize) as *mut *mut VP8LHistogram,
                        );
                        HistogramSetRemoveHistogram(image_histo, idx, num_used);
                        *cluster_mappings.offset(*clusters.offset(idx as isize) as isize) =
                            *clusters.offset(first as isize);
                    } else {
                        bin_info[bin_id as usize].num_combine_failures = bin_info[bin_id as usize]
                            .num_combine_failures
                            .wrapping_add(1);
                    }
                }
            }
        }
        idx += 1;
    }
    if low_effort != 0 {
        idx = 0 as ::core::ffi::c_int;
        while idx < (*image_histo).size {
            if !(*histograms.offset(idx as isize)).is_null() {
                UpdateHistogramCost(*histograms.offset(idx as isize));
            }
            idx += 1;
        }
    }
}
unsafe extern "C" fn MyRand(seed: *mut uint32_t) -> uint32_t {
    *seed = (*seed as uint64_t)
        .wrapping_mul(48271 as uint64_t)
        .wrapping_rem(2147483647 as uint64_t) as uint32_t;
    '_c2rust_label: {
        if *seed > 0 as uint32_t {
        } else {
            __assert_fail(
                b"*seed > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                714 as ::core::ffi::c_uint,
                b"uint32_t MyRand(uint32_t *const)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    return *seed;
}
unsafe extern "C" fn HistoQueueInit(
    histo_queue: *mut HistoQueue,
    max_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*histo_queue).size = 0 as ::core::ffi::c_int;
    (*histo_queue).max_size = max_size;
    (*histo_queue).queue = WebPSafeMalloc(
        ((*histo_queue).max_size + 1 as ::core::ffi::c_int) as uint64_t,
        ::core::mem::size_of::<HistogramPair>() as size_t,
    ) as *mut HistogramPair;
    return ((*histo_queue).queue != NULL as *mut HistogramPair) as ::core::ffi::c_int;
}
unsafe extern "C" fn HistoQueueClear(histo_queue: *mut HistoQueue) {
    '_c2rust_label: {
        if !histo_queue.is_null() {
        } else {
            __assert_fail(
                b"histo_queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                746 as ::core::ffi::c_uint,
                b"void HistoQueueClear(HistoQueue *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    WebPSafeFree((*histo_queue).queue as *mut ::core::ffi::c_void);
    (*histo_queue).size = 0 as ::core::ffi::c_int;
    (*histo_queue).max_size = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn HistoQueuePopPair(histo_queue: *mut HistoQueue, pair: *mut HistogramPair) {
    '_c2rust_label: {
        if pair >= (*histo_queue).queue
            && pair < (*histo_queue).queue.offset((*histo_queue).size as isize)
        {
        } else {
            __assert_fail(
                b"pair >= histo_queue->queue && pair < (histo_queue->queue + histo_queue->size)\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                757 as ::core::ffi::c_uint,
                b"void HistoQueuePopPair(HistoQueue *const, HistogramPair *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*histo_queue).size > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"histo_queue->size > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                758 as ::core::ffi::c_uint,
                b"void HistoQueuePopPair(HistoQueue *const, HistogramPair *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    *pair = *(*histo_queue)
        .queue
        .offset(((*histo_queue).size - 1 as ::core::ffi::c_int) as isize);
    (*histo_queue).size -= 1;
}
unsafe extern "C" fn HistoQueueUpdateHead(histo_queue: *mut HistoQueue, pair: *mut HistogramPair) {
    '_c2rust_label: {
        if ((*pair).cost_diff as ::core::ffi::c_double) < 0.0f64 {
        } else {
            __assert_fail(
                b"pair->cost_diff < 0.\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                766 as ::core::ffi::c_uint,
                b"void HistoQueueUpdateHead(HistoQueue *const, HistogramPair *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if pair >= (*histo_queue).queue
            && pair < (*histo_queue).queue.offset((*histo_queue).size as isize)
        {
        } else {
            __assert_fail(
                b"pair >= histo_queue->queue && pair < (histo_queue->queue + histo_queue->size)\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                768 as ::core::ffi::c_uint,
                b"void HistoQueueUpdateHead(HistoQueue *const, HistogramPair *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*histo_queue).size > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"histo_queue->size > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                769 as ::core::ffi::c_uint,
                b"void HistoQueueUpdateHead(HistoQueue *const, HistogramPair *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*pair).cost_diff
        < (*(*histo_queue)
            .queue
            .offset(0 as ::core::ffi::c_int as isize))
        .cost_diff
    {
        let tmp: HistogramPair = *(*histo_queue)
            .queue
            .offset(0 as ::core::ffi::c_int as isize);
        *(*histo_queue)
            .queue
            .offset(0 as ::core::ffi::c_int as isize) = *pair;
        *pair = tmp;
    }
}
unsafe extern "C" fn HistoQueueUpdatePair(
    h1: *const VP8LHistogram,
    h2: *const VP8LHistogram,
    mut threshold: ::core::ffi::c_float,
    pair: *mut HistogramPair,
) {
    let sum_cost: ::core::ffi::c_float = (*h1).bit_cost_ + (*h2).bit_cost_;
    (*pair).cost_combo = 0.0f32;
    GetCombinedHistogramEntropy(h1, h2, sum_cost + threshold, &raw mut (*pair).cost_combo);
    (*pair).cost_diff = (*pair).cost_combo - sum_cost;
}
unsafe extern "C" fn HistoQueuePush(
    histo_queue: *mut HistoQueue,
    histograms: *mut *mut VP8LHistogram,
    mut idx1: ::core::ffi::c_int,
    mut idx2: ::core::ffi::c_int,
    mut threshold: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    let mut h1: *const VP8LHistogram = ::core::ptr::null::<VP8LHistogram>();
    let mut h2: *const VP8LHistogram = ::core::ptr::null::<VP8LHistogram>();
    let mut pair: HistogramPair = HistogramPair {
        idx1: 0,
        idx2: 0,
        cost_diff: 0.,
        cost_combo: 0.,
    };
    if (*histo_queue).size == (*histo_queue).max_size {
        return 0.0f32;
    }
    '_c2rust_label: {
        if threshold as ::core::ffi::c_double <= 0.0f64 {
        } else {
            __assert_fail(
                b"threshold <= 0.\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                801 as ::core::ffi::c_uint,
                b"float HistoQueuePush(HistoQueue *const, VP8LHistogram **const, int, int, float)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if idx1 > idx2 {
        let tmp: ::core::ffi::c_int = idx2;
        idx2 = idx1;
        idx1 = tmp;
    }
    pair.idx1 = idx1;
    pair.idx2 = idx2;
    h1 = *histograms.offset(idx1 as isize);
    h2 = *histograms.offset(idx2 as isize);
    HistoQueueUpdatePair(h1, h2, threshold, &raw mut pair);
    if pair.cost_diff >= threshold {
        return 0.0f32;
    }
    let fresh7 = (*histo_queue).size;
    (*histo_queue).size = (*histo_queue).size + 1;
    *(*histo_queue).queue.offset(fresh7 as isize) = pair;
    HistoQueueUpdateHead(
        histo_queue,
        (*histo_queue)
            .queue
            .offset(((*histo_queue).size - 1 as ::core::ffi::c_int) as isize)
            as *mut HistogramPair,
    );
    return pair.cost_diff;
}
unsafe extern "C" fn HistogramCombineGreedy(
    image_histo: *mut VP8LHistogramSet,
    num_used: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let image_histo_size: ::core::ffi::c_int = (*image_histo).size;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let histograms: *mut *mut VP8LHistogram = (*image_histo).histograms;
    let mut histo_queue: HistoQueue = HistoQueue {
        queue: ::core::ptr::null_mut::<HistogramPair>(),
        size: 0,
        max_size: 0,
    };
    if !(HistoQueueInit(&raw mut histo_queue, image_histo_size * image_histo_size) == 0) {
        i = 0 as ::core::ffi::c_int;
        while i < image_histo_size {
            if !(*(*image_histo).histograms.offset(i as isize)).is_null() {
                j = i + 1 as ::core::ffi::c_int;
                while j < image_histo_size {
                    if !(*(*image_histo).histograms.offset(j as isize)).is_null() {
                        HistoQueuePush(&raw mut histo_queue, histograms, i, j, 0.0f32);
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        while histo_queue.size > 0 as ::core::ffi::c_int {
            let idx1: ::core::ffi::c_int =
                (*histo_queue.queue.offset(0 as ::core::ffi::c_int as isize)).idx1;
            let idx2: ::core::ffi::c_int =
                (*histo_queue.queue.offset(0 as ::core::ffi::c_int as isize)).idx2;
            HistogramAdd(
                *histograms.offset(idx2 as isize),
                *histograms.offset(idx1 as isize),
                *histograms.offset(idx1 as isize),
            );
            (**histograms.offset(idx1 as isize)).bit_cost_ =
                (*histo_queue.queue.offset(0 as ::core::ffi::c_int as isize)).cost_combo;
            HistogramSetRemoveHistogram(image_histo, idx2, num_used);
            i = 0 as ::core::ffi::c_int;
            while i < histo_queue.size {
                let p: *mut HistogramPair = histo_queue.queue.offset(i as isize);
                if (*p).idx1 == idx1 || (*p).idx2 == idx1 || (*p).idx1 == idx2 || (*p).idx2 == idx2
                {
                    HistoQueuePopPair(&raw mut histo_queue, p);
                } else {
                    HistoQueueUpdateHead(&raw mut histo_queue, p);
                    i += 1;
                }
            }
            i = 0 as ::core::ffi::c_int;
            while i < (*image_histo).size {
                if !(i == idx1 || (*(*image_histo).histograms.offset(i as isize)).is_null()) {
                    HistoQueuePush(
                        &raw mut histo_queue,
                        (*image_histo).histograms,
                        idx1,
                        i,
                        0.0f32,
                    );
                }
                i += 1;
            }
        }
        ok = 1 as ::core::ffi::c_int;
    }
    HistoQueueClear(&raw mut histo_queue);
    return ok;
}
unsafe extern "C" fn PairComparison(
    mut idx1: *const ::core::ffi::c_void,
    mut idx2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return *(idx1 as *mut ::core::ffi::c_int) - *(idx2 as *mut ::core::ffi::c_int);
}
unsafe extern "C" fn HistogramCombineStochastic(
    image_histo: *mut VP8LHistogramSet,
    num_used: *mut ::core::ffi::c_int,
    mut min_cluster_size: ::core::ffi::c_int,
    do_greedy: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut iter: ::core::ffi::c_int = 0;
    let mut seed: uint32_t = 1 as uint32_t;
    let mut tries_with_no_success: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let outer_iters: ::core::ffi::c_int = *num_used;
    let num_tries_no_success: ::core::ffi::c_int = outer_iters / 2 as ::core::ffi::c_int;
    let histograms: *mut *mut VP8LHistogram = (*image_histo).histograms;
    let mut histo_queue: HistoQueue = HistoQueue {
        queue: ::core::ptr::null_mut::<HistogramPair>(),
        size: 0,
        max_size: 0,
    };
    let kHistoQueueSize: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
    let mut ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mappings: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    if *num_used < min_cluster_size {
        *do_greedy = 1 as ::core::ffi::c_int;
        return 1 as ::core::ffi::c_int;
    }
    mappings = WebPSafeMalloc(
        *num_used as uint64_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    if mappings.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !(HistoQueueInit(&raw mut histo_queue, kHistoQueueSize) == 0) {
        j = 0 as ::core::ffi::c_int;
        iter = 0 as ::core::ffi::c_int;
        while iter < (*image_histo).size {
            if !(*histograms.offset(iter as isize)).is_null() {
                let fresh9 = j;
                j = j + 1;
                *mappings.offset(fresh9 as isize) = iter;
            }
            iter += 1;
        }
        '_c2rust_label: {
            if j == *num_used {
            } else {
                __assert_fail(
                    b"j == *num_used\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    930 as ::core::ffi::c_uint,
                    b"int HistogramCombineStochastic(VP8LHistogramSet *const, int *const, int, int *const)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        iter = 0 as ::core::ffi::c_int;
        while iter < outer_iters && *num_used >= min_cluster_size && {
            tries_with_no_success += 1;
            tries_with_no_success < num_tries_no_success
        } {
            let mut mapping_index: *mut ::core::ffi::c_int =
                ::core::ptr::null_mut::<::core::ffi::c_int>();
            let mut best_cost: ::core::ffi::c_float = if histo_queue.size == 0 as ::core::ffi::c_int
            {
                0.0f32
            } else {
                (*histo_queue.queue.offset(0 as ::core::ffi::c_int as isize)).cost_diff
            };
            let mut best_idx1: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
            let mut best_idx2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let rand_range: uint32_t =
                ((*num_used - 1 as ::core::ffi::c_int) * *num_used) as uint32_t;
            let num_tries: ::core::ffi::c_int = *num_used / 2 as ::core::ffi::c_int;
            j = 0 as ::core::ffi::c_int;
            while *num_used >= 2 as ::core::ffi::c_int && j < num_tries {
                let mut curr_cost: ::core::ffi::c_float = 0.;
                let tmp: uint32_t = (MyRand(&raw mut seed) as uint32_t).wrapping_rem(rand_range);
                let mut idx1: uint32_t =
                    tmp.wrapping_div((*num_used - 1 as ::core::ffi::c_int) as uint32_t);
                let mut idx2: uint32_t =
                    tmp.wrapping_rem((*num_used - 1 as ::core::ffi::c_int) as uint32_t);
                if idx2 >= idx1 {
                    idx2 = idx2.wrapping_add(1);
                }
                idx1 = *mappings.offset(idx1 as isize) as uint32_t;
                idx2 = *mappings.offset(idx2 as isize) as uint32_t;
                curr_cost = HistoQueuePush(
                    &raw mut histo_queue,
                    histograms,
                    idx1 as ::core::ffi::c_int,
                    idx2 as ::core::ffi::c_int,
                    best_cost,
                );
                if curr_cost < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
                    best_cost = curr_cost;
                    if histo_queue.size == histo_queue.max_size {
                        break;
                    }
                }
                j += 1;
            }
            if !(histo_queue.size == 0 as ::core::ffi::c_int) {
                best_idx1 = (*histo_queue.queue.offset(0 as ::core::ffi::c_int as isize)).idx1;
                best_idx2 = (*histo_queue.queue.offset(0 as ::core::ffi::c_int as isize)).idx2;
                '_c2rust_label_0: {
                    if best_idx1 < best_idx2 {
                    } else {
                        __assert_fail(
                            b"best_idx1 < best_idx2\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            971 as ::core::ffi::c_uint,
                            b"int HistogramCombineStochastic(VP8LHistogramSet *const, int *const, int, int *const)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                };
                mapping_index = bsearch(
                    &raw mut best_idx2 as *const ::core::ffi::c_void,
                    mappings as *const ::core::ffi::c_void,
                    *num_used as size_t,
                    ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                    Some(
                        PairComparison
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_void,
                                *const ::core::ffi::c_void,
                            )
                                -> ::core::ffi::c_int,
                    ),
                ) as *mut ::core::ffi::c_int;
                '_c2rust_label_1: {
                    if !mapping_index.is_null() {
                    } else {
                        __assert_fail(
                            b"mapping_index != NULL\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            975 as ::core::ffi::c_uint,
                            b"int HistogramCombineStochastic(VP8LHistogramSet *const, int *const, int, int *const)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                };
                memmove(
                    mapping_index as *mut ::core::ffi::c_void,
                    mapping_index.offset(1 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_void,
                    (::core::mem::size_of::<::core::ffi::c_int>() as size_t).wrapping_mul(
                        (*num_used as ::core::ffi::c_long
                            - mapping_index.offset_from(mappings) as ::core::ffi::c_long
                            - 1 as ::core::ffi::c_long) as size_t,
                    ),
                );
                HistogramAdd(
                    *histograms.offset(best_idx2 as isize),
                    *histograms.offset(best_idx1 as isize),
                    *histograms.offset(best_idx1 as isize),
                );
                (**histograms.offset(best_idx1 as isize)).bit_cost_ =
                    (*histo_queue.queue.offset(0 as ::core::ffi::c_int as isize)).cost_combo;
                HistogramSetRemoveHistogram(image_histo, best_idx2, num_used);
                j = 0 as ::core::ffi::c_int;
                while j < histo_queue.size {
                    let p: *mut HistogramPair = histo_queue.queue.offset(j as isize);
                    let is_idx1_best: ::core::ffi::c_int =
                        ((*p).idx1 == best_idx1 || (*p).idx1 == best_idx2) as ::core::ffi::c_int;
                    let is_idx2_best: ::core::ffi::c_int =
                        ((*p).idx2 == best_idx1 || (*p).idx2 == best_idx2) as ::core::ffi::c_int;
                    let mut do_eval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    if is_idx1_best != 0 && is_idx2_best != 0 {
                        HistoQueuePopPair(&raw mut histo_queue, p);
                    } else {
                        if is_idx1_best != 0 {
                            (*p).idx1 = best_idx1;
                            do_eval = 1 as ::core::ffi::c_int;
                        } else if is_idx2_best != 0 {
                            (*p).idx2 = best_idx1;
                            do_eval = 1 as ::core::ffi::c_int;
                        }
                        if (*p).idx1 > (*p).idx2 {
                            let tmp_0: ::core::ffi::c_int = (*p).idx2;
                            (*p).idx2 = (*p).idx1;
                            (*p).idx1 = tmp_0;
                        }
                        if do_eval != 0 {
                            HistoQueueUpdatePair(
                                *histograms.offset((*p).idx1 as isize),
                                *histograms.offset((*p).idx2 as isize),
                                0.0f32,
                                p,
                            );
                            if (*p).cost_diff as ::core::ffi::c_double >= 0.0f64 {
                                HistoQueuePopPair(&raw mut histo_queue, p);
                                continue;
                            }
                        }
                        HistoQueueUpdateHead(&raw mut histo_queue, p);
                        j += 1;
                    }
                }
                tries_with_no_success = 0 as ::core::ffi::c_int;
            }
            iter += 1;
        }
        *do_greedy = (*num_used <= min_cluster_size) as ::core::ffi::c_int;
        ok = 1 as ::core::ffi::c_int;
    }
    HistoQueueClear(&raw mut histo_queue);
    WebPSafeFree(mappings as *mut ::core::ffi::c_void);
    return ok;
}
unsafe extern "C" fn HistogramRemap(
    in_0: *const VP8LHistogramSet,
    out: *mut VP8LHistogramSet,
    symbols: *mut uint16_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let in_histo: *mut *mut VP8LHistogram = (*in_0).histograms;
    let out_histo: *mut *mut VP8LHistogram = (*out).histograms;
    let in_size: ::core::ffi::c_int = (*out).max_size;
    let out_size: ::core::ffi::c_int = (*out).size;
    if out_size > 1 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < in_size {
            let mut best_out: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut best_bits: ::core::ffi::c_float = MAX_BIT_COST;
            let mut k: ::core::ffi::c_int = 0;
            if (*in_histo.offset(i as isize)).is_null() {
                *symbols.offset(i as isize) =
                    *symbols.offset((i - 1 as ::core::ffi::c_int) as isize);
            } else {
                k = 0 as ::core::ffi::c_int;
                while k < out_size {
                    let mut cur_bits: ::core::ffi::c_float = 0.;
                    cur_bits = HistogramAddThresh(
                        *out_histo.offset(k as isize),
                        *in_histo.offset(i as isize),
                        best_bits,
                    );
                    if k == 0 as ::core::ffi::c_int || cur_bits < best_bits {
                        best_bits = cur_bits;
                        best_out = k;
                    }
                    k += 1;
                }
                *symbols.offset(i as isize) = best_out as uint16_t;
            }
            i += 1;
        }
    } else {
        '_c2rust_label: {
            if out_size == 1 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"out_size == 1\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1068 as ::core::ffi::c_uint,
                    b"void HistogramRemap(const VP8LHistogramSet *const, VP8LHistogramSet *const, uint16_t *const)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        i = 0 as ::core::ffi::c_int;
        while i < in_size {
            *symbols.offset(i as isize) = 0 as uint16_t;
            i += 1;
        }
    }
    VP8LHistogramSetClear(out);
    (*out).size = out_size;
    i = 0 as ::core::ffi::c_int;
    while i < in_size {
        let mut idx: ::core::ffi::c_int = 0;
        if !(*in_histo.offset(i as isize)).is_null() {
            idx = *symbols.offset(i as isize) as ::core::ffi::c_int;
            HistogramAdd(
                *in_histo.offset(i as isize),
                *out_histo.offset(idx as isize),
                *out_histo.offset(idx as isize),
            );
        }
        i += 1;
    }
}
unsafe extern "C" fn GetCombineCostFactor(
    mut histo_size: ::core::ffi::c_int,
    mut quality: ::core::ffi::c_int,
) -> ::core::ffi::c_float {
    let mut combine_cost_factor: ::core::ffi::c_float = 0.16f32;
    if quality < 90 as ::core::ffi::c_int {
        if histo_size > 256 as ::core::ffi::c_int {
            combine_cost_factor /= 2.0f32;
        }
        if histo_size > 512 as ::core::ffi::c_int {
            combine_cost_factor /= 2.0f32;
        }
        if histo_size > 1024 as ::core::ffi::c_int {
            combine_cost_factor /= 2.0f32;
        }
        if quality <= 50 as ::core::ffi::c_int {
            combine_cost_factor /= 2.0f32;
        }
    }
    return combine_cost_factor;
}
unsafe extern "C" fn OptimizeHistogramSymbols(
    set: *const VP8LHistogramSet,
    cluster_mappings: *mut uint16_t,
    mut num_clusters: ::core::ffi::c_int,
    cluster_mappings_tmp: *mut uint16_t,
    symbols: *mut uint16_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut cluster_max: ::core::ffi::c_int = 0;
    let mut do_continue: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while do_continue != 0 {
        do_continue = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < num_clusters {
            let mut k: ::core::ffi::c_int = 0;
            k = *cluster_mappings.offset(i as isize) as ::core::ffi::c_int;
            while k != *cluster_mappings.offset(k as isize) as ::core::ffi::c_int {
                *cluster_mappings.offset(k as isize) =
                    *cluster_mappings.offset(*cluster_mappings.offset(k as isize) as isize);
                k = *cluster_mappings.offset(k as isize) as ::core::ffi::c_int;
            }
            if k != *cluster_mappings.offset(i as isize) as ::core::ffi::c_int {
                do_continue = 1 as ::core::ffi::c_int;
                *cluster_mappings.offset(i as isize) = k as uint16_t;
            }
            i += 1;
        }
    }
    cluster_max = 0 as ::core::ffi::c_int;
    memset(
        cluster_mappings_tmp as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ((*set).max_size as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
    );
    '_c2rust_label: {
        if *cluster_mappings.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"cluster_mappings[0] == 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1127 as ::core::ffi::c_uint,
                b"void OptimizeHistogramSymbols(const VP8LHistogramSet *const, uint16_t *const, int, uint16_t *const, uint16_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    i = 0 as ::core::ffi::c_int;
    while i < (*set).max_size {
        let mut cluster: ::core::ffi::c_int = 0;
        if !(*symbols.offset(i as isize) as ::core::ffi::c_int
            == kInvalidHistogramSymbol as ::core::ffi::c_int)
        {
            cluster = *cluster_mappings.offset(*symbols.offset(i as isize) as isize)
                as ::core::ffi::c_int;
            '_c2rust_label_0: {
                if (*symbols.offset(i as isize) as ::core::ffi::c_int) < num_clusters {
                } else {
                    __assert_fail(
                        b"symbols[i] < num_clusters\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        1133 as ::core::ffi::c_uint,
                        b"void OptimizeHistogramSymbols(const VP8LHistogramSet *const, uint16_t *const, int, uint16_t *const, uint16_t *const)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            if cluster > 0 as ::core::ffi::c_int
                && *cluster_mappings_tmp.offset(cluster as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                cluster_max += 1;
                *cluster_mappings_tmp.offset(cluster as isize) = cluster_max as uint16_t;
            }
            *symbols.offset(i as isize) = *cluster_mappings_tmp.offset(cluster as isize);
        }
        i += 1;
    }
    cluster_max = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*set).max_size {
        if !(*symbols.offset(i as isize) as ::core::ffi::c_int
            == kInvalidHistogramSymbol as ::core::ffi::c_int)
        {
            if !(*symbols.offset(i as isize) as ::core::ffi::c_int <= cluster_max) {
                cluster_max += 1;
                '_c2rust_label_1: {
                    if *symbols.offset(i as isize) as ::core::ffi::c_int == cluster_max {
                    } else {
                        __assert_fail(
                            b"symbols[i] == cluster_max\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/histogram_enc.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            1147 as ::core::ffi::c_uint,
                            b"void OptimizeHistogramSymbols(const VP8LHistogramSet *const, uint16_t *const, int, uint16_t *const, uint16_t *const)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                };
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn RemoveEmptyHistograms(image_histo: *mut VP8LHistogramSet) {
    let mut size: uint32_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    size = 0 as uint32_t;
    while i < (*image_histo).size {
        if !(*(*image_histo).histograms.offset(i as isize)).is_null() {
            let fresh5 = size;
            size = size.wrapping_add(1);
            let ref mut fresh6 = *(*image_histo).histograms.offset(fresh5 as isize);
            *fresh6 = *(*image_histo).histograms.offset(i as isize);
        }
        i += 1;
    }
    (*image_histo).size = size as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LGetHistoImageSymbols(
    mut xsize: ::core::ffi::c_int,
    mut ysize: ::core::ffi::c_int,
    refs: *const VP8LBackwardRefs,
    mut quality: ::core::ffi::c_int,
    mut low_effort: ::core::ffi::c_int,
    mut histogram_bits: ::core::ffi::c_int,
    mut cache_bits: ::core::ffi::c_int,
    image_histo: *mut VP8LHistogramSet,
    tmp_histo: *mut VP8LHistogram,
    histogram_symbols: *mut uint16_t,
    pic: *const WebPPicture,
    mut percent_range: ::core::ffi::c_int,
    percent: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let histo_xsize: ::core::ffi::c_int = (if histogram_bits != 0 {
        VP8LSubSampleSize(xsize as uint32_t, histogram_bits as uint32_t)
    } else {
        1 as uint32_t
    }) as ::core::ffi::c_int;
    let histo_ysize: ::core::ffi::c_int = (if histogram_bits != 0 {
        VP8LSubSampleSize(ysize as uint32_t, histogram_bits as uint32_t)
    } else {
        1 as uint32_t
    }) as ::core::ffi::c_int;
    let image_histo_raw_size: ::core::ffi::c_int = histo_xsize * histo_ysize;
    let orig_histo: *mut VP8LHistogramSet =
        VP8LAllocateHistogramSet(image_histo_raw_size, cache_bits) as *mut VP8LHistogramSet;
    let entropy_combine_num_bins: ::core::ffi::c_int = if low_effort != 0 {
        NUM_PARTITIONS
    } else {
        BIN_SIZE
    };
    let mut entropy_combine: ::core::ffi::c_int = 0;
    let map_tmp: *mut uint16_t = WebPSafeMalloc(
        (2 as ::core::ffi::c_int * image_histo_raw_size) as uint64_t,
        ::core::mem::size_of::<uint16_t>() as size_t,
    ) as *mut uint16_t;
    let cluster_mappings: *mut uint16_t = map_tmp.offset(image_histo_raw_size as isize);
    let mut num_used: ::core::ffi::c_int = image_histo_raw_size;
    if orig_histo.is_null() || map_tmp.is_null() {
        WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    } else {
        HistogramBuild(xsize, histogram_bits, refs, orig_histo);
        HistogramCopyAndAnalyze(
            orig_histo,
            image_histo,
            &raw mut num_used,
            histogram_symbols,
        );
        entropy_combine = (num_used > entropy_combine_num_bins * 2 as ::core::ffi::c_int
            && quality < 100 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if entropy_combine != 0 {
            let bin_map: *mut uint16_t = map_tmp;
            let combine_cost_factor: ::core::ffi::c_float =
                GetCombineCostFactor(image_histo_raw_size, quality) as ::core::ffi::c_float;
            let num_clusters: uint32_t = num_used as uint32_t;
            HistogramAnalyzeEntropyBin(image_histo, bin_map, low_effort);
            HistogramCombineEntropyBin(
                image_histo,
                &raw mut num_used,
                histogram_symbols,
                cluster_mappings,
                tmp_histo,
                bin_map,
                entropy_combine_num_bins,
                combine_cost_factor,
                low_effort,
            );
            OptimizeHistogramSymbols(
                image_histo,
                cluster_mappings,
                num_clusters as ::core::ffi::c_int,
                map_tmp,
                histogram_symbols,
            );
        }
        if low_effort == 0 || entropy_combine == 0 {
            let x: ::core::ffi::c_float = quality as ::core::ffi::c_float / 100.0f32;
            let threshold_size: ::core::ffi::c_int = (1 as ::core::ffi::c_int
                as ::core::ffi::c_float
                + x * x * x * (MAX_HISTO_GREEDY - 1 as ::core::ffi::c_int) as ::core::ffi::c_float)
                as ::core::ffi::c_int;
            let mut do_greedy: ::core::ffi::c_int = 0;
            if HistogramCombineStochastic(
                image_histo,
                &raw mut num_used,
                threshold_size,
                &raw mut do_greedy,
            ) == 0
            {
                WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
                current_block = 17934864625402247792;
            } else if do_greedy != 0 {
                RemoveEmptyHistograms(image_histo);
                if HistogramCombineGreedy(image_histo, &raw mut num_used) == 0 {
                    WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
                    current_block = 17934864625402247792;
                } else {
                    current_block = 11584701595673473500;
                }
            } else {
                current_block = 11584701595673473500;
            }
        } else {
            current_block = 11584701595673473500;
        }
        match current_block {
            17934864625402247792 => {}
            _ => {
                RemoveEmptyHistograms(image_histo);
                HistogramRemap(orig_histo, image_histo, histogram_symbols);
                WebPReportProgress(pic, *percent + percent_range, percent) == 0;
            }
        }
    }
    VP8LFreeHistogramSet(orig_histo);
    WebPSafeFree(map_tmp as *mut ::core::ffi::c_void);
    return ((*pic).error_code as ::core::ffi::c_uint
        == VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
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
#[inline]
unsafe extern "C" fn VP8LPrefixEncodeBitsNoLUT(
    mut distance: ::core::ffi::c_int,
    code: *mut ::core::ffi::c_int,
    extra_bits: *mut ::core::ffi::c_int,
) {
    distance -= 1;
    let highest_bit: ::core::ffi::c_int = BitsLog2Floor(distance as uint32_t) as ::core::ffi::c_int;
    let second_highest_bit: ::core::ffi::c_int =
        distance >> highest_bit - 1 as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
    *extra_bits = highest_bit - 1 as ::core::ffi::c_int;
    *code = 2 as ::core::ffi::c_int * highest_bit + second_highest_bit;
}
pub const PREFIX_LOOKUP_IDX_MAX: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn VP8LPrefixEncodeBits(
    mut distance: ::core::ffi::c_int,
    code: *mut ::core::ffi::c_int,
    extra_bits: *mut ::core::ffi::c_int,
) {
    if distance < PREFIX_LOOKUP_IDX_MAX {
        let prefix_code: VP8LPrefixCode = kPrefixEncodeCode[distance as usize];
        *code = prefix_code.code_ as ::core::ffi::c_int;
        *extra_bits = prefix_code.extra_bits_ as ::core::ffi::c_int;
    } else {
        VP8LPrefixEncodeBitsNoLUT(distance, code, extra_bits);
    };
}
pub const FLT_MAX: ::core::ffi::c_float = __FLT_MAX__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const WEBP_ALIGN_CST: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn BitsLog2Floor(mut n: uint32_t) -> ::core::ffi::c_int {
    return 31 as ::core::ffi::c_int ^ n.leading_zeros() as i32;
}
pub const __FLT_MAX__: ::core::ffi::c_float = 3.40282347e+38f32;
