extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn WebPSafeCalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LColorCache {
    pub colors_: *mut uint32_t,
    pub hash_shift_: ::core::ffi::c_int,
    pub hash_bits_: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn VP8LColorCacheInit(
    color_cache: *mut VP8LColorCache,
    mut hash_bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let hash_size: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << hash_bits;
    '_c2rust_label: {
        if !color_cache.is_null() {
        } else {
            __assert_fail(
                b"color_cache != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/color_cache_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                25 as ::core::ffi::c_uint,
                b"int VP8LColorCacheInit(VP8LColorCache *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if hash_bits > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"hash_bits > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/color_cache_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                26 as ::core::ffi::c_uint,
                b"int VP8LColorCacheInit(VP8LColorCache *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*color_cache).colors_ = WebPSafeCalloc(
        hash_size as uint64_t,
        ::core::mem::size_of::<uint32_t>() as size_t,
    ) as *mut uint32_t;
    if (*color_cache).colors_.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*color_cache).hash_shift_ = 32 as ::core::ffi::c_int - hash_bits;
    (*color_cache).hash_bits_ = hash_bits;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LColorCacheClear(color_cache: *mut VP8LColorCache) {
    if !color_cache.is_null() {
        WebPSafeFree((*color_cache).colors_ as *mut ::core::ffi::c_void);
        (*color_cache).colors_ = ::core::ptr::null_mut::<uint32_t>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LColorCacheCopy(src: *const VP8LColorCache, dst: *mut VP8LColorCache) {
    '_c2rust_label: {
        if !src.is_null() {
        } else {
            __assert_fail(
                b"src != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/color_cache_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                44 as ::core::ffi::c_uint,
                b"void VP8LColorCacheCopy(const VP8LColorCache *const, VP8LColorCache *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !dst.is_null() {
        } else {
            __assert_fail(
                b"dst != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/color_cache_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                45 as ::core::ffi::c_uint,
                b"void VP8LColorCacheCopy(const VP8LColorCache *const, VP8LColorCache *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*src).hash_bits_ == (*dst).hash_bits_ {
        } else {
            __assert_fail(
                b"src->hash_bits_ == dst->hash_bits_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/color_cache_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                46 as ::core::ffi::c_uint,
                b"void VP8LColorCacheCopy(const VP8LColorCache *const, VP8LColorCache *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    memcpy(
        (*dst).colors_ as *mut ::core::ffi::c_void,
        (*src).colors_ as *const ::core::ffi::c_void,
        ((1 as ::core::ffi::c_uint as size_t) << (*dst).hash_bits_)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
    );
}
