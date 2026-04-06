use crate::link::{
    infer_library_name, inspect_shared_library, RelinkManifest, LIBRARIES, RELINK_TARGETS,
};
use crate::package::{
    InstallSurface, EXPECTED_CMAKE_FILES, EXPECTED_HEADERS, EXPECTED_MANPAGES, EXPECTED_PACKAGES,
    EXPECTED_PKGCONFIG_FILES, EXPECTED_WEBP_MANPAGE_GLOBS, EXPECTED_WEBP_TOOLS,
};
use crate::tools::{nonempty_lines, read_json, sort_dedup};
use anyhow::{bail, Context, Result};
use clap::Args;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Args, Clone, Debug)]
pub struct VerifyNeededArgs {
    #[arg(long)]
    pub baseline_dir: PathBuf,
    #[arg(long = "library", value_name = "PATH")]
    pub libraries: Vec<PathBuf>,
    #[arg(long, value_name = "DIR")]
    pub library_dir: Option<PathBuf>,
}

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
    assert_library_keys("needed", expected.keys().cloned().collect::<Vec<_>>())?;

    let libraries = gather_libraries(args)?;
    let actual_keys = libraries.keys().cloned().collect::<Vec<_>>();
    assert_library_keys("actual libraries", actual_keys)?;

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

fn gather_libraries(args: &VerifyNeededArgs) -> Result<BTreeMap<String, PathBuf>> {
    if args.libraries.is_empty() && args.library_dir.is_none() {
        bail!("provide at least one --library or a --library-dir for verify-needed");
    }

    let mut libraries = BTreeMap::new();
    for path in &args.libraries {
        let name = infer_library_name(path)
            .with_context(|| format!("could not infer library name from {}", path.display()))?;
        libraries.insert(name.to_owned(), path.clone());
    }

    if let Some(directory) = &args.library_dir {
        for entry in WalkDir::new(directory).follow_links(false) {
            let entry = entry?;
            if !entry.file_type().is_file() && !entry.file_type().is_symlink() {
                continue;
            }
            let path = entry.into_path();
            if let Some(name) = infer_library_name(&path) {
                libraries.entry(name.to_owned()).or_insert(path);
            }
        }
    }

    Ok(libraries)
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
