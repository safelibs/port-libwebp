use crate::link::{find_library_artifact, inspect_shared_library, LIBRARIES};
use crate::tools::{capture_output, nonempty_lines, repo_root, reset_dir, run, workspace_root};
use anyhow::{bail, Context, Result};
use clap::Args;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::Command;

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

pub const EXPECTED_WEBP_INSTALL_PATHS: &[&str] = &[
    "usr/bin/cwebp",
    "usr/bin/dwebp",
    "usr/bin/gif2webp",
    "usr/bin/img2webp",
    "usr/bin/vwebp",
    "usr/bin/webpinfo",
    "usr/bin/webpmux",
    "usr/bin/anim_diff",
    "usr/bin/anim_dump",
];

pub const EXPECTED_WEBP_MANPAGE_PATHS: &[&str] = &[
    "usr/share/man/man1/cwebp.1",
    "usr/share/man/man1/dwebp.1",
    "usr/share/man/man1/gif2webp.1",
    "usr/share/man/man1/img2webp.1",
    "usr/share/man/man1/vwebp.1",
    "usr/share/man/man1/webpinfo.1",
    "usr/share/man/man1/webpmux.1",
];

const EXPECTED_INSTALL_FILE_LINES: &[(&str, &[&str])] = &[
    (
        "libwebp-dev.install",
        &[
            "usr/include/webp/decode.h",
            "usr/include/webp/demux.h",
            "usr/include/webp/encode.h",
            "usr/include/webp/mux.h",
            "usr/include/webp/mux_types.h",
            "usr/include/webp/types.h",
            "usr/lib/*/libwebp.so",
            "usr/lib/*/libwebpdecoder.so",
            "usr/lib/*/libwebpdemux.so",
            "usr/lib/*/libwebpmux.so",
            "usr/lib/*/pkgconfig/libwebp.pc",
            "usr/lib/*/pkgconfig/libwebpdecoder.pc",
            "usr/lib/*/pkgconfig/libwebpdemux.pc",
            "usr/lib/*/pkgconfig/libwebpmux.pc",
            "usr/lib/*/cmake/WebP/WebPConfig.cmake",
            "usr/lib/*/cmake/WebP/WebPConfigVersion.cmake",
            "usr/lib/*/cmake/WebP/WebPTargets.cmake",
        ],
    ),
    ("libwebp7.install", &["usr/lib/*/libwebp.so.7"]),
    (
        "libwebpdecoder3.install",
        &["usr/lib/*/libwebpdecoder.so.3"],
    ),
    ("libwebpdemux2.install", &["usr/lib/*/libwebpdemux.so.2"]),
    ("libwebpmux3.install", &["usr/lib/*/libwebpmux.so.3"]),
    (
        "libsharpyuv-dev.install",
        &[
            "usr/include/webp/sharpyuv/sharpyuv.h",
            "usr/include/webp/sharpyuv/sharpyuv_csp.h",
            "usr/lib/*/pkgconfig/libsharpyuv.pc",
            "usr/lib/*/libsharpyuv.so",
        ],
    ),
    ("libsharpyuv0.install", &["usr/lib/*/libsharpyuv.so.0"]),
    ("webp.install", EXPECTED_WEBP_INSTALL_PATHS),
    ("webp.manpages", EXPECTED_WEBP_MANPAGE_PATHS),
];

const PKGCONFIG_TEMPLATES: &[(&str, &str)] = &[
    ("libwebp.pc.in", "libwebp.pc"),
    ("libwebpdecoder.pc.in", "libwebpdecoder.pc"),
    ("libwebpdemux.pc.in", "libwebpdemux.pc"),
    ("libwebpmux.pc.in", "libwebpmux.pc"),
    ("libsharpyuv.pc.in", "libsharpyuv.pc"),
];

const PUBLIC_HEADERS: &[(&str, &str)] = &[
    ("webp/decode.h", "webp/decode.h"),
    ("webp/demux.h", "webp/demux.h"),
    ("webp/encode.h", "webp/encode.h"),
    ("webp/mux.h", "webp/mux.h"),
    ("webp/mux_types.h", "webp/mux_types.h"),
    ("webp/types.h", "webp/types.h"),
    ("webp/sharpyuv/sharpyuv.h", "webp/sharpyuv/sharpyuv.h"),
    (
        "webp/sharpyuv/sharpyuv_csp.h",
        "webp/sharpyuv/sharpyuv_csp.h",
    ),
];

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

