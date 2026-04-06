extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    static mut VP8GetCPUInfo: VP8CPUInfo;
    fn WebPInitSamplersSSE2();
    fn WebPInitConvertARGBToYUVSSE2();
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type CPUFeature = ::core::ffi::c_uint;
pub const kMSA: CPUFeature = 9;
pub const kMIPSdspR2: CPUFeature = 8;
pub const kMIPS32: CPUFeature = 7;
pub const kNEON: CPUFeature = 6;
pub const kAVX2: CPUFeature = 5;
pub const kAVX: CPUFeature = 4;
pub const kSSE4_1: CPUFeature = 3;
pub const kSlowSSSE3: CPUFeature = 2;
pub const kSSE3: CPUFeature = 1;
pub const kSSE2: CPUFeature = 0;
pub type VP8CPUInfo = Option<unsafe extern "C" fn(CPUFeature) -> ::core::ffi::c_int>;
pub type WebPSamplerRowFunc = Option<
    unsafe extern "C" fn(
        *const uint8_t,
        *const uint8_t,
        *const uint8_t,
        *mut uint8_t,
        ::core::ffi::c_int,
    ) -> (),
>;
pub const YUV_FIX2: C2RustUnnamed = 6;
pub const YUV_MASK2: C2RustUnnamed = 16383;
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
pub const YUV_HALF: C2RustUnnamed = 32768;
pub const YUV_FIX: C2RustUnnamed = 16;
pub type WEBP_CSP_MODE = ::core::ffi::c_uint;
pub const MODE_LAST: WEBP_CSP_MODE = 13;
pub const MODE_YUVA: WEBP_CSP_MODE = 12;
pub const MODE_YUV: WEBP_CSP_MODE = 11;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn MultHi(
    mut v: ::core::ffi::c_int,
    mut coeff: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return v * coeff >> 8 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn VP8Clip8(mut v: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if v & !(YUV_MASK2 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int {
        v >> YUV_FIX2 as ::core::ffi::c_int
    } else if v < 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        255 as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn VP8YUVToR(
    mut y: ::core::ffi::c_int,
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return VP8Clip8(
        MultHi(y, 19077 as ::core::ffi::c_int) + MultHi(v, 26149 as ::core::ffi::c_int)
            - 14234 as ::core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C" fn VP8YUVToG(
    mut y: ::core::ffi::c_int,
    mut u: ::core::ffi::c_int,
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return VP8Clip8(
        MultHi(y, 19077 as ::core::ffi::c_int)
            - MultHi(u, 6419 as ::core::ffi::c_int)
            - MultHi(v, 13320 as ::core::ffi::c_int)
            + 8708 as ::core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C" fn VP8YUVToB(
    mut y: ::core::ffi::c_int,
    mut u: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return VP8Clip8(
        MultHi(y, 19077 as ::core::ffi::c_int) + MultHi(u, 33050 as ::core::ffi::c_int)
            - 17685 as ::core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C" fn VP8YuvToRgb(
    mut y: ::core::ffi::c_int,
    mut u: ::core::ffi::c_int,
    mut v: ::core::ffi::c_int,
    rgb: *mut uint8_t,
) {
    *rgb.offset(0 as ::core::ffi::c_int as isize) = VP8YUVToR(y, v) as uint8_t;
    *rgb.offset(1 as ::core::ffi::c_int as isize) = VP8YUVToG(y, u, v) as uint8_t;
    *rgb.offset(2 as ::core::ffi::c_int as isize) = VP8YUVToB(y, u) as uint8_t;
}
#[inline]
unsafe extern "C" fn VP8YuvToBgr(
    mut y: ::core::ffi::c_int,
    mut u: ::core::ffi::c_int,
    mut v: ::core::ffi::c_int,
    bgr: *mut uint8_t,
) {
    *bgr.offset(0 as ::core::ffi::c_int as isize) = VP8YUVToB(y, u) as uint8_t;
    *bgr.offset(1 as ::core::ffi::c_int as isize) = VP8YUVToG(y, u, v) as uint8_t;
    *bgr.offset(2 as ::core::ffi::c_int as isize) = VP8YUVToR(y, v) as uint8_t;
}
#[inline]
unsafe extern "C" fn VP8YuvToRgb565(
    mut y: ::core::ffi::c_int,
    mut u: ::core::ffi::c_int,
    mut v: ::core::ffi::c_int,
    rgb: *mut uint8_t,
) {
    let r: ::core::ffi::c_int = VP8YUVToR(y, v) as ::core::ffi::c_int;
    let g: ::core::ffi::c_int = VP8YUVToG(y, u, v) as ::core::ffi::c_int;
    let b: ::core::ffi::c_int = VP8YUVToB(y, u) as ::core::ffi::c_int;
    let rg: ::core::ffi::c_int = r & 0xf8 as ::core::ffi::c_int | g >> 5 as ::core::ffi::c_int;
    let gb: ::core::ffi::c_int =
        g << 3 as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int | b >> 3 as ::core::ffi::c_int;
    *rgb.offset(0 as ::core::ffi::c_int as isize) = rg as uint8_t;
    *rgb.offset(1 as ::core::ffi::c_int as isize) = gb as uint8_t;
}
#[inline]
unsafe extern "C" fn VP8YuvToRgba4444(
    mut y: ::core::ffi::c_int,
    mut u: ::core::ffi::c_int,
    mut v: ::core::ffi::c_int,
    argb: *mut uint8_t,
) {
    let r: ::core::ffi::c_int = VP8YUVToR(y, v) as ::core::ffi::c_int;
    let g: ::core::ffi::c_int = VP8YUVToG(y, u, v) as ::core::ffi::c_int;
    let b: ::core::ffi::c_int = VP8YUVToB(y, u) as ::core::ffi::c_int;
    let rg: ::core::ffi::c_int = r & 0xf0 as ::core::ffi::c_int | g >> 4 as ::core::ffi::c_int;
    let ba: ::core::ffi::c_int = b & 0xf0 as ::core::ffi::c_int | 0xf as ::core::ffi::c_int;
    *argb.offset(0 as ::core::ffi::c_int as isize) = rg as uint8_t;
    *argb.offset(1 as ::core::ffi::c_int as isize) = ba as uint8_t;
}
#[inline]
unsafe extern "C" fn VP8YuvToArgb(
    mut y: uint8_t,
    mut u: uint8_t,
    mut v: uint8_t,
    argb: *mut uint8_t,
) {
    *argb.offset(0 as ::core::ffi::c_int as isize) = 0xff as uint8_t;
    VP8YuvToRgb(
        y as ::core::ffi::c_int,
        u as ::core::ffi::c_int,
        v as ::core::ffi::c_int,
        argb.offset(1 as ::core::ffi::c_int as isize),
    );
}
#[inline]
unsafe extern "C" fn VP8YuvToBgra(
    mut y: uint8_t,
    mut u: uint8_t,
    mut v: uint8_t,
    bgra: *mut uint8_t,
) {
    VP8YuvToBgr(
        y as ::core::ffi::c_int,
        u as ::core::ffi::c_int,
        v as ::core::ffi::c_int,
        bgra,
    );
    *bgra.offset(3 as ::core::ffi::c_int as isize) = 0xff as uint8_t;
}
#[inline]
unsafe extern "C" fn VP8YuvToRgba(
    mut y: uint8_t,
    mut u: uint8_t,
    mut v: uint8_t,
    rgba: *mut uint8_t,
) {
    VP8YuvToRgb(
        y as ::core::ffi::c_int,
        u as ::core::ffi::c_int,
        v as ::core::ffi::c_int,
        rgba,
    );
    *rgba.offset(3 as ::core::ffi::c_int as isize) = 0xff as uint8_t;
}
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
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 41] = unsafe {
    ::core::mem::transmute::<[u8; 41], [::core::ffi::c_char; 41]>(
        *b"void WebPInitConvertARGBToYUV_body(void)\0",
    )
};
unsafe extern "C" fn YuvToRgbRow(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: ::core::ffi::c_int,
) {
    let end: *const uint8_t =
        dst.offset(((len & !(1 as ::core::ffi::c_int)) * 3 as ::core::ffi::c_int) as isize);
    while dst != end as *mut uint8_t {
        VP8YuvToRgb(
            *y.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *u.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *v.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            dst,
        );
        VP8YuvToRgb(
            *y.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *u.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *v.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            dst.offset(3 as ::core::ffi::c_int as isize),
        );
        y = y.offset(2 as ::core::ffi::c_int as isize);
        u = u.offset(1);
        v = v.offset(1);
        dst = dst.offset((2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize);
    }
    if len & 1 as ::core::ffi::c_int != 0 {
        VP8YuvToRgb(
            *y.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *u.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *v.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            dst,
        );
    }
}
unsafe extern "C" fn YuvToBgrRow(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: ::core::ffi::c_int,
) {
    let end: *const uint8_t =
        dst.offset(((len & !(1 as ::core::ffi::c_int)) * 3 as ::core::ffi::c_int) as isize);
    while dst != end as *mut uint8_t {
        VP8YuvToBgr(
            *y.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *u.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *v.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            dst,
        );
        VP8YuvToBgr(
            *y.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *u.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *v.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            dst.offset(3 as ::core::ffi::c_int as isize),
        );
        y = y.offset(2 as ::core::ffi::c_int as isize);
        u = u.offset(1);
        v = v.offset(1);
        dst = dst.offset((2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize);
    }
    if len & 1 as ::core::ffi::c_int != 0 {
        VP8YuvToBgr(
            *y.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *u.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *v.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            dst,
        );
    }
}
unsafe extern "C" fn YuvToRgbaRow(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: ::core::ffi::c_int,
) {
    let end: *const uint8_t =
        dst.offset(((len & !(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize);
    while dst != end as *mut uint8_t {
        VP8YuvToRgba(
            *y.offset(0 as ::core::ffi::c_int as isize),
            *u.offset(0 as ::core::ffi::c_int as isize),
            *v.offset(0 as ::core::ffi::c_int as isize),
            dst,
        );
        VP8YuvToRgba(
            *y.offset(1 as ::core::ffi::c_int as isize),
            *u.offset(0 as ::core::ffi::c_int as isize),
            *v.offset(0 as ::core::ffi::c_int as isize),
            dst.offset(4 as ::core::ffi::c_int as isize),
        );
        y = y.offset(2 as ::core::ffi::c_int as isize);
        u = u.offset(1);
        v = v.offset(1);
        dst = dst.offset((2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize);
    }
    if len & 1 as ::core::ffi::c_int != 0 {
        VP8YuvToRgba(
            *y.offset(0 as ::core::ffi::c_int as isize),
            *u.offset(0 as ::core::ffi::c_int as isize),
            *v.offset(0 as ::core::ffi::c_int as isize),
            dst,
        );
    }
}
unsafe extern "C" fn YuvToBgraRow(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: ::core::ffi::c_int,
) {
    let end: *const uint8_t =
        dst.offset(((len & !(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize);
    while dst != end as *mut uint8_t {
        VP8YuvToBgra(
            *y.offset(0 as ::core::ffi::c_int as isize),
            *u.offset(0 as ::core::ffi::c_int as isize),
            *v.offset(0 as ::core::ffi::c_int as isize),
            dst,
        );
        VP8YuvToBgra(
            *y.offset(1 as ::core::ffi::c_int as isize),
            *u.offset(0 as ::core::ffi::c_int as isize),
            *v.offset(0 as ::core::ffi::c_int as isize),
            dst.offset(4 as ::core::ffi::c_int as isize),
        );
        y = y.offset(2 as ::core::ffi::c_int as isize);
        u = u.offset(1);
        v = v.offset(1);
        dst = dst.offset((2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize);
    }
    if len & 1 as ::core::ffi::c_int != 0 {
        VP8YuvToBgra(
            *y.offset(0 as ::core::ffi::c_int as isize),
            *u.offset(0 as ::core::ffi::c_int as isize),
            *v.offset(0 as ::core::ffi::c_int as isize),
            dst,
        );
    }
}
unsafe extern "C" fn YuvToArgbRow(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: ::core::ffi::c_int,
) {
    let end: *const uint8_t =
        dst.offset(((len & !(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize);
    while dst != end as *mut uint8_t {
        VP8YuvToArgb(
            *y.offset(0 as ::core::ffi::c_int as isize),
            *u.offset(0 as ::core::ffi::c_int as isize),
            *v.offset(0 as ::core::ffi::c_int as isize),
            dst,
        );
        VP8YuvToArgb(
            *y.offset(1 as ::core::ffi::c_int as isize),
            *u.offset(0 as ::core::ffi::c_int as isize),
            *v.offset(0 as ::core::ffi::c_int as isize),
            dst.offset(4 as ::core::ffi::c_int as isize),
        );
        y = y.offset(2 as ::core::ffi::c_int as isize);
        u = u.offset(1);
        v = v.offset(1);
        dst = dst.offset((2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize);
    }
    if len & 1 as ::core::ffi::c_int != 0 {
        VP8YuvToArgb(
            *y.offset(0 as ::core::ffi::c_int as isize),
            *u.offset(0 as ::core::ffi::c_int as isize),
            *v.offset(0 as ::core::ffi::c_int as isize),
            dst,
        );
    }
}
unsafe extern "C" fn YuvToRgba4444Row(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: ::core::ffi::c_int,
) {
    let end: *const uint8_t =
        dst.offset(((len & !(1 as ::core::ffi::c_int)) * 2 as ::core::ffi::c_int) as isize);
    while dst != end as *mut uint8_t {
        VP8YuvToRgba4444(
            *y.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *u.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *v.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            dst,
        );
        VP8YuvToRgba4444(
            *y.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *u.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *v.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            dst.offset(2 as ::core::ffi::c_int as isize),
        );
        y = y.offset(2 as ::core::ffi::c_int as isize);
        u = u.offset(1);
        v = v.offset(1);
        dst = dst.offset((2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize);
    }
    if len & 1 as ::core::ffi::c_int != 0 {
        VP8YuvToRgba4444(
            *y.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *u.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *v.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            dst,
        );
    }
}
unsafe extern "C" fn YuvToRgb565Row(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: ::core::ffi::c_int,
) {
    let end: *const uint8_t =
        dst.offset(((len & !(1 as ::core::ffi::c_int)) * 2 as ::core::ffi::c_int) as isize);
    while dst != end as *mut uint8_t {
        VP8YuvToRgb565(
            *y.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *u.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *v.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            dst,
        );
        VP8YuvToRgb565(
            *y.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *u.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *v.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            dst.offset(2 as ::core::ffi::c_int as isize),
        );
        y = y.offset(2 as ::core::ffi::c_int as isize);
        u = u.offset(1);
        v = v.offset(1);
        dst = dst.offset((2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize);
    }
    if len & 1 as ::core::ffi::c_int != 0 {
        VP8YuvToRgb565(
            *y.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *u.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *v.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            dst,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPSamplerProcessPlane(
    mut y: *const uint8_t,
    mut y_stride: ::core::ffi::c_int,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut uv_stride: ::core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut func: WebPSamplerRowFunc,
) {
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < height {
        func.expect("non-null function pointer")(y, u, v, dst, width);
        y = y.offset(y_stride as isize);
        if j & 1 as ::core::ffi::c_int != 0 {
            u = u.offset(uv_stride as isize);
            v = v.offset(uv_stride as isize);
        }
        dst = dst.offset(dst_stride as isize);
        j += 1;
    }
}
#[no_mangle]
pub static mut WebPSamplers: [WebPSamplerRowFunc; 13] = [None; 13];
unsafe extern "C" fn WebPInitSamplers_body() {
    WebPSamplers[MODE_RGB as ::core::ffi::c_int as usize] = Some(
        YuvToRgbRow
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> (),
    ) as WebPSamplerRowFunc;
    WebPSamplers[MODE_RGBA as ::core::ffi::c_int as usize] = Some(
        YuvToRgbaRow
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> (),
    ) as WebPSamplerRowFunc;
    WebPSamplers[MODE_BGR as ::core::ffi::c_int as usize] = Some(
        YuvToBgrRow
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> (),
    ) as WebPSamplerRowFunc;
    WebPSamplers[MODE_BGRA as ::core::ffi::c_int as usize] = Some(
        YuvToBgraRow
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> (),
    ) as WebPSamplerRowFunc;
    WebPSamplers[MODE_ARGB as ::core::ffi::c_int as usize] = Some(
        YuvToArgbRow
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> (),
    ) as WebPSamplerRowFunc;
    WebPSamplers[MODE_RGBA_4444 as ::core::ffi::c_int as usize] = Some(
        YuvToRgba4444Row
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> (),
    ) as WebPSamplerRowFunc;
    WebPSamplers[MODE_RGB_565 as ::core::ffi::c_int as usize] = Some(
        YuvToRgb565Row
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> (),
    ) as WebPSamplerRowFunc;
    WebPSamplers[MODE_rgbA as ::core::ffi::c_int as usize] = Some(
        YuvToRgbaRow
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> (),
    ) as WebPSamplerRowFunc;
    WebPSamplers[MODE_bgrA as ::core::ffi::c_int as usize] = Some(
        YuvToBgraRow
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> (),
    ) as WebPSamplerRowFunc;
    WebPSamplers[MODE_Argb as ::core::ffi::c_int as usize] = Some(
        YuvToArgbRow
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> (),
    ) as WebPSamplerRowFunc;
    WebPSamplers[MODE_rgbA_4444 as ::core::ffi::c_int as usize] = Some(
        YuvToRgba4444Row
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> (),
    ) as WebPSamplerRowFunc;
    if VP8GetCPUInfo.is_some() {
        if VP8GetCPUInfo.expect("non-null function pointer")(kSSE2) != 0 {
            WebPInitSamplersSSE2();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPInitSamplers() {
    static mut WebPInitSamplers_body_last_cpuinfo_used: VP8CPUInfo = None;
    if !(WebPInitSamplers_body_last_cpuinfo_used == VP8GetCPUInfo) {
        WebPInitSamplers_body();
        ::core::ptr::write_volatile(
            &mut WebPInitSamplers_body_last_cpuinfo_used as *mut VP8CPUInfo,
            VP8GetCPUInfo,
        );
    }
}
unsafe extern "C" fn ConvertARGBToY_C(
    mut argb: *const uint32_t,
    mut y: *mut uint8_t,
    mut width: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < width {
        let p: uint32_t = *argb.offset(i as isize);
        *y.offset(i as isize) = VP8RGBToY(
            (p >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
            (p >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
            (p >> 0 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
            YUV_HALF as ::core::ffi::c_int,
        ) as uint8_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPConvertARGBToUV_C(
    mut argb: *const uint32_t,
    mut u: *mut uint8_t,
    mut v: *mut uint8_t,
    mut src_width: ::core::ffi::c_int,
    mut do_store: ::core::ffi::c_int,
) {
    let uv_width: ::core::ffi::c_int = src_width >> 1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < uv_width {
        let v0: uint32_t =
            *argb.offset((2 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as isize);
        let v1: uint32_t =
            *argb.offset((2 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as isize);
        let r: ::core::ffi::c_int = (v0 >> 15 as ::core::ffi::c_int & 0x1fe as uint32_t)
            .wrapping_add(v1 >> 15 as ::core::ffi::c_int & 0x1fe as uint32_t)
            as ::core::ffi::c_int;
        let g: ::core::ffi::c_int = (v0 >> 7 as ::core::ffi::c_int & 0x1fe as uint32_t)
            .wrapping_add(v1 >> 7 as ::core::ffi::c_int & 0x1fe as uint32_t)
            as ::core::ffi::c_int;
        let b: ::core::ffi::c_int = (v0 << 1 as ::core::ffi::c_int & 0x1fe as uint32_t)
            .wrapping_add(v1 << 1 as ::core::ffi::c_int & 0x1fe as uint32_t)
            as ::core::ffi::c_int;
        let tmp_u: ::core::ffi::c_int = VP8RGBToU(
            r,
            g,
            b,
            (YUV_HALF as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        let tmp_v: ::core::ffi::c_int = VP8RGBToV(
            r,
            g,
            b,
            (YUV_HALF as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        if do_store != 0 {
            *u.offset(i as isize) = tmp_u as uint8_t;
            *v.offset(i as isize) = tmp_v as uint8_t;
        } else {
            *u.offset(i as isize) =
                (*u.offset(i as isize) as ::core::ffi::c_int + tmp_u + 1 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) as uint8_t;
            *v.offset(i as isize) =
                (*v.offset(i as isize) as ::core::ffi::c_int + tmp_v + 1 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) as uint8_t;
        }
        i += 1;
    }
    if src_width & 1 as ::core::ffi::c_int != 0 {
        let v0_0: uint32_t =
            *argb.offset((2 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as isize);
        let r_0: ::core::ffi::c_int =
            (v0_0 >> 14 as ::core::ffi::c_int & 0x3fc as uint32_t) as ::core::ffi::c_int;
        let g_0: ::core::ffi::c_int =
            (v0_0 >> 6 as ::core::ffi::c_int & 0x3fc as uint32_t) as ::core::ffi::c_int;
        let b_0: ::core::ffi::c_int =
            (v0_0 << 2 as ::core::ffi::c_int & 0x3fc as uint32_t) as ::core::ffi::c_int;
        let tmp_u_0: ::core::ffi::c_int = VP8RGBToU(
            r_0,
            g_0,
            b_0,
            (YUV_HALF as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        let tmp_v_0: ::core::ffi::c_int = VP8RGBToV(
            r_0,
            g_0,
            b_0,
            (YUV_HALF as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        if do_store != 0 {
            *u.offset(i as isize) = tmp_u_0 as uint8_t;
            *v.offset(i as isize) = tmp_v_0 as uint8_t;
        } else {
            *u.offset(i as isize) =
                (*u.offset(i as isize) as ::core::ffi::c_int + tmp_u_0 + 1 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) as uint8_t;
            *v.offset(i as isize) =
                (*v.offset(i as isize) as ::core::ffi::c_int + tmp_v_0 + 1 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) as uint8_t;
        }
    }
}
unsafe extern "C" fn ConvertRGB24ToY_C(
    mut rgb: *const uint8_t,
    mut y: *mut uint8_t,
    mut width: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < width {
        *y.offset(i as isize) = VP8RGBToY(
            *rgb.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *rgb.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *rgb.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            YUV_HALF as ::core::ffi::c_int,
        ) as uint8_t;
        i += 1;
        rgb = rgb.offset(3 as ::core::ffi::c_int as isize);
    }
}
unsafe extern "C" fn ConvertBGR24ToY_C(
    mut bgr: *const uint8_t,
    mut y: *mut uint8_t,
    mut width: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < width {
        *y.offset(i as isize) = VP8RGBToY(
            *bgr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *bgr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *bgr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            YUV_HALF as ::core::ffi::c_int,
        ) as uint8_t;
        i += 1;
        bgr = bgr.offset(3 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPConvertRGBA32ToUV_C(
    mut rgb: *const uint16_t,
    mut u: *mut uint8_t,
    mut v: *mut uint8_t,
    mut width: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < width {
        let r: ::core::ffi::c_int =
            *rgb.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let g: ::core::ffi::c_int =
            *rgb.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let b: ::core::ffi::c_int =
            *rgb.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        *u.offset(i as isize) = VP8RGBToU(
            r,
            g,
            b,
            (YUV_HALF as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
        ) as uint8_t;
        *v.offset(i as isize) = VP8RGBToV(
            r,
            g,
            b,
            (YUV_HALF as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
        ) as uint8_t;
        i += 1 as ::core::ffi::c_int;
        rgb = rgb.offset(4 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub static mut WebPConvertRGB24ToY: Option<
    unsafe extern "C" fn(*const uint8_t, *mut uint8_t, ::core::ffi::c_int) -> (),
> = None;
#[no_mangle]
pub static mut WebPConvertBGR24ToY: Option<
    unsafe extern "C" fn(*const uint8_t, *mut uint8_t, ::core::ffi::c_int) -> (),
> = None;
#[no_mangle]
pub static mut WebPConvertRGBA32ToUV: Option<
    unsafe extern "C" fn(*const uint16_t, *mut uint8_t, *mut uint8_t, ::core::ffi::c_int) -> (),
> = None;
#[no_mangle]
pub static mut WebPConvertARGBToY: Option<
    unsafe extern "C" fn(*const uint32_t, *mut uint8_t, ::core::ffi::c_int) -> (),
> = None;
#[no_mangle]
pub static mut WebPConvertARGBToUV: Option<
    unsafe extern "C" fn(
        *const uint32_t,
        *mut uint8_t,
        *mut uint8_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
> = None;
unsafe extern "C" fn WebPInitConvertARGBToYUV_body() {
    WebPConvertARGBToY = Some(
        ConvertARGBToY_C
            as unsafe extern "C" fn(*const uint32_t, *mut uint8_t, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*const uint32_t, *mut uint8_t, ::core::ffi::c_int) -> ()>;
    WebPConvertARGBToUV = Some(
        WebPConvertARGBToUV_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *mut uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *const uint32_t,
                *mut uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >;
    WebPConvertRGB24ToY = Some(
        ConvertRGB24ToY_C
            as unsafe extern "C" fn(*const uint8_t, *mut uint8_t, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*const uint8_t, *mut uint8_t, ::core::ffi::c_int) -> ()>;
    WebPConvertBGR24ToY = Some(
        ConvertBGR24ToY_C
            as unsafe extern "C" fn(*const uint8_t, *mut uint8_t, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*const uint8_t, *mut uint8_t, ::core::ffi::c_int) -> ()>;
    WebPConvertRGBA32ToUV = Some(
        WebPConvertRGBA32ToUV_C
            as unsafe extern "C" fn(
                *const uint16_t,
                *mut uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *const uint16_t,
                *mut uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    if VP8GetCPUInfo.is_some() {
        if VP8GetCPUInfo.expect("non-null function pointer")(kSSE2) != 0 {
            WebPInitConvertARGBToYUVSSE2();
        }
    }
    '_c2rust_label: {
        if WebPConvertARGBToY.is_some() {
        } else {
            __assert_fail(
                b"WebPConvertARGBToY != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/yuv.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                240 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_0: {
        if WebPConvertARGBToUV.is_some() {
        } else {
            __assert_fail(
                b"WebPConvertARGBToUV != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/yuv.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                241 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_1: {
        if WebPConvertRGB24ToY.is_some() {
        } else {
            __assert_fail(
                b"WebPConvertRGB24ToY != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/yuv.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                242 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_2: {
        if WebPConvertBGR24ToY.is_some() {
        } else {
            __assert_fail(
                b"WebPConvertBGR24ToY != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/yuv.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                243 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_3: {
        if WebPConvertRGBA32ToUV.is_some() {
        } else {
            __assert_fail(
                b"WebPConvertRGBA32ToUV != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/yuv.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                244 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPInitConvertARGBToYUV() {
    static mut WebPInitConvertARGBToYUV_body_last_cpuinfo_used: VP8CPUInfo = None;
    if !(WebPInitConvertARGBToYUV_body_last_cpuinfo_used == VP8GetCPUInfo) {
        WebPInitConvertARGBToYUV_body();
        ::core::ptr::write_volatile(
            &mut WebPInitConvertARGBToYUV_body_last_cpuinfo_used as *mut VP8CPUInfo,
            VP8GetCPUInfo,
        );
    }
}
