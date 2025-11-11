# GitHub Enterprise Configuration

## For GitHub Enterprise Cloud (github.com)

No changes needed - works as configured!

## For GitHub Enterprise Server (self-hosted)

### 1. Update Git URLs

```bash
# If your GHE server is at git.company.com
git remote set-url origin git@git.company.com:Singularity-ng/singularity-language-registry.git
```

### 2. Update Workflow Configurations

#### CI Workflow Updates

```yaml
# In .github/workflows/*.yml files, update:

# For GitHub Enterprise Server API calls
env:
  GITHUB_API_URL: https://git.company.com/api/v3
  GITHUB_SERVER_URL: https://git.company.com
```

### 3. Enterprise-Specific Features

#### Internal Package Registry

```yaml
# Use internal registry instead of crates.io
- name: Publish to Internal Registry
  run: |
    cargo publish --registry enterprise \
      --token ${{ secrets.GITHUB_TOKEN }}
```

#### Enhanced Caching (often 50GB+)

```yaml
# GitHub Enterprise often has larger cache limits
- uses: actions/cache@v4
  with:
    path: |
      ~/.cargo
      target/
      /nix/store
    key: ${{ runner.os }}-enterprise-${{ hashFiles('**/Cargo.lock') }}
    # Can use larger caches in Enterprise
```

#### Enterprise Runners

```yaml
# Use enterprise-specific runners if available
runs-on: [self-hosted, linux, x64, rust]
# Or use runner groups
runs-on:
  group: enterprise-runners
  labels: [rust, nix]
```

## Secrets Configuration for Enterprise

### Required Secrets

1. **GITHUB_TOKEN** (Automatic)
   - Provided by GitHub Enterprise
   - Enhanced permissions in Enterprise
   - Can access internal packages

2. **CRATES_TOKEN** (Only if publishing to crates.io)
   - Skip if using internal registry
   - Use `GITHUB_TOKEN` for internal packages

3. **Internal Secrets** (Enterprise-specific)
   ```yaml
   # Common enterprise secrets
   ARTIFACTORY_TOKEN     # If using Artifactory
   NEXUS_TOKEN          # If using Nexus
   LDAP_SERVICE_ACCOUNT # For auth
   VAULT_TOKEN          # If using HashiCorp Vault
   ```

## Enterprise-Optimized Workflows

### Using GitHub Enterprise Package Registry

```yaml
# .github/workflows/release.yml
- name: Publish to GitHub Packages
  run: |
    # Configure cargo for GitHub Packages
    echo "[registries.github]" >> ~/.cargo/config.toml
    echo "index = \"sparse+https://git.company.com/_cargo/index/\"" >> ~/.cargo/config.toml
    echo "token = \"${{ secrets.GITHUB_TOKEN }}\"" >> ~/.cargo/config.toml

    # Publish
    cargo publish --registry github
```

### Enterprise Cachix Alternative

```yaml
# Use internal Artifactory/Nexus instead of Cachix
- name: Setup Internal Binary Cache
  run: |
    nix-env -iA cachix -f https://cachix.org/api/v1/install
    cachix use company-internal --host https://artifactory.company.com
  env:
    CACHIX_AUTH_TOKEN: ${{ secrets.ARTIFACTORY_TOKEN }}
```

### Enterprise SSO Integration

```yaml
# For SAML/OIDC authenticated registries
- name: Authenticate with SSO
  uses: company/sso-action@v1
  with:
    realm: engineering
    client-id: ${{ secrets.SSO_CLIENT_ID }}
    client-secret: ${{ secrets.SSO_CLIENT_SECRET }}
```

## Enterprise-Specific CI Features

### 1. Required Status Checks

```yaml
# Enterprise often requires additional checks
required-checks:
  - security-scan       # Enterprise security tools
  - license-scan       # License compliance
  - sonarqube          # Code quality gates
  - dependency-check   # Vulnerability scanning
```

### 2. Compliance Automation

```yaml
- name: Compliance Checks
  uses: company/compliance-action@v1
  with:
    policy: engineering-rust
    attestation: true
    sign-commits: true
```

### 3. Internal Deployment

```yaml
# Deploy to internal infrastructure
- name: Deploy to Enterprise Cloud
  if: github.ref == 'refs/heads/main'
  uses: company/deploy-action@v1
  with:
    environment: production
    cluster: internal-k8s
    namespace: singularity
```

## Best Practices for Enterprise

1. **Use GitHub App instead of PAT**
   ```yaml
   # More secure than personal tokens
   - uses: tibdex/github-app-token@v2
     id: generate_token
     with:
       app_id: ${{ secrets.APP_ID }}
       private_key: ${{ secrets.APP_PRIVATE_KEY }}
   ```

2. **Leverage Enterprise Runners**
   - Faster builds with local caches
   - Access to internal resources
   - No minute limits

3. **Use CODEOWNERS**
   ```
   # .github/CODEOWNERS
   * @Singularity-ng/maintainers
   /src/ @Singularity-ng/rust-team
   /.github/ @Singularity-ng/devops
   ```

4. **Enable Advanced Security**
   - Dependency scanning
   - Secret scanning
   - Code scanning with CodeQL

## Monitoring & Observability

```yaml
# Send metrics to enterprise monitoring
- name: Report Metrics
  if: always()
  run: |
    curl -X POST https://metrics.company.com/v1/ci \
      -H "Authorization: Bearer ${{ secrets.METRICS_TOKEN }}" \
      -d '{
        "repo": "${{ github.repository }}",
        "workflow": "${{ github.workflow }}",
        "status": "${{ job.status }}",
        "duration": "${{ github.run_duration }}",
        "runner": "${{ runner.name }}"
      }'
```

## Migration Checklist

- [ ] Update git remote URLs if self-hosted
- [ ] Configure enterprise package registry
- [ ] Set up internal binary caches
- [ ] Add enterprise-specific secrets
- [ ] Configure SSO/SAML if required
- [ ] Update API endpoints in workflows
- [ ] Set up enterprise runners
- [ ] Configure compliance policies
- [ ] Enable Advanced Security features
- [ ] Set up monitoring/alerting

## Support

For GitHub Enterprise-specific issues:
- Internal: #github-enterprise-support
- External: https://support.github.com/enterprise