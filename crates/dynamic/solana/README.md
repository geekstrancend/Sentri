# sentri-dynamic-solana

Dynamic invariant fuzzing for Solana programs — the account-model counterpart
to `sentri-dynamic-evm`.

Solana's execution model is account-based (an instruction is a program id, a
list of `AccountMeta`s, and an opaque data blob), not the EVM's flat-calldata,
single-caller shape. So this crate defines its own call model, invariant
oracles, instruction generator, and fuzz/shrink loop rather than forcing the
EVM types onto it. Everything else — the search + delta-debug shrink to a
minimal proof-of-concept — mirrors the proven EVM engine.

## What it checks

- **Token conservation** — `sum(token account amounts) == mint supply`. Catches
  "mint out of thin air": an instruction that credits a balance without
  updating supply.
- **Account-owner integrity** — an account's `owner` must not change (or must
  change only as declared). Catches unchecked ownership reassignment.

Both come with the minimal reproducing instruction sequence.

## Backends

- **Default (no features):** the engine is proven against an in-memory
  `MockSvm` (a tiny SPL-token-like program with a deliberately buggy airdrop),
  exactly as the EVM crate proved its engine before wiring revm. Builds and
  tests with no Solana dependency.
- **`litesvm-backend`:** a real in-process SVM (`litesvm`) that runs compiled
  BPF bytecode. Pins the same granular `solana-*` 2.2 crates litesvm uses so
  the whole thing links against one coherent Solana version. Seed a genesis,
  deploy the program `.so`, and the same invariant search runs over real code.

```bash
cargo test  -p sentri-dynamic-solana                        # engine proof (mock)
cargo build -p sentri-dynamic-solana --features litesvm-backend  # real SVM adapter
```
