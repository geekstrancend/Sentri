# Invar - Multi-Chain Invariant Enforcement Tool

Production-grade invariant verification system for smart contracts.

## Quick Start

1. [Getting Started](docs/getting-started.md) - Installation and setup
2. [Deployment Guide](docs/deployment.md) - Configure and deploy
3. [FAQ](docs/faq.md) - Common questions and answers

## Documentation

### For Users
- [Getting Started](docs/getting-started.md) - Installation, quick start, basic usage
- [Deployment](docs/deployment.md) - Configuration, deployment, troubleshooting
- [Versioning](docs/versioning.md) - Version policy and compatibility
- [FAQ](docs/faq.md) - Common questions and solutions

### For Developers
- [Testing Guide](docs/testing.md) - Running tests and measuring coverage
  - [Unit Tests](docs/testing/unit-testing.md)
  - [Integration Tests](docs/testing/integration-testing.md)
  - [Property Tests](docs/testing/property-testing.md)
  - [CLI Tests](docs/testing/cli-testing.md)
- [Contributing](CONTRIBUTING.md) - Development guidelines

### For Project Maintainers
- [Versioning Policy](docs/versioning.md) - Version numbering and support tiers

## Documentation Structure

```
docs/
├── getting-started.md       Installation and setup
├── deployment.md            Configuration and deployment
├── testing.md               Testing guide
├── faq.md                   Frequently asked questions
├── versioning.md            Version policy and support
└── testing/                 Detailed testing guides
    ├── unit-testing.md
    ├── integration-testing.md
    ├── property-testing.md
    └── cli-testing.md
```

## Getting Help

- **Documentation**: See [docs/](docs/) folder
- **Issues**: [GitHub Issues](https://github.com/Emmyhack/Invar/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Emmyhack/Invar/discussions)
- **Security**: See [SECURITY.md](SECURITY.md)



## Building from Source

```bash
git clone https://github.com/Emmyhack/Invar.git
cd Invar
cargo build --release
./target/release/invar --version
```

See [INSTALL.md](INSTALL.md) for detailed installation options.

## Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Security

For security issues, see [SECURITY.md](SECURITY.md) - please do not open public issues for vulnerabilities.

## License

MIT License - see [LICENSE](LICENSE)

## Project Metadata

- **Repository**: [github.com/Emmyhack/Invar](https://github.com/Emmyhack/Invar)
- **Issues**: [GitHub Issues](https://github.com/Emmyhack/Invar/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Emmyhack/Invar/discussions)
- **Code of Conduct**: [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
