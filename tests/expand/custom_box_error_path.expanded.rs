use chewie_macros::define_boxed_error;
type BoxError = Box<dyn std::error::Error + Send + Sync>;
#[non_exhaustive]
pub struct CustomPathError {
    inner: BoxError,
}
#[automatically_derived]
impl ::core::fmt::Debug for CustomPathError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "CustomPathError",
            "inner",
            &&self.inner,
        )
    }
}
impl std::fmt::Display for CustomPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{0}: {1}", "custom path error", self.inner))
    }
}
impl std::error::Error for CustomPathError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.inner.as_ref())
    }
}
#[allow(unexpected_cfgs)]
impl CustomPathError {
    /// Attempts to downcast the error to a concrete type.
    #[must_use]
    pub fn downcast_ref<E: std::error::Error + 'static>(&self) -> Option<&E> {
        self.inner.downcast_ref::<E>()
    }
    /// Consumes the error and attempts to downcast to a concrete type.
    ///
    /// # Errors
    /// Returns the original error if the downcast fails.
    pub fn downcast<E: std::error::Error + 'static>(
        self,
    ) -> ::core::result::Result<E, Self> {
        self.inner
            .downcast::<E>()
            .map(|boxed| *boxed)
            .map_err(|inner| CustomPathError { inner })
    }
    /// Returns a reference to the underlying error.
    #[must_use]
    pub fn inner(&self) -> &dyn std::error::Error {
        self.inner.as_ref()
    }
}
