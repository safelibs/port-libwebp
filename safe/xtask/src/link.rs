use crate::tools::{capture_output, sort_dedup};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

pub const LIBRARIES: &[&str] = &[
    "libsharpyuv",
    "libwebp",
    "libwebpdecoder",
    "libwebpdemux",
    "libwebpmux",
];

pub const RELINK_TARGETS: &[(&str, &str, &str)] = &[
    (
        "webp_public_api_test",
        "tests/public_api_test.c",
        "tests/CMakeFiles/webp_public_api_test.dir/link.txt",
    ),
    ("cwebp", "examples/cwebp.c", "CMakeFiles/cwebp.dir/link.txt"),
    ("dwebp", "examples/dwebp.c", "CMakeFiles/dwebp.dir/link.txt"),
    (
        "img2webp",
        "examples/img2webp.c",
        "CMakeFiles/img2webp.dir/link.txt",
    ),
    (
        "webpinfo",
        "examples/webpinfo.c",
        "CMakeFiles/webpinfo.dir/link.txt",
    ),
    (
        "webpmux",
        "examples/webpmux.c",
        "CMakeFiles/webpmux.dir/link.txt",
    ),
];

#[derive(Debug)]
pub struct SharedLibraryInfo {
    pub exports: Vec<String>,
    pub needed: Vec<String>,
    pub soname: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RelinkEntry {
    pub link_line: String,
    pub link_txt: String,
    pub source: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RelinkManifest {
    pub targets: BTreeMap<String, RelinkEntry>,
}

pub fn capture_relink_manifest(build_dir: &Path) -> Result<RelinkManifest> {
    let mut targets = BTreeMap::new();
    for (target, source, link_txt) in RELINK_TARGETS {
        let link_path = build_dir.join(link_txt);
        let link_line = fs::read_to_string(&link_path)
            .with_context(|| format!("failed to read {}", link_path.display()))?;
        targets.insert(
            (*target).to_owned(),
            RelinkEntry {
                link_line: normalize_link_line(&link_line, build_dir),
                link_txt: (*link_txt).to_owned(),
                source: (*source).to_owned(),
            },
        );
    }
    Ok(RelinkManifest { targets })
}

pub fn find_library_artifact(build_dir: &Path, logical_name: &str) -> Result<PathBuf> {
    let prefix = format!("{logical_name}.so");
    let mut matches = Vec::new();
    for entry in WalkDir::new(build_dir).follow_links(false) {
        let entry = entry?;
        if !entry.file_type().is_file() && !entry.file_type().is_symlink() {
            continue;
        }
        let file_name = entry.file_name().to_string_lossy();
        if file_name == prefix || file_name.starts_with(&format!("{prefix}.")) {
            let metadata = fs::symlink_metadata(entry.path())?;
            let score = (
                !metadata.file_type().is_symlink(),
                file_name.len(),
                entry.depth(),
            );
            matches.push((score, entry.into_path()));
        }
    }
    matches.sort_by(|left, right| right.0.cmp(&left.0));
    matches
        .into_iter()
        .map(|(_, path)| path)
        .next()
        .with_context(|| format!("failed to find built artifact for {logical_name}"))
}

pub fn infer_library_name(path: &Path) -> Option<&'static str> {
    let file_name = path.file_name()?.to_string_lossy();
    LIBRARIES.iter().copied().find(|name| {
        file_name == format!("{name}.so") || file_name.starts_with(&format!("{name}.so."))
    })
}

pub fn inspect_shared_library(path: &Path) -> Result<SharedLibraryInfo> {
    let dynamic = capture_output(Command::new("readelf").arg("-d").arg(path))?;
    let mut needed = Vec::new();
    let mut soname = None;
    for line in dynamic.lines() {
        if line.contains("(NEEDED)") {
            if let Some(value) = bracket_value(line) {
                needed.push(value.to_owned());
            }
        }
        if line.contains("(SONAME)") {
            soname = bracket_value(line).map(ToOwned::to_owned);
        }
    }
    sort_dedup(&mut needed);
    let soname = soname.with_context(|| format!("missing SONAME in {}", path.display()))?;

    let exports = capture_exports(path)?;
    Ok(SharedLibraryInfo {
        exports,
        needed,
        soname,
    })
}

fn capture_exports(path: &Path) -> Result<Vec<String>> {
    let stdout = capture_output(
        Command::new("nm")
            .arg("-D")
            .arg("--defined-only")
            .arg("--extern-only")
            .arg("-j")
            .arg(path),
    )?;
    let mut exports = stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    sort_dedup(&mut exports);
    if exports.is_empty() {
        bail!("no dynamic exports found in {}", path.display());
    }
    Ok(exports)
}

fn bracket_value(line: &str) -> Option<&str> {
    let start = line.find('[')? + 1;
    let end = line.rfind(']')?;
    (start < end).then(|| &line[start..end])
}

fn normalize_link_line(raw: &str, build_dir: &Path) -> String {
    let normalized = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    normalized.replace(build_dir.to_string_lossy().as_ref(), "$BUILD_DIR")
}
