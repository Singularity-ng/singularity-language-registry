# Singularity Organization Defaults

This repository provides default files and workflow templates for all repositories in the Singularity organization.

## What's Included

- **Workflow Templates**: Pre-configured GitHub Actions workflows that appear when creating new workflows
- **Security Policy**: Organization-wide security guidelines
- **Support Information**: How to get help with Singularity projects

## Workflow Templates

### Rust + Nix CI
Standard CI pipeline for Rust projects using Nix flake for reproducible builds.

Features:
- Nix flake checks
- Zero warnings tolerance (Clippy pedantic + nursery)
- Multi-platform testing (Linux, macOS)
- 4-layer caching (GitHub + Magic Nix + FlakeHub + Cachix)

### Rust + Nix Release
Automated release workflow for Rust crates.

Features:
- Comprehensive quality reports (Clippy, security, SBOM, coverage)
- Crate package generation
- GitHub Release creation
- Platform binaries (Linux, macOS, Windows)

## Using Templates

1. Go to any repository in the organization
2. Click "Actions" → "New workflow"
3. Templates will appear under "By Singularity-ng"
4. Click "Set up this workflow"

## Customizing for Your Repo

Templates are starting points. You can customize them for your specific needs:
- Add/remove build steps
- Adjust caching strategy
- Enable/disable specific checks

## Questions?

See SUPPORT.md for how to get help.
