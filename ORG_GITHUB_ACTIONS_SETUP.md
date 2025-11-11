# Organization GitHub Actions Setup

This guide shows how to set up centralized GitHub Actions for the Singularity organization.

## Overview

We'll create 3 repositories in the `Singularity-ng` organization:

1. **`.github`** - Organization defaults and workflow templates
2. **`github-workflows`** - Reusable workflows
3. **`github-actions`** - Composite actions

---

## Step 1: Create `.github` Repository

This repository provides workflow templates that appear in GitHub UI.

### Create Repository

```bash
# On GitHub, create: Singularity-ng/.github (public or private)
git clone git@github.com:Singularity-ng/.github.git
cd .github
```

### Files to Create

See `org-repos-setup/.github/` directory for complete file structure.

**Benefits:**
- Templates appear in "Actions" → "New workflow" in all org repos
- Automatic defaults for issues, PRs, security policies

---

## Step 2: Create `github-workflows` Repository

This repository contains reusable workflows that can be called from any repo.

### Create Repository

```bash
# On GitHub, create: Singularity-ng/github-workflows (public or private)
git clone git@github.com:Singularity-ng/github-workflows.git
cd github-workflows
```

### Files to Create

See `org-repos-setup/github-workflows/` directory for complete file structure.

**Benefits:**
- Single source of truth for CI/CD logic
- Update once, applies to all repos
- Versioned via tags (e.g., `@v1`, `@main`)

---

## Step 3: Create `github-actions` Repository

This repository contains composite actions (reusable action steps).

### Create Repository

```bash
# On GitHub, create: Singularity-ng/github-actions (public or private)
git clone git@github.com:Singularity-ng/github-actions.git
cd github-actions
```

### Files to Create

See `org-repos-setup/github-actions/` directory for complete file structure.

**Benefits:**
- Encapsulate complex setup steps
- Reuse across workflows
- Easier to test and maintain

---

## Step 4: Update Existing Projects

Once the organization repos are created, update projects like `singularity-language-registry`:

### Before (Manual Workflow)
```yaml
jobs:
  ci:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Nix
        uses: DeterminateSystems/nix-installer-action@v12
      # ... many more steps
```

### After (Reusable Workflow)
```yaml
jobs:
  ci:
    uses: Singularity-ng/github-workflows/.github/workflows/rust-nix-ci.yml@v1
    with:
      enable-coverage: true
```

---

## Versioning Strategy

### For `github-workflows` and `github-actions`:

1. **Development**: Use `@main` for active development
2. **Stable**: Create tags for stable versions
   ```bash
   git tag -a v1.0.0 -m "Stable release v1.0.0"
   git push --tags
   ```
3. **In projects**: Reference by version
   ```yaml
   uses: Singularity-ng/github-workflows/.github/workflows/rust-nix-ci.yml@v1.0.0
   # or
   uses: Singularity-ng/github-workflows/.github/workflows/rust-nix-ci.yml@v1  # latest v1.x
   # or
   uses: Singularity-ng/github-workflows/.github/workflows/rust-nix-ci.yml@main  # bleeding edge
   ```

---

## Repository Structure Reference

```
Singularity-ng/
├── .github/                           # Org defaults & templates
│   ├── workflow-templates/
│   │   ├── rust-nix-ci.yml            # Template: CI
│   │   ├── rust-nix-ci.properties.json
│   │   ├── rust-nix-release.yml       # Template: Release
│   │   └── rust-nix-release.properties.json
│   ├── SECURITY.md                    # Org-wide security policy
│   ├── SUPPORT.md                     # Org-wide support info
│   └── README.md
│
├── github-workflows/                  # Reusable workflows
│   ├── .github/workflows/
│   │   ├── rust-nix-ci.yml            # Reusable: CI
│   │   ├── rust-nix-release.yml       # Reusable: Release
│   │   ├── security-audit.yml         # Reusable: Security
│   │   └── docs-deploy.yml            # Reusable: Docs
│   └── README.md
│
└── github-actions/                    # Composite actions
    ├── setup-nix-rust/
    │   ├── action.yml
    │   └── README.md
    ├── generate-release-reports/
    │   ├── action.yml
    │   └── README.md
    ├── build-crate-package/
    │   ├── action.yml
    │   └── README.md
    └── README.md
```

---

## Private Organization Setup

If your organization repositories are **private**, you need to configure access:

### Option A: GitHub App (Recommended for Private Orgs)
1. Create GitHub App with `contents: read` permission
2. Install app in organization
3. Use app for authentication in workflows

### Option B: Personal Access Token
1. Create PAT with `repo` scope
2. Add as organization secret: `ORG_GITHUB_TOKEN`
3. Use in workflows:
   ```yaml
   uses: Singularity-ng/github-workflows/.github/workflows/rust-nix-ci.yml@v1
   secrets:
     GITHUB_TOKEN: ${{ secrets.ORG_GITHUB_TOKEN }}
   ```

### Option C: Make Workflow Repos Public
- Keep source code repos private
- Make `.github`, `github-workflows`, `github-actions` public
- No authentication needed

---

## Migration Steps

### Phase 1: Create Organization Repos (Week 1)
1. Create `Singularity-ng/.github`
2. Create `Singularity-ng/github-workflows`
3. Create `Singularity-ng/github-actions`
4. Populate with files from `org-repos-setup/`

### Phase 2: Test in One Project (Week 2)
1. Update `singularity-language-registry` to use reusable workflows
2. Test all workflows (CI, release, security)
3. Fix any issues

### Phase 3: Roll Out to Other Projects (Week 3+)
1. Update each project one at a time
2. Monitor for issues
3. Iterate on reusable workflows

---

## Maintenance

### Updating Workflows

1. **Make changes** in `github-workflows` repo
2. **Test** in a dev project using `@main`
3. **Tag** new version when stable
4. **Update** projects to use new version

### Monitoring Usage

GitHub provides insights on which repos use your workflows:
- Go to `github-workflows` repository
- Click "Insights" → "Dependency graph" → "Dependents"

---

## Next Steps

1. **Run setup script**: `./setup-org-repos.sh` (creates directory structure)
2. **Create repos on GitHub**: `.github`, `github-workflows`, `github-actions`
3. **Push files**: Copy from `org-repos-setup/` to each repo
4. **Update this project**: Replace workflows with reusable ones
5. **Tag versions**: `v1.0.0` in workflow repos

---

## Benefits Summary

✅ **DRY (Don't Repeat Yourself)** - Write once, use everywhere
✅ **Consistency** - All projects use same CI/CD logic
✅ **Easy updates** - Fix once, all projects benefit
✅ **Versioning** - Control when projects adopt changes
✅ **Discoverability** - Templates appear in GitHub UI
✅ **Testing** - Test changes in one place before rollout
✅ **Maintenance** - Much easier to maintain 3 repos than N projects

---

See `org-repos-setup/` directory for all ready-to-use files.
