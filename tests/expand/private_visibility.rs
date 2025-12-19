use chewie_macros::define_boxed_error;

define_boxed_error! {
    PrivateError {
        display: "private error",
    }
}
