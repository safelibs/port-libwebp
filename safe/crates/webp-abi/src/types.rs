use core::ffi::c_void;

pub const fn WEBP_ABI_IS_INCOMPATIBLE(a: i32, b: i32) -> bool {
    (a >> 8) != (b >> 8)
}

c_enum! {
    pub struct WebPFeatureFlags(i32) {
        const ANIMATION_FLAG = 0x0000_0002;
        const XMP_FLAG = 0x0000_0004;
        const EXIF_FLAG = 0x0000_0008;
        const ALPHA_FLAG = 0x0000_0010;
        const ICCP_FLAG = 0x0000_0020;
        const ALL_VALID_FLAGS = 0x0000_003e;
    }
}

c_enum! {
    pub struct WebPMuxAnimDispose(i32) {
        const WEBP_MUX_DISPOSE_NONE = 0;
        const WEBP_MUX_DISPOSE_BACKGROUND = 1;
    }
}

c_enum! {
    pub struct WebPMuxAnimBlend(i32) {
        const WEBP_MUX_BLEND = 0;
        const WEBP_MUX_NO_BLEND = 1;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WebPData {
    pub bytes: *const u8,
    pub size: usize,
}

pub type WebPMallocFn = Option<unsafe extern "C" fn(size: usize) -> *mut c_void>;
pub type WebPFreeFn = Option<unsafe extern "C" fn(ptr: *mut c_void)>;
