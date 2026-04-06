use crate::checked::{checked_add_usize, checked_allocation_size, checked_mul_usize};

extern "C" {
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
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
    fn WebPConfigInitInternal(
        _: *mut WebPConfig,
        _: WebPPreset,
        _: ::core::ffi::c_float,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPPictureImportRGB(
        picture: *mut WebPPicture,
        rgb: *const uint8_t,
        rgb_stride: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPPictureImportRGBA(
        picture: *mut WebPPicture,
        rgba: *const uint8_t,
        rgba_stride: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPPictureImportBGR(
        picture: *mut WebPPicture,
        bgr: *const uint8_t,
        bgr_stride: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPPictureImportBGRA(
        picture: *mut WebPPicture,
        bgra: *const uint8_t,
        bgra_stride: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPEncode(config: *const WebPConfig, picture: *mut WebPPicture) -> ::core::ffi::c_int;
    fn WebPEncodingSetError(
        pic: *const WebPPicture,
        error: WebPEncodingError,
    ) -> ::core::ffi::c_int;
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
pub type uintptr_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPConfig {
    pub lossless: ::core::ffi::c_int,
    pub quality: ::core::ffi::c_float,
    pub method: ::core::ffi::c_int,
    pub image_hint: WebPImageHint,
    pub target_size: ::core::ffi::c_int,
    pub target_PSNR: ::core::ffi::c_float,
    pub segments: ::core::ffi::c_int,
    pub sns_strength: ::core::ffi::c_int,
    pub filter_strength: ::core::ffi::c_int,
    pub filter_sharpness: ::core::ffi::c_int,
    pub filter_type: ::core::ffi::c_int,
    pub autofilter: ::core::ffi::c_int,
    pub alpha_compression: ::core::ffi::c_int,
    pub alpha_filtering: ::core::ffi::c_int,
    pub alpha_quality: ::core::ffi::c_int,
    pub pass: ::core::ffi::c_int,
    pub show_compressed: ::core::ffi::c_int,
    pub preprocessing: ::core::ffi::c_int,
    pub partitions: ::core::ffi::c_int,
    pub partition_limit: ::core::ffi::c_int,
    pub emulate_jpeg_size: ::core::ffi::c_int,
    pub thread_level: ::core::ffi::c_int,
    pub low_memory: ::core::ffi::c_int,
    pub near_lossless: ::core::ffi::c_int,
    pub exact: ::core::ffi::c_int,
    pub use_delta_palette: ::core::ffi::c_int,
    pub use_sharp_yuv: ::core::ffi::c_int,
    pub qmin: ::core::ffi::c_int,
    pub qmax: ::core::ffi::c_int,
}
pub type WebPImageHint = ::core::ffi::c_uint;
pub const WEBP_HINT_LAST: WebPImageHint = 4;
pub const WEBP_HINT_GRAPH: WebPImageHint = 3;
pub const WEBP_HINT_PHOTO: WebPImageHint = 2;
pub const WEBP_HINT_PICTURE: WebPImageHint = 1;
pub const WEBP_HINT_DEFAULT: WebPImageHint = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPMemoryWriter {
    pub mem: *mut uint8_t,
    pub size: size_t,
    pub max_size: size_t,
    pub pad: [uint32_t; 1],
}
pub type Importer = Option<
    unsafe extern "C" fn(
        *mut WebPPicture,
        *const uint8_t,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type WebPPreset = ::core::ffi::c_uint;
pub const WEBP_PRESET_TEXT: WebPPreset = 5;
pub const WEBP_PRESET_ICON: WebPPreset = 4;
pub const WEBP_PRESET_DRAWING: WebPPreset = 3;
pub const WEBP_PRESET_PHOTO: WebPPreset = 2;
pub const WEBP_PRESET_PICTURE: WebPPreset = 1;
pub const WEBP_PRESET_DEFAULT: WebPPreset = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const WEBP_ALIGN_CST: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const WEBP_ENCODER_ABI_VERSION: ::core::ffi::c_int = 0x20f as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn WebPConfigPreset(
    mut config: *mut WebPConfig,
    mut preset: WebPPreset,
    mut quality: ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    return WebPConfigInitInternal(config, preset, quality, WEBP_ENCODER_ABI_VERSION);
}
#[inline]
unsafe extern "C" fn WebPPictureInit(mut picture: *mut WebPPicture) -> ::core::ffi::c_int {
    return WebPPictureInitInternal(picture, WEBP_ENCODER_ABI_VERSION);
}
unsafe extern "C" fn DummyWriter(
    mut data: *const uint8_t,
    mut data_size: size_t,
    picture: *const WebPPicture,
) -> ::core::ffi::c_int {
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureInitInternal(
    mut picture: *mut WebPPicture,
    mut version: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if version >> 8 as ::core::ffi::c_int != 0x20f as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if !picture.is_null() {
        memset(
            picture as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<WebPPicture>() as size_t,
        );
        (*picture).writer = Some(
            DummyWriter
                as unsafe extern "C" fn(
                    *const uint8_t,
                    size_t,
                    *const WebPPicture,
                ) -> ::core::ffi::c_int,
        ) as WebPWriterFunction;
        WebPEncodingSetError(picture, VP8_ENC_OK);
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPValidatePicture(picture: *const WebPPicture) -> ::core::ffi::c_int {
    if picture.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*picture).width <= 0 as ::core::ffi::c_int || (*picture).height <= 0 as ::core::ffi::c_int {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_BAD_DIMENSION);
    }
    if (*picture).width <= 0 as ::core::ffi::c_int
        || (*picture).width / 4 as ::core::ffi::c_int > INT_MAX / 4 as ::core::ffi::c_int
        || (*picture).height <= 0 as ::core::ffi::c_int
        || (*picture).height / 4 as ::core::ffi::c_int > INT_MAX / 4 as ::core::ffi::c_int
    {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_BAD_DIMENSION);
    }
    if (*picture).colorspace as ::core::ffi::c_uint
        != WEBP_YUV420 as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*picture).colorspace as ::core::ffi::c_uint
            != WEBP_YUV420A as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_INVALID_CONFIGURATION);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn WebPPictureResetBufferARGB(picture: *mut WebPPicture) {
    (*picture).memory_argb_ = NULL;
    (*picture).argb = ::core::ptr::null_mut::<uint32_t>();
    (*picture).argb_stride = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn WebPPictureResetBufferYUVA(picture: *mut WebPPicture) {
    (*picture).memory_ = NULL;
    (*picture).a = ::core::ptr::null_mut::<uint8_t>();
    (*picture).v = (*picture).a;
    (*picture).u = (*picture).v;
    (*picture).y = (*picture).u;
    (*picture).uv_stride = 0 as ::core::ffi::c_int;
    (*picture).y_stride = (*picture).uv_stride;
    (*picture).a_stride = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureResetBuffers(picture: *mut WebPPicture) {
    WebPPictureResetBufferARGB(picture);
    WebPPictureResetBufferYUVA(picture);
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureAllocARGB(picture: *mut WebPPicture) -> ::core::ffi::c_int {
    let mut memory: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let width: ::core::ffi::c_int = (*picture).width;
    let height: ::core::ffi::c_int = (*picture).height;
    if WebPValidatePicture(picture) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    let Ok(width_usize) = usize::try_from(width) else {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_BAD_DIMENSION);
    };
    let Ok(height_usize) = usize::try_from(height) else {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_BAD_DIMENSION);
    };
    let Some(argb_size) = checked_mul_usize(width_usize, height_usize) else {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    };
    let Some(aligned_pixels) = checked_add_usize(argb_size, WEBP_ALIGN_CST as usize) else {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    };
    let Some(total_bytes) =
        checked_allocation_size(aligned_pixels as u64, ::core::mem::size_of::<uint32_t>() as size_t)
    else {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    };
    WebPSafeFree((*picture).memory_argb_);
    WebPPictureResetBufferARGB(picture);
    memory = WebPSafeMalloc(1 as uint64_t, total_bytes as size_t);
    if memory.is_null() {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    (*picture).memory_argb_ = memory;
    (*picture).argb = ((memory as uintptr_t).wrapping_add(WEBP_ALIGN_CST as uintptr_t)
        & !(WEBP_ALIGN_CST as uintptr_t)) as *mut uint32_t;
    (*picture).argb_stride = width;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureAllocYUVA(picture: *mut WebPPicture) -> ::core::ffi::c_int {
    let has_alpha: ::core::ffi::c_int = (*picture).colorspace as ::core::ffi::c_int
        & WEBP_CSP_ALPHA_BIT as ::core::ffi::c_int as ::core::ffi::c_int;
    let width: ::core::ffi::c_int = (*picture).width;
    let height: ::core::ffi::c_int = (*picture).height;
    let y_stride: ::core::ffi::c_int = width;
    let uv_width: ::core::ffi::c_int =
        (width as int64_t + 1 as int64_t >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let uv_height: ::core::ffi::c_int =
        (height as int64_t + 1 as int64_t >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let uv_stride: ::core::ffi::c_int = uv_width;
    let mut a_width: ::core::ffi::c_int = 0;
    let mut a_stride: ::core::ffi::c_int = 0;
    let mut y_size: uint64_t = 0;
    let mut uv_size: uint64_t = 0;
    let mut a_size: uint64_t = 0;
    let mut total_size: uint64_t = 0;
    let mut mem: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if WebPValidatePicture(picture) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    WebPSafeFree((*picture).memory_);
    WebPPictureResetBufferYUVA(picture);
    a_width = if has_alpha != 0 {
        width
    } else {
        0 as ::core::ffi::c_int
    };
    a_stride = a_width;
    if width <= 0 as ::core::ffi::c_int
        || height <= 0 as ::core::ffi::c_int
        || uv_width <= 0 as ::core::ffi::c_int
        || uv_height <= 0 as ::core::ffi::c_int
    {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_BAD_DIMENSION);
    }
    let Ok(width_usize) = usize::try_from(width) else {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_BAD_DIMENSION);
    };
    let Ok(height_usize) = usize::try_from(height) else {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_BAD_DIMENSION);
    };
    let Ok(uv_width_usize) = usize::try_from(uv_width) else {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_BAD_DIMENSION);
    };
    let Ok(uv_height_usize) = usize::try_from(uv_height) else {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_BAD_DIMENSION);
    };
    let Some(y_size_usize) = checked_mul_usize(width_usize, height_usize) else {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    };
    let Some(uv_size_usize) = checked_mul_usize(uv_width_usize, uv_height_usize) else {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    };
    let a_size_usize = if has_alpha != 0 { y_size_usize } else { 0 };
    let Some(uv_total_usize) = checked_mul_usize(2, uv_size_usize) else {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    };
    let Some(base_size_usize) = checked_add_usize(y_size_usize, a_size_usize) else {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    };
    let Some(total_size_usize) = checked_add_usize(base_size_usize, uv_total_usize) else {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    };
    y_size = y_size_usize as uint64_t;
    uv_size = uv_size_usize as uint64_t;
    a_size = a_size_usize as uint64_t;
    total_size = total_size_usize as uint64_t;
    mem = WebPSafeMalloc(1 as uint64_t, total_size_usize as size_t) as *mut uint8_t;
    if mem.is_null() {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    (*picture).memory_ = mem as *mut ::core::ffi::c_void;
    (*picture).y_stride = y_stride;
    (*picture).uv_stride = uv_stride;
    (*picture).a_stride = a_stride;
    (*picture).y = mem;
    mem = mem.offset(y_size as isize);
    (*picture).u = mem;
    mem = mem.offset(uv_size as isize);
    (*picture).v = mem;
    mem = mem.offset(uv_size as isize);
    if a_size > 0 as uint64_t {
        (*picture).a = mem;
        mem = mem.offset(a_size as isize);
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureAlloc(mut picture: *mut WebPPicture) -> ::core::ffi::c_int {
    if !picture.is_null() {
        WebPPictureFree(picture);
        if (*picture).use_argb == 0 {
            return WebPPictureAllocYUVA(picture);
        } else {
            return WebPPictureAllocARGB(picture);
        }
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureFree(mut picture: *mut WebPPicture) {
    if !picture.is_null() {
        WebPSafeFree((*picture).memory_);
        WebPSafeFree((*picture).memory_argb_);
        WebPPictureResetBuffers(picture);
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPMemoryWriterInit(mut writer: *mut WebPMemoryWriter) {
    (*writer).mem = ::core::ptr::null_mut::<uint8_t>();
    (*writer).size = 0 as size_t;
    (*writer).max_size = 0 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMemoryWrite(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut picture: *const WebPPicture,
) -> ::core::ffi::c_int {
    let w: *mut WebPMemoryWriter = (*picture).custom_ptr as *mut WebPMemoryWriter;
    if w.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    let Some(next_size) = checked_add_usize((*w).size, data_size) else {
        return 0 as ::core::ffi::c_int;
    };
    if next_size > (*w).max_size {
        let mut new_mem: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut next_max_size = checked_mul_usize((*w).max_size, 2).unwrap_or(next_size);
        if next_max_size < next_size {
            next_max_size = next_size;
        }
        if next_max_size < 8192 {
            next_max_size = 8192;
        }
        new_mem = WebPSafeMalloc(1 as uint64_t, next_max_size as size_t) as *mut uint8_t;
        if new_mem.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*w).size > 0 as size_t {
            memcpy(
                new_mem as *mut ::core::ffi::c_void,
                (*w).mem as *const ::core::ffi::c_void,
                (*w).size,
            );
        }
        WebPSafeFree((*w).mem as *mut ::core::ffi::c_void);
        (*w).mem = new_mem;
        (*w).max_size = next_max_size as size_t;
    }
    if data_size > 0 as size_t {
        memcpy(
            (*w).mem.offset((*w).size as isize) as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            data_size,
        );
        (*w).size = next_size;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMemoryWriterClear(mut writer: *mut WebPMemoryWriter) {
    if !writer.is_null() {
        WebPSafeFree((*writer).mem as *mut ::core::ffi::c_void);
        (*writer).mem = ::core::ptr::null_mut::<uint8_t>();
        (*writer).size = 0 as size_t;
        (*writer).max_size = 0 as size_t;
    }
}
unsafe extern "C" fn Encode(
    mut rgba: *const uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
    mut import: Importer,
    mut quality_factor: ::core::ffi::c_float,
    mut lossless: ::core::ffi::c_int,
    mut output: *mut *mut uint8_t,
) -> size_t {
    let mut pic: WebPPicture = WebPPicture {
        use_argb: 0,
        colorspace: WEBP_YUV420,
        width: 0,
        height: 0,
        y: ::core::ptr::null_mut::<uint8_t>(),
        u: ::core::ptr::null_mut::<uint8_t>(),
        v: ::core::ptr::null_mut::<uint8_t>(),
        y_stride: 0,
        uv_stride: 0,
        a: ::core::ptr::null_mut::<uint8_t>(),
        a_stride: 0,
        pad1: [0; 2],
        argb: ::core::ptr::null_mut::<uint32_t>(),
        argb_stride: 0,
        pad2: [0; 3],
        writer: None,
        custom_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        extra_info_type: 0,
        extra_info: ::core::ptr::null_mut::<uint8_t>(),
        stats: ::core::ptr::null_mut::<WebPAuxStats>(),
        error_code: VP8_ENC_OK,
        progress_hook: None,
        user_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pad3: [0; 3],
        pad4: ::core::ptr::null_mut::<uint8_t>(),
        pad5: ::core::ptr::null_mut::<uint8_t>(),
        pad6: [0; 8],
        memory_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        memory_argb_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pad7: [::core::ptr::null_mut::<::core::ffi::c_void>(); 2],
    };
    let mut config: WebPConfig = WebPConfig {
        lossless: 0,
        quality: 0.,
        method: 0,
        image_hint: WEBP_HINT_DEFAULT,
        target_size: 0,
        target_PSNR: 0.,
        segments: 0,
        sns_strength: 0,
        filter_strength: 0,
        filter_sharpness: 0,
        filter_type: 0,
        autofilter: 0,
        alpha_compression: 0,
        alpha_filtering: 0,
        alpha_quality: 0,
        pass: 0,
        show_compressed: 0,
        preprocessing: 0,
        partitions: 0,
        partition_limit: 0,
        emulate_jpeg_size: 0,
        thread_level: 0,
        low_memory: 0,
        near_lossless: 0,
        exact: 0,
        use_delta_palette: 0,
        use_sharp_yuv: 0,
        qmin: 0,
        qmax: 0,
    };
    let mut wrt: WebPMemoryWriter = WebPMemoryWriter {
        mem: ::core::ptr::null_mut::<uint8_t>(),
        size: 0,
        max_size: 0,
        pad: [0; 1],
    };
    let mut ok: ::core::ffi::c_int = 0;
    if output.is_null() {
        return 0 as size_t;
    }
    if WebPConfigPreset(&raw mut config, WEBP_PRESET_DEFAULT, quality_factor) == 0
        || WebPPictureInit(&raw mut pic) == 0
    {
        return 0 as size_t;
    }
    config.lossless = (lossless != 0) as ::core::ffi::c_int;
    pic.use_argb = (lossless != 0) as ::core::ffi::c_int;
    pic.width = width;
    pic.height = height;
    pic.writer = Some(
        WebPMemoryWrite
            as unsafe extern "C" fn(
                *const uint8_t,
                size_t,
                *const WebPPicture,
            ) -> ::core::ffi::c_int,
    ) as WebPWriterFunction;
    pic.custom_ptr = &raw mut wrt as *mut ::core::ffi::c_void;
    WebPMemoryWriterInit(&raw mut wrt);
    ok = (import.expect("non-null function pointer")(&raw mut pic, rgba, stride) != 0
        && WebPEncode(&raw mut config, &raw mut pic) != 0) as ::core::ffi::c_int;
    WebPPictureFree(&raw mut pic);
    if ok == 0 {
        WebPMemoryWriterClear(&raw mut wrt);
        *output = ::core::ptr::null_mut::<uint8_t>();
        return 0 as size_t;
    }
    *output = wrt.mem;
    return wrt.size;
}
#[no_mangle]
pub unsafe extern "C" fn WebPEncodeRGB(
    mut in_0: *const uint8_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut bps: ::core::ffi::c_int,
    mut q: ::core::ffi::c_float,
    mut out: *mut *mut uint8_t,
) -> size_t {
    return Encode(
        in_0,
        w,
        h,
        bps,
        Some(
            WebPPictureImportRGB
                as unsafe extern "C" fn(
                    *mut WebPPicture,
                    *const uint8_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        q,
        0 as ::core::ffi::c_int,
        out,
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPEncodeRGBA(
    mut in_0: *const uint8_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut bps: ::core::ffi::c_int,
    mut q: ::core::ffi::c_float,
    mut out: *mut *mut uint8_t,
) -> size_t {
    return Encode(
        in_0,
        w,
        h,
        bps,
        Some(
            WebPPictureImportRGBA
                as unsafe extern "C" fn(
                    *mut WebPPicture,
                    *const uint8_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        q,
        0 as ::core::ffi::c_int,
        out,
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPEncodeBGR(
    mut in_0: *const uint8_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut bps: ::core::ffi::c_int,
    mut q: ::core::ffi::c_float,
    mut out: *mut *mut uint8_t,
) -> size_t {
    return Encode(
        in_0,
        w,
        h,
        bps,
        Some(
            WebPPictureImportBGR
                as unsafe extern "C" fn(
                    *mut WebPPicture,
                    *const uint8_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        q,
        0 as ::core::ffi::c_int,
        out,
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPEncodeBGRA(
    mut in_0: *const uint8_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut bps: ::core::ffi::c_int,
    mut q: ::core::ffi::c_float,
    mut out: *mut *mut uint8_t,
) -> size_t {
    return Encode(
        in_0,
        w,
        h,
        bps,
        Some(
            WebPPictureImportBGRA
                as unsafe extern "C" fn(
                    *mut WebPPicture,
                    *const uint8_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        q,
        0 as ::core::ffi::c_int,
        out,
    );
}
pub const LOSSLESS_DEFAULT_QUALITY: ::core::ffi::c_double = 70.0f64;
#[no_mangle]
pub unsafe extern "C" fn WebPEncodeLosslessRGB(
    mut in_0: *const uint8_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut bps: ::core::ffi::c_int,
    mut out: *mut *mut uint8_t,
) -> size_t {
    return Encode(
        in_0,
        w,
        h,
        bps,
        Some(
            WebPPictureImportRGB
                as unsafe extern "C" fn(
                    *mut WebPPicture,
                    *const uint8_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        LOSSLESS_DEFAULT_QUALITY as ::core::ffi::c_float,
        1 as ::core::ffi::c_int,
        out,
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPEncodeLosslessRGBA(
    mut in_0: *const uint8_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut bps: ::core::ffi::c_int,
    mut out: *mut *mut uint8_t,
) -> size_t {
    return Encode(
        in_0,
        w,
        h,
        bps,
        Some(
            WebPPictureImportRGBA
                as unsafe extern "C" fn(
                    *mut WebPPicture,
                    *const uint8_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        LOSSLESS_DEFAULT_QUALITY as ::core::ffi::c_float,
        1 as ::core::ffi::c_int,
        out,
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPEncodeLosslessBGR(
    mut in_0: *const uint8_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut bps: ::core::ffi::c_int,
    mut out: *mut *mut uint8_t,
) -> size_t {
    return Encode(
        in_0,
        w,
        h,
        bps,
        Some(
            WebPPictureImportBGR
                as unsafe extern "C" fn(
                    *mut WebPPicture,
                    *const uint8_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        LOSSLESS_DEFAULT_QUALITY as ::core::ffi::c_float,
        1 as ::core::ffi::c_int,
        out,
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPEncodeLosslessBGRA(
    mut in_0: *const uint8_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut bps: ::core::ffi::c_int,
    mut out: *mut *mut uint8_t,
) -> size_t {
    return Encode(
        in_0,
        w,
        h,
        bps,
        Some(
            WebPPictureImportBGRA
                as unsafe extern "C" fn(
                    *mut WebPPicture,
                    *const uint8_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        LOSSLESS_DEFAULT_QUALITY as ::core::ffi::c_float,
        1 as ::core::ffi::c_int,
        out,
    );
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
