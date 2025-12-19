use chewie_macros::define_boxed_error;

define_boxed_error! {
    pub(crate) CrateError {
        display: "crate-private error",
    }
}
