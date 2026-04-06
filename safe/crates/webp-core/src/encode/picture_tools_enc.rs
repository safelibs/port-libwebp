extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut WebPAlphaReplace:
        Option<unsafe extern "C" fn(*mut uint32_t, ::core::ffi::c_int, uint32_t) -> ()>;
    fn WebPInitAlphaProcessing();
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
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
pub const YUV_HALF: C2RustUnnamed = 32768;
pub const YUV_FIX: C2RustUnnamed = 16;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const YUV_MASK2: C2RustUnnamed = 16383;
pub const YUV_FIX2: C2RustUnnamed = 6;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn VP8ClipUV(
    mut uv: ::core::ffi::c_int,
    mut rounding: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    uv = uv
        + rounding
        + ((128 as ::core::ffi::c_int) << YUV_FIX as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
        >> YUV_FIX as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
    return if uv & !(0xff as ::core::ffi::c_int) == 0 as ::core::ffi::c_int {
        uv
    } else if uv < 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        255 as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn VP8RGBToY(
    mut r: ::core::ffi::c_int,
    mut g: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
    mut rounding: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let luma: ::core::ffi::c_int = 16839 as ::core::ffi::c_int * r
        + 33059 as ::core::ffi::c_int * g
        + 6420 as ::core::ffi::c_int * b;
    return luma + rounding + ((16 as ::core::ffi::c_int) << YUV_FIX as ::core::ffi::c_int)
        >> YUV_FIX as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn VP8RGBToU(
    mut r: ::core::ffi::c_int,
    mut g: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
    mut rounding: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let u: ::core::ffi::c_int = -(9719 as ::core::ffi::c_int) * r - 19081 as ::core::ffi::c_int * g
        + 28800 as ::core::ffi::c_int * b;
    return VP8ClipUV(u, rounding);
}
#[inline]
unsafe extern "C" fn VP8RGBToV(
    mut r: ::core::ffi::c_int,
    mut g: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
    mut rounding: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let v: ::core::ffi::c_int = 28800 as ::core::ffi::c_int * r
        - 24116 as ::core::ffi::c_int * g
        - 4684 as ::core::ffi::c_int * b;
    return VP8ClipUV(v, rounding);
}
pub const SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SIZE2: ::core::ffi::c_int = SIZE / 2 as ::core::ffi::c_int;
unsafe extern "C" fn IsTransparentARGBArea(
    mut ptr: *const uint32_t,
    mut stride: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut y: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < size {
        x = 0 as ::core::ffi::c_int;
        while x < size {
            if *ptr.offset(x as isize) & 0xff000000 as uint32_t != 0 {
                return 0 as ::core::ffi::c_int;
            }
            x += 1;
        }
        ptr = ptr.offset(stride as isize);
        y += 1;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn Flatten(
    mut ptr: *mut uint8_t,
    mut v: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < size {
        memset(ptr as *mut ::core::ffi::c_void, v, size as size_t);
        ptr = ptr.offset(stride as isize);
        y += 1;
    }
}
unsafe extern "C" fn FlattenARGB(
    mut ptr: *mut uint32_t,
    mut v: uint32_t,
    mut stride: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < size {
        x = 0 as ::core::ffi::c_int;
        while x < size {
            *ptr.offset(x as isize) = v;
            x += 1;
        }
        ptr = ptr.offset(stride as isize);
        y += 1;
    }
}
unsafe extern "C" fn SmoothenBlock(
    mut a_ptr: *const uint8_t,
    mut a_stride: ::core::ffi::c_int,
    mut y_ptr: *mut uint8_t,
    mut y_stride: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut alpha_ptr: *const uint8_t = a_ptr;
    let mut luma_ptr: *mut uint8_t = y_ptr;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        x = 0 as ::core::ffi::c_int;
        while x < width {
            if *alpha_ptr.offset(x as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                count += 1;
                sum += *luma_ptr.offset(x as isize) as ::core::ffi::c_int;
            }
            x += 1;
        }
        alpha_ptr = alpha_ptr.offset(a_stride as isize);
        luma_ptr = luma_ptr.offset(y_stride as isize);
        y += 1;
    }
    if count > 0 as ::core::ffi::c_int && count < width * height {
        let avg_u8: uint8_t = (sum / count) as uint8_t;
        alpha_ptr = a_ptr;
        luma_ptr = y_ptr;
        y = 0 as ::core::ffi::c_int;
        while y < height {
            x = 0 as ::core::ffi::c_int;
            while x < width {
                if *alpha_ptr.offset(x as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    *luma_ptr.offset(x as isize) = avg_u8;
                }
                x += 1;
            }
            alpha_ptr = alpha_ptr.offset(a_stride as isize);
            luma_ptr = luma_ptr.offset(y_stride as isize);
            y += 1;
        }
    }
    return (count == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPReplaceTransparentPixels(pic: *mut WebPPicture, mut color: uint32_t) {
    if !pic.is_null() && (*pic).use_argb != 0 {
        let mut y: ::core::ffi::c_int = (*pic).height;
        let mut argb: *mut uint32_t = (*pic).argb;
        color = (color as ::core::ffi::c_uint & 0xffffff as ::core::ffi::c_uint) as uint32_t;
        WebPInitAlphaProcessing();
        loop {
            let fresh0 = y;
            y = y - 1;
            if !(fresh0 > 0 as ::core::ffi::c_int) {
                break;
            }
            WebPAlphaReplace.expect("non-null function pointer")(argb, (*pic).width, color);
            argb = argb.offset((*pic).argb_stride as isize);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPCleanupTransparentArea(mut pic: *mut WebPPicture) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut w: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    if pic.is_null() {
        return;
    }
    w = (*pic).width / SIZE;
    h = (*pic).height / SIZE;
    if (*pic).use_argb != 0 {
        let mut argb_value: uint32_t = 0 as uint32_t;
        y = 0 as ::core::ffi::c_int;
        while y < h {
            let mut need_reset: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            x = 0 as ::core::ffi::c_int;
            while x < w {
                let off: ::core::ffi::c_int = (y * (*pic).argb_stride + x) * SIZE;
                if IsTransparentARGBArea((*pic).argb.offset(off as isize), (*pic).argb_stride, SIZE)
                    != 0
                {
                    if need_reset != 0 {
                        argb_value = *(*pic).argb.offset(off as isize);
                        need_reset = 0 as ::core::ffi::c_int;
                    }
                    FlattenARGB(
                        (*pic).argb.offset(off as isize),
                        argb_value,
                        (*pic).argb_stride,
                        SIZE,
                    );
                } else {
                    need_reset = 1 as ::core::ffi::c_int;
                }
                x += 1;
            }
            y += 1;
        }
    } else {
        let width: ::core::ffi::c_int = (*pic).width;
        let height: ::core::ffi::c_int = (*pic).height;
        let y_stride: ::core::ffi::c_int = (*pic).y_stride;
        let uv_stride: ::core::ffi::c_int = (*pic).uv_stride;
        let a_stride: ::core::ffi::c_int = (*pic).a_stride;
        let mut y_ptr: *mut uint8_t = (*pic).y;
        let mut u_ptr: *mut uint8_t = (*pic).u;
        let mut v_ptr: *mut uint8_t = (*pic).v;
        let mut a_ptr: *const uint8_t = (*pic).a;
        let mut values: [::core::ffi::c_int; 3] = [0 as ::core::ffi::c_int; 3];
        if a_ptr.is_null() || y_ptr.is_null() || u_ptr.is_null() || v_ptr.is_null() {
            return;
        }
        y = 0 as ::core::ffi::c_int;
        while y + SIZE <= height {
            let mut need_reset_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            x = 0 as ::core::ffi::c_int;
            while x + SIZE <= width {
                if SmoothenBlock(
                    a_ptr.offset(x as isize),
                    a_stride,
                    y_ptr.offset(x as isize),
                    y_stride,
                    SIZE,
                    SIZE,
                ) != 0
                {
                    if need_reset_0 != 0 {
                        values[0 as ::core::ffi::c_int as usize] =
                            *y_ptr.offset(x as isize) as ::core::ffi::c_int;
                        values[1 as ::core::ffi::c_int as usize] = *u_ptr
                            .offset((x >> 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int;
                        values[2 as ::core::ffi::c_int as usize] = *v_ptr
                            .offset((x >> 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int;
                        need_reset_0 = 0 as ::core::ffi::c_int;
                    }
                    Flatten(
                        y_ptr.offset(x as isize),
                        values[0 as ::core::ffi::c_int as usize],
                        y_stride,
                        SIZE,
                    );
                    Flatten(
                        u_ptr.offset((x >> 1 as ::core::ffi::c_int) as isize),
                        values[1 as ::core::ffi::c_int as usize],
                        uv_stride,
                        SIZE2,
                    );
                    Flatten(
                        v_ptr.offset((x >> 1 as ::core::ffi::c_int) as isize),
                        values[2 as ::core::ffi::c_int as usize],
                        uv_stride,
                        SIZE2,
                    );
                } else {
                    need_reset_0 = 1 as ::core::ffi::c_int;
                }
                x += SIZE;
            }
            if x < width {
                SmoothenBlock(
                    a_ptr.offset(x as isize),
                    a_stride,
                    y_ptr.offset(x as isize),
                    y_stride,
                    width - x,
                    SIZE,
                );
            }
            a_ptr = a_ptr.offset((SIZE * a_stride) as isize);
            y_ptr = y_ptr.offset((SIZE * y_stride) as isize);
            u_ptr = u_ptr.offset((SIZE2 * uv_stride) as isize);
            v_ptr = v_ptr.offset((SIZE2 * uv_stride) as isize);
            y += SIZE;
        }
        if y < height {
            let sub_height: ::core::ffi::c_int = height - y;
            x = 0 as ::core::ffi::c_int;
            while x + SIZE <= width {
                SmoothenBlock(
                    a_ptr.offset(x as isize),
                    a_stride,
                    y_ptr.offset(x as isize),
                    y_stride,
                    SIZE,
                    sub_height,
                );
                x += SIZE;
            }
            if x < width {
                SmoothenBlock(
                    a_ptr.offset(x as isize),
                    a_stride,
                    y_ptr.offset(x as isize),
                    y_stride,
                    width - x,
                    sub_height,
                );
            }
        }
    };
}
#[inline]
unsafe extern "C" fn MakeARGB32(
    mut r: ::core::ffi::c_int,
    mut g: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
) -> uint32_t {
    return 0xff000000 as uint32_t
        | (r << 16 as ::core::ffi::c_int) as uint32_t
        | (g << 8 as ::core::ffi::c_int) as uint32_t
        | b as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn WebPBlendAlpha(
    mut picture: *mut WebPPicture,
    mut background_rgb: uint32_t,
) {
    let red: ::core::ffi::c_int =
        (background_rgb >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int;
    let green: ::core::ffi::c_int =
        (background_rgb >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int;
    let blue: ::core::ffi::c_int =
        (background_rgb >> 0 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    if picture.is_null() {
        return;
    }
    if (*picture).use_argb == 0 {
        let uv_width: ::core::ffi::c_int = (*picture).width >> 1 as ::core::ffi::c_int;
        let Y0: ::core::ffi::c_int =
            VP8RGBToY(red, green, blue, YUV_HALF as ::core::ffi::c_int) as ::core::ffi::c_int;
        let U0: ::core::ffi::c_int = VP8RGBToU(
            4 as ::core::ffi::c_int * red,
            4 as ::core::ffi::c_int * green,
            4 as ::core::ffi::c_int * blue,
            4 as ::core::ffi::c_int * YUV_HALF as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        let V0: ::core::ffi::c_int = VP8RGBToV(
            4 as ::core::ffi::c_int * red,
            4 as ::core::ffi::c_int * green,
            4 as ::core::ffi::c_int * blue,
            4 as ::core::ffi::c_int * YUV_HALF as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        let has_alpha: ::core::ffi::c_int = ((*picture).colorspace as ::core::ffi::c_uint
            & WEBP_CSP_ALPHA_BIT as ::core::ffi::c_int as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        let mut y_ptr: *mut uint8_t = (*picture).y;
        let mut u_ptr: *mut uint8_t = (*picture).u;
        let mut v_ptr: *mut uint8_t = (*picture).v;
        let mut a_ptr: *mut uint8_t = (*picture).a;
        if has_alpha == 0 || a_ptr.is_null() {
            return;
        }
        y = 0 as ::core::ffi::c_int;
        while y < (*picture).height {
            x = 0 as ::core::ffi::c_int;
            while x < (*picture).width {
                let alpha: uint8_t = *a_ptr.offset(x as isize);
                if (alpha as ::core::ffi::c_int) < 0xff as ::core::ffi::c_int {
                    *y_ptr.offset(x as isize) =
                        ((Y0 * (255 as ::core::ffi::c_int - alpha as ::core::ffi::c_int)
                            + *y_ptr.offset(x as isize) as ::core::ffi::c_int
                                * alpha as ::core::ffi::c_int)
                            * 0x101 as ::core::ffi::c_int
                            + 256 as ::core::ffi::c_int
                            >> 16 as ::core::ffi::c_int) as uint8_t;
                }
                x += 1;
            }
            if y & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                let a_ptr2: *mut uint8_t = if y + 1 as ::core::ffi::c_int == (*picture).height {
                    a_ptr
                } else {
                    a_ptr.offset((*picture).a_stride as isize)
                };
                x = 0 as ::core::ffi::c_int;
                while x < uv_width {
                    let alpha_0: uint32_t = (*a_ptr
                        .offset((2 as ::core::ffi::c_int * x + 0 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        + *a_ptr.offset(
                            (2 as ::core::ffi::c_int * x + 1 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int
                        + *a_ptr2.offset(
                            (2 as ::core::ffi::c_int * x + 0 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int
                        + *a_ptr2.offset(
                            (2 as ::core::ffi::c_int * x + 1 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int)
                        as uint32_t;
                    *u_ptr.offset(x as isize) = ((U0 as uint32_t)
                        .wrapping_mul((1020 as uint32_t).wrapping_sub(alpha_0))
                        .wrapping_add((*u_ptr.offset(x as isize) as uint32_t).wrapping_mul(alpha_0))
                        .wrapping_mul(0x101 as uint32_t)
                        .wrapping_add(1024 as uint32_t)
                        >> 18 as ::core::ffi::c_int)
                        as uint8_t;
                    *v_ptr.offset(x as isize) = ((V0 as uint32_t)
                        .wrapping_mul((1020 as uint32_t).wrapping_sub(alpha_0))
                        .wrapping_add((*v_ptr.offset(x as isize) as uint32_t).wrapping_mul(alpha_0))
                        .wrapping_mul(0x101 as uint32_t)
                        .wrapping_add(1024 as uint32_t)
                        >> 18 as ::core::ffi::c_int)
                        as uint8_t;
                    x += 1;
                }
                if (*picture).width & 1 as ::core::ffi::c_int != 0 {
                    let alpha_1: uint32_t = (2 as ::core::ffi::c_int
                        * (*a_ptr.offset(
                            (2 as ::core::ffi::c_int * x + 0 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int
                            + *a_ptr2.offset(
                                (2 as ::core::ffi::c_int * x + 0 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_int))
                        as uint32_t;
                    *u_ptr.offset(x as isize) = ((U0 as uint32_t)
                        .wrapping_mul((1020 as uint32_t).wrapping_sub(alpha_1))
                        .wrapping_add((*u_ptr.offset(x as isize) as uint32_t).wrapping_mul(alpha_1))
                        .wrapping_mul(0x101 as uint32_t)
                        .wrapping_add(1024 as uint32_t)
                        >> 18 as ::core::ffi::c_int)
                        as uint8_t;
                    *v_ptr.offset(x as isize) = ((V0 as uint32_t)
                        .wrapping_mul((1020 as uint32_t).wrapping_sub(alpha_1))
                        .wrapping_add((*v_ptr.offset(x as isize) as uint32_t).wrapping_mul(alpha_1))
                        .wrapping_mul(0x101 as uint32_t)
                        .wrapping_add(1024 as uint32_t)
                        >> 18 as ::core::ffi::c_int)
                        as uint8_t;
                }
            } else {
                u_ptr = u_ptr.offset((*picture).uv_stride as isize);
                v_ptr = v_ptr.offset((*picture).uv_stride as isize);
            }
            memset(
                a_ptr as *mut ::core::ffi::c_void,
                0xff as ::core::ffi::c_int,
                (*picture).width as size_t,
            );
            a_ptr = a_ptr.offset((*picture).a_stride as isize);
            y_ptr = y_ptr.offset((*picture).y_stride as isize);
            y += 1;
        }
    } else {
        let mut argb: *mut uint32_t = (*picture).argb;
        let background: uint32_t = MakeARGB32(red, green, blue) as uint32_t;
        y = 0 as ::core::ffi::c_int;
        while y < (*picture).height {
            x = 0 as ::core::ffi::c_int;
            while x < (*picture).width {
                let alpha_2: ::core::ffi::c_int =
                    (*argb.offset(x as isize) >> 24 as ::core::ffi::c_int & 0xff as uint32_t)
                        as ::core::ffi::c_int;
                if alpha_2 != 0xff as ::core::ffi::c_int {
                    if alpha_2 > 0 as ::core::ffi::c_int {
                        let mut r: ::core::ffi::c_int = (*argb.offset(x as isize)
                            >> 16 as ::core::ffi::c_int
                            & 0xff as uint32_t)
                            as ::core::ffi::c_int;
                        let mut g: ::core::ffi::c_int = (*argb.offset(x as isize)
                            >> 8 as ::core::ffi::c_int
                            & 0xff as uint32_t)
                            as ::core::ffi::c_int;
                        let mut b: ::core::ffi::c_int = (*argb.offset(x as isize)
                            >> 0 as ::core::ffi::c_int
                            & 0xff as uint32_t)
                            as ::core::ffi::c_int;
                        r = (red * (255 as ::core::ffi::c_int - alpha_2) + r * alpha_2)
                            * 0x101 as ::core::ffi::c_int
                            + 256 as ::core::ffi::c_int
                            >> 16 as ::core::ffi::c_int;
                        g = (green * (255 as ::core::ffi::c_int - alpha_2) + g * alpha_2)
                            * 0x101 as ::core::ffi::c_int
                            + 256 as ::core::ffi::c_int
                            >> 16 as ::core::ffi::c_int;
                        b = (blue * (255 as ::core::ffi::c_int - alpha_2) + b * alpha_2)
                            * 0x101 as ::core::ffi::c_int
                            + 256 as ::core::ffi::c_int
                            >> 16 as ::core::ffi::c_int;
                        *argb.offset(x as isize) = MakeARGB32(r, g, b);
                    } else {
                        *argb.offset(x as isize) = background;
                    }
                }
                x += 1;
            }
            argb = argb.offset((*picture).argb_stride as isize);
            y += 1;
        }
    };
}
