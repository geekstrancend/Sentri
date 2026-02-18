# CI/CD Integration

Guide for integrating Invar into your continuous integration pipeline.

## GitHub Actions

### Basic Integration

Create `.github/workflows/invar.yml`:

```yaml
name: Invariant Checks

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]

jobs:
  invariants:
    name: Check Invariants
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Invar
        run: cargo install invar
      
      - name: Check invariants
        run: invar check invariants/
```

### With Reporting

```yaml
- name: Check invariants and generate report
  run: invar check --format json invariants/ > report.json
  
- name: Upload report
  uses: actions/upload-artifact@v3
  with:
    name: invariant-report
    path: report.json
```

## Pre-commit Hooks

Prevent committing code that violates invariants:

### Setup

```bash
# Create .git/hooks/pre-commit
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
set -e

echo "🔍 Checking invariants..."
invar check invariants/ || exit 1

echo "Invariants passed"
exit 0
EOF

chmod +x .git/hooks/pre-commit
```

### Using Tools

With [husky](https://typicode.github.io/husky):

```bash
npm install husky --save-dev
npx husky install
npx husky add .husky/pre-commit "invar check invariants/"
```

## Build Systems

### Cargo

In your `Cargo.toml`:

```toml
[dev-dependencies]
# Invar is available via crates.io
```

Or in a build script `build.rs`:

```rust
fn main() {
    // Run invariant checks during build
    std::process::Command::new("invar")
        .args(&["check", "invariants/"])
        .output()
        .expect("Failed to check invariants");
}
```

### Maven (Java)

```xml
<plugin>
    <groupId>org.apache.maven.plugins</groupId>
    <artifactId>maven-exec-plugin</artifactId>
    <executions>
        <execution>
            <goals>
                <goal>exec</goal>
            </goals>
            <configuration>
                <executable>invar</executable>
                <arguments>
                    <argument>check</argument>
                    <argument>invariants/</argument>
                </arguments>
            </configuration>
        </execution>
    </executions>
</plugin>
```

### Gradle

```gradle
task checkInvariants(type: Exec) {
    commandLine 'invar', 'check', 'invariants/'
}

build.dependsOn checkInvariants
```

### npm/Node.js

```json
{
  "scripts": {
    "check:invariants": "invar check invariants/",
    "pretest": "npm run check:invariants",
    "test": "jest"
  }
}
```

## IDE Integration

### VS Code

Install [Invar VS Code Extension](https://marketplace.visualstudio.com/items?itemName=invar.invar):

```json
{
  "invar.checkOnSave": true,
  "invar.checkOnChange": false,
  "invar.format": "json"
}
```

### IntelliJ IDEA

Create a run configuration:

1. Run → Edit Configurations
2. Add new "Gradle"
3. Set tasks to: `invar check invariants/`

## CI/CD Platforms

### GitLab CI

In `.gitlab-ci.yml`:

```yaml
invariant_check:
  stage: test
  script:
    - cargo install invar
    - invar check invariants/
```

### CircleCI

In `.circleci/config.yml`:

```yaml
jobs:
  invariant-check:
    docker:
      - image: rust:latest
    steps:
      - checkout
      - run:
          name: Install Invar
          command: cargo install invar
      - run:
          name: Check invariants
          command: invar check invariants/
```

### Jenkins

In Jenkinsfile:

```groovy
pipeline {
    stages {
        stage('Invariants') {
            steps {
                sh 'cargo install invar'
                sh 'invar check invariants/'
            }
        }
    }
}
```

## Output Formats

### JSON Output

For machine processing:

```bash
invar check --format json invariants/ > report.json
```

Example output:

```json
{
  "status": "pass",
  "timestamp": "2026-02-18T10:30:00Z",
  "invariants": [
    {
      "name": "vault_conservation",
      "passed": true,
      "execution_time_ms": 2.3
    }
  ],
  "summary": {
    "total": 1,
    "passed": 1,
    "failed": 0
  }
}
```

### Markdown Output

For reports:

```bash
invar check --format markdown invariants/ > report.md
```

### SARIF Output

For GitHub Security features:

```bash
invar check --format sarif invariants/ > report.sarif
```

Upload to GitHub:

```yaml
- name: Upload SARIF
  uses: github/codeql-action/upload-sarif@v2
  with:
    sarif_file: report.sarif
```

## Failure Handling

### Fail on Violations

```bash
invar check invariants/ || exit 1
```

### Conditional Failure

```bash
invar check invariants/ > report.json
if grep -q '"failed": 0' report.json; then
  echo "All invariants passed"
  exit 0
else
  echo "Some invariants failed"
  exit 1
fi
```

### Warning Only

```bash
invar check invariants/ || echo "Warning: Invariant violations detected"
```

## Parallel Execution

### Run Multiple Chains

```bash
invar check invariants/ --chain solana &
invar check invariants/ --chain evm &
wait
```

### GitHub Actions Matrix

```yaml
jobs:
  invariants:
    strategy:
      matrix:
        chain: [solana, evm, move]
    steps:
      - run: invar check invariants/ --chain ${{ matrix.chain }}
```

## Caching

### GitHub Actions Cache

```yaml
- uses: actions/cache@v3
  with:
    path: ~/.cache/invar
    key: ${{ runner.os }}-invar-${{ hashFiles('invariants/*') }}
```

## Performance

### Parallel Checking

```bash
# Enable parallel processing (default: yes)
invar check --parallel invariants/

# Disable for debugging
invar check --threads 1 invariants/
```

### Timeout

```bash
timeout 30 invar check invariants/ || {
  if [ $? -eq 124 ]; then
    echo "Invariant check timed out"
    exit 1
  fi
}
```

## Best Practices

1. **Run on every commit** - Catch issues early
2. **Block on failures** - Don't merge broken code
3. **Store reports** - Track history
4. **Keep invariants updated** - As code changes
5. **Review violations** - Understand why they failed

## Troubleshooting

### "invar: command not found"

Ensure installation:

```bash
# In CI environment
cargo install invar

# Or use pre-installed docker image
docker run --rm -v $PWD:/workspace invar:latest check /workspace/invariants/
```

### Timeout Issues

```bash
# Increase timeout
timeout 60 invar check invariants/

# Or check parallelism settings
invar check --threads 4 invariants/
```

### Cache Issues

```bash
# Clear cache
rm -rf ~/.cache/invar

# Rebuild
invar check invariants/ --force
```

## Next Steps

- [Getting Started](getting-started.md) - Set up invariants
- [Writing Invariants](writing-invariants.md) - Create your invariants
- [CI Pipeline Documentation](ci/ci-pipeline.md) - Invar's own CI
