pub const SHARPYUV_VERSION_MAJOR: u32 = 0;
pub const SHARPYUV_VERSION_MINOR: u32 = 2;
pub const SHARPYUV_VERSION_PATCH: u32 = 1;
pub const fn SHARPYUV_MAKE_VERSION(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 24) | (minor << 16) | patch
}
pub const SHARPYUV_VERSION: u32 = SHARPYUV_MAKE_VERSION(
    SHARPYUV_VERSION_MAJOR,
    SHARPYUV_VERSION_MINOR,
    SHARPYUV_VERSION_PATCH,
);

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct SharpYuvConversionMatrix {
    pub rgb_to_y: [i32; 4],
    pub rgb_to_u: [i32; 4],
    pub rgb_to_v: [i32; 4],
}

c_enum! {
    pub struct SharpYuvRange(i32) {
        const kSharpYuvRangeFull = 0;
        const kSharpYuvRangeLimited = 1;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct SharpYuvColorSpace {
    pub kr: f32,
    pub kb: f32,
    pub bit_depth: i32,
    pub range: SharpYuvRange,
}

c_enum! {
    pub struct SharpYuvMatrixType(i32) {
        const kSharpYuvMatrixWebp = 0;
        const kSharpYuvMatrixRec601Limited = 1;
        const kSharpYuvMatrixRec601Full = 2;
        const kSharpYuvMatrixRec709Limited = 3;
        const kSharpYuvMatrixRec709Full = 4;
        const kSharpYuvMatrixNum = 5;
    }
}
