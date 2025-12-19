# Macro Tests

This directory contains tests for the `chewie-macros` crate using [`macrotest`](https://github.com/eupn/macrotest).

## Test Structure

### Pass Tests (`tests/expand/`)

These tests verify that the macro expands correctly for valid inputs:

- **`basic.rs`** - Simple public error type
- **`with_docs.rs`** - Error type with doc comments
- **`custom_box_error_path.rs`** - Using custom BoxError type path
- **`private_visibility.rs`** - Private (no visibility modifier) error type
- **`pub_crate_visibility.rs`** - Crate-private error type
- **`trailing_comma.rs`** - With trailing comma in macro body
- **`no_trailing_comma.rs`** - Without trailing comma in macro body

### Fail Tests (`tests/expand-fail/`)

These tests verify that the macro fails gracefully with appropriate errors:

- **`missing_display.rs`** - Missing required `display` field
- **`unknown_key.rs`** - Unknown field in macro body
- **`missing_braces.rs`** - Missing curly braces for macro body
- **`empty_body.rs`** - Empty macro body
- **`invalid_syntax.rs`** - Invalid syntax (missing colon)

## Running Tests

```bash
# Run all tests
cargo test

# Run only macro tests
cargo test --test macro_tests

# Review snapshots (*.expanded.rs files are auto-generated)
ls tests/expand/*.expanded.rs
```

## Updating Snapshots

If you intentionally change the macro expansion output:

```bash
# Delete old snapshots
rm tests/expand/*.expanded.rs tests/expand-fail/*.expanded.rs

# Regenerate
cargo test --test macro_tests
```

## How It Works

`macrotest` compiles each test file and captures the expanded output. For pass tests, it verifies the expansion matches the snapshot. For fail tests, it verifies compilation fails (resulting in empty `.expanded.rs` files).
