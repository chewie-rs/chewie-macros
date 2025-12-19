use chewie_macros::define_boxed_error;

define_boxed_error! {
    /// This is a documented error type.
    ///
    /// It demonstrates that doc comments are preserved.
    pub DocumentedError {
        display: "documented error",
    }
}
