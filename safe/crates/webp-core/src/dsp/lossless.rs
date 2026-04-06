extern "C" {
    static mut WebPApplyAlphaMultiply: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >;
    static mut WebPApplyAlphaMultiply4444: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    static mut VP8GetCPUInfo: VP8CPUInfo;
    fn VP8LDspInitSSE2();
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type int8_t = __int8_t;
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
pub type VP8LImageTransformType = ::core::ffi::c_uint;
pub const COLOR_INDEXING_TRANSFORM: VP8LImageTransformType = 3;
pub const SUBTRACT_GREEN_TRANSFORM: VP8LImageTransformType = 2;
pub const CROSS_COLOR_TRANSFORM: VP8LImageTransformType = 1;
pub const PREDICTOR_TRANSFORM: VP8LImageTransformType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LTransform {
    pub type_: VP8LImageTransformType,
    pub bits_: ::core::ffi::c_int,
    pub xsize_: ::core::ffi::c_int,
    pub ysize_: ::core::ffi::c_int,
    pub data_: *mut uint32_t,
}
pub type VP8LPredictorFunc =
    Option<unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t>;
pub type VP8LPredictorAddSubFunc = Option<
    unsafe extern "C" fn(*const uint32_t, *const uint32_t, ::core::ffi::c_int, *mut uint32_t) -> (),
>;
pub type VP8LProcessDecBlueAndRedFunc =
    Option<unsafe extern "C" fn(*const uint32_t, ::core::ffi::c_int, *mut uint32_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LMultipliers {
    pub green_to_red_: uint8_t,
    pub green_to_blue_: uint8_t,
    pub red_to_blue_: uint8_t,
}
pub type VP8LTransformColorInverseFunc = Option<
    unsafe extern "C" fn(
        *const VP8LMultipliers,
        *const uint32_t,
        ::core::ffi::c_int,
        *mut uint32_t,
    ) -> (),
>;
pub type VP8LMapARGBFunc = Option<
    unsafe extern "C" fn(
        *const uint32_t,
        *const uint32_t,
        *mut uint32_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type VP8LConvertFunc =
    Option<unsafe extern "C" fn(*const uint32_t, ::core::ffi::c_int, *mut uint8_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub w: uint16_t,
    pub b: [uint8_t; 2],
}
pub type VP8LMapAlphaFunc = Option<
    unsafe extern "C" fn(
        *const uint8_t,
        *const uint32_t,
        *mut uint8_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
>;
pub const ARGB_BLACK: ::core::ffi::c_uint = 0xff000000 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn BSwap32(mut x: uint32_t) -> uint32_t {
    return x.swap_bytes();
}
#[inline]
unsafe extern "C" fn WebPUint32ToMem(ptr: *mut uint8_t, mut val: uint32_t) {
    memcpy(
        ptr as *mut ::core::ffi::c_void,
        &raw mut val as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn Average2(mut a0: uint32_t, mut a1: uint32_t) -> uint32_t {
    return (((a0 ^ a1) & 0xfefefefe as uint32_t) >> 1 as ::core::ffi::c_int).wrapping_add(a0 & a1);
}
#[inline]
unsafe extern "C" fn Average3(mut a0: uint32_t, mut a1: uint32_t, mut a2: uint32_t) -> uint32_t {
    return Average2(Average2(a0, a2), a1);
}
#[inline]
unsafe extern "C" fn Average4(
    mut a0: uint32_t,
    mut a1: uint32_t,
    mut a2: uint32_t,
    mut a3: uint32_t,
) -> uint32_t {
    return Average2(Average2(a0, a1), Average2(a2, a3));
}
#[inline]
unsafe extern "C" fn Clip255(mut a: uint32_t) -> uint32_t {
    if a < 256 as uint32_t {
        return a;
    }
    return !a >> 24 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn AddSubtractComponentFull(
    mut a: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return Clip255((a + b - c) as uint32_t) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn ClampedAddSubtractFull(
    mut c0: uint32_t,
    mut c1: uint32_t,
    mut c2: uint32_t,
) -> uint32_t {
    let a: ::core::ffi::c_int = AddSubtractComponentFull(
        (c0 >> 24 as ::core::ffi::c_int) as ::core::ffi::c_int,
        (c1 >> 24 as ::core::ffi::c_int) as ::core::ffi::c_int,
        (c2 >> 24 as ::core::ffi::c_int) as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    let r: ::core::ffi::c_int = AddSubtractComponentFull(
        (c0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
        (c1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
        (c2 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    let g: ::core::ffi::c_int = AddSubtractComponentFull(
        (c0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
        (c1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
        (c2 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    let b: ::core::ffi::c_int = AddSubtractComponentFull(
        (c0 & 0xff as uint32_t) as ::core::ffi::c_int,
        (c1 & 0xff as uint32_t) as ::core::ffi::c_int,
        (c2 & 0xff as uint32_t) as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    return (a as uint32_t) << 24 as ::core::ffi::c_int
        | (r << 16 as ::core::ffi::c_int) as uint32_t
        | (g << 8 as ::core::ffi::c_int) as uint32_t
        | b as uint32_t;
}
#[inline]
unsafe extern "C" fn AddSubtractComponentHalf(
    mut a: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return Clip255((a + (a - b) / 2 as ::core::ffi::c_int) as uint32_t) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn ClampedAddSubtractHalf(
    mut c0: uint32_t,
    mut c1: uint32_t,
    mut c2: uint32_t,
) -> uint32_t {
    let ave: uint32_t = Average2(c0, c1) as uint32_t;
    let a: ::core::ffi::c_int = AddSubtractComponentHalf(
        (ave >> 24 as ::core::ffi::c_int) as ::core::ffi::c_int,
        (c2 >> 24 as ::core::ffi::c_int) as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    let r: ::core::ffi::c_int = AddSubtractComponentHalf(
        (ave >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
        (c2 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    let g: ::core::ffi::c_int = AddSubtractComponentHalf(
        (ave >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
        (c2 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    let b: ::core::ffi::c_int = AddSubtractComponentHalf(
        (ave >> 0 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
        (c2 >> 0 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    return (a as uint32_t) << 24 as ::core::ffi::c_int
        | (r << 16 as ::core::ffi::c_int) as uint32_t
        | (g << 8 as ::core::ffi::c_int) as uint32_t
        | b as uint32_t;
}
#[inline]
unsafe extern "C" fn Sub3(
    mut a: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pb: ::core::ffi::c_int = b - c;
    let pa: ::core::ffi::c_int = a - c;
    return abs(pb) - abs(pa);
}
#[inline]
unsafe extern "C" fn Select(mut a: uint32_t, mut b: uint32_t, mut c: uint32_t) -> uint32_t {
    let pa_minus_pb: ::core::ffi::c_int = Sub3(
        (a >> 24 as ::core::ffi::c_int) as ::core::ffi::c_int,
        (b >> 24 as ::core::ffi::c_int) as ::core::ffi::c_int,
        (c >> 24 as ::core::ffi::c_int) as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        + Sub3(
            (a >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
            (b >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
            (c >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
        + Sub3(
            (a >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
            (b >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
            (c >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
        + Sub3(
            (a & 0xff as uint32_t) as ::core::ffi::c_int,
            (b & 0xff as uint32_t) as ::core::ffi::c_int,
            (c & 0xff as uint32_t) as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
    return if pa_minus_pb <= 0 as ::core::ffi::c_int {
        a
    } else {
        b
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor0_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t {
    return ARGB_BLACK as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor1_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t {
    return *left;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor2_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t {
    return *top.offset(0 as ::core::ffi::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor3_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t {
    return *top.offset(1 as ::core::ffi::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor4_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t {
    return *top.offset(-(1 as ::core::ffi::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor5_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t {
    let pred: uint32_t = Average3(
        *left,
        *top.offset(0 as ::core::ffi::c_int as isize),
        *top.offset(1 as ::core::ffi::c_int as isize),
    ) as uint32_t;
    return pred;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor6_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t {
    let pred: uint32_t =
        Average2(*left, *top.offset(-(1 as ::core::ffi::c_int) as isize)) as uint32_t;
    return pred;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor7_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t {
    let pred: uint32_t = Average2(*left, *top.offset(0 as ::core::ffi::c_int as isize)) as uint32_t;
    return pred;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor8_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t {
    let pred: uint32_t = Average2(
        *top.offset(-(1 as ::core::ffi::c_int) as isize),
        *top.offset(0 as ::core::ffi::c_int as isize),
    ) as uint32_t;
    return pred;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor9_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t {
    let pred: uint32_t = Average2(
        *top.offset(0 as ::core::ffi::c_int as isize),
        *top.offset(1 as ::core::ffi::c_int as isize),
    ) as uint32_t;
    return pred;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor10_C(
    left: *const uint32_t,
    top: *const uint32_t,
) -> uint32_t {
    let pred: uint32_t = Average4(
        *left,
        *top.offset(-(1 as ::core::ffi::c_int) as isize),
        *top.offset(0 as ::core::ffi::c_int as isize),
        *top.offset(1 as ::core::ffi::c_int as isize),
    ) as uint32_t;
    return pred;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor11_C(
    left: *const uint32_t,
    top: *const uint32_t,
) -> uint32_t {
    let pred: uint32_t = Select(
        *top.offset(0 as ::core::ffi::c_int as isize),
        *left,
        *top.offset(-(1 as ::core::ffi::c_int) as isize),
    ) as uint32_t;
    return pred;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor12_C(
    left: *const uint32_t,
    top: *const uint32_t,
) -> uint32_t {
    let pred: uint32_t = ClampedAddSubtractFull(
        *left,
        *top.offset(0 as ::core::ffi::c_int as isize),
        *top.offset(-(1 as ::core::ffi::c_int) as isize),
    ) as uint32_t;
    return pred;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor13_C(
    left: *const uint32_t,
    top: *const uint32_t,
) -> uint32_t {
    let pred: uint32_t = ClampedAddSubtractHalf(
        *left,
        *top.offset(0 as ::core::ffi::c_int as isize),
        *top.offset(-(1 as ::core::ffi::c_int) as isize),
    ) as uint32_t;
    return pred;
}
unsafe extern "C" fn PredictorAdd0_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: ::core::ffi::c_int = 0;
    x = 0 as ::core::ffi::c_int;
    while x < num_pixels {
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), ARGB_BLACK as uint32_t);
        x += 1;
    }
}
unsafe extern "C" fn PredictorAdd1_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut out: *mut uint32_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut left: uint32_t = *out.offset(-(1 as ::core::ffi::c_int) as isize);
    i = 0 as ::core::ffi::c_int;
    while i < num_pixels {
        left = VP8LAddPixels(*in_0.offset(i as isize), left);
        *out.offset(i as isize) = left;
        i += 1;
    }
}
unsafe extern "C" fn PredictorAdd2_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if !upper.is_null() {
        } else {
            __assert_fail(
                b"upper != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                199 as ::core::ffi::c_uint,
                b"void PredictorAdd2_C(const uint32_t *, const uint32_t *, int, uint32_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    x = 0 as ::core::ffi::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor2_C(
            out.offset((x - 1 as ::core::ffi::c_int) as isize) as *mut uint32_t,
            upper.offset(x as isize),
        ) as uint32_t;
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
    }
}
unsafe extern "C" fn PredictorAdd3_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if !upper.is_null() {
        } else {
            __assert_fail(
                b"upper != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                200 as ::core::ffi::c_uint,
                b"void PredictorAdd3_C(const uint32_t *, const uint32_t *, int, uint32_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    x = 0 as ::core::ffi::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor3_C(
            out.offset((x - 1 as ::core::ffi::c_int) as isize) as *mut uint32_t,
            upper.offset(x as isize),
        ) as uint32_t;
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
    }
}
unsafe extern "C" fn PredictorAdd4_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if !upper.is_null() {
        } else {
            __assert_fail(
                b"upper != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                201 as ::core::ffi::c_uint,
                b"void PredictorAdd4_C(const uint32_t *, const uint32_t *, int, uint32_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    x = 0 as ::core::ffi::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor4_C(
            out.offset((x - 1 as ::core::ffi::c_int) as isize) as *mut uint32_t,
            upper.offset(x as isize),
        ) as uint32_t;
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
    }
}
unsafe extern "C" fn PredictorAdd5_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if !upper.is_null() {
        } else {
            __assert_fail(
                b"upper != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                202 as ::core::ffi::c_uint,
                b"void PredictorAdd5_C(const uint32_t *, const uint32_t *, int, uint32_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    x = 0 as ::core::ffi::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor5_C(
            out.offset((x - 1 as ::core::ffi::c_int) as isize) as *mut uint32_t,
            upper.offset(x as isize),
        ) as uint32_t;
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
    }
}
unsafe extern "C" fn PredictorAdd6_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if !upper.is_null() {
        } else {
            __assert_fail(
                b"upper != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                203 as ::core::ffi::c_uint,
                b"void PredictorAdd6_C(const uint32_t *, const uint32_t *, int, uint32_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    x = 0 as ::core::ffi::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor6_C(
            out.offset((x - 1 as ::core::ffi::c_int) as isize) as *mut uint32_t,
            upper.offset(x as isize),
        ) as uint32_t;
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
    }
}
unsafe extern "C" fn PredictorAdd7_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if !upper.is_null() {
        } else {
            __assert_fail(
                b"upper != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                204 as ::core::ffi::c_uint,
                b"void PredictorAdd7_C(const uint32_t *, const uint32_t *, int, uint32_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    x = 0 as ::core::ffi::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor7_C(
            out.offset((x - 1 as ::core::ffi::c_int) as isize) as *mut uint32_t,
            upper.offset(x as isize),
        ) as uint32_t;
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
    }
}
unsafe extern "C" fn PredictorAdd8_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if !upper.is_null() {
        } else {
            __assert_fail(
                b"upper != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_uint,
                b"void PredictorAdd8_C(const uint32_t *, const uint32_t *, int, uint32_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    x = 0 as ::core::ffi::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor8_C(
            out.offset((x - 1 as ::core::ffi::c_int) as isize) as *mut uint32_t,
            upper.offset(x as isize),
        ) as uint32_t;
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
    }
}
unsafe extern "C" fn PredictorAdd9_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if !upper.is_null() {
        } else {
            __assert_fail(
                b"upper != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                206 as ::core::ffi::c_uint,
                b"void PredictorAdd9_C(const uint32_t *, const uint32_t *, int, uint32_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    x = 0 as ::core::ffi::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor9_C(
            out.offset((x - 1 as ::core::ffi::c_int) as isize) as *mut uint32_t,
            upper.offset(x as isize),
        ) as uint32_t;
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
    }
}
unsafe extern "C" fn PredictorAdd10_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if !upper.is_null() {
        } else {
            __assert_fail(
                b"upper != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                207 as ::core::ffi::c_uint,
                b"void PredictorAdd10_C(const uint32_t *, const uint32_t *, int, uint32_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    x = 0 as ::core::ffi::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor10_C(
            out.offset((x - 1 as ::core::ffi::c_int) as isize) as *mut uint32_t,
            upper.offset(x as isize),
        ) as uint32_t;
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
    }
}
unsafe extern "C" fn PredictorAdd11_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if !upper.is_null() {
        } else {
            __assert_fail(
                b"upper != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                208 as ::core::ffi::c_uint,
                b"void PredictorAdd11_C(const uint32_t *, const uint32_t *, int, uint32_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    x = 0 as ::core::ffi::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor11_C(
            out.offset((x - 1 as ::core::ffi::c_int) as isize) as *mut uint32_t,
            upper.offset(x as isize),
        ) as uint32_t;
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
    }
}
unsafe extern "C" fn PredictorAdd12_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if !upper.is_null() {
        } else {
            __assert_fail(
                b"upper != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                209 as ::core::ffi::c_uint,
                b"void PredictorAdd12_C(const uint32_t *, const uint32_t *, int, uint32_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    x = 0 as ::core::ffi::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor12_C(
            out.offset((x - 1 as ::core::ffi::c_int) as isize) as *mut uint32_t,
            upper.offset(x as isize),
        ) as uint32_t;
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
    }
}
unsafe extern "C" fn PredictorAdd13_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if !upper.is_null() {
        } else {
            __assert_fail(
                b"upper != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                210 as ::core::ffi::c_uint,
                b"void PredictorAdd13_C(const uint32_t *, const uint32_t *, int, uint32_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    x = 0 as ::core::ffi::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor13_C(
            out.offset((x - 1 as ::core::ffi::c_int) as isize) as *mut uint32_t,
            upper.offset(x as isize),
        ) as uint32_t;
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
    }
}
unsafe extern "C" fn PredictorInverseTransform_C(
    transform: *const VP8LTransform,
    mut y_start: ::core::ffi::c_int,
    mut y_end: ::core::ffi::c_int,
    mut in_0: *const uint32_t,
    mut out: *mut uint32_t,
) {
    let width: ::core::ffi::c_int = (*transform).xsize_;
    if y_start == 0 as ::core::ffi::c_int {
        PredictorAdd0_C(
            in_0,
            ::core::ptr::null::<uint32_t>(),
            1 as ::core::ffi::c_int,
            out,
        );
        PredictorAdd1_C(
            in_0.offset(1 as ::core::ffi::c_int as isize),
            ::core::ptr::null::<uint32_t>(),
            width - 1 as ::core::ffi::c_int,
            out.offset(1 as ::core::ffi::c_int as isize),
        );
        in_0 = in_0.offset(width as isize);
        out = out.offset(width as isize);
        y_start += 1;
    }
    let mut y: ::core::ffi::c_int = y_start;
    let tile_width: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << (*transform).bits_;
    let mask: ::core::ffi::c_int = tile_width - 1 as ::core::ffi::c_int;
    let tiles_per_row: ::core::ffi::c_int =
        VP8LSubSampleSize(width as uint32_t, (*transform).bits_ as uint32_t) as ::core::ffi::c_int;
    let mut pred_mode_base: *const uint32_t = (*transform)
        .data_
        .offset(((y >> (*transform).bits_) * tiles_per_row) as isize);
    while y < y_end {
        let mut pred_mode_src: *const uint32_t = pred_mode_base;
        let mut x: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        PredictorAdd2_C(
            in_0,
            out.offset(-(width as isize)),
            1 as ::core::ffi::c_int,
            out,
        );
        while x < width {
            let fresh4 = pred_mode_src;
            pred_mode_src = pred_mode_src.offset(1);
            let pred_func: VP8LPredictorAddSubFunc =
                VP8LPredictorsAdd[(*fresh4 >> 8 as ::core::ffi::c_int & 0xf as uint32_t) as usize];
            let mut x_end: ::core::ffi::c_int = (x & !mask) + tile_width;
            if x_end > width {
                x_end = width;
            }
            pred_func.expect("non-null function pointer")(
                in_0.offset(x as isize),
                out.offset(x as isize).offset(-(width as isize)),
                x_end - x,
                out.offset(x as isize),
            );
            x = x_end;
        }
        in_0 = in_0.offset(width as isize);
        out = out.offset(width as isize);
        y += 1;
        if y & mask == 0 as ::core::ffi::c_int {
            pred_mode_base = pred_mode_base.offset(tiles_per_row as isize);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LAddGreenToBlueAndRed_C(
    mut src: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut dst: *mut uint32_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < num_pixels {
        let argb: uint32_t = *src.offset(i as isize);
        let green: uint32_t = argb >> 8 as ::core::ffi::c_int & 0xff as uint32_t;
        let mut red_blue: uint32_t = argb & 0xff00ff as uint32_t;
        red_blue = red_blue.wrapping_add(green << 16 as ::core::ffi::c_int | green);
        red_blue = (red_blue as ::core::ffi::c_uint & 0xff00ff as ::core::ffi::c_uint) as uint32_t;
        *dst.offset(i as isize) = argb & 0xff00ff00 as uint32_t | red_blue;
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn ColorTransformDelta(
    mut color_pred: int8_t,
    mut color: int8_t,
) -> ::core::ffi::c_int {
    return color_pred as ::core::ffi::c_int * color as ::core::ffi::c_int
        >> 5 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn ColorCodeToMultipliers(mut color_code: uint32_t, m: *mut VP8LMultipliers) {
    (*m).green_to_red_ = (color_code >> 0 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    (*m).green_to_blue_ = (color_code >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    (*m).red_to_blue_ = (color_code >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LTransformColorInverse_C(
    m: *const VP8LMultipliers,
    mut src: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut dst: *mut uint32_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < num_pixels {
        let argb: uint32_t = *src.offset(i as isize);
        let green: int8_t = (argb >> 8 as ::core::ffi::c_int) as int8_t;
        let red: uint32_t = argb >> 16 as ::core::ffi::c_int;
        let mut new_red: ::core::ffi::c_int = (red & 0xff as uint32_t) as ::core::ffi::c_int;
        let mut new_blue: ::core::ffi::c_int = (argb & 0xff as uint32_t) as ::core::ffi::c_int;
        new_red += ColorTransformDelta((*m).green_to_red_ as int8_t, green);
        new_red &= 0xff as ::core::ffi::c_int;
        new_blue += ColorTransformDelta((*m).green_to_blue_ as int8_t, green);
        new_blue += ColorTransformDelta((*m).red_to_blue_ as int8_t, new_red as int8_t);
        new_blue &= 0xff as ::core::ffi::c_int;
        *dst.offset(i as isize) = argb & 0xff00ff00 as uint32_t
            | (new_red << 16 as ::core::ffi::c_int) as uint32_t
            | new_blue as uint32_t;
        i += 1;
    }
}
unsafe extern "C" fn ColorSpaceInverseTransform_C(
    transform: *const VP8LTransform,
    mut y_start: ::core::ffi::c_int,
    mut y_end: ::core::ffi::c_int,
    mut src: *const uint32_t,
    mut dst: *mut uint32_t,
) {
    let width: ::core::ffi::c_int = (*transform).xsize_;
    let tile_width: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << (*transform).bits_;
    let mask: ::core::ffi::c_int = tile_width - 1 as ::core::ffi::c_int;
    let safe_width: ::core::ffi::c_int = width & !mask;
    let remaining_width: ::core::ffi::c_int = width - safe_width;
    let tiles_per_row: ::core::ffi::c_int =
        VP8LSubSampleSize(width as uint32_t, (*transform).bits_ as uint32_t) as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = y_start;
    let mut pred_row: *const uint32_t = (*transform)
        .data_
        .offset(((y >> (*transform).bits_) * tiles_per_row) as isize);
    while y < y_end {
        let mut pred: *const uint32_t = pred_row;
        let mut m: VP8LMultipliers = VP8LMultipliers {
            green_to_red_: 0 as uint8_t,
            green_to_blue_: 0 as uint8_t,
            red_to_blue_: 0 as uint8_t,
        };
        let src_safe_end: *const uint32_t = src.offset(safe_width as isize);
        let src_end: *const uint32_t = src.offset(width as isize);
        while src < src_safe_end {
            let fresh2 = pred;
            pred = pred.offset(1);
            ColorCodeToMultipliers(*fresh2, &raw mut m);
            VP8LTransformColorInverse.expect("non-null function pointer")(
                &raw mut m, src, tile_width, dst,
            );
            src = src.offset(tile_width as isize);
            dst = dst.offset(tile_width as isize);
        }
        if src < src_end {
            let fresh3 = pred;
            pred = pred.offset(1);
            ColorCodeToMultipliers(*fresh3, &raw mut m);
            VP8LTransformColorInverse.expect("non-null function pointer")(
                &raw mut m,
                src,
                remaining_width,
                dst,
            );
            src = src.offset(remaining_width as isize);
            dst = dst.offset(remaining_width as isize);
        }
        y += 1;
        if y & mask == 0 as ::core::ffi::c_int {
            pred_row = pred_row.offset(tiles_per_row as isize);
        }
    }
}
unsafe extern "C" fn MapARGB_C(
    mut src: *const uint32_t,
    color_map: *const uint32_t,
    mut dst: *mut uint32_t,
    mut y_start: ::core::ffi::c_int,
    mut y_end: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0;
    y = y_start;
    while y < y_end {
        let mut x: ::core::ffi::c_int = 0;
        x = 0 as ::core::ffi::c_int;
        while x < width {
            let fresh29 = src;
            src = src.offset(1);
            let fresh30 = dst;
            dst = dst.offset(1);
            *fresh30 = VP8GetARGBValue(*color_map.offset(VP8GetARGBIndex(*fresh29) as isize));
            x += 1;
        }
        y += 1;
    }
}
unsafe extern "C" fn ColorIndexInverseTransform_C(
    transform: *const VP8LTransform,
    mut y_start: ::core::ffi::c_int,
    mut y_end: ::core::ffi::c_int,
    mut src: *const uint32_t,
    mut dst: *mut uint32_t,
) {
    let mut y: ::core::ffi::c_int = 0;
    let bits_per_pixel: ::core::ffi::c_int = 8 as ::core::ffi::c_int >> (*transform).bits_;
    let width: ::core::ffi::c_int = (*transform).xsize_;
    let color_map: *const uint32_t = (*transform).data_;
    if bits_per_pixel < 8 as ::core::ffi::c_int {
        let pixels_per_byte: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << (*transform).bits_;
        let count_mask: ::core::ffi::c_int = pixels_per_byte - 1 as ::core::ffi::c_int;
        let bit_mask: uint32_t =
            (((1 as ::core::ffi::c_int) << bits_per_pixel) - 1 as ::core::ffi::c_int) as uint32_t;
        y = y_start;
        while y < y_end {
            let mut packed_pixels: uint32_t = 0 as uint32_t;
            let mut x: ::core::ffi::c_int = 0;
            x = 0 as ::core::ffi::c_int;
            while x < width {
                if x & count_mask == 0 as ::core::ffi::c_int {
                    let fresh0 = src;
                    src = src.offset(1);
                    packed_pixels = VP8GetARGBIndex(*fresh0);
                }
                let fresh1 = dst;
                dst = dst.offset(1);
                *fresh1 = VP8GetARGBValue(*color_map.offset((packed_pixels & bit_mask) as isize));
                packed_pixels >>= bits_per_pixel;
                x += 1;
            }
            y += 1;
        }
    } else {
        VP8LMapColor32b.expect("non-null function pointer")(
            src, color_map, dst, y_start, y_end, width,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LColorIndexInverseTransformAlpha(
    transform: *const VP8LTransform,
    mut y_start: ::core::ffi::c_int,
    mut y_end: ::core::ffi::c_int,
    mut src: *const uint8_t,
    mut dst: *mut uint8_t,
) {
    let mut y: ::core::ffi::c_int = 0;
    let bits_per_pixel: ::core::ffi::c_int = 8 as ::core::ffi::c_int >> (*transform).bits_;
    let width: ::core::ffi::c_int = (*transform).xsize_;
    let color_map: *const uint32_t = (*transform).data_;
    if bits_per_pixel < 8 as ::core::ffi::c_int {
        let pixels_per_byte: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << (*transform).bits_;
        let count_mask: ::core::ffi::c_int = pixels_per_byte - 1 as ::core::ffi::c_int;
        let bit_mask: uint32_t =
            (((1 as ::core::ffi::c_int) << bits_per_pixel) - 1 as ::core::ffi::c_int) as uint32_t;
        y = y_start;
        while y < y_end {
            let mut packed_pixels: uint32_t = 0 as uint32_t;
            let mut x: ::core::ffi::c_int = 0;
            x = 0 as ::core::ffi::c_int;
            while x < width {
                if x & count_mask == 0 as ::core::ffi::c_int {
                    let fresh6 = src;
                    src = src.offset(1);
                    packed_pixels = VP8GetAlphaIndex(*fresh6) as uint32_t;
                }
                let fresh7 = dst;
                dst = dst.offset(1);
                *fresh7 = VP8GetAlphaValue(*color_map.offset((packed_pixels & bit_mask) as isize));
                packed_pixels >>= bits_per_pixel;
                x += 1;
            }
            y += 1;
        }
    } else {
        VP8LMapColor8b.expect("non-null function pointer")(
            src, color_map, dst, y_start, y_end, width,
        );
    };
}
unsafe extern "C" fn MapAlpha_C(
    mut src: *const uint8_t,
    color_map: *const uint32_t,
    mut dst: *mut uint8_t,
    mut y_start: ::core::ffi::c_int,
    mut y_end: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0;
    y = y_start;
    while y < y_end {
        let mut x: ::core::ffi::c_int = 0;
        x = 0 as ::core::ffi::c_int;
        while x < width {
            let fresh27 = src;
            src = src.offset(1);
            let fresh28 = dst;
            dst = dst.offset(1);
            *fresh28 = VP8GetAlphaValue(*color_map.offset(VP8GetAlphaIndex(*fresh27) as isize));
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LInverseTransform(
    transform: *const VP8LTransform,
    mut row_start: ::core::ffi::c_int,
    mut row_end: ::core::ffi::c_int,
    in_0: *const uint32_t,
    out: *mut uint32_t,
) {
    let width: ::core::ffi::c_int = (*transform).xsize_;
    '_c2rust_label: {
        if row_start < row_end {
        } else {
            __assert_fail(
                b"row_start < row_end\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                395 as ::core::ffi::c_uint,
                b"void VP8LInverseTransform(const VP8LTransform *const, int, int, const uint32_t *const, uint32_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if row_end <= (*transform).ysize_ {
        } else {
            __assert_fail(
                b"row_end <= transform->ysize_\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                396 as ::core::ffi::c_uint,
                b"void VP8LInverseTransform(const VP8LTransform *const, int, int, const uint32_t *const, uint32_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    match (*transform).type_ as ::core::ffi::c_uint {
        2 => {
            VP8LAddGreenToBlueAndRed.expect("non-null function pointer")(
                in_0,
                (row_end - row_start) * width,
                out,
            );
        }
        0 => {
            PredictorInverseTransform_C(transform, row_start, row_end, in_0, out);
            if row_end != (*transform).ysize_ {
                memcpy(
                    out.offset(-(width as isize)) as *mut ::core::ffi::c_void,
                    out.offset(((row_end - row_start - 1 as ::core::ffi::c_int) * width) as isize)
                        as *const ::core::ffi::c_void,
                    (width as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                );
            }
        }
        1 => {
            ColorSpaceInverseTransform_C(transform, row_start, row_end, in_0, out);
        }
        3 => {
            if in_0 == out as *const uint32_t && (*transform).bits_ > 0 as ::core::ffi::c_int {
                let out_stride: ::core::ffi::c_int = (row_end - row_start) * width;
                let in_stride: ::core::ffi::c_int = ((row_end - row_start) as uint32_t)
                    .wrapping_mul(VP8LSubSampleSize(
                        (*transform).xsize_ as uint32_t,
                        (*transform).bits_ as uint32_t,
                    )) as ::core::ffi::c_int;
                let src: *mut uint32_t = out
                    .offset(out_stride as isize)
                    .offset(-(in_stride as isize));
                memmove(
                    src as *mut ::core::ffi::c_void,
                    out as *const ::core::ffi::c_void,
                    (in_stride as size_t)
                        .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                );
                ColorIndexInverseTransform_C(transform, row_start, row_end, src, out);
            } else {
                ColorIndexInverseTransform_C(transform, row_start, row_end, in_0, out);
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn is_big_endian() -> ::core::ffi::c_int {
    static mut tmp: C2RustUnnamed = C2RustUnnamed {
        w: 1 as ::core::ffi::c_int as uint16_t,
    };
    return (tmp.b[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        != 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LConvertBGRAToRGB_C(
    mut src: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut dst: *mut uint8_t,
) {
    let src_end: *const uint32_t = src.offset(num_pixels as isize);
    while src < src_end {
        let fresh8 = src;
        src = src.offset(1);
        let argb: uint32_t = *fresh8;
        let fresh9 = dst;
        dst = dst.offset(1);
        *fresh9 = (argb >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        let fresh10 = dst;
        dst = dst.offset(1);
        *fresh10 = (argb >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        let fresh11 = dst;
        dst = dst.offset(1);
        *fresh11 = (argb >> 0 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LConvertBGRAToRGBA_C(
    mut src: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut dst: *mut uint8_t,
) {
    let src_end: *const uint32_t = src.offset(num_pixels as isize);
    while src < src_end {
        let fresh12 = src;
        src = src.offset(1);
        let argb: uint32_t = *fresh12;
        let fresh13 = dst;
        dst = dst.offset(1);
        *fresh13 = (argb >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        let fresh14 = dst;
        dst = dst.offset(1);
        *fresh14 = (argb >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        let fresh15 = dst;
        dst = dst.offset(1);
        *fresh15 = (argb >> 0 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        let fresh16 = dst;
        dst = dst.offset(1);
        *fresh16 = (argb >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LConvertBGRAToRGBA4444_C(
    mut src: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut dst: *mut uint8_t,
) {
    let src_end: *const uint32_t = src.offset(num_pixels as isize);
    while src < src_end {
        let fresh17 = src;
        src = src.offset(1);
        let argb: uint32_t = *fresh17;
        let rg: uint8_t = (argb >> 16 as ::core::ffi::c_int & 0xf0 as uint32_t
            | argb >> 12 as ::core::ffi::c_int & 0xf as uint32_t)
            as uint8_t;
        let ba: uint8_t = (argb >> 0 as ::core::ffi::c_int & 0xf0 as uint32_t
            | argb >> 28 as ::core::ffi::c_int & 0xf as uint32_t)
            as uint8_t;
        let fresh18 = dst;
        dst = dst.offset(1);
        *fresh18 = rg;
        let fresh19 = dst;
        dst = dst.offset(1);
        *fresh19 = ba;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LConvertBGRAToRGB565_C(
    mut src: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut dst: *mut uint8_t,
) {
    let src_end: *const uint32_t = src.offset(num_pixels as isize);
    while src < src_end {
        let fresh20 = src;
        src = src.offset(1);
        let argb: uint32_t = *fresh20;
        let rg: uint8_t = (argb >> 16 as ::core::ffi::c_int & 0xf8 as uint32_t
            | argb >> 13 as ::core::ffi::c_int & 0x7 as uint32_t)
            as uint8_t;
        let gb: uint8_t = (argb >> 5 as ::core::ffi::c_int & 0xe0 as uint32_t
            | argb >> 3 as ::core::ffi::c_int & 0x1f as uint32_t)
            as uint8_t;
        let fresh21 = dst;
        dst = dst.offset(1);
        *fresh21 = rg;
        let fresh22 = dst;
        dst = dst.offset(1);
        *fresh22 = gb;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LConvertBGRAToBGR_C(
    mut src: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut dst: *mut uint8_t,
) {
    let src_end: *const uint32_t = src.offset(num_pixels as isize);
    while src < src_end {
        let fresh23 = src;
        src = src.offset(1);
        let argb: uint32_t = *fresh23;
        let fresh24 = dst;
        dst = dst.offset(1);
        *fresh24 = (argb >> 0 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        let fresh25 = dst;
        dst = dst.offset(1);
        *fresh25 = (argb >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        let fresh26 = dst;
        dst = dst.offset(1);
        *fresh26 = (argb >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    }
}
unsafe extern "C" fn CopyOrSwap(
    mut src: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut swap_on_big_endian: ::core::ffi::c_int,
) {
    if is_big_endian() == swap_on_big_endian {
        let src_end: *const uint32_t = src.offset(num_pixels as isize);
        while src < src_end {
            let fresh5 = src;
            src = src.offset(1);
            let argb: uint32_t = *fresh5;
            WebPUint32ToMem(dst, BSwap32(argb));
            dst = dst.offset(::core::mem::size_of::<uint32_t>() as usize as isize);
        }
    } else {
        memcpy(
            dst as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            (num_pixels as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LConvertFromBGRA(
    in_data: *const uint32_t,
    mut num_pixels: ::core::ffi::c_int,
    mut out_colorspace: WEBP_CSP_MODE,
    rgba: *mut uint8_t,
) {
    match out_colorspace as ::core::ffi::c_uint {
        0 => {
            VP8LConvertBGRAToRGB.expect("non-null function pointer")(in_data, num_pixels, rgba);
        }
        1 => {
            VP8LConvertBGRAToRGBA.expect("non-null function pointer")(in_data, num_pixels, rgba);
        }
        7 => {
            VP8LConvertBGRAToRGBA.expect("non-null function pointer")(in_data, num_pixels, rgba);
            WebPApplyAlphaMultiply.expect("non-null function pointer")(
                rgba,
                0 as ::core::ffi::c_int,
                num_pixels,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        }
        2 => {
            VP8LConvertBGRAToBGR.expect("non-null function pointer")(in_data, num_pixels, rgba);
        }
        3 => {
            CopyOrSwap(in_data, num_pixels, rgba, 1 as ::core::ffi::c_int);
        }
        8 => {
            CopyOrSwap(in_data, num_pixels, rgba, 1 as ::core::ffi::c_int);
            WebPApplyAlphaMultiply.expect("non-null function pointer")(
                rgba,
                0 as ::core::ffi::c_int,
                num_pixels,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        }
        4 => {
            CopyOrSwap(in_data, num_pixels, rgba, 0 as ::core::ffi::c_int);
        }
        9 => {
            CopyOrSwap(in_data, num_pixels, rgba, 0 as ::core::ffi::c_int);
            WebPApplyAlphaMultiply.expect("non-null function pointer")(
                rgba,
                1 as ::core::ffi::c_int,
                num_pixels,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        }
        5 => {
            VP8LConvertBGRAToRGBA4444.expect("non-null function pointer")(
                in_data, num_pixels, rgba,
            );
        }
        10 => {
            VP8LConvertBGRAToRGBA4444.expect("non-null function pointer")(
                in_data, num_pixels, rgba,
            );
            WebPApplyAlphaMultiply4444.expect("non-null function pointer")(
                rgba,
                num_pixels,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        }
        6 => {
            VP8LConvertBGRAToRGB565.expect("non-null function pointer")(in_data, num_pixels, rgba);
        }
        _ => {
            '_c2rust_label: {
                __assert_fail(
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    567 as ::core::ffi::c_uint,
                    b"void VP8LConvertFromBGRA(const uint32_t *const, int, WEBP_CSP_MODE, uint8_t *const)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            };
        }
    };
}
#[no_mangle]
pub static mut VP8LAddGreenToBlueAndRed: VP8LProcessDecBlueAndRedFunc = None;
#[no_mangle]
pub static mut VP8LPredictorsAdd: [VP8LPredictorAddSubFunc; 16] = [None; 16];
#[no_mangle]
pub static mut VP8LPredictors: [VP8LPredictorFunc; 16] = [None; 16];
#[no_mangle]
pub static mut VP8LPredictorsAdd_C: [VP8LPredictorAddSubFunc; 16] = [None; 16];
#[no_mangle]
pub static mut VP8LTransformColorInverse: VP8LTransformColorInverseFunc = None;
#[no_mangle]
pub static mut VP8LConvertBGRAToRGB: VP8LConvertFunc = None;
#[no_mangle]
pub static mut VP8LConvertBGRAToRGBA: VP8LConvertFunc = None;
#[no_mangle]
pub static mut VP8LConvertBGRAToRGBA4444: VP8LConvertFunc = None;
#[no_mangle]
pub static mut VP8LConvertBGRAToRGB565: VP8LConvertFunc = None;
#[no_mangle]
pub static mut VP8LConvertBGRAToBGR: VP8LConvertFunc = None;
#[no_mangle]
pub static mut VP8LMapColor32b: VP8LMapARGBFunc = None;
#[no_mangle]
pub static mut VP8LMapColor8b: VP8LMapAlphaFunc = None;
unsafe extern "C" fn VP8LDspInit_body() {
    VP8LPredictors[0 as ::core::ffi::c_int as usize] = Some(
        VP8LPredictor0_C as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    ) as VP8LPredictorFunc;
    VP8LPredictors[1 as ::core::ffi::c_int as usize] = Some(
        VP8LPredictor1_C as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    ) as VP8LPredictorFunc;
    VP8LPredictors[2 as ::core::ffi::c_int as usize] = Some(
        VP8LPredictor2_C as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    ) as VP8LPredictorFunc;
    VP8LPredictors[3 as ::core::ffi::c_int as usize] = Some(
        VP8LPredictor3_C as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    ) as VP8LPredictorFunc;
    VP8LPredictors[4 as ::core::ffi::c_int as usize] = Some(
        VP8LPredictor4_C as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    ) as VP8LPredictorFunc;
    VP8LPredictors[5 as ::core::ffi::c_int as usize] = Some(
        VP8LPredictor5_C as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    ) as VP8LPredictorFunc;
    VP8LPredictors[6 as ::core::ffi::c_int as usize] = Some(
        VP8LPredictor6_C as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    ) as VP8LPredictorFunc;
    VP8LPredictors[7 as ::core::ffi::c_int as usize] = Some(
        VP8LPredictor7_C as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    ) as VP8LPredictorFunc;
    VP8LPredictors[8 as ::core::ffi::c_int as usize] = Some(
        VP8LPredictor8_C as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    ) as VP8LPredictorFunc;
    VP8LPredictors[9 as ::core::ffi::c_int as usize] = Some(
        VP8LPredictor9_C as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    ) as VP8LPredictorFunc;
    VP8LPredictors[10 as ::core::ffi::c_int as usize] = Some(
        VP8LPredictor10_C as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    ) as VP8LPredictorFunc;
    VP8LPredictors[11 as ::core::ffi::c_int as usize] = Some(
        VP8LPredictor11_C as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    ) as VP8LPredictorFunc;
    VP8LPredictors[12 as ::core::ffi::c_int as usize] = Some(
        VP8LPredictor12_C as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    ) as VP8LPredictorFunc;
    VP8LPredictors[13 as ::core::ffi::c_int as usize] = Some(
        VP8LPredictor13_C as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    ) as VP8LPredictorFunc;
    VP8LPredictors[14 as ::core::ffi::c_int as usize] = Some(
        VP8LPredictor0_C as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    ) as VP8LPredictorFunc;
    VP8LPredictors[15 as ::core::ffi::c_int as usize] = Some(
        VP8LPredictor0_C as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    ) as VP8LPredictorFunc;
    VP8LPredictorsAdd[0 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd0_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd[1 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd1_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd[2 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd2_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd[3 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd3_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd[4 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd4_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd[5 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd5_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd[6 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd6_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd[7 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd7_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd[8 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd8_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd[9 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd9_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd[10 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd10_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd[11 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd11_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd[12 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd12_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd[13 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd13_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd[14 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd0_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd[15 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd0_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd_C[0 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd0_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd_C[1 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd1_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd_C[2 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd2_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd_C[3 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd3_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd_C[4 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd4_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd_C[5 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd5_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd_C[6 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd6_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd_C[7 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd7_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd_C[8 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd8_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd_C[9 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd9_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd_C[10 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd10_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd_C[11 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd11_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd_C[12 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd12_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd_C[13 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd13_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd_C[14 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd0_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LPredictorsAdd_C[15 as ::core::ffi::c_int as usize] = Some(
        PredictorAdd0_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LPredictorAddSubFunc;
    VP8LAddGreenToBlueAndRed = Some(
        VP8LAddGreenToBlueAndRed_C
            as unsafe extern "C" fn(*const uint32_t, ::core::ffi::c_int, *mut uint32_t) -> (),
    ) as VP8LProcessDecBlueAndRedFunc;
    VP8LTransformColorInverse = Some(
        VP8LTransformColorInverse_C
            as unsafe extern "C" fn(
                *const VP8LMultipliers,
                *const uint32_t,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    ) as VP8LTransformColorInverseFunc;
    VP8LConvertBGRAToRGBA = Some(
        VP8LConvertBGRAToRGBA_C
            as unsafe extern "C" fn(*const uint32_t, ::core::ffi::c_int, *mut uint8_t) -> (),
    ) as VP8LConvertFunc;
    VP8LConvertBGRAToRGB = Some(
        VP8LConvertBGRAToRGB_C
            as unsafe extern "C" fn(*const uint32_t, ::core::ffi::c_int, *mut uint8_t) -> (),
    ) as VP8LConvertFunc;
    VP8LConvertBGRAToBGR = Some(
        VP8LConvertBGRAToBGR_C
            as unsafe extern "C" fn(*const uint32_t, ::core::ffi::c_int, *mut uint8_t) -> (),
    ) as VP8LConvertFunc;
    VP8LConvertBGRAToRGBA4444 = Some(
        VP8LConvertBGRAToRGBA4444_C
            as unsafe extern "C" fn(*const uint32_t, ::core::ffi::c_int, *mut uint8_t) -> (),
    ) as VP8LConvertFunc;
    VP8LConvertBGRAToRGB565 = Some(
        VP8LConvertBGRAToRGB565_C
            as unsafe extern "C" fn(*const uint32_t, ::core::ffi::c_int, *mut uint8_t) -> (),
    ) as VP8LConvertFunc;
    VP8LMapColor32b = Some(
        MapARGB_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                *mut uint32_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    ) as VP8LMapARGBFunc;
    VP8LMapColor8b = Some(
        MapAlpha_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint32_t,
                *mut uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    ) as VP8LMapAlphaFunc;
    if VP8GetCPUInfo.is_some() {
        if VP8GetCPUInfo.expect("non-null function pointer")(kSSE2) != 0 {
            VP8LDspInitSSE2();
        }
    }
    '_c2rust_label: {
        if VP8LAddGreenToBlueAndRed.is_some() {
        } else {
            __assert_fail(
                b"VP8LAddGreenToBlueAndRed != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                669 as ::core::ffi::c_uint,
                b"void VP8LDspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if VP8LTransformColorInverse.is_some() {
        } else {
            __assert_fail(
                b"VP8LTransformColorInverse != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                670 as ::core::ffi::c_uint,
                b"void VP8LDspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if VP8LConvertBGRAToRGBA.is_some() {
        } else {
            __assert_fail(
                b"VP8LConvertBGRAToRGBA != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                671 as ::core::ffi::c_uint,
                b"void VP8LDspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if VP8LConvertBGRAToRGB.is_some() {
        } else {
            __assert_fail(
                b"VP8LConvertBGRAToRGB != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                672 as ::core::ffi::c_uint,
                b"void VP8LDspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if VP8LConvertBGRAToBGR.is_some() {
        } else {
            __assert_fail(
                b"VP8LConvertBGRAToBGR != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                673 as ::core::ffi::c_uint,
                b"void VP8LDspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if VP8LConvertBGRAToRGBA4444.is_some() {
        } else {
            __assert_fail(
                b"VP8LConvertBGRAToRGBA4444 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                674 as ::core::ffi::c_uint,
                b"void VP8LDspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_5: {
        if VP8LConvertBGRAToRGB565.is_some() {
        } else {
            __assert_fail(
                b"VP8LConvertBGRAToRGB565 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                675 as ::core::ffi::c_uint,
                b"void VP8LDspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_6: {
        if VP8LMapColor32b.is_some() {
        } else {
            __assert_fail(
                b"VP8LMapColor32b != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                676 as ::core::ffi::c_uint,
                b"void VP8LDspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_7: {
        if VP8LMapColor8b.is_some() {
        } else {
            __assert_fail(
                b"VP8LMapColor8b != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/lossless.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                677 as ::core::ffi::c_uint,
                b"void VP8LDspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LDspInit() {
    static mut VP8LDspInit_body_last_cpuinfo_used: VP8CPUInfo = None;
    if !(VP8LDspInit_body_last_cpuinfo_used == VP8GetCPUInfo) {
        VP8LDspInit_body();
        ::core::ptr::write_volatile(
            &mut VP8LDspInit_body_last_cpuinfo_used as *mut VP8CPUInfo,
            VP8GetCPUInfo,
        );
    }
}
#[inline]
unsafe extern "C" fn VP8GetARGBIndex(mut idx: uint32_t) -> uint32_t {
    return idx >> 8 as ::core::ffi::c_int & 0xff as uint32_t;
}
#[inline]
unsafe extern "C" fn VP8GetAlphaIndex(mut idx: uint8_t) -> uint8_t {
    return idx;
}
#[inline]
unsafe extern "C" fn VP8GetARGBValue(mut val: uint32_t) -> uint32_t {
    return val;
}
#[inline]
unsafe extern "C" fn VP8GetAlphaValue(mut val: uint32_t) -> uint8_t {
    return (val >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
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
unsafe extern "C" fn VP8LAddPixels(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let alpha_and_green: uint32_t =
        (a & 0xff00ff00 as uint32_t).wrapping_add(b & 0xff00ff00 as uint32_t);
    let red_and_blue: uint32_t = (a & 0xff00ff as uint32_t).wrapping_add(b & 0xff00ff as uint32_t);
    return alpha_and_green & 0xff00ff00 as uint32_t | red_and_blue & 0xff00ff as uint32_t;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
