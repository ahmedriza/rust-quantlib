#[allow(unused)]
use rust_quantlib::maths::solvers1d::{brent::Brent, solver1d::Solver1D};

#[allow(unused)]
#[test]
fn test_brent() {
    let solver1d = Brent::new(-10.0, 10.0, false, false);
    // solver1d.solve_with_step(f, accuracy, guess, step);
    // solver1d.solve_with_xmin_xmax(f, accuracy, guess, xmin, xmax);
}
