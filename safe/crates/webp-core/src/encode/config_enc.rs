pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
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
pub type WebPPreset = ::core::ffi::c_uint;
pub const WEBP_PRESET_TEXT: WebPPreset = 5;
pub const WEBP_PRESET_ICON: WebPPreset = 4;
pub const WEBP_PRESET_DRAWING: WebPPreset = 3;
pub const WEBP_PRESET_PHOTO: WebPPreset = 2;
pub const WEBP_PRESET_PICTURE: WebPPreset = 1;
pub const WEBP_PRESET_DEFAULT: WebPPreset = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub method_: uint8_t,
    pub quality_: uint8_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn WebPConfigInitInternal(
    mut config: *mut WebPConfig,
    mut preset: WebPPreset,
    mut quality: ::core::ffi::c_float,
    mut version: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if version >> 8 as ::core::ffi::c_int != 0x20f as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if config.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*config).quality = quality;
    (*config).target_size = 0 as ::core::ffi::c_int;
    (*config).target_PSNR = 0.0f32;
    (*config).method = 4 as ::core::ffi::c_int;
    (*config).sns_strength = 50 as ::core::ffi::c_int;
    (*config).filter_strength = 60 as ::core::ffi::c_int;
    (*config).filter_sharpness = 0 as ::core::ffi::c_int;
    (*config).filter_type = 1 as ::core::ffi::c_int;
    (*config).partitions = 0 as ::core::ffi::c_int;
    (*config).segments = 4 as ::core::ffi::c_int;
    (*config).pass = 1 as ::core::ffi::c_int;
    (*config).qmin = 0 as ::core::ffi::c_int;
    (*config).qmax = 100 as ::core::ffi::c_int;
    (*config).show_compressed = 0 as ::core::ffi::c_int;
    (*config).preprocessing = 0 as ::core::ffi::c_int;
    (*config).autofilter = 0 as ::core::ffi::c_int;
    (*config).partition_limit = 0 as ::core::ffi::c_int;
    (*config).alpha_compression = 1 as ::core::ffi::c_int;
    (*config).alpha_filtering = 1 as ::core::ffi::c_int;
    (*config).alpha_quality = 100 as ::core::ffi::c_int;
    (*config).lossless = 0 as ::core::ffi::c_int;
    (*config).exact = 0 as ::core::ffi::c_int;
    (*config).image_hint = WEBP_HINT_DEFAULT;
    (*config).emulate_jpeg_size = 0 as ::core::ffi::c_int;
    (*config).thread_level = 0 as ::core::ffi::c_int;
    (*config).low_memory = 0 as ::core::ffi::c_int;
    (*config).near_lossless = 100 as ::core::ffi::c_int;
    (*config).use_delta_palette = 0 as ::core::ffi::c_int;
    (*config).use_sharp_yuv = 0 as ::core::ffi::c_int;
    match preset as ::core::ffi::c_uint {
        1 => {
            (*config).sns_strength = 80 as ::core::ffi::c_int;
            (*config).filter_sharpness = 4 as ::core::ffi::c_int;
            (*config).filter_strength = 35 as ::core::ffi::c_int;
            (*config).preprocessing &= !(2 as ::core::ffi::c_int);
        }
        2 => {
            (*config).sns_strength = 80 as ::core::ffi::c_int;
            (*config).filter_sharpness = 3 as ::core::ffi::c_int;
            (*config).filter_strength = 30 as ::core::ffi::c_int;
            (*config).preprocessing |= 2 as ::core::ffi::c_int;
        }
        3 => {
            (*config).sns_strength = 25 as ::core::ffi::c_int;
            (*config).filter_sharpness = 6 as ::core::ffi::c_int;
            (*config).filter_strength = 10 as ::core::ffi::c_int;
        }
        4 => {
            (*config).sns_strength = 0 as ::core::ffi::c_int;
            (*config).filter_strength = 0 as ::core::ffi::c_int;
            (*config).preprocessing &= !(2 as ::core::ffi::c_int);
        }
        5 => {
            (*config).sns_strength = 0 as ::core::ffi::c_int;
            (*config).filter_strength = 0 as ::core::ffi::c_int;
            (*config).preprocessing &= !(2 as ::core::ffi::c_int);
            (*config).segments = 2 as ::core::ffi::c_int;
        }
        0 | _ => {}
    }
    return WebPValidateConfig(config);
}
#[no_mangle]
pub unsafe extern "C" fn WebPValidateConfig(mut config: *const WebPConfig) -> ::core::ffi::c_int {
    if config.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).quality < 0 as ::core::ffi::c_int as ::core::ffi::c_float
        || (*config).quality > 100 as ::core::ffi::c_int as ::core::ffi::c_float
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).target_size < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).target_PSNR < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).method < 0 as ::core::ffi::c_int || (*config).method > 6 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).segments < 1 as ::core::ffi::c_int || (*config).segments > 4 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).sns_strength < 0 as ::core::ffi::c_int
        || (*config).sns_strength > 100 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).filter_strength < 0 as ::core::ffi::c_int
        || (*config).filter_strength > 100 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).filter_sharpness < 0 as ::core::ffi::c_int
        || (*config).filter_sharpness > 7 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).filter_type < 0 as ::core::ffi::c_int
        || (*config).filter_type > 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).autofilter < 0 as ::core::ffi::c_int
        || (*config).autofilter > 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).pass < 1 as ::core::ffi::c_int || (*config).pass > 10 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).qmin < 0 as ::core::ffi::c_int
        || (*config).qmax > 100 as ::core::ffi::c_int
        || (*config).qmin > (*config).qmax
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).show_compressed < 0 as ::core::ffi::c_int
        || (*config).show_compressed > 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).preprocessing < 0 as ::core::ffi::c_int
        || (*config).preprocessing > 7 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).partitions < 0 as ::core::ffi::c_int
        || (*config).partitions > 3 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).partition_limit < 0 as ::core::ffi::c_int
        || (*config).partition_limit > 100 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).alpha_compression < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).alpha_filtering < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).alpha_quality < 0 as ::core::ffi::c_int
        || (*config).alpha_quality > 100 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).lossless < 0 as ::core::ffi::c_int || (*config).lossless > 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).near_lossless < 0 as ::core::ffi::c_int
        || (*config).near_lossless > 100 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).image_hint as ::core::ffi::c_uint
        >= WEBP_HINT_LAST as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).emulate_jpeg_size < 0 as ::core::ffi::c_int
        || (*config).emulate_jpeg_size > 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).thread_level < 0 as ::core::ffi::c_int
        || (*config).thread_level > 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).low_memory < 0 as ::core::ffi::c_int
        || (*config).low_memory > 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).exact < 0 as ::core::ffi::c_int || (*config).exact > 1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).use_delta_palette < 0 as ::core::ffi::c_int
        || (*config).use_delta_palette > 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*config).use_sharp_yuv < 0 as ::core::ffi::c_int
        || (*config).use_sharp_yuv > 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
