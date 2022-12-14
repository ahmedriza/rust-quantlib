use crate::types::{Real, Size};

/// Follows somewhat the advice of Knuth on checking for floating-point
/// equality. The closeness relationship is:
///
/// `close(x,y,n) ≡ |x − y| ≤ ε|x| ∧ |x − y| ≤ ε|y|`
///
/// where `ε` is `n` times the machine accuracy; `n` equals 42 if not given.
pub fn close(x: Real, y: Real) -> bool {
    // deals with +infinity and -infinity representations etc
    if x == y {
        return true;
    }
    let diff = (x - y).abs();
    let tolerance = 42.0 * f64::EPSILON;

    if x == 0.0 || y == 0.0 {
        return diff < (tolerance * tolerance);
    }
    diff <= tolerance * x.abs() && diff <= tolerance * y.abs()
}

pub fn close_n(x: Real, y: Real, n: Size) -> bool {
    // deals with +infinity and -infinity representations etc
    if x == y {
        return true;
    }
    let diff = (x - y).abs();
    let tolerance = n as f64 * f64::EPSILON;

    if x == 0.0 || y == 0.0 {
        return diff < (tolerance * tolerance);
    }
    diff <= tolerance * x.abs() && diff <= tolerance * y.abs()
}

/// Follows somewhat the advice of Knuth on checking for floating-point
/// equality. The closeness relationship is:
///
/// `close_enough(x,y,n) ≡ |x − y| ≤ ε|x| v |x − y| ≤ ε|y|`
///
/// where `ε` is `n` times the machine accuracy; `n` equals 42 if not given.
pub fn close_enough(x: Real, y: Real) -> bool {
    // deals with +infinity and -infinity representations etc
    if x == y {
        return true;
    }
    let diff = (x - y).abs();
    let tolerance = 42.0 * f64::EPSILON;

    if x == 0.0 || y == 0.0 {
        return diff < (tolerance * tolerance);
    }
    diff <= tolerance * x.abs() || diff <= tolerance * y.abs()
}
