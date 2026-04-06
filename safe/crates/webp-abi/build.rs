use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Copy)]
struct TypeLayout {
    c_name: &'static str,
    rust_name: &'static str,
    fields: &'static [&'static str],
}

#[derive(Clone, Copy)]
struct EnumLayout {
    c_name: &'static str,
    rust_name: &'static str,
    values: &'static [(&'static str, &'static str)],
}

const HEADERS: &[&str] = &[
    "webp/types.h",
    "webp/mux_types.h",
    "webp/decode.h",
    "webp/encode.h",
    "webp/demux.h",
    "webp/mux.h",
    "webp/sharpyuv/sharpyuv.h",
    "webp/sharpyuv/sharpyuv_csp.h",
];

const TYPES: &[TypeLayout] = &[
    TypeLayout {
        c_name: "WebPData",
        rust_name: "webp_abi::WebPData",
        fields: &["bytes", "size"],
    },
    TypeLayout {
        c_name: "WebPRGBABuffer",
        rust_name: "webp_abi::WebPRGBABuffer",
        fields: &["rgba", "stride", "size"],
    },
    TypeLayout {
        c_name: "WebPYUVABuffer",
        rust_name: "webp_abi::WebPYUVABuffer",
        fields: &[
            "y", "u", "v", "a", "y_stride", "u_stride", "v_stride", "a_stride", "y_size", "u_size",
            "v_size", "a_size",
        ],
    },
    TypeLayout {
        c_name: "WebPDecBuffer",
        rust_name: "webp_abi::WebPDecBuffer",
        fields: &[
            "colorspace",
            "width",
            "height",
            "is_external_memory",
            "u",
            "pad",
            "private_memory",
        ],
    },
    TypeLayout {
        c_name: "WebPBitstreamFeatures",
        rust_name: "webp_abi::WebPBitstreamFeatures",
        fields: &[
            "width",
            "height",
            "has_alpha",
            "has_animation",
            "format",
            "pad",
        ],
    },
    TypeLayout {
        c_name: "WebPDecoderOptions",
        rust_name: "webp_abi::WebPDecoderOptions",
        fields: &[
            "bypass_filtering",
            "no_fancy_upsampling",
            "use_cropping",
            "crop_left",
            "crop_top",
            "crop_width",
            "crop_height",
            "use_scaling",
            "scaled_width",
            "scaled_height",
            "use_threads",
            "dithering_strength",
            "flip",
            "alpha_dithering_strength",
            "pad",
        ],
    },
    TypeLayout {
        c_name: "WebPDecoderConfig",
        rust_name: "webp_abi::WebPDecoderConfig",
        fields: &["input", "output", "options"],
    },
    TypeLayout {
        c_name: "WebPConfig",
        rust_name: "webp_abi::WebPConfig",
        fields: &[
            "lossless",
            "quality",
            "method",
            "image_hint",
            "target_size",
            "target_PSNR",
            "segments",
            "sns_strength",
            "filter_strength",
            "filter_sharpness",
            "filter_type",
            "autofilter",
            "alpha_compression",
            "alpha_filtering",
            "alpha_quality",
            "pass",
            "show_compressed",
            "preprocessing",
            "partitions",
            "partition_limit",
            "emulate_jpeg_size",
            "thread_level",
            "low_memory",
            "near_lossless",
            "exact",
            "use_delta_palette",
            "use_sharp_yuv",
            "qmin",
            "qmax",
        ],
    },
    TypeLayout {
        c_name: "WebPAuxStats",
        rust_name: "webp_abi::WebPAuxStats",
        fields: &[
            "coded_size",
            "PSNR",
            "block_count",
            "header_bytes",
            "residual_bytes",
            "segment_size",
            "segment_quant",
            "segment_level",
            "alpha_data_size",
            "layer_data_size",
            "lossless_features",
            "histogram_bits",
            "transform_bits",
            "cache_bits",
            "palette_size",
            "lossless_size",
            "lossless_hdr_size",
            "lossless_data_size",
            "pad",
        ],
    },
    TypeLayout {
        c_name: "WebPMemoryWriter",
        rust_name: "webp_abi::WebPMemoryWriter",
        fields: &["mem", "size", "max_size", "pad"],
    },
    TypeLayout {
        c_name: "WebPPicture",
        rust_name: "webp_abi::WebPPicture",
        fields: &[
            "use_argb",
            "colorspace",
            "width",
            "height",
            "y",
            "u",
            "v",
            "y_stride",
            "uv_stride",
            "a",
            "a_stride",
            "pad1",
            "argb",
            "argb_stride",
            "pad2",
            "writer",
            "custom_ptr",
            "extra_info_type",
            "extra_info",
            "stats",
            "error_code",
            "progress_hook",
            "user_data",
            "pad3",
            "pad4",
            "pad5",
            "pad6",
            "memory_",
            "memory_argb_",
            "pad7",
        ],
    },
    TypeLayout {
        c_name: "WebPIterator",
        rust_name: "webp_abi::WebPIterator",
        fields: &[
            "frame_num",
            "num_frames",
            "x_offset",
            "y_offset",
            "width",
            "height",
            "duration",
            "dispose_method",
            "complete",
            "fragment",
            "has_alpha",
            "blend_method",
            "pad",
            "private_",
        ],
    },
    TypeLayout {
        c_name: "WebPChunkIterator",
        rust_name: "webp_abi::WebPChunkIterator",
        fields: &["chunk_num", "num_chunks", "chunk", "pad", "private_"],
    },
    TypeLayout {
        c_name: "WebPAnimDecoderOptions",
        rust_name: "webp_abi::WebPAnimDecoderOptions",
        fields: &["color_mode", "use_threads", "padding"],
    },
    TypeLayout {
        c_name: "WebPAnimInfo",
        rust_name: "webp_abi::WebPAnimInfo",
        fields: &[
            "canvas_width",
            "canvas_height",
            "loop_count",
            "bgcolor",
            "frame_count",
            "pad",
        ],
    },
    TypeLayout {
        c_name: "WebPMuxFrameInfo",
        rust_name: "webp_abi::WebPMuxFrameInfo",
        fields: &[
            "bitstream",
            "x_offset",
            "y_offset",
            "duration",
            "id",
            "dispose_method",
            "blend_method",
            "pad",
        ],
    },
    TypeLayout {
        c_name: "WebPMuxAnimParams",
        rust_name: "webp_abi::WebPMuxAnimParams",
        fields: &["bgcolor", "loop_count"],
    },
    TypeLayout {
        c_name: "WebPAnimEncoderOptions",
        rust_name: "webp_abi::WebPAnimEncoderOptions",
        fields: &[
            "anim_params",
            "minimize_size",
            "kmin",
            "kmax",
            "allow_mixed",
            "verbose",
            "padding",
        ],
    },
    TypeLayout {
        c_name: "SharpYuvConversionMatrix",
        rust_name: "webp_abi::SharpYuvConversionMatrix",
        fields: &["rgb_to_y", "rgb_to_u", "rgb_to_v"],
    },
    TypeLayout {
        c_name: "SharpYuvColorSpace",
        rust_name: "webp_abi::SharpYuvColorSpace",
        fields: &["kr", "kb", "bit_depth", "range"],
    },
];

