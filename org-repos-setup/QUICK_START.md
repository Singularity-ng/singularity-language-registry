# Quick Start - Organization GitHub Actions

Get your organization GitHub Actions set up in 5 minutes.

## Prerequisites

- [ ] GitHub CLI (`gh`) installed - [Download](https://cli.github.com/)
- [ ] Authenticated with GitHub - Run `gh auth login`
- [ ] Organization admin access to `Singularity-ng`

## Step 1: Run Setup Script (2 minutes)

```bash
cd org-repos-setup
./setup-script.sh
```

This will:
1. Create 3 repositories (`.github`, `github-workflows`, `github-actions`)
2. Push all files to each repository
3. Create `v1.0.0` tags for versioning

## Step 2: Update This Project (3 minutes)

Replace the current CI and release workflows with reusable ones:

### Replace `.github/workflows/ci.yml`

```yaml
name: CI

on:
  push:
    branches: [main, development]
  pull_request:
    branches: [main, development]

concurrency:
  group: ${{ github.workflow }}-${{ github.event.pull_request.number || github.ref }}
  cancel-in-progress: true

permissions:
  contents: read

jobs:
  ci:
    uses: Singularity-ng/github-workflows/.github/workflows/rust-nix-ci.yml@v1
    with:
      enable-coverage: true
      rust-version: stable
```

### Replace `.github/workflows/release.yml`

```yaml
name: Release

on:
  push:
    tags: ['v[0-9]+.*']
  workflow_dispatch:
    inputs:
      version:
        description: 'Version to release (e.g., 0.2.0)'
        required: true
        type: string

permissions:
  contents: write
  security-events: write

jobs:
  release:
    uses: Singularity-ng/github-workflows/.github/workflows/rust-nix-release.yml@v1
    with:
      enable-crate-publish: false
      enable-platform-binaries: true
      enable-release-reports: true
```

## Step 3: Test (1 minute)

```bash
# Trigger CI
git add .github/workflows/
git commit -m "Switch to reusable workflows"
git push

# Watch it run
gh run watch
```

## What You Get

### Before (Manual)
- ❌ 494 lines of workflow YAML
- ❌ Duplicate logic in every project
- ❌ Hard to maintain
- ❌ Inconsistent across projects

### After (Reusable)
- ✅ ~30 lines of workflow YAML
- ✅ Single source of truth
- ✅ Easy to maintain
- ✅ Consistent across all projects

### Reduction
- **94% less workflow code** (494 lines → 30 lines)
- **3 organization repos** to maintain instead of N projects
- **Versioned workflows** - control when updates apply

## Repository Structure

```
Singularity-ng/
├── .github/                    # Templates visible in GitHub UI
├── github-workflows/           # Reusable workflows (private)
└── github-actions/             # Composite actions (private)
```

## Using in Other Projects

Once set up, any new Rust + Nix project can use:

```yaml
# .github/workflows/ci.yml
jobs:
  ci:
    uses: Singularity-ng/github-workflows/.github/workflows/rust-nix-ci.yml@v1

# .github/workflows/release.yml
jobs:
  release:
    uses: Singularity-ng/github-workflows/.github/workflows/rust-nix-release.yml@v1
```

That's it! No need to copy 500 lines of YAML.

## Creating New Workflows in GitHub UI

1. Go to any repo → Actions → New workflow
2. Look for "By Singularity-ng" section
3. Click "Set up this workflow" on a template
4. Customize if needed
5. Commit

The templates from `.github/workflow-templates/` will appear automatically.

## Updating Workflows

When you need to update the CI/release logic:

1. **Update in one place**: `github-workflows` repository
2. **Test**: Use `@main` in a test project
3. **Tag**: Create new version tag (e.g., `v1.1.0`)
4. **Roll out**: Update projects to use `@v1.1.0` (or they auto-update if using `@v1`)

## Troubleshooting

### Script fails with permission error
```bash
# Make sure you're org admin
gh api orgs/Singularity-ng/memberships/$USER
```

### Workflows not appearing in UI
- Check `.github` repository is created
- Check it has `workflow-templates/` directory
- Wait a few minutes for GitHub to sync

### Reusable workflow not found
- Check repository visibility (must be accessible to calling repo)
- Check path is correct: `org/repo/.github/workflows/file.yml@ref`
- Check tag exists: `gh release list -R Singularity-ng/github-workflows`

## Next Steps

- [ ] Test CI in this project
- [ ] Test release in this project
- [ ] Roll out to next project
- [ ] Document any custom needs

## Documentation

- **Full setup guide**: `ORG_GITHUB_ACTIONS_SETUP.md`
- **Reusable workflows**: `github-workflows/README.md`
- **Composite actions**: `github-actions/README.md`

## Questions?

See the organization's `SUPPORT.md` in the `.github` repository.

---

**Total setup time: ~5 minutes**

**Time saved per project: ~2 hours of workflow development**

**ROI: 24x after 3rd project**
