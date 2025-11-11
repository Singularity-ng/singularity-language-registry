# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive release reports with security scanning
- SBOM (Software Bill of Materials) generation
- Zero warnings validation with Clippy pedantic + nursery
- Local report generation via `just release-reports`
- All-Nix CI workflow for dev-CI parity
- Auto-reload direnv configuration
- Enhanced development shell with curated tools

### Changed
- License changed to proprietary (all rights reserved)
- CI workflow streamlined to single primary job
- Development shell reduced from 22 to 15 essential tools
- Release workflow now uses Nix for consistency

### Removed
- Low-value development tools (watchexec, hyperfine, tokei, typos, taplo, yamlfmt, renovate from dev shell)

### Documentation
- Added RELEASE_PROCESS.md - Complete release guide
- Added RELEASE_REPORTS_SUMMARY.md - Report documentation
- Added CARGO_TOOLS.md - Tool analysis
- Added REFACTORING_SUMMARY.md - CI/dev changes
- Updated CONTRIBUTING.md with Nix workflow

### Infrastructure
- 4-layer caching in CI (GitHub + Magic Nix + FlakeHub + Cachix)
- Release reports automatically attached to GitHub releases
- Local development environment matches CI exactly

## [0.1.0] - YYYY-MM-DD

### Added
- Initial release
- Language detection for 18+ programming languages
- Support for BEAM, Systems, Web, JVM, and Scripting language families
- RCA (Runtime Code Analysis) support tracking
- AST-Grep support tracking
- Pattern-based content detection
- Shebang detection
- Special file detection (Makefile, Dockerfile, etc.)
- Comprehensive language metadata (extensions, aliases, MIME types)
- Zero dependencies on other Singularity crates
- Nix flake for reproducible development environment
- GitHub Actions CI/CD pipeline
- Documentation with examples

### Features
- `detect_language()` - Detect from file path
- `detect_from_content()` - Detect from file content
- `detect_from_shebang()` - Detect from shebang line
- `get_language()` - Get language by ID
- `supported_languages()` - List all supported languages
- `rca_supported_languages()` - List RCA-supported languages
- Language family grouping and queries
- Detection confidence scoring

### Supported Languages
- **BEAM**: Elixir, Erlang, Gleam
- **Systems**: Rust, C, C++, Go
- **Web**: JavaScript, TypeScript, Python
- **JVM**: Java, Kotlin
- **Other**: C#, Bash, Lua, SQL, TOML, YAML, JSON, Markdown, Dockerfile

---

## Release Notes Format

Each release includes:
- **CHANGELOG.md** - Human-readable changes (this file)
- **RELEASE_SUMMARY.md** - Quality metrics and installation
- **clippy-report.md** - Zero warnings proof
- **security-audit.md** - Vulnerability scan
- **sbom.md** - Complete dependency list with licenses
- **coverage-report.md** - Test coverage statistics
- **build-info.md** - Build environment details
- **dependency-report.md** - Dependency health status

---

## Versioning Guidelines

- **Major version (X.0.0)** - Breaking API changes
- **Minor version (0.X.0)** - New features, backwards compatible
- **Patch version (0.0.X)** - Bug fixes, backwards compatible

## Categories

- **Added** - New features
- **Changed** - Changes to existing functionality
- **Deprecated** - Soon-to-be removed features
- **Removed** - Removed features
- **Fixed** - Bug fixes
- **Security** - Vulnerability fixes
- **Documentation** - Documentation changes
- **Infrastructure** - CI/CD, tooling, build changes

---

[Unreleased]: https://github.com/singularity-ng/singularity-language-registry/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/singularity-ng/singularity-language-registry/releases/tag/v0.1.0