#[derive(Args, Clone, Debug)]
pub struct StageDistArgs {
    #[arg(long, value_name = "DIR")]
    pub install_root: Option<PathBuf>,
    #[arg(long, value_name = "TRIPLET")]
    pub multiarch: Option<String>,
}

#[derive(Args, Clone, Debug)]
pub struct PackageDebArgs {
    #[arg(long, value_name = "DIR")]
    pub output_dir: PathBuf,
}

pub fn stage_dist(args: &StageDistArgs) -> Result<()> {
    verify_safe_debian_metadata()?;

    let root = workspace_root();
    let install_root = args
        .install_root
        .clone()
        .unwrap_or_else(|| root.join("dist"));
    let multiarch = match &args.multiarch {
        Some(value) => value.clone(),
        None => detect_multiarch()?,
    };
    let search_dir = root.join("target/release");
    let usr_root = install_root.join("usr");
    let include_dir = usr_root.join("include");
    let lib_dir = usr_root.join("lib").join(&multiarch);
    let bin_dir = usr_root.join("bin");
    let man_dir = usr_root.join("share/man/man1");
    let upstream_version = upstream_library_version()?;

    build_workspace_libraries(&root)?;
    reset_dir(&install_root)?;
    fs::create_dir_all(&include_dir)
        .with_context(|| format!("failed to create {}", include_dir.display()))?;
    fs::create_dir_all(&lib_dir)
        .with_context(|| format!("failed to create {}", lib_dir.display()))?;
    fs::create_dir_all(&bin_dir)
        .with_context(|| format!("failed to create {}", bin_dir.display()))?;
    fs::create_dir_all(&man_dir)
        .with_context(|| format!("failed to create {}", man_dir.display()))?;

    stage_public_headers(&root.join("include"), &include_dir)?;
    for library in LIBRARIES {
        stage_library_artifact(&search_dir, library, &lib_dir)?;
    }
    render_pkg_config_files(
        &root.join("pkgconfig"),
        &lib_dir,
        &multiarch,
        &upstream_version,
    )?;
    render_cmake_files(&root.join("cmake"), &lib_dir, &multiarch, &upstream_version)?;

    let requested = EXPECTED_WEBP_TOOLS
        .iter()
        .map(|tool| (*tool).to_owned())
        .collect::<Vec<_>>();
    build_upstream_tools_into(&requested, &include_dir, &lib_dir, &bin_dir)?;
    stage_manpages(&root.join("man"), &man_dir)?;
    Ok(())
}

pub fn package_deb(args: &PackageDebArgs) -> Result<()> {
    verify_safe_debian_metadata()?;

    let root = workspace_root();
    let output_dir = &args.output_dir;
    let parent = root
        .parent()
        .context("workspace root should live under the repository root")?;
    let version = package_version(&root.join("debian/changelog"))?;

    fs::create_dir_all(output_dir)
        .with_context(|| format!("failed to create {}", output_dir.display()))?;
    clear_matching_packages(output_dir)?;
    build_workspace_libraries(&root)?;
    build_debian_packages_in_docker(&root)?;

    for package in EXPECTED_PACKAGES {
        let source = find_built_package(parent, package, &version)?;
        let destination = output_dir.join(
            source
                .file_name()
                .with_context(|| format!("missing filename for {}", source.display()))?,
        );
        fs::copy(&source, &destination).with_context(|| {
            format!(
                "failed to copy built package {} to {}",
                source.display(),
                destination.display()
            )
        })?;
    }
    Ok(())
}

pub fn verify_safe_debian_metadata() -> Result<()> {
    let debian_dir = workspace_root().join("debian");
    let package_names = parse_control_packages(&debian_dir.join("control"))?;
    require_exact("package names", &package_names, EXPECTED_PACKAGES)?;

    for (file_name, expected_lines) in EXPECTED_INSTALL_FILE_LINES {
        let path = debian_dir.join(file_name);
        let lines = nonempty_lines(
            &fs::read_to_string(&path)
                .with_context(|| format!("failed to read {}", path.display()))?,
        );
        require_exact(file_name, &lines, expected_lines)?;
    }
    Ok(())
}

