# Tools

This folder contains small helper tools used to generate and publish the canonical
Linguist-derived snapshot of languages for the registry.

- `linguist_to_snapshot` - a Rust CLI that converts a GitHub Linguist `languages.yml` (or JSON) into the registry's snapshot JSON format (an array of snapshot objects).

- `generate_snapshot_job` - a thin wrapper intended for CI or Renovate to run. It will call the converter and write the result to `canonical/snapshot.json` (or another output path you specify). The wrapper will try to find a built converter binary in common locations and will fall back to invoking `cargo run --manifest-path tools/linguist_to_snapshot/Cargo.toml` if needed.

Usage examples

Build and run the converter manually:

```bash
cd tools/linguist_to_snapshot
cargo build --release
./target/release/linguist_to_snapshot --input path/to/languages.yml --output canonical/snapshot.json
```

Run the wrapper (from the repo root):

```bash
# This will try to run a built binary, or fall back to `cargo run` for the converter
cargo run --manifest-path tools/generate_snapshot_job/Cargo.toml -- --output canonical/snapshot.json
```

CI notes

- The wrapper is safe for Renovate or GitHub Actions: it does not require a pre-built binary, and will invoke the converter via `cargo run` if necessary.
- If you prefer, CI can build the converter and call the binary directly for slightly faster runs.
