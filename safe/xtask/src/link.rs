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

#[derive(Debug, Clone)]
pub struct DefinedSymbol {
    pub kind: char,
    pub name: String,
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
    let mut direct_matches = Vec::new();
    for entry in fs::read_dir(build_dir)
        .with_context(|| format!("failed to read {}", build_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name().to_string_lossy().into_owned();
        if file_name != prefix && !file_name.starts_with(&format!("{prefix}.")) {
            continue;
        }
        let metadata = fs::symlink_metadata(&path)?;
        let score = (!metadata.file_type().is_symlink(), file_name == prefix, file_name.len());
        direct_matches.push((score, path));
    }
    direct_matches.sort_by(|left, right| right.0.cmp(&left.0));
    if let Some((_, path)) = direct_matches.into_iter().next() {
        return Ok(path);
    }

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
                usize::MAX - entry.depth(),
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

    let defined_symbols = capture_defined_symbols(path)?;
    let mut exports = defined_symbols
        .iter()
        .map(|symbol| symbol.name.clone())
        .collect::<Vec<_>>();
    sort_dedup(&mut exports);
    Ok(SharedLibraryInfo {
        exports,
        needed,
        soname,
    })
}

pub fn capture_defined_symbols(path: &Path) -> Result<Vec<DefinedSymbol>> {
    let stdout = capture_output(Command::new("readelf").arg("-sW").arg(path))?;
    let mut symbols = stdout
        .lines()
        .scan(false, |in_dynsym, line| {
            if line.starts_with("Symbol table '.dynsym'") {
                *in_dynsym = true;
                return Some(None);
            }
            if *in_dynsym && line.starts_with("Symbol table '") {
                *in_dynsym = false;
            }
            Some((*in_dynsym).then_some(line))
        })
        .flatten()
        .filter_map(parse_readelf_dynamic_symbol_line)
        .collect::<Vec<_>>();
    symbols.sort_by(|left, right| left.name.cmp(&right.name).then(left.kind.cmp(&right.kind)));
    symbols.dedup_by(|left, right| left.name == right.name && left.kind == right.kind);
    if symbols.is_empty() {
        bail!("no dynamic exports found in {}", path.display());
    }
    Ok(symbols)
}

fn parse_readelf_dynamic_symbol_line(line: &str) -> Option<DefinedSymbol> {
    let trimmed = line.trim_start();
    let first = trimmed.chars().next()?;
    if !first.is_ascii_digit() {
        return None;
    }

    let parts = trimmed.split_whitespace().collect::<Vec<_>>();
    if parts.len() < 8 {
        return None;
    }

    let symbol_type = parts[3];
    let bind = parts[4];
    let visibility = parts[5];
    let index = parts[6];
    let name = parts[7].split('@').next().unwrap_or(parts[7]);

    if index == "UND" || visibility == "HIDDEN" || visibility == "INTERNAL" || name.is_empty() {
        return None;
    }
    if bind != "GLOBAL" && bind != "WEAK" {
        return None;
    }

    let kind = match symbol_type {
        "FUNC" | "IFUNC" => 'T',
        "OBJECT" => 'D',
        "NOTYPE" => 'B',
        other => {
            if other == "TLS" {
                'D'
            } else {
                return None;
            }
        }
    };

    Some(DefinedSymbol {
        kind,
        name: name.to_owned(),
    })
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
