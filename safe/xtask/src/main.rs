mod baseline;
mod link;
mod package;
mod tools;
mod verify;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use verify::VerifyNeededArgs;

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
    VerifyNeeded(VerifyNeededArgs),
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
        Command::VerifyNeeded(args) => verify::verify_needed(&args),
    }
}
