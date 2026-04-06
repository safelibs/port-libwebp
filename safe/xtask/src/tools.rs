use anyhow::{bail, Context, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Command;

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
