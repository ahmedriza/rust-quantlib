use crate::types::{Integer, Time};

use crate::datetime::date::Date;

/// 30/365 day count convention
#[derive(Clone, Copy)]
pub struct Thirty365 {}

impl Default for Thirty365 {
    fn default() -> Self {
        Self::new()
    }
}

impl Thirty365 {
    pub fn new() -> Self {
        Thirty365 {}
    }

    pub fn name(&self) -> String {
        "30/365".into()
    }

    pub fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        let dd1 = d1.day_of_month();
        let dd2 = d2.day_of_month();
        let mm1 = d1.month() as Integer;
        let mm2 = d2.month() as Integer;
        let yy1 = d1.year();
        let yy2 = d2.year();

        360 * (yy2 - yy1) + 30 * (mm2 - mm1) + (dd2 - dd1) as Integer
    }

    pub fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        _ref_period_start: &Date,
        _ref_period_end: &Date,
    ) -> Time {
        self.day_count(d1, d2) as Time / 365.0
    }
}
