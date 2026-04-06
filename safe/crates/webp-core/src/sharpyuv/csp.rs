use webp_abi::{
    kSharpYuvMatrixNum, kSharpYuvMatrixRec601Full, kSharpYuvMatrixRec601Limited,
    kSharpYuvMatrixRec709Full, kSharpYuvMatrixRec709Limited, kSharpYuvMatrixWebp,
    kSharpYuvRangeLimited, SharpYuvColorSpace, SharpYuvConversionMatrix, SharpYuvMatrixType,
};

unsafe extern "C" {
    fn floorf(value: f32) -> f32;
}

#[inline]
fn to_fixed16(value: f32) -> i32 {
    // SAFETY: libm's `floorf` is pure for finite f32 inputs and is used only to match the C oracle.
    unsafe { floorf(value * 65_536.0 + 0.5) as i32 }
}

pub const WEBP_MATRIX: SharpYuvConversionMatrix = SharpYuvConversionMatrix {
    rgb_to_y: [16_839, 33_059, 6_420, 16 << 16],
    rgb_to_u: [-9_719, -19_081, 28_800, 128 << 16],
    rgb_to_v: [28_800, -24_116, -4_684, 128 << 16],
};

pub const REC601_LIMITED_MATRIX: SharpYuvConversionMatrix = SharpYuvConversionMatrix {
    rgb_to_y: [16_829, 33_039, 6_416, 16 << 16],
    rgb_to_u: [-9_714, -19_071, 28_784, 128 << 16],
    rgb_to_v: [28_784, -24_103, -4_681, 128 << 16],
};

pub const REC601_FULL_MATRIX: SharpYuvConversionMatrix = SharpYuvConversionMatrix {
    rgb_to_y: [19_595, 38_470, 7_471, 0],
    rgb_to_u: [-11_058, -21_710, 32_768, 128 << 16],
    rgb_to_v: [32_768, -27_439, -5_329, 128 << 16],
};

pub const REC709_LIMITED_MATRIX: SharpYuvConversionMatrix = SharpYuvConversionMatrix {
    rgb_to_y: [11_966, 40_254, 4_064, 16 << 16],
    rgb_to_u: [-6_596, -22_189, 28_784, 128 << 16],
    rgb_to_v: [28_784, -26_145, -2_639, 128 << 16],
};

pub const REC709_FULL_MATRIX: SharpYuvConversionMatrix = SharpYuvConversionMatrix {
    rgb_to_y: [13_933, 46_871, 4_732, 0],
    rgb_to_u: [-7_509, -25_259, 32_768, 128 << 16],
    rgb_to_v: [32_768, -29_763, -3_005, 128 << 16],
};

pub fn compute_conversion_matrix(
    yuv_color_space: &SharpYuvColorSpace,
    matrix: &mut SharpYuvConversionMatrix,
) {
    let kr = yuv_color_space.kr;
    let kb = yuv_color_space.kb;
    let kg = 1.0 - kr - kb;
    let cr = 0.5 / (1.0 - kb);
    let cb = 0.5 / (1.0 - kr);
    let shift = yuv_color_space.bit_depth - 8;
    let denom = ((1_i32 << yuv_color_space.bit_depth) - 1) as f32;

    let mut scale_y = 1.0_f32;
    let mut add_y = 0.0_f32;
    let mut scale_u = cr;
    let mut scale_v = cb;
    let add_uv = (128_i32 << shift) as f32;

    if yuv_color_space.range == kSharpYuvRangeLimited {
        scale_y *= ((219_i32 << shift) as f32) / denom;
        scale_u *= ((224_i32 << shift) as f32) / denom;
        scale_v *= ((224_i32 << shift) as f32) / denom;
        add_y = (16_i32 << shift) as f32;
    }

    matrix.rgb_to_y = [
        to_fixed16(kr * scale_y),
        to_fixed16(kg * scale_y),
        to_fixed16(kb * scale_y),
        to_fixed16(add_y),
    ];
    matrix.rgb_to_u = [
        to_fixed16(-kr * scale_u),
        to_fixed16(-kg * scale_u),
        to_fixed16((1.0 - kb) * scale_u),
        to_fixed16(add_uv),
    ];
    matrix.rgb_to_v = [
        to_fixed16((1.0 - kr) * scale_v),
        to_fixed16(-kg * scale_v),
        to_fixed16(-kb * scale_v),
        to_fixed16(add_uv),
    ];
}

pub fn get_conversion_matrix(matrix_type: SharpYuvMatrixType) -> *const SharpYuvConversionMatrix {
    match matrix_type {
        value if value == kSharpYuvMatrixWebp => &WEBP_MATRIX,
        value if value == kSharpYuvMatrixRec601Limited => &REC601_LIMITED_MATRIX,
        value if value == kSharpYuvMatrixRec601Full => &REC601_FULL_MATRIX,
        value if value == kSharpYuvMatrixRec709Limited => &REC709_LIMITED_MATRIX,
        value if value == kSharpYuvMatrixRec709Full => &REC709_FULL_MATRIX,
        value if value == kSharpYuvMatrixNum => core::ptr::null(),
        _ => core::ptr::null(),
    }
}
