# Rust PR Production Diff Analyzer

## Overview

A semantic analyzer for Rust PR diffs that distinguishes production code from test code, providing meaningful metrics for code review automation.

## Architecture

### Module Structure

```
src/
  lib.rs                    # Public API exports

  error.rs                  # masterror-based error types
  config.rs                 # Configuration loading and validation

  types/
    semantic_unit.rs        # Core types: Function, Struct, Impl, etc.
    change.rs               # Change representation
    classification.rs       # CodeType enum and classification

  git/
    diff_parser.rs          # Unified diff parsing
    hunk.rs                 # Hunk representation

  analysis/
    ast_visitor.rs          # syn-based AST traversal
    extractor.rs            # Extract semantic units from AST
    mapper.rs               # Map diff lines to semantic units

  classifier/
    rules.rs                # Classification rules
    path_classifier.rs      # Path-based classification
    attr_classifier.rs      # Attribute-based classification (#[test], #[cfg(test)])

  output/
    formatter.rs            # Formatter trait
    github.rs               # GitHub Actions output
    json.rs                 # JSON output

bin/
  main.rs                   # CLI entrypoint
```

### Core Types

```rust
enum SemanticUnitKind {
    Function,
    Struct,
    Enum,
    Trait,
    Impl,
    Const,
    Static,
    TypeAlias,
    Macro,
    Module,
}

enum CodeType {
    Production,
    Test,
    TestUtility,
    Benchmark,
    Example,
    BuildScript,
}

enum Visibility {
    Public,
    Crate,
    Restricted,
    Private,
}

struct SemanticUnit {
    kind: SemanticUnitKind,
    name: String,
    visibility: Visibility,
    span: LineSpan,
    attributes: Vec<String>,
}

struct Change {
    file_path: PathBuf,
    unit: SemanticUnit,
    classification: CodeType,
    lines_added: usize,
    lines_removed: usize,
}

struct AnalysisResult {
    changes: Vec<Change>,
    summary: Summary,
}

struct Summary {
    prod_functions: usize,
    prod_structs: usize,
    prod_other: usize,
    test_units: usize,
    weighted_score: usize,
}
```

### Configuration

```toml
# .rust-diff-analyzer.toml

[classification]
test_features = ["test-utils", "testing", "mock"]
test_paths = ["tests/", "benches/", "examples/"]
ignore_paths = ["fixtures/", "test_data/", "snapshots/"]

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

## Algorithm

### Phase 1: Diff Parsing

1. Read unified diff from stdin or file
2. Parse into structured format:
   - File path
   - Hunks with line numbers
   - Added/removed lines

### Phase 2: AST Extraction

For each changed `.rs` file:

1. Read full file content
2. Parse with `syn` into AST
3. Traverse AST, extract all semantic units
4. Record span (start_line, end_line) for each unit

### Phase 3: Classification

For each semantic unit:

1. Check path-based rules (tests/, benches/, examples/)
2. Check attribute-based rules:
   - `#[test]` → Test
   - `#[cfg(test)]` → Test
   - `#[bench]` → Benchmark
   - `#[cfg(feature = "test-utils")]` → TestUtility
3. Check parent module classification
4. Apply default: Production

### Phase 4: Mapping

1. For each changed line in diff
2. Find which semantic unit contains this line
3. Mark unit as changed
4. Aggregate line counts per unit

### Phase 5: Aggregation

1. Group changes by classification
2. Calculate weighted scores
3. Check against limits
4. Generate output

## Dependencies

```toml
[dependencies]
syn = { version = "2", features = ["full", "parsing", "visit", "extra-traits"] }
proc-macro2 = "1"
similar = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
clap = { version = "4", features = ["derive"] }
masterror = "0.1"

[dev-dependencies]
pretty_assertions = "1"
tempfile = "3"
```

## CLI Interface

```bash
# From stdin
git diff HEAD~1 | rust-diff-analyzer

# From file
rust-diff-analyzer --diff-file changes.diff

# With config
rust-diff-analyzer --config .rust-diff-analyzer.toml

# Override output format
rust-diff-analyzer --format json

# Set limits
rust-diff-analyzer --max-units 20 --max-score 50
```

## Output Examples

### GitHub Actions

```
prod_functions_changed=5
prod_structs_changed=2
prod_other_changed=1
test_units_changed=12
weighted_score=23
exceeds_limit=false
```

### JSON

```json
{
  "summary": {
    "prod_functions": 5,
    "prod_structs": 2,
    "prod_other": 1,
    "test_units": 12,
    "weighted_score": 23,
    "exceeds_limit": false
  },
  "changes": [
    {
      "file": "src/parser.rs",
      "unit": "parse_token",
      "kind": "function",
      "visibility": "public",
      "classification": "production",
      "lines_added": 10,
      "lines_removed": 5,
      "weight": 3
    }
  ]
}
```

## Edge Cases

| Case | Handling |
|------|----------|
| Macro definitions | Parse as separate unit, classify by location |
| Proc-macro invocations | Attribute on parent unit |
| Nested modules | Inherit parent classification if `#[cfg(test)]` |
| Multiple impl blocks | Each is separate unit |
| Associated functions | Part of impl block unit |
| Doc-tests | Ignored (part of documentation) |
| Generated code | Classify by output location |
| Conditional compilation | Parse all `#[cfg]` branches |

## Testing Strategy

### Unit Tests

- Diff parser: various diff formats
- AST extractor: all Rust constructs
- Classifier: all classification rules
- Mapper: line-to-unit mapping
- Formatters: output correctness

### Integration Tests

- End-to-end with real Rust files
- Various PR scenarios
- Edge cases

### Property Tests

- Arbitrary valid Rust code
- Arbitrary diffs

## Implementation Order

1. Error types (masterror)
2. Core types (SemanticUnit, Change, etc.)
3. Configuration
4. Diff parser
5. AST visitor and extractor
6. Classifier
7. Mapper
8. Output formatters
9. CLI
10. Integration tests

## Future Enhancements

- Complexity metrics (cyclomatic)
- Breaking change detection
- Git blame integration
- Caching for large repos
- LSP integration
- VS Code extension
