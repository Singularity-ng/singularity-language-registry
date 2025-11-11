# Private Crate Registry Setup

## ✅ Yes! You have multiple options for private crates:

## Option 1: GitHub Packages (Recommended for GitHub Enterprise)

### Setup

1. **Create a GitHub PAT**:
```bash
# Go to: Settings → Developer settings → Personal access tokens
# Scopes needed: write:packages, read:packages, delete:packages (optional)
```

2. **Configure Cargo**:
```bash
# Login to GitHub registry
cargo login --registry github
# Enter your PAT when prompted

# Or set in ~/.cargo/credentials
[registries.github]
token = "ghp_xxxxxxxxxxxxxxxxxxxx"
```

3. **Update Cargo.toml** for publishing:
```toml
[package]
name = "singularity-language-registry"
version = "0.1.0"
# Add registry field
registry = "github"

# Or publish to multiple registries
[package.metadata.registries]
github = { index = "sparse+https://github.com/Singularity-ng/_cargo-index/" }
```

4. **Publish to GitHub Packages**:
```bash
# Publish to GitHub instead of crates.io
cargo publish --registry github

# Users install with:
cargo add singularity-language-registry --registry github
```

### In Dependencies (Cargo.toml)

```toml
[dependencies]
# From GitHub Packages
singularity-language-registry = { version = "0.1", registry = "github" }

# Or directly from git (simpler for private)
singularity-language-registry = { git = "ssh://git@github.com/Singularity-ng/singularity-language-registry.git", branch = "main" }

# With credentials
singularity-language-registry = { git = "https://oauth2:TOKEN@github.com/Singularity-ng/singularity-language-registry.git" }
```

## Option 2: Cloudsmith (Enterprise Cloud Registry)

### Setup
```bash
# Install cloudsmith CLI
pip install cloudsmith-cli

# Login
cloudsmith login

# Create repository
cloudsmith repos create cargo --name singularity-crates

# Configure cargo
cargo login --registry cloudsmith
```

### Cargo.toml
```toml
[registries.cloudsmith]
index = "https://dl.cloudsmith.io/basic/singularity-ng/cargo/cargo/index.git"

[dependencies]
singularity-language-registry = { version = "0.1", registry = "cloudsmith" }
```

## Option 3: Self-Hosted Kellnr (Open Source)

### Deploy Kellnr
```yaml
# docker-compose.yml
version: '3'
services:
  kellnr:
    image: kellnr/kellnr:latest
    ports:
      - "8080:8080"
    volumes:
      - ./data:/opt/kellnr/data
    environment:
      KELLNR_ORIGIN: "https://crates.company.com"
```

### Configure
```toml
# .cargo/config.toml
[registries.kellnr]
index = "sparse+https://crates.company.com/api/v1/crates/"

[source.crates-io]
replace-with = "kellnr"

[source.kellnr]
registry = "sparse+https://crates.company.com/api/v1/crates/"
```

## Option 4: JFrog Artifactory (Enterprise)

### Setup
```bash
# Configure Artifactory
curl -u username:password -X PUT \
  "https://artifactory.company.com/artifactory/api/repositories/cargo-local" \
  -H "Content-Type: application/json" \
  -d '{"rclass":"local","packageType":"cargo"}'
```

### Usage
```toml
[registries.artifactory]
index = "https://artifactory.company.com/artifactory/git/cargo-local.git"

[dependencies]
singularity-language-registry = { version = "0.1", registry = "artifactory" }
```

## Option 5: Direct Git Dependencies (Simplest)

### No registry needed!
```toml
[dependencies]
# SSH (recommended for private repos)
singularity-language-registry = { git = "ssh://git@github.com/Singularity-ng/singularity-language-registry.git" }

# HTTPS with token
singularity-language-registry = { git = "https://x-token:ghp_TOKEN@github.com/Singularity-ng/singularity-language-registry.git" }

# Specific branch/tag/commit
singularity-language-registry = {
    git = "ssh://git@github.com/Singularity-ng/singularity-language-registry.git",
    branch = "main"  # or tag = "v0.1.0" or rev = "abc123"
}
```

