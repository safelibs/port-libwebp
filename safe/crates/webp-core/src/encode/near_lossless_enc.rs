extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn VP8LNearLosslessBits(
    mut near_lossless_quality: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return 5 as ::core::ffi::c_int - near_lossless_quality / 20 as ::core::ffi::c_int;
}
pub const MIN_DIM_FOR_NEAR_LOSSLESS: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
unsafe extern "C" fn FindClosestDiscretized(
    mut a: uint32_t,
    mut bits: ::core::ffi::c_int,
) -> uint32_t {
    let mask: uint32_t = ((1 as uint32_t) << bits).wrapping_sub(1 as uint32_t);
    let biased: uint32_t = a
        .wrapping_add(mask >> 1 as ::core::ffi::c_int)
        .wrapping_add(a >> bits & 1 as uint32_t);
    '_c2rust_label: {
        if bits > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"bits > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/near_lossless_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                34 as ::core::ffi::c_uint,
                b"uint32_t FindClosestDiscretized(uint32_t, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if biased > 0xff as uint32_t {
        return 0xff as uint32_t;
    }
    return biased & !mask;
}
unsafe extern "C" fn ClosestDiscretizedArgb(
    mut a: uint32_t,
    mut bits: ::core::ffi::c_int,
) -> uint32_t {
    return FindClosestDiscretized(a >> 24 as ::core::ffi::c_int, bits) << 24 as ::core::ffi::c_int
        | FindClosestDiscretized(a >> 16 as ::core::ffi::c_int & 0xff as uint32_t, bits)
            << 16 as ::core::ffi::c_int
        | FindClosestDiscretized(a >> 8 as ::core::ffi::c_int & 0xff as uint32_t, bits)
            << 8 as ::core::ffi::c_int
        | FindClosestDiscretized(a & 0xff as uint32_t, bits);
}
unsafe extern "C" fn IsNear(
    mut a: uint32_t,
    mut b: uint32_t,
    mut limit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = 0;
    k = 0 as ::core::ffi::c_int;
    while k < 4 as ::core::ffi::c_int {
        let delta: ::core::ffi::c_int = (a >> k * 8 as ::core::ffi::c_int & 0xff as uint32_t)
            as ::core::ffi::c_int
            - (b >> k * 8 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int;
        if delta >= limit || delta <= -limit {
            return 0 as ::core::ffi::c_int;
        }
        k += 1;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn IsSmooth(
    prev_row: *const uint32_t,
    curr_row: *const uint32_t,
    next_row: *const uint32_t,
    mut ix: ::core::ffi::c_int,
    mut limit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (IsNear(
        *curr_row.offset(ix as isize),
        *curr_row.offset((ix - 1 as ::core::ffi::c_int) as isize),
        limit,
    ) != 0
        && IsNear(
            *curr_row.offset(ix as isize),
            *curr_row.offset((ix + 1 as ::core::ffi::c_int) as isize),
            limit,
        ) != 0
        && IsNear(
            *curr_row.offset(ix as isize),
            *prev_row.offset(ix as isize),
            limit,
        ) != 0
        && IsNear(
            *curr_row.offset(ix as isize),
            *next_row.offset(ix as isize),
            limit,
        ) != 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn NearLossless(
    mut xsize: ::core::ffi::c_int,
    mut ysize: ::core::ffi::c_int,
    mut argb_src: *const uint32_t,
    mut stride: ::core::ffi::c_int,
    mut limit_bits: ::core::ffi::c_int,
    mut copy_buffer: *mut uint32_t,
    mut argb_dst: *mut uint32_t,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let limit: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << limit_bits;
    let mut prev_row: *mut uint32_t = copy_buffer;
    let mut curr_row: *mut uint32_t = prev_row.offset(xsize as isize);
    let mut next_row: *mut uint32_t = curr_row.offset(xsize as isize);
    memcpy(
        curr_row as *mut ::core::ffi::c_void,
        argb_src as *const ::core::ffi::c_void,
        (xsize as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
    );
    memcpy(
        next_row as *mut ::core::ffi::c_void,
        argb_src.offset(stride as isize) as *const ::core::ffi::c_void,
        (xsize as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
    );
    y = 0 as ::core::ffi::c_int;
    while y < ysize {
        if y == 0 as ::core::ffi::c_int || y == ysize - 1 as ::core::ffi::c_int {
            memcpy(
                argb_dst as *mut ::core::ffi::c_void,
                argb_src as *const ::core::ffi::c_void,
                (xsize as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            );
        } else {
            memcpy(
                next_row as *mut ::core::ffi::c_void,
                argb_src.offset(stride as isize) as *const ::core::ffi::c_void,
                (xsize as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            );
            *argb_dst.offset(0 as ::core::ffi::c_int as isize) =
                *argb_src.offset(0 as ::core::ffi::c_int as isize);
            *argb_dst.offset((xsize - 1 as ::core::ffi::c_int) as isize) =
                *argb_src.offset((xsize - 1 as ::core::ffi::c_int) as isize);
            x = 1 as ::core::ffi::c_int;
            while x < xsize - 1 as ::core::ffi::c_int {
                if IsSmooth(prev_row, curr_row, next_row, x, limit) != 0 {
                    *argb_dst.offset(x as isize) = *curr_row.offset(x as isize);
                } else {
                    *argb_dst.offset(x as isize) =
                        ClosestDiscretizedArgb(*curr_row.offset(x as isize), limit_bits);
                }
                x += 1;
            }
        }
        let temp: *mut uint32_t = prev_row;
        prev_row = curr_row;
        curr_row = next_row;
        next_row = temp;
        y += 1;
        argb_src = argb_src.offset(stride as isize);
        argb_dst = argb_dst.offset(xsize as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8ApplyNearLossless(
    picture: *const WebPPicture,
    mut quality: ::core::ffi::c_int,
    argb_dst: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let xsize: ::core::ffi::c_int = (*picture).width;
    let ysize: ::core::ffi::c_int = (*picture).height;
    let stride: ::core::ffi::c_int = (*picture).argb_stride;
    let copy_buffer: *mut uint32_t = WebPSafeMalloc(
        (xsize * 3 as ::core::ffi::c_int) as uint64_t,
        ::core::mem::size_of::<uint32_t>() as size_t,
    ) as *mut uint32_t;
    let limit_bits: ::core::ffi::c_int = VP8LNearLosslessBits(quality) as ::core::ffi::c_int;
    '_c2rust_label: {
        if !argb_dst.is_null() {
        } else {
            __assert_fail(
                b"argb_dst != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/near_lossless_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                119 as ::core::ffi::c_uint,
                b"int VP8ApplyNearLossless(const WebPPicture *const, int, uint32_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if limit_bits > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"limit_bits > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/near_lossless_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                120 as ::core::ffi::c_uint,
                b"int VP8ApplyNearLossless(const WebPPicture *const, int, uint32_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if limit_bits <= 5 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"limit_bits <= MAX_LIMIT_BITS\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/near_lossless_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                121 as ::core::ffi::c_uint,
                b"int VP8ApplyNearLossless(const WebPPicture *const, int, uint32_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if copy_buffer.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if xsize < MIN_DIM_FOR_NEAR_LOSSLESS && ysize < MIN_DIM_FOR_NEAR_LOSSLESS
        || ysize < 3 as ::core::ffi::c_int
    {
        i = 0 as ::core::ffi::c_int;
        while i < ysize {
            memcpy(
                argb_dst.offset((i * xsize) as isize) as *mut ::core::ffi::c_void,
                (*picture)
                    .argb
                    .offset((i * (*picture).argb_stride) as isize)
                    as *const ::core::ffi::c_void,
                (xsize as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            );
            i += 1;
        }
        WebPSafeFree(copy_buffer as *mut ::core::ffi::c_void);
        return 1 as ::core::ffi::c_int;
    }
    NearLossless(
        xsize,
        ysize,
        (*picture).argb,
        stride,
        limit_bits,
        copy_buffer,
        argb_dst,
    );
    i = limit_bits - 1 as ::core::ffi::c_int;
    while i != 0 as ::core::ffi::c_int {
        NearLossless(xsize, ysize, argb_dst, xsize, i, copy_buffer, argb_dst);
        i -= 1;
    }
    WebPSafeFree(copy_buffer as *mut ::core::ffi::c_void);
    return 1 as ::core::ffi::c_int;
}
