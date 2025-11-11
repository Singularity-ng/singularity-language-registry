# Installation Guide

Multiple ways to use `singularity-language-registry` in your project.

## 📦 Mix Dependencies (Elixir)

Add to your `mix.exs`:

### Always Use Latest (Recommended for Development)

```elixir
def deps do
  [
    {:singularity_language_registry,
     git: "https://github.com/Singularity-ng/singularity-language-registry",
     tag: "latest"}
  ]
end
```

### Pin to Specific Version (Recommended for Production)

```elixir
def deps do
  [
    {:singularity_language_registry,
     git: "https://github.com/Singularity-ng/singularity-language-registry",
     tag: "v0.1.0"}
  ]
end
```

### Use Main Branch (Bleeding Edge)

```elixir
def deps do
  [
    {:singularity_language_registry,
     git: "https://github.com/Singularity-ng/singularity-language-registry",
     branch: "main"}
  ]
end
```

Then run:
```bash
mix deps.get
mix deps.compile
```

## 🔧 Using Pre-built Binaries

Download the library binary for your platform:

### Linux (x64)

```bash
curl -L https://github.com/Singularity-ng/singularity-language-registry/releases/download/latest/singularity-language-registry-linux-x64.tar.gz | tar xz

# Verify checksum
curl -L https://github.com/Singularity-ng/singularity-language-registry/releases/download/latest/BINARY_SHA256SUMS -o BINARY_SHA256SUMS
sha256sum -c BINARY_SHA256SUMS --ignore-missing
```

### macOS (Apple Silicon - ARM64)

```bash
curl -L https://github.com/Singularity-ng/singularity-language-registry/releases/download/latest/singularity-language-registry-macos-arm64.tar.gz | tar xz

# Verify checksum
curl -L https://github.com/Singularity-ng/singularity-language-registry/releases/download/latest/BINARY_SHA256SUMS -o BINARY_SHA256SUMS
shasum -a 256 -c BINARY_SHA256SUMS --ignore-missing
```

### macOS (Intel - x64)

```bash
curl -L https://github.com/Singularity-ng/singularity-language-registry/releases/download/latest/singularity-language-registry-macos-x64.tar.gz | tar xz

# Verify checksum
curl -L https://github.com/Singularity-ng/singularity-language-registry/releases/download/latest/BINARY_SHA256SUMS -o BINARY_SHA256SUMS
shasum -a 256 -c BINARY_SHA256SUMS --ignore-missing
```

### Windows (x64)

PowerShell:
```powershell
Invoke-WebRequest -Uri "https://github.com/Singularity-ng/singularity-language-registry/releases/download/latest/singularity-language-registry-windows-x64.zip" -OutFile "singularity-language-registry.zip"
Expand-Archive -Path "singularity-language-registry.zip" -DestinationPath "."

# Verify checksum
Invoke-WebRequest -Uri "https://github.com/Singularity-ng/singularity-language-registry/releases/download/latest/BINARY_SHA256SUMS" -OutFile "BINARY_SHA256SUMS"
# Then manually verify hash matches
```

## 🔐 Verify Artifact Attestations

If you have `gh` CLI installed, verify artifacts came from official GitHub Actions:

```bash
# Download artifact
curl -LO https://github.com/Singularity-ng/singularity-language-registry/releases/download/v0.1.0/singularity-language-registry-0.1.0.crate

# Verify attestation
gh attestation verify singularity-language-registry-0.1.0.crate \
  -R Singularity-ng/singularity-language-registry
```

Expected output:
```
✓ Verification succeeded!

sha256:abc123... was attested by:
REPO                    PREDICATE_TYPE                  WORKFLOW
Singularity-ng/singul…  https://slsa.dev/provenance/v1  .github/workflows/release.yml@refs/tags/v0.1.0
```

## 📚 Using in Your Rust Project

### From Git (in Cargo.toml)

```toml
[dependencies]
singularity-language-registry = { git = "https://github.com/Singularity-ng/singularity-language-registry", tag = "v0.1.0" }
```

### From Downloaded .crate File

```bash
# Download crate
curl -LO https://github.com/Singularity-ng/singularity-language-registry/releases/download/latest/singularity-language-registry-0.1.0.crate

# Extract
tar -xzf singularity-language-registry-0.1.0.crate

# Add as local dependency in Cargo.toml
[dependencies]
singularity-language-registry = { path = "./singularity-language-registry-0.1.0" }
```

## 🆘 Troubleshooting

### "Failed to download"
- Check your network connection
- Ensure you have access to GitHub (may require VPN in some regions)
- Try using a specific version tag instead of `latest`

### "Checksum mismatch"
- Re-download the file (may have been corrupted)
- Verify you're downloading from official Singularity-ng/singularity-language-registry repo

### Mix compilation errors
- Ensure Rust toolchain is installed: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Update Mix dependencies: `mix deps.clean --all && mix deps.get`

### Binary won't run on macOS
- On first run, you may need to allow the binary in System Preferences → Security & Privacy
- Or remove quarantine: `xattr -d com.apple.quarantine libsingularity_language_registry.*`

## 📖 Documentation

- **API Docs**: https://docs.rs/singularity-language-registry
- **Examples**: See `examples/` directory
- **Changelog**: See GitHub Releases

## 📄 License

Proprietary software. See LICENSE file for terms.
