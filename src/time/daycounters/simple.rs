use crate::{
    time::date::Date,
    types::{Integer, Time},
};

use super::thirty360::{Thirty360, Thiry360Convention, ISMA};

/// Simple day counter for reproducing theoretical calculations.
///
/// This day counter tries to ensure that whole-month distances are returned as a simple
/// fraction, i.e., 1 year = 1.0, 6 months = 0.5, 3 months = 0.25 and so forth.
///
/// This day counter should be used together with NullCalendar, which ensures that dates at
/// whole-month distances share the same day of month. It is **not** guaranteed to work with
/// any other calendar.
#[derive(Clone, Copy)]
pub struct Simple {
    pub fallback: Thirty360,
}

impl Default for Simple {
    fn default() -> Self {
        Self::new()
    }
}

impl Simple {
    pub fn new() -> Self {
        Self {
            fallback: Thirty360 {
                convention: Thiry360Convention::ISMA(ISMA {}),
            },
        }
    }

    pub fn name(&self) -> String {
        "Simple".into()
    }

    pub fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        self.fallback.day_count(d1, d2)
    }

    pub fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        ref_period_start: &Date,
        ref_period_end: &Date,
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
            self.fallback
                .year_fraction(d1, d2, ref_period_start, ref_period_end)
        }
    }
}
