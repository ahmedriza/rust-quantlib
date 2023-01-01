use std::sync::Arc;

use crate::datetime::{
    calendar::easter_monday,
    calendar::{Calendar, Holiday, Weekend, WesternWeekend},
    date::Date,
    months::Month::*,
    weekday::Weekday,
};

/// Swiss calendar
///
/// Holidays:
/// * Saturdays
/// * Sundays
/// * New Year's Day, January 1st
/// * Berchtoldstag, January 2nd
/// * Good Friday
/// * Easter Monday
/// * Ascension Day
/// * Whit Monday
/// * Labour Day, May 1st
/// * National Day, August 1st
/// * Christmas, December 25th
/// * St. Stephen's Day, December 26th
#[derive(Clone)]
pub struct Switzerland {
    pub weekend: Arc<dyn Weekend>,
}

impl Switzerland {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Calendar {
        Calendar::new(Arc::new(Self {
            weekend: Arc::new(WesternWeekend {}),
        }))
    }
}

impl Holiday for Switzerland {
    fn name(&self) -> String {
        "Switzerland".into()
    }

    fn is_business_day(&self, date: &Date) -> bool {
        let w = date.weekday();
        let d = date.day_of_month();
        let dd = date.day_of_year();
        let m = date.month();
        let y = date.year();
        let em = easter_monday(y);

        if self.is_weekend(w)
            // New Year's Day
            || (d == 1  && m == January)
            // Berchtoldstag
            || (d == 2  && m == January)
            // Good Friday
            || (dd == em-3)
            // Easter Monday
            || (dd == em)
            // Ascension Day
            || (dd == em+38)
            // Whit Monday
            || (dd == em+49)
            // Labour Day
            || (d == 1  && m == May)
            // National Day
            || (d == 1  && m == August)
            // Christmas
            || (d == 25 && m == December)
            // St. Stephen's Day
            || (d == 26 && m == December)
        {
            return false;
        }
        true
    }

    fn is_weekend(&self, weekday: Weekday) -> bool {
        self.weekend.is_weekend(weekday)
    }
}
