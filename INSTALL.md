# Invar Installation Guide

Invar is a production-grade, audit-ready multi-chain smart contract invariant enforcement tool.

## System Requirements

- Minimum 2GB RAM
- 500MB disk space
- Supported platforms: Linux, macOS, Windows

## Installation Methods

### 1. Pre-compiled Binaries (Recommended)

Download the latest release from [GitHub Releases](https://github.com/Emmyhack/Invar/releases).

#### Linux / macOS

```bash
# Download and verify
curl -L -O https://github.com/Emmyhack/Invar/releases/download/v0.1.0/invar-linux-x86_64-0.1.0
curl -L -O https://github.com/Emmyhack/Invar/releases/download/v0.1.0/invar-linux-x86_64-0.1.0.sha256

# Verify checksum (critical for security)
sha256sum -c invar-linux-x86_64-0.1.0.sha256

# Install
chmod +x invar-linux-x86_64-0.1.0
sudo mv invar-linux-x86_64-0.1.0 /usr/local/bin/invar

# Verify installation
invar --version
```

#### macOS (ARM64 / Apple Silicon)

```bash
curl -L -O https://github.com/Emmyhack/Invar/releases/download/v0.1.0/invar-darwin-aarch64-0.1.0
sha256sum -c invar-darwin-aarch64-0.1.0.sha256
chmod +x invar-darwin-aarch64-0.1.0
sudo mv invar-darwin-aarch64-0.1.0 /usr/local/bin/invar
invar --version
```

#### Windows

Download `invar-windows-x86_64-0.1.0.exe` from the releases page and add it to your PATH.

### 2. Install from Source

Requires Rust 1.70.0+. Install from https://rustup.rs/.

```bash
git clone https://github.com/Emmyhack/Invar.git
cd Invar
cargo install --path crates/cli
```

### 3. Cargo Install

```bash
cargo install invar
```

## Verification

### Check Installation

```bash
invar --version
invar --help
```

### Verify Binary Integrity

Always verify checksums for downloaded binaries:

```bash
# Get the SHA256 sum of your binary
sha256sum /usr/local/bin/invar

# Compare with the published checksum from releases page
# If they match, your binary is verified
```

## Configuration

Invar supports configuration via:

1. **Command-line arguments** (highest priority)
   ```bash
   invar --config path/to/config.toml
   ```

2. **Environment variables**
   ```bash
   export INVAR_STRICT_MODE=true
   export INVAR_CHAIN=solana
   ```

3. **Configuration file** (`~/.invar/config.toml`)
   ```toml
   [enforcement]
   strict_mode = true
   re_parse_verification = true
   tamper_detection = true

   [chains]
   enabled = ["solana", "evm", "move"]
   ```

## Uninstallation

```bash
# If installed to /usr/local/bin
sudo rm /usr/local/bin/invar

# If installed via cargo
cargo uninstall invar
```

## Troubleshooting

### Binary not found after installation

Ensure `/usr/local/bin` is in your PATH:
```bash
echo $PATH | grep -q /usr/local/bin || echo "/usr/local/bin not in PATH"
```

### Checksum verification fails

This indicates a corrupted download. Re-download the binary and try again:
```bash
rm invar-*
# Re-download from releases page
```

### Permission denied on Linux/macOS

Make sure the binary is executable:
```bash
chmod +x /usr/local/bin/invar
```

## Security Considerations

1. **Always verify checksums** before running downloaded binaries
2. **Keep Invar updated** to get security patches
3. **Run with least privilege** - don't run as root unless necessary
4. **Enable strict mode** for production deployments:
   ```bash
   invar --strict-mode analyze --file invariants.invar
   ```

## Getting Help

- Report bugs: https://github.com/Emmyhack/Invar/issues
- Documentation: https://github.com/Emmyhack/Invar/wiki
- Community: Discussions at https://github.com/Emmyhack/Invar/discussions

## Release Notes

See [RELEASES.md](RELEASES.md) for version history and changelog.

## Building from Source (Advanced)

For reproducible builds from source:

```bash
# Ensure Rust 1.70.0 is installed
rustc --version

# Build in release mode with reproducible settings
cargo build --release -p invar

# Verify the build (if tests are available)
cargo test --release

# Binary will be at target/release/invar
./target/release/invar --version
```

### Reproducibility Verification

All binaries in official releases are built with:
- Rust 1.70.0 (pinned version)
- LTO (Link Time Optimization) enabled
- Optimization level 3
- Cargo.lock committed and locked
- Deterministic build ordering

This ensures that rebuilding the same source code produces bit-for-bit identical binaries.
