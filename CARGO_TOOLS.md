# Cargo Tools: Dev vs CI vs Prod

## Production (Library Only)
**Required tools: NONE**

This is a library crate consumed via `Cargo.toml`:
```toml
[dependencies]
singularity-language-registry = "0.1.0"
```

---

## Development Environment (Optimized)

### What's Included in Dev Shell

Our `flake.nix` provides a carefully curated set of tools:

```nix
nativeBuildInputs = [
  # Core Rust toolchain
  rustToolchain       # cargo, rustc, rustfmt, clippy, rust-analyzer
  rust-analyzer       # IDE support

  # Essential cargo tools (daily use)
  cargo-edit          # cargo add/rm/upgrade
  cargo-watch         # Auto-run on file changes
  cargo-nextest       # Faster test runner
  cargo-expand        # Macro debugging

  # Quality & security (frequent use)
  cargo-audit         # Security vulnerability checking
  cargo-outdated      # Dependency updates
  cargo-machete       # Unused dependency detection
  cargo-deny          # License and security policy

  # Performance tools (occasional use)
  cargo-tarpaulin     # Code coverage
  cargo-criterion     # Benchmarking

  # Development tools
  git                 # Version control
  gh                  # GitHub CLI
  just                # Task runner
];
```

### Why These Tools?

| Tool | Daily Use | Why Included |
|------|-----------|--------------|
| `cargo-edit` | ⭐⭐⭐⭐⭐ | Essential for managing dependencies |
| `cargo-watch` | ⭐⭐⭐⭐⭐ | Instant feedback during development |
| `cargo-nextest` | ⭐⭐⭐⭐⭐ | 3x faster than `cargo test` |
| `cargo-audit` | ⭐⭐⭐⭐ | Critical for security |
| `cargo-outdated` | ⭐⭐⭐⭐ | Keep dependencies current |
| `cargo-expand` | ⭐⭐⭐⭐ | Essential for macro debugging |
| `cargo-machete` | ⭐⭐⭐ | Clean up unused deps |
| `cargo-deny` | ⭐⭐⭐ | Enforce dependency policies |
| `cargo-tarpaulin` | ⭐⭐ | Coverage when needed |
| `cargo-criterion` | ⭐⭐ | Benchmarking when needed |

### What We Removed (and why)

| Tool | Reason for Removal |
|------|-------------------|
| `watchexec` | Duplicates `cargo-watch` |
| `hyperfine` | Rarely used, `cargo-criterion` is better |
| `tokei` | Vanity metrics, not actionable |
| `typos` | Can run in CI if needed |
| `taplo` | `cargo fmt` handles most formatting |
| `yamlfmt` | Rarely needed |
| `renovate` | Runs in CI via GitHub Actions |

---

## CI Environment (All-Nix Approach)

### Primary CI Job: `nix flake check`

One command runs all essential checks:

```bash
nix flake check -L
```

This executes:
- ✅ **Build** - Full package build
- ✅ **Test** - All tests with `--all-features`
- ✅ **Clippy** - Pedantic + nursery lints
- ✅ **Format** - `cargo fmt --check`
- ✅ **Audit** - Security vulnerabilities
- ✅ **Doc** - Documentation builds

### What Each Check Does

```nix
checks = {
  # Builds the package with strict warnings
  inherit singularity-language-registry;

  # Format check
  fmt = cargo fmt --check;

  # Clippy with strict lints
  clippy = cargo clippy --all-targets --all-features
           -- -D warnings -W clippy::pedantic -W clippy::nursery;

  # Security audit
  audit = cargo audit;

  # Documentation builds
  doc = cargo doc --all-features --no-deps;

  # All tests in release mode
  test = cargo test --all-features --release;
};
```

### Additional CI Jobs

1. **MSRV Check** - Validates minimum Rust version (1.91.0)
2. **Coverage** - Runs `cargo tarpaulin` on PRs only
3. **Benchmarks** - Runs on `main` branch only
4. **Dependency Audit** - Weekly scheduled run

### Benefits of All-Nix CI

✅ **Dev-CI Parity** - Same environment everywhere
✅ **No `cargo install`** - Pre-built binaries from cache
✅ **4-Layer Caching** - GitHub + Magic Nix + FlakeHub + Cachix
✅ **Reproducible** - Lock file ensures consistency
✅ **Fast** - Parallel builds, shared cache

---

## Comparison: Old vs New CI

### Old CI (264 lines)
- ❌ 10 separate jobs
- ❌ Manual `cargo install` in each job
- ❌ Redundant checks across jobs
- ❌ Different environments (dev vs CI)
- ⏱️ ~15-20 minutes

### New CI (190 lines)
- ✅ 1 primary job (`nix flake check`)
- ✅ Pre-built binaries from cache
- ✅ All checks run once, efficiently
- ✅ Exact dev-CI parity
- ⏱️ ~5-10 minutes

---

## Quick Reference

### Local Development

```bash
# Enter dev shell
nix develop

# Quick iteration
just watch           # Auto-run tests on changes

# Before committing
just verify          # fmt, clippy, test, audit

# Run CI locally
nix flake check      # Same as CI runs
```

### What Gets Cached

1. **Nix Store** - All packages and dependencies
2. **Cargo artifacts** - Compiled dependencies
3. **Build outputs** - Final binaries
4. **Test results** - When deterministic

### Cache Layers (All FREE without tokens!)

1. **GitHub Actions Cache** - Cross-run persistence
2. **Magic Nix Cache** - Within workflow
3. **cache.nixos.org** - Upstream packages
4. **FlakeHub** (optional) - If you set token
5. **Cachix** (optional) - If you set token

---

## Summary

| Environment | Tools Needed | Method |
|-------------|--------------|--------|
| **Development** | 15 cargo tools | `nix develop` |
| **CI** | Same tools (via Nix) | `nix flake check` |
| **Production** | None | Library dependency |

**Key Insight**: Using Nix for both dev and CI means:
- Same tools everywhere (no "works on my machine")
- No manual installations
- Faster CI with better caching
- One source of truth (flake.nix)
