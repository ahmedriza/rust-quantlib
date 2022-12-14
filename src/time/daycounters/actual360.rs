use std::sync::Arc;

use crate::types::{Integer, Time};

use crate::time::{
    date::Date,
    daycounter::{DayCounter, DayCounterDetail},
};

/// Actual/360 day count convention, also known as "Act/360", or "A/360".
pub struct Actual360 {
    include_last_day: bool,
}

impl Actual360 {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> DayCounter {
        Actual360::with_last_day(false)
    }

    pub fn with_last_day(include_last_day: bool) -> DayCounter {
        DayCounter::new(Arc::new(Actual360 { include_last_day }))
    }
}

impl DayCounterDetail for Actual360 {
    fn name(&self) -> String {
        if self.include_last_day {
            "Actual/360 (inc)".into()
        } else {
            "Actual/360".into()
        }
    }

    fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        if self.include_last_day {
            (d2 - d1) + 1
        } else {
            d2 - d1
        }
    }

    fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        _ref_period_start: &Date,
        _ref_period_end: &Date,
    ) -> Time {
        if self.include_last_day {
            (Date::days_between(d1, d2) + 1.0) / 360.0
        } else {
            (Date::days_between(d1, d2)) / 360.0
        }
    }
}
