use serde_json::from_str;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    if env::var("PROFILE").as_deref() != Ok("release") {
        link_webp_core_roots();
    }
    compile_encoder_support();
    ensure_sharpyuv_dependency();
    emit_linker_baseline("libwebp", &["-lc", "-lm"]);
}

const INTERNAL_LINK_ROOTS: &[&str] = &[
    "VP8BitReaderSetBuffer",
    "VP8DitherCombine8x8",
    "VP8DspInit",
    "VP8FiltersInit",
    "VP8GetSignedValue",
    "VP8GetValue",
    "VP8HFilter16",
    "VP8HFilter16i",
    "VP8HFilter8",
    "VP8HFilter8i",
    "VP8InitBitReader",
    "VP8LBitReaderSetBuffer",
    "VP8LBuildHuffmanTable",
    "VP8LColorCacheClear",
    "VP8LColorCacheCopy",
    "VP8LColorCacheInit",
    "VP8LColorIndexInverseTransformAlpha",
    "VP8LConvertBGRAToRGBA",
    "VP8LConvertFromBGRA",
    "VP8LDoFillBitWindow",
    "VP8LDspInit",
    "VP8LHtreeGroupsFree",
    "VP8LHtreeGroupsNew",
    "VP8LHuffmanTablesAllocate",
    "VP8LHuffmanTablesDeallocate",
    "VP8LInitBitReader",
    "VP8LInverseTransform",
    "VP8LPredictor10_C",
    "VP8LPredictor11_C",
    "VP8LPredictor12_C",
    "VP8LPredictor13_C",
    "VP8LPredictor2_C",
    "VP8LPredictor3_C",
    "VP8LPredictor4_C",
    "VP8LPredictor5_C",
    "VP8LPredictor6_C",
    "VP8LPredictor7_C",
    "VP8LPredictor8_C",
    "VP8LPredictor9_C",
    "VP8LPredictors",
    "VP8LReadBits",
    "VP8LoadFinalBytes",
    "VP8PredChroma8",
    "VP8PredLuma16",
    "VP8PredLuma4",
    "VP8RemapBitReader",
    "VP8SimpleHFilter16",
    "VP8SimpleHFilter16i",
    "VP8SimpleVFilter16",
    "VP8SimpleVFilter16i",
    "VP8Transform",
    "VP8TransformAC3",
    "VP8TransformDC",
    "VP8TransformDCUV",
    "VP8TransformUV",
    "VP8TransformWHT",
    "VP8VFilter16",
    "VP8VFilter16i",
    "VP8VFilter8",
    "VP8VFilter8i",
    "WebPAlphaReplace",
    "WebPApplyAlphaMultiply",
    "WebPApplyAlphaMultiply4444",
    "WebPConvertARGBToUV",
    "WebPConvertARGBToY",
    "WebPConvertBGR24ToY",
    "WebPConvertRGB24ToY",
    "WebPConvertRGBA32ToUV",
    "WebPDequantizeLevels",
    "WebPDispatchAlpha",
    "WebPDispatchAlphaToGreen",
    "WebPEstimateBestFilter",
    "WebPExtractAlpha",
    "WebPExtractGreen",
    "WebPFilters",
    "WebPGetLinePairConverter",
    "WebPHasAlpha32b",
    "WebPHasAlpha8b",
    "WebPInitAlphaProcessing",
    "WebPInitConvertARGBToYUV",
    "WebPInitSamplers",
    "WebPInitUpsamplers",
    "WebPInitYUV444Converters",
    "WebPMultARGBRow",
    "WebPMultARGBRows",
    "WebPMultRows",
    "WebPPackRGB",
    "WebPRescaleNeededLines",
    "WebPRescalerDspInit",
    "WebPRescalerExport",
    "WebPRescalerExportRow",
    "WebPRescalerGetScaledDimensions",
    "WebPRescalerImport",
    "WebPRescalerImportRow",
    "WebPRescalerInit",
    "WebPSamplerProcessPlane",
    "WebPSamplers",
    "WebPUnfilters",
    "WebPUpsamplers",
    "WebPYUV444Converters",
    "kVP8Log2Range",
    "kVP8NewRange",
];

