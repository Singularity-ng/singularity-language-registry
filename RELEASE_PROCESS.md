# Release Process

This document describes how to create a new release of Singularity Language Registry, including comprehensive quality reports.

## Overview

Every release includes:
- ✅ **Zero warnings tolerance** - Clippy pedantic + nursery validation
- 🔒 **Security audit** - Vulnerability scanning via cargo-audit & cargo-deny
- 📦 **SBOM** - Complete Software Bill of Materials with licenses
- 📊 **Coverage report** - Test coverage statistics
- 📝 **Build info** - Environment and toolchain details
- 🔄 **Dependency status** - Outdated dependency check

## Prerequisites

1. **Access required:**
   - Write access to the repository
   - `CRATES_TOKEN` secret configured (for crates.io publishing)

2. **Branch requirements:**
   - All changes merged to `development`
   - All CI checks passing
   - Documentation up to date

## Release Steps

### 1. Prepare Release Locally

```bash
# Ensure you're on development branch
git checkout development
git pull origin development

# Update version in Cargo.toml
cargo set-version 0.2.0

# Generate release reports locally to verify everything passes
just release-reports

# Review reports in ./release-reports/
# - Check clippy-report.md for zero warnings
# - Check security-audit.md for vulnerabilities
# - Review sbom.md for dependency licenses
# - Check coverage-report.md for test coverage

# If all looks good, commit version bump
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to 0.2.0"
git push origin development
```

### 2. Create Release PR (development → main)

```bash
# Create PR from development to main
gh pr create --base main --head development \
  --title "Release v0.2.0" \
  --body "Release v0.2.0

## Changes
- [List major changes here]

## Checklist
- [ ] Version bumped in Cargo.toml
- [ ] All tests passing
- [ ] Documentation updated
- [ ] CHANGELOG updated (if applicable)
- [ ] Local release reports reviewed

## Quality Assurance
All release quality checks will run automatically:
- Zero warnings validation (clippy)
- Security audit
- SBOM generation
- Test coverage report
- Build verification
"

# Wait for:
# 1. All CI checks to pass
# 2. Code review approval
# 3. Claude AI review (if configured)
```

### 3. Merge and Tag

```bash
# Once PR is approved, merge to main
gh pr merge --merge

# Switch to main and pull
git checkout main
git pull origin main

# Create and push tag
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

### 4. Automatic Release Process

When the tag is pushed, GitHub Actions automatically:

1. **Validates release** (`validate` job)
   - Checks version in Cargo.toml matches tag
   - Runs `nix flake check` (all quality checks)

2. **Generates reports** (`generate-reports` job)
   - ✅ Clippy report with zero warnings validation
   - 🔒 Security audit (cargo-audit + cargo-deny)
   - 📦 SBOM with full dependency tree & licenses
   - 📊 Test coverage report
   - 📝 Build information
   - 🔄 Dependency update status

3. **Publishes to crates.io** (`publish-crates` job)
   - Verifies package
   - Publishes to crates.io

4. **Creates GitHub Release** (`create-release` job)
   - Creates release with summary
   - Attaches all 7 quality reports as downloadable assets

5. **Builds artifacts** (`build-artifacts` job)
   - Builds for Linux (x64)
   - Builds for macOS (x64 + arm64)
   - Builds for Windows (x64)
   - Uploads binaries to release

6. **Notifies completion** (`notify` job)
   - Reports success/failure

### 5. Verify Release

After workflow completes:

```bash
# Check crates.io
open https://crates.io/crates/singularity-language-registry

# Check GitHub release with reports
open https://github.com/Singularity-ng/singularity-language-registry/releases/latest

