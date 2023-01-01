use std::sync::Arc;

use crate::datetime::{
    calendar::{easter_monday, Calendar, Holiday, Weekend, WesternWeekend},
    date::Date,
    months::Month::*,
    weekday::Weekday,
};

#[derive(Clone)]
pub struct Italy {}

impl Italy {
    #[allow(clippy::new_ret_no_self)]
    /// The default calendar is the settlement calendar
    pub fn new() -> Calendar {
        ItalySettlement::new()
    }

    pub fn exchange() -> Calendar {
        ItalyExchange::new()
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct ItalySettlement {
    pub weekend: Arc<dyn Weekend>,
}

impl ItalySettlement {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> Calendar {
        Calendar::new(Arc::new(ItalySettlement {
            weekend: Arc::new(WesternWeekend {}),
        }))
    }
}

impl Holiday for ItalySettlement {
    fn name(&self) -> String {
        "Italian settlement".into()
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
            || (d == 1 && m == January)
            // Epiphany
            || (d == 6 && m == January)
            // Easter Monday
            || (dd == em)
            // Liberation Day
            || (d == 25 && m == April)
            // Labour Day
            || (d == 1 && m == May)
            // Republic Day
            || (d == 2 && m == June && y >= 2000)
            // Assumption
            || (d == 15 && m == August)
            // All Saints' Day
            || (d == 1 && m == November)
            // Immaculate Conception
            || (d == 8 && m == December)
            // Christmas
            || (d == 25 && m == December)
            // St. Stephen
            || (d == 26 && m == December)
            // December 31st, 1999 only
            || (d == 31 && m == December && y == 1999)
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
pub struct ItalyExchange {
    pub weekend: Arc<dyn Weekend>,
}

impl ItalyExchange {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> Calendar {
        Calendar::new(Arc::new(ItalyExchange {
            weekend: Arc::new(WesternWeekend {}),
        }))
    }
}

impl Holiday for ItalyExchange {
    fn name(&self) -> String {
        "Milan stock exchange".into()
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
            || (d == 1 && m == January)
            // Good Friday
            || (dd == em-3)
            // Easter Monday
            || (dd == em)
            // Labour Day
            || (d == 1 && m == May)
            // Assumption
            || (d == 15 && m == August)
            // Christmas' Eve
            || (d == 24 && m == December)
            // Christmas
            || (d == 25 && m == December)
            // St. Stephen
            || (d == 26 && m == December)
            // New Year's Eve
            || (d == 31 && m == December)
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

    use super::Italy;

    #[test]
    fn test_exchange() {
        let expected_hol = vec![
            Date::new(1, January, 2002),
            Date::new(29, March, 2002),
            Date::new(1, April, 2002),
            Date::new(1, May, 2002),
            Date::new(15, August, 2002),
            Date::new(24, December, 2002),
            Date::new(25, December, 2002),
            Date::new(26, December, 2002),
            Date::new(31, December, 2002),
            //
            Date::new(1, January, 2003),
            Date::new(18, April, 2003),
            Date::new(21, April, 2003),
            Date::new(1, May, 2003),
            Date::new(15, August, 2003),
            Date::new(24, December, 2003),
            Date::new(25, December, 2003),
            Date::new(26, December, 2003),
            Date::new(31, December, 2003),
            //
            Date::new(1, January, 2004),
            Date::new(9, April, 2004),
            Date::new(12, April, 2004),
            Date::new(24, December, 2004),
            Date::new(31, December, 2004),
        ];

        let c = Italy::exchange();

        let hol = c.holiday_list(
            Date::new(1, January, 2002),
            Date::new(31, December, 2004),
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
