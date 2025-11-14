#!/usr/bin/env bash
set -euo pipefail

# Build the converter then run the wrapper to generate canonical/snapshot.json by default
OUT="canonical/snapshot.json"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --output)
      OUT="$2"
      shift 2
      ;;
    --help|-h)
      echo "Usage: $0 [--output PATH]"
      exit 0
      ;;
    *)
      echo "Unknown arg: $1"
      exit 1
      ;;
  esac
done

mkdir -p canonical
cd tools/linguist_to_snapshot
cargo build --release
cd - >/dev/null

# Run wrapper which will either call built binary or `cargo run` for the converter
cargo run --manifest-path tools/generate_snapshot_job/Cargo.toml -- --output "$OUT"

echo "Wrote snapshot to $OUT"
