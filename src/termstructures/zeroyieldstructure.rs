use crate::types::{DiscountFactor, Rate, Time};

/// Discount and forward are calculated from zero yields.
///
/// Zero rates are assumed to be annual continuous compounding.
pub trait ZeroYieldStructure {
    /// Calcualte zero yield.  When it is called, range check must have been performed; therefore
    /// it must assume that extrapolation is required.
    fn zero_yield(&self, time: Time) -> Rate;

    /// Returns the discount factor for the given date calculating it from the zero yield.
    fn discount(&self, time: Time) -> DiscountFactor {
        if time == 0.0 {
            return 1.0;
        }
        let r = self.zero_yield(time);
        (-r * time).exp()
    }
}
