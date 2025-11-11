# Documentation Setup Guide

This guide explains how to set up and use the rustdoc documentation generation and GitHub Pages deployment for this project.

## Overview

The project uses:
- **rustdoc** for API documentation generation
- **GitHub Actions** for automated builds and deployment
- **GitHub Pages** for hosting the documentation
- **Nix** for local documentation generation

## Quick Start

### View Documentation Locally

Using Nix (recommended):
```bash
# Build and open documentation in your browser
nix run .#docs

# Or build documentation manually
cargo doc --all-features --no-deps --document-private-items --open
```

Using cargo directly:
```bash
# Generate and open documentation
cargo doc --all-features --open

# Generate with private items (internal docs)
cargo doc --all-features --document-private-items --open

# Generate with docs.rs style
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --open
```

### Build Documentation Package with Nix

```bash
# Build the documentation package
nix build .#docs

# View the generated documentation
firefox ./result/share/doc/index.html
```

## GitHub Pages Setup

### For GitHub.com

1. **Enable GitHub Pages in Repository Settings**
   - Go to: `Settings` → `Pages`
   - Under "Source", select: **GitHub Actions**
   - Save the settings

2. **Trigger Documentation Build**
   - Push to the `main` branch
   - Or manually trigger: `Actions` → `Documentation` → `Run workflow`

3. **Access Your Documentation**
   - Once deployed, documentation will be available at:
     - `https://<org>.github.io/<repo>/`
     - Example: `https://singularity-ng.github.io/singularity-language-registry/`

### For GitHub Enterprise

1. **Verify GitHub Pages is Enabled**
   - Check with your GitHub Enterprise administrator
   - GitHub Pages must be enabled at the enterprise or organization level

2. **Enable in Repository Settings**
   - Go to: `Settings` → `Pages`
   - Under "Source", select: **GitHub Actions**
   - Save the settings

3. **Update Base URL (if needed)**
   - If your GitHub Enterprise uses a custom domain, update the documentation URL in `Cargo.toml`:
   ```toml
   [package]
   documentation = "https://<your-gh-enterprise-domain>/pages/<org>/<repo>/"
   ```

4. **Trigger Documentation Build**
   - Push to the `main` branch
   - Or manually trigger the workflow

## Workflow Details

The documentation workflow (`.github/workflows/docs.yml`) runs when:
- Code is pushed to the `main` branch
- Documentation-related files are modified (`src/**`, `Cargo.toml`)
- Manually triggered via GitHub Actions UI

### What the Workflow Does

1. **Build Documentation**
   - Uses `cargo +nightly doc` for advanced features
   - Enables `--document-private-items` for complete API coverage
   - Applies docs.rs styling with `--cfg docsrs`
   - Generates documentation for all features

2. **Create Landing Page**
   - Auto-generates `index.html` that redirects to the crate documentation
   - Adds `.nojekyll` file to disable Jekyll processing

3. **Deploy to GitHub Pages**
   - Uploads documentation artifacts
   - Deploys to GitHub Pages environment
   - Provides deployment URL in workflow summary

## Documentation Features

### Cargo.toml Configuration

The project includes metadata for optimal documentation:

```toml
[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
cargo-args = ["-Zunstable-options", "-Zrustdoc-scrape-examples"]
```

This enables:
- All feature flags in documentation
- docs.rs-style conditional compilation
- Code example scraping (nightly feature)

### Documentation Best Practices

1. **Document Public APIs**
   ```rust
   /// Brief one-line description.
   ///
   /// Detailed explanation with examples.
   ///
   /// # Examples
   ///
   /// ```
   /// use singularity_language_registry::Language;
   /// let lang = Language::new("Rust");
   /// ```
   pub struct Language { }
   ```

2. **Use Doc Links**
   ```rust
   /// See [`Language`] for more information.
   /// Check the [`detect`] function for usage.
   ```

3. **Add Examples**
   - Document examples with `///` or `//!`
   - Use `# Examples` section in doc comments
   - Add doctests that are automatically run by `cargo test`

## Nix Flake Integration

The flake provides:

### Packages
- `nix build .#docs` - Build documentation package

### Apps
- `nix run .#docs` - Build and open documentation in browser

### Checks
- `nix flake check` - Runs `doc` check to ensure documentation builds

### Dev Shell
The development shell includes rustdoc and related tools:
```bash
nix develop
cargo doc --open
```

## Troubleshooting

### Documentation Fails to Build

1. **Check for doc warnings**
   ```bash
   RUSTDOCFLAGS="-D warnings" cargo doc --all-features
   ```

2. **Fix broken doc links**
   - Ensure all `[`links`]` in documentation are valid
   - Use `cargo rustdoc` for detailed error messages

3. **Check feature flags**
   - Some docs may require specific features
   - Use `--all-features` to include everything

### GitHub Pages Not Updating

1. **Verify workflow ran successfully**
   - Check `Actions` tab for workflow status
   - Review logs for any errors

2. **Check GitHub Pages settings**
   - Ensure "Source" is set to "GitHub Actions"
   - Verify GitHub Pages is enabled for your organization/enterprise

3. **Clear browser cache**
   - GitHub Pages may cache old content
   - Hard refresh: `Ctrl+Shift+R` (Linux/Windows) or `Cmd+Shift+R` (Mac)

### Permission Issues

If deployment fails with permission errors:

1. **Check repository settings**
   - Go to: `Settings` → `Actions` → `General`
   - Under "Workflow permissions", select: **Read and write permissions**
   - Enable: **Allow GitHub Actions to create and approve pull requests**

2. **Verify Pages permissions**
   - The workflow requires `pages: write` and `id-token: write`
   - These are set in the workflow file

## Manual Deployment

For testing or one-off deployments:

```bash
# Build documentation
cargo doc --all-features --no-deps

# Add redirect index page
cat > target/doc/index.html <<'EOF'
<!DOCTYPE html>
<html>
<head>
  <meta http-equiv="refresh" content="0; url=singularity_language_registry/index.html">
  <title>Redirecting...</title>
</head>
<body>
  <p>Redirecting to <a href="singularity_language_registry/index.html">documentation</a>...</p>
</body>
</html>
EOF

# Add .nojekyll
touch target/doc/.nojekyll

# Deploy manually (using gh-pages or your preferred method)
```

## CI Integration

Documentation is also checked in the main CI workflow (`.github/workflows/ci.yml`):

```yaml
- name: Check documentation
  run: cargo doc --all-features --no-deps
```

This ensures documentation always builds successfully before merging PRs.

## Further Reading

- [The rustdoc Book](https://doc.rust-lang.org/rustdoc/)
- [docs.rs Documentation](https://docs.rs/about)
- [GitHub Pages Documentation](https://docs.github.com/en/pages)
- [Nix Flakes Guide](https://nixos.wiki/wiki/Flakes)

## Support

For issues specific to:
- **Documentation generation**: Check rustdoc errors with `cargo doc --verbose`
- **GitHub Actions**: Review workflow logs in the Actions tab
- **GitHub Pages**: Verify settings and consult GitHub Enterprise admin
- **Nix builds**: Run `nix flake check` for detailed diagnostics