pub fn select_upstream_tools(tools: &[String]) -> Result<Vec<String>> {
    let allowed = EXPECTED_WEBP_TOOLS.iter().copied().collect::<BTreeSet<_>>();
    let mut selected = if tools.is_empty() {
        EXPECTED_WEBP_TOOLS
            .iter()
            .map(|tool| (*tool).to_owned())
            .collect::<Vec<_>>()
    } else {
        tools
            .iter()
            .map(|tool| {
                if !allowed.contains(tool.as_str()) {
                    bail!("unknown upstream tool `{tool}`");
                }
                Ok(tool.clone())
            })
            .collect::<Result<Vec<_>>>()?
    };
    selected.sort();
    selected.dedup();
    Ok(selected)
}

pub fn build_upstream_tools_into(
    tools: &[String],
    include_dir: &Path,
    lib_dir: &Path,
    bin_dir: &Path,
) -> Result<()> {
    fs::create_dir_all(bin_dir)
        .with_context(|| format!("failed to create {}", bin_dir.display()))?;
    for tool in select_upstream_tools(tools)? {
        build_single_upstream_tool(&tool, include_dir, lib_dir, &bin_dir.join(&tool))?;
    }
    Ok(())
}

pub fn stage_library_artifact(search_dir: &Path, logical_name: &str, lib_dir: &Path) -> Result<()> {
    let artifact = find_library_artifact(search_dir, logical_name)?;
    let info = inspect_shared_library(&artifact)?;
    let artifact_name = artifact
        .file_name()
        .with_context(|| format!("missing file name for {}", artifact.display()))?;
    let linker_name = format!("{logical_name}.so");
    let staged_artifact = lib_dir.join(artifact_name);

    fs::copy(&artifact, &staged_artifact).with_context(|| {
        format!(
            "failed to copy shared library {} to {}",
            artifact.display(),
            staged_artifact.display()
        )
    })?;

    let soname_path = lib_dir.join(&info.soname);
    if soname_path != staged_artifact {
        fs::copy(&artifact, &soname_path).with_context(|| {
            format!(
                "failed to stage SONAME copy {} to {}",
                artifact.display(),
                soname_path.display()
            )
        })?;
    }

    let linker_path = lib_dir.join(linker_name);
    if linker_path != staged_artifact && linker_path != soname_path {
        fs::copy(&artifact, &linker_path).with_context(|| {
            format!(
                "failed to stage linker-name copy {} to {}",
                artifact.display(),
                linker_path.display()
            )
        })?;
    }

    Ok(())
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

pub fn build_workspace_libraries(root: &Path) -> Result<()> {
    for package in [
        "libsharpyuv",
        "libwebpdecoder",
        "libwebp",
        "libwebpdemux",
        "libwebpmux",
    ] {
        let mut cargo = Command::new("cargo");
        cargo
            .arg("build")
            .arg("--manifest-path")
            .arg(root.join("Cargo.toml"))
            .arg("--release")
            .arg("-p")
            .arg(package);
        run(&mut cargo)?;
    }
    Ok(())
}

fn build_debian_packages_in_docker(root: &Path) -> Result<()> {
    let dockerfile_dir =
        tempfile::TempDir::new().context("failed to create temporary Dockerfile directory")?;
    let dockerfile_path = dockerfile_dir.path().join("Dockerfile");
    let repo_root = root
        .parent()
        .context("workspace root should live under the repository root")?;
    let metadata = fs::metadata(repo_root)
        .with_context(|| format!("failed to stat {}", repo_root.display()))?;
    let image_tag = "libwebp-safe-deb:ubuntu24.04";

    fs::write(&dockerfile_path, deb_build_dockerfile())
        .with_context(|| format!("failed to write {}", dockerfile_path.display()))?;

    let mut build_image = Command::new("docker");
    build_image
        .arg("build")
        .arg("-t")
        .arg(image_tag)
        .arg("-f")
        .arg(&dockerfile_path)
        .arg(dockerfile_dir.path());
    run(&mut build_image)?;

    let mut build = Command::new("docker");
    build
        .arg("run")
        .arg("--rm")
        .arg("--user")
        .arg(format!("{}:{}", metadata.uid(), metadata.gid()))
        .arg("-e")
        .arg("HOME=/tmp/libwebp-home")
        .arg("-e")
        .arg("PATH=/usr/local/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin")
        .arg("-v")
        .arg(format!("{}:/work", repo_root.display()))
        .arg("-w")
        .arg("/work/safe")
        .arg(image_tag)
        .arg("bash")
        .arg("-lc")
        .arg("dpkg-buildpackage -us -uc -b");
    run(&mut build)
}

fn deb_build_dockerfile() -> &'static str {
    r#"FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
      build-essential \
      ca-certificates \
      cargo \
      curl \
      debhelper \
      dpkg-dev \
      libgif-dev \
      libglut-dev \
      libjpeg-dev \
      libpng-dev \
      libtiff-dev \
      pkg-config \
      rustc \
 && rm -rf /var/lib/apt/lists/*

ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH=/usr/local/cargo/bin:${PATH}

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain stable \
 && chmod -R a+rwX /usr/local/cargo /usr/local/rustup
"#
}

fn stage_public_headers(src_root: &Path, dst_root: &Path) -> Result<()> {
    for (source, dest) in PUBLIC_HEADERS {
        let source_path = src_root.join(source);
        let dest_path = dst_root.join(dest);
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        fs::copy(&source_path, &dest_path).with_context(|| {
            format!(
                "failed to copy public header {} to {}",
                source_path.display(),
                dest_path.display()
            )
        })?;
    }
    Ok(())
}

fn stage_manpages(src_root: &Path, dst_root: &Path) -> Result<()> {
    for manpage in EXPECTED_MANPAGES {
        let source = src_root.join(manpage);
        let destination = dst_root.join(manpage);
        fs::copy(&source, &destination).with_context(|| {
            format!(
                "failed to copy manpage {} to {}",
                source.display(),
                destination.display()
            )
        })?;
    }
    Ok(())
}

fn render_pkg_config_files(
    template_dir: &Path,
    lib_dir: &Path,
    multiarch: &str,
    upstream_version: &str,
) -> Result<()> {
    let destination_dir = lib_dir.join("pkgconfig");
    fs::create_dir_all(&destination_dir)
        .with_context(|| format!("failed to create {}", destination_dir.display()))?;
    let libdir = format!("${{exec_prefix}}/lib/{multiarch}");

    for (template_name, output_name) in PKGCONFIG_TEMPLATES {
        let template_path = template_dir.join(template_name);
        let template = fs::read_to_string(&template_path)
            .with_context(|| format!("failed to read {}", template_path.display()))?;
        let rendered = render_template(
            &template,
            &[
                ("@prefix@", "/usr"),
                ("@exec_prefix@", "${prefix}"),
                ("@libdir@", &libdir),
                ("@includedir@", "${prefix}/include"),
                ("@PACKAGE_VERSION@", upstream_version),
                ("@webp_libname_prefix@", ""),
                ("@PTHREAD_CFLAGS@", ""),
                ("@PTHREAD_LIBS@", ""),
            ],
        );
        fs::write(destination_dir.join(output_name), rendered).with_context(|| {
            format!(
                "failed to write {}",
                destination_dir.join(output_name).display()
            )
        })?;
    }
    Ok(())
}

fn render_cmake_files(
    template_dir: &Path,
    lib_dir: &Path,
    multiarch: &str,
    upstream_version: &str,
) -> Result<()> {
    let destination_dir = lib_dir.join("cmake/WebP");
    let template_path = template_dir.join("WebPConfig.cmake.in");
    let template = fs::read_to_string(&template_path)
        .with_context(|| format!("failed to read {}", template_path.display()))?;
    let package_init = cmake_package_init();
    let config = render_template(
        &template,
        &[
            ("@PROJECT_VERSION@", upstream_version),
            ("@PACKAGE_INIT@", &package_init),
            ("@WEBP_USE_THREAD@", "FALSE"),
            ("@PROJECT_NAME@", "WebP"),
            (
                "@PACKAGE_CMAKE_INSTALL_INCLUDEDIR@",
                "${PACKAGE_PREFIX_DIR}/include",
            ),
            (
                "@INSTALLED_LIBRARIES@",
                "webpdecoder;webp;webpdemux;webpmux",
            ),
        ],
    );

    fs::create_dir_all(&destination_dir)
        .with_context(|| format!("failed to create {}", destination_dir.display()))?;
    fs::write(destination_dir.join("WebPConfig.cmake"), config).with_context(|| {
        format!(
            "failed to write {}",
            destination_dir.join("WebPConfig.cmake").display()
        )
    })?;
    fs::write(
        destination_dir.join("WebPConfigVersion.cmake"),
        cmake_version_file(upstream_version),
    )
    .with_context(|| {
        format!(
            "failed to write {}",
            destination_dir.join("WebPConfigVersion.cmake").display()
        )
    })?;
    fs::write(
        destination_dir.join("WebPTargets.cmake"),
        cmake_targets_file(multiarch),
    )
    .with_context(|| {
        format!(
            "failed to write {}",
            destination_dir.join("WebPTargets.cmake").display()
        )
    })?;
    Ok(())
}

fn build_single_upstream_tool(
    tool: &str,
    include_dir: &Path,
    lib_dir: &Path,
    output: &Path,
) -> Result<()> {
    let root = repo_root()?;
    let original = root.join("original");
    let spec = tool_spec(tool)?;
    let pkg_config_args = pkg_config_args(spec.pkg_config_packages)?;

    let mut compile = Command::new("cc");
    compile
        .arg("-std=c11")
        .arg("-O2")
        .arg(format!("-I{}", include_dir.display()))
        .arg(format!("-I{}", include_dir.join("webp").display()))
        .arg(format!("-I{}", original.display()))
        .arg(format!("-I{}", original.join("src").display()))
        .arg(format!("-L{}", lib_dir.display()));

    for define in spec.defines {
        compile.arg(format!("-D{define}"));
    }
    for arg in &pkg_config_args.cflags {
        compile.arg(arg);
    }
    for source in spec.sources {
        compile.arg(original.join(source));
    }
    compile
        .arg("-lwebpmux")
        .arg("-lwebpdemux")
        .arg("-lwebp")
        .arg("-lsharpyuv");
    for library in resolve_link_arguments(spec.extra_libraries)? {
        compile.arg(library);
    }
    for arg in &pkg_config_args.libs {
        compile.arg(arg);
    }
    compile.arg("-o").arg(output);
    run(&mut compile)
}

fn tool_spec(tool: &str) -> Result<ToolSpec> {
    const COMMON_SOURCES: &[&str] = &["examples/example_util.c", "imageio/imageio_util.c"];
    const IMAGE_DEC_SOURCES: &[&str] = &[
        "imageio/image_dec.c",
        "imageio/jpegdec.c",
        "imageio/metadata.c",
        "imageio/pngdec.c",
        "imageio/pnmdec.c",
        "imageio/tiffdec.c",
        "imageio/webpdec.c",
        "imageio/wicdec.c",
    ];
    const IMAGE_ENC_SOURCES: &[&str] = &["imageio/image_enc.c"];
    const ANIM_SOURCES: &[&str] = &["examples/anim_util.c", "examples/gifdec.c"];

    let spec = match tool {
        "cwebp" => ToolSpec {
            sources: concat_sources(&[&["examples/cwebp.c"], COMMON_SOURCES, IMAGE_DEC_SOURCES]),
            defines: &["WEBP_HAVE_PNG=1", "WEBP_HAVE_JPEG=1", "WEBP_HAVE_TIFF=1"],
            pkg_config_packages: &["libpng", "libtiff-4"],
            extra_libraries: &["-ljpeg", "-lm"],
        },
        "dwebp" => ToolSpec {
            sources: concat_sources(&[
                &["examples/dwebp.c"],
                COMMON_SOURCES,
                IMAGE_DEC_SOURCES,
                IMAGE_ENC_SOURCES,
            ]),
            defines: &["WEBP_HAVE_PNG=1", "WEBP_HAVE_JPEG=1", "WEBP_HAVE_TIFF=1"],
            pkg_config_packages: &["libpng", "libtiff-4"],
            extra_libraries: &["-ljpeg", "-lm"],
        },
        "gif2webp" => ToolSpec {
            sources: concat_sources(&[
                &["examples/gif2webp.c", "examples/gifdec.c"],
                COMMON_SOURCES,
            ]),
            defines: &["WEBP_HAVE_GIF=1"],
            pkg_config_packages: &[],
            extra_libraries: &["-lgif", "-lm"],
        },
        "img2webp" => ToolSpec {
            sources: concat_sources(&[&["examples/img2webp.c"], COMMON_SOURCES, IMAGE_DEC_SOURCES]),
            defines: &["WEBP_HAVE_PNG=1", "WEBP_HAVE_JPEG=1", "WEBP_HAVE_TIFF=1"],
            pkg_config_packages: &["libpng", "libtiff-4"],
            extra_libraries: &["-ljpeg", "-lm"],
        },
        "vwebp" => ToolSpec {
            sources: concat_sources(&[&["examples/vwebp.c"], COMMON_SOURCES]),
            defines: &["WEBP_HAVE_GL=1"],
            pkg_config_packages: &[],
            extra_libraries: &["-lglut", "-lGLU", "-lGL", "-lm"],
        },
        "webpinfo" => ToolSpec {
            sources: concat_sources(&[&["examples/webpinfo.c"], COMMON_SOURCES]),
            defines: &[],
            pkg_config_packages: &[],
            extra_libraries: &["-lm"],
        },
        "webpmux" => ToolSpec {
            sources: concat_sources(&[&["examples/webpmux.c"], COMMON_SOURCES]),
            defines: &[],
            pkg_config_packages: &[],
            extra_libraries: &["-lm"],
        },
        "anim_diff" => ToolSpec {
            sources: concat_sources(&[
                &["examples/anim_diff.c"],
                COMMON_SOURCES,
                ANIM_SOURCES,
                IMAGE_DEC_SOURCES,
                IMAGE_ENC_SOURCES,
            ]),
            defines: &[
                "WEBP_HAVE_PNG=1",
                "WEBP_HAVE_JPEG=1",
                "WEBP_HAVE_TIFF=1",
                "WEBP_HAVE_GIF=1",
            ],
            pkg_config_packages: &["libpng", "libtiff-4"],
            extra_libraries: &["-ljpeg", "-lgif", "-lm"],
        },
        "anim_dump" => ToolSpec {
            sources: concat_sources(&[
                &["examples/anim_dump.c"],
                COMMON_SOURCES,
                ANIM_SOURCES,
                IMAGE_DEC_SOURCES,
                IMAGE_ENC_SOURCES,
            ]),
            defines: &[
                "WEBP_HAVE_PNG=1",
                "WEBP_HAVE_JPEG=1",
                "WEBP_HAVE_TIFF=1",
                "WEBP_HAVE_GIF=1",
            ],
            pkg_config_packages: &["libpng", "libtiff-4"],
            extra_libraries: &["-ljpeg", "-lgif", "-lm"],
        },
        other => bail!("unknown upstream tool `{other}`"),
    };
    Ok(spec)
}

fn concat_sources(groups: &[&'static [&'static str]]) -> Vec<&'static str> {
    let mut sources = Vec::new();
    for group in groups {
        sources.extend(group.iter().copied());
    }
    sources
}

fn pkg_config_args(packages: &[&str]) -> Result<PkgConfigArgs> {
    if packages.is_empty() {
        return Ok(PkgConfigArgs {
            cflags: Vec::new(),
            libs: Vec::new(),
        });
    }

    let mut cflags = Command::new("pkg-config");
    cflags.arg("--cflags");
    for package in packages {
        cflags.arg(package);
    }
    let mut libs = Command::new("pkg-config");
    libs.arg("--libs");
    for package in packages {
        libs.arg(package);
    }

    Ok(PkgConfigArgs {
        cflags: capture_output(&mut cflags)?
            .split_whitespace()
            .map(ToOwned::to_owned)
            .collect(),
        libs: capture_output(&mut libs)?
            .split_whitespace()
            .map(ToOwned::to_owned)
            .collect(),
    })
}

fn resolve_link_arguments(libraries: &[&str]) -> Result<Vec<String>> {
    libraries
        .iter()
        .map(|library| resolve_link_argument(library))
        .collect()
}

fn resolve_link_argument(library: &str) -> Result<String> {
    let Some(name) = library.strip_prefix("-l") else {
        return Ok(library.to_owned());
    };
    let linker_name = format!("lib{name}.so");
    let mut cc = Command::new("cc");
    cc.arg(format!("-print-file-name={linker_name}"));
    let candidate = capture_output(&mut cc)?.trim().to_owned();
    if candidate != linker_name && Path::new(&candidate).exists() {
        return Ok(candidate);
    }
    Ok(find_system_link_library(name).unwrap_or_else(|| library.to_owned()))
}

fn find_system_link_library(name: &str) -> Option<String> {
    let output = Command::new("ldconfig").arg("-p").output().ok()?;
    if !output.status.success() {
        return None;
    }

    let prefix = format!("lib{name}.so");
    String::from_utf8(output.stdout)
        .ok()?
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with(&prefix))
        .and_then(|line| line.split_once("=>"))
        .map(|(_, path)| path.trim().to_owned())
}

fn render_template(template: &str, replacements: &[(&str, &str)]) -> String {
    let mut rendered = template.to_owned();
    for (needle, replacement) in replacements {
        rendered = rendered.replace(needle, replacement);
    }
    rendered
}

fn cmake_version_file(version: &str) -> String {
    format!(
        r#"set(PACKAGE_VERSION "{version}")

if(PACKAGE_FIND_VERSION VERSION_GREATER PACKAGE_VERSION)
  set(PACKAGE_VERSION_COMPATIBLE FALSE)
else()
  set(PACKAGE_VERSION_COMPATIBLE TRUE)
  if(PACKAGE_FIND_VERSION VERSION_EQUAL PACKAGE_VERSION)
    set(PACKAGE_VERSION_EXACT TRUE)
  endif()
endif()
"#
    )
}

fn cmake_package_init() -> String {
    r#"get_filename_component(PACKAGE_PREFIX_DIR "${CMAKE_CURRENT_LIST_DIR}/../../../.." ABSOLUTE)

macro(set_and_check _var _file)
  set(${_var} "${_file}")
  if(NOT EXISTS "${_file}")
    message(FATAL_ERROR "File or directory ${_file} referenced by variable ${_var} does not exist!")
  endif()
endmacro()

macro(check_required_components _NAME)
  foreach(comp ${${_NAME}_FIND_COMPONENTS})
    if(NOT ${_NAME}_${comp}_FOUND)
      if(${_NAME}_FIND_REQUIRED_${comp})
        set(${_NAME}_FOUND FALSE)
      endif()
    endif()
  endforeach()
endmacro()
"#
    .to_owned()
}

fn cmake_targets_file(multiarch: &str) -> String {
    r#"get_filename_component(_IMPORT_PREFIX "${CMAKE_CURRENT_LIST_DIR}/../../../.." ABSOLUTE)
set(_WEBP_INCLUDE_DIR "${_IMPORT_PREFIX}/include")
set(_WEBP_LIBRARY_DIR "${_IMPORT_PREFIX}/lib/@MULTIARCH@")

if(NOT TARGET WebP::sharpyuv)
  add_library(WebP::sharpyuv SHARED IMPORTED)
  set_target_properties(WebP::sharpyuv PROPERTIES
    IMPORTED_LOCATION "${_WEBP_LIBRARY_DIR}/libsharpyuv.so.0"
    IMPORTED_SONAME "libsharpyuv.so.0"
    INTERFACE_INCLUDE_DIRECTORIES "${_WEBP_INCLUDE_DIR};${_WEBP_INCLUDE_DIR}/webp")
endif()

if(NOT TARGET WebP::webpdecoder)
  add_library(WebP::webpdecoder SHARED IMPORTED)
  set_target_properties(WebP::webpdecoder PROPERTIES
    IMPORTED_LOCATION "${_WEBP_LIBRARY_DIR}/libwebpdecoder.so.3"
    IMPORTED_SONAME "libwebpdecoder.so.3"
    INTERFACE_INCLUDE_DIRECTORIES "${_WEBP_INCLUDE_DIR}")
endif()

if(NOT TARGET WebP::webp)
  add_library(WebP::webp SHARED IMPORTED)
  set_target_properties(WebP::webp PROPERTIES
    IMPORTED_LOCATION "${_WEBP_LIBRARY_DIR}/libwebp.so.7"
    IMPORTED_SONAME "libwebp.so.7"
    INTERFACE_INCLUDE_DIRECTORIES "${_WEBP_INCLUDE_DIR}"
    INTERFACE_LINK_LIBRARIES "WebP::sharpyuv")
endif()

if(NOT TARGET WebP::webpdemux)
  add_library(WebP::webpdemux SHARED IMPORTED)
  set_target_properties(WebP::webpdemux PROPERTIES
    IMPORTED_LOCATION "${_WEBP_LIBRARY_DIR}/libwebpdemux.so.2"
    IMPORTED_SONAME "libwebpdemux.so.2"
    INTERFACE_INCLUDE_DIRECTORIES "${_WEBP_INCLUDE_DIR}"
    INTERFACE_LINK_LIBRARIES "WebP::webp")
endif()

if(NOT TARGET WebP::libwebpmux)
  add_library(WebP::libwebpmux SHARED IMPORTED)
  set_target_properties(WebP::libwebpmux PROPERTIES
    IMPORTED_LOCATION "${_WEBP_LIBRARY_DIR}/libwebpmux.so.3"
    IMPORTED_SONAME "libwebpmux.so.3"
    INTERFACE_INCLUDE_DIRECTORIES "${_WEBP_INCLUDE_DIR}"
    INTERFACE_LINK_LIBRARIES "WebP::webp")
endif()

unset(_WEBP_LIBRARY_DIR)
unset(_WEBP_INCLUDE_DIR)
unset(_IMPORT_PREFIX)
"#
    .replace("@MULTIARCH@", multiarch)
}

fn detect_multiarch() -> Result<String> {
    let mut command = Command::new("dpkg-architecture");
    command.arg("-qDEB_HOST_MULTIARCH");
    Ok(capture_output(&mut command)?.trim().to_owned())
}

fn upstream_library_version() -> Result<String> {
    let version = package_version(&workspace_root().join("debian/changelog"))?;
    Ok(version
        .split_once('-')
        .map(|(upstream, _)| upstream.to_owned())
        .unwrap_or(version))
}

fn package_version(changelog_path: &Path) -> Result<String> {
    let changelog = fs::read_to_string(changelog_path)
        .with_context(|| format!("failed to read {}", changelog_path.display()))?;
    let first_line = changelog
        .lines()
        .next()
        .context("debian/changelog is empty")?;
    let regex = Regex::new(r"^[^(]+\(([^)]+)\)")?;
    let captures = regex
        .captures(first_line)
        .with_context(|| format!("failed to parse version from `{first_line}`"))?;
    Ok(captures[1].trim().to_owned())
}

fn clear_matching_packages(output_dir: &Path) -> Result<()> {
    for entry in fs::read_dir(output_dir)
        .with_context(|| format!("failed to read {}", output_dir.display()))?
    {
        let path = entry?.path();
        if path.extension().is_some_and(|extension| extension == "deb") {
            fs::remove_file(&path)
                .with_context(|| format!("failed to remove {}", path.display()))?;
        }
    }
    Ok(())
}

fn find_built_package(parent: &Path, package: &str, version: &str) -> Result<PathBuf> {
    let prefix = format!("{package}_{version}_");
    let matches = fs::read_dir(parent)
        .with_context(|| format!("failed to read {}", parent.display()))?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|extension| extension == "deb"))
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(&prefix))
        })
        .collect::<Vec<_>>();

    match matches.as_slice() {
        [path] => Ok(path.clone()),
        [] => bail!("failed to find built package `{package}` for version `{version}`"),
        _ => bail!(
            "found multiple built packages for `{package}` version `{version}` under {}",
            parent.display()
        ),
    }
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

struct ToolSpec {
    sources: Vec<&'static str>,
    defines: &'static [&'static str],
    pkg_config_packages: &'static [&'static str],
    extra_libraries: &'static [&'static str],
}

struct PkgConfigArgs {
    cflags: Vec<String>,
    libs: Vec<String>,
}
