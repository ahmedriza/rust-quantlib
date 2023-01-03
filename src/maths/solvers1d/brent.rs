use crate::{maths::comparison::close, types::Real};

use super::solver1d::{
    private::{self, SolverData},
    Solver1D,
};

/// Brent 1-D solver
#[derive(Default)]
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
    fn solve_impl<F, G>(&self, f: F, _derivative: G, accuracy: Real, sd: &mut SolverData) -> Real
    where
        F: Fn(Real) -> Real,
        G: Fn(Real) -> Real,
    {
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
                f(sd.root);
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

#[cfg(test)]
mod test {
    use crate::maths::solvers1d::solver_test_util::test_solver;

    use super::Brent;

    #[test]
    fn test_brent() {
        let solver = Brent::default();
        let name = "Brent";

        test_solver(&solver, name);
    }
}
