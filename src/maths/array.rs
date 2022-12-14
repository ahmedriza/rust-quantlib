use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

use crate::types::{Real, Size};

/// 1-D array used in linear algebra.
///
/// Implements the concept of vector as used in linear algebra.
/// As such, it is <b>not</b> meant to be used as a container - [Vec] should be used instead.
#[derive(Debug, PartialEq)]
pub struct Array {
    pub data: Vec<Real>,
}

// -------------------------------------------------------------------------------------------------

impl Index<Size> for Array {
    type Output = Real;

    fn index(&self, index: Size) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<Size> for Array {
    fn index_mut(&mut self, index: Size) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Neg for Array {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Array::new(self.data.iter().map(|x| -x).collect())
    }
}

impl Add for Array {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert!(
            self.size() == rhs.size(),
            "arrays with different sizes ({}, {}) cannot be added",
            self.size(),
            rhs.size()
        );

        Array::new(
            self.data
                .iter()
                .zip(rhs.data.iter())
                .map(|(x, y)| x + y)
                .collect(),
        )
    }
}

impl Add<Real> for Array {
    type Output = Self;

    fn add(self, rhs: Real) -> Self::Output {
        Array::new(self.data.iter().map(|x| x + rhs).collect())
    }
}

impl Sub for Array {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert!(
            self.size() == rhs.size(),
            "arrays with different sizes ({}, {}) cannot be subtracted",
            self.size(),
            rhs.size()
        );

        Array::new(
            self.data
                .iter()
                .zip(rhs.data.iter())
                .map(|(x, y)| x - y)
                .collect(),
        )
    }
}

impl Sub<Real> for Array {
    type Output = Self;

    fn sub(self, rhs: Real) -> Self::Output {
        Array::new(self.data.iter().map(|x| x - rhs).collect())
    }
}

impl Mul for Array {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(
            self.size() == rhs.size(),
            "arrays with different sizes ({}, {}) cannot be multiplied",
            self.size(),
            rhs.size()
        );

        Array::new(
            self.data
                .iter()
                .zip(rhs.data.iter())
                .map(|(x, y)| x * y)
                .collect(),
        )
    }
}

impl Mul<Real> for Array {
    type Output = Self;

    fn mul(self, rhs: Real) -> Self::Output {
        Array::new(self.data.iter().map(|x| x * rhs).collect())
    }
}

impl Div for Array {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert!(
            self.size() == rhs.size(),
            "arrays with different sizes ({}, {}) cannot be divided",
            self.size(),
            rhs.size()
        );

        Array::new(
            self.data
                .iter()
                .zip(rhs.data.iter())
                .map(|(x, y)| x / y)
                .collect(),
        )
    }
}

impl Div<Real> for Array {
    type Output = Self;

    fn div(self, rhs: Real) -> Self::Output {
        Array::new(self.data.iter().map(|x| x / rhs).collect())
    }
}

// -------------------------------------------------------------------------------------------------

impl Array {
    /// Construct a new [Array] from the given vector
    pub fn new(data: Vec<Real>) -> Self {
        Self { data }
    }

    pub fn size(&self) -> Size {
        self.data.len()
    }

    pub fn empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the dot product of `v1` and `v2`
    pub fn dot_product(&self, rhs: &Array) -> Real {
        assert!(
            self.size() == rhs.size(),
            "arrays with different sizes ({}, {}) cannot be multiplied",
            self.size(),
            rhs.size()
        );
        self.data
            .iter()
            .zip(rhs.data.iter())
            .map(|(x, y)| x * y)
            .sum()
    }

    /// L2 or Euclidean norm
    pub fn norm2(&self) -> Real {
        Array::dot_product(self, self).sqrt()
    }

    /// Return the absolute value of self
    pub fn abs(&self) -> Array {
        Array::new(self.data.clone().iter().map(|x| x.abs()).collect())
    }

    /// Return the square root of self
    pub fn sqrt(&self) -> Array {
        Array::new(self.data.clone().iter().map(|x| x.sqrt()).collect())
    }

    /// Return the natural log of self     
    pub fn log(&self) -> Array {
        let e = std::f64::consts::E;
        Array::new(self.data.clone().iter().map(|x| x.log(e)).collect())
    }

    /// Return e^(self)
    pub fn exp(&self) -> Array {
        Array::new(self.data.clone().iter().map(|x| x.exp()).collect())
    }

    /// Return self raised to the power `alpha`
    pub fn pow(&self, alpha: Real) -> Array {
        Array::new(self.data.clone().iter().map(|x| x.powf(alpha)).collect())
    }

