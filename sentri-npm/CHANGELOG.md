# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.3] - 2026-03-08

### Added
- Initial npm package release for `@sentri/cli`
- CLI binary wrapper for all 5 supported platforms (Linux x86_64/ARM64, macOS x86_64/ARM64, Windows x86_64)
- Automatic binary download and extraction on `npm install`
- SHA256 checksum verification for downloaded binaries
- Programmatic Node.js API for running analyses in JavaScript/TypeScript
- Support for EVM, Solana, and Move smart contract analysis
- Environment variable configuration (SENTRI_BINARY_PATH, HTTPS_PROXY, SENTRI_SKIP_DOWNLOAD)
- GitHub Actions CI/CD integration examples
- Hardhat task integration example
- TypeScript type definitions (index.d.ts)
- Comprehensive README with usage examples
- Jest test suite with platform detection and API tests
- Support for proxy configurations via https-proxy-agent

### Features
- **CLI**: `sentri check <path> --chain <evm|solana|move>`
- **API**: `analyze()`, `doctor()`, `init()`, `version()`, `isInstalled()`
- **output Formats**: text (default), json, html
- **CI Integration**: `--fail-on` severity levels for automated checks
- **Configuration**: `.sentri.toml` support
- **Platform Detection**: Automatic detection of OS/architecture

### Supported Platforms
- Linux x86_64 (glibc)
- Linux ARM64 (aarch64)
- macOS x86_64 (Intel)
- macOS ARM64 (Apple Silicon / M1/M2)
- Windows x86_64 (MSVC)

### Known Limitations
- Requires Node.js 16 or higher
- Binary download requires internet connection (can be skipped with SENTRI_SKIP_DOWNLOAD=1)
- Postinstall script may be skipped with `npm install --ignore-scripts`

---

## Versioning

This npm package version **always matches** the Rust `sentri-cli` crate version.
When a new Sentri release is published, the npm package is automatically published with the same version.

For example:
- Sentri Rust v0.1.3 → `npm install @sentri/cli@0.1.3`
- Sentri Rust v0.2.0 → `npm install @sentri/cli@0.2.0`

---

## Upgrade Guide

### From v0.1.2 → v0.1.3

```bash
npm install @sentri/cli@latest
```

No breaking changes.

---

## Release Schedule

This package follows the Sentri release schedule:
- New releases published automatically when Rust crate is released
- Version sync maintained via GitHub Actions workflow
- Pre-release versions available on npm with `-alpha` or `-beta` tags

---

For detailed release notes, see the main Sentri repository:
https://github.com/geekstrancend/Sentri/releases
