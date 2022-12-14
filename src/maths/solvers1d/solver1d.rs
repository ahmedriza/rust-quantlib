use crate::types::Real;

use crate::maths::comparison::close;

pub trait Solver1D: private::SolverDetail {
    /// This method returns the zero of the function `f`, determined with the given accuracy `ϵ`;
    /// depending on the particular solver, this might mean that the returned `x` is such that
    /// `|f(x)| < ϵ`, or that `|x − ξ| < ϵ` where `ξ` is the real zero.
    ///
    /// This method contains a bracketing routine to which an initial guess must be supplied as
    /// well as a step used to scan the range of the possible bracketing values.
    fn solve_with_step<F>(&self, f: F, accuracy: Real, guess: Real, step: Real) -> Real
    where
        F: Fn(Real) -> Real,
    {
        assert!(accuracy > 0.0, "accurancy ({}) must be positive", accuracy);
        // check whether we really want to use epsilon
        let accuracy = accuracy.max(f64::EPSILON);

        let growth_factor = 1.6;
        let mut flip_flop = -1;

        let mut root = guess;
        let mut fx_max = f(root);
        let mut fx_min;
        let mut xmin;
        let mut xmax;

        if close(fx_max, 0.0) {
            return root;
        } else if fx_max > 0.0 {
            xmin = self.enforce_bounds(root - step);
            fx_min = f(xmin);
            xmax = root;
        } else {
            xmin = root;
            fx_min = fx_max;
            xmax = self.enforce_bounds(root + step);
            fx_max = f(xmax);
        }

        let mut evaluation_number = 2;
        while evaluation_number < self.max_evaluations() {
            if fx_min * fx_max <= 0.0 {
                if close(fx_min, 0.0) {
                    return xmin;
                }
                if close(fx_max, 0.0) {
                    return xmax;
                }
                root = (xmax + xmin) / 2.0;
                return self.solve_impl(f, accuracy);
            }
            if fx_min.abs() < fx_max.abs() {
                xmin = self.enforce_bounds(xmin + growth_factor * (xmin - xmax));
                fx_min = f(xmin);
            } else if fx_min.abs() > fx_max.abs() {
                xmax = self.enforce_bounds(xmax + growth_factor * (xmax - xmin));
                fx_max = f(xmax);
            } else if flip_flop == -1 {
                xmin = self.enforce_bounds(xmin + growth_factor * (xmin - xmax));
                fx_min = f(xmin);
                evaluation_number += 1;
                flip_flop = 1;
            } else if flip_flop == 1 {
                xmax = self.enforce_bounds(xmax + growth_factor * (xmax - xmin));
                fx_max = f(xmax);
                flip_flop = -1;
            }
            evaluation_number += 1;
        }

        panic!(
            "Unable to bracket root in {} function evaluations, (last bracket attempt: \
                f[{}, {}] -> [{}, {}])",
            self.max_evaluations(),
            xmin,
            xmax,
            fx_min,
            fx_max
        );
    }

    /// This method returns the zero of the function `f`, determined with the given accuracy `ϵ`;
    /// depending on the particular solver, this might mean that the returned `x` is such
    /// that `|f(x)| < ϵ`, or that `|x − ξ| < ϵ` where `ξ` is the real zero.
    ///
    /// An initial guess must be supplied, as well as two values `xmin` and `xmax` which must
    /// bracket the zero (i.e., either `f(xmin) ≤ 0 ≤ f(xmax)`, or `f(xmax) ≤ 0 ≤ f(xmin)`
    /// must be true).
    fn solve_with_xmin_xmax<F>(
        &self,
        f: F,
        accuracy: Real,
        guess: Real,
        xmin: Real,
        xmax: Real,
    ) -> Real
    where
        F: Fn(Real) -> Real,
    {
        assert!(accuracy > 0.0, "accurancy ({}) must be positive", accuracy);
        // check whether we really want to use epsilon
        let accuracy = accuracy.max(f64::EPSILON);

        let xmin = xmin;
        let xmax = xmax;

        assert!(
            xmin < xmax,
            "invalid range: xmin ({}) >= xmax ({})",
            xmin,
            xmax
        );
        assert!(
            self.lower_bound_enforced() || xmin >= self.lower_bound(),
            "xmin ({}) < enforced lower bound ({})",
            xmin,
            self.lower_bound()
        );
        assert!(
            self.upper_bound_enforced() || xmax <= self.upper_bound(),
            "xmax ({}) > enforced upper bound ({})",
            xmax,
            self.upper_bound()
        );

        let fx_min = f(xmin);
        if close(fx_min, 0.0) {
            return xmin;
        }
        let fx_max = f(xmax);
        if close(fx_max, 0.0) {
            return xmax;
        }

        assert!(
            fx_min * fx_max < 0.0,
            "root not bracketed: f[{}, {}] -> [{}, {}]",
            xmin,
            xmax,
            fx_min,
            fx_max
        );
        assert!(guess > xmin, "guess ({}) < xmin ({})", guess, xmin);
        assert!(guess < xmax, "guess ({}) > xmax ({})", guess, xmax);

        // TODO
        // let root = guess;
        self.solve_impl(f, accuracy)
    }
}

// -------------------------------------------------------------------------------------------------

pub(crate) mod private {
    const MAX_FUNCTION_EVALUATIONS: Size = 100;
    use crate::types::{Real, Size};

    pub trait SolverDetail {
        fn solve_impl<F: Fn(Real) -> Real>(&self, f: F, accuracy: Real) -> Real;

        fn max_evaluations(&self) -> Size {
            MAX_FUNCTION_EVALUATIONS
        }

        fn lower_bound(&self) -> Real;

        fn upper_bound(&self) -> Real;

        fn lower_bound_enforced(&self) -> bool;

        fn upper_bound_enforced(&self) -> bool;

        fn enforce_bounds(&self, x: Real) -> Real {
            if self.lower_bound_enforced() && x < self.lower_bound() {
                return self.lower_bound();
            }
            if self.upper_bound_enforced() && x > self.upper_bound() {
                return self.upper_bound();
            }
            x
        }
    }
}
