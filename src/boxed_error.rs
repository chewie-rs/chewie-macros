//! Implementation of the `define_boxed_error!` macro.

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Ident, LitStr, Path, Result, Token, Visibility,
    parse::{Parse, ParseStream},
};

/// Input for the `define_boxed_error!` macro.
struct BoxedErrorInput {
    /// Doc comments and attributes
    attrs: Vec<Attribute>,
    /// Visibility (pub, pub(crate), etc.)
    vis: Visibility,
    /// Name of the error type
    name: Ident,
    /// Display message
    display: LitStr,
    /// Path to `BoxError` type (optional, defaults to `Box<dyn std::error::Error + ...>`).
    box_error_path: Option<Path>,
}

impl Parse for BoxedErrorInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse attributes (doc comments, etc.)
        let attrs = input.call(Attribute::parse_outer)?;

        // Parse visibility
        let vis: Visibility = input.parse()?;

        // Parse the struct name
        let name: Ident = input.parse()?;

        // Parse the body: { display: "...", box_error_path: crate::BoxError }
        let content;
        syn::braced!(content in input);

        let mut display = None;
        let mut box_error_path = None;

        while !content.is_empty() {
            let key: Ident = content.parse()?;
            content.parse::<Token![:]>()?;

            match key.to_string().as_ref() {
                "display" => {
                    display = Some(content.parse::<LitStr>()?);
                }
                "box_error_path" => {
                    box_error_path = Some(content.parse::<Path>()?);
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        key,
                        "unknown key; expected 'display' or 'box_error_path'",
                    ));
                }
            }

            // Parse optional comma (allow trailing)
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        let display = display.ok_or_else(|| input.error("missing 'display' field"))?;

        Ok(BoxedErrorInput {
            attrs,
            vis,
            name,
            display,
            box_error_path,
        })
    }
}

pub fn define_boxed_error_impl(input: TokenStream) -> TokenStream {
    let BoxedErrorInput {
        attrs,
        vis,
        name,
        display,
        box_error_path,
    } = syn::parse_macro_input!(input as BoxedErrorInput);

    let display_msg = display.value();

    // Generate the struct definition with conditional inner type
    let struct_def = if let Some(path) = box_error_path {
        // User specified a custom type
        quote! {
            #(#attrs)*
            #[derive(Debug)]
            #[non_exhaustive]
            #vis struct #name {
                inner: #path,
            }
        }
    } else {
        // Use feature-aware default
        quote! {
            #(#attrs)*
            #[derive(Debug)]
            #[non_exhaustive]
            #[allow(unexpected_cfgs)]
            #vis struct #name {
                #[cfg(feature = "send")]
                inner: Box<dyn std::error::Error + Send + Sync>,
                #[cfg(not(feature = "send"))]
                inner: Box<dyn std::error::Error>,
            }
        }
    };

    let expanded = quote! {
        #struct_def

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}: {}", #display_msg, self.inner)
            }
        }

        impl std::error::Error for #name {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                Some(self.inner.as_ref())
            }
        }

        #[allow(unexpected_cfgs)]
        impl #name {
            /// Attempts to downcast the error to a concrete type.
            #[must_use]
            pub fn downcast_ref<E: std::error::Error + 'static>(&self) -> Option<&E> {
                self.inner.downcast_ref::<E>()
            }

            /// Consumes the error and attempts to downcast to a concrete type.
            ///
            /// # Errors
            /// Returns the original error if the downcast fails.
            pub fn downcast<E: std::error::Error + 'static>(self) -> ::core::result::Result<E, Self> {
                self.inner
                    .downcast::<E>()
                    .map(|boxed| *boxed)
                    .map_err(|inner| #name { inner })
            }

            /// Returns a reference to the underlying error.
            #[must_use]
            #[cfg(feature = "send")]
            pub fn inner(&self) -> &(dyn std::error::Error + Send + Sync) {
                self.inner.as_ref()
            }

            /// Returns a reference to the underlying error.
            #[must_use]
            #[cfg(not(feature = "send"))]
            pub fn inner(&self) -> &dyn std::error::Error {
                self.inner.as_ref()
            }
        }
    };

    TokenStream::from(expanded)
}
