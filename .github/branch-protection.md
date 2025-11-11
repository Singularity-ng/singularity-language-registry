# Branch Protection Settings

This document describes the branch protection rules for the Singularity Language Registry.

## Main Branch Protection

To protect the `main` branch, apply the following settings in GitHub repository settings:

### Required Status Checks
- ✅ **Require branches to be up to date before merging**
- Required checks:
  - `Test Suite (ubuntu-latest, stable)`
  - `Clippy (Pedantic)`
  - `Format Check`
  - `Security Audit`
  - `Minimum Supported Rust Version`

### Pull Request Requirements
- ✅ **Require pull request reviews before merging**
  - Required approving reviews: **1**
  - Dismiss stale pull request approvals when new commits are pushed
  - Require review from CODEOWNERS: Optional
  - Require approval of the most recent push

### Merge Requirements
- ✅ **Require conversation resolution before merging**
- ✅ **Require signed commits** (optional but recommended)
- ✅ **Require linear history** (optional)

### Admin Restrictions
- ❌ Do not allow bypassing the above settings
- ❌ Restrict who can push to matching branches (optional)

### Force Push Protection
- ❌ **Do not allow force pushes**
- ❌ **Do not allow deletions**

## Development Branch Protection

To protect the `development` branch with lighter restrictions:

### Required Status Checks
- ✅ **Require status checks to pass before merging**
- Required checks:
  - `Test Suite (ubuntu-latest, stable)`
  - `Clippy (Pedantic)`
- ❌ Do not require branches to be up to date before merging

### Pull Request Requirements
- ✅ **Allow direct pushes** (for active development)
- ⚠️ Optional: Require pull request reviews for external contributors

### Force Push Protection
- ❌ **Do not allow force pushes**
- ❌ **Do not allow deletions**

## Workflow

1. **Development Branch** (`development`):
   - Active development happens here
   - Direct pushes allowed for maintainers
   - CI must pass

2. **Main Branch** (`main`):
   - Production-ready code only
   - Requires PR from `development` or feature branches
   - Strict CI requirements
   - Requires review

3. **Release Process**:
   - Create release from `main` branch only
   - Tag with version (e.g., `v0.1.0`)
   - Automated release workflow publishes to crates.io

## Setting Up Protection via GitHub UI

1. Go to **Settings** → **Branches**
2. Click **Add rule** or edit existing rule
3. Enter branch name pattern (e.g., `main` or `development`)
4. Configure settings as described above
5. Click **Create** or **Save changes**

## Setting Up Protection via GitHub CLI

```bash
# Install GitHub CLI if not already installed
# See: https://cli.github.com/

# For main branch (run from repository root)
gh repo edit --default-branch main

# Note: Full branch protection requires using GitHub UI or API
# as gh CLI has limited branch protection support
```

## Automated Setup Script

For automated setup, use the GitHub API directly:

```bash
# This requires a GitHub token with repo permissions
# The token is automatically used by gh CLI if logged in

# Check current protection status
gh api repos/Singularity-ng/singularity-language-registry/branches/main/protection

# Apply protection (requires custom API calls)
# See .github/scripts/setup-branch-protection.sh
```