pub trait TraceErrorExt<T, E: std::fmt::Display> {
    fn trace_err(self) -> Result<T, E>;
}

impl<T, E: std::fmt::Display> TraceErrorExt<T, E> for Result<T, E> {
    fn trace_err(self) -> Result<T, E> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                tracing::error!(error = %e);
                Err(e)
            }
        }
    }
}
