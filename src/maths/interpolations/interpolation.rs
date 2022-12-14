use crate::types::{Real, Size};

/// 1-D interpolation.
///
/// Provide interpolated values from two sequences of equal length, representing discretized
/// values of a variable and a function of the former, respectively.
pub trait Interpolation {
    /// Integral of the function from `xmin` to `x`
    fn primitive(&self, x: Real) -> Real {
        self.primitive_with_extrapolation(x, false)
    }

    /// Integral of the function from `xmin` to `x`. If `allow_extrapolation` is true
    /// then values of `x` outside the domain of the interpolation range are allowed.
    fn primitive_with_extrapolation(&self, x: Real, allow_extrapolation: bool) -> Real;

    /// Derivative of the function at x
    fn derivative(&self, x: Real) -> Real {
        self.derivative_with_extrapolation(x, false)
    }

    /// Derivative of the function at x. If `allow_extrapolation` is true
    /// then values of `x` outside the domain of the interpolation range are allowed.    
    fn derivative_with_extrapolation(&self, x: Real, allow_extrapolation: bool) -> Real;

    /// Second derivative of the function at x
    fn second_derivative(&self, x: Real) -> Real {
        self.second_derivative_with_extrapolation(x, false)
    }

    /// Second derivative of the function at x. If `allow_extrapolation` is true
    /// then values of `x` outside the domain of the interpolation range are allowed.        
    fn second_derivative_with_extrapolation(&self, x: Real, allow_extrapolation: bool) -> Real;

    /// Minimum value of x
    fn xmin(&self) -> Real;

    /// Maximum value of x
    fn xmax(&self) -> Real;

    fn value(&self, x: Real) -> Real {
        self.value_with_extrapolation(x, false)
    }

    /// Interpolated value at `x`
    fn value_with_extrapolation(&self, x: Real, allow_extrapolation: bool) -> Real;

    /// Returns whether the given `x` is in the range of acceptable `x` values
    fn is_in_range(&self, x: Real) -> bool;

    /// Find the index into the table of `x` values to start the interpolation
    fn locate(&self, x: Real) -> Size;

    /// Update internal state
    fn update(&mut self);

    /// Check whether `x` is in the range of the interpolation domain
    fn check_range(&self, x: Real, allow_extrapolation: bool) {
        assert!(
            allow_extrapolation || self.is_in_range(x),
            "interpolation range is [{}, {}]: extrapolation at {} is not allowed",
            self.xmin(),
            self.xmax(),
            x
        );
    }
}
