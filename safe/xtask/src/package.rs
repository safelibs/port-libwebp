use anyhow::{bail, Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

pub const EXPECTED_PACKAGES: &[&str] = &[
    "libwebp-dev",
    "libwebp7",
    "libwebpdemux2",
    "libwebpmux3",
    "libwebpdecoder3",
    "libsharpyuv0",
    "libsharpyuv-dev",
    "webp",
];

pub const EXPECTED_HEADERS: &[&str] = &[
    "include/webp/decode.h",
    "include/webp/demux.h",
    "include/webp/encode.h",
    "include/webp/mux.h",
    "include/webp/mux_types.h",
    "include/webp/types.h",
    "include/webp/sharpyuv/sharpyuv.h",
    "include/webp/sharpyuv/sharpyuv_csp.h",
];

pub const EXPECTED_PKGCONFIG_FILES: &[&str] = &[
    "lib/pkgconfig/libwebp.pc",
    "lib/pkgconfig/libwebpdecoder.pc",
    "lib/pkgconfig/libwebpdemux.pc",
    "lib/pkgconfig/libwebpmux.pc",
    "lib/pkgconfig/libsharpyuv.pc",
];

pub const EXPECTED_CMAKE_FILES: &[&str] = &[
    "lib/cmake/WebP/WebPConfig.cmake",
    "lib/cmake/WebP/WebPConfigVersion.cmake",
    "lib/cmake/WebP/WebPTargets.cmake",
];

pub const EXPECTED_WEBP_TOOLS: &[&str] = &[
    "cwebp",
    "dwebp",
    "gif2webp",
    "img2webp",
    "vwebp",
    "webpinfo",
    "webpmux",
    "anim_diff",
    "anim_dump",
];

pub const EXPECTED_MANPAGES: &[&str] = &[
    "cwebp.1",
    "dwebp.1",
    "gif2webp.1",
    "img2webp.1",
    "vwebp.1",
    "webpinfo.1",
    "webpmux.1",
];

pub const EXPECTED_WEBP_MANPAGE_GLOBS: &[&str] = &["usr/share/man/man1/*.1"];

#[derive(Debug, Deserialize, Serialize)]
pub struct InstallSurface {
    pub binaries: Vec<String>,
    pub cmake_files: Vec<String>,
    pub headers: Vec<String>,
    pub manpages: Vec<String>,
    pub package_names: Vec<String>,
    pub packages: BTreeMap<String, PackageInstall>,
    pub pkg_config_files: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageInstall {
    pub install_globs: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manpage_globs: Vec<String>,
}

pub fn capture_install_surface(original_dir: &Path) -> Result<InstallSurface> {
    let debian_dir = original_dir.join("debian");
    let cmake_path = original_dir.join("CMakeLists.txt");
    let man_makefile_path = original_dir.join("man/Makefile.am");
    let cmake = fs::read_to_string(&cmake_path)
        .with_context(|| format!("failed to read {}", cmake_path.display()))?;
    let man_makefile = fs::read_to_string(&man_makefile_path)
        .with_context(|| format!("failed to read {}", man_makefile_path.display()))?;

    let package_names = parse_control_packages(&debian_dir.join("control"))?;
    let mut packages = parse_install_files(&debian_dir)?;
    let webp_manpage_globs = parse_webp_manpages(&debian_dir.join("webp.manpages"))?;
    packages
        .get_mut("webp")
        .context("missing `webp` package entry for Debian manpages")?
        .manpage_globs = webp_manpage_globs;
    let headers = parse_public_headers(&cmake)?;
    let pkg_config_files = parse_pkg_config_files(&cmake)?;
    let cmake_files = parse_cmake_install_files(&cmake)?;
    let binaries = parse_binaries(&cmake)?;
    let manpages = parse_manpages(&cmake, &man_makefile)?;

    Ok(InstallSurface {
        binaries,
        cmake_files,
        headers,
        manpages,
        package_names,
        packages,
        pkg_config_files,
    })
}

fn parse_control_packages(path: &Path) -> Result<Vec<String>> {
    let contents =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let package_names = contents
        .lines()
        .filter_map(|line| line.strip_prefix("Package: "))
        .map(str::trim)
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    require_exact("package names", &package_names, EXPECTED_PACKAGES)
}

fn parse_install_files(debian_dir: &Path) -> Result<BTreeMap<String, PackageInstall>> {
    let mut install_paths = fs::read_dir(debian_dir)?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "install")
        })
        .collect::<Vec<PathBuf>>();
    install_paths.sort();

    let mut packages = BTreeMap::new();
    for path in install_paths {
        let package_name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .with_context(|| format!("invalid install filename {}", path.display()))?
            .to_owned();
        let install_globs = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        packages.insert(
            package_name,
            PackageInstall {
                install_globs,
                manpage_globs: Vec::new(),
            },
        );
    }

    let actual = packages.keys().map(String::as_str).collect::<Vec<_>>();
    let expected = EXPECTED_PACKAGES.iter().copied().collect::<BTreeSet<_>>();
    let found = actual.iter().copied().collect::<BTreeSet<_>>();
    if found != expected {
        bail!(
            "unexpected package install files: expected {:?}, found {:?}",
            EXPECTED_PACKAGES,
            actual
        );
    }
    Ok(packages)
}

