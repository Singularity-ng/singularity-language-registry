# Singularity Composite GitHub Actions

This repository contains reusable composite actions for Singularity projects.

## Available Actions

### `setup-nix-rust`
Sets up Nix development environment with all caching layers for Rust projects.

**Features:**
- Installs Nix with flakes enabled
- Configures Magic Nix Cache
- Optionally configures FlakeHub Cache
- Optionally configures Cachix
- Sets up Rust toolchain from flake

**Usage:**
```yaml
- uses: Singularity-ng/github-actions/setup-nix-rust@v1
  with:
    cachix-name: singularity  # optional
    enable-flakehub: true     # optional
```

### `generate-release-reports`
Generates comprehensive quality reports for releases.

**Features:**
- Clippy report (zero warnings validation)
- Security audit (cargo-audit + cargo-deny)
- SBOM generation
- Test coverage report
- Build information
- Dependency status report
- Changelog extraction
- Release summary

**Usage:**
```yaml
- uses: Singularity-ng/github-actions/generate-release-reports@v1
  with:
    version: "1.0.0"
```

### `build-crate-package`
Builds a .crate package with installation instructions.

**Features:**
- Builds .crate file
- Lists package contents
- Generates installation guide

**Usage:**
```yaml
- uses: Singularity-ng/github-actions/build-crate-package@v1
  with:
    version: "1.0.0"
```

## Versioning

We use semantic versioning with Git tags:

- **`@main`** - Latest development version (may be unstable)
- **`@v1`** - Latest stable v1.x.x release
- **`@v1.2.3`** - Specific version

**Recommendation**: Use `@v1` in production to get bug fixes while avoiding breaking changes.

## Inputs

See each action's `action.yml` for detailed input documentation.

## Examples

### Full Release Workflow
```yaml
steps:
  - uses: actions/checkout@v4

  - uses: Singularity-ng/github-actions/setup-nix-rust@v1
    with:
      cachix-name: singularity

  - uses: Singularity-ng/github-actions/generate-release-reports@v1
    with:
      version: ${{ needs.validate.outputs.version }}

  - uses: Singularity-ng/github-actions/build-crate-package@v1
    with:
      version: ${{ needs.validate.outputs.version }}
```

## Contributing

To update these actions:

1. Make changes in a branch
2. Test in a project using `@branch-name`
3. Open PR
4. After merge, tag a new version
5. Update projects to use new version

## Questions?

See the organization's SUPPORT.md for help.
