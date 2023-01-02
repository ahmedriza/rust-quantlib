use std::rc::Rc;

use crate::datetime::date::Date;
use crate::types::Real;

pub trait CashFlow {
    /// Returns the amount of the cash flow. The amount is not discounted, i.e., it is the
    /// actual amount paid at the cash flow date.
    fn amount(&self) -> Real;
    
    /// Returns the date at which the cashflow occurs
    fn date(&self) -> Date;

    /// Returns the date that the cash flow trades ex-coupon
    fn ex_coupon_date(&self) -> Date {
        Date::default()
    }
    
    /// Returns true if a cashflow has already occurred before a date.
    fn has_occurred(&self, _ref_date: &Date, _include_ref_date: Option<bool>) -> bool {
        todo!()
    }

    /// Returns true if the cashflow is trading ex-coupon on the `ref_date`.
    /// TODO `ref_date` should default to pricing date 
    fn trading_ex_coupon(&self, ref_date: Option<Date>) -> bool {
        let ecd = self.ex_coupon_date();
        if ecd == Date::default() {
            return false;
        }
        let ref_date = ref_date.unwrap_or_default();
        ecd <= ref_date
    }
}

/// Sequence of cashflows
pub type Leg = Vec<Rc<dyn CashFlow>>;

