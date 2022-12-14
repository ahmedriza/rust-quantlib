use crate::types::Real;

/// A Quote is a market observable
pub trait Quote {
    /// Get the current value
    fn value(&self) -> Real;

    /// Returns true if the Quote holds a valid value
    fn is_valid(&self) -> bool;
}
