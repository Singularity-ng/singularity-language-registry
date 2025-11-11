# Contributing to Singularity Language Registry

Thank you for your interest in contributing! This document explains our development workflow and standards.

## 🚀 Quick Start

1. **Setup development environment:**

   **Option A: Nix (Recommended - Everything included)**
   ```bash
   # Clone the repository
   git clone https://github.com/Singularity-ng/singularity-language-registry.git
   cd singularity-language-registry

   # Install Nix (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSL https://install.determinate.systems/nix | sh -s -- install

   # Enable direnv for automatic environment loading
   direnv allow

   # Or manually enter dev shell
   nix develop
   ```

   **Option B: Standard Rust (Manual tool installation)**
   ```bash
   # Clone the repository
   git clone https://github.com/Singularity-ng/singularity-language-registry.git
   cd singularity-language-registry

   # Setup git hooks for code quality
   ./setup-hooks.sh

   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Install development tools
   cargo install cargo-edit cargo-watch cargo-nextest cargo-audit cargo-outdated
   ```

2. **Create a feature branch:**
   ```bash
   git checkout development
   git pull origin development
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes and test:**

   **Using Nix + Just (Recommended):**
   ```bash
   just verify                  # Run all checks (fmt, clippy, test, audit)
   just watch                   # Auto-run tests on file changes
   nix flake check              # Run full CI locally
   ```

   **Using Cargo directly:**
   ```bash
   cargo fmt                    # Format code
   cargo clippy -- -D warnings  # Check for issues
   cargo test --all-features    # Run tests
   cargo doc --no-deps          # Build documentation
   ```

4. **Commit your changes:**
   ```bash
   git add .
   git commit  # Will open editor with commit template
   ```

5. **Push and create PR:**
   ```bash
   git push origin feature/your-feature-name
   # Then create a PR on GitHub
   ```

## 🌳 Branch Structure

- **`main`**: Production-ready code only
  - Protected branch - no direct pushes
  - Requires PR with review
  - All CI must pass
  - Requires Claude AI review

- **`development`**: Active development
  - Default branch for new features
  - Less strict requirements
  - Integration testing happens here

- **`feature/*`**: Feature branches
  - Branch from `development`
  - Merge back to `development`

## 🔒 Git Hooks

Our git hooks ensure code quality before commit/push:

### Pre-commit checks:
- ✅ Rust formatting (`cargo fmt`)
- ✅ Clippy with pedantic mode
- ✅ Build verification
- ✅ Test execution
- ✅ Sensitive data detection
- ✅ File size limits

### Pre-push checks:
- 🚫 Blocks direct push to main
- ✅ Comprehensive test suite
- ✅ Security audit
- ✅ Documentation build

To bypass hooks in emergency (not recommended):
```bash
git commit --no-verify
git push --no-verify
```

## 📝 Commit Message Format

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description

[optional body]

[optional footer]
```

### Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Test updates
- `chore`: Maintenance tasks
- `ci`: CI/CD changes
- `build`: Build system changes

### Examples:
```
feat(registry): add support for Crystal language
fix(detection): correct Python shebang detection
docs: update API documentation for v0.2.0
chore(deps): update serde to 1.0.150
```

## 🤖 Pull Request Process

### For all PRs:
1. Fill out the PR template completely
2. Ensure all CI checks pass
3. Update documentation if needed
4. Add tests for new functionality

### For PRs to main:
1. Must come from `development` or hotfix branch
2. Requires at least 1 approval
3. **Requires Claude AI review** - maintainer will trigger
4. All conversations must be resolved
5. Branch must be up to date

### Claude Review:
PRs to main require Claude AI review for:
- Security vulnerabilities
- Performance issues
- Code quality
- Best practices
- Documentation completeness

Trigger with: `/claude review` (maintainers only)

## 🧪 Testing Requirements

### Unit Tests:
```rust
#[test]
fn test_language_detection() {
    let path = Path::new("example.rs");
    let lang = detect_language(path).unwrap();
    assert_eq!(lang.name, "Rust");
}
```

### Integration Tests:
Place in `tests/` directory

### Coverage:
Aim for >80% code coverage

## 📚 Documentation

- All public APIs must have doc comments
- Include examples in doc comments
- Update README for user-facing changes
- Keep CHANGELOG.md updated

## 🔧 Code Standards

### Rust Style:
- Follow standard Rust conventions
- Use `cargo fmt` for formatting
- Pass `cargo clippy --all -- -D warnings -W clippy::pedantic`
- Prefer explicit over implicit
- Avoid `unwrap()` - use proper error handling

### Performance:
- Benchmark critical paths
- Use `&str` over `String` when possible
- Prefer iterators over collecting
- Cache expensive computations

### Security:
- No hardcoded credentials
- Validate all inputs
- Use `Result` for fallible operations
- Regular dependency audits

## 🚀 Release Process

1. All changes merged to `development`
2. Create PR from `development` to `main`
3. Pass all checks including Claude review
4. Merge to main
5. Tag release: `git tag v0.x.x`
6. Push tag: `git push origin v0.x.x`
7. GitHub Actions publishes to crates.io

## 💡 Tips

**With Nix:**
- Use `just` to see all available commands
- Run `nix flake check` to test CI locally before pushing
- Use `just watch` for instant feedback during development
- All tools are pre-installed in the Nix dev shell

**General:**
- Run `cargo outdated` regularly (or `just outdated`)
- Use `cargo audit` for security checks (or `just audit`)
- Profile with `cargo bench` for performance (or `just bench`)
- Check docs with `cargo doc --open` (or `just doc`)

## 📮 Getting Help

- Open an issue for bugs
- Start a discussion for features
- Check existing issues first
- Include reproduction steps

## 📄 License

This is proprietary software. All rights reserved.

By contributing, you agree that:
1. Your contributions become the exclusive property of Singularity Team
2. You assign all intellectual property rights to Singularity Team
3. You waive any moral rights to your contributions
4. You have the legal right to make these assignments

Contributors may be required to sign a Contributor License Agreement (CLA) before contributions can be accepted.

---

Thank you for contributing to make Singularity Language Registry better! 🎉