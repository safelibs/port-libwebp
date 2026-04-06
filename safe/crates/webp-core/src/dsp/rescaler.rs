extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    static mut VP8GetCPUInfo: VP8CPUInfo;
    fn WebPRescalerDspInitSSE2();
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPRescaler {
    pub x_expand: ::core::ffi::c_int,
    pub y_expand: ::core::ffi::c_int,
    pub num_channels: ::core::ffi::c_int,
    pub fx_scale: uint32_t,
    pub fy_scale: uint32_t,
    pub fxy_scale: uint32_t,
    pub y_accum: ::core::ffi::c_int,
    pub y_add: ::core::ffi::c_int,
    pub y_sub: ::core::ffi::c_int,
    pub x_add: ::core::ffi::c_int,
    pub x_sub: ::core::ffi::c_int,
    pub src_width: ::core::ffi::c_int,
    pub src_height: ::core::ffi::c_int,
    pub dst_width: ::core::ffi::c_int,
    pub dst_height: ::core::ffi::c_int,
    pub src_y: ::core::ffi::c_int,
    pub dst_y: ::core::ffi::c_int,
    pub dst: *mut uint8_t,
    pub dst_stride: ::core::ffi::c_int,
    pub irow: *mut rescaler_t,
    pub frow: *mut rescaler_t,
}
pub type rescaler_t = uint32_t;
pub type WebPRescalerImportRowFunc =
    Option<unsafe extern "C" fn(*mut WebPRescaler, *const uint8_t) -> ()>;
pub type WebPRescalerExportRowFunc = Option<unsafe extern "C" fn(*mut WebPRescaler) -> ()>;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const WEBP_RESCALER_RFIX: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const WEBP_RESCALER_ONE: ::core::ffi::c_ulonglong =
    (1 as ::core::ffi::c_ulonglong) << WEBP_RESCALER_RFIX;
