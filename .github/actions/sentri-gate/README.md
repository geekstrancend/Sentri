# Sentri Security Gate (GitHub Action)

A **deterministic** smart-contract security gate for CI. It runs Sentri's
compiled static engine on every push/PR, **fails the check** when findings at
or above a threshold exist, and uploads **SARIF** so findings appear inline on
the PR diff and in the repository's **Security → Code scanning** tab.

No LLM. No API keys. No per-run cost. The identical result every time — the
same scan a developer can reproduce locally with `sentri scan`.

## Usage

```yaml
name: Security
on: [pull_request, push]

permissions:
  contents: read
  security-events: write   # required to upload SARIF

jobs:
  sentri-gate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      # Build the sentri binary (cache it for speed).
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo build --release --bin sentri
      - run: echo "$PWD/target/release" >> "$GITHUB_PATH"

      # The gate: fail the PR on critical findings, publish SARIF.
      - uses: geekstrancend/Sentri/.github/actions/sentri-gate@main
        with:
          path: src            # your contracts
          chain: evm           # evm | solana | move | soroban
          fail-on: critical    # low | medium | high | critical
```

## Inputs

| Input | Default | Description |
|-------|---------|-------------|
| `path` | `.` | File or directory to scan. |
| `chain` | `evm` | `evm`, `solana`, `move`, or `soroban`. |
| `fail-on` | `critical` | Minimum severity that fails the check. |
| `sentri-bin` | `sentri` | Path to the (already built) binary. |
| `upload-sarif` | `true` | Upload SARIF to code scanning. |

## Why a gate, not a chat

Prompt-only audit tools are interactive and non-deterministic — they can't be a
silent, reproducible check on every commit. Sentri can: it's a compiled engine,
so the gate is free, fast, and gives the same verdict to every developer and
every reviewer. Pair it with the [`sentri-audit`](../../../skills/sentri-audit)
skill for deep on-demand review.
