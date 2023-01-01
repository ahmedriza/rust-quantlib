use std::sync::Arc;

use crate::datetime::{
    calendar::{easter_monday, Calendar, Holiday, Weekend, WesternWeekend},
    date::Date,
    months::Month::*,
    weekday::Weekday,
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
/// * Corpus Christiqqq
/// * The last business day of the year
#[derive(Clone)]
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

#[derive(Clone)]
pub struct BrazilSettlement {
    pub weekend: Arc<dyn Weekend>,
}

impl BrazilSettlement {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> Calendar {
        Calendar::new(Arc::new(BrazilSettlement {
            weekend: Arc::new(WesternWeekend {}),
        }))
    }
}

impl Holiday for BrazilSettlement {
    fn name(&self) -> String {
        "Brazil".into()
    }

    fn is_business_day(&self, date: &Date) -> bool {
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

    fn is_weekend(&self, weekday: Weekday) -> bool {
        self.weekend.is_weekend(weekday)
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct BrazilExchange {
    pub weekend: Arc<dyn Weekend>,
}

impl BrazilExchange {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> Calendar {
        Calendar::new(Arc::new(BrazilExchange {
            weekend: Arc::new(WesternWeekend {}),
        }))
    }
}

impl Holiday for BrazilExchange {
    fn name(&self) -> String {
        "BOVESPA".into()
    }

    fn is_business_day(&self, date: &Date) -> bool {
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

    fn is_weekend(&self, weekday: Weekday) -> bool {
        self.weekend.is_weekend(weekday)
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::datetime::date::Date;
    use crate::datetime::months::Month::*;

    use super::Brazil;

    #[test]
    fn test_brazil() {
        let expected_hol = vec![
            Date::new(7, February, 2005),
            Date::new(8, February, 2005),
            Date::new(25, March, 2005),
            Date::new(21, April, 2005),
            Date::new(26, May, 2005),
            Date::new(7, September, 2005),
            Date::new(12, October, 2005),
            Date::new(2, November, 2005),
            Date::new(15, November, 2005),
            //
            Date::new(27, February, 2006),
            Date::new(28, February, 2006),
            Date::new(14, April, 2006),
            Date::new(21, April, 2006),
            Date::new(1, May, 2006),
            Date::new(15, June, 2006),
            Date::new(7, September, 2006),
            Date::new(12, October, 2006),
            Date::new(2, November, 2006),
            Date::new(15, November, 2006),
            Date::new(25, December, 2006),
        ];

        let c = Brazil::new();

        let hol = c.holiday_list(
            Date::new(1, January, 2005),
            Date::new(31, December, 2006),
            false,
        );

        assert!(
            hol.len() == expected_hol.len(),
            "there were {} expected holidays, while there are {} calculated holidays",
            expected_hol.len(),
            hol.len()
        );

        for i in 0..expected_hol.len() {
            assert!(
                hol[i] == expected_hol[i],
                "expected holiday was {:?} while calculated holiday is {:?}",
                expected_hol[i],
                hol[i]
            );
        }
    }
}
