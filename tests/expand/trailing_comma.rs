use chewie_macros::define_boxed_error;

define_boxed_error! {
    pub TrailingCommaError {
        display: "error with trailing comma",
    }
}