const ENUMS: &[EnumLayout] = &[
    EnumLayout {
        c_name: "WebPFeatureFlags",
        rust_name: "webp_abi::WebPFeatureFlags",
        values: &[
            ("ANIMATION_FLAG", "webp_abi::ANIMATION_FLAG.0"),
            ("XMP_FLAG", "webp_abi::XMP_FLAG.0"),
            ("EXIF_FLAG", "webp_abi::EXIF_FLAG.0"),
            ("ALPHA_FLAG", "webp_abi::ALPHA_FLAG.0"),
            ("ICCP_FLAG", "webp_abi::ICCP_FLAG.0"),
            ("ALL_VALID_FLAGS", "webp_abi::ALL_VALID_FLAGS.0"),
        ],
    },
    EnumLayout {
        c_name: "WebPMuxAnimDispose",
        rust_name: "webp_abi::WebPMuxAnimDispose",
        values: &[
            ("WEBP_MUX_DISPOSE_NONE", "webp_abi::WEBP_MUX_DISPOSE_NONE.0"),
            (
                "WEBP_MUX_DISPOSE_BACKGROUND",
                "webp_abi::WEBP_MUX_DISPOSE_BACKGROUND.0",
            ),
        ],
    },
    EnumLayout {
        c_name: "WebPMuxAnimBlend",
        rust_name: "webp_abi::WebPMuxAnimBlend",
        values: &[
            ("WEBP_MUX_BLEND", "webp_abi::WEBP_MUX_BLEND.0"),
            ("WEBP_MUX_NO_BLEND", "webp_abi::WEBP_MUX_NO_BLEND.0"),
        ],
    },
    EnumLayout {
        c_name: "WEBP_CSP_MODE",
        rust_name: "webp_abi::WEBP_CSP_MODE",
        values: &[
            ("MODE_RGB", "webp_abi::MODE_RGB.0"),
            ("MODE_RGBA", "webp_abi::MODE_RGBA.0"),
            ("MODE_BGR", "webp_abi::MODE_BGR.0"),
            ("MODE_BGRA", "webp_abi::MODE_BGRA.0"),
            ("MODE_ARGB", "webp_abi::MODE_ARGB.0"),
            ("MODE_RGBA_4444", "webp_abi::MODE_RGBA_4444.0"),
            ("MODE_RGB_565", "webp_abi::MODE_RGB_565.0"),
            ("MODE_rgbA", "webp_abi::MODE_rgbA.0"),
            ("MODE_bgrA", "webp_abi::MODE_bgrA.0"),
            ("MODE_Argb", "webp_abi::MODE_Argb.0"),
            ("MODE_rgbA_4444", "webp_abi::MODE_rgbA_4444.0"),
            ("MODE_YUV", "webp_abi::MODE_YUV.0"),
            ("MODE_YUVA", "webp_abi::MODE_YUVA.0"),
            ("MODE_LAST", "webp_abi::MODE_LAST.0"),
        ],
    },
    EnumLayout {
        c_name: "VP8StatusCode",
        rust_name: "webp_abi::VP8StatusCode",
        values: &[
            ("VP8_STATUS_OK", "webp_abi::VP8_STATUS_OK.0"),
            (
                "VP8_STATUS_OUT_OF_MEMORY",
                "webp_abi::VP8_STATUS_OUT_OF_MEMORY.0",
            ),
            (
                "VP8_STATUS_INVALID_PARAM",
                "webp_abi::VP8_STATUS_INVALID_PARAM.0",
            ),
            (
                "VP8_STATUS_BITSTREAM_ERROR",
                "webp_abi::VP8_STATUS_BITSTREAM_ERROR.0",
            ),
            (
                "VP8_STATUS_UNSUPPORTED_FEATURE",
                "webp_abi::VP8_STATUS_UNSUPPORTED_FEATURE.0",
            ),
            ("VP8_STATUS_SUSPENDED", "webp_abi::VP8_STATUS_SUSPENDED.0"),
            ("VP8_STATUS_USER_ABORT", "webp_abi::VP8_STATUS_USER_ABORT.0"),
            (
                "VP8_STATUS_NOT_ENOUGH_DATA",
                "webp_abi::VP8_STATUS_NOT_ENOUGH_DATA.0",
            ),
        ],
    },
    EnumLayout {
        c_name: "WebPImageHint",
        rust_name: "webp_abi::WebPImageHint",
        values: &[
            ("WEBP_HINT_DEFAULT", "webp_abi::WEBP_HINT_DEFAULT.0"),
            ("WEBP_HINT_PICTURE", "webp_abi::WEBP_HINT_PICTURE.0"),
            ("WEBP_HINT_PHOTO", "webp_abi::WEBP_HINT_PHOTO.0"),
            ("WEBP_HINT_GRAPH", "webp_abi::WEBP_HINT_GRAPH.0"),
            ("WEBP_HINT_LAST", "webp_abi::WEBP_HINT_LAST.0"),
        ],
    },
    EnumLayout {
        c_name: "WebPPreset",
        rust_name: "webp_abi::WebPPreset",
        values: &[
            ("WEBP_PRESET_DEFAULT", "webp_abi::WEBP_PRESET_DEFAULT.0"),
            ("WEBP_PRESET_PICTURE", "webp_abi::WEBP_PRESET_PICTURE.0"),
            ("WEBP_PRESET_PHOTO", "webp_abi::WEBP_PRESET_PHOTO.0"),
            ("WEBP_PRESET_DRAWING", "webp_abi::WEBP_PRESET_DRAWING.0"),
            ("WEBP_PRESET_ICON", "webp_abi::WEBP_PRESET_ICON.0"),
            ("WEBP_PRESET_TEXT", "webp_abi::WEBP_PRESET_TEXT.0"),
        ],
    },
    EnumLayout {
        c_name: "WebPEncCSP",
        rust_name: "webp_abi::WebPEncCSP",
        values: &[
            ("WEBP_YUV420", "webp_abi::WEBP_YUV420.0"),
            ("WEBP_YUV420A", "webp_abi::WEBP_YUV420A.0"),
            ("WEBP_CSP_UV_MASK", "webp_abi::WEBP_CSP_UV_MASK.0"),
            ("WEBP_CSP_ALPHA_BIT", "webp_abi::WEBP_CSP_ALPHA_BIT.0"),
        ],
    },
    EnumLayout {
        c_name: "WebPEncodingError",
        rust_name: "webp_abi::WebPEncodingError",
        values: &[
            ("VP8_ENC_OK", "webp_abi::VP8_ENC_OK.0"),
            (
                "VP8_ENC_ERROR_OUT_OF_MEMORY",
                "webp_abi::VP8_ENC_ERROR_OUT_OF_MEMORY.0",
            ),
            (
                "VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY",
                "webp_abi::VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY.0",
            ),
            (
                "VP8_ENC_ERROR_NULL_PARAMETER",
                "webp_abi::VP8_ENC_ERROR_NULL_PARAMETER.0",
            ),
            (
                "VP8_ENC_ERROR_INVALID_CONFIGURATION",
                "webp_abi::VP8_ENC_ERROR_INVALID_CONFIGURATION.0",
            ),
            (
                "VP8_ENC_ERROR_BAD_DIMENSION",
                "webp_abi::VP8_ENC_ERROR_BAD_DIMENSION.0",
            ),
            (
                "VP8_ENC_ERROR_PARTITION0_OVERFLOW",
                "webp_abi::VP8_ENC_ERROR_PARTITION0_OVERFLOW.0",
            ),
            (
                "VP8_ENC_ERROR_PARTITION_OVERFLOW",
                "webp_abi::VP8_ENC_ERROR_PARTITION_OVERFLOW.0",
            ),
            (
                "VP8_ENC_ERROR_BAD_WRITE",
                "webp_abi::VP8_ENC_ERROR_BAD_WRITE.0",
            ),
            (
                "VP8_ENC_ERROR_FILE_TOO_BIG",
                "webp_abi::VP8_ENC_ERROR_FILE_TOO_BIG.0",
            ),
            (
                "VP8_ENC_ERROR_USER_ABORT",
                "webp_abi::VP8_ENC_ERROR_USER_ABORT.0",
            ),
            ("VP8_ENC_ERROR_LAST", "webp_abi::VP8_ENC_ERROR_LAST.0"),
        ],
    },
    EnumLayout {
        c_name: "WebPDemuxState",
        rust_name: "webp_abi::WebPDemuxState",
        values: &[
            (
                "WEBP_DEMUX_PARSE_ERROR",
                "webp_abi::WEBP_DEMUX_PARSE_ERROR.0",
            ),
            (
                "WEBP_DEMUX_PARSING_HEADER",
                "webp_abi::WEBP_DEMUX_PARSING_HEADER.0",
            ),
            (
                "WEBP_DEMUX_PARSED_HEADER",
                "webp_abi::WEBP_DEMUX_PARSED_HEADER.0",
            ),
            ("WEBP_DEMUX_DONE", "webp_abi::WEBP_DEMUX_DONE.0"),
        ],
    },
    EnumLayout {
        c_name: "WebPFormatFeature",
        rust_name: "webp_abi::WebPFormatFeature",
        values: &[
            ("WEBP_FF_FORMAT_FLAGS", "webp_abi::WEBP_FF_FORMAT_FLAGS.0"),
            ("WEBP_FF_CANVAS_WIDTH", "webp_abi::WEBP_FF_CANVAS_WIDTH.0"),
            ("WEBP_FF_CANVAS_HEIGHT", "webp_abi::WEBP_FF_CANVAS_HEIGHT.0"),
            ("WEBP_FF_LOOP_COUNT", "webp_abi::WEBP_FF_LOOP_COUNT.0"),
            (
                "WEBP_FF_BACKGROUND_COLOR",
                "webp_abi::WEBP_FF_BACKGROUND_COLOR.0",
            ),
            ("WEBP_FF_FRAME_COUNT", "webp_abi::WEBP_FF_FRAME_COUNT.0"),
        ],
    },
    EnumLayout {
        c_name: "WebPMuxError",
        rust_name: "webp_abi::WebPMuxError",
        values: &[
            ("WEBP_MUX_OK", "webp_abi::WEBP_MUX_OK.0"),
            ("WEBP_MUX_NOT_FOUND", "webp_abi::WEBP_MUX_NOT_FOUND.0"),
            (
                "WEBP_MUX_INVALID_ARGUMENT",
                "webp_abi::WEBP_MUX_INVALID_ARGUMENT.0",
            ),
            ("WEBP_MUX_BAD_DATA", "webp_abi::WEBP_MUX_BAD_DATA.0"),
            ("WEBP_MUX_MEMORY_ERROR", "webp_abi::WEBP_MUX_MEMORY_ERROR.0"),
            (
                "WEBP_MUX_NOT_ENOUGH_DATA",
                "webp_abi::WEBP_MUX_NOT_ENOUGH_DATA.0",
            ),
        ],
    },
    EnumLayout {
        c_name: "WebPChunkId",
        rust_name: "webp_abi::WebPChunkId",
        values: &[
            ("WEBP_CHUNK_VP8X", "webp_abi::WEBP_CHUNK_VP8X.0"),
            ("WEBP_CHUNK_ICCP", "webp_abi::WEBP_CHUNK_ICCP.0"),
            ("WEBP_CHUNK_ANIM", "webp_abi::WEBP_CHUNK_ANIM.0"),
            ("WEBP_CHUNK_ANMF", "webp_abi::WEBP_CHUNK_ANMF.0"),
            ("WEBP_CHUNK_DEPRECATED", "webp_abi::WEBP_CHUNK_DEPRECATED.0"),
            ("WEBP_CHUNK_ALPHA", "webp_abi::WEBP_CHUNK_ALPHA.0"),
            ("WEBP_CHUNK_IMAGE", "webp_abi::WEBP_CHUNK_IMAGE.0"),
            ("WEBP_CHUNK_EXIF", "webp_abi::WEBP_CHUNK_EXIF.0"),
            ("WEBP_CHUNK_XMP", "webp_abi::WEBP_CHUNK_XMP.0"),
            ("WEBP_CHUNK_UNKNOWN", "webp_abi::WEBP_CHUNK_UNKNOWN.0"),
            ("WEBP_CHUNK_NIL", "webp_abi::WEBP_CHUNK_NIL.0"),
        ],
    },
    EnumLayout {
        c_name: "SharpYuvRange",
        rust_name: "webp_abi::SharpYuvRange",
        values: &[
            ("kSharpYuvRangeFull", "webp_abi::kSharpYuvRangeFull.0"),
            ("kSharpYuvRangeLimited", "webp_abi::kSharpYuvRangeLimited.0"),
        ],
    },
    EnumLayout {
        c_name: "SharpYuvMatrixType",
        rust_name: "webp_abi::SharpYuvMatrixType",
        values: &[
            ("kSharpYuvMatrixWebp", "webp_abi::kSharpYuvMatrixWebp.0"),
            (
                "kSharpYuvMatrixRec601Limited",
                "webp_abi::kSharpYuvMatrixRec601Limited.0",
            ),
            (
                "kSharpYuvMatrixRec601Full",
                "webp_abi::kSharpYuvMatrixRec601Full.0",
            ),
            (
                "kSharpYuvMatrixRec709Limited",
                "webp_abi::kSharpYuvMatrixRec709Limited.0",
            ),
            (
                "kSharpYuvMatrixRec709Full",
                "webp_abi::kSharpYuvMatrixRec709Full.0",
            ),
            ("kSharpYuvMatrixNum", "webp_abi::kSharpYuvMatrixNum.0"),
        ],
    },
];

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let include_dir = manifest_dir.join("../../include");
    for header in HEADERS {
        println!(
            "cargo:rerun-if-changed={}",
            include_dir.join(header).display()
        );
    }
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/types.rs");
    println!("cargo:rerun-if-changed=src/decode.rs");
    println!("cargo:rerun-if-changed=src/encode.rs");
    println!("cargo:rerun-if-changed=src/demux.rs");
    println!("cargo:rerun-if-changed=src/mux.rs");
    println!("cargo:rerun-if-changed=src/sharpyuv.rs");

    let c_source = out_dir.join("ffi_layout_oracle.c");
    let oracle_bin = out_dir.join("ffi_layout_oracle");
    let rust_tests = out_dir.join("ffi_layout.rs");

    fs::write(&c_source, generate_c_source()).unwrap();
    compile_oracle(&c_source, &oracle_bin, &include_dir);
    let output = Command::new(&oracle_bin).output().unwrap();
    assert!(output.status.success(), "layout oracle failed");
    let oracle = String::from_utf8(output.stdout).unwrap();
    fs::write(&rust_tests, generate_rust_tests(&oracle)).unwrap();
}

