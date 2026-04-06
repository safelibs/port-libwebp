use crate::link::{
    capture_defined_symbols, find_library_artifact, inspect_shared_library, DefinedSymbol,
    RelinkManifest, LIBRARIES, RELINK_TARGETS,
};
use crate::package::{
    build_upstream_tools_into, build_workspace_libraries, select_upstream_tools,
    stage_library_artifact, InstallSurface, EXPECTED_CMAKE_FILES, EXPECTED_HEADERS,
    EXPECTED_MANPAGES, EXPECTED_PACKAGES, EXPECTED_PKGCONFIG_FILES, EXPECTED_WEBP_INSTALL_PATHS,
    EXPECTED_WEBP_MANPAGE_GLOBS, EXPECTED_WEBP_MANPAGE_PATHS, EXPECTED_WEBP_TOOLS,
};
use crate::tools::{
    copy_dir_contents, nonempty_lines, read_json, repo_root, reset_dir, run, sort_dedup,
    workspace_root,
};
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
pub struct VerifyInstallTreeArgs {
    #[arg(long)]
    pub baseline: PathBuf,
    #[arg(long, value_name = "DIR")]
    pub package_dir: PathBuf,
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

#[derive(Args, Clone, Debug)]
pub struct BuildCTestsArgs {
    #[arg(long)]
    pub suite: String,
    #[arg(long, value_name = "DIR")]
    pub library_dir: Option<PathBuf>,
}

#[derive(Args, Clone, Debug)]
pub struct BuildUpstreamPublicApiTestArgs {
    #[arg(long)]
    pub source: PathBuf,
    #[arg(long, value_name = "DIR")]
    pub library_dir: Option<PathBuf>,
}

#[derive(Args, Clone, Debug)]
pub struct BuildUpstreamToolsArgs {
    #[arg(long, value_name = "TOOL", num_args = 1..)]
    pub tools: Vec<String>,
    #[arg(long, value_name = "DIR")]
    pub safe_prefix: PathBuf,
}

#[derive(Args, Clone, Debug)]
pub struct ToolSmokeArgs {
    #[arg(long, value_name = "DIR")]
    pub prefix: PathBuf,
    #[arg(long, value_name = "TOOL", num_args = 1..)]
    pub tools: Vec<String>,
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

const ENCODE_SYMBOLS: &[ExpectedSymbol] = &[
    ExpectedSymbol {
        name: "WebPBlendAlpha",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPCleanupTransparentArea",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPConfigInitInternal",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPConfigLosslessPreset",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPCopyPixels",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPCopyPlane",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPEncode",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPEncodeBGR",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPEncodeBGRA",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPEncodeLosslessBGR",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPEncodeLosslessBGRA",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPEncodeLosslessRGB",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPEncodeLosslessRGBA",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPEncodeRGB",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPEncodeRGBA",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPGetColorPalette",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPGetEncoderVersion",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPMemoryWrite",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPMemoryWriterClear",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPMemoryWriterInit",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureARGBToYUVA",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureARGBToYUVADithered",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureAlloc",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureCopy",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureCrop",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureFree",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureHasTransparency",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureImportBGR",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureImportBGRA",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureImportBGRX",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureImportRGB",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureImportRGBA",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureImportRGBX",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureInitInternal",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureIsView",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureDistortion",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureRescale",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureSharpARGBToYUVA",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureSmartARGBToYUVA",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureView",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPictureYUVAToARGB",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPPlaneDistortion",
        kind: ExpectedSymbolKind::Function,
    },
    ExpectedSymbol {
        name: "WebPValidateConfig",
        kind: ExpectedSymbolKind::Function,
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

pub fn verify_install_tree(args: &VerifyInstallTreeArgs) -> Result<()> {
    let baseline = read_json::<InstallSurface>(&args.baseline)?;
    assert_exact("package_names", &baseline.package_names, EXPECTED_PACKAGES)?;
    assert_exact("headers", &baseline.headers, EXPECTED_HEADERS)?;
    assert_exact(
        "pkg_config_files",
        &baseline.pkg_config_files,
        EXPECTED_PKGCONFIG_FILES,
    )?;
    assert_exact("cmake_files", &baseline.cmake_files, EXPECTED_CMAKE_FILES)?;
    assert_exact("binaries", &baseline.binaries, EXPECTED_WEBP_TOOLS)?;
    assert_exact("manpages", &baseline.manpages, EXPECTED_MANPAGES)?;

    let packages = extract_binary_packages(&args.package_dir)?;
    let package_names = packages.keys().cloned().collect::<BTreeSet<_>>();
    let expected_package_names = EXPECTED_PACKAGES
        .iter()
        .map(|package| (*package).to_owned())
        .collect::<BTreeSet<_>>();
    if package_names != expected_package_names {
        bail!(
            "built package names mismatch: expected {:?}, found {:?}",
            EXPECTED_PACKAGES,
            packages.keys().collect::<Vec<_>>()
        );
    }

    let webp_files = packages
        .get("webp")
        .context("missing extracted webp package")?;
    let mut expected_webp_binaries = EXPECTED_WEBP_INSTALL_PATHS
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<Vec<_>>();
    sort_dedup(&mut expected_webp_binaries);
    assert_exact_values(
        "webp binaries",
        &collect_files_under(webp_files, "usr/bin"),
        expected_webp_binaries,
    )?;
    let manpages = webp_files
        .iter()
        .filter_map(|path| {
            path.strip_prefix("usr/share/man/man1/")
                .map(|_| normalize_manpage_path(path))
        })
        .collect::<Vec<_>>();
    assert_exact("webp manpages", &manpages, EXPECTED_WEBP_MANPAGE_PATHS)?;

    let multiarch = detect_extracted_multiarch(&packages)?;
    let lib_dir = format!("usr/lib/{multiarch}");
    assert_exact(
        "libwebp-dev headers",
        &collect_files_under(
            packages
                .get("libwebp-dev")
                .context("missing extracted libwebp-dev package")?,
            "usr/include/webp",
        ),
        &[
            "usr/include/webp/decode.h",
            "usr/include/webp/demux.h",
            "usr/include/webp/encode.h",
            "usr/include/webp/mux.h",
            "usr/include/webp/mux_types.h",
            "usr/include/webp/types.h",
        ],
    )?;
    assert_exact(
        "libsharpyuv-dev headers",
        &collect_files_under(
            packages
                .get("libsharpyuv-dev")
                .context("missing extracted libsharpyuv-dev package")?,
            "usr/include/webp/sharpyuv",
        ),
        &[
            "usr/include/webp/sharpyuv/sharpyuv.h",
            "usr/include/webp/sharpyuv/sharpyuv_csp.h",
        ],
    )?;
    assert_exact_values(
        "libwebp-dev pkg-config files",
        &collect_files_under(
            packages
                .get("libwebp-dev")
                .context("missing extracted libwebp-dev package")?,
            &format!("{lib_dir}/pkgconfig"),
        ),
        vec![
            format!("{lib_dir}/pkgconfig/libwebp.pc"),
            format!("{lib_dir}/pkgconfig/libwebpdecoder.pc"),
            format!("{lib_dir}/pkgconfig/libwebpdemux.pc"),
            format!("{lib_dir}/pkgconfig/libwebpmux.pc"),
        ],
    )?;
    assert_exact_values(
        "libsharpyuv-dev pkg-config files",
        &collect_files_under(
            packages
                .get("libsharpyuv-dev")
                .context("missing extracted libsharpyuv-dev package")?,
            &format!("{lib_dir}/pkgconfig"),
        ),
        vec![format!("{lib_dir}/pkgconfig/libsharpyuv.pc")],
    )?;
    assert_exact_values(
        "libwebp-dev cmake files",
        &collect_files_under(
            packages
                .get("libwebp-dev")
                .context("missing extracted libwebp-dev package")?,
            &format!("{lib_dir}/cmake/WebP"),
        ),
        vec![
            format!("{lib_dir}/cmake/WebP/WebPConfig.cmake"),
            format!("{lib_dir}/cmake/WebP/WebPConfigVersion.cmake"),
            format!("{lib_dir}/cmake/WebP/WebPTargets.cmake"),
        ],
    )?;
    assert_exact_values(
        "libwebp-dev linker symlinks",
        &collect_files_with_suffix(
            packages
                .get("libwebp-dev")
                .context("missing extracted libwebp-dev package")?,
            &format!("{lib_dir}/"),
            ".so",
        ),
        vec![
            format!("{lib_dir}/libwebp.so"),
            format!("{lib_dir}/libwebpdecoder.so"),
            format!("{lib_dir}/libwebpdemux.so"),
            format!("{lib_dir}/libwebpmux.so"),
        ],
    )?;
    assert_exact_values(
        "libsharpyuv-dev linker symlinks",
        &collect_files_with_suffix(
            packages
                .get("libsharpyuv-dev")
                .context("missing extracted libsharpyuv-dev package")?,
            &format!("{lib_dir}/"),
            ".so",
        ),
        vec![format!("{lib_dir}/libsharpyuv.so")],
    )?;

    for (package, expected_file) in [
        ("libwebp7", format!("{lib_dir}/libwebp.so.7")),
        ("libwebpdecoder3", format!("{lib_dir}/libwebpdecoder.so.3")),
        ("libwebpdemux2", format!("{lib_dir}/libwebpdemux.so.2")),
        ("libwebpmux3", format!("{lib_dir}/libwebpmux.so.3")),
        ("libsharpyuv0", format!("{lib_dir}/libsharpyuv.so.0")),
    ] {
        assert_exact_values(
            &format!("{package} runtime files"),
            &collect_files_under(
                packages
                    .get(package)
                    .with_context(|| format!("missing extracted {package} package"))?,
                &lib_dir,
            ),
            vec![expected_file],
        )?;
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
    let subset = resolve_symbol_subset(args)?;
    let libraries = resolve_libraries(&args.libs, args.library_dir.as_deref())?;
    for (name, path) in libraries {
        let baseline = nonempty_lines(
            &fs::read_to_string(args.baseline_dir.join(format!("{name}.exports")))
                .with_context(|| format!("failed to read baseline exports for {name}"))?,
        );
        for expected in &subset {
            if !baseline.iter().any(|symbol| symbol == &expected.name) {
                bail!(
                    "subset member {} is not present in the {} baseline",
                    expected.name,
                    name
                );
            }
        }

        let actual = capture_defined_symbols(&path)?;
        for expected in &subset {
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

pub fn build_c_tests(args: &BuildCTestsArgs) -> Result<()> {
    let root = workspace_root();
    let search_dir = args
        .library_dir
        .clone()
        .unwrap_or_else(|| root.join("target/release"));
    let build_dir = root.join("build/tests");
    let source_dir = root.join("tests/c");
    let include_dir = root.join("include");
    let repo_root = root
        .parent()
        .context("workspace root should live under the repository root")?;
    let sample_webp = repo_root.join("original/examples/test.webp");
    let sample_ppm = repo_root.join("original/examples/test_ref.ppm");
    let need_webpdecoder = matches!(args.suite.as_str(), "decode_api" | "all");
    let need_webpdemux = matches!(args.suite.as_str(), "demux_animdecode" | "all");
    let need_webp = matches!(args.suite.as_str(), "decode_api" | "encode_api" | "all");

    let webpdecoder = if need_webpdecoder {
        Some(find_library_artifact(&search_dir, "libwebpdecoder")?)
    } else {
        None
    };
    let webpdemux = if need_webpdemux {
        Some(find_library_artifact(&search_dir, "libwebpdemux")?)
    } else {
        None
    };
    let webp = if need_webp {
        Some(find_library_artifact(&search_dir, "libwebp")?)
    } else {
        None
    };
    let oracle_webpdecoder = if need_webpdecoder {
        Some(find_system_library("libwebpdecoder")?)
    } else {
        None
    };
    let oracle_webpdemux = if need_webpdemux {
        Some(find_system_library("libwebpdemux")?)
    } else {
        None
    };
    let oracle_webp = if matches!(args.suite.as_str(), "decode_api" | "all") {
        Some(find_system_library("libwebp")?)
    } else {
        None
    };

    fs::create_dir_all(&build_dir)
        .with_context(|| format!("failed to create {}", build_dir.display()))?;

    let mut configure = Command::new("cmake");
    configure
        .arg("-S")
        .arg(&source_dir)
        .arg("-B")
        .arg(&build_dir)
        .arg(format!("-DTEST_SUITE={}", args.suite))
        .arg(format!("-DWEBP_INCLUDE_DIR={}", include_dir.display()))
        .arg(format!("-DTEST_WEBP_PATH={}", sample_webp.display()))
        .arg(format!("-DTEST_PPM_PATH={}", sample_ppm.display()));
    if let Some(path) = &webpdecoder {
        configure.arg(format!("-DWEBPDECODER_LIBRARY={}", path.display()));
    }
    if let Some(path) = &webpdemux {
        configure.arg(format!("-DWEBPDEMUX_LIBRARY={}", path.display()));
    }
    if let Some(path) = &webp {
        configure.arg(format!("-DWEBP_LIBRARY={}", path.display()));
    }
    if let Some(path) = &oracle_webpdecoder {
        configure.arg(format!("-DORACLE_WEBPDECODER_LIBRARY={}", path.display()));
    }
    if let Some(path) = &oracle_webpdemux {
        configure.arg(format!("-DORACLE_WEBPDEMUX_LIBRARY={}", path.display()));
    }
    if let Some(path) = &oracle_webp {
        configure.arg(format!("-DORACLE_WEBP_LIBRARY={}", path.display()));
    }
    run(&mut configure)?;

    let mut build = Command::new("cmake");
    build.arg("--build").arg(&build_dir);
    run(&mut build)
}

pub fn build_upstream_public_api_test(args: &BuildUpstreamPublicApiTestArgs) -> Result<()> {
    let root = workspace_root();
    let search_dir = args
        .library_dir
        .clone()
        .unwrap_or_else(|| root.join("target/release"));
    let build_dir = root.join("build/tests");
    let source_dir = root.join("tests/c");
    let include_glue_dir = build_dir.join("upstream-include");
    let webpmux = find_library_artifact(&search_dir, "libwebpmux")?;
    let webpdemux = find_library_artifact(&search_dir, "libwebpdemux")?;
    let webp = find_library_artifact(&search_dir, "libwebp")?;

    prepare_upstream_include_glue(&root.join("include"), &include_glue_dir)?;
    fs::create_dir_all(&build_dir)
        .with_context(|| format!("failed to create {}", build_dir.display()))?;

    let mut configure = Command::new("cmake");
    configure
        .arg("-S")
        .arg(&source_dir)
        .arg("-B")
        .arg(&build_dir)
        .arg("-DTEST_SUITE=upstream_public_api")
        .arg(format!(
            "-DUPSTREAM_PUBLIC_API_SOURCE={}",
            args.source.display()
        ))
        .arg(format!(
            "-DUPSTREAM_INCLUDE_GLUE_DIR={}",
            include_glue_dir.display()
        ))
        .arg(format!("-DWEBPMUX_LIBRARY={}", webpmux.display()))
        .arg(format!("-DWEBPDEMUX_LIBRARY={}", webpdemux.display()))
        .arg(format!("-DWEBP_LIBRARY={}", webp.display()))
        .arg(format!(
            "-DWEBP_INCLUDE_DIR={}",
            root.join("include").display()
        ));
    run(&mut configure)?;

    let mut build = Command::new("cmake");
    build
        .arg("--build")
        .arg(&build_dir)
        .arg("--target")
        .arg("webp_public_api_test");
    run(&mut build)
}

pub fn build_upstream_tools(args: &BuildUpstreamToolsArgs) -> Result<()> {
    let requested = select_upstream_tools(&args.tools)?;
    let root = workspace_root();
    let search_dir = root.join("target/release");
    let prefix = &args.safe_prefix;

    build_workspace_libraries(&root)?;

    stage_flat_prefix(&search_dir, prefix)?;
    build_upstream_tools_into(
        &requested,
        &prefix.join("include"),
        &prefix.join("lib"),
        &prefix.join("bin"),
    )
}

pub fn tool_smoke(args: &ToolSmokeArgs) -> Result<()> {
    let requested = select_upstream_tools(&args.tools)?;
    let root = workspace_root();
    let repo_root = repo_root()?;
    let bin_dir = args.prefix.join("bin");
    let lib_dir = args.prefix.join("lib");
    let smoke_dir = root.join("build/tool-smoke");
    let sample_webp = repo_root.join("original/examples/test.webp");
    let sample_ppm = repo_root.join("original/examples/test_ref.ppm");
    let generated_webp = smoke_dir.join("smoke-lossless.webp");
    let decoded_ppm = smoke_dir.join("smoke-decoded.ppm");
    let muxed_webp = smoke_dir.join("smoke-img2webp.webp");

    reset_dir(&smoke_dir)?;

    if requested.iter().any(|tool| tool == "cwebp") {
        let mut encode = Command::new(bin_dir.join("cwebp"));
        encode
            .arg("-quiet")
            .arg("-lossless")
            .arg(&sample_ppm)
            .arg("-o")
            .arg(&generated_webp)
            .env("LD_LIBRARY_PATH", &lib_dir);
        run(&mut encode)?;
        ensure_nonempty_file(&generated_webp)?;
    }

    if requested.iter().any(|tool| tool == "dwebp") {
        let input = if generated_webp.exists() {
            &generated_webp
        } else {
            &sample_webp
        };
        let mut decode = Command::new(bin_dir.join("dwebp"));
        decode
            .arg(input)
            .arg("-ppm")
            .arg("-o")
            .arg(&decoded_ppm)
            .env("LD_LIBRARY_PATH", &lib_dir);
        run(&mut decode)?;
        ensure_nonempty_file(&decoded_ppm)?;
    }

    if requested.iter().any(|tool| tool == "img2webp") {
        let mut img2webp = Command::new(bin_dir.join("img2webp"));
        img2webp
            .arg("-o")
            .arg(&muxed_webp)
            .arg(&sample_ppm)
            .env("LD_LIBRARY_PATH", &lib_dir);
        run(&mut img2webp)?;
        ensure_nonempty_file(&muxed_webp)?;
    }

    if requested.iter().any(|tool| tool == "webpmux") {
        let input = if muxed_webp.exists() {
            &muxed_webp
        } else {
            &sample_webp
        };
        let mut webpmux = Command::new(bin_dir.join("webpmux"));
        webpmux
            .arg("-info")
            .arg(input)
            .env("LD_LIBRARY_PATH", &lib_dir);
        run(&mut webpmux)?;
    }

    if requested.iter().any(|tool| tool == "webpinfo") {
        let input = if muxed_webp.exists() {
            &muxed_webp
        } else {
            &sample_webp
        };
        let mut webpinfo = Command::new(bin_dir.join("webpinfo"));
        webpinfo.arg(input).env("LD_LIBRARY_PATH", &lib_dir);
        run(&mut webpinfo)?;
    }

    if requested.iter().any(|tool| tool == "gif2webp") {
        let mut gif2webp = Command::new(bin_dir.join("gif2webp"));
        gif2webp.arg("-version").env("LD_LIBRARY_PATH", &lib_dir);
        run(&mut gif2webp)?;
    }

    if requested.iter().any(|tool| tool == "anim_diff") {
        let mut anim_diff = Command::new(bin_dir.join("anim_diff"));
        anim_diff.arg("-version").env("LD_LIBRARY_PATH", &lib_dir);
        run(&mut anim_diff)?;
    }

    if requested.iter().any(|tool| tool == "anim_dump") {
        let mut anim_dump = Command::new(bin_dir.join("anim_dump"));
        anim_dump.arg("-version").env("LD_LIBRARY_PATH", &lib_dir);
        run(&mut anim_dump)?;
    }

    if requested.iter().any(|tool| tool == "vwebp") {
        let mut vwebp = Command::new("xvfb-run");
        vwebp
            .arg("-a")
            .arg(bin_dir.join("vwebp"))
            .arg("-version")
            .env("LD_LIBRARY_PATH", &lib_dir);
        run(&mut vwebp)?;
    }

    Ok(())
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

#[derive(Clone)]
struct ExpectedSymbolSpec {
    name: String,
    kind: ExpectedSymbolKind,
}

fn resolve_symbol_subset(args: &VerifySymbolSubsetArgs) -> Result<Vec<ExpectedSymbolSpec>> {
    match args.subset.as_str() {
        "common_runtime" => Ok(COMMON_RUNTIME_SYMBOLS
            .iter()
            .map(|symbol| ExpectedSymbolSpec {
                name: symbol.name.to_owned(),
                kind: symbol.kind,
            })
            .collect()),
        "decode" => {
            let exports = nonempty_lines(
                &fs::read_to_string(args.baseline_dir.join("libwebpdecoder.exports"))
                    .with_context(|| "failed to read baseline exports for libwebpdecoder")?,
            );
            Ok(exports
                .into_iter()
                .map(|name| ExpectedSymbolSpec {
                    kind: if name == "VP8GetCPUInfo" {
                        ExpectedSymbolKind::Object
                    } else {
                        ExpectedSymbolKind::Function
                    },
                    name,
                })
                .collect())
        }
        "encode" => Ok(ENCODE_SYMBOLS
            .iter()
            .map(|symbol| ExpectedSymbolSpec {
                name: symbol.name.to_owned(),
                kind: symbol.kind,
            })
            .collect()),
        other => bail!("unknown symbol subset `{other}`"),
    }
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

fn prepare_upstream_include_glue(include_dir: &Path, dest: &Path) -> Result<()> {
    let webp_headers = include_dir.join("webp");
    let webp_dest = dest.join("src/webp");

    if dest.exists() {
        fs::remove_dir_all(dest).with_context(|| format!("failed to remove {}", dest.display()))?;
    }
    fs::create_dir_all(&webp_dest)
        .with_context(|| format!("failed to create {}", webp_dest.display()))?;
    copy_dir_contents(&webp_headers, &webp_dest)
}

fn ensure_nonempty_file(path: &Path) -> Result<()> {
    let metadata =
        fs::metadata(path).with_context(|| format!("failed to stat {}", path.display()))?;
    if !metadata.is_file() || metadata.len() == 0 {
        bail!("expected non-empty output file at {}", path.display());
    }
    Ok(())
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

fn find_system_library(logical_name: &str) -> Result<PathBuf> {
    let output = Command::new("ldconfig")
        .arg("-p")
        .output()
        .context("failed to execute ldconfig -p")?;
    if !output.status.success() {
        bail!("ldconfig -p failed with status {}", output.status);
    }

    let prefix = format!("{logical_name}.so");
    let stdout = String::from_utf8(output.stdout).context("ldconfig output was not valid UTF-8")?;
    let mut fallback = None;
    for line in stdout.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with(&prefix) {
            continue;
        }
        let Some((_, path)) = trimmed.split_once("=>") else {
            continue;
        };
        let path = PathBuf::from(path.trim());
        if trimmed.contains("x86-64") {
            return Ok(path);
        }
        fallback = Some(path);
    }

    fallback.with_context(|| format!("failed to locate system path for {logical_name}"))
}

fn stage_flat_prefix(search_dir: &Path, prefix: &Path) -> Result<()> {
    let root = workspace_root();
    let include_src = root.join("include");
    let include_dst = prefix.join("include");
    let lib_dst = prefix.join("lib");

    reset_dir(prefix)?;
    fs::create_dir_all(&include_dst)
        .with_context(|| format!("failed to create {}", include_dst.display()))?;
    fs::create_dir_all(&lib_dst)
        .with_context(|| format!("failed to create {}", lib_dst.display()))?;
    fs::create_dir_all(prefix.join("bin"))
        .with_context(|| format!("failed to create {}", prefix.join("bin").display()))?;

    copy_dir_contents(&include_src, &include_dst)?;
    for library in LIBRARIES {
        stage_library_artifact(search_dir, library, &lib_dst)?;
    }
    Ok(())
}

fn extract_binary_packages(package_dir: &Path) -> Result<BTreeMap<String, BTreeSet<String>>> {
    let temp_dir = TempDir::new().context("failed to create temporary extraction directory")?;
    let mut packages = BTreeMap::new();

    for entry in fs::read_dir(package_dir)
        .with_context(|| format!("failed to read {}", package_dir.display()))?
    {
        let path = entry?.path();
        if path.extension().is_none_or(|extension| extension != "deb") {
            continue;
        }
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .with_context(|| format!("invalid package filename {}", path.display()))?;
        let Some((package_name, _)) = file_name.split_once('_') else {
            continue;
        };
        if !EXPECTED_PACKAGES.contains(&package_name) {
            continue;
        }

        let extract_root = temp_dir.path().join(package_name);
        fs::create_dir_all(&extract_root)
            .with_context(|| format!("failed to create {}", extract_root.display()))?;
        let mut extract = Command::new("dpkg-deb");
        extract.arg("-x").arg(&path).arg(&extract_root);
        run(&mut extract)?;
        packages.insert(
            package_name.to_owned(),
            collect_relative_files(&extract_root)?,
        );
    }

    Ok(packages)
}

fn collect_relative_files(root: &Path) -> Result<BTreeSet<String>> {
    let mut files = BTreeSet::new();
    for entry in walkdir::WalkDir::new(root) {
        let entry = entry?;
        if !entry.file_type().is_file() && !entry.file_type().is_symlink() {
            continue;
        }
        let relative = entry.path().strip_prefix(root).with_context(|| {
            format!(
                "failed to compute relative path for {}",
                entry.path().display()
            )
        })?;
        files.insert(relative.to_string_lossy().into_owned());
    }
    Ok(files)
}

fn collect_files_under(files: &BTreeSet<String>, prefix: &str) -> Vec<String> {
    let mut matches = files
        .iter()
        .filter(|path| path.starts_with(&(prefix.to_owned() + "/")))
        .cloned()
        .collect::<Vec<_>>();
    sort_dedup(&mut matches);
    matches
}

fn collect_files_with_suffix(files: &BTreeSet<String>, prefix: &str, suffix: &str) -> Vec<String> {
    let mut matches = files
        .iter()
        .filter(|path| path.starts_with(prefix) && path.ends_with(suffix))
        .cloned()
        .collect::<Vec<_>>();
    sort_dedup(&mut matches);
    matches
}

fn normalize_manpage_path(path: &str) -> String {
    path.strip_suffix(".gz").unwrap_or(path).to_owned()
}

fn detect_extracted_multiarch(packages: &BTreeMap<String, BTreeSet<String>>) -> Result<String> {
    for files in packages.values() {
        for path in files {
            if let Some(rest) = path.strip_prefix("usr/lib/") {
                let triplet = rest.split('/').next().unwrap_or_default();
                if !triplet.is_empty() {
                    return Ok(triplet.to_owned());
                }
            }
        }
    }
    bail!("failed to determine multiarch triplet from extracted package contents")
}

fn assert_exact(label: &str, actual: &[String], expected: &[&str]) -> Result<()> {
    let expected = expected
        .iter()
        .map(|value| (*value).to_owned())
        .collect::<Vec<_>>();
    assert_exact_values(label, actual, expected)
}

fn assert_exact_values(label: &str, actual: &[String], expected: Vec<String>) -> Result<()> {
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
