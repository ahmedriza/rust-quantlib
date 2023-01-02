#![cfg(test)]
use super::solver1d::Solver1D;
use crate::types::Real;

pub(crate) fn test_solver<S>(solver: &S, name: &str)
where
    S: Solver1D,
{
    let f1 = |x| x * x - 1.0;
    let expected = 1.0;
    let xmin = 0.0;
    let xmax = 2.0;

    // guess on the left side of the root, increasing function
    test_not_bracketed(solver, name, &f1, 0.5, expected);
    test_bracketed(solver, name, &f1, 0.5, xmin, xmax, expected);

    let f2 = |x| 1.0 - x * x;
    // guess on the left side of the root, decreasing function
    test_not_bracketed(solver, name, &f2, 1.5, expected);
    test_bracketed(solver, name, &f2, 1.5, xmin, xmax, expected);

    // guess on the right side of the root, decreasing function
    test_not_bracketed(solver, name, &f2, 1.5, expected);
    test_bracketed(solver, name, &f2, 1.5, xmin, xmax, expected);

    let f3 = |x: Real| (x - 1.0).atan();
    // situation where bisection is used in the finite difference
    // newton solver as the first step and where the initial
    // guess is equal to the next estimate (which causes an infinite
    // derivative if we do not handle this case with special care)
    test_not_bracketed(solver, name, &f3, 1.00001, expected);

    // This test is based on the example in <https://en.wikipedia.org/wiki/Brent%27s_method>
    let f4 = |x: Real| (x + 3.0) * (x - 1.0) * (x - 1.0);
    test_bracketed(solver, name, &f4, 0.5, -4.0, 4.0 / 3.0, -3.0);

    // This test is based on a case given in the following paper:
    // Implementation of Brent-Dekker and A Better Root Finding Method and Brent-Dekker
    // Method's Parallelization, Vakkalagadda Satya Sai Prakash
    // <https://tinyurl.com/y3uc5rjn>
    let f5 = |x: Real| (x.exp() * x.cos()) - (x * x.sin());
    test_not_bracketed(solver, name, &f5, 1.0, 1.22539378412362);
}

pub(crate) fn test_not_bracketed<S, F>(solver: &S, name: &str, f: &F, guess: Real, expected: Real)
where
    S: Solver1D,
    F: Fn(Real) -> Real,
{
    let accuracies = vec![1.0e-4, 1.0e-6, 1.0e-8];
    for accuracy in accuracies {
        let root = solver.solve(&f, |_| 0.0, accuracy, guess, 0.1);
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

pub(crate) fn test_bracketed<S, F>(
    solver: &S,
    name: &str,
    f: &F,
    guess: Real,
    xmin: Real,
    xmax: Real,
    expected: Real,
) where
    S: Solver1D,
    F: Fn(Real) -> Real,
{
    let accuracies = vec![1.0e-4, 1.0e-6, 1.0e-8];
    for accuracy in accuracies {
        // guess on the left side of the root, increasing function
        let root = solver.solve_bracketed(&f, |_y| 0.0, accuracy, guess, xmin, xmax);
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
