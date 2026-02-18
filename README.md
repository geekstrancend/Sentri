# Invar Documentation

Welcome to Invar! Here's how to get started based on your needs.

## Quick Start (5 minutes)

1. [Installation & Quick Start](getting-started.md) - Get up and running
2. [Write Your First Invariant](writing-invariants.md) - Learn the DSL
3. [See Real Examples](example-invariants.md) - Practical patterns

## Main Guides

### Usage
- [Writing Invariants](writing-invariants.md) - Complete DSL reference and patterns
- [Example Invariants](example-invariants.md) - Real-world examples you can use
- [Deployment & CI/CD](deployment.md) - Deploy to production, integrate with CI/CD
- [Troubleshooting & FAQ](faq.md) - Common questions and solutions

### Understanding Invar
- [Architecture Overview](architecture-overview.md) - How Invar works internally
- [Security Model](security-model.md) - Security guarantees and trust boundaries
- [Error Design](error-design.md) - How to understand error messages

### Operations
- [Versioning & Stability](versioning.md) - Version numbering and support
- [Migration Guide](migration.md) - Upgrading between versions
- [Release Checklist](release-readiness.md) - For maintainers preparing releases

## For Your Role

### New Users
1. Start: [Getting Started](getting-started.md)
2. Learn: [Writing Invariants](writing-invariants.md)
3. Apply: [Example Invariants](example-invariants.md)
4. Integrate: [Deployment & CI/CD](deployment.md)

### Contributors
1. Understand: [Architecture Overview](architecture-overview.md)
2. Test: [Testing Guide](testing.md)
3. Contribute: See [CONTRIBUTING.md](../CONTRIBUTING.md)

### Security Researchers
- [Security Model](security-model.md) - Threat model
- [Security Validation](security/security-validation.md) - Testing approach
- [Error Design](error-design.md) - Error handling

### Release Managers
- [Release Checklist](release-readiness.md) - Pre-release verification
- [Versioning Policy](versioning.md) - Version numbering
- [Migration Guide](migration.md) - What users need to know

## Documentation Structure

```
docs/
├── getting-started.md           START HERE
├── writing-invariants.md        DSL reference
├── example-invariants.md        Real Examples
├── deployment.md                Install, configure, CI/CD
├── architecture-overview.md     Design internals
├── security-model.md            Security & trust
├── versioning.md                Versions & stability
├── migration.md                 Upgrade guide
├── release-readiness.md         Release process
├── faq.md                       Q&A
│
├── testing/                     Developer guides
│   ├── unit-testing.md
│   ├── property-testing.md
│   ├── cli-testing.md
│   └── integration-testing.md
│
├── security/                    Security details
│   └── security-validation.md
│
├── performance/                 Optimization
│   └── benchmarking.md
│
└── ci/                         CI/CD internals
    └── ci-pipeline.md
```

### "I'm getting an error"
→ [Error Design](error-design.md) → [Getting Started - Troubleshooting](getting-started.md#troubleshooting)

## External Resources

- **Repository**: [github.com/invar/invar](https://github.com/invar/invar)
- **Issues**: [GitHub Issues](https://github.com/invar/invar/issues)
- **Discussions**: [GitHub Discussions](https://github.com/invar/invar/discussions)
- **Security**: [security@invar-project.dev](mailto:security@invar-project.dev)
- **Website**: [invar.sh](https://invar.sh)

## Version

This documentation is for **Invar 0.1.0+**

For other versions:
- [Latest](https://invar.sh/docs/latest)
- [Development](https://github.com/invar/invar/tree/develop/docs)
- [1.0.0 (Stable)](https://invar.sh/docs/1.0) - When released

## Contributing to Documentation

Found an error? Have a suggestion? 

1. Create an issue: [GitHub Issues](https://github.com/invar/invar/issues)
2. Submit a PR: [GitHub PRs](https://github.com/invar/invar/pulls)
3. Email: [docs@invar-project.dev](mailto:docs@invar-project.dev)

## Documentation Principles

This documentation follows:
- **Clarity** - Clear, concise explanations
- **Completeness** - All features covered
- **Examples** - Real-world code samples
- **Honesty** - Limitations documented
- **Accessibility** - No assumed knowledge

Last updated: 2026-02-18
