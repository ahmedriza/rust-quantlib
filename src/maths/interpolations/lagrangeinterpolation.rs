use crate::types::{Real, Size};

use crate::maths::{
    array::Array,
    bounds::{lower_bound, upper_bound},
    comparison::{close, close_enough},
};

use super::interpolation::Interpolation;

/// Lagrange interpolation
///
/// References: J-P. Berrut and L.N. Trefethen,
///             Barycentric Lagrange interpolation,
///             SIAM Review, 46(3):501–517, 2004.
/// <https://people.maths.ox.ac.uk/trefethen/barycentric.pdf>
pub struct LagrangeInterpolation<'a> {
    pub x: &'a [Real],
    pub y: &'a [Real],
    pub lambda: Array,
}

impl<'a> LagrangeInterpolation<'a> {
    pub fn new(x: &'a [Real], y: &'a [Real]) -> Self {
        let mut result = Self {
            x,
            y,
            lambda: Array::new(vec![Real::default(); x.len()]),
        };
        result.update();
        result
    }

    pub fn _value(&self, x: Real) -> Real {
        let eps = 10.0 * f64::EPSILON * x.abs();
        let lb = lower_bound(self.x, x - eps);
        if lb != self.x.len() && self.x[lb] - x < eps {
            return self.y[lb];
        }
        let mut n = 0.0;
        let mut d = 0.0;
        for i in 0..self.x.len() {
            let alpha = self.lambda[i] / (x - self.x[i]);
            n += alpha * self.y[i];
            d += alpha;
        }
        n / d
    }
}

impl<'a> Interpolation for LagrangeInterpolation<'a> {
    fn primitive_with_extrapolation(&self, _x: Real, _allow_extrapolation: bool) -> Real {
        unimplemented!("LagrangeInterpolation primitive is not implemented");
    }

    fn derivative_with_extrapolation(&self, x: Real, allow_extrapolation: bool) -> Real {
        self.check_range(x, allow_extrapolation);
        let mut n = 0.0;
        let mut d = 0.0;
        let mut nd = 0.0;
        let mut dd = 0.0;
        for i in 0..self.x.len() {
            let x_i = self.x[i];
            if close_enough(x, x_i) {
                let mut p = 0.0;
                for j in 0..self.x.len() {
                    if i != j {
                        p += self.lambda[j] / (x - self.x[j]) * (self.y[j] - self.y[i]);
                    }
                }
                return p / self.lambda[i];
            }

            let alpha = self.lambda[i] / (x - x_i);
            let alphad = -alpha / (x - x_i);
            n += alpha * self.y[i];
            d += alpha;
            nd += alphad * self.y[i];
            dd += alphad;
        }

        (nd * d - n * dd) / (d * d)
    }

    fn second_derivative_with_extrapolation(&self, _x: Real, _allow_extrapolation: bool) -> Real {
        unimplemented!("LagrangeInterpolation second derivative is not implemented")
    }

    // TODO remove code duplication
    fn xmin(&self) -> Real {
        self.x[0]
    }

    // TODO remove code duplication
    fn xmax(&self) -> Real {
        self.x[self.x.len() - 1]
    }

    fn value_with_extrapolation(&self, x: Real, allow_extrapolation: bool) -> Real {
        self.check_range(x, allow_extrapolation);
        self._value(x)
    }

    // TODO remove code duplication
    fn is_in_range(&self, x: Real) -> bool {
        let x1 = self.xmin();
        let x2 = self.xmax();
        (x >= x1 && x <= x2) || close(x, x1) || close(x, x2)
    }

    // TODO remove code duplication
    fn locate(&self, x: Real) -> Size {
        if x < self.x[0] {
            0
        } else if x > self.x[self.x.len() - 1] {
            self.x.len() - 2
        } else {
            upper_bound(self.x, x) - 1
        }
    }

