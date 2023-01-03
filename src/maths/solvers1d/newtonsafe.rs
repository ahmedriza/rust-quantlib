use crate::types::Real;

use super::solver1d::{private, Solver1D};

/// Newton 1-D solver.
#[derive(Default)]
pub struct NewtonSafe {
    lower_bound: Real,
    upper_bound: Real,
    lower_bound_enforced: bool,
    upper_bound_enforced: bool,
}

impl NewtonSafe {
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

impl Solver1D for NewtonSafe {}

impl private::SolverDetail for NewtonSafe {
    fn solve_impl<F, G>(
        &self,
        f: F,
        derivative: G,
        accuracy: Real,
        sd: &mut private::SolverData,
    ) -> Real
    where
        F: Fn(Real) -> Real,
        G: Fn(Real) -> Real,
    {
        // The implementation of the algorithm was inspired by
        // Press, Teukolsky, Vetterling, and Flannery,
        // "Numerical Recipes in C", 2nd edition, Cambridge University Press.

        let mut froot: Real;
        let mut dfroot: Real;
        let mut dx: Real;
        let mut dx_old: Real;
        let mut xh: Real;
        let mut xl: Real;

        // Orient the search so that f(xl) < 0
        if sd.fx_min < 0.0 {
            xl = sd.xmin;
            xh = sd.xmax;
        } else {
            xh = sd.xmin;
            xl = sd.xmax;
        }
        // the "stepsize before last"
        dx_old = sd.xmax - sd.xmin;

        // it was dxold=std::fabs(xMax_-xMin_); in Numerical Recipes
        // here (xMax_-xMin_ > 0) is verified in the constructor

        // and the last step
        dx = dx_old;

        froot = f(sd.root);
        dfroot = derivative(sd.root);

        sd.evaluation_number += 1;
        while sd.evaluation_number <= self.max_evaluations() {
            // Bisect if (out of range || not decreasing fast enough)
            if (((sd.root - xh) * dfroot - froot) * ((sd.root - xl) * dfroot - froot) > 0.0)
                || ((2.0 * froot).abs() > (dx_old * dfroot).abs())
            {
                dx_old = dx;
                dx = (xh - xl) / 2.0;
                sd.root = xl + dx;
            } else {
                dx_old = dx;
                dx = froot / dfroot;
                sd.root -= dx;
            }
            // Convergence criterion
            if dx.abs() < accuracy {
                f(sd.root);
                sd.evaluation_number += 1;
                return sd.root;
            }
            froot = f(sd.root);
            dfroot = derivative(sd.root);
            sd.evaluation_number += 1;
            if froot < 0.0 {
                xl = sd.root;
            } else {
                xh = sd.root;
            }
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

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::maths::solvers1d::solver_test_util::test_solver;

    use super::NewtonSafe;

    #[test]
    fn test_newton_safe() {
        let solver = NewtonSafe::default();
        let name = "NewtonSafe";

        test_solver(&solver, name);
    }
}
