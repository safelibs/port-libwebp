use serde_json::from_str;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    emit_linker_baseline("libsharpyuv", &["-lc", "-lm"]);
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

    let mut version_script = String::from("{\n  global:\n");
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
