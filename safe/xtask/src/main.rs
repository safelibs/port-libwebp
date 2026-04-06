mod baseline;
mod link;
mod package;
mod tools;
mod verify;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use verify::{
    CSmokeArgs, VerifyNeededArgs, VerifySonamesArgs, VerifySymbolSubsetArgs, VerifySymbolsArgs,
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
    CSmoke(CSmokeArgs),
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
        Command::CSmoke(args) => verify::c_smoke(&args),
    }
}
