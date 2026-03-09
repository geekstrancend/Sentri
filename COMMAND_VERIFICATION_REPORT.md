# @dextonicx/cli Command Verification Report

## Summary
✅ **CLI Commands are defined but ❌ NOT fully implemented as documented in the npm package**

---

## Documented Commands (from npm package)

According to https://www.npmjs.com/package/@dextonicx/cli, the package claims to support:

```bash
sentri check <PATH> --chain <CHAIN> [OPTIONS]
sentri report <INPUT>
sentri init <PATH>
sentri doctor
```

**With Options:**
- `--chain <CHAIN>` (evm, solana, move) ✅
- `--format <FORMAT>` (text, json, html) ❌ Partially
- `--output <FILE>` ❌ Not implemented in check command
- `--config <FILE>` ✅
- `--fail-on <SEVERITY>` ✅
- `--verbose` ✅
- `--help` ✅
- `--version` ✅

---

## Implementation Status

### ✅ WORKING Commands:
1. **`sentri check`** - Command exists and parses arguments
   - Accepts path, --chain, --fail-on, --config, --verbose
   - Returns exit code 1 when violations found
2. **`sentri init`** - Creates .sentri.toml config file
3. **`sentri doctor`** - Health check of components
4. **`--version`** flag - Returns version

### ❌ BROKEN Features:

#### 1. **Format & Output Handling in `check` Command**
   
**Problem:** The `--format` and `--output` arguments are parsed but NEVER used.

**Location:** [crates/cli/src/main.rs](crates/cli/src/main.rs#L167-L280)

**Code Analysis:**
```rust
struct CheckArgs {
    path: PathBuf,
    chain: ChainArg,
    fail_on: SeverityArg,
    format: FormatArg,      // ← PARSED BUT NEVER USED
    output: Option<PathBuf>, // ← PARSED BUT NEVER USED
    config: Option<PathBuf>,
}

fn cmd_check(args: CheckArgs, ...) -> Result<()> {
    // ... code never accesses args.format or args.output
    // Always prints to stdout in text format
    println!("{}", render_violations(...));  // Only text output
    println!("{}", render_summary(...));     // Never checks args.format
}
```

**Impact:**
- `sentri check ./contracts --format json --output report.json` → ❌ Outputs text to stdout, doesn't write JSON file
- The npm package's `analyze()` function calls: `["check", ..., "--format", "json", "--output", tmpFile]`
- Then tries to read the JSON file that was never created → **FAILS**

---

#### 2. **`doctor` Command Doesn't Support `--format json`**

**Problem:** The doctor command doesn't accept any format parameter.

**CLI Definition:**
```rust
Commands {
    Doctor,  // ← No arguments defined
}

fn cmd_doctor(quiet: bool) -> Result<()> {
    // Takes no format parameter
}
```

**Usage in npm package:**
```javascript
// index.js line ~97
spawnSync(binaryPath, ["doctor", "--format", "json"], {...})
// ↑ Passes --format json but CLI ignores it
```

**Result:** The `--format json` flag is silently ignored, output is text format, and the `doctor()` function trying to parse JSON fails.

---

#### 3. **Hardcoded Mock Data Instead of Real Analysis**

**Problem:** The CLI doesn't perform actual invariant checking - it uses simulated/hardcoded violations.

**Code:**
```rust
fn cmd_check(args: CheckArgs, ...) {
    // Simulate analysis
    std::thread::sleep(std::time::Duration::from_millis(500)); // ← MOCK

    // Hardcoded violations
    let violations = vec![
        Violation {
            severity: "critical",
            title: "Reentrancy Vulnerability",  // ← HARDCODED
            location: format!("{}:142", args.path.display()), // ← FAKE
            // ...
        },
    ];
}
```

**Result:** Analysis results are always the same regardless of input

---

## NPM Package API Issues

The `index.js` module exports functions that don't work:

### `analyze(options)` - ❌ BROKEN
```javascript
async function analyze(options) {
    // ... 
    const args = ["check", targetPath, "--chain", chain, 
                  "--format", "json",        // ← CLI ignores this
                  "--output", tmpFile];      // ← CLI doesn't write this
    
    const reportJson = await fs.readFile(tmpFile, "utf8"); // ← FILE DOESN'T EXIST!
    return JSON.parse(reportJson);                         // ← JSON.parse fails
}
```

**Error:** `Error: ENOENT: no such file or directory, open '/path/to/.sentri-report-XXXXX.json'`

### `doctor()` - ❌ BROKEN
```javascript
async function doctor() {
    const result = spawnSync(binaryPath, ["doctor", "--format", "json"], {
        encoding: "utf8",
    });
    
    return JSON.parse(result.stdout); // ← Expecting JSON, getting text output!
}
```

**Error:** `SyntaxError: Unexpected token...` when parsing text output as JSON

---

## TypeScript Declarations vs. Reality

**Promises:** [index.d.ts](sentri-npm/index.d.ts)
```typescript
export interface SentriReport {
    version: string;
    timestamp: string;
    chain: string;
    summary: ReportSummary;
    violations: Violation[];
    // ...
}

export function analyze(options: AnalyzeOptions): Promise<SentriReport>;
export function doctor(): Promise<DoctorResult>;
```

**Reality:** These functions throw errors because the underlying CLI doesn't produce the expected output format.

---

## What Works vs. What Doesn't

| Feature | Status | Notes |
|---------|--------|-------|
| `sentri check --chain evm` | ✅ Works | Basic invocation works |
| `sentri check --format json` | ❌ Broken | Format argument ignored |
| `sentri check --output file.json` | ❌ Broken | Output file never created |
| `sentri init` | ✅ Works | Creates .sentri.toml |
| `sentri doctor --format json` | ❌ Broken | Format not supported |
| `npx @dextonicx/cli check ...` | ✅ Works | CLI wrapper works |
| `const report = await analyze(...)` | ❌ Broken | JSON not generated |
| `const health = await doctor()` | ❌ Broken | JSON parsing fails |

---

## Root Cause

The Rust CLI was designed with placeholder/mock implementations for demonstration purposes:
- UI rendering is fully implemented
- Argument parsing is complete
- But actual logic for format/output handling is missing
- The programmatic API (index.js) expects JSON but CLI only outputs text

The npm package (sentri-npm/) tries to use the CLI as a library via subprocess, but the CLI isn't exposing the functionality it claims to have.

---

## Recommended Fixes

### Priority 1: Fix CLI Output Format Handling
Modify `cmd_check()` in [crates/cli/src/main.rs](crates/cli/src/main.rs#L167) to:
1. Check `args.format` parameter
2. Serialize violations/summary to JSON if format == Json
3. Write to `args.output` file if specified
4. Support HTML format similarly

### Priority 2: Fix `doctor` Command
Update doctor command to accept format parameter and output JSON

### Priority 3: Replace Hardcoded Data
Integrate real invariant analysis instead of mock violations

### Priority 4: Update CI Tests
Current tests likely don't verify JSON output or file writing

