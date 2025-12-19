use chewie_macros::define_boxed_error;

define_boxed_error! {
    pub NoTrailingCommaError {
        display: "error without trailing comma"
    }
}
