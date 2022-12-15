use crate::types::Real;

use super::solver1d::{Solver1D, private};

/// Safe (bracketed) Newton 1-D solver with finite difference derivatives
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
            solver_data: &mut private::SolverData,
        ) -> Real {
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