# Verify docs.rs built correctly
open https://docs.rs/singularity-language-registry
```

## Release Artifacts

Each release includes these downloadable assets:

### Quality Reports (7 files)

1. **RELEASE_SUMMARY.md**
   - High-level overview of release quality
   - Quick reference for security and quality status

2. **clippy-report.md**
   - Full clippy output with zero warnings tolerance
   - Configuration: `-D warnings -W clippy::pedantic -W clippy::nursery`
   - Demonstrates strictest code quality standards

3. **security-audit.md**
   - cargo-audit vulnerability scan results
   - cargo-deny license and security policy checks
   - Known vulnerabilities with severity ratings

4. **sbom.md** (Software Bill of Materials)
   - Direct dependencies from Cargo.toml
   - Complete dependency tree with all transitive deps
   - License summary for all dependencies
   - Critical for compliance and security audits

5. **coverage-report.md**
   - Test coverage percentage
   - Line-by-line coverage details
   - Uncovered code sections

6. **build-info.md**
   - Rust toolchain version
   - Build metadata from Cargo.toml
   - Code statistics (lines of code)
   - Reproducibility information

7. **dependency-report.md**
   - Outdated dependency list
   - Available updates
   - Breaking changes warnings

### Binary Artifacts (4 files)

- `singularity-language-registry-linux-x64.tar.gz`
- `singularity-language-registry-macos-x64.tar.gz`
- `singularity-language-registry-macos-arm64.tar.gz`
- `singularity-language-registry-windows-x64.zip`

## Local Report Generation

Generate release reports locally before creating release:

```bash
# Generate all reports
just release-reports

# Reports saved to ./release-reports/
ls release-reports/
# clippy-report.md
# security-audit.md
# sbom.md
# coverage-report.md
```

This lets you:
- Verify zero warnings before release
- Check for security vulnerabilities
- Review dependency licenses
- Ensure adequate test coverage

## Quality Standards

Every release must meet these standards:

### Code Quality
- ✅ **Zero warnings** - Clippy with pedantic + nursery lints
- ✅ **All tests pass** - Including release mode tests
- ✅ **Format check** - `cargo fmt` validation
- ✅ **Documentation builds** - All doc comments valid

### Security
- ✅ **No known vulnerabilities** - cargo-audit clean
- ✅ **License compliance** - All deps have approved licenses
- ✅ **No unsafe code** - Unless explicitly documented
- ✅ **SBOM included** - Full transparency on dependencies

### Testing
- ✅ **Unit tests** - All passing
- ✅ **Integration tests** - All passing
- ✅ **Coverage >80%** - Adequate test coverage (target)
- ✅ **MSRV validated** - Minimum Rust version tested

## Troubleshooting

### Release workflow fails

1. **Check validation step:**
   ```bash
   # Run locally
   nix flake check -L
   ```

2. **Check clippy issues:**
   ```bash
   # Generate report locally
   just release-reports
   cat release-reports/clippy-report.md
   ```

3. **Check security issues:**
   ```bash
   cargo audit
   cargo deny check
   ```

### Version mismatch error

Ensure Cargo.toml version matches the git tag:
```bash
# Tag: v0.2.0
# Cargo.toml: version = "0.2.0"  (no 'v' prefix!)
```

### crates.io publish fails

1. Check `CRATES_TOKEN` secret is configured
2. Verify you're a crate owner on crates.io
3. Check for name conflicts

### Reports not generated

Check that Nix dev tools are available:
```bash
nix develop --command cargo audit --version
nix develop --command cargo deny --version
nix develop --command cargo tarpaulin --version
```

## Hotfix Releases

For critical bugs in production:

```bash
# Create hotfix branch from main
git checkout main
git checkout -b hotfix/0.1.1

# Fix the bug
# ... make changes ...

# Test thoroughly
cargo test --all-features
nix flake check

# Commit and push
git add .
git commit -m "fix: critical bug in X"
git push origin hotfix/0.1.1

# Create PR to main
gh pr create --base main --head hotfix/0.1.1 \
  --title "Hotfix v0.1.1: Fix critical bug" \
  --body "Emergency fix for critical bug"

# Once merged, tag and release
git checkout main
git pull origin main
git tag -a v0.1.1 -m "Hotfix: Fix critical bug"
git push origin v0.1.1

# Merge back to development
git checkout development
git merge main
git push origin development
```

## Post-Release

After successful release:

1. **Update documentation** if needed
2. **Announce release** in relevant channels
3. **Monitor** for issues in the first 24 hours
4. **Update dependent projects** if applicable

## Release Cadence

- **Patch releases (0.x.Y)** - As needed for bugs
- **Minor releases (0.X.0)** - Monthly or when features are ready
- **Major releases (X.0.0)** - When breaking changes are needed

## Questions?

- Check [CONTRIBUTING.md](./CONTRIBUTING.md) for development workflow
- See [.github/workflows/release.yml](.github/workflows/release.yml) for CI details
- Open an issue for release-related questions