    fn update(&mut self) {
        let cm1 = 4.0 / (self.x[self.x.len() - 1] - self.x[0]);
        for i in 0..self.x.len() {
            self.lambda[i] = 1.0;

            let x_i = self.x[i];
            for j in 0..self.x.len() {
                if i != j {
                    self.lambda[i] *= cm1 * (x_i - self.x[j]);
                }
            }
            self.lambda[i] = 1.0 / self.lambda[i];
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use crate::types::Real;

    use crate::maths::interpolations::{
        interpolation::Interpolation, lagrangeinterpolation::LagrangeInterpolation,
    };

    #[allow(clippy::excessive_precision)]
    #[test]
    fn test_lagrange_interpolation() {
        let xs = vec![-1.0, -0.5, -0.25, 0.1, 0.4, 0.75, 0.96];

        let mut y = vec![Real::default(); xs.len()];
        xs.iter().enumerate().for_each(|(i, x)| {
            y[i] = lagrange_test_fn(*x);
        });

        let interp = LagrangeInterpolation::new(&xs, &y);

        let references: [f64; 79] = [
            -0.5000000000000000,
            -0.5392414024347419,
            -0.5591485962711904,
            -0.5629199661387594,
            -0.5534414777017116,
            -0.5333043347921566,
            -0.5048221831582063,
            -0.4700478608272949,
            -0.4307896950846587,
            -0.3886273460669714,
            -0.3449271969711449,
            -0.3008572908782903,
            -0.2574018141928359,
            -0.2153751266968088,
            -0.1754353382192734,
            -0.1380974319209344,
            -0.1037459341938971,
            -0.0726471311765894,
            -0.0449608318838433,
            -0.0207516779521373,
            0.0000000000000000,
            0.0173877793964286,
            0.0315691961126723,
            0.0427562482700356,
            0.0512063534145595,
            0.0572137590808174,
            0.0611014067405497,
            0.0632132491361394,
            0.0639070209989264,
            0.0635474631523613,
            0.0625000000000000,
            0.0611248703983366,
            0.0597717119144768,
            0.0587745984686508,
            0.0584475313615655,
            0.0590803836865967,
            0.0609352981268212,
            0.0642435381368876,
            0.0692027925097279,
            0.0759749333281079,
            0.0846842273010179,
            0.0954160004849021,
            0.1082157563897290,
            0.1230887474699003,
            0.1400000000000001,
            0.1588747923353829,
            0.1795995865576031,
            0.2020234135046815,
            0.2259597111862140,
            0.2511886165833182,
            0.2774597108334206,
            0.3044952177998833,
            0.3319936560264689,
            0.3596339440766487,
            0.3870799592577457,
            0.4139855497299214,
            0.4400000000000001,
            0.4647739498001331,
            0.4879657663513030,
            0.5092483700116673,
            0.5283165133097421,
            0.5448945133624253,
            0.5587444376778583,
            0.5696747433431296,
            0.5775493695968156,
            0.5822972837863635,
            0.5839224807103117,
            0.5825144353453510,
            0.5782590089582251,
            0.5714498086024714,
            0.5625000000000000,
            0.5519545738075141,
            0.5405030652677689,
            0.5289927272456703,
            0.5184421566492137,
            0.5100553742352614,
            0.5052363578001620,
            0.5056040287552059,
            0.5130076920869246,
        ];

        let tolerance = 50.0 * f64::EPSILON;
        for (i, _x) in references.iter().enumerate() {
            let xx = -1.0 + i as f64 * 0.025;
            let calculated = interp.value(xx);
            assert!(
                !f64::is_nan(calculated) || (references[i] - calculated).abs() <= tolerance,
                "failed to reproduce the Lagrange interpolation, \
                     x: {}, calculated: {}, expected: {}",
                xx,
                calculated,
                references[i]
            );
        }
    }

    #[test]
    fn test_lagrange_interpolation_at_support_point() {
        let n = 5;
        let mut x = vec![Real::default(); n];
        let mut y = vec![Real::default(); n];
        for i in 0..n {
            x[i] = i as Real / n as Real;
            y[i] = 1.0 / (1.0 - x[i]);
        }
        let interp = LagrangeInterpolation::new(&x, &y);
        let rel_tol = 5e-12;
        for x in x.iter().take(n - 1).skip(1) {
            loop {
                let mut z = x - 100.0 * f64::EPSILON;
                let expected = 1.0 / (1.0 - x);
                let calculated = interp.value(z);
                assert!(
                    !f64::is_nan(calculated) || (expected - calculated).abs() <= rel_tol,
                    "failed to reproduce the Lagrange interpolation, \
                     x: {}, calculated: {}, expected: {}",
                    z,
                    calculated,
                    expected
                );
                z += 2.0 * f64::EPSILON;
                let end = x * 100.0 * f64::EPSILON;
                if z > end {
                    break;
                }
            }
        }
    }

    #[test]
    fn test_lagrange_interpolation_derivative() {
        let x = vec![-1.0, -0.3, 0.1, 0.3, 0.9];
        let y = vec![2.0, 3.0, 6.0, 3.0, -1.0];
        let interp = LagrangeInterpolation::new(&x, &y);

        let eps = f64::EPSILON.sqrt();
        let mut x = -1.0;
        loop {
            let calculated = interp.derivative_with_extrapolation(x, true);
            let expected = (interp.value_with_extrapolation(x + eps, true)
                - interp.value_with_extrapolation(x - eps, true))
                / 2.0
                * eps;

            assert!(
                !f64::is_nan(calculated) || (expected - calculated).abs() <= 25.0 * eps,
                "failed to reproduce the Lagrange interpolation, \
                     x: {}, calculated: {}, expected: {}",
                x,
                calculated,
                expected
            );
            x += 0.01;
            if x > 0.9 {
                break;
            }
        }
    }

    #[test]
    fn test_lagrange_interpolation_on_chebyshev_points() {
        let n = 50;
        let mut x = vec![Real::default(); n + 1];
        let mut y = vec![Real::default(); n + 1];
        for i in 0..=n {
            // Chebyshev nodes
            x[i] = ((2.0 * i as Real + 1.0) * PI / (2.0 * n as f64 + 2.0)).cos();
            y[i] = x[i].exp() / x[i].cos();
        }
        let interp = LagrangeInterpolation::new(&x, &y);

        let tol = 1e-13;
        let tol_deriv = 1e-11;
        let mut x = -1.0;
        loop {
            let calculated = interp.value_with_extrapolation(x, true);
            let expected = x.exp() / x.cos();
            let diff = (expected - calculated).abs();
            assert!(
                !f64::is_nan(calculated) || diff <= tol,
                "failed to reproduce the Lagrange interpolation on Chebyshev nodes, \
                 x: {}, calculated: {}, expected: {}, difference: {}",
                x,
                calculated,
                expected,
                diff
            );

            let calculated_deriv = interp.derivative_with_extrapolation(x, true);
            let expected_deriv = x.exp() * (x.cos() + x.sin()) / (x.cos() * x.cos());
            let diff_deriv = (expected_deriv - calculated_deriv).abs();
            assert!(
                !f64::is_nan(calculated_deriv) || diff_deriv <= tol_deriv,
                "failed to reproduce the Lagrange interpolation derivative on Chebyshev nodes, \
                 x: {}, calculated: {}, expected: {}, difference: {}",
                x,
                calculated_deriv,
                expected_deriv,
                diff_deriv
            );

            if x > 1.0 {
                break;
            }
            x += 0.03;
        }
    }

    fn lagrange_test_fn(x: Real) -> Real {
        x.abs() + 0.5 * x - x * x
    }
}
