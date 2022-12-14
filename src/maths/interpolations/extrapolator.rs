/// Base trait for extrapolators
pub trait Extrapolator {
    /// Enable extrapolation in subsequent calls
    fn enable_extrapolation();

    /// Disable extrapolation in subsequent calls
    fn disable_extrapolation();

    /// Tells whether extrapolation is enabled
    fn allows_extrapolation();
}
