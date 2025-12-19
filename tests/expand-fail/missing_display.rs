use chewie_macros::define_boxed_error;

define_boxed_error! {
    pub MissingDisplay {
        // Missing the required 'display' field
    }
}
