use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
struct Args {
    /// Output path for generated snapshot
    #[arg(long, default_value = "canonical/snapshot.json")]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let out = args.output;

    // For now: run the linguist_to_snapshot tool on a vendored languages.yml
    // (this will be called from Renovate or CI). Keep logic simple so Renovate
    // can run it as a cargo run --bin generate_snapshot_job -- --output ...
    let input = PathBuf::from(".github/linguist/languages.yml");
    if !input.exists() {
        anyhow::bail!("languages.yml not found at {}", input.display());
    }

    // Try a few common places for the built converter binary. If not found,
    // fall back to running it via `cargo run` so Renovate/CI can execute this
    // wrapper without requiring a pre-built binary.
    let candidates = [
        PathBuf::from("./tools/linguist_to_snapshot/target/release/linguist_to_snapshot"),
        PathBuf::from("./target/release/linguist_to_snapshot"),
    ];

    let mut ran = false;
    for cand in &candidates {
        if cand.exists() {
            let status = Command::new(cand)
                .arg("--input")
                .arg(&input)
                .arg("--output")
                .arg(&out)
                .status()?;
            if !status.success() {
                anyhow::bail!("linguist_to_snapshot failed at {}", cand.display());
            }
            ran = true;
            break;
        }
    }

    if !ran {
        // Fall back to invoking `cargo run` for the converter. Use a
        // manifest-path so this can be run from the workspace root.
        let status = Command::new("cargo")
            .arg("run")
            .arg("--release")
            .arg("--manifest-path")
            .arg("tools/linguist_to_snapshot/Cargo.toml")
            .arg("--")
            .arg("--input")
            .arg(&input)
            .arg("--output")
            .arg(&out)
            .status()?;
        if !status.success() {
            anyhow::bail!("cargo run for linguist_to_snapshot failed");
        }
    }

    println!("Wrote snapshot to {}", out.display());
    Ok(())
}