fn parse_webp_manpages(path: &Path) -> Result<Vec<String>> {
    let globs = fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    require_exact("webp.manpages globs", &globs, EXPECTED_WEBP_MANPAGE_GLOBS)
}

fn parse_public_headers(cmake: &str) -> Result<Vec<String>> {
    let block_regex = Regex::new(r#"(?s)PUBLIC_HEADER\s+"([^"]+)""#)?;
    let header_regex = Regex::new(r#"(src/webp|sharpyuv)/[A-Za-z0-9_]+\.h"#)?;
    let mut headers = BTreeSet::new();
    for captures in block_regex.captures_iter(cmake) {
        let block = &captures[1];
        for capture in header_regex.find_iter(block) {
            headers.insert(to_installed_header(capture.as_str())?);
        }
    }

    let actual = headers.into_iter().collect::<Vec<_>>();
    require_set("headers", &actual, EXPECTED_HEADERS)?;
    Ok(EXPECTED_HEADERS
        .iter()
        .map(|value| (*value).to_owned())
        .collect())
}

fn parse_pkg_config_files(cmake: &str) -> Result<Vec<String>> {
    let regex = Regex::new(r#"configure_pkg_config\("([^"]+\.pc)"\)"#)?;
    let mut pkg_config_files = BTreeSet::new();
    for captures in regex.captures_iter(cmake) {
        let file_name = Path::new(&captures[1])
            .file_name()
            .and_then(|file| file.to_str())
            .context("invalid pkg-config filename")?;
        pkg_config_files.insert(format!("lib/pkgconfig/{file_name}"));
    }

    let actual = pkg_config_files.into_iter().collect::<Vec<_>>();
    require_set("pkg-config files", &actual, EXPECTED_PKGCONFIG_FILES)?;
    Ok(EXPECTED_PKGCONFIG_FILES
        .iter()
        .map(|value| (*value).to_owned())
        .collect())
}

fn parse_cmake_install_files(cmake: &str) -> Result<Vec<String>> {
    let export_present = cmake.contains("install(EXPORT ${PROJECT_NAME}Targets");
    let config_present = cmake.contains("WebPConfig.cmake.in");
    let version_present = cmake.contains("WebPConfigVersion.cmake");
    if !(export_present && config_present && version_present) {
        bail!("failed to confirm installed CMake package files from original/CMakeLists.txt");
    }
    Ok(EXPECTED_CMAKE_FILES
        .iter()
        .map(|value| (*value).to_owned())
        .collect())
}

fn parse_binaries(cmake: &str) -> Result<Vec<String>> {
    let regex = Regex::new(r#"install\(TARGETS ([A-Za-z0-9_]+) RUNTIME DESTINATION"#)?;
    let installed = regex
        .captures_iter(cmake)
        .map(|captures| captures[1].to_owned())
        .collect::<BTreeSet<_>>();
    for expected in EXPECTED_WEBP_TOOLS {
        if !installed.contains(*expected) {
            bail!("missing expected installed binary `{expected}` in original/CMakeLists.txt");
        }
    }
    Ok(EXPECTED_WEBP_TOOLS
        .iter()
        .map(|value| (*value).to_owned())
        .collect())
}

fn parse_manpages(cmake: &str, man_makefile: &str) -> Result<Vec<String>> {
    let page_regex = Regex::new(r#"[a-z0-9_]+\.1"#)?;
    let cmake_pages = page_regex
        .find_iter(cmake)
        .map(|capture| capture.as_str().to_owned())
        .collect::<BTreeSet<_>>();
    let makefile_pages = page_regex
        .find_iter(man_makefile)
        .map(|capture| capture.as_str().to_owned())
        .collect::<BTreeSet<_>>();

    for expected in EXPECTED_MANPAGES {
        if !cmake_pages.contains(*expected) {
            bail!("missing expected manpage `{expected}` in original/CMakeLists.txt");
        }
        if !makefile_pages.contains(*expected) {
            bail!("missing expected manpage `{expected}` in original/man/Makefile.am");
        }
    }

    Ok(EXPECTED_MANPAGES
        .iter()
        .map(|value| (*value).to_owned())
        .collect())
}

fn to_installed_header(path: &str) -> Result<String> {
    if let Some(name) = path.strip_prefix("src/webp/") {
        return Ok(format!("include/webp/{name}"));
    }
    if let Some(name) = path.strip_prefix("sharpyuv/") {
        return Ok(format!("include/webp/sharpyuv/{name}"));
    }
    bail!("unexpected public header path `{path}`")
}

fn require_exact(label: &str, actual: &[String], expected: &[&str]) -> Result<Vec<String>> {
    let expected_vec = expected
        .iter()
        .map(|value| (*value).to_owned())
        .collect::<Vec<_>>();
    if actual != expected_vec {
        bail!(
            "{label} mismatch: expected {:?}, found {:?}",
            expected_vec,
            actual
        );
    }
    Ok(expected_vec)
}

fn require_set(label: &str, actual: &[String], expected: &[&str]) -> Result<()> {
    let actual = actual.iter().map(String::as_str).collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    if actual != expected {
        bail!(
            "{label} mismatch: expected {:?}, found {:?}",
            expected,
            actual
        );
    }
    Ok(())
}
