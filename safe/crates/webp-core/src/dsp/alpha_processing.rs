extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    static mut VP8GetCPUInfo: VP8CPUInfo;
    fn WebPInitAlphaProcessingSSE2();
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFIX: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const HALF: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << MFIX >> 1 as ::core::ffi::c_int;
pub const KINV_255: ::core::ffi::c_uint =
    ((1 as ::core::ffi::c_uint) << MFIX).wrapping_div(255 as ::core::ffi::c_uint);
unsafe extern "C" fn Mult(mut x: uint8_t, mut mult: uint32_t) -> uint32_t {
    let v: uint32_t = (x as uint32_t)
        .wrapping_mul(mult)
        .wrapping_add(HALF as uint32_t)
        >> MFIX;
    '_c2rust_label: {
        if v <= 255 as uint32_t {
        } else {
            __assert_fail(
                b"v <= 255\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/alpha_processing.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                31 as ::core::ffi::c_uint,
                b"uint32_t Mult(uint8_t, uint32_t)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    return v;
}
#[inline]
unsafe extern "C" fn GetScale(mut a: uint32_t, mut inverse: ::core::ffi::c_int) -> uint32_t {
    return if inverse != 0 {
        ((255 as uint32_t) << MFIX).wrapping_div(a)
    } else {
        a.wrapping_mul(KINV_255 as uint32_t)
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPMultARGBRow_C(
    ptr: *mut uint32_t,
    mut width: ::core::ffi::c_int,
    mut inverse: ::core::ffi::c_int,
) {
    let mut x: ::core::ffi::c_int = 0;
    x = 0 as ::core::ffi::c_int;
    while x < width {
        let argb: uint32_t = *ptr.offset(x as isize);
        if argb < 0xff000000 as uint32_t {
            if argb <= 0xffffff as uint32_t {
                *ptr.offset(x as isize) = 0 as uint32_t;
            } else {
                let alpha: uint32_t = argb >> 24 as ::core::ffi::c_int & 0xff as uint32_t;
                let scale: uint32_t = GetScale(alpha, inverse) as uint32_t;
                let mut out: uint32_t = argb & 0xff000000 as uint32_t;
                out |= Mult((argb >> 0 as ::core::ffi::c_int) as uint8_t, scale)
                    << 0 as ::core::ffi::c_int;
                out |= Mult((argb >> 8 as ::core::ffi::c_int) as uint8_t, scale)
                    << 8 as ::core::ffi::c_int;
                out |= Mult((argb >> 16 as ::core::ffi::c_int) as uint8_t, scale)
                    << 16 as ::core::ffi::c_int;
                *ptr.offset(x as isize) = out;
            }
        }
        x += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPMultRow_C(
    ptr: *mut uint8_t,
    alpha: *const uint8_t,
    mut width: ::core::ffi::c_int,
    mut inverse: ::core::ffi::c_int,
) {
    let mut x: ::core::ffi::c_int = 0;
    x = 0 as ::core::ffi::c_int;
    while x < width {
        let a: uint32_t = *alpha.offset(x as isize) as uint32_t;
        if a != 255 as uint32_t {
            if a == 0 as uint32_t {
                *ptr.offset(x as isize) = 0 as uint8_t;
            } else {
                let scale: uint32_t = GetScale(a, inverse) as uint32_t;
                *ptr.offset(x as isize) = Mult(*ptr.offset(x as isize), scale) as uint8_t;
            }
        }
        x += 1;
    }
}
#[no_mangle]
pub static mut WebPMultARGBRow: Option<
    unsafe extern "C" fn(*mut uint32_t, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
> = None;
#[no_mangle]
pub static mut WebPMultRow: Option<
    unsafe extern "C" fn(
        *mut uint8_t,
        *const uint8_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub unsafe extern "C" fn WebPMultARGBRows(
    mut ptr: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut num_rows: ::core::ffi::c_int,
    mut inverse: ::core::ffi::c_int,
) {
    let mut n: ::core::ffi::c_int = 0;
    n = 0 as ::core::ffi::c_int;
    while n < num_rows {
        WebPMultARGBRow.expect("non-null function pointer")(ptr as *mut uint32_t, width, inverse);
        ptr = ptr.offset(stride as isize);
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPMultRows(
    mut ptr: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
    mut alpha: *const uint8_t,
    mut alpha_stride: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut num_rows: ::core::ffi::c_int,
    mut inverse: ::core::ffi::c_int,
) {
    let mut n: ::core::ffi::c_int = 0;
    n = 0 as ::core::ffi::c_int;
    while n < num_rows {
        WebPMultRow.expect("non-null function pointer")(ptr, alpha, width, inverse);
        ptr = ptr.offset(stride as isize);
        alpha = alpha.offset(alpha_stride as isize);
        n += 1;
    }
}
unsafe extern "C" fn ApplyAlphaMultiply_C(
    mut rgba: *mut uint8_t,
    mut alpha_first: ::core::ffi::c_int,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
) {
    loop {
        let fresh3 = h;
        h = h - 1;
        if !(fresh3 > 0 as ::core::ffi::c_int) {
            break;
        }
        let rgb: *mut uint8_t = rgba.offset(
            (if alpha_first != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as isize,
        );
        let alpha: *const uint8_t = rgba.offset(
            (if alpha_first != 0 {
                0 as ::core::ffi::c_int
            } else {
                3 as ::core::ffi::c_int
            }) as isize,
        );
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < w {
            let a: uint32_t = *alpha.offset((4 as ::core::ffi::c_int * i) as isize) as uint32_t;
            if a != 0xff as uint32_t {
                let mult: uint32_t = a.wrapping_mul(32897 as uint32_t);
                *rgb.offset((4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as isize) =
                    ((*rgb.offset((4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as isize)
                        as uint32_t)
                        .wrapping_mul(mult)
                        >> 23 as ::core::ffi::c_int) as uint8_t;
                *rgb.offset((4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as isize) =
                    ((*rgb.offset((4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as isize)
                        as uint32_t)
                        .wrapping_mul(mult)
                        >> 23 as ::core::ffi::c_int) as uint8_t;
                *rgb.offset((4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as isize) =
                    ((*rgb.offset((4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as isize)
                        as uint32_t)
                        .wrapping_mul(mult)
                        >> 23 as ::core::ffi::c_int) as uint8_t;
            }
            i += 1;
        }
        rgba = rgba.offset(stride as isize);
    }
}
#[inline]
unsafe extern "C" fn dither_hi(mut x: uint8_t) -> uint8_t {
    return (x as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
        | x as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn dither_lo(mut x: uint8_t) -> uint8_t {
    return (x as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
        | (x as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn multiply(mut x: uint8_t, mut m: uint32_t) -> uint8_t {
    return ((x as uint32_t).wrapping_mul(m) >> 16 as ::core::ffi::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn ApplyAlphaMultiply4444_C(
    mut rgba4444: *mut uint8_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
    mut rg_byte_pos: ::core::ffi::c_int,
) {
    loop {
        let fresh4 = h;
        h = h - 1;
        if !(fresh4 > 0 as ::core::ffi::c_int) {
            break;
        }
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < w {
            let rg: uint32_t =
                *rgba4444.offset((2 as ::core::ffi::c_int * i + rg_byte_pos) as isize) as uint32_t;
            let ba: uint32_t = *rgba4444.offset(
                (2 as ::core::ffi::c_int * i + (rg_byte_pos ^ 1 as ::core::ffi::c_int)) as isize,
            ) as uint32_t;
            let a: uint8_t = (ba & 0xf as uint32_t) as uint8_t;
            let mult: uint32_t =
                (a as ::core::ffi::c_int * 0x1111 as ::core::ffi::c_int) as uint32_t;
            let r: uint8_t = multiply(dither_hi(rg as uint8_t), mult) as uint8_t;
            let g: uint8_t = multiply(dither_lo(rg as uint8_t), mult) as uint8_t;
            let b: uint8_t = multiply(dither_hi(ba as uint8_t), mult) as uint8_t;
            *rgba4444.offset((2 as ::core::ffi::c_int * i + rg_byte_pos) as isize) =
                (r as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                    | g as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                        & 0xf as ::core::ffi::c_int) as uint8_t;
            *rgba4444.offset(
                (2 as ::core::ffi::c_int * i + (rg_byte_pos ^ 1 as ::core::ffi::c_int)) as isize,
            ) = (b as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int | a as ::core::ffi::c_int)
                as uint8_t;
            i += 1;
        }
        rgba4444 = rgba4444.offset(stride as isize);
    }
}
unsafe extern "C" fn ApplyAlphaMultiply_16b_C(
    mut rgba4444: *mut uint8_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
) {
    ApplyAlphaMultiply4444_C(rgba4444, w, h, stride, 0 as ::core::ffi::c_int);
}
unsafe extern "C" fn DispatchAlpha_C(
    mut alpha: *const uint8_t,
    mut alpha_stride: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut alpha_mask: uint32_t = 0xff as uint32_t;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < height {
        i = 0 as ::core::ffi::c_int;
        while i < width {
            let alpha_value: uint32_t = *alpha.offset(i as isize) as uint32_t;
            *dst.offset((4 as ::core::ffi::c_int * i) as isize) = alpha_value as uint8_t;
            alpha_mask &= alpha_value;
            i += 1;
        }
        alpha = alpha.offset(alpha_stride as isize);
        dst = dst.offset(dst_stride as isize);
        j += 1;
    }
    return (alpha_mask != 0xff as uint32_t) as ::core::ffi::c_int;
}
unsafe extern "C" fn DispatchAlphaToGreen_C(
    mut alpha: *const uint8_t,
    mut alpha_stride: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut dst: *mut uint32_t,
    mut dst_stride: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < height {
        i = 0 as ::core::ffi::c_int;
        while i < width {
            *dst.offset(i as isize) = ((*alpha.offset(i as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int) as uint32_t;
            i += 1;
        }
        alpha = alpha.offset(alpha_stride as isize);
        dst = dst.offset(dst_stride as isize);
        j += 1;
    }
}
unsafe extern "C" fn ExtractAlpha_C(
    mut argb: *const uint8_t,
    mut argb_stride: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut alpha: *mut uint8_t,
    mut alpha_stride: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut alpha_mask: uint8_t = 0xff as uint8_t;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < height {
        i = 0 as ::core::ffi::c_int;
        while i < width {
            let alpha_value: uint8_t = *argb.offset((4 as ::core::ffi::c_int * i) as isize);
            *alpha.offset(i as isize) = alpha_value;
            alpha_mask =
                (alpha_mask as ::core::ffi::c_int & alpha_value as ::core::ffi::c_int) as uint8_t;
            i += 1;
        }
        argb = argb.offset(argb_stride as isize);
        alpha = alpha.offset(alpha_stride as isize);
        j += 1;
    }
    return (alpha_mask as ::core::ffi::c_int == 0xff as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn ExtractGreen_C(
    mut argb: *const uint32_t,
    mut alpha: *mut uint8_t,
    mut size: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < size {
        *alpha.offset(i as isize) =
            (*argb.offset(i as isize) >> 8 as ::core::ffi::c_int) as uint8_t;
        i += 1;
    }
}
unsafe extern "C" fn HasAlpha8b_C(
    mut src: *const uint8_t,
    mut length: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    loop {
        let fresh1 = length;
        length = length - 1;
        if !(fresh1 > 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh2 = src;
        src = src.offset(1);
        if *fresh2 as ::core::ffi::c_int != 0xff as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn HasAlpha32b_C(
    mut src: *const uint8_t,
    mut length: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut x: ::core::ffi::c_int = 0;
    x = 0 as ::core::ffi::c_int;
    loop {
        let fresh0 = length;
        length = length - 1;
        if !(fresh0 > 0 as ::core::ffi::c_int) {
            break;
        }
        if *src.offset(x as isize) as ::core::ffi::c_int != 0xff as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        x += 4 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn AlphaReplace_C(
    mut src: *mut uint32_t,
    mut length: ::core::ffi::c_int,
    mut color: uint32_t,
) {
    let mut x: ::core::ffi::c_int = 0;
    x = 0 as ::core::ffi::c_int;
    while x < length {
        if *src.offset(x as isize) >> 24 as ::core::ffi::c_int == 0 as uint32_t {
            *src.offset(x as isize) = color;
        }
        x += 1;
    }
}
#[inline]
unsafe extern "C" fn MakeARGB32(
    mut a: ::core::ffi::c_int,
    mut r: ::core::ffi::c_int,
    mut g: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
) -> uint32_t {
    return (a as uint32_t) << 24 as ::core::ffi::c_int
        | (r << 16 as ::core::ffi::c_int) as uint32_t
        | (g << 8 as ::core::ffi::c_int) as uint32_t
        | b as uint32_t;
}
unsafe extern "C" fn PackRGB_C(
    mut r: *const uint8_t,
    mut g: *const uint8_t,
    mut b: *const uint8_t,
    mut len: ::core::ffi::c_int,
    mut step: ::core::ffi::c_int,
    mut out: *mut uint32_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < len {
        *out.offset(i as isize) = MakeARGB32(
            0xff as ::core::ffi::c_int,
            *r.offset(offset as isize) as ::core::ffi::c_int,
            *g.offset(offset as isize) as ::core::ffi::c_int,
            *b.offset(offset as isize) as ::core::ffi::c_int,
        );
        offset += step;
        i += 1;
    }
}
#[no_mangle]
pub static mut WebPApplyAlphaMultiply: Option<
    unsafe extern "C" fn(
        *mut uint8_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut WebPApplyAlphaMultiply4444: Option<
    unsafe extern "C" fn(
        *mut uint8_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut WebPDispatchAlpha: Option<
    unsafe extern "C" fn(
        *const uint8_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut uint8_t,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
> = None;
#[no_mangle]
pub static mut WebPDispatchAlphaToGreen: Option<
    unsafe extern "C" fn(
        *const uint8_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut uint32_t,
        ::core::ffi::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut WebPExtractAlpha: Option<
    unsafe extern "C" fn(
        *const uint8_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut uint8_t,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
> = None;
#[no_mangle]
pub static mut WebPExtractGreen: Option<
    unsafe extern "C" fn(*const uint32_t, *mut uint8_t, ::core::ffi::c_int) -> (),
> = None;
#[no_mangle]
pub static mut WebPPackRGB: Option<
    unsafe extern "C" fn(
        *const uint8_t,
        *const uint8_t,
        *const uint8_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut uint32_t,
    ) -> (),
> = None;
#[no_mangle]
pub static mut WebPHasAlpha8b: Option<
    unsafe extern "C" fn(*const uint8_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
> = None;
#[no_mangle]
pub static mut WebPHasAlpha32b: Option<
    unsafe extern "C" fn(*const uint8_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
> = None;
#[no_mangle]
pub static mut WebPAlphaReplace: Option<
    unsafe extern "C" fn(*mut uint32_t, ::core::ffi::c_int, uint32_t) -> (),
> = None;
unsafe extern "C" fn WebPInitAlphaProcessing_body() {
    WebPMultARGBRow = Some(
        WebPMultARGBRow_C
            as unsafe extern "C" fn(*mut uint32_t, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut uint32_t, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
        >;
    WebPMultRow = Some(
        WebPMultRow_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut uint8_t,
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >;
    WebPApplyAlphaMultiply4444 = Some(
        ApplyAlphaMultiply_16b_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >;
    WebPPackRGB = Some(
        PackRGB_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut uint32_t,
            ) -> (),
        >;
    WebPApplyAlphaMultiply = Some(
        ApplyAlphaMultiply_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >;
    WebPDispatchAlpha = Some(
        DispatchAlpha_C
            as unsafe extern "C" fn(
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >;
    WebPDispatchAlphaToGreen = Some(
        DispatchAlphaToGreen_C
            as unsafe extern "C" fn(
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut uint32_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut uint32_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    WebPExtractAlpha = Some(
        ExtractAlpha_C
            as unsafe extern "C" fn(
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >;
    WebPExtractGreen = Some(
        ExtractGreen_C
            as unsafe extern "C" fn(*const uint32_t, *mut uint8_t, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*const uint32_t, *mut uint8_t, ::core::ffi::c_int) -> ()>;
    WebPHasAlpha8b = Some(
        HasAlpha8b_C
            as unsafe extern "C" fn(*const uint8_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
    )
        as Option<unsafe extern "C" fn(*const uint8_t, ::core::ffi::c_int) -> ::core::ffi::c_int>;
    WebPHasAlpha32b = Some(
        HasAlpha32b_C
            as unsafe extern "C" fn(*const uint8_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
    )
        as Option<unsafe extern "C" fn(*const uint8_t, ::core::ffi::c_int) -> ::core::ffi::c_int>;
    WebPAlphaReplace = Some(
        AlphaReplace_C as unsafe extern "C" fn(*mut uint32_t, ::core::ffi::c_int, uint32_t) -> (),
    )
        as Option<unsafe extern "C" fn(*mut uint32_t, ::core::ffi::c_int, uint32_t) -> ()>;
    if VP8GetCPUInfo.is_some() {
        if VP8GetCPUInfo.expect("non-null function pointer")(kSSE2) != 0 {
            WebPInitAlphaProcessingSSE2();
        }
    }
    '_c2rust_label: {
        if WebPMultARGBRow.is_some() {
        } else {
            __assert_fail(
                b"WebPMultARGBRow != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/alpha_processing.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                481 as ::core::ffi::c_uint,
                b"void WebPInitAlphaProcessing_body(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if WebPMultRow.is_some() {
        } else {
            __assert_fail(
                b"WebPMultRow != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/alpha_processing.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                482 as ::core::ffi::c_uint,
                b"void WebPInitAlphaProcessing_body(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if WebPApplyAlphaMultiply.is_some() {
        } else {
            __assert_fail(
                b"WebPApplyAlphaMultiply != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/alpha_processing.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                483 as ::core::ffi::c_uint,
                b"void WebPInitAlphaProcessing_body(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if WebPApplyAlphaMultiply4444.is_some() {
        } else {
            __assert_fail(
                b"WebPApplyAlphaMultiply4444 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/alpha_processing.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                484 as ::core::ffi::c_uint,
                b"void WebPInitAlphaProcessing_body(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if WebPDispatchAlpha.is_some() {
        } else {
            __assert_fail(
                b"WebPDispatchAlpha != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/alpha_processing.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                485 as ::core::ffi::c_uint,
                b"void WebPInitAlphaProcessing_body(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if WebPDispatchAlphaToGreen.is_some() {
        } else {
            __assert_fail(
                b"WebPDispatchAlphaToGreen != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/alpha_processing.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                486 as ::core::ffi::c_uint,
                b"void WebPInitAlphaProcessing_body(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_5: {
        if WebPExtractAlpha.is_some() {
        } else {
            __assert_fail(
                b"WebPExtractAlpha != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/alpha_processing.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                487 as ::core::ffi::c_uint,
                b"void WebPInitAlphaProcessing_body(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_6: {
        if WebPExtractGreen.is_some() {
        } else {
            __assert_fail(
                b"WebPExtractGreen != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/alpha_processing.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                488 as ::core::ffi::c_uint,
                b"void WebPInitAlphaProcessing_body(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_7: {
        if WebPPackRGB.is_some() {
        } else {
            __assert_fail(
                b"WebPPackRGB != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/alpha_processing.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                492 as ::core::ffi::c_uint,
                b"void WebPInitAlphaProcessing_body(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_8: {
        if WebPHasAlpha8b.is_some() {
        } else {
            __assert_fail(
                b"WebPHasAlpha8b != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/alpha_processing.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                493 as ::core::ffi::c_uint,
                b"void WebPInitAlphaProcessing_body(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_9: {
        if WebPHasAlpha32b.is_some() {
        } else {
            __assert_fail(
                b"WebPHasAlpha32b != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/alpha_processing.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                494 as ::core::ffi::c_uint,
                b"void WebPInitAlphaProcessing_body(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_10: {
        if WebPAlphaReplace.is_some() {
        } else {
            __assert_fail(
                b"WebPAlphaReplace != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/alpha_processing.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                495 as ::core::ffi::c_uint,
                b"void WebPInitAlphaProcessing_body(void)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPInitAlphaProcessing() {
    static mut WebPInitAlphaProcessing_body_last_cpuinfo_used: VP8CPUInfo = None;
    if !(WebPInitAlphaProcessing_body_last_cpuinfo_used == VP8GetCPUInfo) {
        WebPInitAlphaProcessing_body();
        ::core::ptr::write_volatile(
            &mut WebPInitAlphaProcessing_body_last_cpuinfo_used as *mut VP8CPUInfo,
            VP8GetCPUInfo,
        );
    }
}
