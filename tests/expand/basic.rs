use chewie_macros::define_boxed_error;

define_boxed_error! {
    pub MyError {
        display: "my error",
    }
}
