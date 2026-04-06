use crate::link::{
    capture_defined_symbols, find_library_artifact, inspect_shared_library, DefinedSymbol,
    RelinkManifest, LIBRARIES, RELINK_TARGETS,
};
use crate::package::{
    InstallSurface, EXPECTED_CMAKE_FILES, EXPECTED_HEADERS, EXPECTED_MANPAGES, EXPECTED_PACKAGES,
    EXPECTED_PKGCONFIG_FILES, EXPECTED_WEBP_MANPAGE_GLOBS, EXPECTED_WEBP_TOOLS,
};
use crate::tools::{nonempty_lines, read_json, run, sort_dedup};
use anyhow::{bail, Context, Result};
use clap::Args;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

#[derive(Args, Clone, Debug)]
pub struct VerifyNeededArgs {
    #[arg(long)]
    pub baseline_dir: PathBuf,
    #[arg(long, value_name = "LIB", num_args = 1..)]
    pub libs: Vec<String>,
    #[arg(long, value_name = "DIR")]
    pub library_dir: Option<PathBuf>,
}

#[derive(Args, Clone, Debug)]
pub struct VerifySymbolsArgs {
    #[arg(long)]
    pub baseline_dir: PathBuf,
    #[arg(long, value_name = "LIB", num_args = 1..)]
    pub libs: Vec<String>,
    #[arg(long, value_name = "DIR")]
    pub library_dir: Option<PathBuf>,
}

#[derive(Args, Clone, Debug)]
pub struct VerifySymbolSubsetArgs {
    #[arg(long)]
    pub baseline_dir: PathBuf,
    #[arg(long, value_name = "LIB", num_args = 1..)]
    pub libs: Vec<String>,
    #[arg(long)]
    pub subset: String,
    #[arg(long, value_name = "DIR")]
    pub library_dir: Option<PathBuf>,
}

#[derive(Args, Clone, Debug)]
pub struct VerifySonamesArgs {
    #[arg(long, value_name = "LIB", num_args = 1..)]
    pub libs: Vec<String>,
    #[arg(long, value_name = "DIR")]
    pub library_dir: Option<PathBuf>,
}

#[derive(Args, Clone, Debug)]
pub struct CSmokeArgs {
    #[arg(long)]
    pub name: String,
    #[arg(long, value_name = "DIR")]
    pub library_dir: Option<PathBuf>,
}

#[derive(Clone, Copy)]
enum ExpectedSymbolKind {
    Function,
    Object,
}

#[derive(Clone, Copy)]
struct ExpectedSymbol {
    name: &'static str,
    kind: ExpectedSymbolKind,
}

const COMMON_RUNTIME_SYMBOLS: &[ExpectedSymbol] = &[
    ExpectedSymbol {
        name: "WebPMalloc",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPFree",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPSafeMalloc",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPSafeCalloc",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPSafeFree",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPSetWorkerInterface",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPGetWorkerInterface",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "VP8GetCPUInfo",
        kind: ExpectedSymbolKind::Object,
    },
];

