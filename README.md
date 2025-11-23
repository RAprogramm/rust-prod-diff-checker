# Rust Diff Analyzer

Semantic analyzer for Rust PR diffs that distinguishes production code from test code using AST analysis.

## Features

- **Semantic Analysis**: Uses Rust AST parsing to identify code units (functions, structs, enums, traits, impl blocks)
- **Smart Classification**: Distinguishes between production, test, benchmark, and example code
- **Weighted Scoring**: Configurable weights for different code types
- **Multiple Output Formats**: GitHub Actions, JSON, human-readable
- **Configurable Limits**: Set thresholds for production code changes

## Installation

```bash
cargo install rust-diff-analyzer
```

Or build from source:

```bash
git clone https://github.com/RAprogramm/rust-diff-analyzer
cd rust-diff-analyzer
cargo build --release
```

## Usage

### Command Line

```bash
# Analyze diff from file
rust-diff-analyzer --diff-file changes.diff

# Analyze diff from stdin
git diff HEAD~1 | rust-diff-analyzer

# With custom limits
rust-diff-analyzer --diff-file changes.diff --max-units 50 --max-score 200

# JSON output
rust-diff-analyzer --diff-file changes.diff --format json
```

### GitHub Action

```yaml
name: PR Analysis

on:
  pull_request:

jobs:
  analyze:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Analyze PR
        uses: RAprogramm/rust-diff-analyzer@v1
        with:
          max_prod_units: 30
          max_weighted_score: 100
          fail_on_exceed: true
```

### Configuration File

Create `.rust-diff-analyzer.toml`:

```toml
[classification]
test_features = ["test-utils", "testing", "mock"]
test_paths = ["tests/", "benches/", "examples/"]
ignore_paths = ["generated/"]

[weights]
public_function = 3
private_function = 1
public_struct = 3
private_struct = 1
impl_block = 2
trait_definition = 4
const_static = 1

[limits]
max_prod_units = 30
max_weighted_score = 100
fail_on_exceed = true

[output]
format = "github"
include_details = true
```

## Output

### GitHub Actions Format

```
prod_functions_changed=5
prod_structs_changed=2
prod_other_changed=1
test_units_changed=10
prod_lines_added=150
prod_lines_removed=30
test_lines_added=200
test_lines_removed=50
weighted_score=23
exceeds_limit=false
```

### JSON Format

```json
{
  "summary": {
    "prod_functions": 5,
    "prod_structs": 2,
    "prod_other": 1,
    "test_units": 10,
    "prod_lines_added": 150,
    "prod_lines_removed": 30,
    "test_lines_added": 200,
    "test_lines_removed": 50,
    "weighted_score": 23,
    "exceeds_limit": false
  },
  "changes": [...]
}
```

## Code Classification

The analyzer classifies code based on:

1. **File path**: `tests/`, `benches/`, `examples/`
2. **Attributes**: `#[test]`, `#[bench]`, `#[cfg(test)]`
3. **Module context**: Code inside `mod tests`

| Classification | Description |
|---------------|-------------|
| Production | Regular production code |
| Test | Test functions and test utilities |
| Benchmark | Benchmark code |
| Example | Example code |
| BuildScript | build.rs files |

## Weighted Scoring

Each code unit contributes to a weighted score:

| Unit Type | Public | Private |
|-----------|--------|---------|
| Function | 3 | 1 |
| Struct | 3 | 1 |
| Enum | 3 | 1 |
| Trait | 4 | 4 |
| Impl Block | 2 | 2 |
| Const/Static | 1 | 1 |

## API Usage

```rust
use rust_diff_analyzer::{
    analysis::map_changes,
    config::Config,
    git::parse_diff,
};

fn main() -> Result<(), rust_diff_analyzer::AppError> {
    let diff = std::fs::read_to_string("changes.diff")?;
    let config = Config::default();

    let file_diffs = parse_diff(&diff)?;
    let changes = map_changes(&file_diffs, &config, |path| {
        std::fs::read_to_string(path)
    })?;

    for change in changes {
        println!("{}: {} ({})",
            change.file_path.display(),
            change.unit.name,
            change.classification.as_str()
        );
    }

    Ok(())
}
```

## License

MIT License - see [LICENSE](LICENSE) for details.
