use crate::{maths::comparison::close, types::Real};

use super::solver1d::{
    private::{self, SolverData},
    Solver1D,
};

/// Brent 1-D solver
pub struct Brent {
    lower_bound: Real,
    upper_bound: Real,
    lower_bound_enforced: bool,
    upper_bound_enforced: bool,
}

impl Brent {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        lower_bound: Real,
        upper_bound: Real,
        lower_bound_enforced: bool,
        upper_bound_enforced: bool,
    ) -> impl Solver1D {
        Self {
            lower_bound,
            upper_bound,
            lower_bound_enforced,
            upper_bound_enforced,
        }
    }
}

impl Solver1D for Brent {}

impl private::SolverDetail for Brent {
    /// The implementation of the algorithm was inspired by Press, Teukolsky, Vetterling, and
    /// Flannery, "Numerical Recipes in C", 2nd edition, Cambridge University Press
    fn solve<F: Fn(Real) -> Real>(&self, f: F, accuracy: Real, sd: &mut SolverData) -> Real {
        // we want to start with root (which equals the guess) on one side of the bracket and both
        // xmin and xmax on the other.
        let mut f_root = f(sd.root);
        sd.evaluation_number += 1;

        if f_root * sd.fx_min < 0.0 {
            sd.xmax = sd.xmin;
            sd.fx_max = sd.fx_min;
        } else {
            sd.xmin = sd.xmax;
            sd.fx_min = sd.fx_max;
        }

        let mut d = sd.root - sd.xmax;
        let mut e = d;

        let mut s;
        let mut p;
        let mut q;
        let mut r;
        let mut min1;
        let mut min2;

        while sd.evaluation_number < self.max_evaluations() {
            if (f_root > 0.0 && sd.fx_max > 0.0) || (f_root < 0.0 && sd.fx_max < 0.0) {
                // Rename xmin, root, xmax and adjust bounds
                sd.xmax = sd.xmin;
                sd.fx_max = sd.fx_min;
                e = sd.root - sd.xmin;
                d = sd.root - sd.xmin;
            }
            if sd.fx_max.abs() < f_root.abs() {
                sd.xmin = sd.root;
                sd.root = sd.xmax;
                sd.xmax = sd.xmin;
                sd.fx_min = f_root;
                f_root = sd.fx_max;
                sd.fx_max = sd.fx_min;
            }
            // Convergence check
            let xacc1 = 2.0 * f64::EPSILON * sd.root.abs() + 0.5 * accuracy;
            let xmid = (sd.xmax - sd.root) / 2.0;
            if (xmid.abs() <= xacc1) || (close(f_root, 0.0)) {
                f(sd.root); // TODO check this
                sd.evaluation_number += 1;
                return sd.root;
            }

            if (e.abs() >= xacc1) && (sd.fx_min.abs() > f_root.abs()) {
                // Attempt inverse quadratic interpolation
                s = f_root / sd.fx_min;
                if close(sd.xmin, sd.xmax) {
                    p = 2.0 * xmid * s;
                    q = 1.0 - s;
                } else {
                    q = sd.fx_min / sd.fx_max;
                    r = f_root / sd.fx_max;
                    p = s * (2.0 * xmid * q * (q - r) - (sd.root - sd.xmin) * (r - 1.0));
                    q = (q - 1.0) * (r - 1.0) * (s - 1.0);
                }
                if p > 0.0 {
                    // Check whether in bounds
                    q = -q;
                }
                p = p.abs();
                min1 = 3.0 * xmid * q - (xacc1 * q).abs();
                min2 = (e * q).abs();
                if 2.0 * p < min1.min(min2) {
                    // TODO check this
                    e = d; // accept interpolation
                    d = p / q;
                } else {
                    d = xmid; // interpolation failed, use bisection
                    e = d;
                }
            } else {
                // Bounds decreasing too slowly, use bisection
                d = xmid;
                e = d;
            }
            sd.xmin = sd.root;
            sd.fx_min = f_root;
            if d.abs() > xacc1 {
                sd.root += d;
            } else {
                sd.root += sign(xacc1, xmid);
            }
            f_root = f(sd.root);
            sd.evaluation_number += 1;
        }

        panic!(
            "Maximum number of function evaluations ({}) exceeded",
            self.max_evaluations()
        );
    }

    fn lower_bound(&self) -> Real {
        self.lower_bound
    }

    fn upper_bound(&self) -> Real {
        self.upper_bound
    }

    fn lower_bound_enforced(&self) -> bool {
        self.lower_bound_enforced
    }