pub const MAX_LEVEL: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
static mut kLosslessPresets: [C2RustUnnamed; 10] = [
    C2RustUnnamed {
        method_: 0 as uint8_t,
        quality_: 0 as uint8_t,
    },
    C2RustUnnamed {
        method_: 1 as uint8_t,
        quality_: 20 as uint8_t,
    },
    C2RustUnnamed {
        method_: 2 as uint8_t,
        quality_: 25 as uint8_t,
    },
    C2RustUnnamed {
        method_: 3 as uint8_t,
        quality_: 30 as uint8_t,
    },
    C2RustUnnamed {
        method_: 3 as uint8_t,
        quality_: 50 as uint8_t,
    },
    C2RustUnnamed {
        method_: 4 as uint8_t,
        quality_: 50 as uint8_t,
    },
    C2RustUnnamed {
        method_: 4 as uint8_t,
        quality_: 75 as uint8_t,
    },
    C2RustUnnamed {
        method_: 4 as uint8_t,
        quality_: 90 as uint8_t,
    },
    C2RustUnnamed {
        method_: 5 as uint8_t,
        quality_: 90 as uint8_t,
    },
    C2RustUnnamed {
        method_: 6 as uint8_t,
        quality_: 100 as uint8_t,
    },
];
#[no_mangle]
pub unsafe extern "C" fn WebPConfigLosslessPreset(
    mut config: *mut WebPConfig,
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if config.is_null() || level < 0 as ::core::ffi::c_int || level > MAX_LEVEL {
        return 0 as ::core::ffi::c_int;
    }
    (*config).lossless = 1 as ::core::ffi::c_int;
    (*config).method = kLosslessPresets[level as usize].method_ as ::core::ffi::c_int;
    (*config).quality = kLosslessPresets[level as usize].quality_ as ::core::ffi::c_float;
    return 1 as ::core::ffi::c_int;
}
