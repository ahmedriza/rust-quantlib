use crate::types::Real;

use super::solver1d::{private, Solver1D};

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
    #[allow(unused)]
    fn solve_impl<F: Fn(Real) -> Real>(&self, f: F, accuracy: Real) -> Real {
        todo!()
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

#[allow(unused)]
#[cfg(test)]
mod test {
    use crate::maths::solvers1d::solver1d::Solver1D;

    use super::Brent;

    #[test]
    fn test_brent() {
        let solver1d = Brent::new(-10.0, 10.0, false, false);
        // TODO
        // solver1d.solve_with_step(f, accuracy, guess, step);
    }
}
