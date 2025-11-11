# Security Policy

## Supported Versions

We provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| Latest  | :white_check_mark: |
| < Latest| :x:                |

We recommend always using the latest version of our software.

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to: **security@singularity.example.com**

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

Please include the following information:

- Type of issue (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

## Security Update Process

1. **Receipt**: We acknowledge receipt of your vulnerability report
2. **Assessment**: We assess the vulnerability and determine severity
3. **Fix**: We develop and test a fix
4. **Disclosure**: We coordinate disclosure with you
5. **Release**: We release the security update
6. **Credit**: We credit you in the security advisory (unless you prefer to remain anonymous)

## Security Features in Our Projects

All Singularity projects include:

- **Automated security audits** - `cargo audit` runs on every release
- **Dependency checking** - `cargo deny` validates all dependencies
- **SBOM generation** - Complete dependency transparency
- **Zero warnings tolerance** - Strict linting catches potential issues
- **Regular updates** - Renovate keeps dependencies current

## Security Best Practices

When using Singularity software:

1. **Keep updated** - Always use the latest version
2. **Review dependencies** - Check the SBOM in releases
3. **Enable security features** - Use all available security options
4. **Follow principle of least privilege** - Run with minimal permissions
5. **Monitor security advisories** - Watch the repository for updates

## Contact

- **Security issues**: security@singularity.example.com
- **General questions**: See SUPPORT.md

---

**Thank you for helping keep Singularity and our users safe!**
