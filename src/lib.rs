#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(clippy::pedantic)]

//! Procedural macros for the chewie-auth ecosystem.
//!
//! This crate provides macros to reduce boilerplate for common patterns,
//! particularly around type-erased (boxed) error types.

use proc_macro::TokenStream;

mod boxed_error;

/// Defines a boxed error type with standard downcast support.
///
/// # Example
///
/// ```no_run
/// use chewie_macros::define_boxed_error;
///
/// define_boxed_error! {
///     pub MyError {
///         display: "my error",
///     }
/// }
/// ```
#[proc_macro]
pub fn define_boxed_error(input: TokenStream) -> TokenStream {
    boxed_error::define_boxed_error_impl(input)
}
