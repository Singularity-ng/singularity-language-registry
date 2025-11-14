#!/usr/bin/env bash
set -euo pipefail

# Install repository git hooks to use the Nix devShell for heavy checks.
# This will create .githooks/pre-push and configure git to use .githooks as
# the hooks directory (via core.hooksPath). Running the pre-push hook will
# prefer `nix develop --command just pre-push` so native deps like OpenSSL
# are available when clippy and the build run.

HOOK_DIR=".githooks"
mkdir -p "$HOOK_DIR"

cat > "$HOOK_DIR/pre-push" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

# Pre-push hook that runs the project's pre-push task. If Nix is available
# we'll run the command inside the devShell so native libraries are provided.
if command -v nix >/dev/null 2>&1; then
  echo "🏗️  Running pre-push checks inside Nix devShell (nix develop)..."
  nix develop --command just pre-push
else
  echo "⚠️  Nix not found; running pre-push via just (may fail if native deps missing)..."
  just pre-push
fi
EOF

chmod +x "$HOOK_DIR/pre-push"

# Configure git to use the .githooks directory (safe to re-run)
git config core.hooksPath "$HOOK_DIR" || true

echo "Installed git hooks to $HOOK_DIR and set core.hooksPath."

echo "Tip: run 'nix develop' or 'direnv allow' to enter the devShell before heavy builds." 

exit 0
#!/bin/bash

# Setup git hooks for the Singularity Language Registry

set -e

echo "🔧 Setting up Git hooks for code quality..."

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Make hooks executable
chmod +x .githooks/pre-commit
chmod +x .githooks/pre-push

# Configure git to use our hooks directory
git config core.hooksPath .githooks

# Set up commit message template
git config commit.template .gitmessage

# Configure additional git settings
git config branch.autoSetupRebase always
git config pull.rebase true
git config fetch.prune true

# Set up branch protection locally
git config branch.main.pushRemote no_push
git config branch.main.merge refs/heads/main

echo -e "${GREEN}✅ Git hooks installed successfully!${NC}"
echo ""
echo "📋 Configured hooks:"
echo "  • pre-commit: Runs formatting, clippy, tests, and security checks"
echo "  • pre-push: Prevents direct push to main, runs comprehensive tests"
echo ""
echo "🔒 Protection enabled:"
echo "  • Direct push to main branch blocked"
echo "  • All commits must pass quality checks"
echo "  • Commit messages follow conventional format"
echo ""
echo "📝 Commit message format:"
echo "  type(scope): description"
echo "  Types: feat, fix, docs, style, refactor, test, chore, perf, ci, build"
echo ""
echo -e "${YELLOW}⚠️  Note: To bypass hooks in emergency (not recommended):${NC}"
echo "  git commit --no-verify"
echo "  git push --no-verify"