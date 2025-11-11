# GitHub Workflow Guide

## Branch Strategy

```
main (protected)
  ↑
  PR (auto-created, Claude reviews)
  ↑
feat/your-feature (work here!)
```

### Branch Purposes

- **main** - Production, fully protected, requires PR + approval
- **development** - Stable staging (optional, for manual testing)
- **feat/**, **fix/**, **chore/** - Your work branches

## Recommended Workflow

### Option 1: Direct Feature Branch → Main (Recommended)

**Fastest and cleanest!**

```bash
# Create feature branch from main
git checkout main
git pull
git checkout -b feat/my-feature

# Make changes
# ... edit files ...

# Commit and push
git add .
git commit -m "feat: add awesome feature"
git push origin feat/my-feature

# Auto-PR creates PR to main
# Claude reviews automatically
# If approved + CI passes → Auto-merges!
```

**What runs:**
- Push to `feat/*` → Auto-PR creates PR
- PR opened → CI runs (1 job)
- PR opened → Claude reviews
- If clean → Auto-approves + auto-merges
- **Total: ~2-3 minutes from push to merge**

### Option 2: Feature → Development → Main

**For larger changes that need staging:**

```bash
# Create feature branch
git checkout -b feat/my-feature

# Make changes and push
git push origin feat/my-feature

# Manually merge to development for testing
gh pr create --base development --head feat/my-feature
# ... test in development ...

# Then create PR to main
gh pr create --base main --head development
# ... auto-review and merge ...
```

**What runs:**
- Push to development → CI runs
- PR to main → CI + Claude review + auto-merge

## What Runs Where

### Push to `main`
- Full CI (Ubuntu + macOS)
- Code coverage
- MSRV check
- All quality gates

### Push to `development`
- CI runs (1 Nix check)
- **No auto-PR created**
- Good for testing before main

### Push to `feat/*`, `fix/*`, etc.
- **Auto-PR creates** PR to main
- CI runs on the PR (1 job)
- Claude reviews automatically
- If clean → Auto-merges

### PR to `main`
- CI (1 Nix check on Ubuntu)
- Claude review with:
  - Stale file detection
  - Scope validation
  - Clippy
  - Security audit
- Auto-merge if approved

## Quick Commands

### Start New Feature
```bash
git checkout main
git pull
git checkout -b feat/my-feature
```

### Push and Auto-PR
```bash
git add .
git commit -m "feat: description"
git push origin feat/my-feature
# Auto-PR handles the rest!
```

### Manual PR (if auto-PR disabled)
```bash
gh pr create --base main --head feat/my-feature --fill
```

### Check PR Status
```bash
gh pr status
gh pr checks
```

### Merge to Development First (Optional)
```bash
# From feature branch
gh pr create --base development --fill
# After testing in dev, create PR to main
git checkout development
git pull
gh pr create --base main --head development --fill
```

## CI Load Summary

### Feature Branch Push
- **0 CI jobs** (waits for PR)

### PR Created
- **1 CI job** (Nix Checks Ubuntu)
- Claude review
- **~2-3 minutes**

### Merged to Main
- **2 CI jobs** (Ubuntu + macOS)
- Code coverage
- MSRV
- **~5-7 minutes**

## Auto-PR Behavior

### Creates PR When:
- You push to `feat/*`, `fix/*`, `chore/*`, etc.
- No existing PR for that branch
- Branch is not `main`

### PR Description Includes:
- Commit messages
- Changed files summary
- Testing checklist
- AI-generated summary

### Checks if PR Exists
- Won't create duplicates
- New commits update existing PR

## Claude Auto-Merge

### Approves When:
- No stale files (< 2 years old)
- No out-of-scope files
- No clippy warnings
- No security vulnerabilities
- Score ≥ 3

### Blocks When:
- Stale/old files detected
- Out-of-scope changes (binaries, temp files)
- Clippy warnings
- Security issues
- Changes unrelated to language registry

## Examples

### Quick Fix
```bash
git checkout -b fix/typo
# ... fix typo ...
git commit -am "fix: correct spelling in README"
git push origin fix/typo
# ✨ Auto-PR → Claude approves → Merged in ~3 min
```

### New Feature
```bash
git checkout -b feat/new-language
# ... implement feature ...
git add .
git commit -m "feat: add support for Zig language"
git push origin feat/new-language
# ✨ Auto-PR → Claude reviews → Approves if clean
```

### Breaking Change
```bash
git checkout -b feat/breaking-api
# ... make changes ...
git commit -m "feat!: redesign API structure"
git push origin feat/breaking-api
# ✨ Auto-PR → Claude reviews → May need human review
```

## Disabling Auto Features

### Disable Auto-PR for One Branch
Just push to `development` instead:
```bash
git push origin development
# No auto-PR, just CI
```

### Disable Auto-Merge for One PR
```bash
gh pr merge <number> --disable-auto
```

### Disable Auto-PR Entirely
Remove or comment out `.github/workflows/auto-pr.yml`

## Tips

1. **Use descriptive branch names**: `feat/add-python-support` not `my-branch`
2. **Commit often**: Better PR descriptions
3. **Push early**: Get CI feedback fast
4. **Trust Claude**: If it approves, it's probably good
5. **Review Claude's comments**: Even when approved, learn from it
6. **Keep PRs focused**: Easier to review and merge
7. **Use conventional commits**: `feat:`, `fix:`, `docs:`, `chore:`

## Troubleshooting

**Auto-PR not created?**
- Check branch name matches patterns (feat/*, fix/*, etc.)
- Verify you're not on main or development

**CI not running?**
- Check `.github/workflows/ci.yml` triggers
- Verify GitHub Actions enabled

**Claude didn't approve?**
- Read review comments
- Fix issues and push again
- Check for stale/out-of-scope files

**PR won't merge?**
- Ensure all conversations resolved
- Check required CI status
- Verify approval exists

---

**Recommended**: Use feature branches with auto-PR for fastest workflow!
