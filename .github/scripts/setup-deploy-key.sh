#!/bin/bash

# Setup SSH Deploy Key for bypassing workflow protection
# This allows automated deployments and CI/CD to push directly

set -e

echo "🔐 Setting up SSH deploy key for automated operations..."

# Generate SSH key if it doesn't exist
KEY_PATH="$HOME/.ssh/singularity-deploy"
if [ ! -f "$KEY_PATH" ]; then
    echo "Generating new SSH key..."
    ssh-keygen -t ed25519 -f "$KEY_PATH" -N "" -C "singularity-deploy@github-actions"
fi

echo ""
echo "📋 Public key to add as Deploy Key in GitHub:"
echo "================================================"
cat "${KEY_PATH}.pub"
echo "================================================"
echo ""

# Create SSH config for this repository
echo "Setting up SSH config..."
cat >> ~/.ssh/config << EOF

# Singularity Language Registry Deploy
Host github-singularity-deploy
    HostName github.com
    User git
    IdentityFile ${KEY_PATH}
    IdentitiesOnly yes
    StrictHostKeyChecking no
EOF

echo ""
echo "📝 Instructions to complete setup:"
echo ""
echo "1. Go to: https://github.com/Singularity-ng/singularity-language-registry/settings/keys"
echo ""
echo "2. Click 'Add deploy key'"
echo ""
echo "3. Title: 'Automated Deploy Key (CI/CD)'"
echo ""
echo "4. Key: Paste the public key shown above"
echo ""
echo "5. Check: ✅ Allow write access"
echo ""
echo "6. Click 'Add key'"
echo ""
echo "7. Update your git remote to use SSH:"
echo "   git remote set-url origin git@github-singularity-deploy:Singularity-ng/singularity-language-registry.git"
echo ""
echo "8. For GitHub Actions, add the private key as a secret:"
echo "   - Secret name: DEPLOY_SSH_KEY"
echo "   - Secret value: Contents of ${KEY_PATH}"
echo ""

# Show how to use in GitHub Actions
cat > .github/workflows/deploy-with-ssh.yml.example << 'WORKFLOW'
name: Deploy with SSH

on:
  workflow_dispatch:
  push:
    branches: [development]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - name: Setup SSH
        run: |
          mkdir -p ~/.ssh
          echo "${{ secrets.DEPLOY_SSH_KEY }}" > ~/.ssh/deploy_key
          chmod 600 ~/.ssh/deploy_key
          cat >> ~/.ssh/config <<EOF
          Host github.com
            HostName github.com
            User git
            IdentityFile ~/.ssh/deploy_key
            IdentitiesOnly yes
            StrictHostKeyChecking no
          EOF

      - name: Checkout with SSH
        run: |
          git clone git@github.com:Singularity-ng/singularity-language-registry.git .
          git config user.name "GitHub Actions Deploy"
          git config user.email "deploy@github-actions"

      - name: Make changes and push
        run: |
          # Your deployment steps here
          echo "Deployment at $(date)" >> deploy.log
          git add .
          git commit -m "Automated deployment [skip ci]" || true
          git push origin main
WORKFLOW

echo "✅ Example workflow created at: .github/workflows/deploy-with-ssh.yml.example"