fn compile_encoder_support() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let original_dir = manifest_dir.join("../../../original");
    let support_sources = [
        "src/enc/picture_psnr_enc.c",
        "src/dsp/cost.c",
        "src/dsp/enc.c",
        "src/dsp/lossless_enc.c",
        "src/dsp/ssim.c",
        "src/utils/bit_writer_utils.c",
        "src/utils/huffman_encode_utils.c",
        "src/utils/quant_levels_utils.c",
    ];

    let mut build = cc::Build::new();
    build.include(&original_dir);
    build.warnings(false);

    for source in support_sources {
        let path = original_dir.join(source);
        println!("cargo:rerun-if-changed={}", path.display());
        build.file(path);
    }

    build.compile("webp_encoder_support");
}

fn link_webp_core_roots() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let deps_dir = profile_dir(&out_dir).join("deps");
    let rlib_path = find_webp_core_rlib(&deps_dir);
    let object_members = provider_members(&rlib_path);
    let extract_dir = out_dir.join("webp_core_roots");
    let archive_path = out_dir.join("libwebp_core_roots.a");
    let ar = env::var_os("AR").unwrap_or_else(|| "ar".into());

    assert!(
        !object_members.is_empty(),
        "failed to find provider members in {}",
        rlib_path.display()
    );

    if extract_dir.exists() {
        fs::remove_dir_all(&extract_dir).expect("failed to clear extracted webp-core roots");
    }
    fs::create_dir_all(&extract_dir).expect("failed to create extracted webp-core roots");

    let status = Command::new(&ar)
        .current_dir(&extract_dir)
        .arg("x")
        .arg(&rlib_path)
        .args(&object_members)
        .status()
        .expect("failed to extract webp-core root objects");
    assert!(
        status.success(),
        "ar failed while extracting webp-core root objects"
    );

    if archive_path.exists() {
        fs::remove_file(&archive_path).expect("failed to remove stale webp-core roots archive");
    }

    let object_paths: Vec<_> = object_members
        .iter()
        .map(|member| extract_dir.join(member))
        .collect();
    let status = Command::new(&ar)
        .arg("crs")
        .arg(&archive_path)
        .args(&object_paths)
        .status()
        .expect("failed to archive webp-core root objects");
    assert!(
        status.success(),
        "ar failed while archiving webp-core root objects"
    );

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=webp_core_roots");
}

fn find_webp_core_rlib(deps_dir: &Path) -> PathBuf {
    let profile_dir = deps_dir
        .parent()
        .expect("deps directory was missing its profile parent");
    let fingerprint_dir = profile_dir.join(".fingerprint");

    for entry in
        fs::read_dir(&fingerprint_dir).expect("failed to read target fingerprint directory")
    {
        let entry = entry.expect("failed to read target fingerprint entry");
        let path = entry.path();
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();
        if !file_name.starts_with("webp-core-") {
            continue;
        }
        let metadata_path = path.join("lib-webp_core.json");
        if !metadata_path.exists() {
            continue;
        }
        let metadata: serde_json::Value = from_str(
            &fs::read_to_string(&metadata_path).expect("failed to read webp-core fingerprint"),
        )
        .expect("failed to parse webp-core fingerprint");
        if metadata.get("features").and_then(|value| value.as_str())
            != Some("[\"decode\", \"encode\"]")
        {
            continue;
        }
        let hash = file_name.trim_start_matches("webp-core-");
        let rlib_path = deps_dir.join(format!("libwebp_core-{hash}.rlib"));
        if rlib_path.exists() {
            return rlib_path;
        }
    }

    panic!(
        "failed to locate the decode+encode webp-core rlib in {}",
        deps_dir.display()
    );
}

