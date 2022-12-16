use crate::{
    time::date::Date,
    types::{Integer, Time},
};

/// Actual/360 day count convention, also known as "Act/360", or "A/360".
#[derive(Clone, Copy)]
pub struct Actual360 {
    include_last_day: bool,
}

impl Default for Actual360 {
    fn default() -> Self {
        Self::new()
    }
}

impl Actual360 {
    pub fn new() -> Self {
        Self::with_last_day(false)
    }

    pub fn with_last_day(include_last_day: bool) -> Self {
        Self { include_last_day }
    }

    pub fn name(&self) -> String {
        if self.include_last_day {
            "Actual/360 (inc)".into()
        } else {
            "Actual/360".into()
        }
    }

    pub fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        if self.include_last_day {
            (d2 - d1) + 1
        } else {
            d2 - d1
        }
    }

    pub fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        _ref_period_start: &Date,
        _ref_period_end: &Date,
    ) -> Time {
        if self.include_last_day {
            (Date::days_between(d1, d2) + 1.0) / 360.0
        } else {
            Date::days_between(d1, d2) / 360.0
        }
    }
}
