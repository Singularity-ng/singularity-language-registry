# Real Claude Integration - Complete Setup

## What's Now Configured

You're using the **official Claude Code GitHub Action** (not simulated!), integrated via OAuth token.

### Two Claude Modes

#### 1. Interactive Claude (`@claude` mentions)
**Trigger**: Mention `@claude` in PR comments or reviews
**Action**: `anthropics/claude-code-action@v1`
**Auth**: `CLAUDE_CODE_OAUTH_TOKEN` secret

**Usage**:
```
# In any PR comment
@claude can you review this function?
@claude fix the clippy warnings
@claude explain how this works
```

Claude will respond directly in the PR thread!

#### 2. Automated Review (on PR open)
**Trigger**: PR opened/updated targeting `main`
**Action**: `anthropics/claude-code-action@v1`
**Auth**: `CLAUDE_CODE_OAUTH_TOKEN` secret

**What it does**:
1. Runs scope check (stale files, out-of-scope changes)
2. Calls real Claude to review the code
3. Claude analyzes:
   - Rust code quality
   - Security vulnerabilities
   - Test coverage
   - Documentation
   - Relevance to language registry
4. Posts review comments
5. Auto-approves if clean (no scope issues)
6. Enables auto-merge
7. Merges when CI passes

## When Does Claude Approve?

### Current Criteria
Claude auto-approves when:
- **No scope issues** (no stale files, no out-of-scope changes)
- **Claude's review is positive** (says "LGTM" or similar)

Claude blocks (human review required) when:
- Files older than 2 years detected
- Binary/temp files included
- Changes unrelated to language registry
- Claude identifies code quality issues
- Security concerns raised

## Authentication

You have `CLAUDE_CODE_OAUTH_TOKEN` configured (via `/install-github-app`).

The workflows will try:
1. `ANTHROPIC_API_KEY` (if you add it later)
2. `CLAUDE_CODE_OAUTH_TOKEN` (what you have now)

## How to Use

### Automatic (No Action Needed)
Just push to a feature branch:
```bash
git push origin feat/my-feature
```

Auto-PR creates → Claude reviews automatically → Approves if clean → Merges when CI passes

### Interactive (Ask Claude Anything)
In any PR, comment:
```
@claude review this code
@claude can you explain the changes in src/parser.rs?
@claude fix the test failures
```

Claude will respond and can even make code changes!

## What Runs on Each PR

1. **Nix Checks** (CI) - 1 job, ~2-3 minutes
   - Build, test, lint, format, audit, docs

2. **Scope Check** (Custom) - ~10 seconds
   - Stale file detection
   - Out-of-scope file detection
   - Relevance validation

3. **Claude Review** (Real AI) - ~30-60 seconds
   - Full code analysis
   - Security review
   - Best practices check
   - Auto-approve decision

**Total: ~3-4 minutes from push to merge** (if everything passes)

## CI Load Comparison

### Before
- 25+ checks
- Multiple OS matrix
- Every PR runs everything
- 10+ minutes

### After
- 1 Nix check (Ubuntu only on PRs)
- Full matrix only on main
- Claude review (~60s)
- **3-4 minutes total**

**75% reduction in CI time and cost!**

## Example Workflows

### Quick Fix - Fully Automated
```bash
git checkout -b fix/typo
echo "fix" >> README.md
git commit -am "fix: correct typo in README"
git push origin fix/typo
```

Result:
- Auto-PR creates (10s)
- Scope check passes (10s)
- Claude reviews and approves (60s)
- CI passes (2-3 min)
- **Auto-merged in ~3-4 minutes!**

### Feature with Claude Help
```bash
git checkout -b feat/new-language
# ... make changes ...
git push origin feat/new-language
```

PR opens, then comment:
```
@claude review this implementation
```

Claude responds with detailed feedback. You can iterate:
```
@claude fix the issues you found
```

Claude makes the changes!

### Complex Change
```bash
git checkout -b refactor/parser
# ... major changes ...
git push origin refactor/parser
```

PR opens:
- Scope check runs
- Claude reviews
- If issues → Claude comments, no auto-approve
- You address feedback
- Push again → Claude re-reviews
- Clean → Auto-approves → Merges

## Configuration Files

### `.github/workflows/claude-review.yml`
- `claude-interactive` job: Responds to @claude mentions
- `claude-review` job: Auto-review on PR open
- Both use real `anthropics/claude-code-action@v1`

### Secrets Used
- `CLAUDE_CODE_OAUTH_TOKEN` - Your OAuth token (installed via `/install-github-app`)
- `GITHUB_TOKEN` - Auto-provided by GitHub Actions

## Customization

### Change Claude's Instructions
Edit `.github/workflows/claude-review.yml`:

```yaml
prompt: |
  Your custom instructions here
  Focus on X, Y, Z
  Be strict about...
```

### Change Model
```yaml
claude_args: "--max-turns 5 --model claude-opus-4"  # Use Opus instead
```

### Adjust Auto-Approve Logic
Edit the "Determine auto-approve" step to add more conditions.

## Troubleshooting

### Claude not responding to @claude?
- Check `CLAUDE_CODE_OAUTH_TOKEN` is set
- Verify GitHub App is installed
- Check workflow run logs

### Auto-review not running?
- Ensure PR targets `main` branch
- Check workflow triggers in claude-review.yml
- Verify GitHub Actions enabled

### Claude gives wrong feedback?
- Improve the `prompt` in the workflow
- Add more context about your project
- Use `--max-turns` to give Claude more time

## Next Steps

1. **Test it!** Create a PR and see Claude in action
2. **Try @claude** in a PR comment
3. **Refine prompts** based on Claude's reviews
4. **Adjust auto-approve** criteria if needed

---

**You now have real Claude AI reviewing your code and auto-merging clean PRs!**