    /// Swap self with `rhs`
    pub fn swap(&mut self, rhs: &mut Array) {
        std::mem::swap(&mut self.data, &mut rhs.data);
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::Array;

    #[test]
    fn test_indexing() {
        let a = Array::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(a[0], 1.0);
        assert_eq!(a[1], 2.0);
        assert_eq!(a[2], 3.0);
    }

    #[test]
    fn test_assignment() {
        let mut a = Array::new(vec![1.0, 2.0, 3.0]);
        a[0] = 4.0;
        a[1] = 5.0;
        a[2] = 6.0;
        assert_eq!(a, Array::new(vec![4.0, 5.0, 6.0]));
    }

    #[test]
    fn test_unary_negation() {
        let v1 = Array::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(-v1, Array::new(vec![-1.0, -2.0, -3.0]));
    }

    #[test]
    fn test_add_array() {
        let v1 = Array::new(vec![1.0, 2.0, 3.0]);
        let v2 = Array::new(vec![4.0, 5.0, 6.0]);
        assert_eq!(v1 + v2, Array::new(vec![5.0, 7.0, 9.0]));
    }

    #[test]
    fn test_add_scalar() {
        let v1 = Array::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(v1 + 2.0, Array::new(vec![3.0, 4.0, 5.0]));
    }

    #[test]
    fn test_subtract_array() {
        let v1 = Array::new(vec![1.0, 2.0, 3.0]);
        let v2 = Array::new(vec![4.0, 5.0, 6.0]);
        assert_eq!(v1 - v2, Array::new(vec![-3.0, -3.0, -3.0]));
    }

    #[test]
    fn test_subtract_scalar() {
        let v1 = Array::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(v1 - 2.0, Array::new(vec![-1.0, 0.0, 1.0]));
    }

    #[test]
    fn test_multiply_array() {
        let v1 = Array::new(vec![1.0, 2.0, 3.0]);
        let v2 = Array::new(vec![4.0, 5.0, 6.0]);
        assert_eq!(v1 * v2, Array::new(vec![4.0, 10.0, 18.0]));
    }

    #[test]
    fn test_multiply_scalar() {
        let v1 = Array::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(v1 * 2.0, Array::new(vec![2.0, 4.0, 6.0]));
    }

    #[test]
    fn test_divide_array() {
        let v1 = Array::new(vec![1.0, 2.0, 3.0]);
        let v2 = Array::new(vec![4.0, 5.0, 6.0]);
        assert_eq!(v1 / v2, Array::new(vec![0.25, 2.0 / 5.0, 0.5]));
    }

    #[test]
    fn test_divide_scalar() {
        let v1 = Array::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(v1 / 2.0, Array::new(vec![0.5, 1.0, 1.5]));
    }

    #[test]
    fn test_dot_product() {
        let v1 = Array::new(vec![1.0, 2.0, 3.0]);
        let v2 = Array::new(vec![4.0, 5.0, 6.0]);
        // 4 + 10 + 18 = 32
        assert_eq!(v1.dot_product(&v2), 32.0);
    }

    #[test]
    fn test_norm2() {
        let v1 = Array::new(vec![1.0, 2.0, 3.0]);
        // sqrt(1 + 4 + 9) = sqrt(14)
        assert_eq!(v1.norm2(), 14_f64.sqrt());
    }

    #[test]
    fn test_abs() {
        let v1 = Array::new(vec![-1.0, -2.0, -3.0]);
        let expected = Array::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(v1.abs(), expected);
    }

    #[test]
    fn test_sqrt() {
        let v1 = Array::new(vec![1.0, 2.0, 3.0]);
        let expected = Array::new(vec![1_f64.sqrt(), 2_f64.sqrt(), 3_f64.sqrt()]);
        assert_eq!(v1.sqrt(), expected);
    }

    #[test]
    fn test_log() {
        let v1 = Array::new(vec![1.0, 2.0, 3.0]);
        let e = std::f64::consts::E;
        let expected = Array::new(vec![1_f64.log(e), 2_f64.log(e), 3_f64.log(e)]);
        assert_eq!(v1.log(), expected);
    }

    #[test]
    fn test_exp() {
        let v1 = Array::new(vec![1.0, 2.0, 3.0]);
        let expected = Array::new(vec![1_f64.exp(), 2_f64.exp(), 3_f64.exp()]);
        assert_eq!(v1.exp(), expected);
    }

    #[test]
    fn test_pow() {
        let v1 = Array::new(vec![1.0, 2.0, 3.0]);
        let expected = Array::new(vec![1.0, 4.0, 9.0]);
        assert_eq!(v1.pow(2.0), expected);
    }

    #[test]
    fn test_swap() {
        let mut v1 = Array::new(vec![1.0, 2.0, 3.0]);
        let mut v2 = Array::new(vec![3.0, 4.0, 5.0]);
        v1.swap(&mut v2);
        assert_eq!(v1.data, vec![3.0, 4.0, 5.0]);
        assert_eq!(v2.data, vec![1.0, 2.0, 3.0]);
    }
}