pub fn verify_baseline_manifests(baseline_dir: &Path) -> Result<()> {
    verify_export_files(baseline_dir)?;

    let sonames = read_json::<BTreeMap<String, String>>(&baseline_dir.join("sonames.json"))?;
    assert_library_keys("sonames", sonames.keys().cloned().collect::<Vec<_>>())?;
    for (name, soname) in &sonames {
        if soname.trim().is_empty() {
            bail!("empty SONAME recorded for {name}");
        }
    }

    let needed = read_json::<BTreeMap<String, Vec<String>>>(&baseline_dir.join("needed.json"))?;
    assert_library_keys("needed", needed.keys().cloned().collect::<Vec<_>>())?;
    for (name, edges) in &needed {
        let mut normalized = edges.clone();
        sort_dedup(&mut normalized);
        if *edges != normalized {
            bail!("DT_NEEDED entries for {name} are not sorted and deduplicated");
        }
    }

    let install_surface = read_json::<InstallSurface>(&baseline_dir.join("install-surface.json"))?;
    assert_exact(
        "package_names",
        &install_surface.package_names,
        EXPECTED_PACKAGES,
    )?;
    assert_exact("headers", &install_surface.headers, EXPECTED_HEADERS)?;
    assert_exact(
        "pkg_config_files",
        &install_surface.pkg_config_files,
        EXPECTED_PKGCONFIG_FILES,
    )?;
    assert_exact(
        "cmake_files",
        &install_surface.cmake_files,
        EXPECTED_CMAKE_FILES,
    )?;
    assert_exact("binaries", &install_surface.binaries, EXPECTED_WEBP_TOOLS)?;
    assert_exact("manpages", &install_surface.manpages, EXPECTED_MANPAGES)?;

    let package_keys = install_surface
        .packages
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let expected_keys = EXPECTED_PACKAGES.iter().copied().collect::<BTreeSet<_>>();
    if package_keys != expected_keys {
        bail!(
            "install-surface package entries mismatch: expected {:?}, found {:?}",
            EXPECTED_PACKAGES,
            install_surface.packages.keys().collect::<Vec<_>>()
        );
    }
    for (package, entry) in &install_surface.packages {
        if entry.install_globs.is_empty() {
            bail!("package {package} has no recorded install globs");
        }
        if package == "webp" {
            assert_exact(
                "webp.manpages globs",
                &entry.manpage_globs,
                EXPECTED_WEBP_MANPAGE_GLOBS,
            )?;
        } else if !entry.manpage_globs.is_empty() {
            bail!("unexpected Debian manpage globs recorded for package {package}");
        }
    }

    let relink_manifest = read_json::<RelinkManifest>(&baseline_dir.join("relink-manifest.json"))?;
    let expected_targets = RELINK_TARGETS
        .iter()
        .map(|(target, _, _)| *target)
        .collect::<BTreeSet<_>>();
    let actual_targets = relink_manifest
        .targets
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if actual_targets != expected_targets {
        bail!(
            "relink target mismatch: expected {:?}, found {:?}",
            expected_targets,
            actual_targets
        );
    }
    for (target, source, link_txt) in RELINK_TARGETS {
        let entry = relink_manifest
            .targets
            .get(*target)
            .with_context(|| format!("missing relink target {target}"))?;
        if entry.link_line.trim().is_empty() {
            bail!("empty relink command recorded for {target}");
        }
        if entry.source != *source {
            bail!("unexpected source for {target}: {}", entry.source);
        }
        if entry.link_txt != *link_txt {
            bail!("unexpected link.txt path for {target}: {}", entry.link_txt);
        }
    }

    Ok(())
}

pub fn verify_needed(args: &VerifyNeededArgs) -> Result<()> {
    let expected =
        read_json::<BTreeMap<String, Vec<String>>>(&args.baseline_dir.join("needed.json"))?;
    let libraries = resolve_libraries(&args.libs, args.library_dir.as_deref())?;
    for (name, path) in libraries {
        let info = inspect_shared_library(&path)?;
        let wanted = expected
            .get(&name)
            .with_context(|| format!("missing baseline DT_NEEDED entry for {name}"))?;
        if &info.needed != wanted {
            bail!(
                "DT_NEEDED mismatch for {name}: expected {:?}, found {:?}",
                wanted,
                info.needed
            );
        }
    }
    Ok(())
}

pub fn verify_symbols(args: &VerifySymbolsArgs) -> Result<()> {
    let libraries = resolve_libraries(&args.libs, args.library_dir.as_deref())?;
    for (name, path) in libraries {
        let actual = inspect_shared_library(&path)?.exports;
        let expected = nonempty_lines(
            &fs::read_to_string(args.baseline_dir.join(format!("{name}.exports")))
                .with_context(|| format!("failed to read baseline exports for {name}"))?,
        );
        if actual != expected {
            bail!(
                "export mismatch for {name}: expected {:?}, found {:?}",
                expected,
                actual
            );
        }
    }
    Ok(())
}

