# Rust Diff Analyzer

[![CI](https://github.com/RAprogramm/rust-prod-diff-checker/actions/workflows/ci.yml/badge.svg)](https://github.com/RAprogramm/rust-prod-diff-checker/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/RAprogramm/rust-prod-diff-checker)](https://github.com/RAprogramm/rust-prod-diff-checker/releases/latest)
[![codecov](https://codecov.io/gh/RAprogramm/rust-prod-diff-checker/graph/badge.svg?token=cbXm5iD9PQ)](https://codecov.io/gh/RAprogramm/rust-prod-diff-checker)
[![Hits-of-Code](https://hitsofcode.com/github/RAprogramm/rust-prod-diff-checker?branch=main&exclude=Cargo.lock,.gitignore,CHANGELOG.md)](https://hitsofcode.com/github/RAprogramm/rust-prod-diff-checker/view?branch=main&exclude=Cargo.lock,.gitignore,CHANGELOG.md)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/RAprogramm/rust-prod-diff-checker/blob/main/LICENSES/MIT.txt)
[![REUSE](https://api.reuse.software/badge/github.com/RAprogramm/rust-prod-diff-checker)](https://api.reuse.software/info/github.com/RAprogramm/rust-prod-diff-checker)

Analyze and limit PR size by distinguishing production code from test code using Rust AST analysis. Helps enforce PR size limits by counting only meaningful production code changes, not tests.

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
        uses: RAprogramm/rust-prod-diff-checker@v1
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
max_prod_lines = 200
fail_on_exceed = true

# Optional: per-type limits for fine-grained control
[limits.per_type]
functions = 5
structs = 3
traits = 2
impl_blocks = 10

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

## Coverage

<details>
<summary>Coverage Graphs</summary>

### Sunburst

The inner-most circle is the entire project, moving away from the center are folders then, finally, a single file. The size and color of each slice is representing the number of statements and the coverage, respectively.

![Sunburst](https://codecov.io/gh/RAprogramm/rust-prod-diff-checker/graphs/sunburst.svg?token=cbXm5iD9PQ)

### Grid

Each block represents a single file in the project. The size and color of each block is represented by the number of statements and the coverage, respectively.

![Grid](https://codecov.io/gh/RAprogramm/rust-prod-diff-checker/graphs/tree.svg?token=cbXm5iD9PQ)

### Icicle

The top section represents the entire project. Proceeding with folders and finally individual files. The size and color of each slice is representing the number of statements and the coverage, respectively.

![Icicle](https://codecov.io/gh/RAprogramm/rust-prod-diff-checker/graphs/icicle.svg?token=cbXm5iD9PQ)

</details>

## License

MIT License - see [LICENSE](LICENSE) for details.
