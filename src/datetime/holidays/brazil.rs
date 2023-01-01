use std::fmt::Debug;

use crate::datetime::{
    calendar::Calendar,
    date::Date,
    holiday::{self, easter_monday},
    months::Month::*,
    weekday::Weekday,
    weekend::{Weekend, WesternWeekend},
};

/// Brazilian calendar
///
/// Banking holidays:
/// * Saturdays
/// * Sundays
/// * New Year's Day, January 1st
/// * Tiradentes's Day, April 21th
/// * Labour Day, May 1st
/// * Independence Day, September 7th
/// * Nossa Sra. Aparecida Day, October 12th
/// * All Souls Day, November 2nd
/// * Republic Day, November 15th
/// * Christmas, December 25th
/// * Passion of Christ
/// * Carnival
/// * Corpus Christi
///
/// Holidays for the Bovespa stock exchange
/// * Saturdays
/// * Sundays
/// * New Year's Day, January 1st
/// * Sao Paulo City Day, January 25th
/// * Tiradentes's Day, April 21th
/// * Labour Day, May 1st
/// * Revolution Day, July 9th
/// * Independence Day, September 7th
/// * Nossa Sra. Aparecida Day, October 12th
/// * All Souls Day, November 2nd
/// * Republic Day, November 15th
/// * Black Consciousness Day, November 20th (since 2007)
/// * Christmas Eve, December 24th
/// * Christmas, December 25th
/// * Passion of Christ
/// * Carnival
/// * Corpus Christi
/// * The last business day of the year
#[derive(Clone, Copy)]
pub struct Brazil {}

impl Brazil {
    #[allow(clippy::new_ret_no_self)]
    /// The default calendar is the settlement calendar
    pub fn new() -> Calendar {
        BrazilSettlement::new()
    }

    /// Create an instance of the [BrazilExchange] calendar
    pub fn exchange() -> Calendar {
        BrazilExchange::new()
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct BrazilSettlement {
    pub weekend: Weekend,
}

impl Debug for BrazilSettlement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl BrazilSettlement {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Calendar {
        Calendar::new(holiday::Holiday::BrazilSettlement(Self {
            weekend: Weekend::WesternWeekend(WesternWeekend {}),
        }))
    }

    pub fn name(&self) -> String {
        "Brazil".into()
    }

    pub fn is_business_day(&self, date: &Date) -> bool {
        let w = date.weekday();
        let d = date.day_of_month();
        let m = date.month();
        let y = date.year();
        let dd = date.day_of_year();
        let em = easter_monday(y);

        if self.is_weekend(w)
            // New Year's Day 
            || (d == 1 && m == January)
            // Tiradentes Day
            || (d == 21 && m == April)
            // Labor Day
            || (d == 1 && m == May)
            // Independence Day
            || (d == 7 && m == September)
            // Nossa Sra. Aparecida Day
            || (d == 12 && m == October)
            // All Souls Day
            || (d == 2 && m == November)
            // Republic Day
            || (d == 15 && m == November)
            // Christmas
            || (d == 25 && m == December)
            // Passion of Christ
            || (dd == em - 3)
            // Carnival
            || (dd == em - 49 || dd == em - 48)
            // Corpus Christi
            || (dd == em + 59)
        {
            return false;
        }
        true
    }

    pub fn is_weekend(&self, weekday: Weekday) -> bool {
        self.weekend.is_weekend(weekday)
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct BrazilExchange {
    pub weekend: Weekend,
}

impl Debug for BrazilExchange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl BrazilExchange {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Calendar {
        Calendar::new(holiday::Holiday::BrazilExchange(Self {
            weekend: Weekend::WesternWeekend(WesternWeekend {}),
        }))
    }

    pub fn name(&self) -> String {
        "BOVESPA".into()
    }

    pub fn is_business_day(&self, date: &Date) -> bool {
        let w = date.weekday();
        let d = date.day_of_month();
        let m = date.month();
        let y = date.year();
        let dd = date.day_of_year();
        let em = easter_monday(y);

        if self.is_weekend(w)
            // New Year's Day
            || (d == 1 && m == January)
            // Sao Paulo City Day
            || (d == 25 && m == January)
            // Tiradentes Day
            || (d == 21 && m == April)
            // Labor Day
            || (d == 1 && m == May)
            // Revolution Day
            || (d == 9 && m == July)
            // Independence Day
            || (d == 7 && m == September)
            // Nossa Sra. Aparecida Day
            || (d == 12 && m == October)
            // All Souls Day
            || (d == 2 && m == November)
            // Republic Day
            || (d == 15 && m == November)
            // Black Consciousness Day
            || (d == 20 && m == November && y >= 2007)
            // Christmas Eve
            || (d == 24 && m == December)
            // Christmas
            || (d == 25 && m == December)
            // Passion of Christ
            || (dd == em-3)
            // Carnival
            || (dd == em-49 || dd == em-48)
            // Corpus Christi
            || (dd == em+59)
            // last business day of the year
            || (m == December && (d == 31 || (d >= 29 && w == Weekday::Friday)))
        {
            return false;
        }
        true
    }

    pub fn is_weekend(&self, weekday: Weekday) -> bool {
        self.weekend.is_weekend(weekday)
    }
}
