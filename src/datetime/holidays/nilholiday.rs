use std::fmt::Debug;

use crate::datetime::{
    calendar::Calendar,
    date::Date,
    holiday,
    weekday::Weekday,
    weekend::{NilWeekend, Weekend},
};

/// Calendar for reproducing theoretical calculations.
/// This calendar has no holidays. It ensures that dates at whole-month distances have the same
/// day of month.
#[derive(Clone, Copy)]
pub struct NilHoliday {
    pub weekend: Weekend,
}

impl Debug for NilHoliday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl NilHoliday {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Calendar {
        Calendar::new(holiday::Holiday::NilHoliday(Self {
            weekend: Weekend::NilWeekend(NilWeekend {}),
        }))
    }

    pub fn name(&self) -> String {
        "Null".into()
    }

    pub fn is_business_day(&self, _date: &Date) -> bool {
        true
    }

    pub fn is_weekend(&self, weekday: Weekday) -> bool {
        self.weekend.is_weekend(weekday)
    }
}
