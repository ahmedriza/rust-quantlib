use crate::{maths::comparison::close_n, types::Real};

use super::solver1d::{private, Solver1D};

/// Safe (bracketed) Newton 1-D solver with finite difference derivatives
#[derive(Default)]
pub struct FiniteDifferenceNewtonSafe {
    lower_bound: Real,
    upper_bound: Real,
    lower_bound_enforced: bool,
    upper_bound_enforced: bool,
}

impl FiniteDifferenceNewtonSafe {
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

impl Solver1D for FiniteDifferenceNewtonSafe {}

impl private::SolverDetail for FiniteDifferenceNewtonSafe {
    fn solve_impl<F: Fn(Real) -> Real>(
        &self,
        f: F,
        accuracy: Real,
        sd: &mut private::SolverData,
    ) -> Real {
        // Orient the search so that f(xl) < 0
        let mut xh;
        let mut xl;
        if sd.fx_min < 0.0 {
            xl = sd.xmin;
            xh = sd.xmax;
        } else {
            xh = sd.xmin;
            xl = sd.xmax;
        }

        let mut f_root = f(sd.root);
        sd.evaluation_number += 1;
        // first order finite difference derivative
        let mut df_root = if (sd.xmax - sd.root) < (sd.root - sd.xmin) {
            (sd.fx_max - f_root) / (sd.xmax - sd.root)
        } else {
            (sd.fx_min - f_root) / (sd.xmin - sd.root)
        };

        // xmax - xmin > 0 is verified previously
        let mut dx = sd.xmax - sd.xmin;
        while sd.evaluation_number <= self.max_evaluations() {
            let mut f_root_old = f_root;
            let mut root_old = sd.root;
            let dx_old = dx;
            // Bisect if (out of range || not decreasing fast enough)
            if ((((sd.root - xh) * df_root - f_root) * ((sd.root - xl) * df_root - f_root)) > 0.0)
                || ((2.0 * f_root).abs() > (dx_old * df_root).abs())
            {
                dx = (xh - xl) / 2.0;
                sd.root = xl + dx;
                // if the root estimate just computed is close to the previous one, we should
                // calculate dfroot at root and xh rather than root and rootold
                // (xl instead of xh would be just as good)
                if close_n(sd.root, root_old, 2500) {
                    root_old = xh;
                    f_root_old = f(xh);
                }
            } else {
                // Newton
                dx = f_root / df_root;
                sd.root -= dx;
            }
            // convergence criterion
            if dx.abs() < accuracy {
                return sd.root;
            }

            f_root = f(sd.root);
            sd.evaluation_number += 1;
            df_root = (f_root_old - f_root) / (root_old - sd.root);

            if f_root < 0.0 {
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

    use super::FiniteDifferenceNewtonSafe;

    #[test]
    fn test_fd_newton_safe_one() {
        let solver = FiniteDifferenceNewtonSafe::default();
        let name = "FiniteDifferenceNewtonSafe";

        test_solver(&solver, name);
    }
}
