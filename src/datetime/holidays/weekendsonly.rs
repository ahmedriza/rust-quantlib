use std::fmt::Debug;

use crate::datetime::{
    calendar::Calendar,
    date::Date,
    holiday,
    weekday::Weekday,
    weekend::{Weekend, WesternWeekend},
};

/// Weekends-only calendar
///
/// This calendar has no bank holidays except for weekends (Saturdays and Sundays) as required
/// by ISDA for calculating conventional CDS spreads.
#[derive(Clone, Copy)]
pub struct WeekendsOnly {
    pub weekend: Weekend,
}

impl Debug for WeekendsOnly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl WeekendsOnly {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Calendar {
        Calendar::new(holiday::Holiday::WeekendsOnly(Self {
            weekend: Weekend::WesternWeekend(WesternWeekend {}),
        }))
    }

    pub fn name(&self) -> String {
        "weekends only".into()
    }

    pub fn is_business_day(&self, date: &Date) -> bool {
        !self.is_weekend(date.weekday())
    }

    pub fn is_weekend(&self, weekday: Weekday) -> bool {
        self.weekend.is_weekend(weekday)
    }
}
