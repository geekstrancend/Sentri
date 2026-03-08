# sentri-solana-macro

Procedural macros for Sentri Solana code generation.

Provides derive and attribute macros for automating invariant enforcement in Solana programs.

## Usage

```toml
[dependencies]
sentri-solana-macro = "0.1.3"
sentri-core = "0.1.3"
```

## Available Macros

- `#[invariant]`: Mark functions for invariant checking
- `#[derive(Validate)]`: Automatically generate account validators
- `#[signer_required]`: Enforce signer constraints on accounts
- `#[rent_exempt]`: Validate rent exemption status

## Example

```rust
use sentri_solana_macro::{invariant, Validate};

#[invariant("signer_required")]
pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
    // Macro automatically enforces signer requirement
    Ok(())
}

#[derive(Validate)]
pub struct TransferAccounts {
    #[account(signer)]
    pub authority: Signer<'info>,
    pub from: Account<'info, TokenAccount>,
}
```

## Macro Features

- Procedural derive macros for account validation
- Attribute macros for safety enforcement
- Compile-time constraint checking
- Zero runtime overhead

## Use Cases

- Account constraint generation
- Signer requirement enforcement
- Automatic validation code generation
- Type-safe Anchor integration

See [Sentri documentation](https://github.com/geekstrancend/Sentri) for macro API details.

## License

MIT
