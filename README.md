# Rust Diff Analyzer

[![CI](https://github.com/RAprogramm/rust-prod-diff-checker/actions/workflows/ci.yml/badge.svg)](https://github.com/RAprogramm/rust-prod-diff-checker/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/RAprogramm/rust-prod-diff-checker)](https://github.com/RAprogramm/rust-prod-diff-checker/releases/latest)
[![Crates.io](https://img.shields.io/crates/v/rust-diff-analyzer)](https://crates.io/crates/rust-diff-analyzer)
[![docs.rs](https://img.shields.io/docsrs/rust-diff-analyzer)](https://docs.rs/rust-diff-analyzer)
[![codecov](https://codecov.io/gh/RAprogramm/rust-prod-diff-checker/graph/badge.svg?token=cbXm5iD9PQ)](https://codecov.io/gh/RAprogramm/rust-prod-diff-checker)
[![Hits-of-Code](https://hitsofcode.com/github/RAprogramm/rust-prod-diff-checker?branch=main&exclude=Cargo.lock,.gitignore,CHANGELOG.md)](https://hitsofcode.com/github/RAprogramm/rust-prod-diff-checker/view?branch=main&exclude=Cargo.lock,.gitignore,CHANGELOG.md)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/RAprogramm/rust-prod-diff-checker/blob/main/LICENSES/MIT.txt)
[![REUSE](https://api.reuse.software/badge/github.com/RAprogramm/rust-prod-diff-checker)](https://api.reuse.software/info/github.com/RAprogramm/rust-prod-diff-checker)

A tool that analyzes Pull Requests in Rust projects and enforces PR size limits by distinguishing production code from test code. It uses Rust AST (Abstract Syntax Tree) analysis to count only meaningful production code changes, ignoring tests, benchmarks, and examples.

**Why limit PR size?** Large PRs are harder to review, more likely to contain bugs, and slow down the development process. This tool helps teams enforce reasonable PR sizes automatically.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Command Line](#command-line)
  - [GitHub Action](#github-action)
  - [PR Comments](#pr-comments)
  - [Configuration File](#configuration-file)
- [Output](#output)
  - [GitHub Actions Format](#github-actions-format)
  - [JSON Format](#json-format)
- [Code Classification](#code-classification)
- [Weighted Scoring](#weighted-scoring)
- [API Usage](#api-usage)
- [Coverage](#coverage)
- [License](#license)

## Features

- **Semantic Analysis**: Parses Rust code to identify code units (functions, structs, enums, traits, impl blocks) rather than just counting lines
- **Smart Classification**: Automatically distinguishes between production, test, benchmark, and example code
- **Weighted Scoring**: Assigns different weights to different code types (public functions are worth more than private ones)
- **Flexible Limits**: Set global limits, per-type limits (e.g., max 5 functions), and line-based limits
- **PR Comments**: Automatically posts formatted analysis results as comments on pull requests
- **Multiple Output Formats**: GitHub Actions outputs, JSON for integration, human-readable text, or markdown comments

<div align="right"><a href="#table-of-contents">↑ Back to top</a></div>

## Installation

### From crates.io

```bash
cargo install rust-diff-analyzer
```

### From source

```bash
git clone https://github.com/RAprogramm/rust-prod-diff-checker
cd rust-prod-diff-checker
cargo build --release
```

The binary will be available at `target/release/rust-diff-analyzer`.

<div align="right"><a href="#table-of-contents">↑ Back to top</a></div>

## Usage

### Command Line

The tool reads a diff (output from `git diff`) and analyzes the Rust code changes.

```bash
# Analyze diff from a file
rust-diff-analyzer --diff-file changes.diff

# Pipe diff from git directly
git diff HEAD~1 | rust-diff-analyzer

# Compare with a specific branch
git diff main | rust-diff-analyzer

# Set custom limits
rust-diff-analyzer --diff-file changes.diff --max-units 50 --max-score 200 --max-lines 300

# Different output formats
rust-diff-analyzer --diff-file changes.diff --format json      # Machine-readable JSON
rust-diff-analyzer --diff-file changes.diff --format human     # Human-readable text
rust-diff-analyzer --diff-file changes.diff --format comment   # Markdown for PR comments
rust-diff-analyzer --diff-file changes.diff --format github    # GitHub Actions outputs (default)
```

The tool will exit with code 1 if limits are exceeded (useful for CI).

<div align="right"><a href="#table-of-contents">↑ Back to top</a></div>

### GitHub Action

Add this workflow to your repository at `.github/workflows/pr-analysis.yml`:

```yaml
name: PR Analysis

on:
  pull_request:

jobs:
  analyze:
    name: Analyze PR Size
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: write  # Required for posting comments
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        with:
          fetch-depth: 0  # Required to get full git history for diff

      - name: Analyze PR
        uses: RAprogramm/rust-prod-diff-checker@v1
        with:
          max_prod_units: 30        # Max production code units
          max_weighted_score: 100   # Max weighted score
          fail_on_exceed: true      # Fail CI if limits exceeded
          post_comment: true        # Post results as PR comment
```

#### Action Inputs

| Input | Description | Default |
|-------|-------------|---------|
| `max_prod_units` | Maximum number of production code units (functions, structs, etc.) allowed in a single PR | `30` |
| `max_weighted_score` | Maximum weighted score allowed. Public items have higher weights than private ones | `100` |
| `max_prod_lines` | Maximum number of production code lines added. Leave empty for no limit | - |
| `fail_on_exceed` | Whether to fail the CI job if any limit is exceeded | `true` |
| `post_comment` | Post analysis results as a comment on the PR | `false` |
| `update_comment` | Update existing comment instead of creating a new one on each push | `true` |
| `output_format` | Output format: `github` (Actions outputs), `json`, or `human` | `github` |
| `config_file` | Path to custom configuration file | - |

#### Action Outputs

The action provides these outputs that can be used in subsequent steps:

- `prod_functions_changed` - Number of production functions changed
- `prod_structs_changed` - Number of production structs changed
- `prod_other_changed` - Number of other production units changed
- `test_units_changed` - Number of test units changed
- `prod_lines_added` - Lines added in production code
- `prod_lines_removed` - Lines removed from production code
- `test_lines_added` - Lines added in test code
- `test_lines_removed` - Lines removed from test code
- `weighted_score` - Calculated weighted score
- `exceeds_limit` - Whether any limit was exceeded (`true`/`false`)

<div align="right"><a href="#table-of-contents">↑ Back to top</a></div>

### PR Comments

When `post_comment: true` is set, the action posts a formatted comment on the PR:

```yaml
- name: Analyze PR
  uses: RAprogramm/rust-prod-diff-checker@v1
  with:
    post_comment: true
    update_comment: true  # Updates same comment instead of creating new ones
```

The comment includes:
- **Summary table**: Production vs test metrics at a glance
- **Weighted score**: Total score with pass/fail indicator
- **Changed units**: Collapsible list showing exactly which functions/structs changed

This provides reviewers with immediate context about the PR's scope.

<div align="right"><a href="#table-of-contents">↑ Back to top</a></div>

### Configuration File

For fine-grained control, create `.rust-diff-analyzer.toml` in your project root:

```toml
# Classification settings - what counts as test code
[classification]
# Cargo features that indicate test code
test_features = ["test-utils", "testing", "mock"]
# Directories containing test code (trailing slash important)
test_paths = ["tests/", "benches/", "examples/"]
# Paths to completely ignore in analysis
ignore_paths = ["generated/", "vendor/"]

# Weight settings - how much each item type contributes to the score
[weights]
public_function = 3      # Public functions are significant API surface
private_function = 1     # Private functions are implementation details
public_struct = 3        # Public structs define the API
private_struct = 1
impl_block = 2           # Impl blocks add methods
trait_definition = 4     # Traits are major abstractions
const_static = 1         # Constants are minor

# Limit settings
[limits]
max_prod_units = 30          # Maximum production units per PR
max_weighted_score = 100     # Maximum weighted score per PR
max_prod_lines = 200         # Maximum production lines added
fail_on_exceed = true        # Fail CI when exceeded

# Per-type limits for fine-grained control (all optional)
[limits.per_type]
functions = 5        # Max 5 functions per PR
structs = 3          # Max 3 structs per PR
enums = 3
traits = 2           # Traits are complex, limit to 2
impl_blocks = 10
consts = 5
statics = 2
type_aliases = 3
macros = 2
modules = 3

# Output settings
[output]
format = "github"        # Default output format
include_details = true   # Include list of changed units in output
```

<div align="right"><a href="#table-of-contents">↑ Back to top</a></div>

## Output

### GitHub Actions Format

This format is designed for GitHub Actions. Each line sets an output variable:

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

Machine-readable format for integration with other tools:

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
  "changes": [
    {
      "file_path": "src/lib.rs",
      "unit": {
        "name": "parse_config",
        "kind": "Function",
        "visibility": "Public"
      },
      "classification": "Production",
      "lines_added": 20,
      "lines_removed": 5
    }
  ]
}
```

<div align="right"><a href="#table-of-contents">↑ Back to top</a></div>

## Code Classification

The analyzer automatically classifies code into categories:

### Classification Rules

1. **File path**: Code in `tests/`, `benches/`, or `examples/` directories is not production
2. **Attributes**: Functions with `#[test]`, `#[bench]`, or `#[cfg(test)]` are tests
3. **Module context**: Code inside `mod tests { }` blocks is test code

### Classification Types

| Classification | Description | Counts toward limits? |
|---------------|-------------|----------------------|
| Production | Regular production code | Yes |
| Test | Test functions and utilities | No |
| Benchmark | Performance benchmarks | No |
| Example | Example code in `examples/` | No |
| BuildScript | `build.rs` files | No |

This means you can add as many tests as you want without affecting your PR size limits!

<div align="right"><a href="#table-of-contents">↑ Back to top</a></div>

## Weighted Scoring

Not all code changes are equal. The weighted scoring system assigns different values to different types of changes:

| Unit Type | Public | Private | Rationale |
|-----------|--------|---------|-----------|
| Function | 3 | 1 | Public functions are API surface |
| Struct | 3 | 1 | Public structs define data contracts |
| Enum | 3 | 1 | Public enums are often API types |
| Trait | 4 | 4 | Traits are major abstractions |
| Impl Block | 2 | 2 | Adds behavior to types |
| Const/Static | 1 | 1 | Minor configuration |

**Example**: A PR that adds:
- 2 public functions (2 × 3 = 6)
- 1 private function (1 × 1 = 1)
- 1 public struct (1 × 3 = 3)

Total weighted score: **10**

<div align="right"><a href="#table-of-contents">↑ Back to top</a></div>

## API Usage

You can use this tool as a library in your own Rust projects:

```rust
use rust_diff_analyzer::{
    analysis::map_changes,
    config::Config,
    git::parse_diff,
};

fn main() -> Result<(), rust_diff_analyzer::AppError> {
    // Read the diff
    let diff = std::fs::read_to_string("changes.diff")?;

    // Use default configuration or load from file
    let config = Config::default();

    // Parse the diff into structured data
    let file_diffs = parse_diff(&diff)?;

    // Map changes to semantic units
    // The closure provides file content for AST parsing
    let changes = map_changes(&file_diffs, &config, |path| {
        std::fs::read_to_string(path)
    })?;

    // Process the results
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

Add to your `Cargo.toml`:

```toml
[dependencies]
rust-diff-analyzer = "1.1"
```

<div align="right"><a href="#table-of-contents">↑ Back to top</a></div>

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

<div align="right"><a href="#table-of-contents">↑ Back to top</a></div>

## License

MIT License - see [LICENSE](LICENSE) for details.

<div align="right"><a href="#table-of-contents">↑ Back to top</a></div>