fn compile_oracle(source: &Path, binary: &Path, include_dir: &Path) {
    let compiler = env::var("CC").unwrap_or_else(|_| "cc".to_owned());
    let status = Command::new(compiler)
        .arg("-std=c11")
        .arg(format!("-I{}", include_dir.display()))
        .arg(format!("-I{}", include_dir.join("webp").display()))
        .arg(source)
        .arg("-o")
        .arg(binary)
        .status()
        .unwrap();
    assert!(status.success(), "failed to compile ffi layout oracle");
}

fn generate_c_source() -> String {
    let mut source = String::from(
        "#include <stddef.h>\n#include <stdio.h>\n#include \"webp/types.h\"\n\
         #include \"webp/mux_types.h\"\n#include \"webp/decode.h\"\n\
         #include \"webp/encode.h\"\n#include \"webp/demux.h\"\n\
         #include \"webp/mux.h\"\n#include \"webp/sharpyuv/sharpyuv.h\"\n\
         #include \"webp/sharpyuv/sharpyuv_csp.h\"\n\
         #define PRINT_TYPE(type) printf(\"T|%s|%zu|%zu\\n\", #type, sizeof(type), _Alignof(type))\n\
         #define PRINT_FIELD(type, field) printf(\"F|%s|%s|%zu\\n\", #type, #field, offsetof(type, field))\n\
         #define PRINT_CONST(name) printf(\"C|%s|%lld\\n\", #name, (long long)(name))\n\
         int main(void) {\n",
    );
    for ty in TYPES {
        source.push_str(&format!("  PRINT_TYPE({});\n", ty.c_name));
        for field in ty.fields {
            source.push_str(&format!("  PRINT_FIELD({}, {});\n", ty.c_name, field));
        }
    }
    for enm in ENUMS {
        source.push_str(&format!("  PRINT_TYPE({});\n", enm.c_name));
        for (c_name, _) in enm.values {
            source.push_str(&format!("  PRINT_CONST({});\n", c_name));
        }
    }
    source.push_str("  return 0;\n}\n");
    source
}

