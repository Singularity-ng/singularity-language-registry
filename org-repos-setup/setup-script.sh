#!/usr/bin/env bash

set -euo pipefail

# Organization GitHub Actions Setup Script
# This script helps you set up the organization repositories

BOLD='\033[1m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BOLD}Singularity Organization GitHub Actions Setup${NC}"
echo ""

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    echo -e "${RED}Error: GitHub CLI (gh) is not installed${NC}"
    echo "Install it from: https://cli.github.com/"
    exit 1
fi

# Check if user is authenticated
if ! gh auth status &> /dev/null; then
    echo -e "${YELLOW}You need to authenticate with GitHub${NC}"
    gh auth login
fi

ORG_NAME="${ORG_NAME:-Singularity-ng}"
echo -e "${BLUE}Organization:${NC} $ORG_NAME"
echo ""

# Function to create repo if it doesn't exist
create_repo() {
    local repo_name=$1
    local description=$2
    local visibility=${3:-private}

    echo -e "${BLUE}Checking repository:${NC} $ORG_NAME/$repo_name"

    if gh repo view "$ORG_NAME/$repo_name" &> /dev/null; then
        echo -e "${YELLOW}  Repository already exists${NC}"
    else
        echo -e "${GREEN}  Creating repository...${NC}"
        gh repo create "$ORG_NAME/$repo_name" \
            --$visibility \
            --description "$description" \
            --enable-wiki=false
        echo -e "${GREEN}  ✓ Created${NC}"
    fi
}

# Function to push files to repo
push_files() {
    local repo_name=$1
    local source_dir=$2

    echo -e "${BLUE}Pushing files to:${NC} $ORG_NAME/$repo_name"

    # Create temp directory
    TEMP_DIR=$(mktemp -d)
    cd "$TEMP_DIR"

    # Clone or init repo
    if gh repo view "$ORG_NAME/$repo_name" &> /dev/null; then
        gh repo clone "$ORG_NAME/$repo_name" .
    else
        git init
        git remote add origin "git@github.com:$ORG_NAME/$repo_name.git"
    fi

    # Copy files
    cp -r "$source_dir"/* . || true

    # Commit and push
    git add .
    if git diff --cached --quiet; then
        echo -e "${YELLOW}  No changes to commit${NC}"
    else
        git commit -m "Initial setup of $repo_name repository"
        git branch -M main
        git push -u origin main
        echo -e "${GREEN}  ✓ Pushed files${NC}"
    fi

    # Cleanup
    cd - > /dev/null
    rm -rf "$TEMP_DIR"
}

# Step 1: Create .github repository
echo -e "\n${BOLD}Step 1: Setting up .github repository${NC}"
echo "This provides organization defaults and workflow templates"
echo ""

read -p "Create/update .github repository? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    create_repo ".github" "Organization defaults and workflow templates" "public"
    push_files ".github" "$(pwd)/.github"
fi

# Step 2: Create github-workflows repository
echo -e "\n${BOLD}Step 2: Setting up github-workflows repository${NC}"
echo "This contains reusable workflows"
echo ""

read -p "Create/update github-workflows repository? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    create_repo "github-workflows" "Reusable GitHub Actions workflows for Rust + Nix projects" "private"
    push_files "github-workflows" "$(pwd)/github-workflows"
fi

# Step 3: Create github-actions repository
echo -e "\n${BOLD}Step 3: Setting up github-actions repository${NC}"
echo "This contains composite actions"
echo ""

read -p "Create/update github-actions repository? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    create_repo "github-actions" "Composite GitHub Actions for Rust + Nix projects" "private"
    push_files "github-actions" "$(pwd)/github-actions"
fi

# Step 4: Tag versions
echo -e "\n${BOLD}Step 4: Tagging versions${NC}"
echo "This creates v1.0.0 tags for versioning"
echo ""

tag_repo() {
    local repo_name=$1
    local version=${2:-v1.0.0}

    echo -e "${BLUE}Tagging:${NC} $ORG_NAME/$repo_name @ $version"

    TEMP_DIR=$(mktemp -d)
    cd "$TEMP_DIR"
    gh repo clone "$ORG_NAME/$repo_name" .

    if git rev-parse "$version" >/dev/null 2>&1; then
        echo -e "${YELLOW}  Tag already exists${NC}"
    else
        git tag -a "$version" -m "Release $version"
        git push origin "$version"
        echo -e "${GREEN}  ✓ Tagged $version${NC}"
    fi

    cd - > /dev/null
    rm -rf "$TEMP_DIR"
}

read -p "Create v1.0.0 tags? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    tag_repo "github-workflows" "v1.0.0"
    tag_repo "github-actions" "v1.0.0"
fi

# Summary
echo -e "\n${BOLD}${GREEN}✓ Setup Complete!${NC}"
echo ""
echo -e "${BOLD}Next Steps:${NC}"
echo "1. Review the repositories on GitHub:"
echo "   - https://github.com/$ORG_NAME/.github"
echo "   - https://github.com/$ORG_NAME/github-workflows"
echo "   - https://github.com/$ORG_NAME/github-actions"
echo ""
echo "2. Update your projects to use the reusable workflows:"
echo "   See ORG_GITHUB_ACTIONS_SETUP.md for migration guide"
echo ""
echo "3. Test in one project before rolling out to all projects"
echo ""
echo -e "${BOLD}Repository Visibility:${NC}"
echo "  .github          - Public (templates visible to all)"
echo "  github-workflows - Private (reusable workflows)"
echo "  github-actions   - Private (composite actions)"
echo ""
echo "If you need to change visibility, use:"
echo "  gh repo edit $ORG_NAME/<repo-name> --visibility <public|private>"
echo ""
echo -e "${GREEN}Happy automating! 🚀${NC}"
