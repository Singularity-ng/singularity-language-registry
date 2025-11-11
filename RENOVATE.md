# Renovate Bot Configuration

Automated dependency management for the Singularity Language Registry using Renovate Bot.

## Overview

Renovate automatically creates pull requests to update dependencies when new versions are released. It's configured to:
- Target the `development` branch
- Auto-merge safe updates (patches, security fixes)
- Group related updates (serde, tree-sitter, tokio ecosystems)
- Schedule updates for weekends to minimize disruption
- Support private crates from GitHub Enterprise

## Configuration

**Location**: `renovate.json5` (root directory)

The configuration uses JSON5 format which supports comments for better maintainability.

## Validation

Validate the configuration locally before committing:

```bash
# Enter the Nix development shell
nix develop

# Validate renovate.json5
renovate-config-validator renovate.json5
```

## Update Schedule

| Type | Schedule | Auto-merge |
|------|----------|-----------|
| Security patches | Immediate (any time) | ✅ Yes |
| Patch updates | Every weekend | ✅ Yes (after 3 days) |
| Dev dependencies | Every weekend | ✅ Yes |
| Minor updates | Every weekend | ❌ No (manual review) |
| Major updates | Every weekend | ❌ No (manual review) |
| Lock file maintenance | Monday before 4am | ✅ Yes |
| Nix flake inputs | Monthly | ❌ No |

## Package Grouping

Related packages are grouped into single PRs:

- **Serde ecosystem**: All `serde-*` packages
- **Tree-sitter parsers**: All `tree-sitter-*` packages (manual review required)
- **Tokio ecosystem**: `tokio`, `hyper`, `tower` and related
- **GitHub Actions**: All action updates

## Auto-merge Rules

Renovate will automatically merge PRs when:

1. **Patch updates** (e.g., 1.2.3 → 1.2.4)
   - Minimum release age: 3 days
   - All CI checks pass
   - Internal checks filter: strict

2. **Security updates**
   - Minimum release age: 0 days
   - High priority (merged immediately)
   - Labeled with `security` and `urgent`

3. **Dev dependencies**
   - Minor and patch updates
   - Auto-merge schedule: After 10am Sunday

4. **Lock file maintenance**
   - Updates `Cargo.lock` without changing `Cargo.toml`
   - Schedule: Monday before 4am

## Manual Review Required

The following updates require manual approval:

- ❌ Major version updates (breaking changes)
- ❌ MSRV-sensitive packages (rust, cargo, clippy, rustfmt)
- ❌ Tree-sitter parser updates (compatibility sensitive)
- ❌ Private crates from GitHub Enterprise
- ❌ Nix flake inputs

## Dependency Dashboard

Renovate creates a "Dependency Dashboard" issue in your repository showing:
- Pending updates
- Rate-limited updates
- Errors and warnings
- Dependency graph

**Dashboard location**: GitHub Issues → "🤖 Dependency Dashboard"

## Integration with CI

Renovate PRs automatically trigger the CI pipeline (`.github/workflows/ci.yml`):

1. **Matrix testing** across stable, beta, nightly Rust
2. **Platform testing** on Ubuntu, macOS, Windows
3. **Code quality checks**: clippy, rustfmt, audit
4. **Nix builds** with triple-layer caching
5. **Coverage reports** via tarpaulin + Codecov

### Auto-merge Requirements

For auto-merge to work, PRs must:
- ✅ Pass all CI checks
- ✅ Pass format checks
- ✅ Pass clippy (pedantic mode)
- ✅ Pass security audit
- ✅ Pass all tests
- ✅ Meet minimum release age

## Private Crate Support (GitHub Enterprise)

Configuration includes support for private crates:

```json5
{
  "description": "🔒 Private crates from GitHub Packages",
  "matchDatasources": ["crate"],
  "matchPackagePatterns": ["@Singularity-ng/*"],
  "registryUrls": ["https://github.com/Singularity-ng/_cargo-index/"],
  "labels": ["internal"],
  "automerge": false
}
```

