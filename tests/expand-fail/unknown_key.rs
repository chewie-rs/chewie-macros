use chewie_macros::define_boxed_error;

define_boxed_error! {
    pub UnknownKey {
        display: "error",
        unknown_field: "value",
    }
}
