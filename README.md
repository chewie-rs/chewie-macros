# chewie-macros

Procedural macros for the chewie-auth ecosystem.

This crate provides compile-time code generation to reduce boilerplate and improve developer experience.

## Macros

### `define_boxed_error!`

Generates a type-erased error type with downcast support.

```rust
use chewie_macros::define_boxed_error;

define_boxed_error! {
    /// HTTP client error.
    pub HttpError {
        display: "HTTP error",
    }
}
```

This generates:
- Error struct with `Box<dyn Error>` field (with `Send + Sync` bounds when `send` feature is enabled)
- `Display` and `Error` trait implementations
- `downcast_ref<E>()` and `downcast<E>()` methods for error inspection
- `inner()` method to access the underlying error

#### Example: Basic Usage

```rust
use chewie_macros::define_boxed_error;

define_boxed_error! {
    /// Error returned by the token validation layer.
    pub TokenError {
        display: "token validation failed",
    }
}

// Create from any error type
fn validate_token(token: &str) -> Result<(), TokenError> {
    let parsed: serde_json::Value = serde_json::from_str(token)
        .map_err(|e| TokenError { inner: Box::new(e) })?;
    Ok(())
}

// Downcast to inspect the underlying error
fn handle_error(err: TokenError) {
    if let Some(json_err) = err.downcast_ref::<serde_json::Error>() {
        eprintln!("JSON parsing failed: {}", json_err);
    }
}
```

#### Example: Custom Box Type

You can specify a custom box error path if you have your own error type:

```rust
use chewie_macros::define_boxed_error;

// Assuming you have a custom BoxError type alias
type BoxError = Box<dyn std::error::Error + Send + Sync>;

define_boxed_error! {
    /// Client error with custom box type.
    pub ClientError {
        display: "client error",
        box_error_path: BoxError,
    }
}
```

## Features

- `send` - Controls `Send + Sync` bounds in generated code. Automatically enabled by `build.rs` for non-browser targets (native and WASI). Browser WASM targets (`wasm32-unknown-unknown`) do not enable this feature.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
chewie-macros = "0.1"
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
