use anyhow::{bail, Context, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

pub fn capture_output(command: &mut Command) -> Result<String> {
    let debug = format_command(command);
    let output = command
        .output()
        .with_context(|| format!("failed to run `{debug}`"))?;
    if !output.status.success() {
        bail!(
            "command `{debug}` failed with status {}:\nstdout:\n{}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    String::from_utf8(output.stdout).with_context(|| format!("non-utf8 stdout from `{debug}`"))
}

pub fn run(command: &mut Command) -> Result<()> {
    let debug = format_command(command);
    let output = command
        .output()
        .with_context(|| format!("failed to run `{debug}`"))?;
    if !output.status.success() {
        bail!(
            "command `{debug}` failed with status {}:\nstdout:\n{}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

pub fn read_json<T>(path: &Path) -> Result<T>
where
    T: DeserializeOwned,
{
    let contents =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_str(&contents)
        .with_context(|| format!("failed to parse JSON from {}", path.display()))
}

pub fn write_json<T>(path: &Path, value: &T) -> Result<()>
where
    T: Serialize,
{
    let contents = serde_json::to_string_pretty(value)?;
    fs::write(path, format!("{contents}\n"))
        .with_context(|| format!("failed to write {}", path.display()))
}

pub fn write_text(path: &Path, contents: &str) -> Result<()> {
    fs::write(path, contents).with_context(|| format!("failed to write {}", path.display()))
}

pub fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask manifest should live under the workspace root")
        .to_path_buf()
}

pub fn repo_root() -> Result<PathBuf> {
    workspace_root()
        .parent()
        .map(Path::to_path_buf)
        .context("workspace root should live under the repository root")
}

pub fn reset_dir(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(path).with_context(|| format!("failed to remove {}", path.display()))?;
    }
    fs::create_dir_all(path).with_context(|| format!("failed to create {}", path.display()))
}

pub fn copy_dir_contents(src: &Path, dst: &Path) -> Result<()> {
    for entry in WalkDir::new(src) {
        let entry = entry?;
        let path = entry.path();
        let relative = path
            .strip_prefix(src)
            .with_context(|| format!("failed to compute relative path for {}", path.display()))?;
        let target = dst.join(relative);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)
                .with_context(|| format!("failed to create {}", target.display()))?;
            continue;
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        fs::copy(path, &target).with_context(|| {
            format!("failed to copy {} to {}", path.display(), target.display())
        })?;
    }
    Ok(())
}

pub fn nonempty_lines(contents: &str) -> Vec<String> {
    contents
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

pub fn sort_dedup(values: &mut Vec<String>) {
    values.sort();
    values.dedup();
}

fn format_command(command: &Command) -> String {
    let mut parts = Vec::new();
    parts.push(os_str_lossy(command.get_program()));
    parts.extend(command.get_args().map(os_str_lossy));
    parts.join(" ")
}

fn os_str_lossy(value: &OsStr) -> String {
    value.to_string_lossy().into_owned()
}
