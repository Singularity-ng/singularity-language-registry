# Development tasks for Singularity Language Registry
# Run `just` to see available commands

# Default recipe - show help
default:
    @just --list

# Enter Nix development shell
shell:
    nix develop

# Build the project with Nix
build:
    nix build -L

# Run all flake checks
check:
    nix flake check -L

# Update flake inputs
update:
    nix flake update
    git add flake.lock
    git commit -m "chore(nix): update flake.lock" || true

# Format all code
fmt:
    cargo fmt
    nixpkgs-fmt *.nix

# Run clippy with pedantic mode
clippy:
    cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery

# Run tests
test:
    cargo test --all-features

# Run tests in release mode
test-release:
    cargo test --all-features --release

# Generate code coverage
coverage:
    cargo tarpaulin --all-features --out html

# Check for outdated dependencies
outdated:
    cargo outdated

# Security audit
audit:
    cargo audit

# Validate Renovate configuration
renovate-validate:
    nix develop --command renovate-config-validator renovate.json5

# Build documentation
doc:
    cargo doc --all-features --no-deps --open

# Run benchmarks
bench:
    cargo bench

# Clean build artifacts
clean:
    cargo clean
    rm -rf result result-*

# Watch and run tests
watch:
    cargo watch -x test

# Run example
example:
    cargo run --example usage

# Check reproducibility with Nix
reproducible:
    #!/usr/bin/env bash
    set -e
    echo "Building first time..."
    nix build -L -o result-1
    echo "Building second time..."
    nix build -L -o result-2 --rebuild
    echo "Comparing builds..."
    if diff -r result-1 result-2; then
        echo "✅ Builds are reproducible!"
    else
        echo "⚠️ Builds differ"
        exit 1
    fi

# Create a new release
release version:
    #!/usr/bin/env bash
    set -e
    # Update version in Cargo.toml
    cargo set-version {{version}}
    # Commit changes
    git add Cargo.toml Cargo.lock
    git commit -m "chore: release v{{version}}"
    # Create tag
    git tag -a v{{version}} -m "Release v{{version}}"
    echo "Release prepared. Run 'git push && git push --tags' to publish"

# Setup git hooks
setup:
    ./setup-hooks.sh

# Run CI locally with act
ci-local:
    act -P ubuntu-latest=ubuntu:latest

# Generate changelog
changelog:
    git log --pretty=format:"- %s (%h)" --reverse > CHANGELOG.md

# Verify everything before PR
verify: fmt clippy test audit renovate-validate doc
    @echo "✅ All checks passed!"

# Nix-specific commands

# Build Docker image with Nix
docker:
    nix build .#docker
    echo "Docker image built at: ./result"

# Enter Nix REPL
repl:
    nix repl .

# Show flake info
info:
    nix flake show
    nix flake metadata

# Garbage collect Nix store
gc:
    nix-collect-garbage -d

# Update a specific flake input
update-input input:
    nix flake lock --update-input {{input}}

# Build and push to cachix (requires cachix setup)
cache:
    nix build --json | jq -r '.[].outputs | to_entries[].value' | cachix push singularity-language-registry || true

# Generate release reports locally (simulates CI release reports)
release-reports:
    #!/usr/bin/env bash
    set -e
    echo "Generating release reports..."
    mkdir -p release-reports

    # Clippy report (0 warnings tolerance)
    echo "# Clippy Report - Zero Warnings Tolerance" > release-reports/clippy-report.md
    echo "" >> release-reports/clippy-report.md
    echo "**Date:** $(date -u +"%Y-%m-%d %H:%M:%S UTC")" >> release-reports/clippy-report.md
    echo "## Configuration" >> release-reports/clippy-report.md
    echo "- \`-D warnings\` - \`-W clippy::pedantic\` - \`-W clippy::nursery\`" >> release-reports/clippy-report.md
    echo "\`\`\`" >> release-reports/clippy-report.md
    cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery 2>&1 | tee -a release-reports/clippy-report.md
    echo "\`\`\`" >> release-reports/clippy-report.md

    # Security audit
    echo "# Security Audit Report" > release-reports/security-audit.md
    echo "" >> release-reports/security-audit.md
    echo "**Date:** $(date -u +"%Y-%m-%d %H:%M:%S UTC")" >> release-reports/security-audit.md
    echo "## Cargo Audit" >> release-reports/security-audit.md
    echo "\`\`\`" >> release-reports/security-audit.md
    cargo audit 2>&1 | tee -a release-reports/security-audit.md || true
    echo "\`\`\`" >> release-reports/security-audit.md
    echo "## Cargo Deny" >> release-reports/security-audit.md
    echo "\`\`\`" >> release-reports/security-audit.md
    cargo deny check 2>&1 | tee -a release-reports/security-audit.md || true
    echo "\`\`\`" >> release-reports/security-audit.md

    # SBOM
    echo "# Software Bill of Materials" > release-reports/sbom.md
    echo "" >> release-reports/sbom.md
    echo "**Date:** $(date -u +"%Y-%m-%d %H:%M:%S UTC")" >> release-reports/sbom.md
    echo "## Direct Dependencies" >> release-reports/sbom.md
    echo "\`\`\`toml" >> release-reports/sbom.md
    grep -A 100 '^\[dependencies\]' Cargo.toml | grep -B 100 '^\[' | head -n -1 >> release-reports/sbom.md
    echo "\`\`\`" >> release-reports/sbom.md
    echo "## Full Dependency Tree" >> release-reports/sbom.md
    echo "\`\`\`" >> release-reports/sbom.md
    cargo tree --all-features >> release-reports/sbom.md
    echo "\`\`\`" >> release-reports/sbom.md

    # Coverage
    echo "# Test Coverage Report" > release-reports/coverage-report.md
    echo "" >> release-reports/coverage-report.md
    echo "**Date:** $(date -u +"%Y-%m-%d %H:%M:%S UTC")" >> release-reports/coverage-report.md
    echo "\`\`\`" >> release-reports/coverage-report.md
    cargo tarpaulin --all-features --out Stdout 2>&1 | tee -a release-reports/coverage-report.md || true
    echo "\`\`\`" >> release-reports/coverage-report.md

    echo ""
    echo "✅ Reports generated in ./release-reports/"
    echo "   - clippy-report.md"
    echo "   - security-audit.md"
    echo "   - sbom.md"
    echo "   - coverage-report.md"