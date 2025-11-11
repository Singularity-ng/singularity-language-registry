# Build Crate Package Action

Composite action that builds a Rust `.crate` package with installation instructions and contents listing.

## Features

- ✅ Builds `.crate` file using `cargo package`
- ✅ Lists all files included in package
- ✅ Generates installation guide with 3 methods
- ✅ Shows package size
- ✅ Supports dirty builds (uncommitted changes)

## Usage

### Basic
```yaml
- uses: Singularity-ng/github-actions/build-crate-package@v1
  with:
    version: "1.0.0"
```

### Strict (no dirty builds)
```yaml
- uses: Singularity-ng/github-actions/build-crate-package@v1
  with:
    version: "1.0.0"
    allow-dirty: false
```

## Inputs

| Input | Description | Required | Default |
|-------|-------------|----------|---------|
| `version` | Crate version number (e.g., 1.0.0) | Yes | - |
| `allow-dirty` | Allow building with uncommitted changes | No | `true` |

## Outputs

The action creates:

```
crate-package/
├── {crate-name}-{version}.crate    # The package
├── INSTALL.md                       # Installation instructions
└── PACKAGE_CONTENTS.txt             # File listing
```

### INSTALL.md Contents

The generated installation guide includes:
1. **Git tag method** (recommended) - Add to Cargo.toml with git URL
2. **Local install method** - Install from downloaded .crate file
3. **Path dependency method** - Extract and use as local dependency

### PACKAGE_CONTENTS.txt Contents

Shows:
- Package name and version
- Build date
- Complete file listing
- Package size

## Example Workflow

```yaml
name: Release

on:
  push:
    tags: ['v*']

jobs:
  build-package:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Get version from tag
        id: version
        run: echo "version=${GITHUB_REF#refs/tags/v}" >> $GITHUB_OUTPUT

      - uses: Singularity-ng/github-actions/build-crate-package@v1
        with:
          version: ${{ steps.version.outputs.version }}

      - uses: actions/upload-artifact@v4
        with:
          name: crate-package
          path: crate-package/

      - name: Upload to release
        uses: softprops/action-gh-release@v2
        with:
          files: crate-package/*
```

## Requirements

- Repository must have `Cargo.toml`
- Version in `Cargo.toml` must match the `version` input
- Rust toolchain will be installed if not present

## What Gets Included in Package

The package includes files based on your `Cargo.toml` configuration:

- ✅ **Included by default**: `src/`, `Cargo.toml`, `LICENSE`, `README.md`
- ❌ **Excluded by default**: `target/`, `.git/`, CI configs
- ⚙️ **Customizable**: Use `exclude` in `Cargo.toml` to control

Example:
```toml
[package]
exclude = [
    ".github/",
    "flake.nix",
    "justfile",
    "*.md",  # Exclude all markdown except README.md
]
```

## Verification

After the action runs, you can verify the package:

```yaml
- name: Verify package contents
  run: |
    tar -tzf crate-package/*.crate
    cat crate-package/PACKAGE_CONTENTS.txt
```

## Troubleshooting

### Version mismatch error
```
error: `` does not match pattern `\d+\.\d+\.\d+`
```
**Solution**: Ensure version input matches semantic versioning (e.g., `1.0.0`, not `v1.0.0`)

### Dirty working directory error
```
error: 1 uncommitted file
```
**Solutions**:
- Commit all changes before running
- Or set `allow-dirty: true` (default)

### Package too large
```
error: package is too large
```
**Solution**: Add more files to `exclude` in `Cargo.toml`

## Best Practices

1. **Always specify version explicitly** - Don't rely on default
2. **Use `allow-dirty: false` for releases** - Ensures clean state
3. **Review PACKAGE_CONTENTS.txt** - Verify what gets included
4. **Test installation locally** - Before releasing

## License

Same as parent repository.