fn generate_rust_tests(oracle: &str) -> String {
    let mut type_sizes = BTreeMap::new();
    let mut type_aligns = BTreeMap::new();
    let mut field_offsets = BTreeMap::new();
    let mut const_values = BTreeMap::new();

    for line in oracle.lines() {
        let mut parts = line.split('|');
        match parts.next() {
            Some("T") => {
                let name = parts.next().unwrap().to_owned();
                type_sizes.insert(name.clone(), parts.next().unwrap().to_owned());
                type_aligns.insert(name, parts.next().unwrap().to_owned());
            }
            Some("F") => {
                let ty = parts.next().unwrap().to_owned();
                let field = parts.next().unwrap().to_owned();
                field_offsets.insert((ty, field), parts.next().unwrap().to_owned());
            }
            Some("C") => {
                const_values.insert(
                    parts.next().unwrap().to_owned(),
                    parts.next().unwrap().to_owned(),
                );
            }
            _ => {}
        }
    }

    let mut tests = String::from("use core::mem::{align_of, offset_of, size_of};\n\n");
    for ty in TYPES {
        let size = type_sizes.get(ty.c_name).unwrap();
        let align = type_aligns.get(ty.c_name).unwrap();
        tests.push_str(&format!(
            "#[test]\nfn layout_{}() {{\n    assert_eq!(size_of::<{}>(), {}usize);\n    assert_eq!(align_of::<{}>(), {}usize);\n",
            sanitize(ty.c_name),
            ty.rust_name,
            size,
            ty.rust_name,
            align
        ));
        for field in ty.fields {
            let offset = field_offsets
                .get(&(ty.c_name.to_owned(), (*field).to_owned()))
                .unwrap();
            tests.push_str(&format!(
                "    assert_eq!(offset_of!({}, {}), {}usize);\n",
                ty.rust_name, field, offset
            ));
        }
        tests.push_str("}\n\n");
    }

    for enm in ENUMS {
        let size = type_sizes.get(enm.c_name).unwrap();
        let align = type_aligns.get(enm.c_name).unwrap();
        tests.push_str(&format!(
            "#[test]\nfn enum_{}() {{\n    assert_eq!(size_of::<{}>(), {}usize);\n    assert_eq!(align_of::<{}>(), {}usize);\n",
            sanitize(enm.c_name),
            enm.rust_name,
            size,
            enm.rust_name,
            align
        ));
        for (c_name, rust_expr) in enm.values {
            let value = const_values.get(*c_name).unwrap();
            tests.push_str(&format!("    assert_eq!({rust_expr}, {value});\n"));
        }
        tests.push_str("}\n\n");
    }
    tests
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect()
}