#[inline]
unsafe extern "C" fn WebPRescalerInputDone(rescaler: *const WebPRescaler) -> ::core::ffi::c_int {
    return ((*rescaler).src_y >= (*rescaler).src_height) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn WebPRescalerOutputDone(rescaler: *const WebPRescaler) -> ::core::ffi::c_int {
    return ((*rescaler).dst_y >= (*rescaler).dst_height) as ::core::ffi::c_int;
}
pub const ROUNDER: ::core::ffi::c_ulonglong = WEBP_RESCALER_ONE >> 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerImportRowExpand_C(
    wrk: *mut WebPRescaler,
    mut src: *const uint8_t,
) {
    let x_stride: ::core::ffi::c_int = (*wrk).num_channels;
    let x_out_max: ::core::ffi::c_int = (*wrk).dst_width * (*wrk).num_channels;
    let mut channel: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if WebPRescalerInputDone(wrk) == 0 {
        } else {
            __assert_fail(
                b"!WebPRescalerInputDone(wrk)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                34 as ::core::ffi::c_uint,
                b"void WebPRescalerImportRowExpand_C(WebPRescaler *const, const uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*wrk).x_expand != 0 {
        } else {
            __assert_fail(
                b"wrk->x_expand\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                35 as ::core::ffi::c_uint,
                b"void WebPRescalerImportRowExpand_C(WebPRescaler *const, const uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    channel = 0 as ::core::ffi::c_int;
    while channel < x_stride {
        let mut x_in: ::core::ffi::c_int = channel;
        let mut x_out: ::core::ffi::c_int = channel;
        let mut accum: ::core::ffi::c_int = (*wrk).x_add;
        let mut left: rescaler_t = *src.offset(x_in as isize) as rescaler_t;
        let mut right: rescaler_t = if (*wrk).src_width > 1 as ::core::ffi::c_int {
            *src.offset((x_in + x_stride) as isize) as rescaler_t
        } else {
            left
        };
        x_in += x_stride;
        loop {
            *(*wrk).frow.offset(x_out as isize) = right
                .wrapping_mul((*wrk).x_add as rescaler_t)
                .wrapping_add(left.wrapping_sub(right).wrapping_mul(accum as rescaler_t));
            x_out += x_stride;
            if x_out >= x_out_max {
                break;
            }
            accum -= (*wrk).x_sub;
            if accum < 0 as ::core::ffi::c_int {
                left = right;
                x_in += x_stride;
                '_c2rust_label_1: {
                    if x_in < (*wrk).src_width * x_stride {
                    } else {
                        __assert_fail(
                            b"x_in < wrk->src_width * x_stride\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            53 as ::core::ffi::c_uint,
                            b"void WebPRescalerImportRowExpand_C(WebPRescaler *const, const uint8_t *)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                };
                right = *src.offset(x_in as isize) as rescaler_t;
                accum += (*wrk).x_add;
            }
        }
        '_c2rust_label_2: {
            if (*wrk).x_sub == 0 as ::core::ffi::c_int || accum == 0 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"wrk->x_sub == 0 || accum == 0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    58 as ::core::ffi::c_uint,
                    b"void WebPRescalerImportRowExpand_C(WebPRescaler *const, const uint8_t *)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        channel += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerImportRowShrink_C(
    wrk: *mut WebPRescaler,
    mut src: *const uint8_t,
) {
    let x_stride: ::core::ffi::c_int = (*wrk).num_channels;
    let x_out_max: ::core::ffi::c_int = (*wrk).dst_width * (*wrk).num_channels;
    let mut channel: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if WebPRescalerInputDone(wrk) == 0 {
        } else {
            __assert_fail(
                b"!WebPRescalerInputDone(wrk)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                67 as ::core::ffi::c_uint,
                b"void WebPRescalerImportRowShrink_C(WebPRescaler *const, const uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*wrk).x_expand == 0 {
        } else {
            __assert_fail(
                b"!wrk->x_expand\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                68 as ::core::ffi::c_uint,
                b"void WebPRescalerImportRowShrink_C(WebPRescaler *const, const uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    channel = 0 as ::core::ffi::c_int;
    while channel < x_stride {
        let mut x_in: ::core::ffi::c_int = channel;
        let mut x_out: ::core::ffi::c_int = channel;
        let mut sum: uint32_t = 0 as uint32_t;
        let mut accum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x_out < x_out_max {
            let mut base: uint32_t = 0 as uint32_t;
            accum += (*wrk).x_add;
            while accum > 0 as ::core::ffi::c_int {
                accum -= (*wrk).x_sub;
                '_c2rust_label_1: {
                    if x_in < (*wrk).src_width * x_stride {
                    } else {
                        __assert_fail(
                            b"x_in < wrk->src_width * x_stride\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            79 as ::core::ffi::c_uint,
                            b"void WebPRescalerImportRowShrink_C(WebPRescaler *const, const uint8_t *)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                };
                base = *src.offset(x_in as isize) as uint32_t;
                sum = sum.wrapping_add(base);
                x_in += x_stride;
            }
            let frac: rescaler_t = (base as rescaler_t).wrapping_mul(-accum as rescaler_t);
            *(*wrk).frow.offset(x_out as isize) =
                sum.wrapping_mul((*wrk).x_sub as uint32_t)
                    .wrapping_sub(frac as uint32_t) as rescaler_t;
            sum = (((frac as uint64_t).wrapping_mul((*wrk).fx_scale as uint64_t)
                as ::core::ffi::c_ulonglong)
                .wrapping_add(ROUNDER)
                >> WEBP_RESCALER_RFIX) as ::core::ffi::c_int as uint32_t;
            x_out += x_stride;
        }
        '_c2rust_label_2: {
            if accum == 0 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"accum == 0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    92 as ::core::ffi::c_uint,
                    b"void WebPRescalerImportRowShrink_C(WebPRescaler *const, const uint8_t *)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        channel += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerExportRowExpand_C(wrk: *mut WebPRescaler) {
    let mut x_out: ::core::ffi::c_int = 0;
    let dst: *mut uint8_t = (*wrk).dst;
    let irow: *mut rescaler_t = (*wrk).irow;
    let x_out_max: ::core::ffi::c_int = (*wrk).dst_width * (*wrk).num_channels;
    let frow: *const rescaler_t = (*wrk).frow;
    '_c2rust_label: {
        if WebPRescalerOutputDone(wrk) == 0 {
        } else {
            __assert_fail(
                b"!WebPRescalerOutputDone(wrk)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                105 as ::core::ffi::c_uint,
                b"void WebPRescalerExportRowExpand_C(WebPRescaler *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*wrk).y_accum <= 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"wrk->y_accum <= 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                106 as ::core::ffi::c_uint,
                b"void WebPRescalerExportRowExpand_C(WebPRescaler *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*wrk).y_expand != 0 {
        } else {
            __assert_fail(
                b"wrk->y_expand\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                107 as ::core::ffi::c_uint,
                b"void WebPRescalerExportRowExpand_C(WebPRescaler *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if (*wrk).y_sub != 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"wrk->y_sub != 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                108 as ::core::ffi::c_uint,
                b"void WebPRescalerExportRowExpand_C(WebPRescaler *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*wrk).y_accum == 0 as ::core::ffi::c_int {
        x_out = 0 as ::core::ffi::c_int;
        while x_out < x_out_max {
            let J: uint32_t = *frow.offset(x_out as isize) as uint32_t;
            let v: ::core::ffi::c_int = (((J as uint64_t).wrapping_mul((*wrk).fy_scale as uint64_t)
                as ::core::ffi::c_ulonglong)
                .wrapping_add(ROUNDER)
                >> WEBP_RESCALER_RFIX)
                as ::core::ffi::c_int;
            *dst.offset(x_out as isize) = (if v > 255 as ::core::ffi::c_int {
                255 as ::core::ffi::c_uint
            } else {
                v as uint8_t as ::core::ffi::c_uint
            }) as uint8_t;
            x_out += 1;
        }
    } else {
        let B: uint32_t = ((-(*wrk).y_accum as uint64_t) << WEBP_RESCALER_RFIX)
            .wrapping_div((*wrk).y_sub as uint64_t) as uint32_t;
        let A: uint32_t = WEBP_RESCALER_ONE.wrapping_sub(B as ::core::ffi::c_ulonglong) as uint32_t;
        x_out = 0 as ::core::ffi::c_int;
        while x_out < x_out_max {
            let I: uint64_t = (A as uint64_t)
                .wrapping_mul(*frow.offset(x_out as isize) as uint64_t)
                .wrapping_add(
                    (B as uint64_t).wrapping_mul(*irow.offset(x_out as isize) as uint64_t),
                );
            let J_0: uint32_t = ((I as ::core::ffi::c_ulonglong).wrapping_add(ROUNDER)
                >> WEBP_RESCALER_RFIX) as uint32_t;
            let v_0: ::core::ffi::c_int =
                (((J_0 as uint64_t).wrapping_mul((*wrk).fy_scale as uint64_t)
                    as ::core::ffi::c_ulonglong)
                    .wrapping_add(ROUNDER)
                    >> WEBP_RESCALER_RFIX) as ::core::ffi::c_int;
            *dst.offset(x_out as isize) = (if v_0 > 255 as ::core::ffi::c_int {
                255 as ::core::ffi::c_uint
            } else {
                v_0 as uint8_t as ::core::ffi::c_uint
            }) as uint8_t;
            x_out += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerExportRowShrink_C(wrk: *mut WebPRescaler) {
    let mut x_out: ::core::ffi::c_int = 0;
    let dst: *mut uint8_t = (*wrk).dst;
    let irow: *mut rescaler_t = (*wrk).irow;
    let x_out_max: ::core::ffi::c_int = (*wrk).dst_width * (*wrk).num_channels;
    let frow: *const rescaler_t = (*wrk).frow;
    let yscale: uint32_t = (*wrk).fy_scale.wrapping_mul(-(*wrk).y_accum as uint32_t);
    '_c2rust_label: {
        if WebPRescalerOutputDone(wrk) == 0 {
        } else {
            __assert_fail(
                b"!WebPRescalerOutputDone(wrk)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                135 as ::core::ffi::c_uint,
                b"void WebPRescalerExportRowShrink_C(WebPRescaler *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*wrk).y_accum <= 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"wrk->y_accum <= 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                136 as ::core::ffi::c_uint,
                b"void WebPRescalerExportRowShrink_C(WebPRescaler *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*wrk).y_expand == 0 {
        } else {
            __assert_fail(
                b"!wrk->y_expand\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                137 as ::core::ffi::c_uint,
                b"void WebPRescalerExportRowShrink_C(WebPRescaler *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if yscale != 0 {
        x_out = 0 as ::core::ffi::c_int;
        while x_out < x_out_max {
            let frac: uint32_t = ((*frow.offset(x_out as isize) as uint64_t)
                .wrapping_mul(yscale as uint64_t)
                >> WEBP_RESCALER_RFIX) as uint32_t;
            let v: ::core::ffi::c_int =
                ((((*irow.offset(x_out as isize)).wrapping_sub(frac as rescaler_t) as uint64_t)
                    .wrapping_mul((*wrk).fxy_scale as uint64_t)
                    as ::core::ffi::c_ulonglong)
                    .wrapping_add(ROUNDER)
                    >> WEBP_RESCALER_RFIX) as ::core::ffi::c_int;
            *dst.offset(x_out as isize) = (if v > 255 as ::core::ffi::c_int {
                255 as ::core::ffi::c_uint
            } else {
                v as uint8_t as ::core::ffi::c_uint
            }) as uint8_t;
            *irow.offset(x_out as isize) = frac as rescaler_t;
            x_out += 1;
        }
    } else {
        x_out = 0 as ::core::ffi::c_int;
        while x_out < x_out_max {
            let v_0: ::core::ffi::c_int = (((*irow.offset(x_out as isize) as uint64_t)
                .wrapping_mul((*wrk).fxy_scale as uint64_t)
                as ::core::ffi::c_ulonglong)
                .wrapping_add(ROUNDER)
                >> WEBP_RESCALER_RFIX)
                as ::core::ffi::c_int;
            *dst.offset(x_out as isize) = (if v_0 > 255 as ::core::ffi::c_int {
                255 as ::core::ffi::c_uint
            } else {
                v_0 as uint8_t as ::core::ffi::c_uint
            }) as uint8_t;
            *irow.offset(x_out as isize) = 0 as rescaler_t;
            x_out += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerImportRow(wrk: *mut WebPRescaler, mut src: *const uint8_t) {
    '_c2rust_label: {
        if WebPRescalerInputDone(wrk) == 0 {
        } else {
            __assert_fail(
                b"!WebPRescalerInputDone(wrk)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                162 as ::core::ffi::c_uint,
                b"void WebPRescalerImportRow(WebPRescaler *const, const uint8_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*wrk).x_expand == 0 {
        WebPRescalerImportRowShrink.expect("non-null function pointer")(
            wrk as *mut WebPRescaler,
            src,
        );
    } else {
        WebPRescalerImportRowExpand.expect("non-null function pointer")(
            wrk as *mut WebPRescaler,
            src,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerExportRow(wrk: *mut WebPRescaler) {
    if (*wrk).y_accum <= 0 as ::core::ffi::c_int {
        '_c2rust_label: {
            if WebPRescalerOutputDone(wrk) == 0 {
            } else {
                __assert_fail(
                    b"!WebPRescalerOutputDone(wrk)\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    172 as ::core::ffi::c_uint,
                    b"void WebPRescalerExportRow(WebPRescaler *const)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        if (*wrk).y_expand != 0 {
            WebPRescalerExportRowExpand.expect("non-null function pointer")(
                wrk as *mut WebPRescaler,
            );
        } else if (*wrk).fxy_scale != 0 {
            WebPRescalerExportRowShrink.expect("non-null function pointer")(
                wrk as *mut WebPRescaler,
            );
        } else {
            let mut i: ::core::ffi::c_int = 0;
            '_c2rust_label_0: {
                if (*wrk).src_height == (*wrk).dst_height && (*wrk).x_add == 1 as ::core::ffi::c_int
                {
                } else {
                    __assert_fail(
                        b"wrk->src_height == wrk->dst_height && wrk->x_add == 1\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        179 as ::core::ffi::c_uint,
                        b"void WebPRescalerExportRow(WebPRescaler *const)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            '_c2rust_label_1: {
                if (*wrk).src_width == 1 as ::core::ffi::c_int
                    && (*wrk).dst_width <= 2 as ::core::ffi::c_int
                {
                } else {
                    __assert_fail(
                        b"wrk->src_width == 1 && wrk->dst_width <= 2\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        180 as ::core::ffi::c_uint,
                        b"void WebPRescalerExportRow(WebPRescaler *const)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            i = 0 as ::core::ffi::c_int;
            while i < (*wrk).num_channels * (*wrk).dst_width {
                *(*wrk).dst.offset(i as isize) = *(*wrk).irow.offset(i as isize) as uint8_t;
                *(*wrk).irow.offset(i as isize) = 0 as rescaler_t;
                i += 1;
            }
        }
        (*wrk).y_accum += (*wrk).y_add;
        (*wrk).dst = (*wrk).dst.offset((*wrk).dst_stride as isize);
        (*wrk).dst_y += 1;
    }
}
#[no_mangle]
pub static mut WebPRescalerImportRowExpand: WebPRescalerImportRowFunc = None;
#[no_mangle]
pub static mut WebPRescalerImportRowShrink: WebPRescalerImportRowFunc = None;
#[no_mangle]
pub static mut WebPRescalerExportRowExpand: WebPRescalerExportRowFunc = None;
#[no_mangle]
pub static mut WebPRescalerExportRowShrink: WebPRescalerExportRowFunc = None;
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerDspInit() {
    static mut WebPRescalerDspInit_body_last_cpuinfo_used: VP8CPUInfo = None;
    if !(WebPRescalerDspInit_body_last_cpuinfo_used == VP8GetCPUInfo) {
        WebPRescalerDspInit_body();
        ::core::ptr::write_volatile(
            &mut WebPRescalerDspInit_body_last_cpuinfo_used as *mut VP8CPUInfo,
            VP8GetCPUInfo,
        );
    }
}
unsafe extern "C" fn WebPRescalerDspInit_body() {
    WebPRescalerExportRowExpand =
        Some(WebPRescalerExportRowExpand_C as unsafe extern "C" fn(*mut WebPRescaler) -> ())
            as WebPRescalerExportRowFunc;
    WebPRescalerExportRowShrink =
        Some(WebPRescalerExportRowShrink_C as unsafe extern "C" fn(*mut WebPRescaler) -> ())
            as WebPRescalerExportRowFunc;
    WebPRescalerImportRowExpand = Some(
        WebPRescalerImportRowExpand_C
            as unsafe extern "C" fn(*mut WebPRescaler, *const uint8_t) -> (),
    ) as WebPRescalerImportRowFunc;
    WebPRescalerImportRowShrink = Some(
        WebPRescalerImportRowShrink_C
            as unsafe extern "C" fn(*mut WebPRescaler, *const uint8_t) -> (),
    ) as WebPRescalerImportRowFunc;
    if VP8GetCPUInfo.is_some() {
        if VP8GetCPUInfo.expect("non-null function pointer")(kSSE2) != 0 {
            WebPRescalerDspInitSSE2();
        }
    }
    '_c2rust_label: {
        if WebPRescalerExportRowExpand.is_some() {
        } else {
            __assert_fail(
                b"WebPRescalerExportRowExpand != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                247 as ::core::ffi::c_uint,
                b"void WebPRescalerDspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if WebPRescalerExportRowShrink.is_some() {
        } else {
            __assert_fail(
                b"WebPRescalerExportRowShrink != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                248 as ::core::ffi::c_uint,
                b"void WebPRescalerDspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if WebPRescalerImportRowExpand.is_some() {
        } else {
            __assert_fail(
                b"WebPRescalerImportRowExpand != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                249 as ::core::ffi::c_uint,
                b"void WebPRescalerDspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if WebPRescalerImportRowShrink.is_some() {
        } else {
            __assert_fail(
                b"WebPRescalerImportRowShrink != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dsp/rescaler.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                250 as ::core::ffi::c_uint,
                b"void WebPRescalerDspInit_body(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
}
