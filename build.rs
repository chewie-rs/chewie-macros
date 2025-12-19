// NOTE: This build.rs only affects compilation of this proc-macro crate and its tests.
// It does NOT affect the `send` feature in consuming crates.
//
// The generated code contains `#[cfg(feature = "send")]` which is evaluated in the
// consuming crate's context. Consuming crates must either:
// 1. Have their own build.rs that sets `cargo:rustc-cfg=feature="send"` based on target
// 2. Manually enable the `send` feature in their Cargo.toml
//
// This build.rs exists so that tests within this crate (when written) compile correctly
// for the native target without requiring `--features send`.

fn main() {
    // Tell Cargo about our custom cfg flags so it doesn't warn
    println!("cargo::rustc-check-cfg=cfg(feature, values(\"send\"))");

    // Enable send feature for non-browser targets (same as other crates)
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").ok();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").ok();

    match (target_arch.as_deref(), target_os.as_deref()) {
        (Some("wasm32"), Some("unknown")) => {
            // Browser WASM - no send
        }
        _ => {
            // Native and WASI - enable send
            println!("cargo:rustc-cfg=feature=\"send\"");
        }
    }
}