fn provider_members(rlib_path: &Path) -> BTreeSet<String> {
    let nm = env::var_os("NM").unwrap_or_else(|| "nm".into());
    let output = Command::new(nm)
        .arg("-Ao")
        .arg(rlib_path)
        .output()
        .expect("failed to inspect webp-core rlib");
    assert!(
        output.status.success(),
        "nm failed while inspecting {}",
        rlib_path.display()
    );

    let mut members = BTreeSet::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let mut segments = line.splitn(3, ':');
        let _archive = segments.next();
        let Some(member) = segments.next() else {
            continue;
        };
        let Some(rest) = segments.next() else {
            continue;
        };
        let mut fields = rest.split_whitespace();
        let _address = fields.next();
        let Some(kind) = fields.next() else {
            continue;
        };
        let Some(symbol) = fields.next() else {
            continue;
        };
        if matches!(kind, "T" | "D" | "B" | "R") && INTERNAL_LINK_ROOTS.contains(&symbol) {
            members.insert(member.to_owned());
        }
    }
    members
}
fn ensure_sharpyuv_dependency() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let workspace_manifest = manifest_dir.join("../../Cargo.toml");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let target_dir = out_dir.join("sharpyuv-target");
    let profile = env::var("PROFILE").unwrap();
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut build = Command::new(cargo);

    build
        .arg("build")
        .arg("--manifest-path")
        .arg(&workspace_manifest)
        .arg("--target-dir")
        .arg(&target_dir)
        .arg("-p")
        .arg("libsharpyuv");
    if profile == "release" {
        build.arg("--release");
    } else if profile != "debug" {
        build.arg("--profile").arg(&profile);
    }

    let status = build.status().expect("failed to invoke nested cargo build");
    assert!(
        status.success(),
        "nested cargo build for libsharpyuv failed"
    );

    println!(
        "cargo:rustc-link-search=native={}",
        profile_output_dir(&target_dir, &profile).display()
    );
    println!("cargo:rustc-link-lib=dylib=sharpyuv");
}

fn profile_output_dir(target_dir: &Path, profile: &str) -> PathBuf {
    let target = env::var("TARGET").unwrap();
    let host_dir = target_dir.join(profile);
    if host_dir.exists() {
        host_dir
    } else {
        target_dir.join(target).join(profile)
    }
}

fn profile_dir(out_dir: &Path) -> PathBuf {
    out_dir
        .ancestors()
        .nth(3)
        .expect("OUT_DIR did not contain a cargo profile directory")
        .to_path_buf()
}

fn emit_linker_baseline(library: &str, extra_link_args: &[&str]) {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let baseline_dir = manifest_dir.join("../../abi/original");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let exports_path = baseline_dir.join(format!("{library}.exports"));
    let sonames_path = baseline_dir.join("sonames.json");
    let version_script_path = out_dir.join(format!("{library}.map"));

    println!("cargo:rerun-if-changed={}", exports_path.display());
    println!("cargo:rerun-if-changed={}", sonames_path.display());

    let exports = fs::read_to_string(&exports_path).unwrap();
    let sonames: BTreeMap<String, String> =
        from_str(&fs::read_to_string(&sonames_path).unwrap()).unwrap();
    let soname = sonames.get(library).unwrap();

    let version_node = library.to_ascii_uppercase();
    let mut version_script = format!("{version_node} {{\n  global:\n");
    for symbol in exports
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
    {
        version_script.push_str("    ");
        version_script.push_str(symbol);
        version_script.push_str(";\n");
    }
    version_script.push_str("  local:\n    *;\n};\n");
    fs::write(&version_script_path, version_script).unwrap();

    println!("cargo:rustc-cdylib-link-arg=-Wl,--no-as-needed");
    println!("cargo:rustc-cdylib-link-arg=-Wl,--undefined-version");
    println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,{soname}");
    println!(
        "cargo:rustc-cdylib-link-arg=-Wl,--version-script={}",
        version_script_path.display()
    );
    if env::var("PROFILE").as_deref() != Ok("release") {
        for symbol in INTERNAL_LINK_ROOTS {
            println!("cargo:rustc-cdylib-link-arg=-Wl,-u,{symbol}");
        }
    }
    for link_arg in extra_link_args {
        println!("cargo:rustc-cdylib-link-arg={link_arg}");
    }
}
