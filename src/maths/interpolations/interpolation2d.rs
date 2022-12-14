use crate::types::{Real, Size};

use super::extrapolator::Extrapolator;

/// 2-D interpolation
///
/// Provide interpolated values from two sequences of length `N` and `M`,
/// representing the discretized values of the `x` and `y` variables, and a `N x M`
/// matrix representing the tabulated function values.
///
///
pub trait Interpolation2D: Extrapolator {
    fn xmin() -> Real;

    fn xmax() -> Real;

    fn xvalues() -> Vec<Real>;

    fn ymin() -> Real;

    fn ymax() -> Real;

    fn yvalues() -> Vec<Real>;

    fn is_in_range(x: Real, y: Real) -> bool;

    fn locate_x(x: Real) -> Size;

    fn locate_y(x: Real) -> Size;
}

pub type FirstArgumentType = Real;
pub type SecondArgumentType = Real;
pub type ResultType = Real;
