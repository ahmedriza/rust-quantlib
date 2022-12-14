use crate::types::{Real, Size};

use crate::maths::{bounds::upper_bound, comparison::close};

use super::interpolation::Interpolation;

/// Linear interpolation between discrete points
pub struct LinearInterpolation<'a> {
    pub x: &'a [Real],
    pub y: &'a [Real],
    pub primitive_const: Vec<Real>,
    pub s: Vec<Real>,
}

impl<'a> LinearInterpolation<'a> {
    pub fn new(x: &'a [Real], y: &'a [Real]) -> Self {
        let mut result = Self {
            x,
            y,
            primitive_const: vec![0.0; x.len()],
            s: vec![0.0; x.len()],
        };
        result.update();
        result
    }
}

impl<'a> Interpolation for LinearInterpolation<'a> {
    fn primitive_with_extrapolation(&self, x: Real, allow_extrapolation: bool) -> Real {
        self.check_range(x, allow_extrapolation);
        let i = self.locate(x);
        let dx = x - self.x[i];
        self.primitive_const[i] + dx * (self.y[i] + 0.5 * dx * self.s[i])
    }

    fn derivative_with_extrapolation(&self, x: Real, allow_extrapolation: bool) -> Real {
        self.check_range(x, allow_extrapolation);
        let i = self.locate(x);
        self.s[i]
    }

    fn second_derivative_with_extrapolation(&self, x: Real, allow_extrapolation: bool) -> Real {
        self.check_range(x, allow_extrapolation);
        Real::default()
    }

    fn xmin(&self) -> Real {
        self.x[0]
    }

    fn xmax(&self) -> Real {
        self.x[self.x.len() - 1]
    }

    fn value_with_extrapolation(&self, x: Real, allow_extrapolation: bool) -> Real {
        self.check_range(x, allow_extrapolation);
        let i = self.locate(x);
        self.y[i] + (x - self.x[i]) * self.s[i]
    }

    fn is_in_range(&self, x: Real) -> bool {
        let x1 = self.xmin();
        let x2 = self.xmax();
        (x >= x1 && x <= x2) || close(x, x1) || close(x, x2)
    }

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
        self.primitive_const[0] = 0.0;
        for i in 1..self.x.len() {
            let dx = self.x[i] - self.x[i - 1];
            self.s[i - 1] = (self.y[i] - self.y[i - 1]) / dx;
            self.primitive_const[i] =
                self.primitive_const[i - 1] + dx * (self.y[i - 1] + 0.5 * dx * self.s[i - 1])
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::maths::{
        interpolations::interpolation::Interpolation,
        rounding::{Rounding, RoundingType},
    };

    use super::LinearInterpolation;

    #[test]
    fn test_linear_interpolation() {
        let x = vec![0.0, 1.0, 3.0, 4.0];
        let y = vec![10.0, 20.0, 25.0, 40.0];
        let lin = LinearInterpolation::new(&x, &y);

        assert_eq!(lin.primitive_const, vec![0.0, 15.0, 60.0, 92.5]);
        assert_eq!(lin.s, vec![10.0, 2.5, 15.0, 0.0]);

        let x = 0.8;
        assert_eq!(lin.value(x), 18.0);

        let x = 3.5;
        assert_eq!(lin.value(x), 32.5);
        assert_eq!(lin.primitive(x), 74.375);

        let x = 2.0;
        assert_eq!(lin.primitive(x), 36.25);
    }

    #[test]
    fn test_linear_interpolation_two() {
        let x = vec![94.0, 205.0, 371.0];
        let y = vec![929.0, 902.0, 860.0];
        let lin = LinearInterpolation::new(&x, &y);
        let x = 251.0;
        // 890.3614457831326
        let rounding = RoundingType::closest(2, 5);
        assert_eq!(rounding.round(lin.value(x)), 890.36);
        // 142844.81325301205
        assert_eq!(rounding.round(lin.primitive(x)), 142844.81);
    }
}
