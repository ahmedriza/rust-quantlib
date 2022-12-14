use crate::time::date::Date;
use crate::types::Real;

pub trait Cashflow {
    // Returns the date at which the cashflow occurs
    fn date(&self) -> Date;

    /// Returns true if a cashflow has already occurred before a date.
    fn has_occurred(&self, ref_date: &Date, include_ref_date: Option<bool>) -> bool;

    /// Returns the amount of the cash flow. The amount is not discounted, i.e., it is the
    /// actual amount paid at the cash flow date.
    fn amount(&self) -> Real;

    /// Returns the date that the cash flow trades ex-coupon
    fn ex_coupon_date(&self) -> Date;

    /// Returns true if the cashflow is trading ex-coupon on the `ref_date`.
    fn trading_ex_coupon(&self, ref_date: &Date) -> bool;
}

/// Sequence of cashflows
pub type Leg = Vec<Box<dyn Cashflow>>;