Private crates are:
- Labeled with `internal`
- Never auto-merged (always require review)
- Fetched from GitHub Packages registry

## Rate Limiting

To prevent spam, Renovate is configured with:
- **Max PRs at once**: 5
- **Max PRs per hour**: 2
- **Max branches updating**: 3

## Commit Message Format

All Renovate commits follow conventional commits:

```
chore(deps): update serde to v1.0.200

Co-Authored-By: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>
```

## GitHub Configuration

### Required GitHub Secrets

Renovate uses GitHub's built-in token (no additional secrets needed):
- `GITHUB_TOKEN` - Automatically provided by GitHub Actions

### Branch Protection

Ensure branch protection is configured for `development`:
- ✅ Require pull request reviews
- ✅ Require status checks to pass
- ✅ Require branches to be up to date
- ✅ Restrict who can push to matching branches

## Local Dependency Checks

Check for outdated dependencies manually:

```bash
# Check for outdated crates
cargo outdated

# Check for security vulnerabilities
cargo audit

# Update Cargo.lock manually (if needed)
cargo update
```

## Troubleshooting

### Renovate not creating PRs

1. Check the Dependency Dashboard for errors
2. Verify `baseBranches: ["development"]` matches your default branch
3. Check GitHub Actions logs for Renovate workflow
4. Ensure Renovate bot has write permissions

### Auto-merge not working

1. Verify GitHub allows auto-merge (Settings → General → Allow auto-merge)
2. Check that `platformAutomerge: true` is set in `renovate.json5`
3. Ensure all required CI checks are passing
4. Verify minimum release age has passed

### Private crate updates failing

1. Verify `.cargo/config.toml` has correct registry URL
2. Check `CARGO_REGISTRY_TOKEN` is configured (if needed)
3. Ensure GitHub Packages access token has `read:packages` scope

### Config validation errors

```bash
# Run validation
nix develop --command renovate-config-validator renovate.json5

# Check for migration warnings
# Update deprecated fields based on output
```

## Best Practices

1. **Monitor the Dependency Dashboard weekly**
   - Review pending updates
   - Check for security alerts
   - Approve major updates after testing

2. **Let auto-merge handle patches**
   - Trust the CI pipeline
   - Review failed checks promptly
   - Only intervene when necessary

3. **Review major updates carefully**
   - Read changelogs
   - Test locally before merging
   - Update code if breaking changes

4. **Keep MSRV in sync**
   - Update MSRV in `Cargo.toml` when bumping toolchain
   - Test with `cargo +1.91.0 build`
   - Update Renovate rule if MSRV changes

5. **Use the schedule to your advantage**
   - Weekend updates minimize disruption
   - Monday lock file maintenance keeps things fresh
   - Monthly Nix updates reduce noise

## Disabling Renovate Temporarily

To pause Renovate updates:

1. **Pause via Dashboard**:
   - Open the Dependency Dashboard issue
   - Add a checkbox: `- [ ] 🔴 Pause updates`

2. **Disable in config** (not recommended):
   ```json5
   {
     "enabled": false
   }
   ```

3. **Close PRs individually**:
   - Close a Renovate PR to prevent recreation
   - Renovate will respect closed PRs

## Further Reading

- [Renovate Docs](https://docs.renovatebot.com/)
- [Cargo Manager](https://docs.renovatebot.com/modules/manager/cargo/)
- [Automerge Guide](https://docs.renovatebot.com/key-concepts/automerge/)
- [GitHub Enterprise Setup](https://docs.renovatebot.com/modules/platform/github/)

## Summary

Renovate is configured to:
- ✅ Keep dependencies up-to-date automatically
- ✅ Auto-merge safe updates (patches, security fixes)
- ✅ Group related ecosystem updates
- ✅ Schedule updates for minimal disruption
- ✅ Support private GitHub Enterprise crates
- ✅ Integrate seamlessly with CI/CD pipeline
- ✅ Respect MSRV and breaking changes
- ✅ Provide visibility via Dependency Dashboard

The configuration balances automation with safety, ensuring dependencies stay current while maintaining code quality and stability.
