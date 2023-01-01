use std::fmt::Debug;

use crate::datetime::{
    calendar::{easter_monday, Calendar},
    date::Date,
    holiday,
    months::Month::*,
    weekday::Weekday,
    weekend::{Weekend, WesternWeekend},
};

#[derive(Clone, Copy)]
pub struct Switzerland {
    weekend: Weekend,
}

impl Debug for Switzerland {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Switzerland {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Calendar {
        Calendar::new(holiday::Holiday::Switzerland(Self {
            weekend: Weekend::WesternWeekend(WesternWeekend {}),
        }))
    }

    pub fn name(&self) -> String {
        "Switzerland".into()
    }

    pub fn is_business_day(&self, date: &Date) -> bool {
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

    pub fn is_weekend(&self, weekday: Weekday) -> bool {
        self.weekend.is_weekend(weekday)
    }
}
