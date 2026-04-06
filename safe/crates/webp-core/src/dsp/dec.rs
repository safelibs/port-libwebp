extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    static VP8ksclip1: *const int8_t;
    static VP8ksclip2: *const int8_t;
    static VP8kclip1: *const uint8_t;
    static VP8kabs0: *const uint8_t;
    fn VP8InitClipTables();
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
    static mut VP8GetCPUInfo: VP8CPUInfo;
    fn VP8DspInitSSE2();
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint32_t = u32;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
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
pub type VP8DecIdct = Option<unsafe extern "C" fn(*const int16_t, *mut uint8_t) -> ()>;
pub type VP8DecIdct2 =
    Option<unsafe extern "C" fn(*const int16_t, *mut uint8_t, ::core::ffi::c_int) -> ()>;
pub type VP8PredFunc = Option<unsafe extern "C" fn(*mut uint8_t) -> ()>;
pub type VP8SimpleFilterFunc =
    Option<unsafe extern "C" fn(*mut uint8_t, ::core::ffi::c_int, ::core::ffi::c_int) -> ()>;
pub type VP8LumaFilterFunc = Option<
    unsafe extern "C" fn(
        *mut uint8_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type VP8ChromaFilterFunc = Option<
    unsafe extern "C" fn(
        *mut uint8_t,
        *mut uint8_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
>;
pub const BPS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const VP8_DITHER_DESCALE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const VP8_DITHER_DESCALE_ROUNDER: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << VP8_DITHER_DESCALE - 1 as ::core::ffi::c_int;
pub const VP8_DITHER_AMP_BITS: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const VP8_DITHER_AMP_CENTER: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << VP8_DITHER_AMP_BITS;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn WebPUint32ToMem(ptr: *mut uint8_t, mut val: uint32_t) {
    memcpy(
        ptr as *mut ::core::ffi::c_void,
        &raw mut val as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn clip_8b(mut v: ::core::ffi::c_int) -> uint8_t {
    return (if v & !(0xff as ::core::ffi::c_int) == 0 {
        v
    } else if v < 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        255 as ::core::ffi::c_int
    }) as uint8_t;
}
unsafe extern "C" fn TransformOne_C(mut in_0: *const int16_t, mut dst: *mut uint8_t) {
    let mut C: [::core::ffi::c_int; 16] = [0; 16];
    let mut tmp: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut i: ::core::ffi::c_int = 0;
    tmp = &raw mut C as *mut ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let a: ::core::ffi::c_int = *in_0.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *in_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let b: ::core::ffi::c_int = *in_0.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            - *in_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let c: ::core::ffi::c_int = (*in_0.offset(4 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            * 35468 as ::core::ffi::c_int
            >> 16 as ::core::ffi::c_int)
            - ((*in_0.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                * 20091 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int)
                + *in_0.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int);
        let d: ::core::ffi::c_int = (*in_0.offset(4 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            * 20091 as ::core::ffi::c_int
            >> 16 as ::core::ffi::c_int)
            + *in_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (*in_0.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                * 35468 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int);
        *tmp.offset(0 as ::core::ffi::c_int as isize) = a + d;
        *tmp.offset(1 as ::core::ffi::c_int as isize) = b + c;
        *tmp.offset(2 as ::core::ffi::c_int as isize) = b - c;
        *tmp.offset(3 as ::core::ffi::c_int as isize) = a - d;
        tmp = tmp.offset(4 as ::core::ffi::c_int as isize);
        in_0 = in_0.offset(1);
        i += 1;
    }
    tmp = &raw mut C as *mut ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let dc: ::core::ffi::c_int =
            *tmp.offset(0 as ::core::ffi::c_int as isize) + 4 as ::core::ffi::c_int;
        let a_0: ::core::ffi::c_int = dc + *tmp.offset(8 as ::core::ffi::c_int as isize);
        let b_0: ::core::ffi::c_int = dc - *tmp.offset(8 as ::core::ffi::c_int as isize);
        let c_0: ::core::ffi::c_int = (*tmp.offset(4 as ::core::ffi::c_int as isize)
            * 35468 as ::core::ffi::c_int
            >> 16 as ::core::ffi::c_int)
            - ((*tmp.offset(12 as ::core::ffi::c_int as isize) * 20091 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int)
                + *tmp.offset(12 as ::core::ffi::c_int as isize));
        let d_0: ::core::ffi::c_int = (*tmp.offset(4 as ::core::ffi::c_int as isize)
            * 20091 as ::core::ffi::c_int
            >> 16 as ::core::ffi::c_int)
            + *tmp.offset(4 as ::core::ffi::c_int as isize)
            + (*tmp.offset(12 as ::core::ffi::c_int as isize) * 35468 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int);
        *dst.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
            *dst.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize)
                as ::core::ffi::c_int
                + (a_0 + d_0 >> 3 as ::core::ffi::c_int),
        );
        *dst.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
            *dst.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize)
                as ::core::ffi::c_int
                + (b_0 + c_0 >> 3 as ::core::ffi::c_int),
        );
        *dst.offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
            *dst.offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize)
                as ::core::ffi::c_int
                + (b_0 - c_0 >> 3 as ::core::ffi::c_int),
        );
        *dst.offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
            *dst.offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize)
                as ::core::ffi::c_int
                + (a_0 - d_0 >> 3 as ::core::ffi::c_int),
        );
        tmp = tmp.offset(1);
        dst = dst.offset(BPS as isize);
        i += 1;
    }
}
unsafe extern "C" fn TransformAC3_C(mut in_0: *const int16_t, mut dst: *mut uint8_t) {
    let a: ::core::ffi::c_int = *in_0.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        + 4 as ::core::ffi::c_int;
    let c4: ::core::ffi::c_int = *in_0.offset(4 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        * 35468 as ::core::ffi::c_int
        >> 16 as ::core::ffi::c_int;
    let d4: ::core::ffi::c_int = (*in_0.offset(4 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        * 20091 as ::core::ffi::c_int
        >> 16 as ::core::ffi::c_int)
        + *in_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    let c1: ::core::ffi::c_int = *in_0.offset(1 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        * 35468 as ::core::ffi::c_int
        >> 16 as ::core::ffi::c_int;
    let d1: ::core::ffi::c_int = (*in_0.offset(1 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        * 20091 as ::core::ffi::c_int
        >> 16 as ::core::ffi::c_int)
        + *in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    let DC: ::core::ffi::c_int = a + d4;
    *dst.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
        *dst.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize)
            as ::core::ffi::c_int
            + (DC + d1 >> 3 as ::core::ffi::c_int),
    );
    *dst.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
        *dst.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize)
            as ::core::ffi::c_int
            + (DC + c1 >> 3 as ::core::ffi::c_int),
    );
    *dst.offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
        *dst.offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize)
            as ::core::ffi::c_int
            + (DC - c1 >> 3 as ::core::ffi::c_int),
    );
    *dst.offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
        *dst.offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize)
            as ::core::ffi::c_int
            + (DC - d1 >> 3 as ::core::ffi::c_int),
    );
    let DC_0: ::core::ffi::c_int = a + c4;
    *dst.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
        *dst.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize)
            as ::core::ffi::c_int
            + (DC_0 + d1 >> 3 as ::core::ffi::c_int),
    );
    *dst.offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
        *dst.offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize)
            as ::core::ffi::c_int
            + (DC_0 + c1 >> 3 as ::core::ffi::c_int),
    );
    *dst.offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
        *dst.offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize)
            as ::core::ffi::c_int
            + (DC_0 - c1 >> 3 as ::core::ffi::c_int),
    );
    *dst.offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
        *dst.offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize)
            as ::core::ffi::c_int
            + (DC_0 - d1 >> 3 as ::core::ffi::c_int),
    );
    let DC_1: ::core::ffi::c_int = a - c4;
    *dst.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
        *dst.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize)
            as ::core::ffi::c_int
            + (DC_1 + d1 >> 3 as ::core::ffi::c_int),
    );
    *dst.offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
        *dst.offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize)
            as ::core::ffi::c_int
            + (DC_1 + c1 >> 3 as ::core::ffi::c_int),
    );
    *dst.offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
        *dst.offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize)
            as ::core::ffi::c_int
            + (DC_1 - c1 >> 3 as ::core::ffi::c_int),
    );
    *dst.offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
        *dst.offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize)
            as ::core::ffi::c_int
            + (DC_1 - d1 >> 3 as ::core::ffi::c_int),
    );
    let DC_2: ::core::ffi::c_int = a - d4;
    *dst.offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
        *dst.offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize)
            as ::core::ffi::c_int
            + (DC_2 + d1 >> 3 as ::core::ffi::c_int),
    );
    *dst.offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
        *dst.offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize)
            as ::core::ffi::c_int
            + (DC_2 + c1 >> 3 as ::core::ffi::c_int),
    );
    *dst.offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
        *dst.offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize)
            as ::core::ffi::c_int
            + (DC_2 - c1 >> 3 as ::core::ffi::c_int),
    );
    *dst.offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize) = clip_8b(
        *dst.offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize)
            as ::core::ffi::c_int
            + (DC_2 - d1 >> 3 as ::core::ffi::c_int),
    );
}
unsafe extern "C" fn TransformTwo_C(
    mut in_0: *const int16_t,
    mut dst: *mut uint8_t,
    mut do_two: ::core::ffi::c_int,
) {
    TransformOne_C(in_0, dst);
    if do_two != 0 {
        TransformOne_C(
            in_0.offset(16 as ::core::ffi::c_int as isize),
            dst.offset(4 as ::core::ffi::c_int as isize),
        );
    }
}
unsafe extern "C" fn TransformUV_C(mut in_0: *const int16_t, mut dst: *mut uint8_t) {
    VP8Transform.expect("non-null function pointer")(
        in_0.offset((0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize),
        dst,
        1 as ::core::ffi::c_int,
    );
    VP8Transform.expect("non-null function pointer")(
        in_0.offset((2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize),
        dst.offset((4 as ::core::ffi::c_int * BPS) as isize),
        1 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn TransformDC_C(mut in_0: *const int16_t, mut dst: *mut uint8_t) {
    let DC: ::core::ffi::c_int = *in_0.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        + 4 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < 4 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            *dst.offset((i + j * BPS) as isize) = clip_8b(
                *dst.offset((i + j * BPS) as isize) as ::core::ffi::c_int
                    + (DC >> 3 as ::core::ffi::c_int),
            );
            i += 1;
        }
        j += 1;
    }
}
unsafe extern "C" fn TransformDCUV_C(mut in_0: *const int16_t, mut dst: *mut uint8_t) {
    if *in_0.offset((0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize) != 0 {
        VP8TransformDC.expect("non-null function pointer")(
            in_0.offset((0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize),
            dst,
        );
    }
    if *in_0.offset((1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize) != 0 {
        VP8TransformDC.expect("non-null function pointer")(
            in_0.offset((1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize),
            dst.offset(4 as ::core::ffi::c_int as isize),
        );
    }
    if *in_0.offset((2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize) != 0 {
        VP8TransformDC.expect("non-null function pointer")(
            in_0.offset((2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize),
            dst.offset((4 as ::core::ffi::c_int * BPS) as isize),
        );
    }
    if *in_0.offset((3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize) != 0 {
        VP8TransformDC.expect("non-null function pointer")(
            in_0.offset((3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize),
            dst.offset((4 as ::core::ffi::c_int * BPS) as isize)
                .offset(4 as ::core::ffi::c_int as isize),
        );
    }
}
unsafe extern "C" fn TransformWHT_C(mut in_0: *const int16_t, mut out: *mut int16_t) {
    let mut tmp: [::core::ffi::c_int; 16] = [0; 16];
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let a0: ::core::ffi::c_int = *in_0.offset((0 as ::core::ffi::c_int + i) as isize)
            as ::core::ffi::c_int
            + *in_0.offset((12 as ::core::ffi::c_int + i) as isize) as ::core::ffi::c_int;
        let a1: ::core::ffi::c_int = *in_0.offset((4 as ::core::ffi::c_int + i) as isize)
            as ::core::ffi::c_int
            + *in_0.offset((8 as ::core::ffi::c_int + i) as isize) as ::core::ffi::c_int;
        let a2: ::core::ffi::c_int = *in_0.offset((4 as ::core::ffi::c_int + i) as isize)
            as ::core::ffi::c_int
            - *in_0.offset((8 as ::core::ffi::c_int + i) as isize) as ::core::ffi::c_int;
        let a3: ::core::ffi::c_int = *in_0.offset((0 as ::core::ffi::c_int + i) as isize)
            as ::core::ffi::c_int
            - *in_0.offset((12 as ::core::ffi::c_int + i) as isize) as ::core::ffi::c_int;
        tmp[(0 as ::core::ffi::c_int + i) as usize] = a0 + a1;
        tmp[(8 as ::core::ffi::c_int + i) as usize] = a0 - a1;
        tmp[(4 as ::core::ffi::c_int + i) as usize] = a3 + a2;
        tmp[(12 as ::core::ffi::c_int + i) as usize] = a3 - a2;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let dc: ::core::ffi::c_int = tmp
            [(0 as ::core::ffi::c_int + i * 4 as ::core::ffi::c_int) as usize]
            + 3 as ::core::ffi::c_int;
        let a0_0: ::core::ffi::c_int =
            dc + tmp[(3 as ::core::ffi::c_int + i * 4 as ::core::ffi::c_int) as usize];
        let a1_0: ::core::ffi::c_int = tmp
            [(1 as ::core::ffi::c_int + i * 4 as ::core::ffi::c_int) as usize]
            + tmp[(2 as ::core::ffi::c_int + i * 4 as ::core::ffi::c_int) as usize];
        let a2_0: ::core::ffi::c_int = tmp
            [(1 as ::core::ffi::c_int + i * 4 as ::core::ffi::c_int) as usize]
            - tmp[(2 as ::core::ffi::c_int + i * 4 as ::core::ffi::c_int) as usize];
        let a3_0: ::core::ffi::c_int =
            dc - tmp[(3 as ::core::ffi::c_int + i * 4 as ::core::ffi::c_int) as usize];
        *out.offset(0 as ::core::ffi::c_int as isize) =
            (a0_0 + a1_0 >> 3 as ::core::ffi::c_int) as int16_t;
        *out.offset(16 as ::core::ffi::c_int as isize) =
            (a3_0 + a2_0 >> 3 as ::core::ffi::c_int) as int16_t;
        *out.offset(32 as ::core::ffi::c_int as isize) =
            (a0_0 - a1_0 >> 3 as ::core::ffi::c_int) as int16_t;
        *out.offset(48 as ::core::ffi::c_int as isize) =
            (a3_0 - a2_0 >> 3 as ::core::ffi::c_int) as int16_t;
        out = out.offset(64 as ::core::ffi::c_int as isize);
        i += 1;
    }
}
#[no_mangle]
pub static mut VP8TransformWHT: Option<unsafe extern "C" fn(*const int16_t, *mut int16_t) -> ()> =
    None;
#[inline]
unsafe extern "C" fn TrueMotion(mut dst: *mut uint8_t, mut size: ::core::ffi::c_int) {
    let mut top: *const uint8_t = dst.offset(-(BPS as isize));
    let clip0: *const uint8_t = VP8kclip1
        .offset(-(*top.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int as isize));
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < size {
        let clip: *const uint8_t = clip0.offset(
            *dst.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int as isize
        );
        let mut x: ::core::ffi::c_int = 0;
        x = 0 as ::core::ffi::c_int;
        while x < size {
            *dst.offset(x as isize) = *clip.offset(*top.offset(x as isize) as isize);
            x += 1;
        }
        dst = dst.offset(BPS as isize);
        y += 1;
    }
}
unsafe extern "C" fn TM4_C(mut dst: *mut uint8_t) {
    TrueMotion(dst, 4 as ::core::ffi::c_int);
}
unsafe extern "C" fn TM8uv_C(mut dst: *mut uint8_t) {
    TrueMotion(dst, 8 as ::core::ffi::c_int);
}
unsafe extern "C" fn TM16_C(mut dst: *mut uint8_t) {
    TrueMotion(dst, 16 as ::core::ffi::c_int);
}
unsafe extern "C" fn VE16_C(mut dst: *mut uint8_t) {
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < 16 as ::core::ffi::c_int {
        memcpy(
            dst.offset((j * BPS) as isize) as *mut ::core::ffi::c_void,
            dst.offset(-(BPS as isize)) as *const ::core::ffi::c_void,
            16 as size_t,
        );
        j += 1;
    }
}
unsafe extern "C" fn HE16_C(mut dst: *mut uint8_t) {
    let mut j: ::core::ffi::c_int = 0;
    j = 16 as ::core::ffi::c_int;
    while j > 0 as ::core::ffi::c_int {
        memset(
            dst as *mut ::core::ffi::c_void,
            *dst.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int,
            16 as size_t,
        );
        dst = dst.offset(BPS as isize);
        j -= 1;
    }
}
#[inline]
unsafe extern "C" fn Put16(mut v: ::core::ffi::c_int, mut dst: *mut uint8_t) {
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < 16 as ::core::ffi::c_int {
        memset(
            dst.offset((j * BPS) as isize) as *mut ::core::ffi::c_void,
            v,
            16 as size_t,
        );
        j += 1;
    }
}
unsafe extern "C" fn DC16_C(mut dst: *mut uint8_t) {
    let mut DC: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < 16 as ::core::ffi::c_int {
        DC += *dst.offset((-(1 as ::core::ffi::c_int) + j * BPS) as isize) as ::core::ffi::c_int
            + *dst.offset((j - BPS) as isize) as ::core::ffi::c_int;
        j += 1;
    }
    Put16(DC >> 5 as ::core::ffi::c_int, dst);
}
unsafe extern "C" fn DC16NoTop_C(mut dst: *mut uint8_t) {
    let mut DC: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < 16 as ::core::ffi::c_int {
        DC += *dst.offset((-(1 as ::core::ffi::c_int) + j * BPS) as isize) as ::core::ffi::c_int;
        j += 1;
    }
    Put16(DC >> 4 as ::core::ffi::c_int, dst);
}
unsafe extern "C" fn DC16NoLeft_C(mut dst: *mut uint8_t) {
    let mut DC: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        DC += *dst.offset((i - BPS) as isize) as ::core::ffi::c_int;
        i += 1;
    }
    Put16(DC >> 4 as ::core::ffi::c_int, dst);
}
unsafe extern "C" fn DC16NoTopLeft_C(mut dst: *mut uint8_t) {
    Put16(0x80 as ::core::ffi::c_int, dst);
}
#[no_mangle]
pub static mut VP8PredLuma16: [VP8PredFunc; 7] = [None; 7];
unsafe extern "C" fn VE4_C(mut dst: *mut uint8_t) {
    let mut top: *const uint8_t = dst.offset(-(BPS as isize));
    let vals: [uint8_t; 4] = [
        (*top.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *top.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *top.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t,
        (*top.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *top.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *top.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t,
        (*top.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *top.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *top.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t,
        (*top.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *top.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *top.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t,
    ];
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        memcpy(
            dst.offset((i * BPS) as isize) as *mut ::core::ffi::c_void,
            &raw const vals as *const uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
        );
        i += 1;
    }
}
unsafe extern "C" fn HE4_C(mut dst: *mut uint8_t) {
    let A: ::core::ffi::c_int =
        *dst.offset((-(1 as ::core::ffi::c_int) - BPS) as isize) as ::core::ffi::c_int;
    let B: ::core::ffi::c_int =
        *dst.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int;
    let C: ::core::ffi::c_int =
        *dst.offset((-(1 as ::core::ffi::c_int) + BPS) as isize) as ::core::ffi::c_int;
    let D: ::core::ffi::c_int = *dst
        .offset((-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * BPS) as isize)
        as ::core::ffi::c_int;
    let E: ::core::ffi::c_int = *dst
        .offset((-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * BPS) as isize)
        as ::core::ffi::c_int;
    WebPUint32ToMem(
        dst.offset((0 as ::core::ffi::c_int * BPS) as isize),
        (0x1010101 as uint32_t).wrapping_mul(
            (A + 2 as ::core::ffi::c_int * B + C + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t as uint32_t,
        ),
    );
    WebPUint32ToMem(
        dst.offset((1 as ::core::ffi::c_int * BPS) as isize),
        (0x1010101 as uint32_t).wrapping_mul(
            (B + 2 as ::core::ffi::c_int * C + D + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t as uint32_t,
        ),
    );
    WebPUint32ToMem(
        dst.offset((2 as ::core::ffi::c_int * BPS) as isize),
        (0x1010101 as uint32_t).wrapping_mul(
            (C + 2 as ::core::ffi::c_int * D + E + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t as uint32_t,
        ),
    );
    WebPUint32ToMem(
        dst.offset((3 as ::core::ffi::c_int * BPS) as isize),
        (0x1010101 as uint32_t).wrapping_mul(
            (D + 2 as ::core::ffi::c_int * E + E + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as uint8_t as uint32_t,
        ),
    );
}
unsafe extern "C" fn DC4_C(mut dst: *mut uint8_t) {
    let mut dc: uint32_t = 4 as uint32_t;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        dc = dc.wrapping_add(
            (*dst.offset((i - BPS) as isize) as ::core::ffi::c_int
                + *dst.offset((-(1 as ::core::ffi::c_int) + i * BPS) as isize)
                    as ::core::ffi::c_int) as uint32_t,
        );
        i += 1;
    }
    dc >>= 3 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        memset(
            dst.offset((i * BPS) as isize) as *mut ::core::ffi::c_void,
            dc as ::core::ffi::c_int,
            4 as size_t,
        );
        i += 1;
    }
}
unsafe extern "C" fn RD4_C(mut dst: *mut uint8_t) {
    let I: ::core::ffi::c_int = *dst
        .offset((-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * BPS) as isize)
        as ::core::ffi::c_int;
    let J: ::core::ffi::c_int = *dst
        .offset((-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * BPS) as isize)
        as ::core::ffi::c_int;
    let K: ::core::ffi::c_int = *dst
        .offset((-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * BPS) as isize)
        as ::core::ffi::c_int;
    let L: ::core::ffi::c_int = *dst
        .offset((-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * BPS) as isize)
        as ::core::ffi::c_int;
    let X: ::core::ffi::c_int =
        *dst.offset((-(1 as ::core::ffi::c_int) - BPS) as isize) as ::core::ffi::c_int;
    let A: ::core::ffi::c_int =
        *dst.offset((0 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let B: ::core::ffi::c_int =
        *dst.offset((1 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let C: ::core::ffi::c_int =
        *dst.offset((2 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let D: ::core::ffi::c_int =
        *dst.offset((3 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    *dst.offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize) =
        (J + 2 as ::core::ffi::c_int * K + L + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
            as uint8_t;
    let ref mut fresh36 =
        *dst.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize);
    *fresh36 = (I + 2 as ::core::ffi::c_int * J + K + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize) = *fresh36;
    let ref mut fresh37 =
        *dst.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize);
    *fresh37 = (X + 2 as ::core::ffi::c_int * I + J + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh38 =
        *dst.offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize);
    *fresh38 = *fresh37;
    *dst.offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize) = *fresh38;
    let ref mut fresh39 =
        *dst.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize);
    *fresh39 = (A + 2 as ::core::ffi::c_int * X + I + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh40 =
        *dst.offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize);
    *fresh40 = *fresh39;
    let ref mut fresh41 =
        *dst.offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize);
    *fresh41 = *fresh40;
    *dst.offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize) = *fresh41;
    let ref mut fresh42 =
        *dst.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize);
    *fresh42 = (B + 2 as ::core::ffi::c_int * A + X + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh43 =
        *dst.offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize);
    *fresh43 = *fresh42;
    *dst.offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize) = *fresh43;
    let ref mut fresh44 =
        *dst.offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize);
    *fresh44 = (C + 2 as ::core::ffi::c_int * B + A + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize) = *fresh44;
    *dst.offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) =
        (D + 2 as ::core::ffi::c_int * C + B + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
            as uint8_t;
}
unsafe extern "C" fn LD4_C(mut dst: *mut uint8_t) {
    let A: ::core::ffi::c_int =
        *dst.offset((0 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let B: ::core::ffi::c_int =
        *dst.offset((1 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let C: ::core::ffi::c_int =
        *dst.offset((2 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let D: ::core::ffi::c_int =
        *dst.offset((3 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let E: ::core::ffi::c_int =
        *dst.offset((4 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let F: ::core::ffi::c_int =
        *dst.offset((5 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let G: ::core::ffi::c_int =
        *dst.offset((6 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let H: ::core::ffi::c_int =
        *dst.offset((7 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    *dst.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) =
        (A + 2 as ::core::ffi::c_int * B + C + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
            as uint8_t;
    let ref mut fresh27 =
        *dst.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize);
    *fresh27 = (B + 2 as ::core::ffi::c_int * C + D + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = *fresh27;
    let ref mut fresh28 =
        *dst.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize);
    *fresh28 = (C + 2 as ::core::ffi::c_int * D + E + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh29 =
        *dst.offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize);
    *fresh29 = *fresh28;
    *dst.offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = *fresh29;
    let ref mut fresh30 =
        *dst.offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize);
    *fresh30 = (D + 2 as ::core::ffi::c_int * E + F + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh31 =
        *dst.offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize);
    *fresh31 = *fresh30;
    let ref mut fresh32 =
        *dst.offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize);
    *fresh32 = *fresh31;
    *dst.offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = *fresh32;
    let ref mut fresh33 =
        *dst.offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize);
    *fresh33 = (E + 2 as ::core::ffi::c_int * F + G + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh34 =
        *dst.offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize);
    *fresh34 = *fresh33;
    *dst.offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize) = *fresh34;
    let ref mut fresh35 =
        *dst.offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize);
    *fresh35 = (F + 2 as ::core::ffi::c_int * G + H + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize) = *fresh35;
    *dst.offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize) =
        (G + 2 as ::core::ffi::c_int * H + H + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
            as uint8_t;
}
unsafe extern "C" fn VR4_C(mut dst: *mut uint8_t) {
    let I: ::core::ffi::c_int = *dst
        .offset((-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * BPS) as isize)
        as ::core::ffi::c_int;
    let J: ::core::ffi::c_int = *dst
        .offset((-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * BPS) as isize)
        as ::core::ffi::c_int;
    let K: ::core::ffi::c_int = *dst
        .offset((-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * BPS) as isize)
        as ::core::ffi::c_int;
    let X: ::core::ffi::c_int =
        *dst.offset((-(1 as ::core::ffi::c_int) - BPS) as isize) as ::core::ffi::c_int;
    let A: ::core::ffi::c_int =
        *dst.offset((0 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let B: ::core::ffi::c_int =
        *dst.offset((1 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let C: ::core::ffi::c_int =
        *dst.offset((2 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let D: ::core::ffi::c_int =
        *dst.offset((3 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let ref mut fresh21 =
        *dst.offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize);
    *fresh21 = (X + A + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = *fresh21;
    let ref mut fresh22 =
        *dst.offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize);
    *fresh22 = (A + B + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = *fresh22;
    let ref mut fresh23 =
        *dst.offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize);
    *fresh23 = (B + C + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = *fresh23;
    *dst.offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) =
        (C + D + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize) =
        (K + 2 as ::core::ffi::c_int * J + I + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
            as uint8_t;
    *dst.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize) =
        (J + 2 as ::core::ffi::c_int * I + X + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
            as uint8_t;
    let ref mut fresh24 =
        *dst.offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize);
    *fresh24 = (I + 2 as ::core::ffi::c_int * X + A + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize) = *fresh24;
    let ref mut fresh25 =
        *dst.offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize);
    *fresh25 = (X + 2 as ::core::ffi::c_int * A + B + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize) = *fresh25;
    let ref mut fresh26 =
        *dst.offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize);
    *fresh26 = (A + 2 as ::core::ffi::c_int * B + C + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize) = *fresh26;
    *dst.offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize) =
        (B + 2 as ::core::ffi::c_int * C + D + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
            as uint8_t;
}
unsafe extern "C" fn VL4_C(mut dst: *mut uint8_t) {
    let A: ::core::ffi::c_int =
        *dst.offset((0 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let B: ::core::ffi::c_int =
        *dst.offset((1 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let C: ::core::ffi::c_int =
        *dst.offset((2 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let D: ::core::ffi::c_int =
        *dst.offset((3 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let E: ::core::ffi::c_int =
        *dst.offset((4 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let F: ::core::ffi::c_int =
        *dst.offset((5 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let G: ::core::ffi::c_int =
        *dst.offset((6 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let H: ::core::ffi::c_int =
        *dst.offset((7 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    *dst.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) =
        (A + B + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh15 =
        *dst.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize);
    *fresh15 = (B + C + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = *fresh15;
    let ref mut fresh16 =
        *dst.offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize);
    *fresh16 = (C + D + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = *fresh16;
    let ref mut fresh17 =
        *dst.offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize);
    *fresh17 = (D + E + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = *fresh17;
    *dst.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize) =
        (A + 2 as ::core::ffi::c_int * B + C + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
            as uint8_t;
    let ref mut fresh18 =
        *dst.offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize);
    *fresh18 = (B + 2 as ::core::ffi::c_int * C + D + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize) = *fresh18;
    let ref mut fresh19 =
        *dst.offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize);
    *fresh19 = (C + 2 as ::core::ffi::c_int * D + E + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize) = *fresh19;
    let ref mut fresh20 =
        *dst.offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize);
    *fresh20 = (D + 2 as ::core::ffi::c_int * E + F + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize) = *fresh20;
    *dst.offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize) =
        (E + 2 as ::core::ffi::c_int * F + G + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
            as uint8_t;
    *dst.offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize) =
        (F + 2 as ::core::ffi::c_int * G + H + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
            as uint8_t;
}
unsafe extern "C" fn HU4_C(mut dst: *mut uint8_t) {
    let I: ::core::ffi::c_int = *dst
        .offset((-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * BPS) as isize)
        as ::core::ffi::c_int;
    let J: ::core::ffi::c_int = *dst
        .offset((-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * BPS) as isize)
        as ::core::ffi::c_int;
    let K: ::core::ffi::c_int = *dst
        .offset((-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * BPS) as isize)
        as ::core::ffi::c_int;
    let L: ::core::ffi::c_int = *dst
        .offset((-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * BPS) as isize)
        as ::core::ffi::c_int;
    *dst.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) =
        (I + J + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh0 =
        *dst.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize);
    *fresh0 = (J + K + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = *fresh0;
    let ref mut fresh1 =
        *dst.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize);
    *fresh1 = (K + L + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize) = *fresh1;
    *dst.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) =
        (I + 2 as ::core::ffi::c_int * J + K + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
            as uint8_t;
    let ref mut fresh2 =
        *dst.offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize);
    *fresh2 = (J + 2 as ::core::ffi::c_int * K + L + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = *fresh2;
    let ref mut fresh3 =
        *dst.offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize);
    *fresh3 = (K + 2 as ::core::ffi::c_int * L + L + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize) = *fresh3;
    let ref mut fresh4 =
        *dst.offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize);
    *fresh4 = L as uint8_t;
    let ref mut fresh5 =
        *dst.offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize);
    *fresh5 = *fresh4;
    let ref mut fresh6 =
        *dst.offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize);
    *fresh6 = *fresh5;
    let ref mut fresh7 =
        *dst.offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize);
    *fresh7 = *fresh6;
    let ref mut fresh8 =
        *dst.offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize);
    *fresh8 = *fresh7;
    *dst.offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize) = *fresh8;
}
unsafe extern "C" fn HD4_C(mut dst: *mut uint8_t) {
    let I: ::core::ffi::c_int = *dst
        .offset((-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * BPS) as isize)
        as ::core::ffi::c_int;
    let J: ::core::ffi::c_int = *dst
        .offset((-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * BPS) as isize)
        as ::core::ffi::c_int;
    let K: ::core::ffi::c_int = *dst
        .offset((-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * BPS) as isize)
        as ::core::ffi::c_int;
    let L: ::core::ffi::c_int = *dst
        .offset((-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * BPS) as isize)
        as ::core::ffi::c_int;
    let X: ::core::ffi::c_int =
        *dst.offset((-(1 as ::core::ffi::c_int) - BPS) as isize) as ::core::ffi::c_int;
    let A: ::core::ffi::c_int =
        *dst.offset((0 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let B: ::core::ffi::c_int =
        *dst.offset((1 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let C: ::core::ffi::c_int =
        *dst.offset((2 as ::core::ffi::c_int - BPS) as isize) as ::core::ffi::c_int;
    let ref mut fresh9 =
        *dst.offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize);
    *fresh9 = (I + X + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = *fresh9;
    let ref mut fresh10 =
        *dst.offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize);
    *fresh10 = (J + I + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize) = *fresh10;
    let ref mut fresh11 =
        *dst.offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize);
    *fresh11 = (K + J + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize) = *fresh11;
    *dst.offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize) =
        (L + K + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) =
        (A + 2 as ::core::ffi::c_int * B + C + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
            as uint8_t;
    *dst.offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) =
        (X + 2 as ::core::ffi::c_int * A + B + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
            as uint8_t;
    let ref mut fresh12 =
        *dst.offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize);
    *fresh12 = (I + 2 as ::core::ffi::c_int * X + A + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * BPS) as isize) = *fresh12;
    let ref mut fresh13 =
        *dst.offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize);
    *fresh13 = (J + 2 as ::core::ffi::c_int * I + X + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * BPS) as isize) = *fresh13;
    let ref mut fresh14 =
        *dst.offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize);
    *fresh14 = (K + 2 as ::core::ffi::c_int * J + I + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *dst.offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * BPS) as isize) = *fresh14;
    *dst.offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * BPS) as isize) =
        (L + 2 as ::core::ffi::c_int * K + J + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
            as uint8_t;
}
#[no_mangle]
pub static mut VP8PredLuma4: [VP8PredFunc; 10] = [None; 10];
unsafe extern "C" fn VE8uv_C(mut dst: *mut uint8_t) {
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < 8 as ::core::ffi::c_int {
        memcpy(
            dst.offset((j * BPS) as isize) as *mut ::core::ffi::c_void,
            dst.offset(-(BPS as isize)) as *const ::core::ffi::c_void,
            8 as size_t,
        );
        j += 1;
    }
}
unsafe extern "C" fn HE8uv_C(mut dst: *mut uint8_t) {
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < 8 as ::core::ffi::c_int {
        memset(
            dst as *mut ::core::ffi::c_void,
            *dst.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int,
            8 as size_t,
        );
        dst = dst.offset(BPS as isize);
        j += 1;
    }
}
#[inline]
unsafe extern "C" fn Put8x8uv(mut value: uint8_t, mut dst: *mut uint8_t) {
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < 8 as ::core::ffi::c_int {
        memset(
            dst.offset((j * BPS) as isize) as *mut ::core::ffi::c_void,
            value as ::core::ffi::c_int,
            8 as size_t,
        );
        j += 1;
    }
}
unsafe extern "C" fn DC8uv_C(mut dst: *mut uint8_t) {
    let mut dc0: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        dc0 += *dst.offset((i - BPS) as isize) as ::core::ffi::c_int
            + *dst.offset((-(1 as ::core::ffi::c_int) + i * BPS) as isize) as ::core::ffi::c_int;
        i += 1;
    }
    Put8x8uv((dc0 >> 4 as ::core::ffi::c_int) as uint8_t, dst);
}
unsafe extern "C" fn DC8uvNoLeft_C(mut dst: *mut uint8_t) {
    let mut dc0: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        dc0 += *dst.offset((i - BPS) as isize) as ::core::ffi::c_int;
        i += 1;
    }
    Put8x8uv((dc0 >> 3 as ::core::ffi::c_int) as uint8_t, dst);
}
unsafe extern "C" fn DC8uvNoTop_C(mut dst: *mut uint8_t) {
    let mut dc0: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        dc0 += *dst.offset((-(1 as ::core::ffi::c_int) + i * BPS) as isize) as ::core::ffi::c_int;
        i += 1;
    }
    Put8x8uv((dc0 >> 3 as ::core::ffi::c_int) as uint8_t, dst);
}
unsafe extern "C" fn DC8uvNoTopLeft_C(mut dst: *mut uint8_t) {
    Put8x8uv(0x80 as uint8_t, dst);
}
#[no_mangle]
pub static mut VP8PredChroma8: [VP8PredFunc; 7] = [None; 7];
#[inline]
unsafe extern "C" fn DoFilter2_C(mut p: *mut uint8_t, mut step: ::core::ffi::c_int) {
    let p1: ::core::ffi::c_int =
        *p.offset((-(2 as ::core::ffi::c_int) * step) as isize) as ::core::ffi::c_int;
    let p0: ::core::ffi::c_int = *p.offset(-step as isize) as ::core::ffi::c_int;
    let q0: ::core::ffi::c_int = *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    let q1: ::core::ffi::c_int = *p.offset(step as isize) as ::core::ffi::c_int;
    let a: ::core::ffi::c_int = 3 as ::core::ffi::c_int * (q0 - p0)
        + *VP8ksclip1.offset((p1 - q1) as isize) as ::core::ffi::c_int;
    let a1: ::core::ffi::c_int = *VP8ksclip2
        .offset((a + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let a2: ::core::ffi::c_int = *VP8ksclip2
        .offset((a + 3 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    *p.offset(-step as isize) = *VP8kclip1.offset((p0 + a2) as isize);
    *p.offset(0 as ::core::ffi::c_int as isize) = *VP8kclip1.offset((q0 - a1) as isize);
}
#[inline]
unsafe extern "C" fn DoFilter4_C(mut p: *mut uint8_t, mut step: ::core::ffi::c_int) {
    let p1: ::core::ffi::c_int =
        *p.offset((-(2 as ::core::ffi::c_int) * step) as isize) as ::core::ffi::c_int;
    let p0: ::core::ffi::c_int = *p.offset(-step as isize) as ::core::ffi::c_int;
    let q0: ::core::ffi::c_int = *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    let q1: ::core::ffi::c_int = *p.offset(step as isize) as ::core::ffi::c_int;
    let a: ::core::ffi::c_int = 3 as ::core::ffi::c_int * (q0 - p0);
    let a1: ::core::ffi::c_int = *VP8ksclip2
        .offset((a + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let a2: ::core::ffi::c_int = *VP8ksclip2
        .offset((a + 3 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let a3: ::core::ffi::c_int = a1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
    *p.offset((-(2 as ::core::ffi::c_int) * step) as isize) = *VP8kclip1.offset((p1 + a3) as isize);
    *p.offset(-step as isize) = *VP8kclip1.offset((p0 + a2) as isize);
    *p.offset(0 as ::core::ffi::c_int as isize) = *VP8kclip1.offset((q0 - a1) as isize);
    *p.offset(step as isize) = *VP8kclip1.offset((q1 - a3) as isize);
}
#[inline]
unsafe extern "C" fn DoFilter6_C(mut p: *mut uint8_t, mut step: ::core::ffi::c_int) {
    let p2: ::core::ffi::c_int =
        *p.offset((-(3 as ::core::ffi::c_int) * step) as isize) as ::core::ffi::c_int;
    let p1: ::core::ffi::c_int =
        *p.offset((-(2 as ::core::ffi::c_int) * step) as isize) as ::core::ffi::c_int;
    let p0: ::core::ffi::c_int = *p.offset(-step as isize) as ::core::ffi::c_int;
    let q0: ::core::ffi::c_int = *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    let q1: ::core::ffi::c_int = *p.offset(step as isize) as ::core::ffi::c_int;
    let q2: ::core::ffi::c_int =
        *p.offset((2 as ::core::ffi::c_int * step) as isize) as ::core::ffi::c_int;
    let a: ::core::ffi::c_int = *VP8ksclip1.offset(
        (3 as ::core::ffi::c_int * (q0 - p0)
            + *VP8ksclip1.offset((p1 - q1) as isize) as ::core::ffi::c_int) as isize,
    ) as ::core::ffi::c_int;
    let a1: ::core::ffi::c_int =
        27 as ::core::ffi::c_int * a + 63 as ::core::ffi::c_int >> 7 as ::core::ffi::c_int;
    let a2: ::core::ffi::c_int =
        18 as ::core::ffi::c_int * a + 63 as ::core::ffi::c_int >> 7 as ::core::ffi::c_int;
    let a3: ::core::ffi::c_int =
        9 as ::core::ffi::c_int * a + 63 as ::core::ffi::c_int >> 7 as ::core::ffi::c_int;
    *p.offset((-(3 as ::core::ffi::c_int) * step) as isize) = *VP8kclip1.offset((p2 + a3) as isize);
    *p.offset((-(2 as ::core::ffi::c_int) * step) as isize) = *VP8kclip1.offset((p1 + a2) as isize);
    *p.offset(-step as isize) = *VP8kclip1.offset((p0 + a1) as isize);
    *p.offset(0 as ::core::ffi::c_int as isize) = *VP8kclip1.offset((q0 - a1) as isize);
    *p.offset(step as isize) = *VP8kclip1.offset((q1 - a2) as isize);
    *p.offset((2 as ::core::ffi::c_int * step) as isize) = *VP8kclip1.offset((q2 - a3) as isize);
}
#[inline]
unsafe extern "C" fn Hev(
    mut p: *const uint8_t,
    mut step: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let p1: ::core::ffi::c_int =
        *p.offset((-(2 as ::core::ffi::c_int) * step) as isize) as ::core::ffi::c_int;
    let p0: ::core::ffi::c_int = *p.offset(-step as isize) as ::core::ffi::c_int;
    let q0: ::core::ffi::c_int = *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    let q1: ::core::ffi::c_int = *p.offset(step as isize) as ::core::ffi::c_int;
    return (*VP8kabs0.offset((p1 - p0) as isize) as ::core::ffi::c_int > thresh
        || *VP8kabs0.offset((q1 - q0) as isize) as ::core::ffi::c_int > thresh)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn NeedsFilter_C(
    mut p: *const uint8_t,
    mut step: ::core::ffi::c_int,
    mut t: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let p1: ::core::ffi::c_int =
        *p.offset((-(2 as ::core::ffi::c_int) * step) as isize) as ::core::ffi::c_int;
    let p0: ::core::ffi::c_int = *p.offset(-step as isize) as ::core::ffi::c_int;
    let q0: ::core::ffi::c_int = *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    let q1: ::core::ffi::c_int = *p.offset(step as isize) as ::core::ffi::c_int;
    return (4 as ::core::ffi::c_int * *VP8kabs0.offset((p0 - q0) as isize) as ::core::ffi::c_int
        + *VP8kabs0.offset((p1 - q1) as isize) as ::core::ffi::c_int
        <= t) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn NeedsFilter2_C(
    mut p: *const uint8_t,
    mut step: ::core::ffi::c_int,
    mut t: ::core::ffi::c_int,
    mut it: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let p3: ::core::ffi::c_int =
        *p.offset((-(4 as ::core::ffi::c_int) * step) as isize) as ::core::ffi::c_int;
    let p2: ::core::ffi::c_int =
        *p.offset((-(3 as ::core::ffi::c_int) * step) as isize) as ::core::ffi::c_int;
    let p1: ::core::ffi::c_int =
        *p.offset((-(2 as ::core::ffi::c_int) * step) as isize) as ::core::ffi::c_int;
    let p0: ::core::ffi::c_int = *p.offset(-step as isize) as ::core::ffi::c_int;
    let q0: ::core::ffi::c_int = *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    let q1: ::core::ffi::c_int = *p.offset(step as isize) as ::core::ffi::c_int;
    let q2: ::core::ffi::c_int =
        *p.offset((2 as ::core::ffi::c_int * step) as isize) as ::core::ffi::c_int;
    let q3: ::core::ffi::c_int =
        *p.offset((3 as ::core::ffi::c_int * step) as isize) as ::core::ffi::c_int;
    if 4 as ::core::ffi::c_int * *VP8kabs0.offset((p0 - q0) as isize) as ::core::ffi::c_int
        + *VP8kabs0.offset((p1 - q1) as isize) as ::core::ffi::c_int
        > t
    {
        return 0 as ::core::ffi::c_int;
    }
    return (*VP8kabs0.offset((p3 - p2) as isize) as ::core::ffi::c_int <= it
        && *VP8kabs0.offset((p2 - p1) as isize) as ::core::ffi::c_int <= it
        && *VP8kabs0.offset((p1 - p0) as isize) as ::core::ffi::c_int <= it
        && *VP8kabs0.offset((q3 - q2) as isize) as ::core::ffi::c_int <= it
        && *VP8kabs0.offset((q2 - q1) as isize) as ::core::ffi::c_int <= it
        && *VP8kabs0.offset((q1 - q0) as isize) as ::core::ffi::c_int <= it)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn SimpleVFilter16_C(
    mut p: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let thresh2: ::core::ffi::c_int = 2 as ::core::ffi::c_int * thresh + 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        if NeedsFilter_C(p.offset(i as isize), stride, thresh2) != 0 {
            DoFilter2_C(p.offset(i as isize), stride);
        }
        i += 1;
    }
}
unsafe extern "C" fn SimpleHFilter16_C(
    mut p: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let thresh2: ::core::ffi::c_int = 2 as ::core::ffi::c_int * thresh + 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        if NeedsFilter_C(
            p.offset((i * stride) as isize),
            1 as ::core::ffi::c_int,
            thresh2,
        ) != 0
        {
            DoFilter2_C(p.offset((i * stride) as isize), 1 as ::core::ffi::c_int);
        }
        i += 1;
    }
}
unsafe extern "C" fn SimpleVFilter16i_C(
    mut p: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
) {
    let mut k: ::core::ffi::c_int = 0;
    k = 3 as ::core::ffi::c_int;
    while k > 0 as ::core::ffi::c_int {
        p = p.offset((4 as ::core::ffi::c_int * stride) as isize);
        SimpleVFilter16_C(p, stride, thresh);
        k -= 1;
    }
}
unsafe extern "C" fn SimpleHFilter16i_C(
    mut p: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
) {
    let mut k: ::core::ffi::c_int = 0;
    k = 3 as ::core::ffi::c_int;
    while k > 0 as ::core::ffi::c_int {
        p = p.offset(4 as ::core::ffi::c_int as isize);
        SimpleHFilter16_C(p, stride, thresh);
        k -= 1;
    }
}
#[inline]
unsafe extern "C" fn FilterLoop26_C(
    mut p: *mut uint8_t,
    mut hstride: ::core::ffi::c_int,
    mut vstride: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
    mut ithresh: ::core::ffi::c_int,
    mut hev_thresh: ::core::ffi::c_int,
) {
    let thresh2: ::core::ffi::c_int = 2 as ::core::ffi::c_int * thresh + 1 as ::core::ffi::c_int;
    loop {
        let fresh46 = size;
        size = size - 1;
        if !(fresh46 > 0 as ::core::ffi::c_int) {
            break;
        }
        if NeedsFilter2_C(p, hstride, thresh2, ithresh) != 0 {
            if Hev(p, hstride, hev_thresh) != 0 {
                DoFilter2_C(p, hstride);
            } else {
                DoFilter6_C(p, hstride);
            }
        }
        p = p.offset(vstride as isize);
    }
}
#[inline]
unsafe extern "C" fn FilterLoop24_C(
    mut p: *mut uint8_t,
    mut hstride: ::core::ffi::c_int,
    mut vstride: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
    mut ithresh: ::core::ffi::c_int,
    mut hev_thresh: ::core::ffi::c_int,
) {
    let thresh2: ::core::ffi::c_int = 2 as ::core::ffi::c_int * thresh + 1 as ::core::ffi::c_int;
    loop {
        let fresh45 = size;
        size = size - 1;
        if !(fresh45 > 0 as ::core::ffi::c_int) {
            break;
        }
        if NeedsFilter2_C(p, hstride, thresh2, ithresh) != 0 {
            if Hev(p, hstride, hev_thresh) != 0 {
                DoFilter2_C(p, hstride);
            } else {
                DoFilter4_C(p, hstride);
            }
        }
        p = p.offset(vstride as isize);
    }
}
unsafe extern "C" fn VFilter16_C(
    mut p: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
    mut ithresh: ::core::ffi::c_int,
    mut hev_thresh: ::core::ffi::c_int,
) {
    FilterLoop26_C(
        p,
        stride,
        1 as ::core::ffi::c_int,
        16 as ::core::ffi::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
}
unsafe extern "C" fn HFilter16_C(
    mut p: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
    mut ithresh: ::core::ffi::c_int,
    mut hev_thresh: ::core::ffi::c_int,
) {
    FilterLoop26_C(
        p,
        1 as ::core::ffi::c_int,
        stride,
        16 as ::core::ffi::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
}
unsafe extern "C" fn VFilter16i_C(
    mut p: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
    mut ithresh: ::core::ffi::c_int,
    mut hev_thresh: ::core::ffi::c_int,
) {
    let mut k: ::core::ffi::c_int = 0;
    k = 3 as ::core::ffi::c_int;
    while k > 0 as ::core::ffi::c_int {
        p = p.offset((4 as ::core::ffi::c_int * stride) as isize);
        FilterLoop24_C(
            p,
            stride,
            1 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            thresh,
            ithresh,
            hev_thresh,
        );
        k -= 1;
    }
}
unsafe extern "C" fn HFilter16i_C(
    mut p: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
    mut ithresh: ::core::ffi::c_int,
    mut hev_thresh: ::core::ffi::c_int,
) {
    let mut k: ::core::ffi::c_int = 0;
    k = 3 as ::core::ffi::c_int;
    while k > 0 as ::core::ffi::c_int {
        p = p.offset(4 as ::core::ffi::c_int as isize);
        FilterLoop24_C(
            p,
            1 as ::core::ffi::c_int,
            stride,
            16 as ::core::ffi::c_int,
            thresh,
            ithresh,
            hev_thresh,
        );
        k -= 1;
    }
}
unsafe extern "C" fn VFilter8_C(
    mut u: *mut uint8_t,
    mut v: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
    mut ithresh: ::core::ffi::c_int,
    mut hev_thresh: ::core::ffi::c_int,
) {
    FilterLoop26_C(
        u,
        stride,
        1 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
    FilterLoop26_C(
        v,
        stride,
        1 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
}
unsafe extern "C" fn HFilter8_C(
    mut u: *mut uint8_t,
    mut v: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
    mut ithresh: ::core::ffi::c_int,
    mut hev_thresh: ::core::ffi::c_int,
) {
    FilterLoop26_C(
        u,
        1 as ::core::ffi::c_int,
        stride,
        8 as ::core::ffi::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
    FilterLoop26_C(
        v,
        1 as ::core::ffi::c_int,
        stride,
        8 as ::core::ffi::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
}
unsafe extern "C" fn VFilter8i_C(
    mut u: *mut uint8_t,
    mut v: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
    mut ithresh: ::core::ffi::c_int,
    mut hev_thresh: ::core::ffi::c_int,
) {
    FilterLoop24_C(
        u.offset((4 as ::core::ffi::c_int * stride) as isize),
        stride,
        1 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
    FilterLoop24_C(
        v.offset((4 as ::core::ffi::c_int * stride) as isize),
        stride,
        1 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
}
unsafe extern "C" fn HFilter8i_C(
    mut u: *mut uint8_t,
    mut v: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
    mut ithresh: ::core::ffi::c_int,
    mut hev_thresh: ::core::ffi::c_int,
) {
    FilterLoop24_C(
        u.offset(4 as ::core::ffi::c_int as isize),
        1 as ::core::ffi::c_int,
        stride,
        8 as ::core::ffi::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
    FilterLoop24_C(
        v.offset(4 as ::core::ffi::c_int as isize),
        1 as ::core::ffi::c_int,
        stride,
        8 as ::core::ffi::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
}
unsafe extern "C" fn DitherCombine8x8_C(
    mut dither: *const uint8_t,
    mut dst: *mut uint8_t,
    mut dst_stride: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < 8 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            let delta0: ::core::ffi::c_int =
                *dither.offset(i as isize) as ::core::ffi::c_int - VP8_DITHER_AMP_CENTER;
            let delta1: ::core::ffi::c_int =
                delta0 + VP8_DITHER_DESCALE_ROUNDER >> VP8_DITHER_DESCALE;
            *dst.offset(i as isize) =
                clip_8b(*dst.offset(i as isize) as ::core::ffi::c_int + delta1);
            i += 1;
        }
        dst = dst.offset(dst_stride as isize);
        dither = dither.offset(8 as ::core::ffi::c_int as isize);
        j += 1;
    }
}
#[no_mangle]
pub static mut VP8Transform: VP8DecIdct2 = None;
#[no_mangle]
pub static mut VP8TransformAC3: VP8DecIdct = None;
#[no_mangle]
pub static mut VP8TransformUV: VP8DecIdct = None;
#[no_mangle]
pub static mut VP8TransformDC: VP8DecIdct = None;
#[no_mangle]
pub static mut VP8TransformDCUV: VP8DecIdct = None;
#[no_mangle]
pub static mut VP8VFilter16: VP8LumaFilterFunc = None;
#[no_mangle]
pub static mut VP8HFilter16: VP8LumaFilterFunc = None;
#[no_mangle]
pub static mut VP8VFilter8: VP8ChromaFilterFunc = None;
#[no_mangle]
pub static mut VP8HFilter8: VP8ChromaFilterFunc = None;
#[no_mangle]
pub static mut VP8VFilter16i: VP8LumaFilterFunc = None;
#[no_mangle]
pub static mut VP8HFilter16i: VP8LumaFilterFunc = None;
#[no_mangle]
pub static mut VP8VFilter8i: VP8ChromaFilterFunc = None;
#[no_mangle]
pub static mut VP8HFilter8i: VP8ChromaFilterFunc = None;
#[no_mangle]
pub static mut VP8SimpleVFilter16: VP8SimpleFilterFunc = None;
#[no_mangle]
pub static mut VP8SimpleHFilter16: VP8SimpleFilterFunc = None;
#[no_mangle]
pub static mut VP8SimpleVFilter16i: VP8SimpleFilterFunc = None;
#[no_mangle]
pub static mut VP8SimpleHFilter16i: VP8SimpleFilterFunc = None;
#[no_mangle]
pub static mut VP8DitherCombine8x8: Option<
    unsafe extern "C" fn(*const uint8_t, *mut uint8_t, ::core::ffi::c_int) -> (),
> = None;
unsafe extern "C" fn VP8DspInit_body() {
    VP8InitClipTables();
    VP8TransformWHT =
        Some(TransformWHT_C as unsafe extern "C" fn(*const int16_t, *mut int16_t) -> ())
            as Option<unsafe extern "C" fn(*const int16_t, *mut int16_t) -> ()>;
    VP8Transform = Some(
        TransformTwo_C
            as unsafe extern "C" fn(*const int16_t, *mut uint8_t, ::core::ffi::c_int) -> (),
    ) as VP8DecIdct2;
    VP8TransformDC = Some(TransformDC_C as unsafe extern "C" fn(*const int16_t, *mut uint8_t) -> ())
        as VP8DecIdct;
    VP8TransformAC3 =
        Some(TransformAC3_C as unsafe extern "C" fn(*const int16_t, *mut uint8_t) -> ())
            as VP8DecIdct;
    VP8TransformUV = Some(TransformUV_C as unsafe extern "C" fn(*const int16_t, *mut uint8_t) -> ())
        as VP8DecIdct;
    VP8TransformDCUV =
        Some(TransformDCUV_C as unsafe extern "C" fn(*const int16_t, *mut uint8_t) -> ())
            as VP8DecIdct;
    VP8VFilter16 = Some(
        VFilter16_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    ) as VP8LumaFilterFunc;
    VP8VFilter16i = Some(
        VFilter16i_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    ) as VP8LumaFilterFunc;
    VP8HFilter16 = Some(
        HFilter16_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    ) as VP8LumaFilterFunc;
    VP8VFilter8 = Some(
        VFilter8_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    ) as VP8ChromaFilterFunc;
    VP8VFilter8i = Some(
        VFilter8i_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    ) as VP8ChromaFilterFunc;
    VP8SimpleVFilter16 = Some(
        SimpleVFilter16_C
            as unsafe extern "C" fn(*mut uint8_t, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    ) as VP8SimpleFilterFunc;
    VP8SimpleHFilter16 = Some(
        SimpleHFilter16_C
            as unsafe extern "C" fn(*mut uint8_t, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    ) as VP8SimpleFilterFunc;
    VP8SimpleVFilter16i = Some(
        SimpleVFilter16i_C
            as unsafe extern "C" fn(*mut uint8_t, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    ) as VP8SimpleFilterFunc;
    VP8SimpleHFilter16i = Some(
        SimpleHFilter16i_C
            as unsafe extern "C" fn(*mut uint8_t, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    ) as VP8SimpleFilterFunc;
    VP8HFilter16i = Some(
        HFilter16i_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    ) as VP8LumaFilterFunc;
    VP8HFilter8 = Some(
        HFilter8_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    ) as VP8ChromaFilterFunc;
    VP8HFilter8i = Some(
        HFilter8i_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    ) as VP8ChromaFilterFunc;
    VP8PredLuma4[0 as ::core::ffi::c_int as usize] =
        Some(DC4_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredLuma4[1 as ::core::ffi::c_int as usize] =
        Some(TM4_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredLuma4[2 as ::core::ffi::c_int as usize] =
        Some(VE4_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredLuma4[4 as ::core::ffi::c_int as usize] =
        Some(RD4_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredLuma4[6 as ::core::ffi::c_int as usize] =
        Some(LD4_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredLuma4[3 as ::core::ffi::c_int as usize] =
        Some(HE4_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredLuma4[5 as ::core::ffi::c_int as usize] =
        Some(VR4_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredLuma4[7 as ::core::ffi::c_int as usize] =
        Some(VL4_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredLuma4[8 as ::core::ffi::c_int as usize] =
        Some(HD4_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredLuma4[9 as ::core::ffi::c_int as usize] =
        Some(HU4_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredLuma16[0 as ::core::ffi::c_int as usize] =
        Some(DC16_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredLuma16[1 as ::core::ffi::c_int as usize] =
        Some(TM16_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredLuma16[2 as ::core::ffi::c_int as usize] =
        Some(VE16_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredLuma16[3 as ::core::ffi::c_int as usize] =
        Some(HE16_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredLuma16[4 as ::core::ffi::c_int as usize] =
        Some(DC16NoTop_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredLuma16[5 as ::core::ffi::c_int as usize] =
        Some(DC16NoLeft_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredLuma16[6 as ::core::ffi::c_int as usize] =
        Some(DC16NoTopLeft_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredChroma8[0 as ::core::ffi::c_int as usize] =
        Some(DC8uv_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredChroma8[1 as ::core::ffi::c_int as usize] =
        Some(TM8uv_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredChroma8[2 as ::core::ffi::c_int as usize] =
        Some(VE8uv_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredChroma8[3 as ::core::ffi::c_int as usize] =
        Some(HE8uv_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredChroma8[4 as ::core::ffi::c_int as usize] =
        Some(DC8uvNoTop_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredChroma8[5 as ::core::ffi::c_int as usize] =
        Some(DC8uvNoLeft_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8PredChroma8[6 as ::core::ffi::c_int as usize] =
        Some(DC8uvNoTopLeft_C as unsafe extern "C" fn(*mut uint8_t) -> ()) as VP8PredFunc;
    VP8DitherCombine8x8 = Some(
        DitherCombine8x8_C
            as unsafe extern "C" fn(*const uint8_t, *mut uint8_t, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*const uint8_t, *mut uint8_t, ::core::ffi::c_int) -> ()>;
    if VP8GetCPUInfo.is_some() {
        if VP8GetCPUInfo.expect("non-null function pointer")(kSSE2) != 0 {
            VP8DspInitSSE2();
        }
    }
    '_c2rust_label: {
        if VP8TransformWHT.is_some() {
        } else {
            __assert_fail(
                b"VP8TransformWHT != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                845 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if VP8Transform.is_some() {
        } else {
            __assert_fail(
                b"VP8Transform != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                846 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if VP8TransformDC.is_some() {
        } else {
            __assert_fail(
                b"VP8TransformDC != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                847 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if VP8TransformAC3.is_some() {
        } else {
            __assert_fail(
                b"VP8TransformAC3 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                848 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if VP8TransformUV.is_some() {
        } else {
            __assert_fail(
                b"VP8TransformUV != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                849 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if VP8TransformDCUV.is_some() {
        } else {
            __assert_fail(
                b"VP8TransformDCUV != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                850 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_5: {
        if VP8VFilter16.is_some() {
        } else {
            __assert_fail(
                b"VP8VFilter16 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                851 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_6: {
        if VP8HFilter16.is_some() {
        } else {
            __assert_fail(
                b"VP8HFilter16 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                852 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_7: {
        if VP8VFilter8.is_some() {
        } else {
            __assert_fail(
                b"VP8VFilter8 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                853 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_8: {
        if VP8HFilter8.is_some() {
        } else {
            __assert_fail(
                b"VP8HFilter8 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                854 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_9: {
        if VP8VFilter16i.is_some() {
        } else {
            __assert_fail(
                b"VP8VFilter16i != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                855 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_10: {
        if VP8HFilter16i.is_some() {
        } else {
            __assert_fail(
                b"VP8HFilter16i != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                856 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_11: {
        if VP8VFilter8i.is_some() {
        } else {
            __assert_fail(
                b"VP8VFilter8i != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                857 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_12: {
        if VP8HFilter8i.is_some() {
        } else {
            __assert_fail(
                b"VP8HFilter8i != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                858 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_13: {
        if VP8SimpleVFilter16.is_some() {
        } else {
            __assert_fail(
                b"VP8SimpleVFilter16 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                859 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_14: {
        if VP8SimpleHFilter16.is_some() {
        } else {
            __assert_fail(
                b"VP8SimpleHFilter16 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                860 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_15: {
        if VP8SimpleVFilter16i.is_some() {
        } else {
            __assert_fail(
                b"VP8SimpleVFilter16i != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                861 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_16: {
        if VP8SimpleHFilter16i.is_some() {
        } else {
            __assert_fail(
                b"VP8SimpleHFilter16i != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                862 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_17: {
        if VP8PredLuma4[0 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredLuma4[0] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                863 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_18: {
        if VP8PredLuma4[1 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredLuma4[1] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                864 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_19: {
        if VP8PredLuma4[2 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredLuma4[2] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                865 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_20: {
        if VP8PredLuma4[3 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredLuma4[3] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                866 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_21: {
        if VP8PredLuma4[4 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredLuma4[4] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                867 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_22: {
        if VP8PredLuma4[5 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredLuma4[5] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                868 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_23: {
        if VP8PredLuma4[6 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredLuma4[6] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                869 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_24: {
        if VP8PredLuma4[7 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredLuma4[7] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                870 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_25: {
        if VP8PredLuma4[8 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredLuma4[8] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                871 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_26: {
        if VP8PredLuma4[9 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredLuma4[9] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                872 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_27: {
        if VP8PredLuma16[0 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredLuma16[0] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                873 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_28: {
        if VP8PredLuma16[1 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredLuma16[1] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                874 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_29: {
        if VP8PredLuma16[2 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredLuma16[2] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                875 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_30: {
        if VP8PredLuma16[3 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredLuma16[3] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                876 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_31: {
        if VP8PredLuma16[4 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredLuma16[4] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                877 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_32: {
        if VP8PredLuma16[5 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredLuma16[5] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                878 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_33: {
        if VP8PredLuma16[6 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredLuma16[6] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                879 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_34: {
        if VP8PredChroma8[0 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredChroma8[0] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                880 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_35: {
        if VP8PredChroma8[1 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredChroma8[1] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                881 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_36: {
        if VP8PredChroma8[2 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredChroma8[2] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                882 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_37: {
        if VP8PredChroma8[3 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredChroma8[3] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                883 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_38: {
        if VP8PredChroma8[4 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredChroma8[4] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                884 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_39: {
        if VP8PredChroma8[5 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredChroma8[5] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                885 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_40: {
        if VP8PredChroma8[6 as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"VP8PredChroma8[6] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                886 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_41: {
        if VP8DitherCombine8x8.is_some() {
        } else {
            __assert_fail(
                b"VP8DitherCombine8x8 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/dec.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                887 as ::core::ffi::c_uint,
                b"void VP8DspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8DspInit() {
    static mut VP8DspInit_body_last_cpuinfo_used: VP8CPUInfo = None;
    if !(VP8DspInit_body_last_cpuinfo_used == VP8GetCPUInfo) {
        VP8DspInit_body();
        ::core::ptr::write_volatile(
            &mut VP8DspInit_body_last_cpuinfo_used as *mut VP8CPUInfo,
            VP8GetCPUInfo,
        );
    }
}
