use std::sync::Arc;

use crate::types::{Integer, Time};

use crate::time::{
    date::Date,
    daycounter::{DayCounter, DayCounterDetail},
};

/// 30/365 day count convention
pub struct Thirty365 {}

impl Thirty365 {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> DayCounter {
        DayCounter::new(Arc::new(Thirty365 {}))
    }
}

impl DayCounterDetail for Thirty365 {
    fn name(&self) -> String {
        "30/365".into()
    }

    fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        let dd1 = d1.day_of_month();
        let dd2 = d2.day_of_month();
        let mm1 = d1.month() as Integer;
        let mm2 = d2.month() as Integer;
        let yy1 = d1.year();
        let yy2 = d2.year();

        360 * (yy2 - yy1) + 30 * (mm2 - mm1) + (dd2 - dd1) as Integer
    }

    fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        _ref_period_start: &Date,
        _ref_period_end: &Date,
    ) -> Time {
        self.day_count(d1, d2) as Time / 365.0
    }
}
