Snapshot & CI workflow

Purpose

This document describes the canonical snapshot workflow for the `singularity-language-registry` crate. The goal:

- Keep the canonical language list generated from GitHub Linguist (or another upstream source) outside of the Rust source file.
- Make releases fail loudly if the canonical snapshot is missing or malformed.
- Let Renovate (or other automation) open PRs that update the Linguist source and let CI validate and test the generated snapshot.

What we added

- `tools/linguist_to_snapshot` — a small Rust CLI that converts a Linguist YAML/JSON (languages.yml) to the registry snapshot JSON expected by the crate.
- `tests/fixtures/builtin_snapshot.json` — a test fixture used only for unit tests; local devs can run tests without setting env vars.
- A GitHub Actions workflow `.github/workflows/validate-snapshot.yml` that runs on PRs and:
  - Builds the conversion tool,
  - If a `languages.yml` is present under `.github/linguist/` in the PR, runs the tool and produces `target/generated_snapshot.json`, and
  - Runs `cargo test -p singularity-language-registry` with `SINGULARITY_LANGUAGE_SNAPSHOT` pointing to the generated snapshot.

Behavior rules (recommended)

- Release and CI builds MUST provide `SINGULARITY_LANGUAGE_SNAPSHOT` pointing to a validated snapshot JSON (typically produced by Renovate or a scheduled generator). The crate will panic/init-fail if the env var is absent or the file cannot be parsed.
- Unit tests use the committed fixture so developers don't need the env var locally.

How to run locally

Build the converter and generate a snapshot (example):

```bash
# build
cd tools/linguist_to_snapshot
cargo build --release

# generate snapshot
./target/release/linguist_to_snapshot --input path/to/languages.yml --output /tmp/snapshot.json

# run tests using the snapshot
export SINGULARITY_LANGUAGE_SNAPSHOT=/tmp/snapshot.json
cd -
cargo test -p singularity-language-registry
```

If you don't set `SINGULARITY_LANGUAGE_SNAPSHOT`, tests will run against the built-in test fixture (convenient for development).

Renovate/automation guidance

Renovate (or any scheduled automation) should:

1. Periodically fetch/refresh GitHub Linguist's language definitions (or your curated source).
2. Convert them to a snapshot JSON using the `linguist_to_snapshot` tool (run in CI or locally). Example script (pseudocode):

```bash
# fetch linguist repo or languages.yml
git clone https://github.com/github/linguist /tmp/linguist
# or update languages.yml in-place

# generate snapshot
/path/to/linguist_to_snapshot --input /tmp/linguist/lib/linguist/languages.yml --output canonical/snapshot.json

# create a PR that contains the generated languages.yml (or the snapshot) — whichever workflow you prefer
```

Renovate can be configured to open a PR containing `.github/linguist/languages.yml` or the produced `canonical/snapshot.json`. The CI workflow in this repo will:

- If PR contains `.github/linguist/languages.yml`, generate a snapshot and use it for testing (failing the PR if generation/parsing fails).
- If PR doesn't include a snapshot, CI still runs tests (using the fixture) — but we recommend PRs from Renovate include the linguist file to validate snapshot changes.

Example Renovate snippet (illustrative)

Renovate can be extended with custom scripts or jobs. A minimal Renovate rule might look like a scheduled job that runs your snapshot generator and opens a PR with `.github/linguist/languages.yml` updated. Implementation depends on your Renovate setup and where you run the fetch/convert step.

Troubleshooting

- If CI fails because `SINGULARITY_LANGUAGE_SNAPSHOT` isn't set in a release build, ensure your release pipeline sets it to the latest validated snapshot.
- If the converter fails to parse languages.yml, run it locally and inspect the YAML for unexpected fields. The converter is intentionally lenient but expects a mapping from language id -> properties.

Next steps you may want me to implement

- A CI job that publishes a validated snapshot to `canonical/snapshot.json` on merge (or uploads it as an artifact).
- A Renovate example script that fetches Linguist and opens PRs with `languages.yml` or the generated snapshot.
- Move the `tools/linguist_to_snapshot` binary into a workspace member and add `just` tasks for convenience.

If you want me to implement any of the next steps, tell me which and I'll proceed.