pub fn verify_symbol_subset(args: &VerifySymbolSubsetArgs) -> Result<()> {
    let subset = match args.subset.as_str() {
        "common_runtime" => COMMON_RUNTIME_SYMBOLS,
        other => bail!("unknown symbol subset `{other}`"),
    };
    let libraries = resolve_libraries(&args.libs, args.library_dir.as_deref())?;
    for (name, path) in libraries {
        let baseline = nonempty_lines(
            &fs::read_to_string(args.baseline_dir.join(format!("{name}.exports")))
                .with_context(|| format!("failed to read baseline exports for {name}"))?,
        );
        for expected in subset {
            if !baseline.iter().any(|symbol| symbol == expected.name) {
                bail!(
                    "subset member {} is not present in the {} baseline",
                    expected.name,
                    name
                );
            }
        }

        let actual = capture_defined_symbols(&path)?;
        for expected in subset {
            let symbol = actual
                .iter()
                .find(|symbol| symbol.name == expected.name)
                .with_context(|| format!("missing subset symbol {} in {name}", expected.name))?;
            if !symbol_kind_matches(symbol, expected.kind) {
                bail!(
                    "symbol kind mismatch for {} in {name}: expected {}, found `{}`",
                    expected.name,
                    expected_kind_name(expected.kind),
                    symbol.kind
                );
            }
        }
    }
    Ok(())
}

pub fn verify_sonames(args: &VerifySonamesArgs) -> Result<()> {
    let baseline_dir = workspace_root().join("abi/original");
    let expected = read_json::<BTreeMap<String, String>>(&baseline_dir.join("sonames.json"))?;
    let libraries = resolve_libraries(&args.libs, args.library_dir.as_deref())?;
    for (name, path) in libraries {
        let info = inspect_shared_library(&path)?;
        let wanted = expected
            .get(&name)
            .with_context(|| format!("missing baseline SONAME entry for {name}"))?;
        if &info.soname != wanted {
            bail!(
                "SONAME mismatch for {name}: expected `{wanted}`, found `{}`",
                info.soname
            );
        }
    }
    Ok(())
}

pub fn c_smoke(args: &CSmokeArgs) -> Result<()> {
    match args.name.as_str() {
        "sharpyuv_runtime" => run_sharpyuv_runtime_smoke(args.library_dir.as_deref()),
        other => bail!("unknown c-smoke scenario `{other}`"),
    }
}

fn verify_export_files(baseline_dir: &Path) -> Result<()> {
    for library in LIBRARIES {
        let path = baseline_dir.join(format!("{library}.exports"));
        let contents = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        let lines = nonempty_lines(&contents);
        if lines.is_empty() {
            bail!("no exports recorded in {}", path.display());
        }
        let mut normalized = lines.clone();
        sort_dedup(&mut normalized);
        if lines != normalized {
            bail!(
                "exports in {} are not sorted and deduplicated",
                path.display()
            );
        }
    }
    Ok(())
}

fn resolve_libraries(
    libs: &[String],
    library_dir: Option<&Path>,
) -> Result<BTreeMap<String, PathBuf>> {
    let logical_names = select_libraries(libs)?;
    let search_dir = library_dir
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| workspace_root().join("target/release"));
    let mut libraries = BTreeMap::new();
    for name in logical_names {
        let path = find_library_artifact(&search_dir, &name)
            .with_context(|| format!("failed to resolve built artifact for {name}"))?;
        libraries.insert(name, path);
    }
    Ok(libraries)
}

fn select_libraries(libs: &[String]) -> Result<Vec<String>> {
    if libs.is_empty() {
        return Ok(LIBRARIES.iter().map(|value| (*value).to_owned()).collect());
    }
    let allowed = LIBRARIES.iter().copied().collect::<BTreeSet<_>>();
    let mut selected = libs
        .iter()
        .map(|name| {
            if !allowed.contains(name.as_str()) {
                bail!("unknown library `{name}`");
            }
            Ok(name.clone())
        })
        .collect::<Result<Vec<_>>>()?;
    sort_dedup(&mut selected);
    Ok(selected)
}

fn symbol_kind_matches(symbol: &DefinedSymbol, expected: ExpectedSymbolKind) -> bool {
    match expected {
        ExpectedSymbolKind::Function => matches!(symbol.kind, 'T' | 't' | 'W' | 'w' | 'i'),
        ExpectedSymbolKind::Object => {
            matches!(
                symbol.kind,
                'B' | 'b' | 'D' | 'd' | 'R' | 'r' | 'G' | 'g' | 'S' | 's' | 'V' | 'v'
            )
        }
    }
}

fn expected_kind_name(kind: ExpectedSymbolKind) -> &'static str {
    match kind {
        ExpectedSymbolKind::Function => "function",
        ExpectedSymbolKind::Object => "object",
    }
}

