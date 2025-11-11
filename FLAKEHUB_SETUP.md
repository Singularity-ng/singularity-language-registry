# FlakeHub Setup Guide

## ✅ FlakeHub Token Configured!

Your FlakeHub token is now securely stored in `.envrc.local` (gitignored).

## What This Enables

With your FlakeHub token configured:

1. **Push to FlakeHub Cache** - Your builds will be cached and available instantly
2. **Binary Distribution** - Users can install pre-built binaries instead of compiling
3. **Faster CI/CD** - Pull from cache instead of rebuilding
4. **Public Access** - Anyone can use your cached builds (read-only)

## Local Usage

```bash
# Token is automatically loaded when you enter the directory (direnv)
cd /home/mhugo/code/singularity-language-registry
# You should see: "🔐 Local secrets loaded from .envrc.local"

# Push local build to FlakeHub
nix build --json | jq -r '.[].outputs | to_entries[].value' | \
  flakehub push "Singularity-ng/singularity-language-registry"

# Or use the determinate CLI
det push --token $FLAKEHUB_PUSH_TOKEN
```

## GitHub Actions Setup

Add the token as a GitHub secret:

1. Go to: https://github.com/Singularity-ng/singularity-language-registry/settings/secrets/actions
2. Click "New repository secret"
3. Name: `FLAKEHUB_PUSH_TOKEN`
4. Value: (paste your token)
5. Click "Add secret"

The CI workflow is already configured to use it:
```yaml
env:
  FLAKEHUB_PUSH_TOKEN: ${{ secrets.FLAKEHUB_PUSH_TOKEN }}
```

## FlakeHub URLs

Once pushed, your flake will be available at:

```bash
# Latest version
nix build "https://flakehub.com/f/Singularity-ng/singularity-language-registry/*"

# Specific version
nix build "https://flakehub.com/f/Singularity-ng/singularity-language-registry/0.1.*"

# In flake.nix
inputs.singularity-lang-reg.url = "https://flakehub.com/f/Singularity-ng/singularity-language-registry/*";
```

## Publishing to FlakeHub

### Manual Push

```bash
# Build and push
nix build
flakehub push --token $FLAKEHUB_PUSH_TOKEN

# Or with metadata
flakehub push \
  --token $FLAKEHUB_PUSH_TOKEN \
  --tag "v0.1.0" \
  --description "Production-ready language registry"
```

### Automatic via CI

Every push to `main` will automatically publish to FlakeHub (configured in CI).

## Benefits for Users

Users get instant installs with binary cache:

```bash
# Without FlakeHub: ~5-10 minutes (builds from source)
nix build github:Singularity-ng/singularity-language-registry

# With FlakeHub: ~10 seconds (downloads binary)
nix build "https://flakehub.com/f/Singularity-ng/singularity-language-registry/*"
```

## Security Notes

- ✅ Token is in `.envrc.local` (gitignored)
- ✅ Token expires: 2025-02-09 (90 days)
- ✅ Token is user-scoped (not org-wide)
- ⚠️ Never commit the token to git
- ⚠️ Rotate token before expiration

## Token Management

```bash
# Check token expiration
echo $FLAKEHUB_PUSH_TOKEN | jwt decode -

# Renew token (when needed)
# Go to: https://flakehub.com/settings/tokens
# Generate new token and update .envrc.local
```

## Verification

Test that FlakeHub is working:

```bash
# 1. Build locally
nix build

# 2. Push to FlakeHub (dry run)
flakehub push --dry-run --token $FLAKEHUB_PUSH_TOKEN

# 3. Try pulling from FlakeHub
nix build "https://flakehub.com/f/Singularity-ng/singularity-language-registry/*"
```

## Dashboard

View your FlakeHub dashboard at:
https://flakehub.com/flake/Singularity-ng/singularity-language-registry

This shows:
- Download statistics
- Available versions
- Cache status
- Binary availability

Your FlakeHub integration is ready! 🚀