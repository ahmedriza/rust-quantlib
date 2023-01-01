use std::marker::PhantomData;

use crate::{
    datetime::date::Date,
    instruments::instrument::Instrument,
    types::{DiscountFactor, Real, Time},
};

use super::iterativebootstrap::IterativeBootstrap;

/// Piecewise yield term structure
///
/// This term structure is bootstrapped on a number of interest rate instruments which are passed
/// as a vector of RateHelper instances. Their maturities mark the boundaries of the interpolated
/// segments.
///
/// Each segment is determined sequentially starting from the earliest period to the latest and is
/// chosen so that the instrument whose maturity marks the end of such segment is correctly
/// repriced on the curve.
///
/// The bootstrapping algorithm will fail if any two instruments have the same maturity date.
///
/// # Type parameters
///
/// * `C` - type of curve
/// * `I` - type of interpolation
///
/// # Arguments
///
/// * `instruments` - vector of instruments
/// * `accuracy` - desired accuracy of the bootstrapping
/// * `bootstrap` - bootstrapping algorithm implementation
pub struct PiecewiseYieldCurve<C, I> {
    pub instruments: Vec<Box<dyn Instrument>>,
    pub accuracy: Real,
    pub bootstrap: IterativeBootstrap,
    _ph_c: PhantomData<C>,
    _ph_i: PhantomData<I>,
}

impl<C, I> PiecewiseYieldCurve<C, I> {
    pub fn max_date() -> Date {
        todo!()
    }

    pub fn discount_impl(_time: Time) -> DiscountFactor {
        todo!()
    }

    pub fn perform_calculations(&self) {
        self.bootstrap.calculate();
    }
}
