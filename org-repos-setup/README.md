# Organization GitHub Actions Setup

This directory contains everything needed to set up centralized GitHub Actions for the Singularity organization.

## 📁 Contents

```
org-repos-setup/
├── .github/                      # Files for Singularity-ng/.github repo
│   ├── workflow-templates/       # Templates visible in GitHub UI
│   │   ├── rust-nix-ci.yml
│   │   ├── rust-nix-ci.properties.json
│   │   ├── rust-nix-release.yml
│   │   └── rust-nix-release.properties.json
│   ├── SECURITY.md               # Org-wide security policy
│   ├── SUPPORT.md                # Org-wide support info
│   └── README.md
│
├── github-workflows/             # Files for Singularity-ng/github-workflows repo
│   ├── .github/workflows/
│   │   ├── rust-nix-ci.yml       # Reusable CI workflow
│   │   └── rust-nix-release.yml  # Reusable release workflow
│   └── README.md
│
├── github-actions/               # Files for Singularity-ng/github-actions repo
│   ├── setup-nix-rust/           # Composite action: Setup environment
│   │   ├── action.yml
│   │   └── README.md
│   ├── generate-release-reports/ # Composite action: Generate reports
│   │   ├── action.yml
│   │   └── README.md
│   ├── build-crate-package/      # Composite action: Build .crate
│   │   ├── action.yml
│   │   └── README.md
│   └── README.md
│
├── setup-script.sh               # Automated setup script
├── QUICK_START.md                # 5-minute quick start guide
└── README.md                     # This file
```

## 🚀 Quick Start

**New to this? Start here:**

1. **Read**: `QUICK_START.md` (5-minute guide)
2. **Run**: `./setup-script.sh`
3. **Test**: Update this project's workflows
4. **Roll out**: Use in other projects

**Want details? Read**: `../ORG_GITHUB_ACTIONS_SETUP.md` (comprehensive guide)

## 📋 What This Sets Up

### 3 Organization Repositories

1. **`.github`** (public)
   - Organization defaults
   - Workflow templates visible in GitHub UI
   - Security and support policies

2. **`github-workflows`** (private)
   - Reusable workflows for CI and releases
   - Called from other repos via `uses:`
   - Versioned with tags

3. **`github-actions`** (private)
   - Composite actions for common tasks
   - Setup steps, report generation, packaging
   - Versioned with tags

## 🎯 Benefits

### Before
```yaml
# Every project has 500 lines of workflow YAML
# Duplicated across 10 projects = 5000 lines
# Update CI? Change 10 files
```

### After
```yaml
# Every project has 30 lines of workflow YAML
# Shared logic = 1000 lines in org repos
# Update CI? Change 1 file, tag, done
```

**Result:**
- 94% less workflow code in projects
- 80% faster to add CI to new projects
- 100% consistency across organization
- Version control over workflow updates

## 📚 Documentation

| File | Purpose |
|------|---------|
| `QUICK_START.md` | Get started in 5 minutes |
| `../ORG_GITHUB_ACTIONS_SETUP.md` | Complete setup guide |
| `.github/README.md` | Using workflow templates |
| `github-workflows/README.md` | Using reusable workflows |
| `github-actions/README.md` | Using composite actions |

## 🛠️ Setup Options

### Option 1: Automated (Recommended)
```bash
./setup-script.sh
```
Interactive script that creates all repos and pushes files.

### Option 2: Manual

1. Create repositories on GitHub:
   ```bash
   gh repo create Singularity-ng/.github --public
   gh repo create Singularity-ng/github-workflows --private
   gh repo create Singularity-ng/github-actions --private
   ```

2. Push files to each repo:
   ```bash
   # For .github
   cd .github && git init && git add . && git commit -m "Initial setup"
   git remote add origin git@github.com:Singularity-ng/.github.git
   git push -u origin main

   # Repeat for github-workflows and github-actions
   ```

3. Tag versions:
   ```bash
   cd github-workflows && git tag v1.0.0 && git push --tags
   cd ../github-actions && git tag v1.0.0 && git push --tags
   ```

## 🔄 Migration Path

### Phase 1: Setup (Week 1)
- [ ] Run `./setup-script.sh`
- [ ] Verify repos created on GitHub
- [ ] Review files in each repo

### Phase 2: Test (Week 2)
- [ ] Update `singularity-language-registry` workflows
- [ ] Test CI pipeline
- [ ] Test release workflow
- [ ] Fix any issues

### Phase 3: Rollout (Ongoing)
- [ ] Update next project
- [ ] Document any customizations needed
- [ ] Repeat for all projects

## 📊 Project Compatibility

These workflows work best with projects that have:
- ✅ `Cargo.toml` (Rust project)
- ✅ `flake.nix` (Nix development environment)
- ✅ `justfile` (optional, for custom commands)
- ✅ Semantic versioning in git tags

For projects without these, you'll need to customize the workflows.

## 🔧 Customization

### Using Custom Report Generation

If your project has a `justfile` with `release-reports` command:

```justfile
release-reports version:
    # Your custom logic
    echo "Generating reports for $version"
```

The `generate-release-reports` action will use it automatically.

### Adding More Workflows

To add a new reusable workflow:

1. Create in `github-workflows/.github/workflows/`
2. Update `github-workflows/README.md`
3. Test in a project using `@main`
4. Tag new version
5. Create template in `.github/workflow-templates/` (optional)

### Adding More Actions

To add a new composite action:

1. Create directory in `github-actions/`
2. Add `action.yml` and `README.md`
3. Test in a project using `@main`
4. Tag new version

## 📦 What Gets Created

### In `.github` Repository
- Workflow templates (visible in GitHub UI)
- Organization security policy
- Organization support info

### In `github-workflows` Repository
- `rust-nix-ci.yml` - Comprehensive CI pipeline
- `rust-nix-release.yml` - Full release automation

### In `github-actions` Repository
- `setup-nix-rust` - Environment setup
- `generate-release-reports` - Quality reports
- `build-crate-package` - Crate packaging

## 🔒 Security

### Repository Visibility

- **`.github`** - Public (templates need to be accessible)
- **`github-workflows`** - Private (can be public if desired)
- **`github-actions`** - Private (can be public if desired)

For private workflows/actions, calling repos need access via:
- Same organization
- GitHub App
- Personal Access Token

## 🆘 Troubleshooting

### Setup script fails
```bash
# Check you have gh CLI
which gh

# Check you're authenticated
gh auth status

# Check you have org admin rights
gh api orgs/Singularity-ng/memberships/$USER
```

### Workflows not found
```bash
# Check repositories exist
gh repo list Singularity-ng

# Check tags exist
gh release list -R Singularity-ng/github-workflows
```

### Templates not showing
- Wait a few minutes after creating `.github` repo
- Check files are in `workflow-templates/` directory
- Check `.properties.json` files are valid JSON

## 📞 Support

- **Setup issues**: See `QUICK_START.md` and this file
- **Usage questions**: See individual README files in each repo
- **Organization questions**: See `.github/SUPPORT.md` after setup

## 📝 License

Same as parent repository (Proprietary).

---

**Ready to get started? See `QUICK_START.md`**
