# Sentri Skills

Deterministic-first security skills for AI coding agents (Claude Code, Cursor,
Codex, Copilot, Windsurf). Unlike prompt-only audit skills, these are backed by
[Sentri](https://github.com/geekstrancend/Sentri)'s **compiled engine** — static
analyzers plus a real `revm`-backed invariant fuzzer — so findings are
machine-verified and reproducible, not an LLM's opinion.

| Skill | What it does |
|-------|--------------|
| [sentri-audit](sentri-audit/) | Deterministic-first, engine-verified smart-contract audit across EVM · Solana · Move · Soroban. Runs the compiled engine first, then amplifies with LLM attacker lenses whose findings are verified back through the engine before being reported. |

## Prerequisite

The skills call the `sentri` binary. Install it once:

```bash
# from a clone of the Sentri repo
cargo install --path crates/cli
# or build in-place
cargo build --release --bin sentri
```

Verify: `sentri doctor` should report all components healthy.

## Install (Claude Code)

```
Install https://github.com/geekstrancend/Sentri and run sentri-audit on the codebase
```

Or copy `skills/sentri-audit/` into your agent's skills directory.
