use crate::checked::checked_mul_usize;

extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn SharpYuvConvert(
        r_ptr: *const ::core::ffi::c_void,
        g_ptr: *const ::core::ffi::c_void,
        b_ptr: *const ::core::ffi::c_void,
        rgb_step: ::core::ffi::c_int,
        rgb_stride: ::core::ffi::c_int,
        rgb_bit_depth: ::core::ffi::c_int,
        y_ptr: *mut ::core::ffi::c_void,
        y_stride: ::core::ffi::c_int,
        u_ptr: *mut ::core::ffi::c_void,
        u_stride: ::core::ffi::c_int,
        v_ptr: *mut ::core::ffi::c_void,
        v_stride: ::core::ffi::c_int,
        yuv_bit_depth: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        yuv_matrix: *const SharpYuvConversionMatrix,
    ) -> ::core::ffi::c_int;
    fn SharpYuvGetConversionMatrix(
        matrix_type: SharpYuvMatrixType,
    ) -> *const SharpYuvConversionMatrix;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn WebPGetLinePairConverter(alpha_is_last: ::core::ffi::c_int) -> WebPUpsampleLinePairFunc;
    static mut WebPConvertRGBA32ToUV: Option<
        unsafe extern "C" fn(*const uint16_t, *mut uint8_t, *mut uint8_t, ::core::ffi::c_int) -> (),
    >;
    static mut WebPConvertRGB24ToY:
        Option<unsafe extern "C" fn(*const uint8_t, *mut uint8_t, ::core::ffi::c_int) -> ()>;
    static mut WebPConvertBGR24ToY:
        Option<unsafe extern "C" fn(*const uint8_t, *mut uint8_t, ::core::ffi::c_int) -> ()>;
    fn WebPInitConvertARGBToYUV();
    static mut WebPExtractAlpha: Option<
        unsafe extern "C" fn(
            *const uint8_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut uint8_t,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >;
    static mut WebPPackRGB: Option<
        unsafe extern "C" fn(
            *const uint8_t,
            *const uint8_t,
            *const uint8_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut uint32_t,
        ) -> (),
    >;
    static mut WebPHasAlpha8b:
        Option<unsafe extern "C" fn(*const uint8_t, ::core::ffi::c_int) -> ::core::ffi::c_int>;
    static mut WebPHasAlpha32b:
        Option<unsafe extern "C" fn(*const uint8_t, ::core::ffi::c_int) -> ::core::ffi::c_int>;
    fn WebPInitAlphaProcessing();
    fn WebPEncodingSetError(
        pic: *const WebPPicture,
        error: WebPEncodingError,
    ) -> ::core::ffi::c_int;
    fn WebPPictureAllocARGB(picture: *mut WebPPicture) -> ::core::ffi::c_int;
    fn WebPPictureAllocYUVA(picture: *mut WebPPicture) -> ::core::ffi::c_int;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
    static mut VP8GetCPUInfo: VP8CPUInfo;
    fn SharpYuvInit(cpu_info_func: VP8CPUInfo);
    fn VP8InitRandom(rg: *mut VP8Random, dithering: ::core::ffi::c_float);
    fn WebPPictureAlloc(picture: *mut WebPPicture) -> ::core::ffi::c_int;
    static mut VP8LConvertBGRAToRGBA: VP8LConvertFunc;
    fn VP8LDspInit();
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharpYuvConversionMatrix {
    pub rgb_to_y: [::core::ffi::c_int; 4],
    pub rgb_to_u: [::core::ffi::c_int; 4],
    pub rgb_to_v: [::core::ffi::c_int; 4],
}
pub type SharpYuvMatrixType = ::core::ffi::c_uint;
pub const kSharpYuvMatrixNum: SharpYuvMatrixType = 5;
pub const kSharpYuvMatrixRec709Full: SharpYuvMatrixType = 4;
pub const kSharpYuvMatrixRec709Limited: SharpYuvMatrixType = 3;
pub const kSharpYuvMatrixRec601Full: SharpYuvMatrixType = 2;
pub const kSharpYuvMatrixRec601Limited: SharpYuvMatrixType = 1;
pub const kSharpYuvMatrixWebp: SharpYuvMatrixType = 0;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
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
pub type WebPUpsampleLinePairFunc = Option<
    unsafe extern "C" fn(
        *const uint8_t,
        *const uint8_t,
        *const uint8_t,
        *const uint8_t,
        *const uint8_t,
        *const uint8_t,
        *mut uint8_t,
        *mut uint8_t,
        ::core::ffi::c_int,
    ) -> (),
>;
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
pub type VP8LConvertFunc =
    Option<unsafe extern "C" fn(*const uint32_t, ::core::ffi::c_int, *mut uint8_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Random {
    pub index1_: ::core::ffi::c_int,
    pub index2_: ::core::ffi::c_int,
    pub tab_: [uint32_t; 55],
    pub amp_: ::core::ffi::c_int,
}
pub const YUV_FIX: C2RustUnnamed = 16;
pub const YUV_HALF: C2RustUnnamed = 32768;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const YUV_MASK2: C2RustUnnamed = 16383;
pub const YUV_FIX2: C2RustUnnamed = 6;
pub const __ASSERT_FUNCTION_0: [::core::ffi::c_char; 47] =
    crate::compat::c_char_array(b"int VP8RandomBits2(VP8Random *const, int, int)\0");
pub const ALPHA_OFFSET: ::core::ffi::c_int = 3 as ::core::ffi::c_int - 0 as ::core::ffi::c_int;
unsafe extern "C" fn CheckNonOpaque(
    mut alpha: *const uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut x_step: ::core::ffi::c_int,
    mut y_step: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if alpha.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    WebPInitAlphaProcessing();
    if x_step == 1 as ::core::ffi::c_int {
        loop {
            let fresh0 = height;
            height = height - 1;
            if !(fresh0 > 0 as ::core::ffi::c_int) {
                break;
            }
            if WebPHasAlpha8b.expect("non-null function pointer")(alpha, width) != 0 {
                return 1 as ::core::ffi::c_int;
            }
            alpha = alpha.offset(y_step as isize);
        }
    } else {
        loop {
            let fresh1 = height;
            height = height - 1;
            if !(fresh1 > 0 as ::core::ffi::c_int) {
                break;
            }
            if WebPHasAlpha32b.expect("non-null function pointer")(alpha, width) != 0 {
                return 1 as ::core::ffi::c_int;
            }
            alpha = alpha.offset(y_step as isize);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureHasTransparency(
    mut picture: *const WebPPicture,
) -> ::core::ffi::c_int {
    if picture.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*picture).use_argb != 0 {
        if !(*picture).argb.is_null() {
            return CheckNonOpaque(
                ((*picture).argb as *const uint8_t).offset(ALPHA_OFFSET as isize),
                (*picture).width,
                (*picture).height,
                4 as ::core::ffi::c_int,
                ((*picture).argb_stride as usize)
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as usize)
                    as ::core::ffi::c_int,
            );
        }
        return 0 as ::core::ffi::c_int;
    }
    return CheckNonOpaque(
        (*picture).a,
        (*picture).width,
        (*picture).height,
        1 as ::core::ffi::c_int,
        (*picture).a_stride,
    );
}
pub const GAMMA_FIX: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const GAMMA_TAB_FIX: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const GAMMA_TAB_SIZE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << GAMMA_FIX - GAMMA_TAB_FIX;
static mut kGamma: ::core::ffi::c_double = 0.80f64;
static mut kGammaScale: ::core::ffi::c_int =
    ((1 as ::core::ffi::c_int) << GAMMA_FIX) - 1 as ::core::ffi::c_int;
static mut kGammaTabScale: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << GAMMA_TAB_FIX;
static mut kGammaTabRounder: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << GAMMA_TAB_FIX >> 1 as ::core::ffi::c_int;
static mut kLinearToGammaTab: [::core::ffi::c_int; 33] = [0; 33];
static mut kGammaToLinearTab: [uint16_t; 256] = [0; 256];
static mut kGammaTablesOk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn InitGammaTables_body() {
    if kGammaTablesOk == 0 {
        let mut v: ::core::ffi::c_int = 0;
        let scale: ::core::ffi::c_double = ((1 as ::core::ffi::c_int) << GAMMA_TAB_FIX)
            as ::core::ffi::c_double
            / kGammaScale as ::core::ffi::c_double;
        let norm: ::core::ffi::c_double = 1.0f64 / 255.0f64;
        v = 0 as ::core::ffi::c_int;
        while v <= 255 as ::core::ffi::c_int {
            kGammaToLinearTab[v as usize] = (pow(norm * v as ::core::ffi::c_double, kGamma)
                * kGammaScale as ::core::ffi::c_double
                + 0.5f64) as uint16_t;
            v += 1;
        }
        v = 0 as ::core::ffi::c_int;
        while v <= GAMMA_TAB_SIZE {
            kLinearToGammaTab[v as usize] = (255.0f64
                * pow(scale * v as ::core::ffi::c_double, 1.0f64 / kGamma)
                + 0.5f64) as ::core::ffi::c_int;
            v += 1;
        }
        ::core::ptr::write_volatile(
            &mut kGammaTablesOk as *mut ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn InitGammaTables() {
    static mut InitGammaTables_body_last_cpuinfo_used: VP8CPUInfo = None;
    if !(InitGammaTables_body_last_cpuinfo_used == VP8GetCPUInfo) {
        InitGammaTables_body();
        ::core::ptr::write_volatile(
            &mut InitGammaTables_body_last_cpuinfo_used as *mut VP8CPUInfo,
            VP8GetCPUInfo,
        );
    }
}
#[inline]
unsafe extern "C" fn GammaToLinear(mut v: uint8_t) -> uint32_t {
    return kGammaToLinearTab[v as usize] as uint32_t;
}
#[inline]
unsafe extern "C" fn Interpolate(mut v: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let tab_pos: ::core::ffi::c_int = v >> GAMMA_TAB_FIX + 2 as ::core::ffi::c_int;
    let x: ::core::ffi::c_int =
        v & (kGammaTabScale << 2 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
    let v0: ::core::ffi::c_int = kLinearToGammaTab[tab_pos as usize];
    let v1: ::core::ffi::c_int = kLinearToGammaTab[(tab_pos + 1 as ::core::ffi::c_int) as usize];
    let y: ::core::ffi::c_int = v1 * x + v0 * ((kGammaTabScale << 2 as ::core::ffi::c_int) - x);
    '_c2rust_label: {
        if (tab_pos + 1 as ::core::ffi::c_int)
            < ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"tab_pos + 1 < GAMMA_TAB_SIZE + 1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/picture_csp_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                129 as ::core::ffi::c_uint,
                b"int Interpolate(int)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    return y;
}
#[inline]
unsafe extern "C" fn LinearToGamma(
    mut base_value: uint32_t,
    mut shift: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let y: ::core::ffi::c_int =
        Interpolate((base_value << shift) as ::core::ffi::c_int) as ::core::ffi::c_int;
    return y + kGammaTabRounder >> GAMMA_TAB_FIX;
}
unsafe extern "C" fn RGBToY(
    mut r: ::core::ffi::c_int,
    mut g: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
    rg: *mut VP8Random,
) -> ::core::ffi::c_int {
    return if rg.is_null() {
        VP8RGBToY(r, g, b, YUV_HALF as ::core::ffi::c_int)
    } else {
        VP8RGBToY(r, g, b, VP8RandomBits(rg, YUV_FIX as ::core::ffi::c_int))
    };
}
unsafe extern "C" fn RGBToU(
    mut r: ::core::ffi::c_int,
    mut g: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
    rg: *mut VP8Random,
) -> ::core::ffi::c_int {
    return if rg.is_null() {
        VP8RGBToU(
            r,
            g,
            b,
            (YUV_HALF as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
        )
    } else {
        VP8RGBToU(
            r,
            g,
            b,
            VP8RandomBits(rg, YUV_FIX as ::core::ffi::c_int + 2 as ::core::ffi::c_int),
        )
    };
}
unsafe extern "C" fn RGBToV(
    mut r: ::core::ffi::c_int,
    mut g: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
    rg: *mut VP8Random,
) -> ::core::ffi::c_int {
    return if rg.is_null() {
        VP8RGBToV(
            r,
            g,
            b,
            (YUV_HALF as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
        )
    } else {
        VP8RGBToV(
            r,
            g,
            b,
            VP8RandomBits(rg, YUV_FIX as ::core::ffi::c_int + 2 as ::core::ffi::c_int),
        )
    };
}
static mut kMinDimensionIterativeConversion: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn PreprocessARGB(
    mut r_ptr: *const uint8_t,
    mut g_ptr: *const uint8_t,
    mut b_ptr: *const uint8_t,
    mut step: ::core::ffi::c_int,
    mut rgb_stride: ::core::ffi::c_int,
    picture: *mut WebPPicture,
) -> ::core::ffi::c_int {
    let ok: ::core::ffi::c_int = SharpYuvConvert(
        r_ptr as *const ::core::ffi::c_void,
        g_ptr as *const ::core::ffi::c_void,
        b_ptr as *const ::core::ffi::c_void,
        step,
        rgb_stride,
        8 as ::core::ffi::c_int,
        (*picture).y as *mut ::core::ffi::c_void,
        (*picture).y_stride,
        (*picture).u as *mut ::core::ffi::c_void,
        (*picture).uv_stride,
        (*picture).v as *mut ::core::ffi::c_void,
        (*picture).uv_stride,
        8 as ::core::ffi::c_int,
        (*picture).width,
        (*picture).height,
        SharpYuvGetConversionMatrix(kSharpYuvMatrixWebp),
    ) as ::core::ffi::c_int;
    if ok == 0 {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    return ok;
}
static mut kAlphaFix: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
static mut kInvAlpha: [uint32_t; 1021] = [
    0 as ::core::ffi::c_int as uint32_t,
    524288 as ::core::ffi::c_int as uint32_t,
    262144 as ::core::ffi::c_int as uint32_t,
    174762 as ::core::ffi::c_int as uint32_t,
    131072 as ::core::ffi::c_int as uint32_t,
    104857 as ::core::ffi::c_int as uint32_t,
    87381 as ::core::ffi::c_int as uint32_t,
    74898 as ::core::ffi::c_int as uint32_t,
    65536 as ::core::ffi::c_int as uint32_t,
    58254 as ::core::ffi::c_int as uint32_t,
    52428 as ::core::ffi::c_int as uint32_t,
    47662 as ::core::ffi::c_int as uint32_t,
    43690 as ::core::ffi::c_int as uint32_t,
    40329 as ::core::ffi::c_int as uint32_t,
    37449 as ::core::ffi::c_int as uint32_t,
    34952 as ::core::ffi::c_int as uint32_t,
    32768 as ::core::ffi::c_int as uint32_t,
    30840 as ::core::ffi::c_int as uint32_t,
    29127 as ::core::ffi::c_int as uint32_t,
    27594 as ::core::ffi::c_int as uint32_t,
    26214 as ::core::ffi::c_int as uint32_t,
    24966 as ::core::ffi::c_int as uint32_t,
    23831 as ::core::ffi::c_int as uint32_t,
    22795 as ::core::ffi::c_int as uint32_t,
    21845 as ::core::ffi::c_int as uint32_t,
    20971 as ::core::ffi::c_int as uint32_t,
    20164 as ::core::ffi::c_int as uint32_t,
    19418 as ::core::ffi::c_int as uint32_t,
    18724 as ::core::ffi::c_int as uint32_t,
    18078 as ::core::ffi::c_int as uint32_t,
    17476 as ::core::ffi::c_int as uint32_t,
    16912 as ::core::ffi::c_int as uint32_t,
    16384 as ::core::ffi::c_int as uint32_t,
    15887 as ::core::ffi::c_int as uint32_t,
    15420 as ::core::ffi::c_int as uint32_t,
    14979 as ::core::ffi::c_int as uint32_t,
    14563 as ::core::ffi::c_int as uint32_t,
    14169 as ::core::ffi::c_int as uint32_t,
    13797 as ::core::ffi::c_int as uint32_t,
    13443 as ::core::ffi::c_int as uint32_t,
    13107 as ::core::ffi::c_int as uint32_t,
    12787 as ::core::ffi::c_int as uint32_t,
    12483 as ::core::ffi::c_int as uint32_t,
    12192 as ::core::ffi::c_int as uint32_t,
    11915 as ::core::ffi::c_int as uint32_t,
    11650 as ::core::ffi::c_int as uint32_t,
    11397 as ::core::ffi::c_int as uint32_t,
    11155 as ::core::ffi::c_int as uint32_t,
    10922 as ::core::ffi::c_int as uint32_t,
    10699 as ::core::ffi::c_int as uint32_t,
    10485 as ::core::ffi::c_int as uint32_t,
    10280 as ::core::ffi::c_int as uint32_t,
    10082 as ::core::ffi::c_int as uint32_t,
    9892 as ::core::ffi::c_int as uint32_t,
    9709 as ::core::ffi::c_int as uint32_t,
    9532 as ::core::ffi::c_int as uint32_t,
    9362 as ::core::ffi::c_int as uint32_t,
    9198 as ::core::ffi::c_int as uint32_t,
    9039 as ::core::ffi::c_int as uint32_t,
    8886 as ::core::ffi::c_int as uint32_t,
    8738 as ::core::ffi::c_int as uint32_t,
    8594 as ::core::ffi::c_int as uint32_t,
    8456 as ::core::ffi::c_int as uint32_t,
    8322 as ::core::ffi::c_int as uint32_t,
    8192 as ::core::ffi::c_int as uint32_t,
    8065 as ::core::ffi::c_int as uint32_t,
    7943 as ::core::ffi::c_int as uint32_t,
    7825 as ::core::ffi::c_int as uint32_t,
    7710 as ::core::ffi::c_int as uint32_t,
    7598 as ::core::ffi::c_int as uint32_t,
    7489 as ::core::ffi::c_int as uint32_t,
    7384 as ::core::ffi::c_int as uint32_t,
    7281 as ::core::ffi::c_int as uint32_t,
    7182 as ::core::ffi::c_int as uint32_t,
    7084 as ::core::ffi::c_int as uint32_t,
    6990 as ::core::ffi::c_int as uint32_t,
    6898 as ::core::ffi::c_int as uint32_t,
    6808 as ::core::ffi::c_int as uint32_t,
    6721 as ::core::ffi::c_int as uint32_t,
    6636 as ::core::ffi::c_int as uint32_t,
    6553 as ::core::ffi::c_int as uint32_t,
    6472 as ::core::ffi::c_int as uint32_t,
    6393 as ::core::ffi::c_int as uint32_t,
    6316 as ::core::ffi::c_int as uint32_t,
    6241 as ::core::ffi::c_int as uint32_t,
    6168 as ::core::ffi::c_int as uint32_t,
    6096 as ::core::ffi::c_int as uint32_t,
    6026 as ::core::ffi::c_int as uint32_t,
    5957 as ::core::ffi::c_int as uint32_t,
    5890 as ::core::ffi::c_int as uint32_t,
    5825 as ::core::ffi::c_int as uint32_t,
    5761 as ::core::ffi::c_int as uint32_t,
    5698 as ::core::ffi::c_int as uint32_t,
    5637 as ::core::ffi::c_int as uint32_t,
    5577 as ::core::ffi::c_int as uint32_t,
    5518 as ::core::ffi::c_int as uint32_t,
    5461 as ::core::ffi::c_int as uint32_t,
    5405 as ::core::ffi::c_int as uint32_t,
    5349 as ::core::ffi::c_int as uint32_t,
    5295 as ::core::ffi::c_int as uint32_t,
    5242 as ::core::ffi::c_int as uint32_t,
    5190 as ::core::ffi::c_int as uint32_t,
    5140 as ::core::ffi::c_int as uint32_t,
    5090 as ::core::ffi::c_int as uint32_t,
    5041 as ::core::ffi::c_int as uint32_t,
    4993 as ::core::ffi::c_int as uint32_t,
    4946 as ::core::ffi::c_int as uint32_t,
    4899 as ::core::ffi::c_int as uint32_t,
    4854 as ::core::ffi::c_int as uint32_t,
    4809 as ::core::ffi::c_int as uint32_t,
    4766 as ::core::ffi::c_int as uint32_t,
    4723 as ::core::ffi::c_int as uint32_t,
    4681 as ::core::ffi::c_int as uint32_t,
    4639 as ::core::ffi::c_int as uint32_t,
    4599 as ::core::ffi::c_int as uint32_t,
    4559 as ::core::ffi::c_int as uint32_t,
    4519 as ::core::ffi::c_int as uint32_t,
    4481 as ::core::ffi::c_int as uint32_t,
    4443 as ::core::ffi::c_int as uint32_t,
    4405 as ::core::ffi::c_int as uint32_t,
    4369 as ::core::ffi::c_int as uint32_t,
    4332 as ::core::ffi::c_int as uint32_t,
    4297 as ::core::ffi::c_int as uint32_t,
    4262 as ::core::ffi::c_int as uint32_t,
    4228 as ::core::ffi::c_int as uint32_t,
    4194 as ::core::ffi::c_int as uint32_t,
    4161 as ::core::ffi::c_int as uint32_t,
    4128 as ::core::ffi::c_int as uint32_t,
    4096 as ::core::ffi::c_int as uint32_t,
    4064 as ::core::ffi::c_int as uint32_t,
    4032 as ::core::ffi::c_int as uint32_t,
    4002 as ::core::ffi::c_int as uint32_t,
    3971 as ::core::ffi::c_int as uint32_t,
    3942 as ::core::ffi::c_int as uint32_t,
    3912 as ::core::ffi::c_int as uint32_t,
    3883 as ::core::ffi::c_int as uint32_t,
    3855 as ::core::ffi::c_int as uint32_t,
    3826 as ::core::ffi::c_int as uint32_t,
    3799 as ::core::ffi::c_int as uint32_t,
    3771 as ::core::ffi::c_int as uint32_t,
    3744 as ::core::ffi::c_int as uint32_t,
    3718 as ::core::ffi::c_int as uint32_t,
    3692 as ::core::ffi::c_int as uint32_t,
    3666 as ::core::ffi::c_int as uint32_t,
    3640 as ::core::ffi::c_int as uint32_t,
    3615 as ::core::ffi::c_int as uint32_t,
    3591 as ::core::ffi::c_int as uint32_t,
    3566 as ::core::ffi::c_int as uint32_t,
    3542 as ::core::ffi::c_int as uint32_t,
    3518 as ::core::ffi::c_int as uint32_t,
    3495 as ::core::ffi::c_int as uint32_t,
    3472 as ::core::ffi::c_int as uint32_t,
    3449 as ::core::ffi::c_int as uint32_t,
    3426 as ::core::ffi::c_int as uint32_t,
    3404 as ::core::ffi::c_int as uint32_t,
    3382 as ::core::ffi::c_int as uint32_t,
    3360 as ::core::ffi::c_int as uint32_t,
    3339 as ::core::ffi::c_int as uint32_t,
    3318 as ::core::ffi::c_int as uint32_t,
    3297 as ::core::ffi::c_int as uint32_t,
    3276 as ::core::ffi::c_int as uint32_t,
    3256 as ::core::ffi::c_int as uint32_t,
    3236 as ::core::ffi::c_int as uint32_t,
    3216 as ::core::ffi::c_int as uint32_t,
    3196 as ::core::ffi::c_int as uint32_t,
    3177 as ::core::ffi::c_int as uint32_t,
    3158 as ::core::ffi::c_int as uint32_t,
    3139 as ::core::ffi::c_int as uint32_t,
    3120 as ::core::ffi::c_int as uint32_t,
    3102 as ::core::ffi::c_int as uint32_t,
    3084 as ::core::ffi::c_int as uint32_t,
    3066 as ::core::ffi::c_int as uint32_t,
    3048 as ::core::ffi::c_int as uint32_t,
    3030 as ::core::ffi::c_int as uint32_t,
    3013 as ::core::ffi::c_int as uint32_t,
    2995 as ::core::ffi::c_int as uint32_t,
    2978 as ::core::ffi::c_int as uint32_t,
    2962 as ::core::ffi::c_int as uint32_t,
    2945 as ::core::ffi::c_int as uint32_t,
    2928 as ::core::ffi::c_int as uint32_t,
    2912 as ::core::ffi::c_int as uint32_t,
    2896 as ::core::ffi::c_int as uint32_t,
    2880 as ::core::ffi::c_int as uint32_t,
    2864 as ::core::ffi::c_int as uint32_t,
    2849 as ::core::ffi::c_int as uint32_t,
    2833 as ::core::ffi::c_int as uint32_t,
    2818 as ::core::ffi::c_int as uint32_t,
    2803 as ::core::ffi::c_int as uint32_t,
    2788 as ::core::ffi::c_int as uint32_t,
    2774 as ::core::ffi::c_int as uint32_t,
    2759 as ::core::ffi::c_int as uint32_t,
    2744 as ::core::ffi::c_int as uint32_t,
    2730 as ::core::ffi::c_int as uint32_t,
    2716 as ::core::ffi::c_int as uint32_t,
    2702 as ::core::ffi::c_int as uint32_t,
    2688 as ::core::ffi::c_int as uint32_t,
    2674 as ::core::ffi::c_int as uint32_t,
    2661 as ::core::ffi::c_int as uint32_t,
    2647 as ::core::ffi::c_int as uint32_t,
    2634 as ::core::ffi::c_int as uint32_t,
    2621 as ::core::ffi::c_int as uint32_t,
    2608 as ::core::ffi::c_int as uint32_t,
    2595 as ::core::ffi::c_int as uint32_t,
    2582 as ::core::ffi::c_int as uint32_t,
    2570 as ::core::ffi::c_int as uint32_t,
    2557 as ::core::ffi::c_int as uint32_t,
    2545 as ::core::ffi::c_int as uint32_t,
    2532 as ::core::ffi::c_int as uint32_t,
    2520 as ::core::ffi::c_int as uint32_t,
    2508 as ::core::ffi::c_int as uint32_t,
    2496 as ::core::ffi::c_int as uint32_t,
    2484 as ::core::ffi::c_int as uint32_t,
    2473 as ::core::ffi::c_int as uint32_t,
    2461 as ::core::ffi::c_int as uint32_t,
    2449 as ::core::ffi::c_int as uint32_t,
    2438 as ::core::ffi::c_int as uint32_t,
    2427 as ::core::ffi::c_int as uint32_t,
    2416 as ::core::ffi::c_int as uint32_t,
    2404 as ::core::ffi::c_int as uint32_t,
    2394 as ::core::ffi::c_int as uint32_t,
    2383 as ::core::ffi::c_int as uint32_t,
    2372 as ::core::ffi::c_int as uint32_t,
    2361 as ::core::ffi::c_int as uint32_t,
    2351 as ::core::ffi::c_int as uint32_t,
    2340 as ::core::ffi::c_int as uint32_t,
    2330 as ::core::ffi::c_int as uint32_t,
    2319 as ::core::ffi::c_int as uint32_t,
    2309 as ::core::ffi::c_int as uint32_t,
    2299 as ::core::ffi::c_int as uint32_t,
    2289 as ::core::ffi::c_int as uint32_t,
    2279 as ::core::ffi::c_int as uint32_t,
    2269 as ::core::ffi::c_int as uint32_t,
    2259 as ::core::ffi::c_int as uint32_t,
    2250 as ::core::ffi::c_int as uint32_t,
    2240 as ::core::ffi::c_int as uint32_t,
    2231 as ::core::ffi::c_int as uint32_t,
    2221 as ::core::ffi::c_int as uint32_t,
    2212 as ::core::ffi::c_int as uint32_t,
    2202 as ::core::ffi::c_int as uint32_t,
    2193 as ::core::ffi::c_int as uint32_t,
    2184 as ::core::ffi::c_int as uint32_t,
    2175 as ::core::ffi::c_int as uint32_t,
    2166 as ::core::ffi::c_int as uint32_t,
    2157 as ::core::ffi::c_int as uint32_t,
    2148 as ::core::ffi::c_int as uint32_t,
    2139 as ::core::ffi::c_int as uint32_t,
    2131 as ::core::ffi::c_int as uint32_t,
    2122 as ::core::ffi::c_int as uint32_t,
    2114 as ::core::ffi::c_int as uint32_t,
    2105 as ::core::ffi::c_int as uint32_t,
    2097 as ::core::ffi::c_int as uint32_t,
    2088 as ::core::ffi::c_int as uint32_t,
    2080 as ::core::ffi::c_int as uint32_t,
    2072 as ::core::ffi::c_int as uint32_t,
    2064 as ::core::ffi::c_int as uint32_t,
    2056 as ::core::ffi::c_int as uint32_t,
    2048 as ::core::ffi::c_int as uint32_t,
    2040 as ::core::ffi::c_int as uint32_t,
    2032 as ::core::ffi::c_int as uint32_t,
    2024 as ::core::ffi::c_int as uint32_t,
    2016 as ::core::ffi::c_int as uint32_t,
    2008 as ::core::ffi::c_int as uint32_t,
    2001 as ::core::ffi::c_int as uint32_t,
    1993 as ::core::ffi::c_int as uint32_t,
    1985 as ::core::ffi::c_int as uint32_t,
    1978 as ::core::ffi::c_int as uint32_t,
    1971 as ::core::ffi::c_int as uint32_t,
    1963 as ::core::ffi::c_int as uint32_t,
    1956 as ::core::ffi::c_int as uint32_t,
    1949 as ::core::ffi::c_int as uint32_t,
    1941 as ::core::ffi::c_int as uint32_t,
    1934 as ::core::ffi::c_int as uint32_t,
    1927 as ::core::ffi::c_int as uint32_t,
    1920 as ::core::ffi::c_int as uint32_t,
    1913 as ::core::ffi::c_int as uint32_t,
    1906 as ::core::ffi::c_int as uint32_t,
    1899 as ::core::ffi::c_int as uint32_t,
    1892 as ::core::ffi::c_int as uint32_t,
    1885 as ::core::ffi::c_int as uint32_t,
    1879 as ::core::ffi::c_int as uint32_t,
    1872 as ::core::ffi::c_int as uint32_t,
    1865 as ::core::ffi::c_int as uint32_t,
    1859 as ::core::ffi::c_int as uint32_t,
    1852 as ::core::ffi::c_int as uint32_t,
    1846 as ::core::ffi::c_int as uint32_t,
    1839 as ::core::ffi::c_int as uint32_t,
    1833 as ::core::ffi::c_int as uint32_t,
    1826 as ::core::ffi::c_int as uint32_t,
    1820 as ::core::ffi::c_int as uint32_t,
    1814 as ::core::ffi::c_int as uint32_t,
    1807 as ::core::ffi::c_int as uint32_t,
    1801 as ::core::ffi::c_int as uint32_t,
    1795 as ::core::ffi::c_int as uint32_t,
    1789 as ::core::ffi::c_int as uint32_t,
    1783 as ::core::ffi::c_int as uint32_t,
    1777 as ::core::ffi::c_int as uint32_t,
    1771 as ::core::ffi::c_int as uint32_t,
    1765 as ::core::ffi::c_int as uint32_t,
    1759 as ::core::ffi::c_int as uint32_t,
    1753 as ::core::ffi::c_int as uint32_t,
    1747 as ::core::ffi::c_int as uint32_t,
    1741 as ::core::ffi::c_int as uint32_t,
    1736 as ::core::ffi::c_int as uint32_t,
    1730 as ::core::ffi::c_int as uint32_t,
    1724 as ::core::ffi::c_int as uint32_t,
    1718 as ::core::ffi::c_int as uint32_t,
    1713 as ::core::ffi::c_int as uint32_t,
    1707 as ::core::ffi::c_int as uint32_t,
    1702 as ::core::ffi::c_int as uint32_t,
    1696 as ::core::ffi::c_int as uint32_t,
    1691 as ::core::ffi::c_int as uint32_t,
    1685 as ::core::ffi::c_int as uint32_t,
    1680 as ::core::ffi::c_int as uint32_t,
    1675 as ::core::ffi::c_int as uint32_t,
    1669 as ::core::ffi::c_int as uint32_t,
    1664 as ::core::ffi::c_int as uint32_t,
    1659 as ::core::ffi::c_int as uint32_t,
    1653 as ::core::ffi::c_int as uint32_t,
    1648 as ::core::ffi::c_int as uint32_t,
    1643 as ::core::ffi::c_int as uint32_t,
    1638 as ::core::ffi::c_int as uint32_t,
    1633 as ::core::ffi::c_int as uint32_t,
    1628 as ::core::ffi::c_int as uint32_t,
    1623 as ::core::ffi::c_int as uint32_t,
    1618 as ::core::ffi::c_int as uint32_t,
    1613 as ::core::ffi::c_int as uint32_t,
    1608 as ::core::ffi::c_int as uint32_t,
    1603 as ::core::ffi::c_int as uint32_t,
    1598 as ::core::ffi::c_int as uint32_t,
    1593 as ::core::ffi::c_int as uint32_t,
    1588 as ::core::ffi::c_int as uint32_t,
    1583 as ::core::ffi::c_int as uint32_t,
    1579 as ::core::ffi::c_int as uint32_t,
    1574 as ::core::ffi::c_int as uint32_t,
    1569 as ::core::ffi::c_int as uint32_t,
    1565 as ::core::ffi::c_int as uint32_t,
    1560 as ::core::ffi::c_int as uint32_t,
    1555 as ::core::ffi::c_int as uint32_t,
    1551 as ::core::ffi::c_int as uint32_t,
    1546 as ::core::ffi::c_int as uint32_t,
    1542 as ::core::ffi::c_int as uint32_t,
    1537 as ::core::ffi::c_int as uint32_t,
    1533 as ::core::ffi::c_int as uint32_t,
    1528 as ::core::ffi::c_int as uint32_t,
    1524 as ::core::ffi::c_int as uint32_t,
    1519 as ::core::ffi::c_int as uint32_t,
    1515 as ::core::ffi::c_int as uint32_t,
    1510 as ::core::ffi::c_int as uint32_t,
    1506 as ::core::ffi::c_int as uint32_t,
    1502 as ::core::ffi::c_int as uint32_t,
    1497 as ::core::ffi::c_int as uint32_t,
    1493 as ::core::ffi::c_int as uint32_t,
    1489 as ::core::ffi::c_int as uint32_t,
    1485 as ::core::ffi::c_int as uint32_t,
    1481 as ::core::ffi::c_int as uint32_t,
    1476 as ::core::ffi::c_int as uint32_t,
    1472 as ::core::ffi::c_int as uint32_t,
    1468 as ::core::ffi::c_int as uint32_t,
    1464 as ::core::ffi::c_int as uint32_t,
    1460 as ::core::ffi::c_int as uint32_t,
    1456 as ::core::ffi::c_int as uint32_t,
    1452 as ::core::ffi::c_int as uint32_t,
    1448 as ::core::ffi::c_int as uint32_t,
    1444 as ::core::ffi::c_int as uint32_t,
    1440 as ::core::ffi::c_int as uint32_t,
    1436 as ::core::ffi::c_int as uint32_t,
    1432 as ::core::ffi::c_int as uint32_t,
    1428 as ::core::ffi::c_int as uint32_t,
    1424 as ::core::ffi::c_int as uint32_t,
    1420 as ::core::ffi::c_int as uint32_t,
    1416 as ::core::ffi::c_int as uint32_t,
    1413 as ::core::ffi::c_int as uint32_t,
    1409 as ::core::ffi::c_int as uint32_t,
    1405 as ::core::ffi::c_int as uint32_t,
    1401 as ::core::ffi::c_int as uint32_t,
    1398 as ::core::ffi::c_int as uint32_t,
    1394 as ::core::ffi::c_int as uint32_t,
    1390 as ::core::ffi::c_int as uint32_t,
    1387 as ::core::ffi::c_int as uint32_t,
    1383 as ::core::ffi::c_int as uint32_t,
    1379 as ::core::ffi::c_int as uint32_t,
    1376 as ::core::ffi::c_int as uint32_t,
    1372 as ::core::ffi::c_int as uint32_t,
    1368 as ::core::ffi::c_int as uint32_t,
    1365 as ::core::ffi::c_int as uint32_t,
    1361 as ::core::ffi::c_int as uint32_t,
    1358 as ::core::ffi::c_int as uint32_t,
    1354 as ::core::ffi::c_int as uint32_t,
    1351 as ::core::ffi::c_int as uint32_t,
    1347 as ::core::ffi::c_int as uint32_t,
    1344 as ::core::ffi::c_int as uint32_t,
    1340 as ::core::ffi::c_int as uint32_t,
    1337 as ::core::ffi::c_int as uint32_t,
    1334 as ::core::ffi::c_int as uint32_t,
    1330 as ::core::ffi::c_int as uint32_t,
    1327 as ::core::ffi::c_int as uint32_t,
    1323 as ::core::ffi::c_int as uint32_t,
    1320 as ::core::ffi::c_int as uint32_t,
    1317 as ::core::ffi::c_int as uint32_t,
    1314 as ::core::ffi::c_int as uint32_t,
    1310 as ::core::ffi::c_int as uint32_t,
    1307 as ::core::ffi::c_int as uint32_t,
    1304 as ::core::ffi::c_int as uint32_t,
    1300 as ::core::ffi::c_int as uint32_t,
    1297 as ::core::ffi::c_int as uint32_t,
    1294 as ::core::ffi::c_int as uint32_t,
    1291 as ::core::ffi::c_int as uint32_t,
    1288 as ::core::ffi::c_int as uint32_t,
    1285 as ::core::ffi::c_int as uint32_t,
    1281 as ::core::ffi::c_int as uint32_t,
    1278 as ::core::ffi::c_int as uint32_t,
    1275 as ::core::ffi::c_int as uint32_t,
    1272 as ::core::ffi::c_int as uint32_t,
    1269 as ::core::ffi::c_int as uint32_t,
    1266 as ::core::ffi::c_int as uint32_t,
    1263 as ::core::ffi::c_int as uint32_t,
    1260 as ::core::ffi::c_int as uint32_t,
    1257 as ::core::ffi::c_int as uint32_t,
    1254 as ::core::ffi::c_int as uint32_t,
    1251 as ::core::ffi::c_int as uint32_t,
    1248 as ::core::ffi::c_int as uint32_t,
    1245 as ::core::ffi::c_int as uint32_t,
    1242 as ::core::ffi::c_int as uint32_t,
    1239 as ::core::ffi::c_int as uint32_t,
    1236 as ::core::ffi::c_int as uint32_t,
    1233 as ::core::ffi::c_int as uint32_t,
    1230 as ::core::ffi::c_int as uint32_t,
    1227 as ::core::ffi::c_int as uint32_t,
    1224 as ::core::ffi::c_int as uint32_t,
    1222 as ::core::ffi::c_int as uint32_t,
    1219 as ::core::ffi::c_int as uint32_t,
    1216 as ::core::ffi::c_int as uint32_t,
    1213 as ::core::ffi::c_int as uint32_t,
    1210 as ::core::ffi::c_int as uint32_t,
    1208 as ::core::ffi::c_int as uint32_t,
    1205 as ::core::ffi::c_int as uint32_t,
    1202 as ::core::ffi::c_int as uint32_t,
    1199 as ::core::ffi::c_int as uint32_t,
    1197 as ::core::ffi::c_int as uint32_t,
    1194 as ::core::ffi::c_int as uint32_t,
    1191 as ::core::ffi::c_int as uint32_t,
    1188 as ::core::ffi::c_int as uint32_t,
    1186 as ::core::ffi::c_int as uint32_t,
    1183 as ::core::ffi::c_int as uint32_t,
    1180 as ::core::ffi::c_int as uint32_t,
    1178 as ::core::ffi::c_int as uint32_t,
    1175 as ::core::ffi::c_int as uint32_t,
    1172 as ::core::ffi::c_int as uint32_t,
    1170 as ::core::ffi::c_int as uint32_t,
    1167 as ::core::ffi::c_int as uint32_t,
    1165 as ::core::ffi::c_int as uint32_t,
    1162 as ::core::ffi::c_int as uint32_t,
    1159 as ::core::ffi::c_int as uint32_t,
    1157 as ::core::ffi::c_int as uint32_t,
    1154 as ::core::ffi::c_int as uint32_t,
    1152 as ::core::ffi::c_int as uint32_t,
    1149 as ::core::ffi::c_int as uint32_t,
    1147 as ::core::ffi::c_int as uint32_t,
    1144 as ::core::ffi::c_int as uint32_t,
    1142 as ::core::ffi::c_int as uint32_t,
    1139 as ::core::ffi::c_int as uint32_t,
    1137 as ::core::ffi::c_int as uint32_t,
    1134 as ::core::ffi::c_int as uint32_t,
    1132 as ::core::ffi::c_int as uint32_t,
    1129 as ::core::ffi::c_int as uint32_t,
    1127 as ::core::ffi::c_int as uint32_t,
    1125 as ::core::ffi::c_int as uint32_t,
    1122 as ::core::ffi::c_int as uint32_t,
    1120 as ::core::ffi::c_int as uint32_t,
    1117 as ::core::ffi::c_int as uint32_t,
    1115 as ::core::ffi::c_int as uint32_t,
    1113 as ::core::ffi::c_int as uint32_t,
    1110 as ::core::ffi::c_int as uint32_t,
    1108 as ::core::ffi::c_int as uint32_t,
    1106 as ::core::ffi::c_int as uint32_t,
    1103 as ::core::ffi::c_int as uint32_t,
    1101 as ::core::ffi::c_int as uint32_t,
    1099 as ::core::ffi::c_int as uint32_t,
    1096 as ::core::ffi::c_int as uint32_t,
    1094 as ::core::ffi::c_int as uint32_t,
    1092 as ::core::ffi::c_int as uint32_t,
    1089 as ::core::ffi::c_int as uint32_t,
    1087 as ::core::ffi::c_int as uint32_t,
    1085 as ::core::ffi::c_int as uint32_t,
    1083 as ::core::ffi::c_int as uint32_t,
    1081 as ::core::ffi::c_int as uint32_t,
    1078 as ::core::ffi::c_int as uint32_t,
    1076 as ::core::ffi::c_int as uint32_t,
    1074 as ::core::ffi::c_int as uint32_t,
    1072 as ::core::ffi::c_int as uint32_t,
    1069 as ::core::ffi::c_int as uint32_t,
    1067 as ::core::ffi::c_int as uint32_t,
    1065 as ::core::ffi::c_int as uint32_t,
    1063 as ::core::ffi::c_int as uint32_t,
    1061 as ::core::ffi::c_int as uint32_t,
    1059 as ::core::ffi::c_int as uint32_t,
    1057 as ::core::ffi::c_int as uint32_t,
    1054 as ::core::ffi::c_int as uint32_t,
    1052 as ::core::ffi::c_int as uint32_t,
    1050 as ::core::ffi::c_int as uint32_t,
    1048 as ::core::ffi::c_int as uint32_t,
    1046 as ::core::ffi::c_int as uint32_t,
    1044 as ::core::ffi::c_int as uint32_t,
    1042 as ::core::ffi::c_int as uint32_t,
    1040 as ::core::ffi::c_int as uint32_t,
    1038 as ::core::ffi::c_int as uint32_t,
    1036 as ::core::ffi::c_int as uint32_t,
    1034 as ::core::ffi::c_int as uint32_t,
    1032 as ::core::ffi::c_int as uint32_t,
    1030 as ::core::ffi::c_int as uint32_t,
    1028 as ::core::ffi::c_int as uint32_t,
    1026 as ::core::ffi::c_int as uint32_t,
    1024 as ::core::ffi::c_int as uint32_t,
    1022 as ::core::ffi::c_int as uint32_t,
    1020 as ::core::ffi::c_int as uint32_t,
    1018 as ::core::ffi::c_int as uint32_t,
    1016 as ::core::ffi::c_int as uint32_t,
    1014 as ::core::ffi::c_int as uint32_t,
    1012 as ::core::ffi::c_int as uint32_t,
    1010 as ::core::ffi::c_int as uint32_t,
    1008 as ::core::ffi::c_int as uint32_t,
    1006 as ::core::ffi::c_int as uint32_t,
    1004 as ::core::ffi::c_int as uint32_t,
    1002 as ::core::ffi::c_int as uint32_t,
    1000 as ::core::ffi::c_int as uint32_t,
    998 as ::core::ffi::c_int as uint32_t,
    996 as ::core::ffi::c_int as uint32_t,
    994 as ::core::ffi::c_int as uint32_t,
    992 as ::core::ffi::c_int as uint32_t,
    991 as ::core::ffi::c_int as uint32_t,
    989 as ::core::ffi::c_int as uint32_t,
    987 as ::core::ffi::c_int as uint32_t,
    985 as ::core::ffi::c_int as uint32_t,
    983 as ::core::ffi::c_int as uint32_t,
    981 as ::core::ffi::c_int as uint32_t,
    979 as ::core::ffi::c_int as uint32_t,
    978 as ::core::ffi::c_int as uint32_t,
    976 as ::core::ffi::c_int as uint32_t,
    974 as ::core::ffi::c_int as uint32_t,
    972 as ::core::ffi::c_int as uint32_t,
    970 as ::core::ffi::c_int as uint32_t,
    969 as ::core::ffi::c_int as uint32_t,
    967 as ::core::ffi::c_int as uint32_t,
    965 as ::core::ffi::c_int as uint32_t,
    963 as ::core::ffi::c_int as uint32_t,
    961 as ::core::ffi::c_int as uint32_t,
    960 as ::core::ffi::c_int as uint32_t,
    958 as ::core::ffi::c_int as uint32_t,
    956 as ::core::ffi::c_int as uint32_t,
    954 as ::core::ffi::c_int as uint32_t,
    953 as ::core::ffi::c_int as uint32_t,
    951 as ::core::ffi::c_int as uint32_t,
    949 as ::core::ffi::c_int as uint32_t,
    948 as ::core::ffi::c_int as uint32_t,
    946 as ::core::ffi::c_int as uint32_t,
    944 as ::core::ffi::c_int as uint32_t,
    942 as ::core::ffi::c_int as uint32_t,
    941 as ::core::ffi::c_int as uint32_t,
    939 as ::core::ffi::c_int as uint32_t,
    937 as ::core::ffi::c_int as uint32_t,
    936 as ::core::ffi::c_int as uint32_t,
    934 as ::core::ffi::c_int as uint32_t,
    932 as ::core::ffi::c_int as uint32_t,
    931 as ::core::ffi::c_int as uint32_t,
    929 as ::core::ffi::c_int as uint32_t,
    927 as ::core::ffi::c_int as uint32_t,
    926 as ::core::ffi::c_int as uint32_t,
    924 as ::core::ffi::c_int as uint32_t,
    923 as ::core::ffi::c_int as uint32_t,
    921 as ::core::ffi::c_int as uint32_t,
    919 as ::core::ffi::c_int as uint32_t,
    918 as ::core::ffi::c_int as uint32_t,
    916 as ::core::ffi::c_int as uint32_t,
    914 as ::core::ffi::c_int as uint32_t,
    913 as ::core::ffi::c_int as uint32_t,
    911 as ::core::ffi::c_int as uint32_t,
    910 as ::core::ffi::c_int as uint32_t,
    908 as ::core::ffi::c_int as uint32_t,
    907 as ::core::ffi::c_int as uint32_t,
    905 as ::core::ffi::c_int as uint32_t,
    903 as ::core::ffi::c_int as uint32_t,
    902 as ::core::ffi::c_int as uint32_t,
    900 as ::core::ffi::c_int as uint32_t,
    899 as ::core::ffi::c_int as uint32_t,
    897 as ::core::ffi::c_int as uint32_t,
    896 as ::core::ffi::c_int as uint32_t,
    894 as ::core::ffi::c_int as uint32_t,
    893 as ::core::ffi::c_int as uint32_t,
    891 as ::core::ffi::c_int as uint32_t,
    890 as ::core::ffi::c_int as uint32_t,
    888 as ::core::ffi::c_int as uint32_t,
    887 as ::core::ffi::c_int as uint32_t,
    885 as ::core::ffi::c_int as uint32_t,
    884 as ::core::ffi::c_int as uint32_t,
    882 as ::core::ffi::c_int as uint32_t,
    881 as ::core::ffi::c_int as uint32_t,
    879 as ::core::ffi::c_int as uint32_t,
    878 as ::core::ffi::c_int as uint32_t,
    876 as ::core::ffi::c_int as uint32_t,
    875 as ::core::ffi::c_int as uint32_t,
    873 as ::core::ffi::c_int as uint32_t,
    872 as ::core::ffi::c_int as uint32_t,
    870 as ::core::ffi::c_int as uint32_t,
    869 as ::core::ffi::c_int as uint32_t,
    868 as ::core::ffi::c_int as uint32_t,
    866 as ::core::ffi::c_int as uint32_t,
    865 as ::core::ffi::c_int as uint32_t,
    863 as ::core::ffi::c_int as uint32_t,
    862 as ::core::ffi::c_int as uint32_t,
    860 as ::core::ffi::c_int as uint32_t,
    859 as ::core::ffi::c_int as uint32_t,
    858 as ::core::ffi::c_int as uint32_t,
    856 as ::core::ffi::c_int as uint32_t,
    855 as ::core::ffi::c_int as uint32_t,
    853 as ::core::ffi::c_int as uint32_t,
    852 as ::core::ffi::c_int as uint32_t,
    851 as ::core::ffi::c_int as uint32_t,
    849 as ::core::ffi::c_int as uint32_t,
    848 as ::core::ffi::c_int as uint32_t,
    846 as ::core::ffi::c_int as uint32_t,
    845 as ::core::ffi::c_int as uint32_t,
    844 as ::core::ffi::c_int as uint32_t,
    842 as ::core::ffi::c_int as uint32_t,
    841 as ::core::ffi::c_int as uint32_t,
    840 as ::core::ffi::c_int as uint32_t,
    838 as ::core::ffi::c_int as uint32_t,
    837 as ::core::ffi::c_int as uint32_t,
    836 as ::core::ffi::c_int as uint32_t,
    834 as ::core::ffi::c_int as uint32_t,
    833 as ::core::ffi::c_int as uint32_t,
    832 as ::core::ffi::c_int as uint32_t,
    830 as ::core::ffi::c_int as uint32_t,
    829 as ::core::ffi::c_int as uint32_t,
    828 as ::core::ffi::c_int as uint32_t,
    826 as ::core::ffi::c_int as uint32_t,
    825 as ::core::ffi::c_int as uint32_t,
    824 as ::core::ffi::c_int as uint32_t,
    823 as ::core::ffi::c_int as uint32_t,
    821 as ::core::ffi::c_int as uint32_t,
    820 as ::core::ffi::c_int as uint32_t,
    819 as ::core::ffi::c_int as uint32_t,
    817 as ::core::ffi::c_int as uint32_t,
    816 as ::core::ffi::c_int as uint32_t,
    815 as ::core::ffi::c_int as uint32_t,
    814 as ::core::ffi::c_int as uint32_t,
    812 as ::core::ffi::c_int as uint32_t,
    811 as ::core::ffi::c_int as uint32_t,
    810 as ::core::ffi::c_int as uint32_t,
    809 as ::core::ffi::c_int as uint32_t,
    807 as ::core::ffi::c_int as uint32_t,
    806 as ::core::ffi::c_int as uint32_t,
    805 as ::core::ffi::c_int as uint32_t,
    804 as ::core::ffi::c_int as uint32_t,
    802 as ::core::ffi::c_int as uint32_t,
    801 as ::core::ffi::c_int as uint32_t,
    800 as ::core::ffi::c_int as uint32_t,
    799 as ::core::ffi::c_int as uint32_t,
    798 as ::core::ffi::c_int as uint32_t,
    796 as ::core::ffi::c_int as uint32_t,
    795 as ::core::ffi::c_int as uint32_t,
    794 as ::core::ffi::c_int as uint32_t,
    793 as ::core::ffi::c_int as uint32_t,
    791 as ::core::ffi::c_int as uint32_t,
    790 as ::core::ffi::c_int as uint32_t,
    789 as ::core::ffi::c_int as uint32_t,
    788 as ::core::ffi::c_int as uint32_t,
    787 as ::core::ffi::c_int as uint32_t,
    786 as ::core::ffi::c_int as uint32_t,
    784 as ::core::ffi::c_int as uint32_t,
    783 as ::core::ffi::c_int as uint32_t,
    782 as ::core::ffi::c_int as uint32_t,
    781 as ::core::ffi::c_int as uint32_t,
    780 as ::core::ffi::c_int as uint32_t,
    779 as ::core::ffi::c_int as uint32_t,
    777 as ::core::ffi::c_int as uint32_t,
    776 as ::core::ffi::c_int as uint32_t,
    775 as ::core::ffi::c_int as uint32_t,
    774 as ::core::ffi::c_int as uint32_t,
    773 as ::core::ffi::c_int as uint32_t,
    772 as ::core::ffi::c_int as uint32_t,
    771 as ::core::ffi::c_int as uint32_t,
    769 as ::core::ffi::c_int as uint32_t,
    768 as ::core::ffi::c_int as uint32_t,
    767 as ::core::ffi::c_int as uint32_t,
    766 as ::core::ffi::c_int as uint32_t,
    765 as ::core::ffi::c_int as uint32_t,
    764 as ::core::ffi::c_int as uint32_t,
    763 as ::core::ffi::c_int as uint32_t,
    762 as ::core::ffi::c_int as uint32_t,
    760 as ::core::ffi::c_int as uint32_t,
    759 as ::core::ffi::c_int as uint32_t,
    758 as ::core::ffi::c_int as uint32_t,
    757 as ::core::ffi::c_int as uint32_t,
    756 as ::core::ffi::c_int as uint32_t,
    755 as ::core::ffi::c_int as uint32_t,
    754 as ::core::ffi::c_int as uint32_t,
    753 as ::core::ffi::c_int as uint32_t,
    752 as ::core::ffi::c_int as uint32_t,
    751 as ::core::ffi::c_int as uint32_t,
    750 as ::core::ffi::c_int as uint32_t,
    748 as ::core::ffi::c_int as uint32_t,
    747 as ::core::ffi::c_int as uint32_t,
    746 as ::core::ffi::c_int as uint32_t,
    745 as ::core::ffi::c_int as uint32_t,
    744 as ::core::ffi::c_int as uint32_t,
    743 as ::core::ffi::c_int as uint32_t,
    742 as ::core::ffi::c_int as uint32_t,
    741 as ::core::ffi::c_int as uint32_t,
    740 as ::core::ffi::c_int as uint32_t,
    739 as ::core::ffi::c_int as uint32_t,
    738 as ::core::ffi::c_int as uint32_t,
    737 as ::core::ffi::c_int as uint32_t,
    736 as ::core::ffi::c_int as uint32_t,
    735 as ::core::ffi::c_int as uint32_t,
    734 as ::core::ffi::c_int as uint32_t,
    733 as ::core::ffi::c_int as uint32_t,
    732 as ::core::ffi::c_int as uint32_t,
    731 as ::core::ffi::c_int as uint32_t,
    730 as ::core::ffi::c_int as uint32_t,
    729 as ::core::ffi::c_int as uint32_t,
    728 as ::core::ffi::c_int as uint32_t,
    727 as ::core::ffi::c_int as uint32_t,
    726 as ::core::ffi::c_int as uint32_t,
    725 as ::core::ffi::c_int as uint32_t,
    724 as ::core::ffi::c_int as uint32_t,
    723 as ::core::ffi::c_int as uint32_t,
    722 as ::core::ffi::c_int as uint32_t,
    721 as ::core::ffi::c_int as uint32_t,
    720 as ::core::ffi::c_int as uint32_t,
    719 as ::core::ffi::c_int as uint32_t,
    718 as ::core::ffi::c_int as uint32_t,
    717 as ::core::ffi::c_int as uint32_t,
    716 as ::core::ffi::c_int as uint32_t,
    715 as ::core::ffi::c_int as uint32_t,
    714 as ::core::ffi::c_int as uint32_t,
    713 as ::core::ffi::c_int as uint32_t,
    712 as ::core::ffi::c_int as uint32_t,
    711 as ::core::ffi::c_int as uint32_t,
    710 as ::core::ffi::c_int as uint32_t,
    709 as ::core::ffi::c_int as uint32_t,
    708 as ::core::ffi::c_int as uint32_t,
    707 as ::core::ffi::c_int as uint32_t,
    706 as ::core::ffi::c_int as uint32_t,
    705 as ::core::ffi::c_int as uint32_t,
    704 as ::core::ffi::c_int as uint32_t,
    703 as ::core::ffi::c_int as uint32_t,
    702 as ::core::ffi::c_int as uint32_t,
    701 as ::core::ffi::c_int as uint32_t,
    700 as ::core::ffi::c_int as uint32_t,
    699 as ::core::ffi::c_int as uint32_t,
    699 as ::core::ffi::c_int as uint32_t,
    698 as ::core::ffi::c_int as uint32_t,
    697 as ::core::ffi::c_int as uint32_t,
    696 as ::core::ffi::c_int as uint32_t,
    695 as ::core::ffi::c_int as uint32_t,
    694 as ::core::ffi::c_int as uint32_t,
    693 as ::core::ffi::c_int as uint32_t,
    692 as ::core::ffi::c_int as uint32_t,
    691 as ::core::ffi::c_int as uint32_t,
    690 as ::core::ffi::c_int as uint32_t,
    689 as ::core::ffi::c_int as uint32_t,
    688 as ::core::ffi::c_int as uint32_t,
    688 as ::core::ffi::c_int as uint32_t,
    687 as ::core::ffi::c_int as uint32_t,
    686 as ::core::ffi::c_int as uint32_t,
    685 as ::core::ffi::c_int as uint32_t,
    684 as ::core::ffi::c_int as uint32_t,
    683 as ::core::ffi::c_int as uint32_t,
    682 as ::core::ffi::c_int as uint32_t,
    681 as ::core::ffi::c_int as uint32_t,
    680 as ::core::ffi::c_int as uint32_t,
    680 as ::core::ffi::c_int as uint32_t,
    679 as ::core::ffi::c_int as uint32_t,
    678 as ::core::ffi::c_int as uint32_t,
    677 as ::core::ffi::c_int as uint32_t,
    676 as ::core::ffi::c_int as uint32_t,
    675 as ::core::ffi::c_int as uint32_t,
    674 as ::core::ffi::c_int as uint32_t,
    673 as ::core::ffi::c_int as uint32_t,
    673 as ::core::ffi::c_int as uint32_t,
    672 as ::core::ffi::c_int as uint32_t,
    671 as ::core::ffi::c_int as uint32_t,
    670 as ::core::ffi::c_int as uint32_t,
    669 as ::core::ffi::c_int as uint32_t,
    668 as ::core::ffi::c_int as uint32_t,
    667 as ::core::ffi::c_int as uint32_t,
    667 as ::core::ffi::c_int as uint32_t,
    666 as ::core::ffi::c_int as uint32_t,
    665 as ::core::ffi::c_int as uint32_t,
    664 as ::core::ffi::c_int as uint32_t,
    663 as ::core::ffi::c_int as uint32_t,
    662 as ::core::ffi::c_int as uint32_t,
    661 as ::core::ffi::c_int as uint32_t,
    661 as ::core::ffi::c_int as uint32_t,
    660 as ::core::ffi::c_int as uint32_t,
    659 as ::core::ffi::c_int as uint32_t,
    658 as ::core::ffi::c_int as uint32_t,
    657 as ::core::ffi::c_int as uint32_t,
    657 as ::core::ffi::c_int as uint32_t,
    656 as ::core::ffi::c_int as uint32_t,
    655 as ::core::ffi::c_int as uint32_t,
    654 as ::core::ffi::c_int as uint32_t,
    653 as ::core::ffi::c_int as uint32_t,
    652 as ::core::ffi::c_int as uint32_t,
    652 as ::core::ffi::c_int as uint32_t,
    651 as ::core::ffi::c_int as uint32_t,
    650 as ::core::ffi::c_int as uint32_t,
    649 as ::core::ffi::c_int as uint32_t,
    648 as ::core::ffi::c_int as uint32_t,
    648 as ::core::ffi::c_int as uint32_t,
    647 as ::core::ffi::c_int as uint32_t,
    646 as ::core::ffi::c_int as uint32_t,
    645 as ::core::ffi::c_int as uint32_t,
    644 as ::core::ffi::c_int as uint32_t,
    644 as ::core::ffi::c_int as uint32_t,
    643 as ::core::ffi::c_int as uint32_t,
    642 as ::core::ffi::c_int as uint32_t,
    641 as ::core::ffi::c_int as uint32_t,
    640 as ::core::ffi::c_int as uint32_t,
    640 as ::core::ffi::c_int as uint32_t,
    639 as ::core::ffi::c_int as uint32_t,
    638 as ::core::ffi::c_int as uint32_t,
    637 as ::core::ffi::c_int as uint32_t,
    637 as ::core::ffi::c_int as uint32_t,
    636 as ::core::ffi::c_int as uint32_t,
    635 as ::core::ffi::c_int as uint32_t,
    634 as ::core::ffi::c_int as uint32_t,
    633 as ::core::ffi::c_int as uint32_t,
    633 as ::core::ffi::c_int as uint32_t,
    632 as ::core::ffi::c_int as uint32_t,
    631 as ::core::ffi::c_int as uint32_t,
    630 as ::core::ffi::c_int as uint32_t,
    630 as ::core::ffi::c_int as uint32_t,
    629 as ::core::ffi::c_int as uint32_t,
    628 as ::core::ffi::c_int as uint32_t,
    627 as ::core::ffi::c_int as uint32_t,
    627 as ::core::ffi::c_int as uint32_t,
    626 as ::core::ffi::c_int as uint32_t,
    625 as ::core::ffi::c_int as uint32_t,
    624 as ::core::ffi::c_int as uint32_t,
    624 as ::core::ffi::c_int as uint32_t,
    623 as ::core::ffi::c_int as uint32_t,
    622 as ::core::ffi::c_int as uint32_t,
    621 as ::core::ffi::c_int as uint32_t,
    621 as ::core::ffi::c_int as uint32_t,
    620 as ::core::ffi::c_int as uint32_t,
    619 as ::core::ffi::c_int as uint32_t,
    618 as ::core::ffi::c_int as uint32_t,
    618 as ::core::ffi::c_int as uint32_t,
    617 as ::core::ffi::c_int as uint32_t,
    616 as ::core::ffi::c_int as uint32_t,
    616 as ::core::ffi::c_int as uint32_t,
    615 as ::core::ffi::c_int as uint32_t,
    614 as ::core::ffi::c_int as uint32_t,
    613 as ::core::ffi::c_int as uint32_t,
    613 as ::core::ffi::c_int as uint32_t,
    612 as ::core::ffi::c_int as uint32_t,
    611 as ::core::ffi::c_int as uint32_t,
    611 as ::core::ffi::c_int as uint32_t,
    610 as ::core::ffi::c_int as uint32_t,
    609 as ::core::ffi::c_int as uint32_t,
    608 as ::core::ffi::c_int as uint32_t,
    608 as ::core::ffi::c_int as uint32_t,
    607 as ::core::ffi::c_int as uint32_t,
    606 as ::core::ffi::c_int as uint32_t,
    606 as ::core::ffi::c_int as uint32_t,
    605 as ::core::ffi::c_int as uint32_t,
    604 as ::core::ffi::c_int as uint32_t,
    604 as ::core::ffi::c_int as uint32_t,
    603 as ::core::ffi::c_int as uint32_t,
    602 as ::core::ffi::c_int as uint32_t,
    601 as ::core::ffi::c_int as uint32_t,
    601 as ::core::ffi::c_int as uint32_t,
    600 as ::core::ffi::c_int as uint32_t,
    599 as ::core::ffi::c_int as uint32_t,
    599 as ::core::ffi::c_int as uint32_t,
    598 as ::core::ffi::c_int as uint32_t,
    597 as ::core::ffi::c_int as uint32_t,
    597 as ::core::ffi::c_int as uint32_t,
    596 as ::core::ffi::c_int as uint32_t,
    595 as ::core::ffi::c_int as uint32_t,
    595 as ::core::ffi::c_int as uint32_t,
    594 as ::core::ffi::c_int as uint32_t,
    593 as ::core::ffi::c_int as uint32_t,
    593 as ::core::ffi::c_int as uint32_t,
    592 as ::core::ffi::c_int as uint32_t,
    591 as ::core::ffi::c_int as uint32_t,
    591 as ::core::ffi::c_int as uint32_t,
    590 as ::core::ffi::c_int as uint32_t,
    589 as ::core::ffi::c_int as uint32_t,
    589 as ::core::ffi::c_int as uint32_t,
    588 as ::core::ffi::c_int as uint32_t,
    587 as ::core::ffi::c_int as uint32_t,
    587 as ::core::ffi::c_int as uint32_t,
    586 as ::core::ffi::c_int as uint32_t,
    585 as ::core::ffi::c_int as uint32_t,
    585 as ::core::ffi::c_int as uint32_t,
    584 as ::core::ffi::c_int as uint32_t,
    583 as ::core::ffi::c_int as uint32_t,
    583 as ::core::ffi::c_int as uint32_t,
    582 as ::core::ffi::c_int as uint32_t,
    581 as ::core::ffi::c_int as uint32_t,
    581 as ::core::ffi::c_int as uint32_t,
    580 as ::core::ffi::c_int as uint32_t,
    579 as ::core::ffi::c_int as uint32_t,
    579 as ::core::ffi::c_int as uint32_t,
    578 as ::core::ffi::c_int as uint32_t,
    578 as ::core::ffi::c_int as uint32_t,
    577 as ::core::ffi::c_int as uint32_t,
    576 as ::core::ffi::c_int as uint32_t,
    576 as ::core::ffi::c_int as uint32_t,
    575 as ::core::ffi::c_int as uint32_t,
    574 as ::core::ffi::c_int as uint32_t,
    574 as ::core::ffi::c_int as uint32_t,
    573 as ::core::ffi::c_int as uint32_t,
    572 as ::core::ffi::c_int as uint32_t,
    572 as ::core::ffi::c_int as uint32_t,
    571 as ::core::ffi::c_int as uint32_t,
    571 as ::core::ffi::c_int as uint32_t,
    570 as ::core::ffi::c_int as uint32_t,
    569 as ::core::ffi::c_int as uint32_t,
    569 as ::core::ffi::c_int as uint32_t,
    568 as ::core::ffi::c_int as uint32_t,
    568 as ::core::ffi::c_int as uint32_t,
    567 as ::core::ffi::c_int as uint32_t,
    566 as ::core::ffi::c_int as uint32_t,
    566 as ::core::ffi::c_int as uint32_t,
    565 as ::core::ffi::c_int as uint32_t,
    564 as ::core::ffi::c_int as uint32_t,
    564 as ::core::ffi::c_int as uint32_t,
    563 as ::core::ffi::c_int as uint32_t,
    563 as ::core::ffi::c_int as uint32_t,
    562 as ::core::ffi::c_int as uint32_t,
    561 as ::core::ffi::c_int as uint32_t,
    561 as ::core::ffi::c_int as uint32_t,
    560 as ::core::ffi::c_int as uint32_t,
    560 as ::core::ffi::c_int as uint32_t,
    559 as ::core::ffi::c_int as uint32_t,
    558 as ::core::ffi::c_int as uint32_t,
    558 as ::core::ffi::c_int as uint32_t,
    557 as ::core::ffi::c_int as uint32_t,
    557 as ::core::ffi::c_int as uint32_t,
    556 as ::core::ffi::c_int as uint32_t,
    555 as ::core::ffi::c_int as uint32_t,
    555 as ::core::ffi::c_int as uint32_t,
    554 as ::core::ffi::c_int as uint32_t,
    554 as ::core::ffi::c_int as uint32_t,
    553 as ::core::ffi::c_int as uint32_t,
    553 as ::core::ffi::c_int as uint32_t,
    552 as ::core::ffi::c_int as uint32_t,
    551 as ::core::ffi::c_int as uint32_t,
    551 as ::core::ffi::c_int as uint32_t,
    550 as ::core::ffi::c_int as uint32_t,
    550 as ::core::ffi::c_int as uint32_t,
    549 as ::core::ffi::c_int as uint32_t,
    548 as ::core::ffi::c_int as uint32_t,
    548 as ::core::ffi::c_int as uint32_t,
    547 as ::core::ffi::c_int as uint32_t,
    547 as ::core::ffi::c_int as uint32_t,
    546 as ::core::ffi::c_int as uint32_t,
    546 as ::core::ffi::c_int as uint32_t,
    545 as ::core::ffi::c_int as uint32_t,
    544 as ::core::ffi::c_int as uint32_t,
    544 as ::core::ffi::c_int as uint32_t,
    543 as ::core::ffi::c_int as uint32_t,
    543 as ::core::ffi::c_int as uint32_t,
    542 as ::core::ffi::c_int as uint32_t,
    542 as ::core::ffi::c_int as uint32_t,
    541 as ::core::ffi::c_int as uint32_t,
    541 as ::core::ffi::c_int as uint32_t,
    540 as ::core::ffi::c_int as uint32_t,
    539 as ::core::ffi::c_int as uint32_t,
    539 as ::core::ffi::c_int as uint32_t,
    538 as ::core::ffi::c_int as uint32_t,
    538 as ::core::ffi::c_int as uint32_t,
    537 as ::core::ffi::c_int as uint32_t,
    537 as ::core::ffi::c_int as uint32_t,
    536 as ::core::ffi::c_int as uint32_t,
    536 as ::core::ffi::c_int as uint32_t,
    535 as ::core::ffi::c_int as uint32_t,
    534 as ::core::ffi::c_int as uint32_t,
    534 as ::core::ffi::c_int as uint32_t,
    533 as ::core::ffi::c_int as uint32_t,
    533 as ::core::ffi::c_int as uint32_t,
    532 as ::core::ffi::c_int as uint32_t,
    532 as ::core::ffi::c_int as uint32_t,
    531 as ::core::ffi::c_int as uint32_t,
    531 as ::core::ffi::c_int as uint32_t,
    530 as ::core::ffi::c_int as uint32_t,
    530 as ::core::ffi::c_int as uint32_t,
    529 as ::core::ffi::c_int as uint32_t,
    529 as ::core::ffi::c_int as uint32_t,
    528 as ::core::ffi::c_int as uint32_t,
    527 as ::core::ffi::c_int as uint32_t,
    527 as ::core::ffi::c_int as uint32_t,
    526 as ::core::ffi::c_int as uint32_t,
    526 as ::core::ffi::c_int as uint32_t,
    525 as ::core::ffi::c_int as uint32_t,
    525 as ::core::ffi::c_int as uint32_t,
    524 as ::core::ffi::c_int as uint32_t,
    524 as ::core::ffi::c_int as uint32_t,
    523 as ::core::ffi::c_int as uint32_t,
    523 as ::core::ffi::c_int as uint32_t,
    522 as ::core::ffi::c_int as uint32_t,
    522 as ::core::ffi::c_int as uint32_t,
    521 as ::core::ffi::c_int as uint32_t,
    521 as ::core::ffi::c_int as uint32_t,
    520 as ::core::ffi::c_int as uint32_t,
    520 as ::core::ffi::c_int as uint32_t,
    519 as ::core::ffi::c_int as uint32_t,
    519 as ::core::ffi::c_int as uint32_t,
    518 as ::core::ffi::c_int as uint32_t,
    518 as ::core::ffi::c_int as uint32_t,
    517 as ::core::ffi::c_int as uint32_t,
    517 as ::core::ffi::c_int as uint32_t,
    516 as ::core::ffi::c_int as uint32_t,
    516 as ::core::ffi::c_int as uint32_t,
    515 as ::core::ffi::c_int as uint32_t,
    515 as ::core::ffi::c_int as uint32_t,
    514 as ::core::ffi::c_int as uint32_t,
    514 as ::core::ffi::c_int as uint32_t,
];
#[inline]
unsafe extern "C" fn LinearToGammaWeighted(
    mut src: *const uint8_t,
    mut a_ptr: *const uint8_t,
    mut total_a: uint32_t,
    mut step: ::core::ffi::c_int,
    mut rgb_stride: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let sum: uint32_t = (*a_ptr.offset(0 as ::core::ffi::c_int as isize) as uint32_t)
        .wrapping_mul(GammaToLinear(*src.offset(0 as ::core::ffi::c_int as isize)) as uint32_t)
        .wrapping_add(
            (*a_ptr.offset(step as isize) as uint32_t)
                .wrapping_mul(GammaToLinear(*src.offset(step as isize)) as uint32_t),
        )
        .wrapping_add(
            (*a_ptr.offset(rgb_stride as isize) as uint32_t)
                .wrapping_mul(GammaToLinear(*src.offset(rgb_stride as isize)) as uint32_t),
        )
        .wrapping_add(
            (*a_ptr.offset((rgb_stride + step) as isize) as uint32_t)
                .wrapping_mul(GammaToLinear(*src.offset((rgb_stride + step) as isize)) as uint32_t),
        );
    '_c2rust_label: {
        if total_a > 0 as uint32_t
            && total_a <= (4 as ::core::ffi::c_int * 0xff as ::core::ffi::c_int) as uint32_t
        {
        } else {
            __assert_fail(
                b"total_a > 0 && total_a <= 4 * 0xff\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/picture_csp_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                368 as ::core::ffi::c_uint,
                b"int LinearToGammaWeighted(const uint8_t *, const uint8_t *, uint32_t, int, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (sum as uint64_t).wrapping_mul(kInvAlpha[total_a as usize] as uint64_t)
            < (1 as ::core::ffi::c_int as uint64_t) << 32 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"(uint64_t)sum * kInvAlpha[total_a] < ((uint64_t)1 << 32)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/picture_csp_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                370 as ::core::ffi::c_uint,
                b"int LinearToGammaWeighted(const uint8_t *, const uint8_t *, uint32_t, int, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    return LinearToGamma(
        sum.wrapping_mul(kInvAlpha[total_a as usize]) >> kAlphaFix - 2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C" fn ConvertRowToY(
    r_ptr: *const uint8_t,
    g_ptr: *const uint8_t,
    b_ptr: *const uint8_t,
    mut step: ::core::ffi::c_int,
    dst_y: *mut uint8_t,
    mut width: ::core::ffi::c_int,
    rg: *mut VP8Random,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while i < width {
        *dst_y.offset(i as isize) = RGBToY(
            *r_ptr.offset(j as isize) as ::core::ffi::c_int,
            *g_ptr.offset(j as isize) as ::core::ffi::c_int,
            *b_ptr.offset(j as isize) as ::core::ffi::c_int,
            rg,
        ) as uint8_t;
        i += 1 as ::core::ffi::c_int;
        j += step;
    }
}
#[inline]
unsafe extern "C" fn AccumulateRGBA(
    r_ptr: *const uint8_t,
    g_ptr: *const uint8_t,
    b_ptr: *const uint8_t,
    a_ptr: *const uint8_t,
    mut rgb_stride: ::core::ffi::c_int,
    mut dst: *mut uint16_t,
    mut width: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while i < width >> 1 as ::core::ffi::c_int {
        let a: uint32_t = (*a_ptr
            .offset(j as isize)
            .offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *a_ptr.offset(j as isize).offset(rgb_stride as isize) as ::core::ffi::c_int
            + (*a_ptr
                .offset(j as isize)
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *a_ptr
                    .offset(j as isize)
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset(rgb_stride as isize) as ::core::ffi::c_int))
            as uint32_t;
        let mut r: ::core::ffi::c_int = 0;
        let mut g: ::core::ffi::c_int = 0;
        let mut b: ::core::ffi::c_int = 0;
        if a == (4 as ::core::ffi::c_int * 0xff as ::core::ffi::c_int) as uint32_t
            || a == 0 as uint32_t
        {
            r = LinearToGamma(
                GammaToLinear(
                    *r_ptr
                        .offset(j as isize)
                        .offset(0 as ::core::ffi::c_int as isize),
                )
                .wrapping_add(GammaToLinear(
                    *r_ptr
                        .offset(j as isize)
                        .offset(4 as ::core::ffi::c_int as isize),
                ))
                .wrapping_add(GammaToLinear(
                    *r_ptr.offset(j as isize).offset(rgb_stride as isize),
                ))
                .wrapping_add(GammaToLinear(
                    *r_ptr
                        .offset(j as isize)
                        .offset((rgb_stride + 4 as ::core::ffi::c_int) as isize),
                )),
                0 as ::core::ffi::c_int,
            );
            g = LinearToGamma(
                GammaToLinear(
                    *g_ptr
                        .offset(j as isize)
                        .offset(0 as ::core::ffi::c_int as isize),
                )
                .wrapping_add(GammaToLinear(
                    *g_ptr
                        .offset(j as isize)
                        .offset(4 as ::core::ffi::c_int as isize),
                ))
                .wrapping_add(GammaToLinear(
                    *g_ptr.offset(j as isize).offset(rgb_stride as isize),
                ))
                .wrapping_add(GammaToLinear(
                    *g_ptr
                        .offset(j as isize)
                        .offset((rgb_stride + 4 as ::core::ffi::c_int) as isize),
                )),
                0 as ::core::ffi::c_int,
            );
            b = LinearToGamma(
                GammaToLinear(
                    *b_ptr
                        .offset(j as isize)
                        .offset(0 as ::core::ffi::c_int as isize),
                )
                .wrapping_add(GammaToLinear(
                    *b_ptr
                        .offset(j as isize)
                        .offset(4 as ::core::ffi::c_int as isize),
                ))
                .wrapping_add(GammaToLinear(
                    *b_ptr.offset(j as isize).offset(rgb_stride as isize),
                ))
                .wrapping_add(GammaToLinear(
                    *b_ptr
                        .offset(j as isize)
                        .offset((rgb_stride + 4 as ::core::ffi::c_int) as isize),
                )),
                0 as ::core::ffi::c_int,
            );
        } else {
            r = LinearToGammaWeighted(
                r_ptr.offset(j as isize),
                a_ptr.offset(j as isize),
                a,
                4 as ::core::ffi::c_int,
                rgb_stride,
            );
            g = LinearToGammaWeighted(
                g_ptr.offset(j as isize),
                a_ptr.offset(j as isize),
                a,
                4 as ::core::ffi::c_int,
                rgb_stride,
            );
            b = LinearToGammaWeighted(
                b_ptr.offset(j as isize),
                a_ptr.offset(j as isize),
                a,
                4 as ::core::ffi::c_int,
                rgb_stride,
            );
        }
        *dst.offset(0 as ::core::ffi::c_int as isize) = r as uint16_t;
        *dst.offset(1 as ::core::ffi::c_int as isize) = g as uint16_t;
        *dst.offset(2 as ::core::ffi::c_int as isize) = b as uint16_t;
        *dst.offset(3 as ::core::ffi::c_int as isize) = a as uint16_t;
        i += 1 as ::core::ffi::c_int;
        j += 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int;
        dst = dst.offset(4 as ::core::ffi::c_int as isize);
    }
    if width & 1 as ::core::ffi::c_int != 0 {
        let a_0: uint32_t = (2 as uint32_t).wrapping_mul(
            (*a_ptr
                .offset(j as isize)
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *a_ptr.offset(j as isize).offset(rgb_stride as isize) as ::core::ffi::c_int)
                as uint32_t,
        );
        let mut r_0: ::core::ffi::c_int = 0;
        let mut g_0: ::core::ffi::c_int = 0;
        let mut b_0: ::core::ffi::c_int = 0;
        if a_0 == (4 as ::core::ffi::c_int * 0xff as ::core::ffi::c_int) as uint32_t
            || a_0 == 0 as uint32_t
        {
            r_0 = LinearToGamma(
                GammaToLinear(
                    *r_ptr
                        .offset(j as isize)
                        .offset(0 as ::core::ffi::c_int as isize),
                )
                .wrapping_add(GammaToLinear(
                    *r_ptr.offset(j as isize).offset(rgb_stride as isize),
                )),
                1 as ::core::ffi::c_int,
            );
            g_0 = LinearToGamma(
                GammaToLinear(
                    *g_ptr
                        .offset(j as isize)
                        .offset(0 as ::core::ffi::c_int as isize),
                )
                .wrapping_add(GammaToLinear(
                    *g_ptr.offset(j as isize).offset(rgb_stride as isize),
                )),
                1 as ::core::ffi::c_int,
            );
            b_0 = LinearToGamma(
                GammaToLinear(
                    *b_ptr
                        .offset(j as isize)
                        .offset(0 as ::core::ffi::c_int as isize),
                )
                .wrapping_add(GammaToLinear(
                    *b_ptr.offset(j as isize).offset(rgb_stride as isize),
                )),
                1 as ::core::ffi::c_int,
            );
        } else {
            r_0 = LinearToGammaWeighted(
                r_ptr.offset(j as isize),
                a_ptr.offset(j as isize),
                a_0,
                0 as ::core::ffi::c_int,
                rgb_stride,
            );
            g_0 = LinearToGammaWeighted(
                g_ptr.offset(j as isize),
                a_ptr.offset(j as isize),
                a_0,
                0 as ::core::ffi::c_int,
                rgb_stride,
            );
            b_0 = LinearToGammaWeighted(
                b_ptr.offset(j as isize),
                a_ptr.offset(j as isize),
                a_0,
                0 as ::core::ffi::c_int,
                rgb_stride,
            );
        }
        *dst.offset(0 as ::core::ffi::c_int as isize) = r_0 as uint16_t;
        *dst.offset(1 as ::core::ffi::c_int as isize) = g_0 as uint16_t;
        *dst.offset(2 as ::core::ffi::c_int as isize) = b_0 as uint16_t;
        *dst.offset(3 as ::core::ffi::c_int as isize) = a_0 as uint16_t;
    }
}
#[inline]
unsafe extern "C" fn AccumulateRGB(
    r_ptr: *const uint8_t,
    g_ptr: *const uint8_t,
    b_ptr: *const uint8_t,
    mut step: ::core::ffi::c_int,
    mut rgb_stride: ::core::ffi::c_int,
    mut dst: *mut uint16_t,
    mut width: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while i < width >> 1 as ::core::ffi::c_int {
        *dst.offset(0 as ::core::ffi::c_int as isize) = LinearToGamma(
            GammaToLinear(
                *r_ptr
                    .offset(j as isize)
                    .offset(0 as ::core::ffi::c_int as isize),
            )
            .wrapping_add(GammaToLinear(
                *r_ptr.offset(j as isize).offset(step as isize),
            ))
            .wrapping_add(GammaToLinear(
                *r_ptr.offset(j as isize).offset(rgb_stride as isize),
            ))
            .wrapping_add(GammaToLinear(
                *r_ptr
                    .offset(j as isize)
                    .offset((rgb_stride + step) as isize),
            )),
            0 as ::core::ffi::c_int,
        ) as uint16_t;
        *dst.offset(1 as ::core::ffi::c_int as isize) = LinearToGamma(
            GammaToLinear(
                *g_ptr
                    .offset(j as isize)
                    .offset(0 as ::core::ffi::c_int as isize),
            )
            .wrapping_add(GammaToLinear(
                *g_ptr.offset(j as isize).offset(step as isize),
            ))
            .wrapping_add(GammaToLinear(
                *g_ptr.offset(j as isize).offset(rgb_stride as isize),
            ))
            .wrapping_add(GammaToLinear(
                *g_ptr
                    .offset(j as isize)
                    .offset((rgb_stride + step) as isize),
            )),
            0 as ::core::ffi::c_int,
        ) as uint16_t;
        *dst.offset(2 as ::core::ffi::c_int as isize) = LinearToGamma(
            GammaToLinear(
                *b_ptr
                    .offset(j as isize)
                    .offset(0 as ::core::ffi::c_int as isize),
            )
            .wrapping_add(GammaToLinear(
                *b_ptr.offset(j as isize).offset(step as isize),
            ))
            .wrapping_add(GammaToLinear(
                *b_ptr.offset(j as isize).offset(rgb_stride as isize),
            ))
            .wrapping_add(GammaToLinear(
                *b_ptr
                    .offset(j as isize)
                    .offset((rgb_stride + step) as isize),
            )),
            0 as ::core::ffi::c_int,
        ) as uint16_t;
        i += 1 as ::core::ffi::c_int;
        j += 2 as ::core::ffi::c_int * step;
        dst = dst.offset(4 as ::core::ffi::c_int as isize);
    }
    if width & 1 as ::core::ffi::c_int != 0 {
        *dst.offset(0 as ::core::ffi::c_int as isize) = LinearToGamma(
            GammaToLinear(
                *r_ptr
                    .offset(j as isize)
                    .offset(0 as ::core::ffi::c_int as isize),
            )
            .wrapping_add(GammaToLinear(
                *r_ptr.offset(j as isize).offset(rgb_stride as isize),
            )),
            1 as ::core::ffi::c_int,
        ) as uint16_t;
        *dst.offset(1 as ::core::ffi::c_int as isize) = LinearToGamma(
            GammaToLinear(
                *g_ptr
                    .offset(j as isize)
                    .offset(0 as ::core::ffi::c_int as isize),
            )
            .wrapping_add(GammaToLinear(
                *g_ptr.offset(j as isize).offset(rgb_stride as isize),
            )),
            1 as ::core::ffi::c_int,
        ) as uint16_t;
        *dst.offset(2 as ::core::ffi::c_int as isize) = LinearToGamma(
            GammaToLinear(
                *b_ptr
                    .offset(j as isize)
                    .offset(0 as ::core::ffi::c_int as isize),
            )
            .wrapping_add(GammaToLinear(
                *b_ptr.offset(j as isize).offset(rgb_stride as isize),
            )),
            1 as ::core::ffi::c_int,
        ) as uint16_t;
    }
}
#[inline]
unsafe extern "C" fn ConvertRowsToUV(
    mut rgb: *const uint16_t,
    dst_u: *mut uint8_t,
    dst_v: *mut uint8_t,
    mut width: ::core::ffi::c_int,
    rg: *mut VP8Random,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < width {
        let r: ::core::ffi::c_int =
            *rgb.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let g: ::core::ffi::c_int =
            *rgb.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let b: ::core::ffi::c_int =
            *rgb.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        *dst_u.offset(i as isize) = RGBToU(r, g, b, rg) as uint8_t;
        *dst_v.offset(i as isize) = RGBToV(r, g, b, rg) as uint8_t;
        i += 1 as ::core::ffi::c_int;
        rgb = rgb.offset(4 as ::core::ffi::c_int as isize);
    }
}
unsafe extern "C" fn ImportYUVAFromRGBA(
    mut r_ptr: *const uint8_t,
    mut g_ptr: *const uint8_t,
    mut b_ptr: *const uint8_t,
    mut a_ptr: *const uint8_t,
    mut step: ::core::ffi::c_int,
    mut rgb_stride: ::core::ffi::c_int,
    mut dithering: ::core::ffi::c_float,
    mut use_iterative_conversion: ::core::ffi::c_int,
    picture: *mut WebPPicture,
) -> ::core::ffi::c_int {
    let mut y: ::core::ffi::c_int = 0;
    let width: ::core::ffi::c_int = (*picture).width;
    let height: ::core::ffi::c_int = (*picture).height;
    let has_alpha: ::core::ffi::c_int =
        CheckNonOpaque(a_ptr, width, height, step, rgb_stride) as ::core::ffi::c_int;
    let is_rgb: ::core::ffi::c_int = (r_ptr < b_ptr) as ::core::ffi::c_int;
    (*picture).colorspace = (if has_alpha != 0 {
        WEBP_YUV420A as ::core::ffi::c_int
    } else {
        WEBP_YUV420 as ::core::ffi::c_int
    }) as WebPEncCSP;
    (*picture).use_argb = 0 as ::core::ffi::c_int;
    if width < kMinDimensionIterativeConversion || height < kMinDimensionIterativeConversion {
        use_iterative_conversion = 0 as ::core::ffi::c_int;
    }
    if WebPPictureAllocYUVA(picture) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if has_alpha != 0 {
        '_c2rust_label: {
            if step == 4 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"step == 4\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/picture_csp_enc.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    502 as ::core::ffi::c_uint,
                    b"int ImportYUVAFromRGBA(const uint8_t *, const uint8_t *, const uint8_t *, const uint8_t *, int, int, float, int, WebPPicture *const)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_0: {
            if kAlphaFix + 12 as ::core::ffi::c_int <= 31 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"kAlphaFix + GAMMA_FIX <= 31\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/picture_csp_enc.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    504 as ::core::ffi::c_uint,
                    b"int ImportYUVAFromRGBA(const uint8_t *, const uint8_t *, const uint8_t *, const uint8_t *, int, int, float, int, WebPPicture *const)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
    }
    if use_iterative_conversion != 0 {
        SharpYuvInit(VP8GetCPUInfo);
        if PreprocessARGB(r_ptr, g_ptr, b_ptr, step, rgb_stride, picture) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        if has_alpha != 0 {
            WebPExtractAlpha.expect("non-null function pointer")(
                a_ptr,
                rgb_stride,
                width,
                height,
                (*picture).a,
                (*picture).a_stride,
            );
        }
    } else {
        let uv_width: ::core::ffi::c_int =
            width + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
        let mut use_dsp: ::core::ffi::c_int =
            (step == 3 as ::core::ffi::c_int) as ::core::ffi::c_int;
        let tmp_rgb: *mut uint16_t = WebPSafeMalloc(
            (4 as ::core::ffi::c_int * uv_width) as uint64_t,
            ::core::mem::size_of::<uint16_t>() as size_t,
        ) as *mut uint16_t;
        let mut dst_y: *mut uint8_t = (*picture).y;
        let mut dst_u: *mut uint8_t = (*picture).u;
        let mut dst_v: *mut uint8_t = (*picture).v;
        let mut dst_a: *mut uint8_t = (*picture).a;
        let mut base_rg: VP8Random = VP8Random {
            index1_: 0,
            index2_: 0,
            tab_: [0; 55],
            amp_: 0,
        };
        let mut rg: *mut VP8Random = ::core::ptr::null_mut::<VP8Random>();
        if dithering as ::core::ffi::c_double > 0.0f64 {
            VP8InitRandom(&raw mut base_rg, dithering);
            rg = &raw mut base_rg;
            use_dsp = 0 as ::core::ffi::c_int;
        }
        WebPInitConvertARGBToYUV();
        InitGammaTables();
        if tmp_rgb.is_null() {
            return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
        }
        y = 0 as ::core::ffi::c_int;
        while y < height >> 1 as ::core::ffi::c_int {
            let mut rows_have_alpha: ::core::ffi::c_int = has_alpha;
            if use_dsp != 0 {
                if is_rgb != 0 {
                    WebPConvertRGB24ToY.expect("non-null function pointer")(r_ptr, dst_y, width);
                    WebPConvertRGB24ToY.expect("non-null function pointer")(
                        r_ptr.offset(rgb_stride as isize),
                        dst_y.offset((*picture).y_stride as isize),
                        width,
                    );
                } else {
                    WebPConvertBGR24ToY.expect("non-null function pointer")(b_ptr, dst_y, width);
                    WebPConvertBGR24ToY.expect("non-null function pointer")(
                        b_ptr.offset(rgb_stride as isize),
                        dst_y.offset((*picture).y_stride as isize),
                        width,
                    );
                }
            } else {
                ConvertRowToY(r_ptr, g_ptr, b_ptr, step, dst_y, width, rg);
                ConvertRowToY(
                    r_ptr.offset(rgb_stride as isize),
                    g_ptr.offset(rgb_stride as isize),
                    b_ptr.offset(rgb_stride as isize),
                    step,
                    dst_y.offset((*picture).y_stride as isize),
                    width,
                    rg,
                );
            }
            dst_y = dst_y.offset((2 as ::core::ffi::c_int * (*picture).y_stride) as isize);
            if has_alpha != 0 {
                rows_have_alpha &= (WebPExtractAlpha.expect("non-null function pointer")(
                    a_ptr,
                    rgb_stride,
                    width,
                    2 as ::core::ffi::c_int,
                    dst_a,
                    (*picture).a_stride,
                ) == 0) as ::core::ffi::c_int;
                dst_a = dst_a.offset((2 as ::core::ffi::c_int * (*picture).a_stride) as isize);
            }
            if rows_have_alpha == 0 {
                AccumulateRGB(r_ptr, g_ptr, b_ptr, step, rgb_stride, tmp_rgb, width);
            } else {
                AccumulateRGBA(r_ptr, g_ptr, b_ptr, a_ptr, rgb_stride, tmp_rgb, width);
            }
            if rg.is_null() {
                WebPConvertRGBA32ToUV.expect("non-null function pointer")(
                    tmp_rgb, dst_u, dst_v, uv_width,
                );
            } else {
                ConvertRowsToUV(tmp_rgb, dst_u, dst_v, uv_width, rg);
            }
            dst_u = dst_u.offset((*picture).uv_stride as isize);
            dst_v = dst_v.offset((*picture).uv_stride as isize);
            r_ptr = r_ptr.offset((2 as ::core::ffi::c_int * rgb_stride) as isize);
            b_ptr = b_ptr.offset((2 as ::core::ffi::c_int * rgb_stride) as isize);
            g_ptr = g_ptr.offset((2 as ::core::ffi::c_int * rgb_stride) as isize);
            if has_alpha != 0 {
                a_ptr = a_ptr.offset((2 as ::core::ffi::c_int * rgb_stride) as isize);
            }
            y += 1;
        }
        if height & 1 as ::core::ffi::c_int != 0 {
            let mut row_has_alpha: ::core::ffi::c_int = has_alpha;
            if use_dsp != 0 {
                if r_ptr < b_ptr {
                    WebPConvertRGB24ToY.expect("non-null function pointer")(r_ptr, dst_y, width);
                } else {
                    WebPConvertBGR24ToY.expect("non-null function pointer")(b_ptr, dst_y, width);
                }
            } else {
                ConvertRowToY(r_ptr, g_ptr, b_ptr, step, dst_y, width, rg);
            }
            if row_has_alpha != 0 {
                row_has_alpha &= (WebPExtractAlpha.expect("non-null function pointer")(
                    a_ptr,
                    0 as ::core::ffi::c_int,
                    width,
                    1 as ::core::ffi::c_int,
                    dst_a,
                    0 as ::core::ffi::c_int,
                ) == 0) as ::core::ffi::c_int;
            }
            if row_has_alpha == 0 {
                AccumulateRGB(
                    r_ptr,
                    g_ptr,
                    b_ptr,
                    step,
                    0 as ::core::ffi::c_int,
                    tmp_rgb,
                    width,
                );
            } else {
                AccumulateRGBA(
                    r_ptr,
                    g_ptr,
                    b_ptr,
                    a_ptr,
                    0 as ::core::ffi::c_int,
                    tmp_rgb,
                    width,
                );
            }
            if rg.is_null() {
                WebPConvertRGBA32ToUV.expect("non-null function pointer")(
                    tmp_rgb, dst_u, dst_v, uv_width,
                );
            } else {
                ConvertRowsToUV(tmp_rgb, dst_u, dst_v, uv_width, rg);
            }
        }
        WebPSafeFree(tmp_rgb as *mut ::core::ffi::c_void);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn PictureARGBToYUVA(
    mut picture: *mut WebPPicture,
    mut colorspace: WebPEncCSP,
    mut dithering: ::core::ffi::c_float,
    mut use_iterative_conversion: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if picture.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*picture).argb.is_null() {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_NULL_PARAMETER);
    } else if colorspace as ::core::ffi::c_uint
        & WEBP_CSP_UV_MASK as ::core::ffi::c_int as ::core::ffi::c_uint
        != WEBP_YUV420 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_INVALID_CONFIGURATION);
    } else {
        let argb: *const uint8_t = (*picture).argb as *const uint8_t;
        let a: *const uint8_t =
            argb.offset((3 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) as isize);
        let r: *const uint8_t =
            argb.offset((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
        let g: *const uint8_t =
            argb.offset((3 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize);
        let b: *const uint8_t =
            argb.offset((3 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize);
        (*picture).colorspace = WEBP_YUV420;
        return ImportYUVAFromRGBA(
            r,
            g,
            b,
            a,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int * (*picture).argb_stride,
            dithering,
            use_iterative_conversion,
            picture,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureARGBToYUVADithered(
    mut picture: *mut WebPPicture,
    mut colorspace: WebPEncCSP,
    mut dithering: ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    return PictureARGBToYUVA(picture, colorspace, dithering, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureARGBToYUVA(
    mut picture: *mut WebPPicture,
    mut colorspace: WebPEncCSP,
) -> ::core::ffi::c_int {
    return PictureARGBToYUVA(picture, colorspace, 0.0f32, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureSharpARGBToYUVA(
    mut picture: *mut WebPPicture,
) -> ::core::ffi::c_int {
    return PictureARGBToYUVA(picture, WEBP_YUV420, 0.0f32, 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureSmartARGBToYUVA(
    mut picture: *mut WebPPicture,
) -> ::core::ffi::c_int {
    return WebPPictureSharpARGBToYUVA(picture);
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureYUVAToARGB(
    mut picture: *mut WebPPicture,
) -> ::core::ffi::c_int {
    if picture.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*picture).y.is_null() || (*picture).u.is_null() || (*picture).v.is_null() {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_NULL_PARAMETER);
    }
    if (*picture).colorspace as ::core::ffi::c_uint
        & WEBP_CSP_ALPHA_BIT as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && (*picture).a.is_null()
    {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_NULL_PARAMETER);
    }
    if (*picture).colorspace as ::core::ffi::c_uint
        & WEBP_CSP_UV_MASK as ::core::ffi::c_int as ::core::ffi::c_uint
        != WEBP_YUV420 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_INVALID_CONFIGURATION);
    }
    if WebPPictureAllocARGB(picture) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*picture).use_argb = 1 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0;
    let width: ::core::ffi::c_int = (*picture).width;
    let height: ::core::ffi::c_int = (*picture).height;
    let argb_stride: ::core::ffi::c_int = 4 as ::core::ffi::c_int * (*picture).argb_stride;
    let mut dst: *mut uint8_t = (*picture).argb as *mut uint8_t;
    let mut cur_u: *const uint8_t = (*picture).u;
    let mut cur_v: *const uint8_t = (*picture).v;
    let mut cur_y: *const uint8_t = (*picture).y;
    let mut upsample: WebPUpsampleLinePairFunc =
        WebPGetLinePairConverter((ALPHA_OFFSET > 0 as ::core::ffi::c_int) as ::core::ffi::c_int);
    upsample.expect("non-null function pointer")(
        cur_y,
        ::core::ptr::null::<uint8_t>(),
        cur_u,
        cur_v,
        cur_u,
        cur_v,
        dst,
        ::core::ptr::null_mut::<uint8_t>(),
        width,
    );
    cur_y = cur_y.offset((*picture).y_stride as isize);
    dst = dst.offset(argb_stride as isize);
    y = 1 as ::core::ffi::c_int;
    while (y + 1 as ::core::ffi::c_int) < height {
        let top_u: *const uint8_t = cur_u;
        let top_v: *const uint8_t = cur_v;
        cur_u = cur_u.offset((*picture).uv_stride as isize);
        cur_v = cur_v.offset((*picture).uv_stride as isize);
        upsample.expect("non-null function pointer")(
            cur_y,
            cur_y.offset((*picture).y_stride as isize),
            top_u,
            top_v,
            cur_u,
            cur_v,
            dst,
            dst.offset(argb_stride as isize),
            width,
        );
        cur_y = cur_y.offset((2 as ::core::ffi::c_int * (*picture).y_stride) as isize);
        dst = dst.offset((2 as ::core::ffi::c_int * argb_stride) as isize);
        y += 2 as ::core::ffi::c_int;
    }
    if height > 1 as ::core::ffi::c_int && height & 1 as ::core::ffi::c_int == 0 {
        upsample.expect("non-null function pointer")(
            cur_y,
            ::core::ptr::null::<uint8_t>(),
            cur_u,
            cur_v,
            cur_u,
            cur_v,
            dst,
            ::core::ptr::null_mut::<uint8_t>(),
            width,
        );
    }
    if (*picture).colorspace as ::core::ffi::c_uint
        & WEBP_CSP_ALPHA_BIT as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        y = 0 as ::core::ffi::c_int;
        while y < height {
            let argb_dst: *mut uint32_t = (*picture)
                .argb
                .offset((y * (*picture).argb_stride) as isize);
            let src: *const uint8_t = (*picture).a.offset((y * (*picture).a_stride) as isize);
            let mut x: ::core::ffi::c_int = 0;
            x = 0 as ::core::ffi::c_int;
            while x < width {
                *argb_dst.offset(x as isize) = *argb_dst.offset(x as isize) & 0xffffff as uint32_t
                    | (*src.offset(x as isize) as uint32_t) << 24 as ::core::ffi::c_int;
                x += 1;
            }
            y += 1;
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn Import(
    picture: *mut WebPPicture,
    mut rgb: *const uint8_t,
    mut rgb_stride: ::core::ffi::c_int,
    mut step: ::core::ffi::c_int,
    mut swap_rb: ::core::ffi::c_int,
    mut import_alpha: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut y: ::core::ffi::c_int = 0;
    let mut r_ptr: *const uint8_t = rgb.offset(
        (if swap_rb != 0 {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as isize,
    );
    let mut g_ptr: *const uint8_t = rgb.offset(1 as ::core::ffi::c_int as isize);
    let mut b_ptr: *const uint8_t = rgb.offset(
        (if swap_rb != 0 {
            0 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        }) as isize,
    );
    let width: ::core::ffi::c_int = (*picture).width;
    let height: ::core::ffi::c_int = (*picture).height;
    let Ok(width_usize) = usize::try_from(width) else {
        return 0 as ::core::ffi::c_int;
    };
    let channels = if import_alpha != 0 { 4 } else { 3 };
    let Some(min_stride) = checked_mul_usize(width_usize, channels) else {
        return 0 as ::core::ffi::c_int;
    };
    if ((rgb_stride as i64).unsigned_abs() as usize) < min_stride {
        return 0 as ::core::ffi::c_int;
    }
    if (*picture).use_argb == 0 {
        let mut a_ptr: *const uint8_t = if import_alpha != 0 {
            rgb.offset(3 as ::core::ffi::c_int as isize)
        } else {
            ::core::ptr::null::<uint8_t>()
        };
        return ImportYUVAFromRGBA(
            r_ptr,
            g_ptr,
            b_ptr,
            a_ptr,
            step,
            rgb_stride,
            0.0f32,
            0 as ::core::ffi::c_int,
            picture,
        );
    }
    if WebPPictureAlloc(picture) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    VP8LDspInit();
    WebPInitAlphaProcessing();
    if import_alpha != 0 {
        let mut dst: *mut uint32_t = (*picture).argb;
        let do_copy: ::core::ffi::c_int =
            (ALPHA_OFFSET == 3 as ::core::ffi::c_int && swap_rb != 0) as ::core::ffi::c_int;
        let Some(row_bytes) = checked_mul_usize(width_usize, 4) else {
            return 0 as ::core::ffi::c_int;
        };
        '_c2rust_label: {
            if step == 4 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"step == 4\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/picture_csp_enc.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    759 as ::core::ffi::c_uint,
                    b"int Import(WebPPicture *const, const uint8_t *, int, int, int, int)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        if do_copy != 0 {
            y = 0 as ::core::ffi::c_int;
            while y < height {
                memcpy(
                    dst as *mut ::core::ffi::c_void,
                    rgb as *const ::core::ffi::c_void,
                    row_bytes as size_t,
                );
                rgb = rgb.offset(rgb_stride as isize);
                dst = dst.offset((*picture).argb_stride as isize);
                y += 1;
            }
        } else {
            y = 0 as ::core::ffi::c_int;
            while y < height {
                VP8LConvertBGRAToRGBA.expect("non-null function pointer")(
                    rgb as *const uint32_t,
                    width,
                    dst as *mut uint8_t,
                );
                rgb = rgb.offset(rgb_stride as isize);
                dst = dst.offset((*picture).argb_stride as isize);
                y += 1;
            }
        }
    } else {
        let mut dst_0: *mut uint32_t = (*picture).argb;
        '_c2rust_label_0: {
            if step >= 3 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"step >= 3\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/picture_csp_enc.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    785 as ::core::ffi::c_uint,
                    b"int Import(WebPPicture *const, const uint8_t *, int, int, int, int)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        y = 0 as ::core::ffi::c_int;
        while y < height {
            WebPPackRGB.expect("non-null function pointer")(
                r_ptr, g_ptr, b_ptr, width, step, dst_0,
            );
            r_ptr = r_ptr.offset(rgb_stride as isize);
            g_ptr = g_ptr.offset(rgb_stride as isize);
            b_ptr = b_ptr.offset(rgb_stride as isize);
            dst_0 = dst_0.offset((*picture).argb_stride as isize);
            y += 1;
        }
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureImportBGR(
    mut picture: *mut WebPPicture,
    mut bgr: *const uint8_t,
    mut bgr_stride: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if !picture.is_null() && !bgr.is_null() {
        Import(
            picture,
            bgr,
            bgr_stride,
            3 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        )
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureImportBGRA(
    mut picture: *mut WebPPicture,
    mut bgra: *const uint8_t,
    mut bgra_stride: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if !picture.is_null() && !bgra.is_null() {
        Import(
            picture,
            bgra,
            bgra_stride,
            4 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        )
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureImportBGRX(
    mut picture: *mut WebPPicture,
    mut bgrx: *const uint8_t,
    mut bgrx_stride: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if !picture.is_null() && !bgrx.is_null() {
        Import(
            picture,
            bgrx,
            bgrx_stride,
            4 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        )
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureImportRGB(
    mut picture: *mut WebPPicture,
    mut rgb: *const uint8_t,
    mut rgb_stride: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if !picture.is_null() && !rgb.is_null() {
        Import(
            picture,
            rgb,
            rgb_stride,
            3 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        )
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureImportRGBA(
    mut picture: *mut WebPPicture,
    mut rgba: *const uint8_t,
    mut rgba_stride: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if !picture.is_null() && !rgba.is_null() {
        Import(
            picture,
            rgba,
            rgba_stride,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        )
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureImportRGBX(
    mut picture: *mut WebPPicture,
    mut rgbx: *const uint8_t,
    mut rgbx_stride: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if !picture.is_null() && !rgbx.is_null() {
        Import(
            picture,
            rgbx,
            rgbx_stride,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        )
    } else {
        0 as ::core::ffi::c_int
    };
}
pub const VP8_RANDOM_DITHER_FIX: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const VP8_RANDOM_TABLE_SIZE: ::core::ffi::c_int = 55 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn VP8RandomBits2(
    rg: *mut VP8Random,
    mut num_bits: ::core::ffi::c_int,
    mut amp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut diff: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if num_bits + 8 as ::core::ffi::c_int <= 31 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"num_bits + VP8_RANDOM_DITHER_FIX <= 31\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"original/src/utils/random_utils.h\0" as *const u8 as *const ::core::ffi::c_char,
                42 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION_0.as_ptr(),
            );
        }
    };
    diff = (*rg).tab_[(*rg).index1_ as usize].wrapping_sub((*rg).tab_[(*rg).index2_ as usize])
        as ::core::ffi::c_int;
    if diff < 0 as ::core::ffi::c_int {
        diff = (diff as ::core::ffi::c_uint)
            .wrapping_add((1 as ::core::ffi::c_uint) << 31 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    (*rg).tab_[(*rg).index1_ as usize] = diff as uint32_t;
    (*rg).index1_ += 1;
    if (*rg).index1_ == VP8_RANDOM_TABLE_SIZE {
        (*rg).index1_ = 0 as ::core::ffi::c_int;
    }
    (*rg).index2_ += 1;
    if (*rg).index2_ == VP8_RANDOM_TABLE_SIZE {
        (*rg).index2_ = 0 as ::core::ffi::c_int;
    }
    diff = ((diff as uint32_t) << 1 as ::core::ffi::c_int) as ::core::ffi::c_int
        >> 32 as ::core::ffi::c_int - num_bits;
    diff = diff * amp >> VP8_RANDOM_DITHER_FIX;
    diff += (1 as ::core::ffi::c_int) << num_bits - 1 as ::core::ffi::c_int;
    return diff;
}
#[inline]
unsafe extern "C" fn VP8RandomBits(
    rg: *mut VP8Random,
    mut num_bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return VP8RandomBits2(rg, num_bits, (*rg).amp_);
}
#[inline]
unsafe extern "C" fn VP8ClipUV(
    mut uv: ::core::ffi::c_int,
    mut rounding: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    uv = uv
        + rounding
        + ((128 as ::core::ffi::c_int) << YUV_FIX as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
        >> YUV_FIX as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
    return if uv & !(0xff as ::core::ffi::c_int) == 0 as ::core::ffi::c_int {
        uv
    } else if uv < 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        255 as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn VP8RGBToY(
    mut r: ::core::ffi::c_int,
    mut g: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
    mut rounding: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let luma: ::core::ffi::c_int = 16839 as ::core::ffi::c_int * r
        + 33059 as ::core::ffi::c_int * g
        + 6420 as ::core::ffi::c_int * b;
    return luma + rounding + ((16 as ::core::ffi::c_int) << YUV_FIX as ::core::ffi::c_int)
        >> YUV_FIX as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn VP8RGBToU(
    mut r: ::core::ffi::c_int,
    mut g: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
    mut rounding: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let u: ::core::ffi::c_int = -(9719 as ::core::ffi::c_int) * r - 19081 as ::core::ffi::c_int * g
        + 28800 as ::core::ffi::c_int * b;
    return VP8ClipUV(u, rounding);
}
#[inline]
unsafe extern "C" fn VP8RGBToV(
    mut r: ::core::ffi::c_int,
    mut g: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
    mut rounding: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let v: ::core::ffi::c_int = 28800 as ::core::ffi::c_int * r
        - 24116 as ::core::ffi::c_int * g
        - 4684 as ::core::ffi::c_int * b;
    return VP8ClipUV(v, rounding);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
