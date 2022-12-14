use crate::maths::interpolations::interpolation::Interpolation;
use crate::time::date::Date;
use crate::types::{Real, Time};

pub struct InterpolatedCurve<I>
where
    I: Interpolation,
{
    pub interpolator: I,
    pub times: Vec<Time>,
    pub data: Vec<Real>,
    // Usually, the maximum date is the one corresponding to the last node. However, it might
    // happen that a bit of extrapolation is used by construction; for instance, when a curve is
    // bootstrapped and the last relevant date for an instrument is after the corresponding pillar.
    pub max_date: Date,
}

impl<I> InterpolatedCurve<I> where I: Interpolation {}
