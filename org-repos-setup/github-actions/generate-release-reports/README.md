# Generate Release Reports Action

Composite action that generates comprehensive quality reports for Rust project releases.

## Features

- ✅ Clippy report (zero warnings validation)
- ✅ Security audit (cargo-audit + cargo-deny)
- ✅ SBOM (Software Bill of Materials)
- ✅ Test coverage report
- ✅ Build information
- ✅ Dependency status
- ✅ Changelog extraction
- ✅ AI/LLM documentation (if available)
- ✅ Creates both tar.gz and zip archives
- ✅ Organizes reports in subdirectories

## Usage

### Basic
```yaml
- uses: Singularity-ng/github-actions/generate-release-reports@v1
  with:
    version: "1.0.0"
```

### With Nix Already Set Up
```yaml
- uses: Singularity-ng/github-actions/setup-nix-rust@v1

- uses: Singularity-ng/github-actions/generate-release-reports@v1
  with:
    version: ${{ needs.validate.outputs.version }}
```

## Inputs

| Input | Description | Required | Default |
|-------|-------------|----------|---------|
| `version` | Release version number (e.g., 1.0.0) | Yes | - |

## Outputs

The action creates:

```
release-artifacts/
├── CHANGELOG.md (if exists)
├── RELEASE_SUMMARY.md (if generated)
├── ai-docs/
│   └── AGENTS.md (if AGENTS.md.release exists)
└── reports/
    ├── clippy-report.md
    ├── security-audit.md
    ├── sbom.md
    ├── coverage-report.md
    ├── build-info.md (if generated)
    └── dependency-report.md (if generated)

release-reports-v{version}.tar.gz
release-reports-v{version}.zip
```

## How It Works

1. **Checks for justfile**: If your project has a `justfile` with a `release-reports` command, it uses that
2. **Falls back to manual generation**: Otherwise, generates reports directly

This allows projects to customize report generation while providing sensible defaults.

## Using with justfile

If your project has a `justfile` with:

```justfile
release-reports version="dev":
    # Custom report generation logic
```

The action will use that instead of generating reports manually. This gives you full control over:
- Report format
- Additional reports
- Custom tooling

## Requirements

- Repository must have `Cargo.toml`
- Nix flake with Rust development environment
- Optional: `justfile` with `release-reports` command
- Optional: `AGENTS.md.release` for AI documentation
- Optional: `CHANGELOG.md` for changelog

## Example Workflow

```yaml
name: Release

on:
  push:
    tags: ['v*']

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Get version
        id: version
        run: echo "version=${GITHUB_REF#refs/tags/v}" >> $GITHUB_OUTPUT

      - uses: Singularity-ng/github-actions/generate-release-reports@v1
        with:
          version: ${{ steps.version.outputs.version }}

      - uses: actions/upload-artifact@v4
        with:
          name: release-reports
          path: |
            release-artifacts/
            release-reports-*.tar.gz
            release-reports-*.zip
```

## Customization

### Custom Clippy Configuration
The action respects your project's clippy settings. To customize:

```toml
# .cargo/config.toml or Cargo.toml
[lints.clippy]
pedantic = "warn"
nursery = "warn"
```

### Custom Coverage Tool
If `cargo-tarpaulin` is not available in your Nix environment, coverage will be skipped gracefully.

### Additional Reports
Use a `justfile` to add custom reports:

```justfile
release-reports version:
    # Standard reports
    cargo clippy > clippy.md
    # Your custom reports
    my-custom-tool > custom-report.md
```

## Troubleshooting

### "No such file or directory" errors
- Ensure Nix is installed (action handles this automatically)
- Check that `Cargo.toml` exists in repository root

### Coverage fails
- Verify `cargo-tarpaulin` is in your Nix devShell
- Or accept that coverage will be marked as "not available"

### Reports missing
- Check GitHub Actions logs for errors
- Verify all required tools are in Nix environment

## License

Same as parent repository.
