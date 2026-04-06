extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    static mut VP8GetCPUInfo: VP8CPUInfo;
    fn VP8FiltersInitSSE2();
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const WEBP_FILTER_FAST: C2RustUnnamed = 6;
pub const WEBP_FILTER_BEST: C2RustUnnamed = 5;
pub const WEBP_FILTER_LAST: C2RustUnnamed = 4;
pub const WEBP_FILTER_GRADIENT: C2RustUnnamed = 3;
pub const WEBP_FILTER_VERTICAL: C2RustUnnamed = 2;
pub const WEBP_FILTER_HORIZONTAL: C2RustUnnamed = 1;
pub const WEBP_FILTER_NONE: C2RustUnnamed = 0;
pub type WebPFilterFunc = Option<
    unsafe extern "C" fn(
        *const uint8_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut uint8_t,
    ) -> (),
>;
pub type WebPUnfilterFunc = Option<
    unsafe extern "C" fn(*const uint8_t, *const uint8_t, *mut uint8_t, ::core::ffi::c_int) -> (),
>;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn PredictLine_C(
    mut src: *const uint8_t,
    mut pred: *const uint8_t,
    mut dst: *mut uint8_t,
    mut length: ::core::ffi::c_int,
    mut inverse: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    if inverse != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < length {
            *dst.offset(i as isize) = (*src.offset(i as isize) as ::core::ffi::c_int
                + *pred.offset(i as isize) as ::core::ffi::c_int)
                as uint8_t;
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < length {
            *dst.offset(i as isize) = (*src.offset(i as isize) as ::core::ffi::c_int
                - *pred.offset(i as isize) as ::core::ffi::c_int)
                as uint8_t;
            i += 1;
        }
    };
}
#[inline]
unsafe extern "C" fn DoHorizontalFilter_C(
    mut in_0: *const uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
    mut row: ::core::ffi::c_int,
    mut num_rows: ::core::ffi::c_int,
    mut inverse: ::core::ffi::c_int,
    mut out: *mut uint8_t,
) {
    let mut preds: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let start_offset: size_t = (row * stride) as size_t;
    let last_row: ::core::ffi::c_int = row + num_rows;
    '_c2rust_label: {
        if !in_0.is_null() {
        } else {
            __assert_fail(
                b"(in) != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                52 as ::core::ffi::c_uint,
                b"void DoHorizontalFilter_C(const uint8_t *, int, int, int, int, int, int, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !out.is_null() {
        } else {
            __assert_fail(
                b"(out) != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                52 as ::core::ffi::c_uint,
                b"void DoHorizontalFilter_C(const uint8_t *, int, int, int, int, int, int, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if width > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"width > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                52 as ::core::ffi::c_uint,
                b"void DoHorizontalFilter_C(const uint8_t *, int, int, int, int, int, int, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if height > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"height > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                52 as ::core::ffi::c_uint,
                b"void DoHorizontalFilter_C(const uint8_t *, int, int, int, int, int, int, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if stride >= width {
        } else {
            __assert_fail(
                b"stride >= width\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                52 as ::core::ffi::c_uint,
                b"void DoHorizontalFilter_C(const uint8_t *, int, int, int, int, int, int, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if row >= 0 as ::core::ffi::c_int
            && num_rows > 0 as ::core::ffi::c_int
            && row + num_rows <= height
        {
        } else {
            __assert_fail(
                b"row >= 0 && num_rows > 0 && row + num_rows <= height\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                52 as ::core::ffi::c_uint,
                b"void DoHorizontalFilter_C(const uint8_t *, int, int, int, int, int, int, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    in_0 = in_0.offset(start_offset as isize);
    out = out.offset(start_offset as isize);
    preds = if inverse != 0 {
        out as *const uint8_t
    } else {
        in_0
    };
    if row == 0 as ::core::ffi::c_int {
        *out.offset(0 as ::core::ffi::c_int as isize) =
            *in_0.offset(0 as ::core::ffi::c_int as isize);
        PredictLine_C(
            in_0.offset(1 as ::core::ffi::c_int as isize),
            preds,
            out.offset(1 as ::core::ffi::c_int as isize),
            width - 1 as ::core::ffi::c_int,
            inverse,
        );
        row = 1 as ::core::ffi::c_int;
        preds = preds.offset(stride as isize);
        in_0 = in_0.offset(stride as isize);
        out = out.offset(stride as isize);
    }
    while row < last_row {
        PredictLine_C(
            in_0,
            preds.offset(-(stride as isize)),
            out,
            1 as ::core::ffi::c_int,
            inverse,
        );
        PredictLine_C(
            in_0.offset(1 as ::core::ffi::c_int as isize),
            preds,
            out.offset(1 as ::core::ffi::c_int as isize),
            width - 1 as ::core::ffi::c_int,
            inverse,
        );
        row += 1;
        preds = preds.offset(stride as isize);
        in_0 = in_0.offset(stride as isize);
        out = out.offset(stride as isize);
    }
}
#[inline]
unsafe extern "C" fn DoVerticalFilter_C(
    mut in_0: *const uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
    mut row: ::core::ffi::c_int,
    mut num_rows: ::core::ffi::c_int,
    mut inverse: ::core::ffi::c_int,
    mut out: *mut uint8_t,
) {
    let mut preds: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let start_offset: size_t = (row * stride) as size_t;
    let last_row: ::core::ffi::c_int = row + num_rows;
    '_c2rust_label: {
        if !in_0.is_null() {
        } else {
            __assert_fail(
                b"(in) != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_uint,
                b"void DoVerticalFilter_C(const uint8_t *, int, int, int, int, int, int, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !out.is_null() {
        } else {
            __assert_fail(
                b"(out) != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_uint,
                b"void DoVerticalFilter_C(const uint8_t *, int, int, int, int, int, int, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if width > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"width > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_uint,
                b"void DoVerticalFilter_C(const uint8_t *, int, int, int, int, int, int, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if height > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"height > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_uint,
                b"void DoVerticalFilter_C(const uint8_t *, int, int, int, int, int, int, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if stride >= width {
        } else {
            __assert_fail(
                b"stride >= width\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_uint,
                b"void DoVerticalFilter_C(const uint8_t *, int, int, int, int, int, int, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if row >= 0 as ::core::ffi::c_int
            && num_rows > 0 as ::core::ffi::c_int
            && row + num_rows <= height
        {
        } else {
            __assert_fail(
                b"row >= 0 && num_rows > 0 && row + num_rows <= height\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_uint,
                b"void DoVerticalFilter_C(const uint8_t *, int, int, int, int, int, int, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    in_0 = in_0.offset(start_offset as isize);
    out = out.offset(start_offset as isize);
    preds = if inverse != 0 {
        out as *const uint8_t
    } else {
        in_0
    };
    if row == 0 as ::core::ffi::c_int {
        *out.offset(0 as ::core::ffi::c_int as isize) =
            *in_0.offset(0 as ::core::ffi::c_int as isize);
        PredictLine_C(
            in_0.offset(1 as ::core::ffi::c_int as isize),
            preds,
            out.offset(1 as ::core::ffi::c_int as isize),
            width - 1 as ::core::ffi::c_int,
            inverse,
        );
        row = 1 as ::core::ffi::c_int;
        in_0 = in_0.offset(stride as isize);
        out = out.offset(stride as isize);
    } else {
        preds = preds.offset(-(stride as isize));
    }
    while row < last_row {
        PredictLine_C(in_0, preds, out, width, inverse);
        row += 1;
        preds = preds.offset(stride as isize);
        in_0 = in_0.offset(stride as isize);
        out = out.offset(stride as isize);
    }
}
#[inline]
unsafe extern "C" fn GradientPredictor_C(
    mut a: uint8_t,
    mut b: uint8_t,
    mut c: uint8_t,
) -> ::core::ffi::c_int {
    let g: ::core::ffi::c_int =
        a as ::core::ffi::c_int + b as ::core::ffi::c_int - c as ::core::ffi::c_int;
    return if g & !(0xff as ::core::ffi::c_int) == 0 as ::core::ffi::c_int {
        g
    } else if g < 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        255 as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn DoGradientFilter_C(
    mut in_0: *const uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
    mut row: ::core::ffi::c_int,
    mut num_rows: ::core::ffi::c_int,
    mut inverse: ::core::ffi::c_int,
    mut out: *mut uint8_t,
) {
    let mut preds: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let start_offset: size_t = (row * stride) as size_t;
    let last_row: ::core::ffi::c_int = row + num_rows;
    '_c2rust_label: {
        if !in_0.is_null() {
        } else {
            __assert_fail(
                b"(in) != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                134 as ::core::ffi::c_uint,
                b"void DoGradientFilter_C(const uint8_t *, int, int, int, int, int, int, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !out.is_null() {
        } else {
            __assert_fail(
                b"(out) != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                134 as ::core::ffi::c_uint,
                b"void DoGradientFilter_C(const uint8_t *, int, int, int, int, int, int, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if width > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"width > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                134 as ::core::ffi::c_uint,
                b"void DoGradientFilter_C(const uint8_t *, int, int, int, int, int, int, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if height > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"height > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                134 as ::core::ffi::c_uint,
                b"void DoGradientFilter_C(const uint8_t *, int, int, int, int, int, int, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if stride >= width {
        } else {
            __assert_fail(
                b"stride >= width\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                134 as ::core::ffi::c_uint,
                b"void DoGradientFilter_C(const uint8_t *, int, int, int, int, int, int, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if row >= 0 as ::core::ffi::c_int
            && num_rows > 0 as ::core::ffi::c_int
            && row + num_rows <= height
        {
        } else {
            __assert_fail(
                b"row >= 0 && num_rows > 0 && row + num_rows <= height\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                134 as ::core::ffi::c_uint,
                b"void DoGradientFilter_C(const uint8_t *, int, int, int, int, int, int, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    in_0 = in_0.offset(start_offset as isize);
    out = out.offset(start_offset as isize);
    preds = if inverse != 0 {
        out as *const uint8_t
    } else {
        in_0
    };
    if row == 0 as ::core::ffi::c_int {
        *out.offset(0 as ::core::ffi::c_int as isize) =
            *in_0.offset(0 as ::core::ffi::c_int as isize);
        PredictLine_C(
            in_0.offset(1 as ::core::ffi::c_int as isize),
            preds,
            out.offset(1 as ::core::ffi::c_int as isize),
            width - 1 as ::core::ffi::c_int,
            inverse,
        );
        row = 1 as ::core::ffi::c_int;
        preds = preds.offset(stride as isize);
        in_0 = in_0.offset(stride as isize);
        out = out.offset(stride as isize);
    }
    while row < last_row {
        let mut w: ::core::ffi::c_int = 0;
        PredictLine_C(
            in_0,
            preds.offset(-(stride as isize)),
            out,
            1 as ::core::ffi::c_int,
            inverse,
        );
        w = 1 as ::core::ffi::c_int;
        while w < width {
            let pred: ::core::ffi::c_int = GradientPredictor_C(
                *preds.offset((w - 1 as ::core::ffi::c_int) as isize),
                *preds.offset((w - stride) as isize),
                *preds.offset((w - stride - 1 as ::core::ffi::c_int) as isize),
            ) as ::core::ffi::c_int;
            *out.offset(w as isize) = (*in_0.offset(w as isize) as ::core::ffi::c_int
                + (if inverse != 0 { pred } else { -pred }))
                as uint8_t;
            w += 1;
        }
        row += 1;
        preds = preds.offset(stride as isize);
        in_0 = in_0.offset(stride as isize);
        out = out.offset(stride as isize);
    }
}
unsafe extern "C" fn HorizontalFilter_C(
    mut data: *const uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
    mut filtered_data: *mut uint8_t,
) {
    DoHorizontalFilter_C(
        data,
        width,
        height,
        stride,
        0 as ::core::ffi::c_int,
        height,
        0 as ::core::ffi::c_int,
        filtered_data,
    );
}
unsafe extern "C" fn VerticalFilter_C(
    mut data: *const uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
    mut filtered_data: *mut uint8_t,
) {
    DoVerticalFilter_C(
        data,
        width,
        height,
        stride,
        0 as ::core::ffi::c_int,
        height,
        0 as ::core::ffi::c_int,
        filtered_data,
    );
}
unsafe extern "C" fn GradientFilter_C(
    mut data: *const uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
    mut filtered_data: *mut uint8_t,
) {
    DoGradientFilter_C(
        data,
        width,
        height,
        stride,
        0 as ::core::ffi::c_int,
        height,
        0 as ::core::ffi::c_int,
        filtered_data,
    );
}
unsafe extern "C" fn HorizontalUnfilter_C(
    mut prev: *const uint8_t,
    mut in_0: *const uint8_t,
    mut out: *mut uint8_t,
    mut width: ::core::ffi::c_int,
) {
    let mut pred: uint8_t = (if prev.is_null() {
        0 as ::core::ffi::c_int
    } else {
        *prev.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
    }) as uint8_t;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < width {
        *out.offset(i as isize) = (pred as ::core::ffi::c_int
            + *in_0.offset(i as isize) as ::core::ffi::c_int)
            as uint8_t;
        pred = *out.offset(i as isize);
        i += 1;
    }
}
unsafe extern "C" fn VerticalUnfilter_C(
    mut prev: *const uint8_t,
    mut in_0: *const uint8_t,
    mut out: *mut uint8_t,
    mut width: ::core::ffi::c_int,
) {
    if prev.is_null() {
        HorizontalUnfilter_C(::core::ptr::null::<uint8_t>(), in_0, out, width);
    } else {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < width {
            *out.offset(i as isize) = (*prev.offset(i as isize) as ::core::ffi::c_int
                + *in_0.offset(i as isize) as ::core::ffi::c_int)
                as uint8_t;
            i += 1;
        }
    };
}
unsafe extern "C" fn GradientUnfilter_C(
    mut prev: *const uint8_t,
    mut in_0: *const uint8_t,
    mut out: *mut uint8_t,
    mut width: ::core::ffi::c_int,
) {
    if prev.is_null() {
        HorizontalUnfilter_C(::core::ptr::null::<uint8_t>(), in_0, out, width);
    } else {
        let mut top: uint8_t = *prev.offset(0 as ::core::ffi::c_int as isize);
        let mut top_left: uint8_t = top;
        let mut left: uint8_t = top;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < width {
            top = *prev.offset(i as isize);
            left = (*in_0.offset(i as isize) as ::core::ffi::c_int
                + GradientPredictor_C(left, top, top_left)) as uint8_t;
            top_left = top;
            *out.offset(i as isize) = left;
            i += 1;
        }
    };
}
#[no_mangle]
pub static mut WebPFilters: [WebPFilterFunc; 4] = [None; 4];
#[no_mangle]
pub static mut WebPUnfilters: [WebPUnfilterFunc; 4] = [None; 4];
#[no_mangle]
pub unsafe extern "C" fn VP8FiltersInit() {
    static mut VP8FiltersInit_body_last_cpuinfo_used: VP8CPUInfo = None;
    if !(VP8FiltersInit_body_last_cpuinfo_used == VP8GetCPUInfo) {
        VP8FiltersInit_body();
        ::core::ptr::write_volatile(
            &mut VP8FiltersInit_body_last_cpuinfo_used as *mut VP8CPUInfo,
            VP8GetCPUInfo,
        );
    }
}
unsafe extern "C" fn VP8FiltersInit_body() {
    WebPUnfilters[WEBP_FILTER_NONE as ::core::ffi::c_int as usize] = None;
    WebPUnfilters[WEBP_FILTER_HORIZONTAL as ::core::ffi::c_int as usize] = Some(
        HorizontalUnfilter_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as WebPUnfilterFunc;
    WebPUnfilters[WEBP_FILTER_VERTICAL as ::core::ffi::c_int as usize] = Some(
        VerticalUnfilter_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> (),
    ) as WebPUnfilterFunc;
    WebPUnfilters[WEBP_FILTER_GRADIENT as ::core::ffi::c_int as usize] = Some(
        GradientUnfilter_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> (),
    ) as WebPUnfilterFunc;
    WebPFilters[WEBP_FILTER_NONE as ::core::ffi::c_int as usize] = None;
    WebPFilters[WEBP_FILTER_HORIZONTAL as ::core::ffi::c_int as usize] = Some(
        HorizontalFilter_C
            as unsafe extern "C" fn(
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut uint8_t,
            ) -> (),
    ) as WebPFilterFunc;
    WebPFilters[WEBP_FILTER_VERTICAL as ::core::ffi::c_int as usize] = Some(
        VerticalFilter_C
            as unsafe extern "C" fn(
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut uint8_t,
            ) -> (),
    ) as WebPFilterFunc;
    WebPFilters[WEBP_FILTER_GRADIENT as ::core::ffi::c_int as usize] = Some(
        GradientFilter_C
            as unsafe extern "C" fn(
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut uint8_t,
            ) -> (),
    ) as WebPFilterFunc;
    if VP8GetCPUInfo.is_some() {
        if VP8GetCPUInfo.expect("non-null function pointer")(kSSE2) != 0 {
            VP8FiltersInitSSE2();
        }
    }
    '_c2rust_label: {
        if WebPUnfilters[WEBP_FILTER_HORIZONTAL as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"WebPUnfilters[WEBP_FILTER_HORIZONTAL] != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                282 as ::core::ffi::c_uint,
                b"void VP8FiltersInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if WebPUnfilters[WEBP_FILTER_VERTICAL as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"WebPUnfilters[WEBP_FILTER_VERTICAL] != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                283 as ::core::ffi::c_uint,
                b"void VP8FiltersInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if WebPUnfilters[WEBP_FILTER_GRADIENT as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"WebPUnfilters[WEBP_FILTER_GRADIENT] != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                284 as ::core::ffi::c_uint,
                b"void VP8FiltersInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if WebPFilters[WEBP_FILTER_HORIZONTAL as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"WebPFilters[WEBP_FILTER_HORIZONTAL] != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                285 as ::core::ffi::c_uint,
                b"void VP8FiltersInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if WebPFilters[WEBP_FILTER_VERTICAL as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"WebPFilters[WEBP_FILTER_VERTICAL] != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                286 as ::core::ffi::c_uint,
                b"void VP8FiltersInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if WebPFilters[WEBP_FILTER_GRADIENT as ::core::ffi::c_int as usize].is_some() {
        } else {
            __assert_fail(
                b"WebPFilters[WEBP_FILTER_GRADIENT] != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/filters.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                287 as ::core::ffi::c_uint,
                b"void VP8FiltersInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
}
