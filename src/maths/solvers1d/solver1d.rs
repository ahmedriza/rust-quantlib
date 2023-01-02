use crate::maths::solvers1d::solver1d::private::SolverData;
use crate::types::Real;

use crate::maths::comparison::close;

pub trait Solver1D: private::SolverDetail {
    /// This method returns the zero of the function `f`, determined with the given accuracy `ϵ`;
    /// depending on the particular solver, this might mean that the returned `x` is such that
    /// `|f(x)| < ϵ`, or that `|x − ξ| < ϵ` where `ξ` is the real zero.
    ///
    /// This method contains a bracketing routine to which an initial guess must be supplied as
    /// well as a step used to scan the range of the possible bracketing values.
    ///
    /// # Arguments
    ///
    /// * `f` - function `f(x)` that provides the value at input `x` (function to solve for root)
    /// * `derivative` - derivative of function `f(x)` that provides derivative at input `x`
    /// * `guess` - initial guess
    /// * `step` - step used to increase or decrease the guess at each iteration
    fn solve<F, G>(&self, f: F, derivative: G, accuracy: Real, guess: Real, step: Real) -> Real
    where
        F: Fn(Real) -> Real,
        G: Fn(Real) -> Real,
    {
        assert!(accuracy > 0.0, "accurancy ({}) must be positive", accuracy);
        // check whether we really want to use epsilon
        let accuracy = accuracy.max(f64::EPSILON);

        let growth_factor = 1.6;
        let mut flip_flop = -1;

        let mut sd = SolverData::default();

        sd.root = guess;
        sd.fx_max = f(sd.root);

        if close(sd.fx_max, 0.0) {
            return sd.root;
        } else if sd.fx_max > 0.0 {
            sd.xmin = self.enforce_bounds(sd.root - step);
            sd.fx_min = f(sd.xmin);
            sd.xmax = sd.root;
        } else {
            sd.xmin = sd.root;
            sd.fx_min = sd.fx_max;
            sd.xmax = self.enforce_bounds(sd.root + step);
            sd.fx_max = f(sd.xmax);
        }

        sd.evaluation_number = 2;
        while sd.evaluation_number < self.max_evaluations() {
            if sd.fx_min * sd.fx_max <= 0.0 {
                if close(sd.fx_min, 0.0) {
                    return sd.xmin;
                }
                if close(sd.fx_max, 0.0) {
                    return sd.xmax;
                }
                sd.root = (sd.xmax + sd.xmin) / 2.0;
                return self.solve_impl(f, derivative, accuracy, &mut sd);
            }
            if sd.fx_min.abs() < sd.fx_max.abs() {
                sd.xmin = self.enforce_bounds(sd.xmin + growth_factor * (sd.xmin - sd.xmax));
                sd.fx_min = f(sd.xmin);
            } else if sd.fx_min.abs() > sd.fx_max.abs() {
                sd.xmax = self.enforce_bounds(sd.xmax + growth_factor * (sd.xmax - sd.xmin));
                sd.fx_max = f(sd.xmax);
            } else if flip_flop == -1 {
                sd.xmin = self.enforce_bounds(sd.xmin + growth_factor * (sd.xmin - sd.xmax));
                sd.fx_min = f(sd.xmin);
                sd.evaluation_number += 1;
                flip_flop = 1;
            } else if flip_flop == 1 {
                sd.xmax = self.enforce_bounds(sd.xmax + growth_factor * (sd.xmax - sd.xmin));
                sd.fx_max = f(sd.xmax);
                flip_flop = -1;
            }
            sd.evaluation_number += 1;
        }

        panic!(
            "Unable to bracket root in {} function evaluations, (last bracket attempt: \
                f[{}, {}] -> [{}, {}])",
            self.max_evaluations(),
            sd.xmin,
            sd.xmax,
            sd.fx_min,
            sd.fx_max
        );
    }

    /// This method returns the zero of the function `f`, determined with the given accuracy `ϵ`;
    /// depending on the particular solver, this might mean that the returned `x` is such
    /// that `|f(x)| < ϵ`, or that `|x − ξ| < ϵ` where `ξ` is the real zero.
    ///
    /// An initial guess must be supplied, as well as two values `xmin` and `xmax` which must
    /// bracket the zero (i.e., either `f(xmin) ≤ 0 ≤ f(xmax)`, or `f(xmax) ≤ 0 ≤ f(xmin)`
    /// must be true).
    ///
    /// # Arguments
    ///
    /// * `f` - function that we need to solve
    /// * `accuracy` - required accuracy
    /// * `guess` - initial guess
    /// * `xmin` - minimum value of `x` for bracketing
    /// * `xmax` - maximum value of `x` for bracketing
    fn solve_bracketed<F, G>(
        &self,
        f: F,
        derivative: G,
        accuracy: Real,
        guess: Real,
        xmin: Real,
        xmax: Real,
    ) -> Real
    where
        F: Fn(Real) -> Real,
        G: Fn(Real) -> Real,
    {
        assert!(accuracy > 0.0, "accurancy ({}) must be positive", accuracy);
        // check whether we really want to use epsilon
        let accuracy = accuracy.max(f64::EPSILON);

        let mut sd = SolverData {
            xmin,
            xmax,
            ..Default::default()
        };

        assert!(
            sd.xmin < sd.xmax,
            "invalid range: xmin ({}) >= xmax ({})",
            sd.xmin,
            sd.xmax
        );
        assert!(
            !self.lower_bound_enforced() || sd.xmin >= self.lower_bound(),
            "xmin ({}) < enforced lower bound ({})",
            sd.xmin,
            self.lower_bound()
        );
        assert!(
            !self.upper_bound_enforced() || sd.xmax <= self.upper_bound(),
            "xmax ({}) > enforced upper bound ({})",
            sd.xmax,
            self.upper_bound()
        );

        sd.fx_min = f(sd.xmin);
        if close(sd.fx_min, 0.0) {
            return sd.xmin;
        }
        sd.fx_max = f(sd.xmax);
        if close(sd.fx_max, 0.0) {
            return sd.xmax;
        }

        sd.evaluation_number = 2;

        assert!(
            sd.fx_min * sd.fx_max < 0.0,
            "root not bracketed: f[{}, {}] -> [{}, {}], evaluations: {}",
            sd.xmin,
            sd.xmax,
            sd.fx_min,
            sd.fx_max,
            sd.evaluation_number
        );
        assert!(guess > sd.xmin, "guess ({}) < xmin ({})", guess, sd.xmin);
        assert!(guess < sd.xmax, "guess ({}) > xmax ({})", guess, sd.xmax);

        sd.root = guess;
        self.solve_impl(f, derivative, accuracy, &mut sd)
    }
}

// -------------------------------------------------------------------------------------------------

pub(crate) mod private {
    const MAX_FUNCTION_EVALUATIONS: Size = 100;
    use crate::types::{Real, Size};

    #[derive(Clone, Copy, Default)]
    pub struct SolverData {
        pub root: Real,
        pub xmin: Real,
        pub xmax: Real,
        pub fx_min: Real,
        pub fx_max: Real,
        pub evaluation_number: Size,
    }

    pub trait SolverDetail {
        fn solve_impl<F: Fn(Real) -> Real, G: Fn(Real) -> Real>(
            &self,
            f: F,
            derivative: G,
            accuracy: Real,
            solver_data: &mut SolverData,
        ) -> Real;

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
