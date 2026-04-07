use serde_json::from_str;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    compile_encoder_support();
    emit_linker_baseline("libwebpdecoder", &["-lc"]);
}

fn compile_encoder_support() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let original_dir = manifest_dir.join("../../../original");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let support_lib = out_dir.join("libwebpdecoder_support.a");
    let support_sources = [
        "src/dsp/cost.c",
        "src/dsp/enc.c",
        "src/dsp/lossless_enc.c",
        "src/dsp/ssim.c",
        "src/utils/bit_writer_utils.c",
        "src/utils/huffman_encode_utils.c",
        "src/utils/quant_levels_utils.c",
    ];
    let cc = env::var_os("CC").unwrap_or_else(|| "cc".into());
    let ar = env::var_os("AR").unwrap_or_else(|| "ar".into());

    let mut objects = Vec::with_capacity(support_sources.len());
    for source in support_sources {
        let path = original_dir.join(source);
        let object_name = source.replace('/', "_").replace(".c", ".o");
        let object_path = out_dir.join(object_name);
        println!("cargo:rerun-if-changed={}", path.display());

        let status = Command::new(&cc)
            .arg("-c")
            .arg(&path)
            .arg("-o")
            .arg(&object_path)
            .arg("-I")
            .arg(&original_dir)
            .arg("-fPIC")
            .arg("-ffunction-sections")
            .arg("-fdata-sections")
            .status()
            .expect("failed to invoke cc for libwebpdecoder support sources");
        assert!(
            status.success(),
            "cc failed while compiling {}",
            path.display()
        );
        objects.push(object_path);
    }

    if support_lib.exists() {
        fs::remove_file(&support_lib).expect("failed to remove stale support archive");
    }

    let status = Command::new(&ar)
        .arg("crs")
        .arg(&support_lib)
        .args(&objects)
        .status()
        .expect("failed to invoke ar for libwebpdecoder support archive");
    assert!(status.success(), "ar failed while building support archive");

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=webpdecoder_support");
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
