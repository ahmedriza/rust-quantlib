use std::sync::Arc;

use crate::types::{Integer, Time};

use crate::time::{
    date::Date,
    daycounter::{DayCounter, DayCounterDetail},
};

use super::thirty360::Thirty360;

/// Simple day counter for reproducing theoretical calculations.
///
/// This day counter tries to ensure that whole-month distances are returned as a simple
/// fraction, i.e., 1 year = 1.0, 6 months = 0.5, 3 months = 0.25 and so forth.
///
/// This day counter should be used together with NullCalendar, which ensures that dates at
/// whole-month distances share the same day of month. It is **not** guaranteed to work with
/// any other calendar.
pub struct SimpleDayCounter {
    fallback: DayCounter,
}

impl SimpleDayCounter {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> DayCounter {
        DayCounter::new(Arc::new(Self {
            fallback: Thirty360::bond_basis(),
        }))
    }
}

impl DayCounterDetail for SimpleDayCounter {
    fn name(&self) -> String {
        "Simple".into()
    }

    fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        self.fallback.day_count(d1, d2)
    }

    fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        _ref_period_start: &Date,
        _ref_period_end: &Date,
    ) -> Time {
        let dm1 = d1.day_of_month();
        let dm2 = d2.day_of_month();

        if dm1 == dm2  ||
            // e.g., Aug 30 -> Feb 28 ?
            (dm1 > dm2 && d2.is_end_of_month()) ||
            // e.g., Feb 28 -> Aug 30 ?
            (dm1 < dm2 && d1.is_end_of_month())
        {
            (d2.year() - d1.year()) as Time
                + (d2.month() as Integer - d1.month() as Integer) as Time / 12.0
        } else {
            self.fallback.year_fraction(d1, d2)
        }
    }
}
