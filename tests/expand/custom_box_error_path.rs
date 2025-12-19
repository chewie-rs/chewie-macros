use chewie_macros::define_boxed_error;

// Simulate a custom BoxError type
type BoxError = Box<dyn std::error::Error + Send + Sync>;

define_boxed_error! {
    pub CustomPathError {
        display: "custom path error",
        box_error_path: BoxError,
    }
}
