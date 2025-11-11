# Setup Nix + Rust Action

Composite action that sets up a complete Nix development environment with multi-layer caching for Rust projects.

## Features

- ✅ Installs Nix with flakes enabled
- ✅ 4-layer caching strategy:
  1. GitHub Actions Cache
  2. Magic Nix Cache (Determinate Systems)
  3. FlakeHub Cache (Linux only)
  4. Cachix (optional)
- ✅ Automatically reads Rust toolchain from flake.nix
- ✅ Works on Linux, macOS, and Windows

## Usage

### Basic (no Cachix)
```yaml
- uses: Singularity-ng/github-actions/setup-nix-rust@v1
```

### With Cachix
```yaml
- uses: Singularity-ng/github-actions/setup-nix-rust@v1
  with:
    cachix-name: singularity
    cachix-auth-token: ${{ secrets.CACHIX_AUTH_TOKEN }}
```

### Minimal (only Magic Nix Cache)
```yaml
- uses: Singularity-ng/github-actions/setup-nix-rust@v1
  with:
    enable-github-cache: false
    enable-flakehub: false
```

## Inputs

| Input | Description | Required | Default |
|-------|-------------|----------|---------|
| `cachix-name` | Cachix cache name | No | `''` |
| `cachix-auth-token` | Cachix auth token (use secrets) | No | `''` |
| `enable-flakehub` | Enable FlakeHub cache (Linux only) | No | `true` |
| `enable-github-cache` | Enable GitHub Actions cache | No | `true` |

## Outputs

None. The action sets up the environment for subsequent steps.

## Example Workflow

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: Singularity-ng/github-actions/setup-nix-rust@v1
        with:
          cachix-name: singularity
          cachix-auth-token: ${{ secrets.CACHIX_AUTH_TOKEN }}

      - name: Run tests
        run: nix develop --command cargo test

      - name: Run clippy
        run: nix develop --command cargo clippy
```

## Caching Strategy

The action uses a 4-layer caching approach:

1. **GitHub Actions Cache** - Fastest for same runner, 10GB limit per repo
2. **Magic Nix Cache** - Automatic, no configuration needed
3. **FlakeHub Cache** - Community cache (Linux only)
4. **Cachix** - Persistent cache across runs, unlimited storage

Each layer falls back to the next if cache miss occurs.

## Requirements

- Repository must have a `flake.nix` file
- `flake.nix` must define a Rust development environment

## Troubleshooting

### Cache not working
- Check that `flake.lock` is committed
- Verify Cachix name and token are correct
- Review cache keys in GitHub Actions UI

### Nix installation fails
- Ensure runner has sufficient disk space
- Check Nix installer logs for errors

## License

Same as parent repository.
