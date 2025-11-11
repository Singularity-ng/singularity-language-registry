# Fully Automated PR → Merge Flow

## The Magic Workflow

```
┌─────────────────────────────────────────────────────────────────┐
│  YOU: git push origin development                               │
└──────────────────┬──────────────────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────────────────┐
│  AUTO-PR: Creates PR with AI-generated description              │
│  → Analyzes commits, diffs, changed files                       │
│  → Generates smart PR description                               │
│  → Opens PR targeting main                                      │
└──────────────────┬──────────────────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────────────────┐
│  CLAUDE AI: Reviews the code                                    │
│  ✓ Runs clippy (Rust linting)                                   │
│  ✓ Checks for security vulnerabilities                          │
│  ✓ Validates tests                                              │
│  ✓ Reviews documentation                                        │
└──────────────────┬──────────────────────────────────────────────┘
                   │
         ┌─────────┴─────────┐
         │                   │
         ▼                   ▼
    ✅ PASS              ⚠️ ISSUES
         │                   │
         ▼                   ▼
┌──────────────────┐   ┌──────────────────┐
│ Claude APPROVES  │   │ Claude COMMENTS  │
│ Auto-merge ON    │   │ Human needed     │
└────────┬─────────┘   └──────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────────────────┐
│  CI CHECKS: Run test, lint, build                               │
└──────────────────┬──────────────────────────────────────────────┘
                   │
                   ▼
            ✅ ALL PASS
                   │
                   ▼
┌─────────────────────────────────────────────────────────────────┐
│  🚀 AUTO-MERGE: PR merged to main automatically!                │
└─────────────────────────────────────────────────────────────────┘
```

## What This Means

### Zero-Touch Deployment
If your code is good (passes Claude's checks + CI), you just push and forget!

### Safety Nets
- Branch protection prevents bad code from reaching main
- Claude catches issues early
- CI validates everything works
- All conversations must be resolved

### Manual Override
If Claude flags issues but you disagree, a human can still approve and merge.

## Current Configuration

### Branch Protection (main)
- ✅ Required checks: test, lint, build
- ✅ Required approval: 1 (can be from Claude!)
- ✅ No force pushes
- ✅ No deletions
- ✅ Admins must follow rules
- ✅ Conversations must be resolved

### Auto-PR Triggers
- `development` branch
- `feat/*` branches
- `feature/*` branches
- `fix/*` branches
- `chore/*` branches

### Claude Review Criteria
Claude auto-approves when:
- No clippy warnings
- No security vulnerabilities
- Tests are present
- Documentation is adequate

If ANY of these fail → human review required

## Example Flow

```bash
# You make changes on development
git add .
git commit -m "feat: add new language support"
git push origin development

# 30 seconds later... PR is created
# 1 minute later... Claude approves
# 2 minutes later... CI passes
# 2.5 minutes later... MERGED! ✅
```

That's it! You just pushed and everything else happened automatically.

## Tips

1. **Write good commit messages** - they become PR descriptions
2. **Keep PRs focused** - easier for Claude to review
3. **Run tests locally** - faster feedback than waiting for CI
4. **Check Claude's review** - even when it approves, learn from it

## Troubleshooting

**PR not created?**
- Check you're on a tracked branch (development, feat/*, etc.)
- Verify GitHub Actions are enabled

**Claude didn't approve?**
- Check the review comments
- Fix the issues
- Push again (Claude re-reviews automatically)

**CI checks failing?**
- Even with Claude's approval, CI must pass
- PR won't merge until all checks are green

**Want to disable auto-merge?**
- Remove the "Enable auto-merge" step from claude-review.yml
- Or just use `gh pr merge --disable-auto` on specific PRs
