extern "C" {
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn WebPRescalerGetScaledDimensions(
        src_width: ::core::ffi::c_int,
        src_height: ::core::ffi::c_int,
        scaled_width: *mut ::core::ffi::c_int,
        scaled_height: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPCheckCropDimensions(
        image_width: ::core::ffi::c_int,
        image_height: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        w: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
    fn WebPCopyPlane(
        src: *const uint8_t,
        src_stride: ::core::ffi::c_int,
        dst: *mut uint8_t,
        dst_stride: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPRGBABuffer {
    pub rgba: *mut uint8_t,
    pub stride: ::core::ffi::c_int,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPYUVABuffer {
    pub y: *mut uint8_t,
    pub u: *mut uint8_t,
    pub v: *mut uint8_t,
    pub a: *mut uint8_t,
    pub y_stride: ::core::ffi::c_int,
    pub u_stride: ::core::ffi::c_int,
    pub v_stride: ::core::ffi::c_int,
    pub a_stride: ::core::ffi::c_int,
    pub y_size: size_t,
    pub u_size: size_t,
    pub v_size: size_t,
    pub a_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPDecBuffer {
    pub colorspace: WEBP_CSP_MODE,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub is_external_memory: ::core::ffi::c_int,
    pub u: C2RustUnnamed,
    pub pad: [uint32_t; 4],
    pub private_memory: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub RGBA: WebPRGBABuffer,
    pub YUVA: WebPYUVABuffer,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPBitstreamFeatures {
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub has_alpha: ::core::ffi::c_int,
    pub has_animation: ::core::ffi::c_int,
    pub format: ::core::ffi::c_int,
    pub pad: [uint32_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPDecoderOptions {
    pub bypass_filtering: ::core::ffi::c_int,
    pub no_fancy_upsampling: ::core::ffi::c_int,
    pub use_cropping: ::core::ffi::c_int,
    pub crop_left: ::core::ffi::c_int,
    pub crop_top: ::core::ffi::c_int,
    pub crop_width: ::core::ffi::c_int,
    pub crop_height: ::core::ffi::c_int,
    pub use_scaling: ::core::ffi::c_int,
    pub scaled_width: ::core::ffi::c_int,
    pub scaled_height: ::core::ffi::c_int,
    pub use_threads: ::core::ffi::c_int,
    pub dithering_strength: ::core::ffi::c_int,
    pub flip: ::core::ffi::c_int,
    pub alpha_dithering_strength: ::core::ffi::c_int,
    pub pad: [uint32_t; 5],
}
pub type VP8StatusCode = ::core::ffi::c_uint;
pub const VP8_STATUS_NOT_ENOUGH_DATA: VP8StatusCode = 7;
pub const VP8_STATUS_USER_ABORT: VP8StatusCode = 6;
pub const VP8_STATUS_SUSPENDED: VP8StatusCode = 5;
pub const VP8_STATUS_UNSUPPORTED_FEATURE: VP8StatusCode = 4;
pub const VP8_STATUS_BITSTREAM_ERROR: VP8StatusCode = 3;
pub const VP8_STATUS_INVALID_PARAM: VP8StatusCode = 2;
pub const VP8_STATUS_OUT_OF_MEMORY: VP8StatusCode = 1;
pub const VP8_STATUS_OK: VP8StatusCode = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn WebPIsPremultipliedMode(mut mode: WEBP_CSP_MODE) -> ::core::ffi::c_int {
    return (mode as ::core::ffi::c_uint == MODE_rgbA as ::core::ffi::c_int as ::core::ffi::c_uint
        || mode as ::core::ffi::c_uint == MODE_bgrA as ::core::ffi::c_int as ::core::ffi::c_uint
        || mode as ::core::ffi::c_uint == MODE_Argb as ::core::ffi::c_int as ::core::ffi::c_uint
        || mode as ::core::ffi::c_uint
            == MODE_rgbA_4444 as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn WebPIsAlphaMode(mut mode: WEBP_CSP_MODE) -> ::core::ffi::c_int {
    return (mode as ::core::ffi::c_uint == MODE_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint
        || mode as ::core::ffi::c_uint == MODE_BGRA as ::core::ffi::c_int as ::core::ffi::c_uint
        || mode as ::core::ffi::c_uint == MODE_ARGB as ::core::ffi::c_int as ::core::ffi::c_uint
        || mode as ::core::ffi::c_uint
            == MODE_RGBA_4444 as ::core::ffi::c_int as ::core::ffi::c_uint
        || mode as ::core::ffi::c_uint == MODE_YUVA as ::core::ffi::c_int as ::core::ffi::c_uint
        || WebPIsPremultipliedMode(mode) != 0) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn WebPIsRGBMode(mut mode: WEBP_CSP_MODE) -> ::core::ffi::c_int {
    return ((mode as ::core::ffi::c_uint) < MODE_YUV as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
static mut kModeBpp: [uint8_t; 13] = [
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
];
unsafe extern "C" fn IsValidColorspace(
    mut webp_csp_mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (webp_csp_mode >= MODE_RGB as ::core::ffi::c_int
        && webp_csp_mode < MODE_LAST as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn CheckDecBuffer(buffer: *const WebPDecBuffer) -> VP8StatusCode {
    let mut ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mode: WEBP_CSP_MODE = (*buffer).colorspace;
    let width: ::core::ffi::c_int = (*buffer).width;
    let height: ::core::ffi::c_int = (*buffer).height;
    if IsValidColorspace(mode as ::core::ffi::c_int) == 0 {
        ok = 0 as ::core::ffi::c_int;
    } else if WebPIsRGBMode(mode) == 0 {
        let buf: *const WebPYUVABuffer = &raw const (*buffer).u.YUVA;
        let uv_width: ::core::ffi::c_int =
            (width + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
        let uv_height: ::core::ffi::c_int =
            (height + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
        let y_stride: ::core::ffi::c_int = abs((*buf).y_stride) as ::core::ffi::c_int;
        let u_stride: ::core::ffi::c_int = abs((*buf).u_stride) as ::core::ffi::c_int;
        let v_stride: ::core::ffi::c_int = abs((*buf).v_stride) as ::core::ffi::c_int;
        let a_stride: ::core::ffi::c_int = abs((*buf).a_stride) as ::core::ffi::c_int;
        let y_size: uint64_t = (y_stride as uint64_t)
            .wrapping_mul((height - 1 as ::core::ffi::c_int) as uint64_t)
            .wrapping_add(width as uint64_t);
        let u_size: uint64_t = (u_stride as uint64_t)
            .wrapping_mul((uv_height - 1 as ::core::ffi::c_int) as uint64_t)
            .wrapping_add(uv_width as uint64_t);
        let v_size: uint64_t = (v_stride as uint64_t)
            .wrapping_mul((uv_height - 1 as ::core::ffi::c_int) as uint64_t)
            .wrapping_add(uv_width as uint64_t);
        let a_size: uint64_t = (a_stride as uint64_t)
            .wrapping_mul((height - 1 as ::core::ffi::c_int) as uint64_t)
            .wrapping_add(width as uint64_t);
        ok &= (y_size <= (*buf).y_size as uint64_t) as ::core::ffi::c_int;
        ok &= (u_size <= (*buf).u_size as uint64_t) as ::core::ffi::c_int;
        ok &= (v_size <= (*buf).v_size as uint64_t) as ::core::ffi::c_int;
        ok &= (y_stride >= width) as ::core::ffi::c_int;
        ok &= (u_stride >= uv_width) as ::core::ffi::c_int;
        ok &= (v_stride >= uv_width) as ::core::ffi::c_int;
        ok &= ((*buf).y != NULL as *mut uint8_t) as ::core::ffi::c_int;
        ok &= ((*buf).u != NULL as *mut uint8_t) as ::core::ffi::c_int;
        ok &= ((*buf).v != NULL as *mut uint8_t) as ::core::ffi::c_int;
        if mode as ::core::ffi::c_uint == MODE_YUVA as ::core::ffi::c_int as ::core::ffi::c_uint {
            ok &= (a_stride >= width) as ::core::ffi::c_int;
            ok &= (a_size <= (*buf).a_size as uint64_t) as ::core::ffi::c_int;
            ok &= ((*buf).a != NULL as *mut uint8_t) as ::core::ffi::c_int;
        }
    } else {
        let buf_0: *const WebPRGBABuffer = &raw const (*buffer).u.RGBA;
        let stride: ::core::ffi::c_int = abs((*buf_0).stride) as ::core::ffi::c_int;
        let size: uint64_t = (stride as uint64_t)
            .wrapping_mul((height - 1 as ::core::ffi::c_int) as uint64_t)
            .wrapping_add((width as uint64_t).wrapping_mul(kModeBpp[mode as usize] as uint64_t));
        ok &= (size <= (*buf_0).size as uint64_t) as ::core::ffi::c_int;
        ok &=
            (stride >= width * kModeBpp[mode as usize] as ::core::ffi::c_int) as ::core::ffi::c_int;
        ok &= ((*buf_0).rgba != NULL as *mut uint8_t) as ::core::ffi::c_int;
    }
    return (if ok != 0 {
        VP8_STATUS_OK as ::core::ffi::c_int
    } else {
        VP8_STATUS_INVALID_PARAM as ::core::ffi::c_int
    }) as VP8StatusCode;
}
unsafe extern "C" fn AllocateBuffer(buffer: *mut WebPDecBuffer) -> VP8StatusCode {
    let w: ::core::ffi::c_int = (*buffer).width;
    let h: ::core::ffi::c_int = (*buffer).height;
    let mode: WEBP_CSP_MODE = (*buffer).colorspace;
    if w <= 0 as ::core::ffi::c_int
        || h <= 0 as ::core::ffi::c_int
        || IsValidColorspace(mode as ::core::ffi::c_int) == 0
    {
        return VP8_STATUS_INVALID_PARAM;
    }
    if (*buffer).is_external_memory <= 0 as ::core::ffi::c_int && (*buffer).private_memory.is_null()
    {
        let mut output: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut uv_stride: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut a_stride: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut uv_size: uint64_t = 0 as uint64_t;
        let mut a_size: uint64_t = 0 as uint64_t;
        let mut total_size: uint64_t = 0;
        let mut stride: ::core::ffi::c_int = 0;
        let mut size: uint64_t = 0;
        if (w as uint64_t).wrapping_mul(kModeBpp[mode as usize] as uint64_t)
            as ::core::ffi::c_ulonglong
            >= (1 as ::core::ffi::c_ulonglong) << 31 as ::core::ffi::c_int
        {
            return VP8_STATUS_INVALID_PARAM;
        }
        stride = w * kModeBpp[mode as usize] as ::core::ffi::c_int;
        size = (stride as uint64_t).wrapping_mul(h as uint64_t);
        if WebPIsRGBMode(mode) == 0 {
            uv_stride = (w + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
            uv_size = (uv_stride as uint64_t).wrapping_mul(
                ((h + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int) as uint64_t,
            );
            if mode as ::core::ffi::c_uint == MODE_YUVA as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                a_stride = w;
                a_size = (a_stride as uint64_t).wrapping_mul(h as uint64_t);
            }
        }
        total_size = size
            .wrapping_add((2 as uint64_t).wrapping_mul(uv_size))
            .wrapping_add(a_size);
        output =
            WebPSafeMalloc(total_size, ::core::mem::size_of::<uint8_t>() as size_t) as *mut uint8_t;
        if output.is_null() {
            return VP8_STATUS_OUT_OF_MEMORY;
        }
        (*buffer).private_memory = output;
        if WebPIsRGBMode(mode) == 0 {
            let buf: *mut WebPYUVABuffer = &raw mut (*buffer).u.YUVA;
            (*buf).y = output;
            (*buf).y_stride = stride;
            (*buf).y_size = size as size_t;
            (*buf).u = output.offset(size as isize);
            (*buf).u_stride = uv_stride;
            (*buf).u_size = uv_size as size_t;
            (*buf).v = output.offset(size as isize).offset(uv_size as isize);
            (*buf).v_stride = uv_stride;
            (*buf).v_size = uv_size as size_t;
            if mode as ::core::ffi::c_uint == MODE_YUVA as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*buf).a = output
                    .offset(size as isize)
                    .offset((2 as uint64_t).wrapping_mul(uv_size) as isize);
            }
            (*buf).a_size = a_size as size_t;
            (*buf).a_stride = a_stride;
        } else {
            let buf_0: *mut WebPRGBABuffer = &raw mut (*buffer).u.RGBA;
            (*buf_0).rgba = output;
            (*buf_0).stride = stride;
            (*buf_0).size = size as size_t;
        }
    }
    return CheckDecBuffer(buffer);
}
#[no_mangle]
pub unsafe extern "C" fn WebPFlipBuffer(buffer: *mut WebPDecBuffer) -> VP8StatusCode {
    if buffer.is_null() {
        return VP8_STATUS_INVALID_PARAM;
    }
    if WebPIsRGBMode((*buffer).colorspace) != 0 {
        let buf: *mut WebPRGBABuffer = &raw mut (*buffer).u.RGBA;
        (*buf).rgba = (*buf).rgba.offset(
            (((*buffer).height - 1 as ::core::ffi::c_int) as int64_t * (*buf).stride as int64_t)
                as isize,
        );
        (*buf).stride = -(*buf).stride;
    } else {
        let buf_0: *mut WebPYUVABuffer = &raw mut (*buffer).u.YUVA;
        let H: int64_t = (*buffer).height as int64_t;
        (*buf_0).y = (*buf_0)
            .y
            .offset(((H - 1 as int64_t) * (*buf_0).y_stride as int64_t) as isize);
        (*buf_0).y_stride = -(*buf_0).y_stride;
        (*buf_0).u = (*buf_0).u.offset(
            ((H - 1 as int64_t >> 1 as ::core::ffi::c_int) * (*buf_0).u_stride as int64_t) as isize,
        );
        (*buf_0).u_stride = -(*buf_0).u_stride;
        (*buf_0).v = (*buf_0).v.offset(
            ((H - 1 as int64_t >> 1 as ::core::ffi::c_int) * (*buf_0).v_stride as int64_t) as isize,
        );
        (*buf_0).v_stride = -(*buf_0).v_stride;
        if !(*buf_0).a.is_null() {
            (*buf_0).a = (*buf_0)
                .a
                .offset(((H - 1 as int64_t) * (*buf_0).a_stride as int64_t) as isize);
            (*buf_0).a_stride = -(*buf_0).a_stride;
        }
    }
    return VP8_STATUS_OK;
}
#[no_mangle]
pub unsafe extern "C" fn WebPAllocateDecBuffer(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    options: *const WebPDecoderOptions,
    buffer: *mut WebPDecBuffer,
) -> VP8StatusCode {
    let mut status: VP8StatusCode = VP8_STATUS_OK;
    if buffer.is_null() || width <= 0 as ::core::ffi::c_int || height <= 0 as ::core::ffi::c_int {
        return VP8_STATUS_INVALID_PARAM;
    }
    if !options.is_null() {
        if (*options).use_cropping != 0 {
            let cw: ::core::ffi::c_int = (*options).crop_width;
            let ch: ::core::ffi::c_int = (*options).crop_height;
            let x: ::core::ffi::c_int = (*options).crop_left & !(1 as ::core::ffi::c_int);
            let y: ::core::ffi::c_int = (*options).crop_top & !(1 as ::core::ffi::c_int);
            if WebPCheckCropDimensions(width, height, x, y, cw, ch) == 0 {
                return VP8_STATUS_INVALID_PARAM;
            }
            width = cw;
            height = ch;
        }
        if (*options).use_scaling != 0 {
            let mut scaled_width: ::core::ffi::c_int = (*options).scaled_width;
            let mut scaled_height: ::core::ffi::c_int = (*options).scaled_height;
            if WebPRescalerGetScaledDimensions(
                width,
                height,
                &raw mut scaled_width,
                &raw mut scaled_height,
            ) == 0
            {
                return VP8_STATUS_INVALID_PARAM;
            }
            width = scaled_width;
            height = scaled_height;
        }
    }
    (*buffer).width = width;
    (*buffer).height = height;
    status = AllocateBuffer(buffer);
    if status as ::core::ffi::c_uint != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        return status;
    }
    if !options.is_null() && (*options).flip != 0 {
        status = WebPFlipBuffer(buffer);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn WebPInitDecBufferInternal(
    mut buffer: *mut WebPDecBuffer,
    mut version: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if version >> 8 as ::core::ffi::c_int != 0x209 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if buffer.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    memset(
        buffer as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<WebPDecBuffer>() as size_t,
    );
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPFreeDecBuffer(mut buffer: *mut WebPDecBuffer) {
    if !buffer.is_null() {
        if (*buffer).is_external_memory <= 0 as ::core::ffi::c_int {
            WebPSafeFree((*buffer).private_memory as *mut ::core::ffi::c_void);
        }
        (*buffer).private_memory = ::core::ptr::null_mut::<uint8_t>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPCopyDecBuffer(src: *const WebPDecBuffer, dst: *mut WebPDecBuffer) {
    if !src.is_null() && !dst.is_null() {
        *dst = *src;
        if !(*src).private_memory.is_null() {
            (*dst).is_external_memory = 1 as ::core::ffi::c_int;
            (*dst).private_memory = ::core::ptr::null_mut::<uint8_t>();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPGrabDecBuffer(src: *mut WebPDecBuffer, dst: *mut WebPDecBuffer) {
    if !src.is_null() && !dst.is_null() {
        *dst = *src;
        if !(*src).private_memory.is_null() {
            (*src).is_external_memory = 1 as ::core::ffi::c_int;
            (*src).private_memory = ::core::ptr::null_mut::<uint8_t>();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPCopyDecBufferPixels(
    src_buf: *const WebPDecBuffer,
    dst_buf: *mut WebPDecBuffer,
) -> VP8StatusCode {
    '_c2rust_label: {
        if !src_buf.is_null() && !dst_buf.is_null() {
        } else {
            __assert_fail(
                b"src_buf != NULL && dst_buf != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/buffer_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                271 as ::core::ffi::c_uint,
                b"VP8StatusCode WebPCopyDecBufferPixels(const WebPDecBuffer *const, WebPDecBuffer *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*src_buf).colorspace as ::core::ffi::c_uint
            == (*dst_buf).colorspace as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"src_buf->colorspace == dst_buf->colorspace\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/buffer_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                272 as ::core::ffi::c_uint,
                b"VP8StatusCode WebPCopyDecBufferPixels(const WebPDecBuffer *const, WebPDecBuffer *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    (*dst_buf).width = (*src_buf).width;
    (*dst_buf).height = (*src_buf).height;
    if CheckDecBuffer(dst_buf) as ::core::ffi::c_uint
        != VP8_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return VP8_STATUS_INVALID_PARAM;
    }
    if WebPIsRGBMode((*src_buf).colorspace) != 0 {
        let src: *const WebPRGBABuffer = &raw const (*src_buf).u.RGBA;
        let dst: *const WebPRGBABuffer = &raw mut (*dst_buf).u.RGBA;
        WebPCopyPlane(
            (*src).rgba,
            (*src).stride,
            (*dst).rgba,
            (*dst).stride,
            (*src_buf).width * kModeBpp[(*src_buf).colorspace as usize] as ::core::ffi::c_int,
            (*src_buf).height,
        );
    } else {
        let src_0: *const WebPYUVABuffer = &raw const (*src_buf).u.YUVA;
        let dst_0: *const WebPYUVABuffer = &raw mut (*dst_buf).u.YUVA;
        WebPCopyPlane(
            (*src_0).y,
            (*src_0).y_stride,
            (*dst_0).y,
            (*dst_0).y_stride,
            (*src_buf).width,
            (*src_buf).height,
        );
        WebPCopyPlane(
            (*src_0).u,
            (*src_0).u_stride,
            (*dst_0).u,
            (*dst_0).u_stride,
            ((*src_buf).width + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int,
            ((*src_buf).height + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int,
        );
        WebPCopyPlane(
            (*src_0).v,
            (*src_0).v_stride,
            (*dst_0).v,
            (*dst_0).v_stride,
            ((*src_buf).width + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int,
            ((*src_buf).height + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int,
        );
        if WebPIsAlphaMode((*src_buf).colorspace) != 0 {
            WebPCopyPlane(
                (*src_0).a,
                (*src_0).a_stride,
                (*dst_0).a,
                (*dst_0).a_stride,
                (*src_buf).width,
                (*src_buf).height,
            );
        }
    }
    return VP8_STATUS_OK;
}
#[no_mangle]
pub unsafe extern "C" fn WebPAvoidSlowMemory(
    output: *const WebPDecBuffer,
    features: *const WebPBitstreamFeatures,
) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if !output.is_null() {
        } else {
            __assert_fail(
                b"output != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/dec/buffer_dec.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                304 as ::core::ffi::c_uint,
                b"int WebPAvoidSlowMemory(const WebPDecBuffer *const, const WebPBitstreamFeatures *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    return ((*output).is_external_memory >= 2 as ::core::ffi::c_int
        && WebPIsPremultipliedMode((*output).colorspace) != 0
        && (!features.is_null() && (*features).has_alpha != 0)) as ::core::ffi::c_int;
}
