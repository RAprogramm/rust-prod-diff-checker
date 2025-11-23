<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# Contributing to Rust Diff Analyzer

Thank you for your interest in contributing to this project!

## Code Style & Standards

This project follows the [RustManifest](https://github.com/RAprogramm/RustManifest) coding standards. Please read it thoroughly before contributing.

Key points:
- Use `cargo +nightly fmt` for formatting
- No `unwrap()` or `expect()` in production code
- Documentation via Rustdoc only (no inline comments)
- Descriptive naming conventions

## Development Setup

### Prerequisites

- Rust nightly toolchain
- cargo-make (optional, for task automation)
- cargo-nextest (for running tests)

### Installation

```bash
git clone https://github.com/RAprogramm/rust-prod-diff-checker
cd rust-prod-diff-checker

# Install nightly toolchain
rustup toolchain install nightly
rustup component add rustfmt --toolchain nightly
rustup component add clippy

# Install test runner (optional but recommended)
cargo install cargo-nextest
```

### Pre-commit Checks

Before committing, ensure all checks pass:

```bash
# Format check
cargo +nightly fmt --all -- --check

# Linting
cargo clippy --all-targets --all-features -- -D warnings

# Tests
cargo test --all-features

# Or with nextest
cargo nextest run --all-features
```

## Git Workflow

### Branch Naming

Use issue number as branch name:
```
123
```

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add new output format
fix: correct line counting in parser
docs: update API examples
test: add property tests for extractor
refactor: simplify config loading
chore: update dependencies
```

For issue-linked commits:
```
feat: implement weighted scoring (#123)
```

### Pull Requests

1. Create branch from `main`
2. Make your changes
3. Ensure all CI checks pass
4. Create PR with descriptive title
5. Include `Closes #<issue>` in description

## Testing

### Test Organization

```
tests/
├── integration.rs    # Integration tests
├── property.rs       # Property-based tests
└── fixtures/         # Test data
```

### Writing Tests

- Cover all public API functions
- Test error paths, not just happy paths
- Use property-based testing for parsers
- No `unwrap()` in tests - use `?` with proper error types

```rust
#[test]
fn test_parse_diff() -> Result<(), Box<dyn std::error::Error>> {
    let diff = include_str!("fixtures/sample.diff");
    let result = parse_diff(diff)?;

    assert_eq!(result.len(), 3);
    Ok(())
}
```

### Running Tests

```bash
# All tests
cargo test --all-features

# With coverage
cargo llvm-cov nextest --all-features

# Specific test
cargo test test_parse_diff

# Property tests only
cargo test --test property
```

## CI/CD Pipeline

### Automated Checks

Every PR triggers:

| Job | Description |
|-----|-------------|
| Format | `cargo +nightly fmt --check` |
| Clippy | `cargo clippy -D warnings` |
| Test | `cargo test --all-features` |
| Doc | `cargo doc --no-deps` |
| Coverage | Upload to Codecov |
| Benchmark | Compile benchmarks |
| Audit | Security vulnerability scan |
| REUSE | License compliance |

### Coverage Requirements

- Project target: auto (maintain current level)
- Patch target: 80% (new code must be well-tested)

## Architecture

### Module Structure

```
src/
├── lib.rs              # Public API exports
├── error.rs            # Error types (AppError)
├── config.rs           # Configuration handling
├── analysis/
│   ├── mod.rs          # Analysis module
│   ├── extractor.rs    # AST extraction
│   └── mapper.rs       # Change mapping
├── git/
│   └── diff_parser.rs  # Git diff parsing
└── output/
    ├── formatter.rs    # Output formatting
    ├── github.rs       # GitHub Actions format
    └── json.rs         # JSON format
```

### Key Types

- `CodeUnit` - Represents a code element (function, struct, etc.)
- `CodeChange` - A change to a code unit with classification
- `Classification` - Production, Test, Benchmark, Example
- `Config` - Runtime configuration

## Adding Features

### New Output Format

1. Create `src/output/newformat.rs`
2. Implement `OutputFormatter` trait
3. Add to `OutputFormat` enum in `config.rs`
4. Register in `src/output/formatter.rs`
5. Add tests and documentation

### New Code Unit Type

1. Add variant to `UnitKind` in `src/analysis/extractor.rs`
2. Implement extraction in `extract_units()`
3. Add weight in `config.rs`
4. Update tests

## Release Process

Releases are automated via CI on tag push:

1. Update version in `Cargo.toml`
2. Commit: `chore(release): prepare v1.x.x`
3. Create and push tag:
   ```bash
   git tag v1.x.x
   git push origin v1.x.x
   ```
4. CI builds binaries for all platforms
5. GitHub Release is created automatically
6. Changelog is updated

### Versioning

Follow [Semantic Versioning](https://semver.org/):
- MAJOR: Breaking API changes
- MINOR: New features, backward compatible
- PATCH: Bug fixes

## Documentation

### Code Documentation

All public items must have Rustdoc:

```rust
/// Parses a unified diff string into structured file diffs.
///
/// # Errors
///
/// Returns `AppError::DiffParse` if the diff format is invalid.
///
/// # Examples
///
/// ```
/// use rust_diff_analyzer::git::parse_diff;
///
/// let diff = "diff --git a/foo.rs b/foo.rs\n...";
/// let files = parse_diff(diff)?;
/// # Ok::<(), rust_diff_analyzer::AppError>(())
/// ```
pub fn parse_diff(input: &str) -> Result<Vec<FileDiff>, AppError> {
    // ...
}
```

### README Updates

Update README.md when:
- Adding new CLI options
- Changing configuration format
- Adding new output formats
- Modifying GitHub Action inputs/outputs

## Getting Help

- Open an issue for bugs or feature requests
- Check existing issues before creating new ones
- Provide minimal reproduction for bugs

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
