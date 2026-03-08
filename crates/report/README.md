# sentri-report

Report generation for Sentri invariant violations.

Generates formatted violation reports (text, JSON, HTML) with detailed context and remediation recommendations.

## Usage

```toml
[dependencies]
sentri-report = "0.1.3"
sentri-core = "0.1.3"
```

## Generating Reports

```rust
use sentri_report::{ReportGenerator, Format};

let generator = ReportGenerator::new();
let report = generator.generate(&violations, Format::Json)?;
println!("{}", report);
```

## Supported Formats

- **Text** — colored terminal output with ASCII formatting
- **JSON** — structured format for programmatic processing
- **HTML** — styled HTML report with interactive features

## Example

```rust
use sentri_report::ReportGenerator;

let gen = ReportGenerator::new();
let text_report = gen.to_text(&violations)?;
let json_report = gen.to_json(&violations)?;
```

See [Sentri documentation](https://github.com/geekstrancend/Sentri) for report examples.

## License

MIT

