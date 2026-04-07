mod baseline;
mod link;
mod package;
mod tools;
mod verify;

use anyhow::Result;
use clap::{Parser, Subcommand};
use package::{PackageDebArgs, StageDistArgs};
use std::path::PathBuf;
use verify::{
    BuildCTestsArgs, BuildUpstreamFuzzersArgs, BuildUpstreamPublicApiTestArgs,
    BuildUpstreamToolsArgs, CSmokeArgs, RelinkOriginalObjectsArgs, ToolSmokeArgs, UnsafeAuditArgs,
    VerifyDependentsMetadataArgs, VerifyInstallTreeArgs, VerifyNeededArgs, VerifySonamesArgs,
    VerifySymbolSubsetArgs, VerifySymbolsArgs,
};

#[derive(Parser, Debug)]
#[command(about = "Workspace maintenance commands for the safe libwebp port.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    CaptureBaseline {
        #[arg(long)]
        original_dir: PathBuf,
        #[arg(long)]
        out_dir: PathBuf,
    },
    VerifyBaselineManifests {
        #[arg(long)]
        baseline_dir: PathBuf,
    },
    VerifySymbols(VerifySymbolsArgs),
    VerifySymbolSubset(VerifySymbolSubsetArgs),
    VerifySonames(VerifySonamesArgs),
    VerifyNeeded(VerifyNeededArgs),
    VerifyInstallTree(VerifyInstallTreeArgs),
    VerifyDependentsMetadata(VerifyDependentsMetadataArgs),
    BuildCTests(BuildCTestsArgs),
    BuildUpstreamPublicApiTest(BuildUpstreamPublicApiTestArgs),
    RelinkOriginalObjects(RelinkOriginalObjectsArgs),
    CSmoke(CSmokeArgs),
    BuildUpstreamTools(BuildUpstreamToolsArgs),
    BuildUpstreamFuzzers(BuildUpstreamFuzzersArgs),
    ToolSmoke(ToolSmokeArgs),
    UnsafeAudit(UnsafeAuditArgs),
    StageDist(StageDistArgs),
    PackageDeb(PackageDebArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::CaptureBaseline {
            original_dir,
            out_dir,
        } => baseline::capture_baseline(&original_dir, &out_dir),
        Command::VerifyBaselineManifests { baseline_dir } => {
            verify::verify_baseline_manifests(&baseline_dir)
        }
        Command::VerifySymbols(args) => verify::verify_symbols(&args),
        Command::VerifySymbolSubset(args) => verify::verify_symbol_subset(&args),
        Command::VerifySonames(args) => verify::verify_sonames(&args),
        Command::VerifyNeeded(args) => verify::verify_needed(&args),
        Command::VerifyInstallTree(args) => verify::verify_install_tree(&args),
        Command::VerifyDependentsMetadata(args) => verify::verify_dependents_metadata(&args),
        Command::BuildCTests(args) => verify::build_c_tests(&args),
        Command::BuildUpstreamPublicApiTest(args) => verify::build_upstream_public_api_test(&args),
        Command::RelinkOriginalObjects(args) => verify::relink_original_objects(&args),
        Command::CSmoke(args) => verify::c_smoke(&args),
        Command::BuildUpstreamTools(args) => verify::build_upstream_tools(&args),
        Command::BuildUpstreamFuzzers(args) => verify::build_upstream_fuzzers(&args),
        Command::ToolSmoke(args) => verify::tool_smoke(&args),
        Command::UnsafeAudit(args) => verify::unsafe_audit(&args),
        Command::StageDist(args) => package::stage_dist(&args),
        Command::PackageDeb(args) => package::package_deb(&args),
    }
}
