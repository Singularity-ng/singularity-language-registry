# GitHub Workflows Setup

The push failed because GitHub OAuth apps need special permissions to create workflow files.
This is a security measure to prevent unauthorized workflow modifications.

## Option 1: Use SSH (Recommended)

```bash
# Check current remote
git remote -v

# Change to SSH
git remote set-url origin git@github.com:Singularity-ng/singularity-language-registry.git

# Push again
git push origin development
```

## Option 2: Use Personal Access Token

1. Go to https://github.com/settings/tokens/new
2. Create token with these scopes:
   - ✅ repo (all)
   - ✅ workflow
3. Use token:
   ```bash
   git push https://<your-token>@github.com/Singularity-ng/singularity-language-registry.git development
   ```

## Option 3: Push via GitHub CLI with correct scope

```bash
# Re-authenticate with workflow scope
gh auth login --scopes "repo,workflow"

# Then push
git push origin development
```

## Option 4: Manual upload

If you can't push workflows directly:
1. Push without workflows:
   ```bash
   git rm -r .github/workflows
   git commit -m "temp: remove workflows"
   git push origin development
   ```

2. Add workflows manually via GitHub UI:
   - Go to repository
   - Click "Actions" tab
   - Click "New workflow"
   - Copy/paste each workflow file

## Files that need workflow scope:
- `.github/workflows/ci.yml`
- `.github/workflows/release.yml`
- `.github/workflows/claude-review.yml`
- `.github/workflows/automated-ops.yml`

Once pushed, the workflows will:
- Run on every push and PR
- Enforce code quality standards
- Automate releases to crates.io
- Enable Claude AI reviews on PRs to main