## Workflow Updates for Private Crates

### Release Workflow

```yaml
# .github/workflows/release.yml
publish-private:
  name: Publish to Private Registry
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4

    # For GitHub Packages
    - name: Publish to GitHub Packages
      run: |
        echo "[registries.github]" >> ~/.cargo/config.toml
        echo 'index = "sparse+https://github.com/Singularity-ng/_cargo-index/"' >> ~/.cargo/config.toml
        cargo publish --registry github
      env:
        CARGO_REGISTRIES_GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

    # Also publish to crates.io if public mirror needed
    - name: Publish to crates.io
      if: github.event.inputs.public == 'true'
      run: cargo publish
      env:
        CARGO_REGISTRY_TOKEN: ${{ secrets.CRATES_TOKEN }}
```

### CI Workflow

```yaml
# .github/workflows/ci.yml
test-private-deps:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4

    - name: Configure Private Registry
      run: |
        mkdir -p ~/.cargo
        echo "[registries.github]" >> ~/.cargo/config.toml
        echo 'index = "sparse+https://github.com/Singularity-ng/_cargo-index/"' >> ~/.cargo/config.toml
        echo "[registries.github]" >> ~/.cargo/credentials
        echo "token = \"${{ secrets.GITHUB_TOKEN }}\"" >> ~/.cargo/credentials

    - name: Build with Private Deps
      run: cargo build --all-features
```

## Dependency Resolution Strategy

```toml
# Cargo.toml for mixed public/private deps
[dependencies]
# Public crate
serde = "1.0"

# Private crate from GitHub Packages
internal-lib = { version = "0.1", registry = "github" }

# Private git dependency
secret-sauce = { git = "ssh://git@github.com/Singularity-ng/secret-sauce.git" }

# Override public crate with private fork
[patch.crates-io]
some-crate = { git = "ssh://git@github.com/Singularity-ng/some-crate-fork.git" }
```

## Best Practices

1. **Use SSH for Git Dependencies**
   - More secure than tokens
   - Works with SSH agent forwarding

2. **Separate Public/Private Packages**
   ```toml
   [workspace]
   members = [
     "public/*",
     "private/*",
   ]

   [workspace.metadata.publish]
   public = ["crates-io"]
   private = ["github"]
   ```

3. **Version Private Deps Carefully**
   ```toml
   # Use exact versions for private crates
   private-lib = "=0.1.2"  # Exact version

   # Use ranges for public crates
   serde = "1.0"  # Compatible range
   ```

4. **Cache Private Dependencies**
   ```yaml
   # In CI, cache the entire cargo directory
   - uses: actions/cache@v4
     with:
       path: |
         ~/.cargo/registry
         ~/.cargo/git
         ~/.cargo/bin
       key: ${{ runner.os }}-cargo-private-${{ hashFiles('**/Cargo.lock') }}
   ```

## Security Considerations

1. **Never Commit Tokens**
   ```bash
   # Add to .gitignore
   .cargo/credentials
   .cargo/config.toml  # If it contains tokens
   ```

2. **Use GitHub Secrets**
   ```yaml
   env:
     CARGO_REGISTRIES_GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
   ```

3. **Rotate Tokens Regularly**
   - Set up token rotation policy
   - Use short-lived tokens when possible

## Quick Start Commands

```bash
# Setup GitHub Packages
cargo login --registry github
cargo publish --registry github

# Install from private registry
cargo add my-private-crate --registry github

# Or use git directly (no setup needed!)
cargo add my-crate --git ssh://git@github.com/org/my-crate.git

# List configured registries
cargo config get registries

# Check registry configuration
cargo search my-crate --registry github
```

## Verification

Test your setup:
```bash
# 1. Create test crate
cargo new test-private-crate
cd test-private-crate

# 2. Configure for GitHub Packages
echo 'registry = "github"' >> Cargo.toml

# 3. Publish
cargo publish --registry github --allow-dirty

# 4. Use in another project
cargo add test-private-crate --registry github
```

Your private crate registry is ready! 🚀