# Singularity Reusable GitHub Workflows

This repository contains reusable GitHub Actions workflows for Singularity projects.

## Available Workflows

### `rust-nix-ci.yml`
Standard CI pipeline for Rust projects using Nix flake.

**Features:**
- Nix flake checks (build, test, clippy, fmt, audit, doc)
- Multi-platform testing (Linux, macOS)
- 4-layer caching (GitHub + Magic Nix + FlakeHub + Cachix)
- Zero warnings tolerance
- Optional coverage reporting

**Usage:**
```yaml
jobs:
  ci:
    uses: Singularity-ng/github-workflows/.github/workflows/rust-nix-ci.yml@v1
    with:
      enable-coverage: true
      rust-version: stable
```

### `rust-nix-release.yml`
Automated release workflow for Rust crates.

**Features:**
- Version validation (git tag vs Cargo.toml)
- Comprehensive quality reports
- Crate package generation
- GitHub Release creation
- Platform binaries (optional)
- Optional crates.io publishing

**Usage:**
```yaml
jobs:
  release:
    uses: Singularity-ng/github-workflows/.github/workflows/rust-nix-release.yml@v1
    with:
      enable-crate-publish: false
      enable-platform-binaries: true
      enable-release-reports: true
```

### `security-audit.yml`
Security scanning and dependency checking.

**Features:**
- cargo-audit vulnerability scanning
- cargo-deny license/source checking
- Dependency review
- SBOM generation

**Usage:**
```yaml
jobs:
  security:
    uses: Singularity-ng/github-workflows/.github/workflows/security-audit.yml@v1
```

## Versioning

We use semantic versioning with Git tags:

- **`@main`** - Latest development version (may be unstable)
- **`@v1`** - Latest stable v1.x.x release
- **`@v1.2.3`** - Specific version

**Recommendation**: Use `@v1` in production to get bug fixes while avoiding breaking changes.

## Inputs

See each workflow file for detailed input documentation.

## Secrets

Most workflows require standard GitHub secrets:
- `GITHUB_TOKEN` - Automatically provided by GitHub
- `CRATES_TOKEN` - Required only if publishing to crates.io (set in repository settings)

## Examples

### Basic CI
```yaml
name: CI
on: [push, pull_request]

jobs:
  ci:
    uses: Singularity-ng/github-workflows/.github/workflows/rust-nix-ci.yml@v1
```

### CI with Custom Settings
```yaml
name: CI
on: [push, pull_request]

jobs:
  ci:
    uses: Singularity-ng/github-workflows/.github/workflows/rust-nix-ci.yml@v1
    with:
      enable-coverage: true
      enable-benchmarks: false
      rust-version: nightly
```

### Release Workflow
```yaml
name: Release
on:
  push:
    tags: ['v[0-9]+.*']

permissions:
  contents: write

jobs:
  release:
    uses: Singularity-ng/github-workflows/.github/workflows/rust-nix-release.yml@v1
    with:
      enable-crate-publish: false  # Proprietary software
      enable-platform-binaries: true
      enable-release-reports: true
```

## Contributing

To update these workflows:

1. Make changes in a branch
2. Test in a project using `@branch-name`
3. Open PR
4. After merge, tag a new version
5. Update projects to use new version

## Questions?

See the organization's SUPPORT.md for help.
