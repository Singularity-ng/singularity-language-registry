# Claude Auto-Approve Setup Guide

This guide explains how to set up the `ORG_GITHUB_TOKEN` secret needed for auto-approve functionality in the Claude AI review workflow.

## Why is this needed?

GitHub's security model prevents workflows from approving their own PRs using the default `GITHUB_TOKEN`. This is a security feature to prevent malicious workflows from bypassing review requirements.

To enable auto-approve, we need a separate token with approval permissions.

## Option 1: Fine-Grained Organization PAT (Recommended)

This is the most secure approach - it limits permissions to only what's needed for this specific repository.

### Steps:

1. **Navigate to Personal Access Token Settings**
   - Go to: https://github.com/settings/personal-access-tokens/new
   - Or: GitHub → Settings → Developer settings → Personal access tokens → Fine-grained tokens → Generate new token

2. **Configure Token Details**
   - Token name: `Claude Auto-Approve for singularity-language-registry`
   - Expiration: Choose appropriate duration (90 days recommended, with calendar reminder to renew)
   - Description: `Allows Claude AI workflow to approve PRs in singularity-language-registry`
   - Resource owner: Select `Singularity-ng` (the organization)

3. **Set Repository Access**
   - Select: "Only select repositories"
   - Choose: `singularity-language-registry`

4. **Repository Permissions**
   - Set **Pull requests** to: `Read and write`
   - All other permissions can remain at "No access"

5. **Generate Token**
   - Click "Generate token"
   - **IMPORTANT**: Copy the token immediately (it won't be shown again)

6. **Add to Repository Secrets**
   - Go to: https://github.com/Singularity-ng/singularity-language-registry/settings/secrets/actions
   - Click "New repository secret"
   - Name: `ORG_GITHUB_TOKEN`
   - Value: Paste the token you copied
   - Click "Add secret"

### Security Benefits:
- ✅ Scoped to single repository only
- ✅ Limited to pull request permissions only
- ✅ Can be rotated without affecting other systems
- ✅ Expiration forces regular review
- ✅ Can be revoked instantly if compromised

## Option 2: Enable Organization Setting (Less Secure)

Alternatively, you can enable GitHub Actions to approve PRs org-wide:

1. Go to: https://github.com/organizations/Singularity-ng/settings/actions
2. Under "Workflow permissions", enable:
   - "Allow GitHub Actions to create and approve pull requests"

### Trade-offs:
- ❌ Applies to ALL repositories in the organization
- ❌ Any workflow in any repo can now approve PRs
- ✅ Simpler setup (no token management)
- ❌ Less granular control

**Recommendation**: Use Option 1 (Fine-Grained PAT) for better security.

## Verification

After adding the secret, the next PR will:
1. Automatically get a Claude AI review
2. Automatically get approved (if checks pass)
3. Automatically merge when all CI checks are green

You can verify the secret is set by checking:
- Repository Settings → Secrets → Actions → `ORG_GITHUB_TOKEN` should be listed

## Troubleshooting

### Token expired
- Symptoms: Auto-approve suddenly stops working
- Fix: Regenerate token and update the secret

### "Resource not accessible by integration" error
- Symptoms: Workflow fails on approval step
- Fix: Verify the PAT has "Pull requests: Read and write" permission

### Still getting "Can not approve your own pull request"
- Symptoms: Error persists after adding token
- Possible causes:
  - Token not added to secrets correctly
  - Token name is wrong (must be exactly `ORG_GITHUB_TOKEN`)
  - Token doesn't have pull_requests scope
  - Token expired

## Token Rotation Schedule

It's recommended to rotate the token every 90 days:
1. Create new PAT with same permissions (following steps above)
2. Update `ORG_GITHUB_TOKEN` secret with new value
3. Delete old PAT from your personal settings (https://github.com/settings/personal-access-tokens)
4. Set calendar reminder for next rotation

## Additional Resources

- [GitHub Fine-Grained PATs Documentation](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
- [GitHub Actions Security Best Practices](https://docs.github.com/en/actions/security-guides/security-hardening-for-github-actions)