fn run_sharpyuv_runtime_smoke(library_dir: Option<&Path>) -> Result<()> {
    let temp_dir = TempDir::new().context("failed to create temporary smoke directory")?;
    let source_path = temp_dir.path().join("sharpyuv_runtime.c");
    let binary_path = temp_dir.path().join("sharpyuv_runtime");
    let include_dir = workspace_root().join("include");
    let search_dir = library_dir
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| workspace_root().join("target/release"));
    let sharpyuv = find_library_artifact(&search_dir, "libsharpyuv")?;
    let rpath_dir = sharpyuv
        .parent()
        .with_context(|| format!("missing parent directory for {}", sharpyuv.display()))?;

    fs::write(&source_path, sharpyuv_runtime_source())?;

    let mut compile = Command::new("cc");
    compile
        .arg("-std=c11")
        .arg(format!("-I{}", include_dir.display()))
        .arg(format!("-I{}", include_dir.join("webp").display()))
        .arg(&source_path)
        .arg(&sharpyuv)
        .arg("-Wl,-rpath")
        .arg(format!("-Wl,{}", rpath_dir.display()))
        .arg("-lm")
        .arg("-o")
        .arg(&binary_path);
    run(&mut compile)?;

    let mut smoke = Command::new(&binary_path);
    smoke.env("LD_LIBRARY_PATH", rpath_dir);
    run(&mut smoke)
}

fn sharpyuv_runtime_source() -> &'static str {
    r#"#include <stdint.h>
#include <string.h>
#include "webp/sharpyuv/sharpyuv.h"
#include "webp/sharpyuv/sharpyuv_csp.h"

typedef int (*VP8CPUInfo)(int feature);
extern void SharpYuvInit(VP8CPUInfo cpu_info_func);

static int FakeCPUInfo(int feature) {
  return feature == 0;
}

int main(void) {
  SharpYuvConversionMatrix matrix;
  SharpYuvColorSpace colorspace = {0.299f, 0.114f, 8, kSharpYuvRangeLimited};
  const SharpYuvConversionMatrix* rec709_full;
  uint8_t r[4] = {0, 0, 0, 0};
  uint8_t g[4] = {0, 0, 0, 0};
  uint8_t b[4] = {0, 0, 0, 0};
  uint8_t y[4] = {0xff, 0xff, 0xff, 0xff};
  uint8_t u[1] = {0};
  uint8_t v[1] = {0};

  if (SharpYuvGetVersion() != 0x00020001) return 1;
  SharpYuvComputeConversionMatrix(&colorspace, &matrix);
  if (matrix.rgb_to_y[0] != 16829 || matrix.rgb_to_y[1] != 33039 ||
      matrix.rgb_to_y[2] != 6416 || matrix.rgb_to_y[3] != (16 << 16)) {
    return 2;
  }
  rec709_full = SharpYuvGetConversionMatrix(kSharpYuvMatrixRec709Full);
  if (rec709_full == NULL || rec709_full->rgb_to_y[0] != 13933 ||
      rec709_full->rgb_to_u[2] != 32768) {
    return 3;
  }

  SharpYuvInit(FakeCPUInfo);
  if (!SharpYuvConvert(r, g, b, 1, 2, 8, y, 2, u, 1, v, 1, 8, 2, 2,
                       SharpYuvGetConversionMatrix(kSharpYuvMatrixWebp))) {
    return 4;
  }
  if (memcmp(y, (uint8_t[]){16, 16, 16, 16}, sizeof(y)) != 0) return 5;
  if (u[0] != 128 || v[0] != 128) return 6;
  return 0;
}
"#
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask manifest should live under the workspace root")
        .to_path_buf()
}

fn assert_exact(label: &str, actual: &[String], expected: &[&str]) -> Result<()> {
    let expected = expected
        .iter()
        .map(|value| (*value).to_owned())
        .collect::<Vec<_>>();
    if actual != expected {
        bail!(
            "{label} mismatch: expected {:?}, found {:?}",
            expected,
            actual
        );
    }
    Ok(())
}

fn assert_library_keys(label: &str, actual: Vec<String>) -> Result<()> {
    let expected = LIBRARIES
        .iter()
        .map(|value| (*value).to_owned())
        .collect::<Vec<_>>();
    if actual != expected {
        bail!(
            "{label} mismatch: expected {:?}, found {:?}",
            expected,
            actual
        );
    }
    Ok(())
}
