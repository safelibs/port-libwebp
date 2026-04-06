use crate::link::{
    capture_relink_manifest, find_library_artifact, inspect_shared_library, LIBRARIES,
};
use crate::package::capture_install_surface;
use crate::tools::{run, write_json, write_text};
use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

pub fn capture_baseline(original_dir: &Path, out_dir: &Path) -> Result<()> {
    fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create {}", out_dir.display()))?;

    let install_surface = capture_install_surface(original_dir)?;
    let temp_dir = TempDir::new().context("failed to create temporary build directory")?;
    let build_dir = temp_dir.path().join("build");

    configure_original_build(original_dir, &build_dir)?;
    build_original_targets(&build_dir)?;

    let mut sonames = BTreeMap::new();
    let mut needed = BTreeMap::new();
    for logical_name in LIBRARIES {
        let artifact = find_library_artifact(&build_dir, logical_name)?;
        let info = inspect_shared_library(&artifact)?;
        sonames.insert((*logical_name).to_owned(), info.soname);
        needed.insert((*logical_name).to_owned(), info.needed);
        write_text(
            &out_dir.join(format!("{logical_name}.exports")),
            &format!("{}\n", info.exports.join("\n")),
        )?;
    }

    write_json(&out_dir.join("sonames.json"), &sonames)?;
    write_json(&out_dir.join("needed.json"), &needed)?;
    write_json(&out_dir.join("install-surface.json"), &install_surface)?;
    write_json(
        &out_dir.join("relink-manifest.json"),
        &capture_relink_manifest(&build_dir)?,
    )?;
    Ok(())
}

fn configure_original_build(original_dir: &Path, build_dir: &Path) -> Result<()> {
    let mut command = Command::new("cmake");
    command
        .arg("-G")
        .arg("Unix Makefiles")
        .arg("-S")
        .arg(original_dir)
        .arg("-B")
        .arg(build_dir)
        .arg("-DBUILD_SHARED_LIBS=ON")
        .arg("-DBUILD_TESTING=ON")
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg("-DWEBP_BUILD_ANIM_UTILS=OFF")
        .arg("-DWEBP_BUILD_CWEBP=ON")
        .arg("-DWEBP_BUILD_DWEBP=ON")
        .arg("-DWEBP_BUILD_EXTRAS=OFF")
        .arg("-DWEBP_BUILD_GIF2WEBP=OFF")
        .arg("-DWEBP_BUILD_IMG2WEBP=ON")
        .arg("-DWEBP_BUILD_LIBWEBPMUX=ON")
        .arg("-DWEBP_BUILD_VWEBP=OFF")
        .arg("-DWEBP_BUILD_WEBPINFO=ON")
        .arg("-DWEBP_BUILD_WEBPMUX=ON")
        .arg("-DWEBP_LINK_STATIC=OFF");
    run(&mut command)
}

fn build_original_targets(build_dir: &Path) -> Result<()> {
    let mut command = Command::new("cmake");
    command
        .arg("--build")
        .arg(build_dir)
        .arg("--parallel")
        .arg(parallelism().to_string())
        .arg("--target")
        .arg("sharpyuv")
        .arg("webpdecoder")
        .arg("webp")
        .arg("webpdemux")
        .arg("libwebpmux")
        .arg("cwebp")
        .arg("dwebp")
        .arg("img2webp")
        .arg("webpinfo")
        .arg("webpmux")
        .arg("webp_public_api_test");
    run(&mut command)
}

fn parallelism() -> usize {
    std::thread::available_parallelism()
        .map(usize::from)
        .unwrap_or(1)
}
