use std::fmt::Debug;

use crate::datetime::{
    calendar::Calendar,
    date::Date,
    holiday::{self, easter_monday},
    months::Month::*,
    weekday::Weekday,
    weekend::{Weekend, WesternWeekend},
};

/// TARGET calendar
///
/// Holidays (see [ECB](http://www.ecb.int)):
/// * Saturdays
/// * Sundays
/// * New Year's Day, January 1st
/// * Good Friday (since 2000)
/// * Easter Monday (since 2000)
/// * Labour Day, May 1st (since 2000)
/// * Christmas, December 25th
/// * Day of Goodwill, December 26th (since 2000)
/// * December 31st (1998, 1999, and 2001)
#[derive(Clone, Copy)]
pub struct Target {
    weekend: Weekend,
}

impl Debug for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Target {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Calendar {
        Calendar::new(holiday::Holiday::Target(Self {
            weekend: Weekend::WesternWeekend(WesternWeekend {}),
        }))
    }

    pub fn name(&self) -> String {
        "TARGET".into()
    }

    pub fn is_business_day(&self, date: &Date) -> bool {
        let weekday = date.weekday();
        let day_of_month = date.day_of_month();
        let day_of_year = date.day_of_year();
        let month = date.month();
        let year = date.year();
        let easter_monday = easter_monday(year);

        if self.is_weekend(weekday)
            // New Year's day 
            || (day_of_month == 1 && month == January)
            // Good Friday 
            || (day_of_year == easter_monday - 3 && year >= 2000)
            // Easter Monday
            || (day_of_year == easter_monday && year >= 2000)
            // Labour Day 
            || (day_of_month == 1 && month == May && year >= 2000)
            // Christmas 
            || (day_of_month == 25 && month == December)
            // Day of Goodwill 
            || (day_of_month == 26 && month == December && year >= 2000)
            // December 31st, 1998, 1999, and 2001 only
            || (day_of_month == 31 && month == December &&
                (year == 1998 || year == 1999 || year == 2001))
        {
            return false;
        }
        true
    }

    pub fn is_weekend(&self, weekday: Weekday) -> bool {
        self.weekend.is_weekend(weekday)
    }
}
