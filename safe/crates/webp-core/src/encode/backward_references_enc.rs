extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn WebPEncodingSetError(
        pic: *const WebPPicture,
        error: WebPEncodingError,
    ) -> ::core::ffi::c_int;
    fn WebPReportProgress(
        pic: *const WebPPicture,
        percent: ::core::ffi::c_int,
        percent_store: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
    static mut VP8LVectorMismatch: VP8LVectorMismatchFunc;
    static kPrefixEncodeCode: [VP8LPrefixCode; 512];
    static kPrefixEncodeExtraBitsValue: [uint8_t; 512];
    fn VP8LHistogramCreate(
        p: *mut VP8LHistogram,
        refs: *const VP8LBackwardRefs,
        palette_code_bits: ::core::ffi::c_int,
    );
    fn VP8LHistogramInit(
        p: *mut VP8LHistogram,
        palette_code_bits: ::core::ffi::c_int,
        init_arrays: ::core::ffi::c_int,
    );
    fn VP8LFreeHistogram(histo: *mut VP8LHistogram);
    fn VP8LAllocateHistogram(cache_bits: ::core::ffi::c_int) -> *mut VP8LHistogram;
    fn VP8LHistogramEstimateBits(p: *mut VP8LHistogram) -> ::core::ffi::c_float;
    fn VP8LColorCacheInit(
        color_cache: *mut VP8LColorCache,
        hash_bits: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VP8LColorCacheClear(color_cache: *mut VP8LColorCache);
    fn VP8LBackwardReferencesTraceBackwards(
        xsize: ::core::ffi::c_int,
        ysize: ::core::ffi::c_int,
        argb: *const uint32_t,
        cache_bits: ::core::ffi::c_int,
        hash_chain: *const VP8LHashChain,
        refs_src: *const VP8LBackwardRefs,
        refs_dst: *mut VP8LBackwardRefs,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub struct VP8LHashChain {
    pub offset_length_: *mut uint32_t,
    pub size_: ::core::ffi::c_int,
}
pub type VP8LVectorMismatchFunc = Option<
    unsafe extern "C" fn(
        *const uint32_t,
        *const uint32_t,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PixOrCopyBlock {
    pub next_: *mut PixOrCopyBlock,
    pub start_: *mut PixOrCopy,
    pub size_: ::core::ffi::c_int,
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
pub type VP8LLZ77Type = ::core::ffi::c_uint;
pub const kLZ77Box: VP8LLZ77Type = 4;
pub const kLZ77RLE: VP8LLZ77Type = 2;
pub const kLZ77Standard: VP8LLZ77Type = 1;
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
pub struct VP8LColorCache {
    pub colors_: *mut uint32_t,
    pub hash_shift_: ::core::ffi::c_int,
    pub hash_bits_: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LPrefixCode {
    pub code_: int8_t,
    pub extra_bits_: int8_t,
}
pub const NUM_LITERAL_CODES: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const NUM_LENGTH_CODES: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const MAX_COLOR_CACHE_BITS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn PixOrCopyCreateCopy(mut distance: uint32_t, mut len: uint16_t) -> PixOrCopy {
    let mut retval: PixOrCopy = PixOrCopy {
        mode: 0,
        len: 0,
        argb_or_distance: 0,
    };
    retval.mode = kCopy as ::core::ffi::c_int as uint8_t;
    retval.argb_or_distance = distance;
    retval.len = len;
    return retval;
}
#[inline]
unsafe extern "C" fn PixOrCopyCreateCacheIdx(mut idx: ::core::ffi::c_int) -> PixOrCopy {
    let mut retval: PixOrCopy = PixOrCopy {
        mode: 0,
        len: 0,
        argb_or_distance: 0,
    };
    '_c2rust_label: {
        if idx >= 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"idx >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/enc/backward_references_enc.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                57 as ::core::ffi::c_uint,
                b"PixOrCopy PixOrCopyCreateCacheIdx(int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if idx < (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"idx < (1 << MAX_COLOR_CACHE_BITS)\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/enc/backward_references_enc.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                58 as ::core::ffi::c_uint,
                b"PixOrCopy PixOrCopyCreateCacheIdx(int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    retval.mode = kCacheIdx as ::core::ffi::c_int as uint8_t;
    retval.argb_or_distance = idx as uint32_t;
    retval.len = 1 as uint16_t;
    return retval;
}
#[inline]
unsafe extern "C" fn PixOrCopyCreateLiteral(mut argb: uint32_t) -> PixOrCopy {
    let mut retval: PixOrCopy = PixOrCopy {
        mode: 0,
        len: 0,
        argb_or_distance: 0,
    };
    retval.mode = kLiteral as ::core::ffi::c_int as uint8_t;
    retval.argb_or_distance = argb;
    retval.len = 1 as uint16_t;
    return retval;
}
#[inline]
unsafe extern "C" fn PixOrCopyIsLiteral(p: *const PixOrCopy) -> ::core::ffi::c_int {
    return ((*p).mode as ::core::ffi::c_int == kLiteral as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn PixOrCopyIsCopy(p: *const PixOrCopy) -> ::core::ffi::c_int {
    return ((*p).mode as ::core::ffi::c_int == kCopy as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn PixOrCopyLength(p: *const PixOrCopy) -> uint32_t {
    return (*p).len as uint32_t;
}
pub const HASH_BITS: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const HASH_SIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << HASH_BITS;
pub const MAX_LENGTH_BITS: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const WINDOW_SIZE_BITS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const MAX_LENGTH: ::core::ffi::c_int =
    ((1 as ::core::ffi::c_int) << MAX_LENGTH_BITS) - 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn VP8LHashChainFindOffset(
    p: *const VP8LHashChain,
    base_position: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (*(*p).offset_length_.offset(base_position as isize) >> MAX_LENGTH_BITS)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn VP8LHashChainFindLength(
    p: *const VP8LHashChain,
    base_position: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (*(*p).offset_length_.offset(base_position as isize)
        & ((1 as uint32_t) << MAX_LENGTH_BITS).wrapping_sub(1 as uint32_t))
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn VP8LHashChainFindCopy(
    p: *const VP8LHashChain,
    mut base_position: ::core::ffi::c_int,
    offset_ptr: *mut ::core::ffi::c_int,
    length_ptr: *mut ::core::ffi::c_int,
) {
    *offset_ptr = VP8LHashChainFindOffset(p, base_position);
    *length_ptr = VP8LHashChainFindLength(p, base_position);
}
#[inline]
unsafe extern "C" fn VP8LRefsCursorOk(c: *const VP8LRefsCursor) -> ::core::ffi::c_int {
    return ((*c).cur_pos != NULL_0 as *mut PixOrCopy) as ::core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn BitsLog2Floor(mut n: uint32_t) -> ::core::ffi::c_int {
    return 31 as ::core::ffi::c_int ^ n.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn VP8LPrefixEncodeNoLUT(
    mut distance: ::core::ffi::c_int,
    code: *mut ::core::ffi::c_int,
    extra_bits: *mut ::core::ffi::c_int,
    extra_bits_value: *mut ::core::ffi::c_int,
) {
    distance -= 1;
    let highest_bit: ::core::ffi::c_int = BitsLog2Floor(distance as uint32_t) as ::core::ffi::c_int;
    let second_highest_bit: ::core::ffi::c_int =
        distance >> highest_bit - 1 as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
    *extra_bits = highest_bit - 1 as ::core::ffi::c_int;
    *extra_bits_value =
        distance & ((1 as ::core::ffi::c_int) << *extra_bits) - 1 as ::core::ffi::c_int;
    *code = 2 as ::core::ffi::c_int * highest_bit + second_highest_bit;
}
pub const PREFIX_LOOKUP_IDX_MAX: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn VP8LPrefixEncode(
    mut distance: ::core::ffi::c_int,
    code: *mut ::core::ffi::c_int,
    extra_bits: *mut ::core::ffi::c_int,
    extra_bits_value: *mut ::core::ffi::c_int,
) {
    if distance < PREFIX_LOOKUP_IDX_MAX {
        let prefix_code: VP8LPrefixCode = kPrefixEncodeCode[distance as usize];
        *code = prefix_code.code_ as ::core::ffi::c_int;
        *extra_bits = prefix_code.extra_bits_ as ::core::ffi::c_int;
        *extra_bits_value = kPrefixEncodeExtraBitsValue[distance as usize] as ::core::ffi::c_int;
    } else {
        VP8LPrefixEncodeNoLUT(distance, code, extra_bits, extra_bits_value);
    };
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
unsafe extern "C" fn VP8LColorCacheSet(
    cc: *const VP8LColorCache,
    mut key: uint32_t,
    mut argb: uint32_t,
) {
    '_c2rust_label: {
        if key >> (*cc).hash_bits_ == 0 as uint32_t {
        } else {
            __assert_fail(
                b"(key >> cc->hash_bits_) == 0u\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/utils/color_cache_utils.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                49 as ::core::ffi::c_uint,
                b"void VP8LColorCacheSet(const VP8LColorCache *const, uint32_t, uint32_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    *(*cc).colors_.offset(key as isize) = argb;
}
#[inline]
unsafe extern "C" fn VP8LColorCacheInsert(cc: *const VP8LColorCache, mut argb: uint32_t) {
    let key: ::core::ffi::c_int = VP8LHashPix(argb, (*cc).hash_shift_) as ::core::ffi::c_int;
    *(*cc).colors_.offset(key as isize) = argb;
}
#[inline]
unsafe extern "C" fn VP8LColorCacheGetIndex(
    cc: *const VP8LColorCache,
    mut argb: uint32_t,
) -> ::core::ffi::c_int {
    return VP8LHashPix(argb, (*cc).hash_shift_);
}
#[inline]
unsafe extern "C" fn VP8LColorCacheContains(
    cc: *const VP8LColorCache,
    mut argb: uint32_t,
) -> ::core::ffi::c_int {
    let key: ::core::ffi::c_int = VP8LHashPix(argb, (*cc).hash_shift_) as ::core::ffi::c_int;
    return if *(*cc).colors_.offset(key as isize) == argb {
        key
    } else {
        -(1 as ::core::ffi::c_int)
    };
}
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FLT_MAX: ::core::ffi::c_float = __FLT_MAX__;
pub const MIN_BLOCK_SIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MAX_ENTROPY: ::core::ffi::c_float = 1e30f32;
pub const WINDOW_SIZE: ::core::ffi::c_int =
    ((1 as ::core::ffi::c_int) << WINDOW_SIZE_BITS) - 120 as ::core::ffi::c_int;
pub const MIN_LENGTH: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
static mut plane_to_code_lut: [uint8_t; 128] = [
    96 as ::core::ffi::c_int as uint8_t,
    73 as ::core::ffi::c_int as uint8_t,
    55 as ::core::ffi::c_int as uint8_t,
    39 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    255 as ::core::ffi::c_int as uint8_t,
    255 as ::core::ffi::c_int as uint8_t,
    255 as ::core::ffi::c_int as uint8_t,
    255 as ::core::ffi::c_int as uint8_t,
    255 as ::core::ffi::c_int as uint8_t,
    255 as ::core::ffi::c_int as uint8_t,
    255 as ::core::ffi::c_int as uint8_t,
    255 as ::core::ffi::c_int as uint8_t,
    101 as ::core::ffi::c_int as uint8_t,
    78 as ::core::ffi::c_int as uint8_t,
    58 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    17 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    43 as ::core::ffi::c_int as uint8_t,
    59 as ::core::ffi::c_int as uint8_t,
    79 as ::core::ffi::c_int as uint8_t,
    102 as ::core::ffi::c_int as uint8_t,
    86 as ::core::ffi::c_int as uint8_t,
    62 as ::core::ffi::c_int as uint8_t,
    46 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    47 as ::core::ffi::c_int as uint8_t,
    63 as ::core::ffi::c_int as uint8_t,
    87 as ::core::ffi::c_int as uint8_t,
    105 as ::core::ffi::c_int as uint8_t,
    90 as ::core::ffi::c_int as uint8_t,
    70 as ::core::ffi::c_int as uint8_t,
    52 as ::core::ffi::c_int as uint8_t,
    37 as ::core::ffi::c_int as uint8_t,
    28 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    12 as ::core::ffi::c_int as uint8_t,
    15 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    29 as ::core::ffi::c_int as uint8_t,
    38 as ::core::ffi::c_int as uint8_t,
    53 as ::core::ffi::c_int as uint8_t,
    71 as ::core::ffi::c_int as uint8_t,
    91 as ::core::ffi::c_int as uint8_t,
    110 as ::core::ffi::c_int as uint8_t,
    99 as ::core::ffi::c_int as uint8_t,
    82 as ::core::ffi::c_int as uint8_t,
    66 as ::core::ffi::c_int as uint8_t,
    48 as ::core::ffi::c_int as uint8_t,
    35 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    31 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    49 as ::core::ffi::c_int as uint8_t,
    67 as ::core::ffi::c_int as uint8_t,
    83 as ::core::ffi::c_int as uint8_t,
    100 as ::core::ffi::c_int as uint8_t,
    115 as ::core::ffi::c_int as uint8_t,
    108 as ::core::ffi::c_int as uint8_t,
    94 as ::core::ffi::c_int as uint8_t,
    76 as ::core::ffi::c_int as uint8_t,
    64 as ::core::ffi::c_int as uint8_t,
    50 as ::core::ffi::c_int as uint8_t,
    44 as ::core::ffi::c_int as uint8_t,
    40 as ::core::ffi::c_int as uint8_t,
    34 as ::core::ffi::c_int as uint8_t,
    41 as ::core::ffi::c_int as uint8_t,
    45 as ::core::ffi::c_int as uint8_t,
    51 as ::core::ffi::c_int as uint8_t,
    65 as ::core::ffi::c_int as uint8_t,
    77 as ::core::ffi::c_int as uint8_t,
    95 as ::core::ffi::c_int as uint8_t,
    109 as ::core::ffi::c_int as uint8_t,
    118 as ::core::ffi::c_int as uint8_t,
    113 as ::core::ffi::c_int as uint8_t,
    103 as ::core::ffi::c_int as uint8_t,
    92 as ::core::ffi::c_int as uint8_t,
    80 as ::core::ffi::c_int as uint8_t,
    68 as ::core::ffi::c_int as uint8_t,
    60 as ::core::ffi::c_int as uint8_t,
    56 as ::core::ffi::c_int as uint8_t,
    54 as ::core::ffi::c_int as uint8_t,
    57 as ::core::ffi::c_int as uint8_t,
    61 as ::core::ffi::c_int as uint8_t,
    69 as ::core::ffi::c_int as uint8_t,
    81 as ::core::ffi::c_int as uint8_t,
    93 as ::core::ffi::c_int as uint8_t,
    104 as ::core::ffi::c_int as uint8_t,
    114 as ::core::ffi::c_int as uint8_t,
    119 as ::core::ffi::c_int as uint8_t,
    116 as ::core::ffi::c_int as uint8_t,
    111 as ::core::ffi::c_int as uint8_t,
    106 as ::core::ffi::c_int as uint8_t,
    97 as ::core::ffi::c_int as uint8_t,
    88 as ::core::ffi::c_int as uint8_t,
    84 as ::core::ffi::c_int as uint8_t,
    74 as ::core::ffi::c_int as uint8_t,
    72 as ::core::ffi::c_int as uint8_t,
    75 as ::core::ffi::c_int as uint8_t,
    85 as ::core::ffi::c_int as uint8_t,
    89 as ::core::ffi::c_int as uint8_t,
    98 as ::core::ffi::c_int as uint8_t,
    107 as ::core::ffi::c_int as uint8_t,
    112 as ::core::ffi::c_int as uint8_t,
    117 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub unsafe extern "C" fn VP8LDistanceToPlaneCode(
    mut xsize: ::core::ffi::c_int,
    mut dist: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let yoffset: ::core::ffi::c_int = dist / xsize;
    let xoffset: ::core::ffi::c_int = dist - yoffset * xsize;
    if xoffset <= 8 as ::core::ffi::c_int && yoffset < 8 as ::core::ffi::c_int {
        return plane_to_code_lut
            [(yoffset * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int - xoffset) as usize]
            as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int;
    } else if xoffset > xsize - 8 as ::core::ffi::c_int && yoffset < 7 as ::core::ffi::c_int {
        return plane_to_code_lut[((yoffset + 1 as ::core::ffi::c_int) * 16 as ::core::ffi::c_int
            + 8 as ::core::ffi::c_int
            + (xsize - xoffset)) as usize] as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int;
    }
    return dist + 120 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn FindMatchLength(
    array1: *const uint32_t,
    array2: *const uint32_t,
    mut best_len_match: ::core::ffi::c_int,
    mut max_limit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if *array1.offset(best_len_match as isize) != *array2.offset(best_len_match as isize) {
        return 0 as ::core::ffi::c_int;
    }
    return VP8LVectorMismatch.expect("non-null function pointer")(array1, array2, max_limit);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LClearBackwardRefs(refs: *mut VP8LBackwardRefs) {
    '_c2rust_label: {
        if !refs.is_null() {
        } else {
            __assert_fail(
                b"refs != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                90 as ::core::ffi::c_uint,
                b"void VP8LClearBackwardRefs(VP8LBackwardRefs *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if !(*refs).tail_.is_null() {
        *(*refs).tail_ = (*refs).free_blocks_;
    }
    (*refs).free_blocks_ = (*refs).refs_;
    (*refs).tail_ = &raw mut (*refs).refs_;
    (*refs).last_block_ = ::core::ptr::null_mut::<PixOrCopyBlock>();
    (*refs).refs_ = ::core::ptr::null_mut::<PixOrCopyBlock>();
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBackwardRefsClear(refs: *mut VP8LBackwardRefs) {
    '_c2rust_label: {
        if !refs.is_null() {
        } else {
            __assert_fail(
                b"refs != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                101 as ::core::ffi::c_uint,
                b"void VP8LBackwardRefsClear(VP8LBackwardRefs *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    VP8LClearBackwardRefs(refs);
    while !(*refs).free_blocks_.is_null() {
        let next: *mut PixOrCopyBlock = (*(*refs).free_blocks_).next_;
        WebPSafeFree((*refs).free_blocks_ as *mut ::core::ffi::c_void);
        (*refs).free_blocks_ = next;
    }
}
unsafe extern "C" fn BackwardRefsSwap(refs1: *mut VP8LBackwardRefs, refs2: *mut VP8LBackwardRefs) {
    let point_to_refs1: ::core::ffi::c_int = (!(*refs1).tail_.is_null()
        && (*refs1).tail_ == &raw mut (*refs1).refs_)
        as ::core::ffi::c_int;
    let point_to_refs2: ::core::ffi::c_int = (!(*refs2).tail_.is_null()
        && (*refs2).tail_ == &raw mut (*refs2).refs_)
        as ::core::ffi::c_int;
    let tmp: VP8LBackwardRefs = *refs1;
    *refs1 = *refs2;
    *refs2 = tmp;
    if point_to_refs2 != 0 {
        (*refs1).tail_ = &raw mut (*refs1).refs_;
    }
    if point_to_refs1 != 0 {
        (*refs2).tail_ = &raw mut (*refs2).refs_;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBackwardRefsInit(
    refs: *mut VP8LBackwardRefs,
    mut block_size: ::core::ffi::c_int,
) {
    '_c2rust_label: {
        if !refs.is_null() {
        } else {
            __assert_fail(
                b"refs != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                125 as ::core::ffi::c_uint,
                b"void VP8LBackwardRefsInit(VP8LBackwardRefs *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    memset(
        refs as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VP8LBackwardRefs>() as size_t,
    );
    (*refs).tail_ = &raw mut (*refs).refs_;
    (*refs).block_size_ = if block_size < MIN_BLOCK_SIZE {
        MIN_BLOCK_SIZE
    } else {
        block_size
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LRefsCursorInit(refs: *const VP8LBackwardRefs) -> VP8LRefsCursor {
    let mut c: VP8LRefsCursor = VP8LRefsCursor {
        cur_pos: ::core::ptr::null_mut::<PixOrCopy>(),
        cur_block_: ::core::ptr::null_mut::<PixOrCopyBlock>(),
        last_pos_: ::core::ptr::null::<PixOrCopy>(),
    };
    c.cur_block_ = (*refs).refs_;
    if !(*refs).refs_.is_null() {
        c.cur_pos = (*c.cur_block_).start_;
        c.last_pos_ = c.cur_pos.offset((*c.cur_block_).size_ as isize);
    } else {
        c.cur_pos = ::core::ptr::null_mut::<PixOrCopy>();
        c.last_pos_ = ::core::ptr::null::<PixOrCopy>();
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LRefsCursorNextBlock(c: *mut VP8LRefsCursor) {
    let b: *mut PixOrCopyBlock = (*(*c).cur_block_).next_;
    (*c).cur_pos = if b.is_null() {
        ::core::ptr::null_mut::<PixOrCopy>()
    } else {
        (*b).start_
    };
    (*c).last_pos_ = if b.is_null() {
        ::core::ptr::null_mut::<PixOrCopy>()
    } else {
        (*b).start_.offset((*b).size_ as isize)
    };
    (*c).cur_block_ = b;
}
unsafe extern "C" fn BackwardRefsNewBlock(refs: *mut VP8LBackwardRefs) -> *mut PixOrCopyBlock {
    let mut b: *mut PixOrCopyBlock = (*refs).free_blocks_;
    if b.is_null() {
        let total_size: size_t = (::core::mem::size_of::<PixOrCopyBlock>() as size_t).wrapping_add(
            ((*refs).block_size_ as size_t)
                .wrapping_mul(::core::mem::size_of::<PixOrCopy>() as size_t),
        );
        b = WebPSafeMalloc(1 as uint64_t, total_size) as *mut PixOrCopyBlock;
        if b.is_null() {
            (*refs).error_ |= 1 as ::core::ffi::c_int;
            return ::core::ptr::null_mut::<PixOrCopyBlock>();
        }
        (*b).start_ = (b as *mut uint8_t)
            .offset(::core::mem::size_of::<PixOrCopyBlock>() as usize as isize)
            as *mut PixOrCopy;
    } else {
        (*refs).free_blocks_ = (*b).next_;
    }
    *(*refs).tail_ = b;
    (*refs).tail_ = &raw mut (*b).next_;
    (*refs).last_block_ = b;
    (*b).next_ = ::core::ptr::null_mut::<PixOrCopyBlock>();
    (*b).size_ = 0 as ::core::ffi::c_int;
    return b;
}
unsafe extern "C" fn BackwardRefsClone(
    from: *const VP8LBackwardRefs,
    to: *mut VP8LBackwardRefs,
) -> ::core::ffi::c_int {
    let mut block_from: *const PixOrCopyBlock = (*from).refs_;
    VP8LClearBackwardRefs(to);
    while !block_from.is_null() {
        let block_to: *mut PixOrCopyBlock = BackwardRefsNewBlock(to) as *mut PixOrCopyBlock;
        if block_to.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        memcpy(
            (*block_to).start_ as *mut ::core::ffi::c_void,
            (*block_from).start_ as *const ::core::ffi::c_void,
            ((*block_from).size_ as size_t)
                .wrapping_mul(::core::mem::size_of::<PixOrCopy>() as size_t),
        );
        (*block_to).size_ = (*block_from).size_;
        block_from = (*block_from).next_;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBackwardRefsCursorAdd(refs: *mut VP8LBackwardRefs, v: PixOrCopy) {
    let mut b: *mut PixOrCopyBlock = (*refs).last_block_;
    if b.is_null() || (*b).size_ == (*refs).block_size_ {
        b = BackwardRefsNewBlock(refs);
        if b.is_null() {
            return;
        }
    }
    let fresh12 = (*b).size_;
    (*b).size_ = (*b).size_ + 1;
    *(*b).start_.offset(fresh12 as isize) = v;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHashChainInit(
    p: *mut VP8LHashChain,
    mut size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if (*p).size_ == 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"p->size_ == 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                207 as ::core::ffi::c_uint,
                b"int VP8LHashChainInit(VP8LHashChain *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*p).offset_length_.is_null() {
        } else {
            __assert_fail(
                b"p->offset_length_ == NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                208 as ::core::ffi::c_uint,
                b"int VP8LHashChainInit(VP8LHashChain *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if size > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"size > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                209 as ::core::ffi::c_uint,
                b"int VP8LHashChainInit(VP8LHashChain *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*p).offset_length_ = WebPSafeMalloc(
        size as uint64_t,
        ::core::mem::size_of::<uint32_t>() as size_t,
    ) as *mut uint32_t;
    if (*p).offset_length_.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*p).size_ = size;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHashChainClear(p: *mut VP8LHashChain) {
    '_c2rust_label: {
        if !p.is_null() {
        } else {
            __assert_fail(
                b"p != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                219 as ::core::ffi::c_uint,
                b"void VP8LHashChainClear(VP8LHashChain *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    WebPSafeFree((*p).offset_length_ as *mut ::core::ffi::c_void);
    (*p).size_ = 0 as ::core::ffi::c_int;
    (*p).offset_length_ = ::core::ptr::null_mut::<uint32_t>();
}
static mut kHashMultiplierHi: uint32_t = 0xc6a4a793 as uint32_t;
static mut kHashMultiplierLo: uint32_t = 0x5bd1e996 as uint32_t;
#[inline]
unsafe extern "C" fn GetPixPairHash64(argb: *const uint32_t) -> uint32_t {
    let mut key: uint32_t = 0;
    key = (*argb.offset(1 as ::core::ffi::c_int as isize)).wrapping_mul(kHashMultiplierHi);
    key = key.wrapping_add(
        (*argb.offset(0 as ::core::ffi::c_int as isize)).wrapping_mul(kHashMultiplierLo),
    );
    key = key >> 32 as ::core::ffi::c_int - HASH_BITS;
    return key;
}
unsafe extern "C" fn GetMaxItersForQuality(mut quality: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return 8 as ::core::ffi::c_int + quality * quality / 128 as ::core::ffi::c_int;
}
unsafe extern "C" fn GetWindowSizeForHashChain(
    mut quality: ::core::ffi::c_int,
    mut xsize: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let max_window_size: ::core::ffi::c_int = if quality > 75 as ::core::ffi::c_int {
        WINDOW_SIZE
    } else if quality > 50 as ::core::ffi::c_int {
        xsize << 8 as ::core::ffi::c_int
    } else if quality > 25 as ::core::ffi::c_int {
        xsize << 6 as ::core::ffi::c_int
    } else {
        xsize << 4 as ::core::ffi::c_int
    };
    '_c2rust_label: {
        if xsize > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"xsize > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                251 as ::core::ffi::c_uint,
                b"int GetWindowSizeForHashChain(int, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return if max_window_size > WINDOW_SIZE {
        WINDOW_SIZE
    } else {
        max_window_size
    };
}
#[inline]
unsafe extern "C" fn MaxFindCopyLength(mut len: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if len < MAX_LENGTH { len } else { MAX_LENGTH };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHashChainFill(
    p: *mut VP8LHashChain,
    mut quality: ::core::ffi::c_int,
    argb: *const uint32_t,
    mut xsize: ::core::ffi::c_int,
    mut ysize: ::core::ffi::c_int,
    mut low_effort: ::core::ffi::c_int,
    pic: *const WebPPicture,
    mut percent_range: ::core::ffi::c_int,
    percent: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let size: ::core::ffi::c_int = xsize * ysize;
    let iter_max: ::core::ffi::c_int = GetMaxItersForQuality(quality) as ::core::ffi::c_int;
    let window_size: uint32_t = GetWindowSizeForHashChain(quality, xsize) as uint32_t;
    let mut remaining_percent: ::core::ffi::c_int = percent_range;
    let mut percent_start: ::core::ffi::c_int = *percent;
    let mut pos: ::core::ffi::c_int = 0;
    let mut argb_comp: ::core::ffi::c_int = 0;
    let mut base_position: uint32_t = 0;
    let mut hash_to_first_index: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
    let mut chain: *mut int32_t = (*p).offset_length_ as *mut int32_t;
    '_c2rust_label: {
        if size > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"size > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                274 as ::core::ffi::c_uint,
                b"int VP8LHashChainFill(VP8LHashChain *const, int, const uint32_t *const, int, int, int, const WebPPicture *const, int, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*p).size_ != 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"p->size_ != 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                275 as ::core::ffi::c_uint,
                b"int VP8LHashChainFill(VP8LHashChain *const, int, const uint32_t *const, int, int, int, const WebPPicture *const, int, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !(*p).offset_length_.is_null() {
        } else {
            __assert_fail(
                b"p->offset_length_ != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                276 as ::core::ffi::c_uint,
                b"int VP8LHashChainFill(VP8LHashChain *const, int, const uint32_t *const, int, int, int, const WebPPicture *const, int, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if size <= 2 as ::core::ffi::c_int {
        let ref mut fresh0 = *(*p)
            .offset_length_
            .offset((size - 1 as ::core::ffi::c_int) as isize);
        *fresh0 = 0 as uint32_t;
        *(*p).offset_length_.offset(0 as ::core::ffi::c_int as isize) = *fresh0;
        return 1 as ::core::ffi::c_int;
    }
    hash_to_first_index = WebPSafeMalloc(
        HASH_SIZE as uint64_t,
        ::core::mem::size_of::<int32_t>() as size_t,
    ) as *mut int32_t;
    if hash_to_first_index.is_null() {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    percent_range = remaining_percent / 2 as ::core::ffi::c_int;
    remaining_percent -= percent_range;
    memset(
        hash_to_first_index as *mut ::core::ffi::c_void,
        0xff as ::core::ffi::c_int,
        (HASH_SIZE as size_t).wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
    );
    argb_comp = (*argb.offset(0 as ::core::ffi::c_int as isize)
        == *argb.offset(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int;
    pos = 0 as ::core::ffi::c_int;
    while pos < size - 2 as ::core::ffi::c_int {
        let mut hash_code: uint32_t = 0;
        let argb_comp_next: ::core::ffi::c_int = (*argb
            .offset((pos + 1 as ::core::ffi::c_int) as isize)
            == *argb.offset((pos + 2 as ::core::ffi::c_int) as isize))
            as ::core::ffi::c_int;
        if argb_comp != 0 && argb_comp_next != 0 {
            let mut tmp: [uint32_t; 2] = [0; 2];
            let mut len: uint32_t = 1 as uint32_t;
            tmp[0 as ::core::ffi::c_int as usize] = *argb.offset(pos as isize);
            while (pos + len as ::core::ffi::c_int + 2 as ::core::ffi::c_int) < size
                && *argb.offset(
                    (pos as uint32_t)
                        .wrapping_add(len)
                        .wrapping_add(2 as uint32_t) as isize,
                ) == *argb.offset(pos as isize)
            {
                len = len.wrapping_add(1);
            }
            if len > MAX_LENGTH as uint32_t {
                memset(
                    chain.offset(pos as isize) as *mut ::core::ffi::c_void,
                    0xff as ::core::ffi::c_int,
                    (len.wrapping_sub(MAX_LENGTH as uint32_t) as size_t)
                        .wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
                );
                pos = (pos as uint32_t).wrapping_add(len.wrapping_sub(MAX_LENGTH as uint32_t))
                    as ::core::ffi::c_int as ::core::ffi::c_int;
                len = MAX_LENGTH as uint32_t;
            }
            while len != 0 {
                let fresh1 = len;
                len = len.wrapping_sub(1);
                tmp[1 as ::core::ffi::c_int as usize] = fresh1;
                hash_code = GetPixPairHash64(&raw mut tmp as *mut uint32_t);
                *chain.offset(pos as isize) = *hash_to_first_index.offset(hash_code as isize);
                let fresh2 = pos;
                pos = pos + 1;
                *hash_to_first_index.offset(hash_code as isize) = fresh2 as int32_t;
            }
            argb_comp = 0 as ::core::ffi::c_int;
        } else {
            hash_code = GetPixPairHash64(argb.offset(pos as isize));
            *chain.offset(pos as isize) = *hash_to_first_index.offset(hash_code as isize);
            let fresh3 = pos;
            pos = pos + 1;
            *hash_to_first_index.offset(hash_code as isize) = fresh3 as int32_t;
            argb_comp = argb_comp_next;
        }
        if WebPReportProgress(
            pic,
            percent_start + percent_range * pos / (size - 2 as ::core::ffi::c_int),
            percent,
        ) == 0
        {
            WebPSafeFree(hash_to_first_index as *mut ::core::ffi::c_void);
            return 0 as ::core::ffi::c_int;
        }
    }
    *chain.offset(pos as isize) =
        *hash_to_first_index.offset(GetPixPairHash64(argb.offset(pos as isize)) as isize);
    WebPSafeFree(hash_to_first_index as *mut ::core::ffi::c_void);
    percent_start += percent_range;
    if WebPReportProgress(pic, percent_start, percent) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    percent_range = remaining_percent;
    '_c2rust_label_2: {
        if size > 2 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"size > 2\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                357 as ::core::ffi::c_uint,
                b"int VP8LHashChainFill(VP8LHashChain *const, int, const uint32_t *const, int, int, int, const WebPPicture *const, int, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    let ref mut fresh4 = *(*p)
        .offset_length_
        .offset((size - 1 as ::core::ffi::c_int) as isize);
    *fresh4 = 0 as uint32_t;
    *(*p).offset_length_.offset(0 as ::core::ffi::c_int as isize) = *fresh4;
    base_position = (size - 2 as ::core::ffi::c_int) as uint32_t;
    while base_position > 0 as uint32_t {
        let max_len: ::core::ffi::c_int = MaxFindCopyLength(
            ((size - 1 as ::core::ffi::c_int) as uint32_t).wrapping_sub(base_position)
                as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        let argb_start: *const uint32_t = argb.offset(base_position as isize);
        let mut iter: ::core::ffi::c_int = iter_max;
        let mut best_length: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut best_distance: uint32_t = 0 as uint32_t;
        let mut best_argb: uint32_t = 0;
        let min_pos: ::core::ffi::c_int = (if base_position > window_size {
            base_position.wrapping_sub(window_size)
        } else {
            0 as uint32_t
        }) as ::core::ffi::c_int;
        let length_max: ::core::ffi::c_int = if max_len < 256 as ::core::ffi::c_int {
            max_len
        } else {
            256 as ::core::ffi::c_int
        };
        let mut max_base_position: uint32_t = 0;
        pos = *chain.offset(base_position as isize) as ::core::ffi::c_int;
        if low_effort == 0 {
            let mut curr_length: ::core::ffi::c_int = 0;
            if base_position >= xsize as uint32_t {
                curr_length = FindMatchLength(
                    argb_start.offset(-(xsize as isize)),
                    argb_start,
                    best_length,
                    max_len,
                );
                if curr_length > best_length {
                    best_length = curr_length;
                    best_distance = xsize as uint32_t;
                }
                iter -= 1;
            }
            curr_length = FindMatchLength(
                argb_start.offset(-(1 as ::core::ffi::c_int as isize)),
                argb_start,
                best_length,
                max_len,
            );
            if curr_length > best_length {
                best_length = curr_length;
                best_distance = 1 as uint32_t;
            }
            iter -= 1;
            if best_length == MAX_LENGTH {
                pos = min_pos - 1 as ::core::ffi::c_int;
            }
        }
        best_argb = *argb_start.offset(best_length as isize);
        while pos >= min_pos && {
            iter -= 1;
            iter != 0
        } {
            let mut curr_length_0: ::core::ffi::c_int = 0;
            '_c2rust_label_3: {
                if base_position > pos as uint32_t {
                } else {
                    __assert_fail(
                        b"base_position > (uint32_t)pos\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        399 as ::core::ffi::c_uint,
                        b"int VP8LHashChainFill(VP8LHashChain *const, int, const uint32_t *const, int, int, int, const WebPPicture *const, int, int *const)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            if !(*argb.offset((pos + best_length) as isize) != best_argb) {
                curr_length_0 = VP8LVectorMismatch.expect("non-null function pointer")(
                    argb.offset(pos as isize),
                    argb_start,
                    max_len,
                );
                if best_length < curr_length_0 {
                    best_length = curr_length_0;
                    best_distance = base_position.wrapping_sub(pos as uint32_t);
                    best_argb = *argb_start.offset(best_length as isize);
                    if best_length >= length_max {
                        break;
                    }
                }
            }
            pos = *chain.offset(pos as isize) as ::core::ffi::c_int;
        }
        max_base_position = base_position;
        loop {
            '_c2rust_label_4: {
                if best_length
                    <= ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int
                {
                } else {
                    __assert_fail(
                        b"best_length <= MAX_LENGTH\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        416 as ::core::ffi::c_uint,
                        b"int VP8LHashChainFill(VP8LHashChain *const, int, const uint32_t *const, int, int, int, const WebPPicture *const, int, int *const)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            '_c2rust_label_5: {
                if best_distance
                    <= (((1 as ::core::ffi::c_int) << 20 as ::core::ffi::c_int)
                        - 120 as ::core::ffi::c_int) as uint32_t
                {
                } else {
                    __assert_fail(
                        b"best_distance <= WINDOW_SIZE\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        417 as ::core::ffi::c_uint,
                        b"int VP8LHashChainFill(VP8LHashChain *const, int, const uint32_t *const, int, int, int, const WebPPicture *const, int, int *const)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            *(*p).offset_length_.offset(base_position as isize) =
                best_distance << MAX_LENGTH_BITS | best_length as uint32_t;
            base_position = base_position.wrapping_sub(1);
            if best_distance == 0 as uint32_t || base_position == 0 as uint32_t {
                break;
            }
            if base_position < best_distance
                || *argb.offset(base_position.wrapping_sub(best_distance) as isize)
                    != *argb.offset(base_position as isize)
            {
                break;
            }
            if best_length == MAX_LENGTH
                && best_distance != 1 as uint32_t
                && base_position.wrapping_add(MAX_LENGTH as uint32_t) < max_base_position
            {
                break;
            }
            if best_length < MAX_LENGTH {
                best_length += 1;
                max_base_position = base_position;
            }
        }
        if WebPReportProgress(
            pic,
            (percent_start as uint32_t).wrapping_add(
                (percent_range as uint32_t)
                    .wrapping_mul(
                        ((size - 2 as ::core::ffi::c_int) as uint32_t).wrapping_sub(base_position),
                    )
                    .wrapping_div((size - 2 as ::core::ffi::c_int) as uint32_t),
            ) as ::core::ffi::c_int,
            percent,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    return WebPReportProgress(pic, percent_start + percent_range, percent);
}
#[inline]
unsafe extern "C" fn AddSingleLiteral(
    mut pixel: uint32_t,
    mut use_color_cache: ::core::ffi::c_int,
    hashers: *mut VP8LColorCache,
    refs: *mut VP8LBackwardRefs,
) {
    let mut v: PixOrCopy = PixOrCopy {
        mode: 0,
        len: 0,
        argb_or_distance: 0,
    };
    if use_color_cache != 0 {
        let key: uint32_t = VP8LColorCacheGetIndex(hashers, pixel) as uint32_t;
        if VP8LColorCacheLookup(hashers, key) == pixel {
            v = PixOrCopyCreateCacheIdx(key as ::core::ffi::c_int);
        } else {
            v = PixOrCopyCreateLiteral(pixel);
            VP8LColorCacheSet(hashers, key, pixel);
        }
    } else {
        v = PixOrCopyCreateLiteral(pixel);
    }
    VP8LBackwardRefsCursorAdd(refs, v);
}
unsafe extern "C" fn BackwardReferencesRle(
    mut xsize: ::core::ffi::c_int,
    mut ysize: ::core::ffi::c_int,
    argb: *const uint32_t,
    mut cache_bits: ::core::ffi::c_int,
    refs: *mut VP8LBackwardRefs,
) -> ::core::ffi::c_int {
    let pix_count: ::core::ffi::c_int = xsize * ysize;
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let use_color_cache: ::core::ffi::c_int =
        (cache_bits > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut hashers: VP8LColorCache = VP8LColorCache {
        colors_: ::core::ptr::null_mut::<uint32_t>(),
        hash_shift_: 0,
        hash_bits_: 0,
    };
    if use_color_cache != 0 && VP8LColorCacheInit(&raw mut hashers, cache_bits) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    VP8LClearBackwardRefs(refs);
    AddSingleLiteral(
        *argb.offset(0 as ::core::ffi::c_int as isize),
        use_color_cache,
        &raw mut hashers,
        refs,
    );
    i = 1 as ::core::ffi::c_int;
    while i < pix_count {
        let max_len: ::core::ffi::c_int = MaxFindCopyLength(pix_count - i) as ::core::ffi::c_int;
        let rle_len: ::core::ffi::c_int = FindMatchLength(
            argb.offset(i as isize),
            argb.offset(i as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)),
            0 as ::core::ffi::c_int,
            max_len,
        ) as ::core::ffi::c_int;
        let prev_row_len: ::core::ffi::c_int = if i < xsize {
            0 as ::core::ffi::c_int
        } else {
            FindMatchLength(
                argb.offset(i as isize),
                argb.offset(i as isize).offset(-(xsize as isize)),
                0 as ::core::ffi::c_int,
                max_len,
            ) as ::core::ffi::c_int
        };
        if rle_len >= prev_row_len && rle_len >= MIN_LENGTH {
            VP8LBackwardRefsCursorAdd(
                refs,
                PixOrCopyCreateCopy(1 as uint32_t, rle_len as uint16_t),
            );
            i += rle_len;
        } else if prev_row_len >= MIN_LENGTH {
            VP8LBackwardRefsCursorAdd(
                refs,
                PixOrCopyCreateCopy(xsize as uint32_t, prev_row_len as uint16_t),
            );
            if use_color_cache != 0 {
                k = 0 as ::core::ffi::c_int;
                while k < prev_row_len {
                    VP8LColorCacheInsert(&raw mut hashers, *argb.offset((i + k) as isize));
                    k += 1;
                }
            }
            i += prev_row_len;
        } else {
            AddSingleLiteral(
                *argb.offset(i as isize),
                use_color_cache,
                &raw mut hashers,
                refs,
            );
            i += 1;
        }
    }
    if use_color_cache != 0 {
        VP8LColorCacheClear(&raw mut hashers);
    }
    return ((*refs).error_ == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn BackwardReferencesLz77(
    mut xsize: ::core::ffi::c_int,
    mut ysize: ::core::ffi::c_int,
    argb: *const uint32_t,
    mut cache_bits: ::core::ffi::c_int,
    hash_chain: *const VP8LHashChain,
    refs: *mut VP8LBackwardRefs,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    let mut i_last_check: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut cc_init: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let use_color_cache: ::core::ffi::c_int =
        (cache_bits > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let pix_count: ::core::ffi::c_int = xsize * ysize;
    let mut hashers: VP8LColorCache = VP8LColorCache {
        colors_: ::core::ptr::null_mut::<uint32_t>(),
        hash_shift_: 0,
        hash_bits_: 0,
    };
    if use_color_cache != 0 {
        cc_init = VP8LColorCacheInit(&raw mut hashers, cache_bits);
        if cc_init == 0 {
            current_block = 699343806474815527;
        } else {
            current_block = 15427931788582360902;
        }
    } else {
        current_block = 15427931788582360902;
    }
    match current_block {
        15427931788582360902 => {
            VP8LClearBackwardRefs(refs);
            i = 0 as ::core::ffi::c_int;
            while i < pix_count {
                let mut offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut j: ::core::ffi::c_int = 0;
                VP8LHashChainFindCopy(hash_chain, i, &raw mut offset, &raw mut len);
                if len >= MIN_LENGTH {
                    let len_ini: ::core::ffi::c_int = len;
                    let mut max_reach: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let j_max: ::core::ffi::c_int = if i + len_ini >= pix_count {
                        pix_count - 1 as ::core::ffi::c_int
                    } else {
                        i + len_ini
                    };
                    i_last_check = if i > i_last_check { i } else { i_last_check };
                    j = i_last_check + 1 as ::core::ffi::c_int;
                    while j <= j_max {
                        let len_j: ::core::ffi::c_int =
                            VP8LHashChainFindLength(hash_chain, j) as ::core::ffi::c_int;
                        let reach: ::core::ffi::c_int = j
                            + (if len_j >= MIN_LENGTH {
                                len_j
                            } else {
                                1 as ::core::ffi::c_int
                            });
                        if reach > max_reach {
                            len = j - i;
                            max_reach = reach;
                            if max_reach >= pix_count {
                                break;
                            }
                        }
                        j += 1;
                    }
                } else {
                    len = 1 as ::core::ffi::c_int;
                }
                '_c2rust_label: {
                    if len > 0 as ::core::ffi::c_int {
                    } else {
                        __assert_fail(
                            b"len > 0\0" as *const u8 as *const ::core::ffi::c_char,
                            b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            565 as ::core::ffi::c_uint,
                            b"int BackwardReferencesLz77(int, int, const uint32_t *const, int, const VP8LHashChain *const, VP8LBackwardRefs *const)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                };
                if len == 1 as ::core::ffi::c_int {
                    AddSingleLiteral(
                        *argb.offset(i as isize),
                        use_color_cache,
                        &raw mut hashers,
                        refs,
                    );
                } else {
                    VP8LBackwardRefsCursorAdd(
                        refs,
                        PixOrCopyCreateCopy(offset as uint32_t, len as uint16_t),
                    );
                    if use_color_cache != 0 {
                        j = i;
                        while j < i + len {
                            VP8LColorCacheInsert(&raw mut hashers, *argb.offset(j as isize));
                            j += 1;
                        }
                    }
                }
                i += len;
            }
            ok = ((*refs).error_ == 0) as ::core::ffi::c_int;
        }
        _ => {}
    }
    if cc_init != 0 {
        VP8LColorCacheClear(&raw mut hashers);
    }
    return ok;
}
pub const WINDOW_OFFSETS_SIZE_MAX: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
unsafe extern "C" fn BackwardReferencesLz77Box(
    mut xsize: ::core::ffi::c_int,
    mut ysize: ::core::ffi::c_int,
    argb: *const uint32_t,
    mut cache_bits: ::core::ffi::c_int,
    hash_chain_best: *const VP8LHashChain,
    mut hash_chain: *mut VP8LHashChain,
    refs: *mut VP8LBackwardRefs,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let pix_count: ::core::ffi::c_int = xsize * ysize;
    let mut counts: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
    let mut window_offsets: [::core::ffi::c_int; 32] = [0 as ::core::ffi::c_int; 32];
    let mut window_offsets_new: [::core::ffi::c_int; 32] = [0 as ::core::ffi::c_int; 32];
    let mut window_offsets_size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut window_offsets_new_size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let counts_ini: *mut uint16_t = WebPSafeMalloc(
        (xsize * ysize) as uint64_t,
        ::core::mem::size_of::<uint16_t>() as size_t,
    ) as *mut uint16_t;
    let mut best_offset_prev: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut best_length_prev: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if counts_ini.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    i = pix_count - 2 as ::core::ffi::c_int;
    counts = counts_ini.offset(i as isize);
    *counts.offset(1 as ::core::ffi::c_int as isize) = 1 as uint16_t;
    while i >= 0 as ::core::ffi::c_int {
        if *argb.offset(i as isize) == *argb.offset((i + 1 as ::core::ffi::c_int) as isize) {
            *counts.offset(0 as ::core::ffi::c_int as isize) =
                (*counts.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + (*counts.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        != MAX_LENGTH) as ::core::ffi::c_int) as uint16_t;
        } else {
            *counts.offset(0 as ::core::ffi::c_int as isize) = 1 as uint16_t;
        }
        i -= 1;
        counts = counts.offset(-1);
    }
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y <= 6 as ::core::ffi::c_int {
        x = -(6 as ::core::ffi::c_int);
        while x <= 6 as ::core::ffi::c_int {
            let offset: ::core::ffi::c_int = y * xsize + x;
            let mut plane_code: ::core::ffi::c_int = 0;
            if !(offset <= 0 as ::core::ffi::c_int) {
                plane_code = VP8LDistanceToPlaneCode(xsize, offset) - 1 as ::core::ffi::c_int;
                if !(plane_code >= WINDOW_OFFSETS_SIZE_MAX) {
                    window_offsets[plane_code as usize] = offset;
                }
            }
            x += 1;
        }
        y += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < WINDOW_OFFSETS_SIZE_MAX {
        if !(window_offsets[i as usize] == 0 as ::core::ffi::c_int) {
            let fresh11 = window_offsets_size;
            window_offsets_size = window_offsets_size + 1;
            window_offsets[fresh11 as usize] = window_offsets[i as usize];
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < window_offsets_size {
        let mut j: ::core::ffi::c_int = 0;
        let mut is_reachable: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        j = 0 as ::core::ffi::c_int;
        while j < window_offsets_size && is_reachable == 0 {
            is_reachable |= (window_offsets[i as usize]
                == window_offsets[j as usize] + 1 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            j += 1;
        }
        if is_reachable == 0 {
            window_offsets_new[window_offsets_new_size as usize] = window_offsets[i as usize];
            window_offsets_new_size += 1;
        }
        i += 1;
    }
    *(*hash_chain)
        .offset_length_
        .offset(0 as ::core::ffi::c_int as isize) = 0 as uint32_t;
    i = 1 as ::core::ffi::c_int;
    while i < pix_count {
        let mut ind: ::core::ffi::c_int = 0;
        let mut best_length: ::core::ffi::c_int = VP8LHashChainFindLength(hash_chain_best, i);
        let mut best_offset: ::core::ffi::c_int = 0;
        let mut do_compute: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        if best_length >= MAX_LENGTH {
            best_offset = VP8LHashChainFindOffset(hash_chain_best, i);
            ind = 0 as ::core::ffi::c_int;
            while ind < window_offsets_size {
                if best_offset == window_offsets[ind as usize] {
                    do_compute = 0 as ::core::ffi::c_int;
                    break;
                } else {
                    ind += 1;
                }
            }
        }
        if do_compute != 0 {
            let use_prev: ::core::ffi::c_int = (best_length_prev > 1 as ::core::ffi::c_int
                && best_length_prev < MAX_LENGTH)
                as ::core::ffi::c_int;
            let num_ind: ::core::ffi::c_int = if use_prev != 0 {
                window_offsets_new_size
            } else {
                window_offsets_size
            };
            best_length = if use_prev != 0 {
                best_length_prev - 1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            best_offset = if use_prev != 0 {
                best_offset_prev
            } else {
                0 as ::core::ffi::c_int
            };
            ind = 0 as ::core::ffi::c_int;
            while ind < num_ind {
                let mut curr_length: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut j_0: ::core::ffi::c_int = i;
                let mut j_offset: ::core::ffi::c_int = if use_prev != 0 {
                    i - window_offsets_new[ind as usize]
                } else {
                    i - window_offsets[ind as usize]
                };
                if !(j_offset < 0 as ::core::ffi::c_int
                    || *argb.offset(j_offset as isize) != *argb.offset(i as isize))
                {
                    loop {
                        let counts_j_offset: ::core::ffi::c_int =
                            *counts_ini.offset(j_offset as isize) as ::core::ffi::c_int;
                        let counts_j: ::core::ffi::c_int =
                            *counts_ini.offset(j_0 as isize) as ::core::ffi::c_int;
                        if counts_j_offset != counts_j {
                            curr_length += if counts_j_offset < counts_j {
                                counts_j_offset
                            } else {
                                counts_j
                            };
                            break;
                        } else {
                            curr_length += counts_j_offset;
                            j_offset += counts_j_offset;
                            j_0 += counts_j_offset;
                            if !(curr_length <= MAX_LENGTH
                                && j_0 < pix_count
                                && *argb.offset(j_offset as isize) == *argb.offset(j_0 as isize))
                            {
                                break;
                            }
                        }
                    }
                    if best_length < curr_length {
                        best_offset = if use_prev != 0 {
                            window_offsets_new[ind as usize]
                        } else {
                            window_offsets[ind as usize]
                        };
                        if curr_length >= MAX_LENGTH {
                            best_length = MAX_LENGTH;
                            break;
                        } else {
                            best_length = curr_length;
                        }
                    }
                }
                ind += 1;
            }
        }
        '_c2rust_label: {
            if i + best_length <= pix_count {
            } else {
                __assert_fail(
                    b"i + best_length <= pix_count\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    716 as ::core::ffi::c_uint,
                    b"int BackwardReferencesLz77Box(int, int, const uint32_t *const, int, const VP8LHashChain *const, VP8LHashChain *, VP8LBackwardRefs *const)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_0: {
            if best_length
                <= ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
            {
            } else {
                __assert_fail(
                    b"best_length <= MAX_LENGTH\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    717 as ::core::ffi::c_uint,
                    b"int BackwardReferencesLz77Box(int, int, const uint32_t *const, int, const VP8LHashChain *const, VP8LHashChain *, VP8LBackwardRefs *const)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        if best_length <= MIN_LENGTH {
            *(*hash_chain).offset_length_.offset(i as isize) = 0 as uint32_t;
            best_offset_prev = 0 as ::core::ffi::c_int;
            best_length_prev = 0 as ::core::ffi::c_int;
        } else {
            *(*hash_chain).offset_length_.offset(i as isize) =
                (best_offset << MAX_LENGTH_BITS) as uint32_t | best_length as uint32_t;
            best_offset_prev = best_offset;
            best_length_prev = best_length;
        }
        i += 1;
    }
    *(*hash_chain)
        .offset_length_
        .offset(0 as ::core::ffi::c_int as isize) = 0 as uint32_t;
    WebPSafeFree(counts_ini as *mut ::core::ffi::c_void);
    return BackwardReferencesLz77(xsize, ysize, argb, cache_bits, hash_chain, refs);
}
unsafe extern "C" fn BackwardReferences2DLocality(
    mut xsize: ::core::ffi::c_int,
    refs: *const VP8LBackwardRefs,
) {
    let mut c: VP8LRefsCursor = VP8LRefsCursorInit(refs);
    while VP8LRefsCursorOk(&raw mut c) != 0 {
        if PixOrCopyIsCopy(c.cur_pos) != 0 {
            let dist: ::core::ffi::c_int = (*c.cur_pos).argb_or_distance as ::core::ffi::c_int;
            let transformed_dist: ::core::ffi::c_int =
                VP8LDistanceToPlaneCode(xsize, dist) as ::core::ffi::c_int;
            (*c.cur_pos).argb_or_distance = transformed_dist as uint32_t;
        }
        VP8LRefsCursorNext(&raw mut c);
    }
}
unsafe extern "C" fn CalculateBestCacheSize(
    mut argb: *const uint32_t,
    mut quality: ::core::ffi::c_int,
    refs: *const VP8LBackwardRefs,
    best_cache_bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    let cache_bits_max: ::core::ffi::c_int = if quality <= 25 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        *best_cache_bits
    };
    let mut entropy_min: ::core::ffi::c_float = MAX_ENTROPY;
    let mut cc_init: [::core::ffi::c_int; 11] = [0 as ::core::ffi::c_int; 11];
    let mut hashers: [VP8LColorCache; 11] = [VP8LColorCache {
        colors_: ::core::ptr::null_mut::<uint32_t>(),
        hash_shift_: 0,
        hash_bits_: 0,
    }; 11];
    let mut c: VP8LRefsCursor = VP8LRefsCursorInit(refs);
    let mut histos: [*mut VP8LHistogram; 11] = [
        ::core::ptr::null_mut::<VP8LHistogram>(),
        ::core::ptr::null_mut::<VP8LHistogram>(),
        ::core::ptr::null_mut::<VP8LHistogram>(),
        ::core::ptr::null_mut::<VP8LHistogram>(),
        ::core::ptr::null_mut::<VP8LHistogram>(),
        ::core::ptr::null_mut::<VP8LHistogram>(),
        ::core::ptr::null_mut::<VP8LHistogram>(),
        ::core::ptr::null_mut::<VP8LHistogram>(),
        ::core::ptr::null_mut::<VP8LHistogram>(),
        ::core::ptr::null_mut::<VP8LHistogram>(),
        ::core::ptr::null_mut::<VP8LHistogram>(),
    ];
    let mut ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    '_c2rust_label: {
        if cache_bits_max >= 0 as ::core::ffi::c_int && cache_bits_max <= 10 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"cache_bits_max >= 0 && cache_bits_max <= MAX_COLOR_CACHE_BITS\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                768 as ::core::ffi::c_uint,
                b"int CalculateBestCacheSize(const uint32_t *, int, const VP8LBackwardRefs *const, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if cache_bits_max == 0 as ::core::ffi::c_int {
        *best_cache_bits = 0 as ::core::ffi::c_int;
        return 1 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    loop {
        if !(i <= cache_bits_max) {
            current_block = 10599921512955367680;
            break;
        }
        histos[i as usize] = VP8LAllocateHistogram(i);
        if histos[i as usize].is_null() {
            current_block = 16906004953079932670;
            break;
        }
        VP8LHistogramInit(histos[i as usize], i, 1 as ::core::ffi::c_int);
        if !(i == 0 as ::core::ffi::c_int) {
            cc_init[i as usize] = VP8LColorCacheInit(
                (&raw mut hashers as *mut VP8LColorCache).offset(i as isize) as *mut VP8LColorCache,
                i,
            );
            if cc_init[i as usize] == 0 {
                current_block = 16906004953079932670;
                break;
            }
        }
        i += 1;
    }
    match current_block {
        10599921512955367680 => {
            while VP8LRefsCursorOk(&raw mut c) != 0 {
                let v: *const PixOrCopy = c.cur_pos;
                if PixOrCopyIsLiteral(v) != 0 {
                    let fresh6 = argb;
                    argb = argb.offset(1);
                    let pix: uint32_t = *fresh6;
                    let a: uint32_t = pix >> 24 as ::core::ffi::c_int & 0xff as uint32_t;
                    let r: uint32_t = pix >> 16 as ::core::ffi::c_int & 0xff as uint32_t;
                    let g: uint32_t = pix >> 8 as ::core::ffi::c_int & 0xff as uint32_t;
                    let b: uint32_t = pix >> 0 as ::core::ffi::c_int & 0xff as uint32_t;
                    let mut key: ::core::ffi::c_int =
                        VP8LHashPix(pix, 32 as ::core::ffi::c_int - cache_bits_max);
                    (*histos[0 as ::core::ffi::c_int as usize]).blue_[b as usize] =
                        (*histos[0 as ::core::ffi::c_int as usize]).blue_[b as usize]
                            .wrapping_add(1);
                    let ref mut fresh7 = *(*histos[0 as ::core::ffi::c_int as usize])
                        .literal_
                        .offset(g as isize);
                    *fresh7 = (*fresh7).wrapping_add(1);
                    (*histos[0 as ::core::ffi::c_int as usize]).red_[r as usize] =
                        (*histos[0 as ::core::ffi::c_int as usize]).red_[r as usize]
                            .wrapping_add(1);
                    (*histos[0 as ::core::ffi::c_int as usize]).alpha_[a as usize] =
                        (*histos[0 as ::core::ffi::c_int as usize]).alpha_[a as usize]
                            .wrapping_add(1);
                    i = cache_bits_max;
                    while i >= 1 as ::core::ffi::c_int {
                        if VP8LColorCacheLookup(
                            (&raw mut hashers as *mut VP8LColorCache).offset(i as isize)
                                as *mut VP8LColorCache,
                            key as uint32_t,
                        ) == pix
                        {
                            let ref mut fresh8 = *(*histos[i as usize])
                                .literal_
                                .offset((NUM_LITERAL_CODES + NUM_LENGTH_CODES + key) as isize);
                            *fresh8 = (*fresh8).wrapping_add(1);
                        } else {
                            VP8LColorCacheSet(
                                (&raw mut hashers as *mut VP8LColorCache).offset(i as isize)
                                    as *mut VP8LColorCache,
                                key as uint32_t,
                                pix,
                            );
                            (*histos[i as usize]).blue_[b as usize] =
                                (*histos[i as usize]).blue_[b as usize].wrapping_add(1);
                            let ref mut fresh9 = *(*histos[i as usize]).literal_.offset(g as isize);
                            *fresh9 = (*fresh9).wrapping_add(1);
                            (*histos[i as usize]).red_[r as usize] =
                                (*histos[i as usize]).red_[r as usize].wrapping_add(1);
                            (*histos[i as usize]).alpha_[a as usize] =
                                (*histos[i as usize]).alpha_[a as usize].wrapping_add(1);
                        }
                        i -= 1;
                        key >>= 1 as ::core::ffi::c_int;
                    }
                } else {
                    let mut code: ::core::ffi::c_int = 0;
                    let mut extra_bits: ::core::ffi::c_int = 0;
                    let mut extra_bits_value: ::core::ffi::c_int = 0;
                    let mut len: ::core::ffi::c_int = PixOrCopyLength(v) as ::core::ffi::c_int;
                    let mut argb_prev: uint32_t = *argb ^ 0xffffffff as uint32_t;
                    VP8LPrefixEncode(
                        len,
                        &raw mut code,
                        &raw mut extra_bits,
                        &raw mut extra_bits_value,
                    );
                    i = 0 as ::core::ffi::c_int;
                    while i <= cache_bits_max {
                        let ref mut fresh10 = *(*histos[i as usize])
                            .literal_
                            .offset((NUM_LITERAL_CODES + code) as isize);
                        *fresh10 = (*fresh10).wrapping_add(1);
                        i += 1;
                    }
                    loop {
                        if *argb != argb_prev {
                            let mut key_0: ::core::ffi::c_int =
                                VP8LHashPix(*argb, 32 as ::core::ffi::c_int - cache_bits_max);
                            i = cache_bits_max;
                            while i >= 1 as ::core::ffi::c_int {
                                *hashers[i as usize].colors_.offset(key_0 as isize) = *argb;
                                i -= 1;
                                key_0 >>= 1 as ::core::ffi::c_int;
                            }
                            argb_prev = *argb;
                        }
                        argb = argb.offset(1);
                        len -= 1;
                        if !(len != 0 as ::core::ffi::c_int) {
                            break;
                        }
                    }
                }
                VP8LRefsCursorNext(&raw mut c);
            }
            i = 0 as ::core::ffi::c_int;
            while i <= cache_bits_max {
                let entropy: ::core::ffi::c_float =
                    VP8LHistogramEstimateBits(histos[i as usize]) as ::core::ffi::c_float;
                if i == 0 as ::core::ffi::c_int || entropy < entropy_min {
                    entropy_min = entropy;
                    *best_cache_bits = i;
                }
                i += 1;
            }
            ok = 1 as ::core::ffi::c_int;
        }
        _ => {}
    }
    i = 0 as ::core::ffi::c_int;
    while i <= cache_bits_max {
        if cc_init[i as usize] != 0 {
            VP8LColorCacheClear(
                (&raw mut hashers as *mut VP8LColorCache).offset(i as isize) as *mut VP8LColorCache
            );
        }
        VP8LFreeHistogram(histos[i as usize]);
        i += 1;
    }
    return ok;
}
unsafe extern "C" fn BackwardRefsWithLocalCache(
    argb: *const uint32_t,
    mut cache_bits: ::core::ffi::c_int,
    refs: *mut VP8LBackwardRefs,
) -> ::core::ffi::c_int {
    let mut pixel_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut hashers: VP8LColorCache = VP8LColorCache {
        colors_: ::core::ptr::null_mut::<uint32_t>(),
        hash_shift_: 0,
        hash_bits_: 0,
    };
    let mut c: VP8LRefsCursor = VP8LRefsCursorInit(refs);
    if VP8LColorCacheInit(&raw mut hashers, cache_bits) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    while VP8LRefsCursorOk(&raw mut c) != 0 {
        let v: *mut PixOrCopy = c.cur_pos;
        if PixOrCopyIsLiteral(v) != 0 {
            let argb_literal: uint32_t = (*v).argb_or_distance;
            let ix: ::core::ffi::c_int =
                VP8LColorCacheContains(&raw mut hashers, argb_literal) as ::core::ffi::c_int;
            if ix >= 0 as ::core::ffi::c_int {
                *v = PixOrCopyCreateCacheIdx(ix);
            } else {
                VP8LColorCacheInsert(&raw mut hashers, argb_literal);
            }
            pixel_index += 1;
        } else {
            let mut k: ::core::ffi::c_int = 0;
            '_c2rust_label: {
                if PixOrCopyIsCopy(v) != 0 {
                } else {
                    __assert_fail(
                        b"PixOrCopyIsCopy(v)\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        885 as ::core::ffi::c_uint,
                        b"int BackwardRefsWithLocalCache(const uint32_t *const, int, VP8LBackwardRefs *const)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            k = 0 as ::core::ffi::c_int;
            while k < (*v).len as ::core::ffi::c_int {
                let fresh5 = pixel_index;
                pixel_index = pixel_index + 1;
                VP8LColorCacheInsert(&raw mut hashers, *argb.offset(fresh5 as isize));
                k += 1;
            }
        }
        VP8LRefsCursorNext(&raw mut c);
    }
    VP8LColorCacheClear(&raw mut hashers);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn GetBackwardReferencesLowEffort(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    argb: *const uint32_t,
    cache_bits: *mut ::core::ffi::c_int,
    hash_chain: *const VP8LHashChain,
    refs_lz77: *mut VP8LBackwardRefs,
) -> *mut VP8LBackwardRefs {
    *cache_bits = 0 as ::core::ffi::c_int;
    if BackwardReferencesLz77(
        width,
        height,
        argb,
        0 as ::core::ffi::c_int,
        hash_chain,
        refs_lz77,
    ) == 0
    {
        return ::core::ptr::null_mut::<VP8LBackwardRefs>();
    }
    BackwardReferences2DLocality(width, refs_lz77);
    return refs_lz77;
}
unsafe extern "C" fn GetBackwardReferences(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    argb: *const uint32_t,
    mut quality: ::core::ffi::c_int,
    mut lz77_types_to_try: ::core::ffi::c_int,
    mut cache_bits_max: ::core::ffi::c_int,
    mut do_no_cache: ::core::ffi::c_int,
    hash_chain: *const VP8LHashChain,
    refs: *mut VP8LBackwardRefs,
    cache_bits_best: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut histo: *mut VP8LHistogram = ::core::ptr::null_mut::<VP8LHistogram>();
    let mut i: ::core::ffi::c_int = 0;
    let mut lz77_type: ::core::ffi::c_int = 0;
    let mut lz77_types_best: [::core::ffi::c_int; 2] =
        [0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int];
    let mut bit_costs_best: [::core::ffi::c_float; 2] = [FLT_MAX, FLT_MAX];
    let mut hash_chain_box: VP8LHashChain = VP8LHashChain {
        offset_length_: ::core::ptr::null_mut::<uint32_t>(),
        size_: 0,
    };
    let refs_tmp: *mut VP8LBackwardRefs = refs.offset(
        (if do_no_cache != 0 {
            2 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as isize,
    ) as *mut VP8LBackwardRefs;
    let mut status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    memset(
        &raw mut hash_chain_box as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VP8LHashChain>() as size_t,
    );
    histo = VP8LAllocateHistogram(MAX_COLOR_CACHE_BITS);
    if !histo.is_null() {
        lz77_type = 1 as ::core::ffi::c_int;
        's_27: loop {
            if !(lz77_types_to_try != 0) {
                current_block = 9007357115414505193;
                break;
            }
            let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut bit_cost: ::core::ffi::c_float = 0.0f32;
            if !(lz77_types_to_try & lz77_type == 0 as ::core::ffi::c_int) {
                match lz77_type {
                    2 => {
                        res = BackwardReferencesRle(
                            width,
                            height,
                            argb,
                            0 as ::core::ffi::c_int,
                            refs_tmp,
                        );
                    }
                    1 => {
                        res = BackwardReferencesLz77(
                            width,
                            height,
                            argb,
                            0 as ::core::ffi::c_int,
                            hash_chain,
                            refs_tmp,
                        );
                    }
                    4 => {
                        if VP8LHashChainInit(&raw mut hash_chain_box, width * height) == 0 {
                            current_block = 12585312436508836224;
                            break;
                        }
                        res = BackwardReferencesLz77Box(
                            width,
                            height,
                            argb,
                            0 as ::core::ffi::c_int,
                            hash_chain,
                            &raw mut hash_chain_box,
                            refs_tmp,
                        );
                    }
                    _ => {
                        '_c2rust_label: {
                            __assert_fail(
                                b"0\0" as *const u8 as *const ::core::ffi::c_char,
                                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                953 as ::core::ffi::c_uint,
                                b"int GetBackwardReferences(int, int, const uint32_t *const, int, int, int, int, const VP8LHashChain *const, VP8LBackwardRefs *const, int *const)\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                            );
                        };
                    }
                }
                if res == 0 {
                    current_block = 12585312436508836224;
                    break;
                }
                i = 1 as ::core::ffi::c_int;
                while i >= 0 as ::core::ffi::c_int {
                    let mut cache_bits: ::core::ffi::c_int = if i == 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int
                    } else {
                        cache_bits_max
                    };
                    if !(i == 1 as ::core::ffi::c_int && do_no_cache == 0) {
                        if i == 0 as ::core::ffi::c_int {
                            if CalculateBestCacheSize(argb, quality, refs_tmp, &raw mut cache_bits)
                                == 0
                            {
                                current_block = 12585312436508836224;
                                break 's_27;
                            }
                            if cache_bits > 0 as ::core::ffi::c_int {
                                if BackwardRefsWithLocalCache(argb, cache_bits, refs_tmp) == 0 {
                                    current_block = 12585312436508836224;
                                    break 's_27;
                                }
                            }
                        }
                        if !(i == 0 as ::core::ffi::c_int
                            && do_no_cache != 0
                            && cache_bits == 0 as ::core::ffi::c_int)
                        {
                            VP8LHistogramCreate(histo, refs_tmp, cache_bits);
                            bit_cost = VP8LHistogramEstimateBits(histo);
                        }
                        if bit_cost < bit_costs_best[i as usize] {
                            if i == 1 as ::core::ffi::c_int {
                                if BackwardRefsClone(
                                    refs_tmp,
                                    refs.offset(1 as ::core::ffi::c_int as isize)
                                        as *mut VP8LBackwardRefs,
                                ) == 0
                                {
                                    current_block = 12585312436508836224;
                                    break 's_27;
                                }
                            } else {
                                BackwardRefsSwap(
                                    refs_tmp,
                                    refs.offset(0 as ::core::ffi::c_int as isize)
                                        as *mut VP8LBackwardRefs,
                                );
                            }
                            bit_costs_best[i as usize] = bit_cost;
                            lz77_types_best[i as usize] = lz77_type;
                            if i == 0 as ::core::ffi::c_int {
                                *cache_bits_best = cache_bits;
                            }
                        }
                    }
                    i -= 1;
                }
            }
            lz77_types_to_try &= !lz77_type;
            lz77_type <<= 1 as ::core::ffi::c_int;
        }
        match current_block {
            12585312436508836224 => {}
            _ => {
                '_c2rust_label_0: {
                    if lz77_types_best[0 as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
                    } else {
                        __assert_fail(
                            b"lz77_types_best[0] > 0\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            996 as ::core::ffi::c_uint,
                            b"int GetBackwardReferences(int, int, const uint32_t *const, int, int, int, int, const VP8LHashChain *const, VP8LBackwardRefs *const, int *const)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                };
                '_c2rust_label_1: {
                    if do_no_cache == 0
                        || lz77_types_best[1 as ::core::ffi::c_int as usize]
                            > 0 as ::core::ffi::c_int
                    {
                    } else {
                        __assert_fail(
                            b"!do_no_cache || lz77_types_best[1] > 0\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/backward_references_enc.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            997 as ::core::ffi::c_uint,
                            b"int GetBackwardReferences(int, int, const uint32_t *const, int, int, int, int, const VP8LHashChain *const, VP8LBackwardRefs *const, int *const)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                };
                i = 1 as ::core::ffi::c_int;
                loop {
                    if !(i >= 0 as ::core::ffi::c_int) {
                        current_block = 9353995356876505083;
                        break;
                    }
                    if !(i == 1 as ::core::ffi::c_int && do_no_cache == 0) {
                        if (lz77_types_best[i as usize] == kLZ77Standard as ::core::ffi::c_int
                            || lz77_types_best[i as usize] == kLZ77Box as ::core::ffi::c_int)
                            && quality >= 25 as ::core::ffi::c_int
                        {
                            let hash_chain_tmp: *const VP8LHashChain = if lz77_types_best
                                [i as usize]
                                == kLZ77Standard as ::core::ffi::c_int
                            {
                                hash_chain
                            } else {
                                &raw mut hash_chain_box as *const VP8LHashChain
                            };
                            let cache_bits_0: ::core::ffi::c_int = if i == 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int
                            } else {
                                *cache_bits_best
                            };
                            let mut bit_cost_trace: ::core::ffi::c_float = 0.;
                            if VP8LBackwardReferencesTraceBackwards(
                                width,
                                height,
                                argb,
                                cache_bits_0,
                                hash_chain_tmp,
                                refs.offset(i as isize) as *mut VP8LBackwardRefs,
                                refs_tmp,
                            ) == 0
                            {
                                current_block = 12585312436508836224;
                                break;
                            }
                            VP8LHistogramCreate(histo, refs_tmp, cache_bits_0);
                            bit_cost_trace = VP8LHistogramEstimateBits(histo);
                            if bit_cost_trace < bit_costs_best[i as usize] {
                                BackwardRefsSwap(
                                    refs_tmp,
                                    refs.offset(i as isize) as *mut VP8LBackwardRefs,
                                );
                            }
                        }
                        BackwardReferences2DLocality(
                            width,
                            refs.offset(i as isize) as *mut VP8LBackwardRefs,
                        );
                        if i == 1 as ::core::ffi::c_int
                            && lz77_types_best[0 as ::core::ffi::c_int as usize]
                                == lz77_types_best[1 as ::core::ffi::c_int as usize]
                            && *cache_bits_best == 0 as ::core::ffi::c_int
                        {
                            if BackwardRefsClone(
                                refs.offset(1 as ::core::ffi::c_int as isize)
                                    as *mut VP8LBackwardRefs,
                                refs.offset(0 as ::core::ffi::c_int as isize)
                                    as *mut VP8LBackwardRefs,
                            ) == 0
                            {
                                current_block = 12585312436508836224;
                                break;
                            } else {
                                current_block = 9353995356876505083;
                                break;
                            }
                        }
                    }
                    i -= 1;
                }
                match current_block {
                    12585312436508836224 => {}
                    _ => {
                        status = 1 as ::core::ffi::c_int;
                    }
                }
            }
        }
    }
    VP8LHashChainClear(&raw mut hash_chain_box);
    VP8LFreeHistogram(histo);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LGetBackwardReferences(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    argb: *const uint32_t,
    mut quality: ::core::ffi::c_int,
    mut low_effort: ::core::ffi::c_int,
    mut lz77_types_to_try: ::core::ffi::c_int,
    mut cache_bits_max: ::core::ffi::c_int,
    mut do_no_cache: ::core::ffi::c_int,
    hash_chain: *const VP8LHashChain,
    refs: *mut VP8LBackwardRefs,
    cache_bits_best: *mut ::core::ffi::c_int,
    pic: *const WebPPicture,
    mut percent_range: ::core::ffi::c_int,
    percent: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if low_effort != 0 {
        let mut refs_best: *mut VP8LBackwardRefs = ::core::ptr::null_mut::<VP8LBackwardRefs>();
        *cache_bits_best = cache_bits_max;
        refs_best =
            GetBackwardReferencesLowEffort(width, height, argb, cache_bits_best, hash_chain, refs);
        if refs_best.is_null() {
            return WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
        }
        BackwardRefsSwap(
            refs_best,
            refs.offset(0 as ::core::ffi::c_int as isize) as *mut VP8LBackwardRefs,
        );
    } else if GetBackwardReferences(
        width,
        height,
        argb,
        quality,
        lz77_types_to_try,
        cache_bits_max,
        do_no_cache,
        hash_chain,
        refs,
        cache_bits_best,
    ) == 0
    {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    return WebPReportProgress(pic, *percent + percent_range, percent);
}
pub const __FLT_MAX__: ::core::ffi::c_float = 3.40282347e+38f32;
