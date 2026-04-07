use serde_json::from_str;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    compile_encoder_support();
    ensure_sharpyuv_dependency();
    emit_linker_baseline("libwebp", &["-lc", "-lm"]);
}

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
    for link_arg in extra_link_args {
        println!("cargo:rustc-cdylib-link-arg={link_arg}");
    }
}
