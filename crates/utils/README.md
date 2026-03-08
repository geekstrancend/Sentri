# sentri-utils

Utility functions for the Sentri invariant checking framework.

Provides helpers for logging, configuration loading, and common operations across Sentri tools.

## Usage

```toml
[dependencies]
sentri-utils = "0.1.3"
```

## Key Functions

- `setup_logging()` — initialize tracing-based logging
- `load_config()` — load `.sentri.toml` configuration
- `format_report()` — format violations for display
- `determine_severity()` — classify violation severity

## Example

```rust
use sentri_utils::{setup_logging, load_config};

setup_logging()?;
let config = load_config("sentri.toml")?;
println!("Analyzing chain: {}", config.chain);
```

See [Sentri documentation](https://github.com/geekstrancend/Sentri) for more examples.

## License

MIT