    fn upper_bound_enforced(&self) -> bool {
        self.upper_bound_enforced
    }
}

fn sign(a: Real, b: Real) -> Real {
    if b >= 0.0 {
        a.abs()
    } else {
        -a.abs()
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused)]
#[cfg(test)]
mod test {
    use crate::{maths::solvers1d::solver1d::Solver1D, types::Real};

    use super::Brent;

    #[test]
    fn test_brent_one() {
        let solver = Brent::new(0.0, 0.0, false, false);
        let name = "Brent";

        let f1 = |x| x * x - 1.0;
        // guess on the left side of the root, increasing function
        test_not_bracketed(&solver, name, f1, 0.5);
        test_bracketed(&solver, name, f1, 0.5);

        // guess on the right side of the root, increasing function
        test_not_bracketed(&solver, name, f1, 1.5);
        test_bracketed(&solver, name, f1, 1.5);

        let f2 = |x| 1.0 - x * x;
        // guess on the left side of the root, decreasing function
        test_not_bracketed(&solver, name, f2, 0.5);
        test_bracketed(&solver, name, f2, 0.5);

        // guess on the right side of the root, decreasing function
        test_not_bracketed(&solver, name, f2, 1.5);
        test_bracketed(&solver, name, f2, 1.5);

        let f3 = |x: Real| (x - 1.0).atan();
        // situation where bisection is used in the finite difference
        // newton solver as the first step and where the initial
        // guess is equal to the next estimate (which causes an infinite
        // derivative if we do not handle this case with special care)
        test_not_bracketed(&solver, name, f3, 1.00001);
    }

    // This test is based on the example in <https://en.wikipedia.org/wiki/Brent%27s_method>
    #[test]
    fn test_brent_two() {
        let solver = Brent::new(0.0, 0.0, false, false);
        let name = "Brent";

        let f = |x: Real| (x + 3.0) * (x - 1.0) * (x - 1.0);
        // [-4, 4/3]
        let accuracy = 1.0e-8;
        let expected = -3.0;

        let root = solver.solve_with_xmin_xmax(&f, accuracy, 0.5, -4.0, 4.0 / 3.0);
        assert!(
            (root - expected).abs() <= accuracy,
            "{} solver (bracketed), expected: {}, calculated: {}, accuracy: {}",
            name,
            expected,
            root,
            accuracy
        );
    }

    // This test is based on a case given in the following paper:
    // Implementation of Brent-Dekker and A Better Root Finding Method and Brent-Dekker
    // Method's Parallelization, Vakkalagadda Satya Sai Prakash
    // <https://tinyurl.com/y3uc5rjn>
    #[test]
    fn test_brent_three() {
        let solver = Brent::new(0.0, 0.0, false, false);
        let name = "Brent";

        let f = |x: Real| {
            (x.exp() * x.cos()) - (x * x.sin())
        };

        let expected = 1.225393;
        let accuracy = 1.0e-6;

        let root = solver.solve_with_step(&f, accuracy, 1.0, 0.1);
        assert!(
            (root - expected).abs() <= accuracy,
            "{} solver (bracketed), expected: {}, calculated: {}, accuracy: {}",
            name,
            expected,
            root,
            accuracy
        );
    }

    fn test_not_bracketed<S, F>(solver: &S, name: &str, f: F, guess: Real)
    where
        S: Solver1D,
        F: Fn(Real) -> Real,
    {
        let accuracies = vec![1.0e-4, 1.0e-6, 1.0e-8];
        let expected = 1.0;
        for accuracy in accuracies {
            let root = solver.solve_with_step(&f, accuracy, guess, 0.1);
            assert!(
                (root - expected).abs() <= accuracy,
                "{} solver (not bracketed), expected: {}, calculated: {}, accuracy: {}",
                name,
                expected,
                root,
                accuracy
            );
        }
    }

    fn test_bracketed<S, F>(solver: &S, name: &str, f: F, guess: Real)
    where
        S: Solver1D,
        F: Fn(Real) -> Real,
    {
        let accuracies = vec![1.0e-4, 1.0e-6, 1.0e-8];
        let expected = 1.0;
        for accuracy in accuracies {
            // guess on the left side of the root, increasing function
            let root = solver.solve_with_xmin_xmax(&f, accuracy, guess, 0.0, 2.0);
            assert!(
                (root - expected).abs() <= accuracy,
                "{} solver (bracketed), expected: {}, calculated: {}, accuracy: {}",
                name,
                expected,
                root,
                accuracy
            );
        }
    }
}
