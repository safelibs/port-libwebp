extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn WebPRescalerImportRow(wrk: *mut WebPRescaler, src: *const uint8_t);
    fn WebPRescalerExportRow(wrk: *mut WebPRescaler);
    fn WebPRescalerDspInit();
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
pub const WEBP_RESCALER_RFIX: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const WEBP_RESCALER_ONE: ::core::ffi::c_ulonglong =
    (1 as ::core::ffi::c_ulonglong) << WEBP_RESCALER_RFIX;
#[inline]
unsafe extern "C" fn WebPRescalerOutputDone(rescaler: *const WebPRescaler) -> ::core::ffi::c_int {
    return ((*rescaler).dst_y >= (*rescaler).dst_height) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn WebPRescalerHasPendingOutput(
    rescaler: *const WebPRescaler,
) -> ::core::ffi::c_int {
    return (WebPRescalerOutputDone(rescaler) == 0 && (*rescaler).y_accum <= 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn CheckSizeOverflow(mut size: uint64_t) -> ::core::ffi::c_int {
    return (size == size) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerInit(
    rescaler: *mut WebPRescaler,
    mut src_width: ::core::ffi::c_int,
    mut src_height: ::core::ffi::c_int,
    dst: *mut uint8_t,
    mut dst_width: ::core::ffi::c_int,
    mut dst_height: ::core::ffi::c_int,
    mut dst_stride: ::core::ffi::c_int,
    mut num_channels: ::core::ffi::c_int,
    work: *mut rescaler_t,
) -> ::core::ffi::c_int {
    let x_add: ::core::ffi::c_int = src_width;
    let x_sub: ::core::ffi::c_int = dst_width;
    let y_add: ::core::ffi::c_int = src_height;
    let y_sub: ::core::ffi::c_int = dst_height;
    let total_size: uint64_t = (2 as ::core::ffi::c_ulonglong)
        .wrapping_mul(dst_width as ::core::ffi::c_ulonglong)
        .wrapping_mul(num_channels as ::core::ffi::c_ulonglong)
        .wrapping_mul(::core::mem::size_of::<rescaler_t>() as ::core::ffi::c_ulonglong)
        as uint64_t;
    if CheckSizeOverflow(total_size) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*rescaler).x_expand = (src_width < dst_width) as ::core::ffi::c_int;
    (*rescaler).y_expand = (src_height < dst_height) as ::core::ffi::c_int;
    (*rescaler).src_width = src_width;
    (*rescaler).src_height = src_height;
    (*rescaler).dst_width = dst_width;
    (*rescaler).dst_height = dst_height;
    (*rescaler).src_y = 0 as ::core::ffi::c_int;
    (*rescaler).dst_y = 0 as ::core::ffi::c_int;
    (*rescaler).dst = dst;
    (*rescaler).dst_stride = dst_stride;
    (*rescaler).num_channels = num_channels;
    (*rescaler).x_add = if (*rescaler).x_expand != 0 {
        x_sub - 1 as ::core::ffi::c_int
    } else {
        x_add
    };
    (*rescaler).x_sub = if (*rescaler).x_expand != 0 {
        x_add - 1 as ::core::ffi::c_int
    } else {
        x_sub
    };
    if (*rescaler).x_expand == 0 {
        (*rescaler).fx_scale = ((1 as ::core::ffi::c_int as uint64_t) << WEBP_RESCALER_RFIX)
            .wrapping_div((*rescaler).x_sub as uint64_t) as uint32_t;
    }
    (*rescaler).y_add = if (*rescaler).y_expand != 0 {
        y_add - 1 as ::core::ffi::c_int
    } else {
        y_add
    };
    (*rescaler).y_sub = if (*rescaler).y_expand != 0 {
        y_sub - 1 as ::core::ffi::c_int
    } else {
        y_sub
    };
    (*rescaler).y_accum = if (*rescaler).y_expand != 0 {
        (*rescaler).y_sub
    } else {
        (*rescaler).y_add
    };
    if (*rescaler).y_expand == 0 {
        let num: uint64_t = (dst_height as uint64_t as ::core::ffi::c_ulonglong)
            .wrapping_mul(WEBP_RESCALER_ONE) as uint64_t;
        let den: uint64_t =
            ((*rescaler).x_add as uint64_t).wrapping_mul((*rescaler).y_add as uint64_t);
        let ratio: uint64_t = num.wrapping_div(den);
        if ratio != ratio as uint32_t as uint64_t {
            (*rescaler).fxy_scale = 0 as uint32_t;
        } else {
            (*rescaler).fxy_scale = ratio as uint32_t;
        }
        (*rescaler).fy_scale = ((1 as ::core::ffi::c_int as uint64_t) << WEBP_RESCALER_RFIX)
            .wrapping_div((*rescaler).y_sub as uint64_t) as uint32_t;
    } else {
        (*rescaler).fy_scale = ((1 as ::core::ffi::c_int as uint64_t) << WEBP_RESCALER_RFIX)
            .wrapping_div((*rescaler).x_add as uint64_t) as uint32_t;
    }
    (*rescaler).irow = work;
    (*rescaler).frow = work.offset((num_channels * dst_width) as isize);
    memset(
        work as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        total_size as size_t,
    );
    WebPRescalerDspInit();
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerGetScaledDimensions(
    mut src_width: ::core::ffi::c_int,
    mut src_height: ::core::ffi::c_int,
    scaled_width: *mut ::core::ffi::c_int,
    scaled_height: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if !scaled_width.is_null() {
        } else {
            __assert_fail(
                b"scaled_width != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/rescaler_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_uint,
                b"int WebPRescalerGetScaledDimensions(int, int, int *const, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !scaled_height.is_null() {
        } else {
            __assert_fail(
                b"scaled_height != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/rescaler_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_uint,
                b"int WebPRescalerGetScaledDimensions(int, int, int *const, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    let mut width: ::core::ffi::c_int = *scaled_width;
    let mut height: ::core::ffi::c_int = *scaled_height;
    let max_size: ::core::ffi::c_int = INT_MAX / 2 as ::core::ffi::c_int;
    if width == 0 as ::core::ffi::c_int && src_height > 0 as ::core::ffi::c_int {
        width = (src_width as uint64_t)
            .wrapping_mul(height as uint64_t)
            .wrapping_add(src_height as uint64_t)
            .wrapping_sub(1 as uint64_t)
            .wrapping_div(src_height as uint64_t) as ::core::ffi::c_int;
    }
    if height == 0 as ::core::ffi::c_int && src_width > 0 as ::core::ffi::c_int {
        height = (src_height as uint64_t)
            .wrapping_mul(width as uint64_t)
            .wrapping_add(src_width as uint64_t)
            .wrapping_sub(1 as uint64_t)
            .wrapping_div(src_width as uint64_t) as ::core::ffi::c_int;
    }
    if width <= 0 as ::core::ffi::c_int
        || height <= 0 as ::core::ffi::c_int
        || width > max_size
        || height > max_size
    {
        return 0 as ::core::ffi::c_int;
    }
    *scaled_width = width;
    *scaled_height = height;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescaleNeededLines(
    rescaler: *const WebPRescaler,
    mut max_num_lines: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let num_lines: ::core::ffi::c_int =
        ((*rescaler).y_accum + (*rescaler).y_sub - 1 as ::core::ffi::c_int) / (*rescaler).y_sub;
    return if num_lines > max_num_lines {
        max_num_lines
    } else {
        num_lines
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerImport(
    rescaler: *mut WebPRescaler,
    mut num_lines: ::core::ffi::c_int,
    mut src: *const uint8_t,
    mut src_stride: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut total_imported: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while total_imported < num_lines && WebPRescalerHasPendingOutput(rescaler) == 0 {
        if (*rescaler).y_expand != 0 {
            let tmp: *mut rescaler_t = (*rescaler).irow;
            (*rescaler).irow = (*rescaler).frow;
            (*rescaler).frow = tmp;
        }
        WebPRescalerImportRow(rescaler as *mut WebPRescaler, src);
        if (*rescaler).y_expand == 0 {
            let mut x: ::core::ffi::c_int = 0;
            x = 0 as ::core::ffi::c_int;
            while x < (*rescaler).num_channels * (*rescaler).dst_width {
                let ref mut fresh0 = *(*rescaler).irow.offset(x as isize);
                *fresh0 = (*fresh0).wrapping_add(*(*rescaler).frow.offset(x as isize));
                x += 1;
            }
        }
        (*rescaler).src_y += 1;
        src = src.offset(src_stride as isize);
        total_imported += 1;
        (*rescaler).y_accum -= (*rescaler).y_sub;
    }
    return total_imported;
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerExport(rescaler: *mut WebPRescaler) -> ::core::ffi::c_int {
    let mut total_exported: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while WebPRescalerHasPendingOutput(rescaler) != 0 {
        WebPRescalerExportRow(rescaler as *mut WebPRescaler);
        total_exported += 1;
    }
    return total_exported;
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
