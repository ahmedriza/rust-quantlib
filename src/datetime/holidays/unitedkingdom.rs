use std::fmt::Debug;

use crate::datetime::{
    calendar::{easter_monday, Calendar},
    date::Date,
    holiday,
    months::Month::{self, *},
    weekday::Weekday::{self, *},
    weekend::{Weekend, WesternWeekend},
    Day, Year,
};

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct UnitedKingdom {}

impl UnitedKingdom {
    #[allow(clippy::new_ret_no_self)]
    /// The default calendar is the [UnitedKingdomSettlement] calendar
    pub fn new() -> Calendar {
        UnitedKingdomSettlement::new()        
    }

    /// Create an instance of [UnitedKingdomExchange] calendar
    pub fn exchange() -> Calendar {
        UnitedKingdomExchange::new()
    }

    /// Create an instance of [UnitedKingdomMetals] calendar    
    pub fn metals() -> Calendar {
        UnitedKingdomMetals::new()
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct UnitedKingdomSettlement {
    pub weekend: Weekend,
}

impl Debug for UnitedKingdomSettlement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl UnitedKingdomSettlement {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Calendar {
        Calendar::new(holiday::Holiday::UnitedKingdomSettlement(Self {
            weekend: Weekend::WesternWeekend(WesternWeekend {}),
        }))
    }

    pub fn name(&self) -> String {
        "UK settlement".into()
    }

    pub fn is_business_day(&self, date: &Date) -> bool {
        let w = date.weekday();
        let d = date.day_of_month();
        let dd = date.day_of_year();
        let m = date.month();
        let y = date.year();
        let em = easter_monday(y);

        if self.is_weekend(w)
            // New Year's Day (possibly moved to Monday)
            || ((d == 1 || ((d == 2 || d == 3) && w == Monday)) &&
                m == January)
            // Good Friday
            || (dd == em-3)
            // Easter Monday
            || (dd == em)
            || is_bank_holiday(d, w, m, y)
            // Christmas (possibly moved to Monday or Tuesday)
            || ((d == 25 || (d == 27 && (w == Monday || w == Tuesday)))
                && m == December)
            // Boxing Day (possibly moved to Monday or Tuesday)
            || ((d == 26 || (d == 28 && (w == Monday || w == Tuesday)))
                && m == December)
            // December 31st, 1999 only
            || (d == 31 && m == December && y == 1999)
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
pub struct UnitedKingdomExchange {
    pub weekend: Weekend,
}

impl Debug for UnitedKingdomExchange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl UnitedKingdomExchange {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Calendar {
        Calendar::new(holiday::Holiday::UnitedKingdomExchange(Self {
            weekend: Weekend::WesternWeekend(WesternWeekend {}),
        }))
    }

    pub fn name(&self) -> String {
        "London stock exchange".into()
    }

    pub fn is_business_day(&self, date: &Date) -> bool {
        let w = date.weekday();
        let d = date.day_of_month();
        let dd = date.day_of_year();
        let m = date.month();
        let y = date.year();
        let em = easter_monday(y);

        if self.is_weekend(w)
            // New Year's Day (possibly moved to Monday)
            || ((d == 1 || ((d == 2 || d == 3) && w == Monday)) &&
                m == January)
            // Good Friday
            || (dd == em-3)
            // Easter Monday
            || (dd == em)
            || is_bank_holiday(d, w, m, y)
            // Christmas (possibly moved to Monday or Tuesday)
            || ((d == 25 || (d == 27 && (w == Monday || w == Tuesday)))
                && m == December)
            // Boxing Day (possibly moved to Monday or Tuesday)
            || ((d == 26 || (d == 28 && (w == Monday || w == Tuesday)))
                && m == December)
            // December 31st, 1999 only
            || (d == 31 && m == December && y == 1999)
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
pub struct UnitedKingdomMetals {
    pub weekend: Weekend,
}

impl Debug for UnitedKingdomMetals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl UnitedKingdomMetals {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Calendar {
        Calendar::new(holiday::Holiday::UnitedKingdomMetals(Self {
            weekend: Weekend::WesternWeekend(WesternWeekend {}),
        }))
    }

    pub fn name(&self) -> String {
        "London metals exchange".into()
    }

    pub fn is_business_day(&self, date: &Date) -> bool {
        let w = date.weekday();
        let d = date.day_of_month();
        let dd = date.day_of_year();
        let m = date.month();
        let y = date.year();
        let em = easter_monday(y);

        if self.is_weekend(w)
            // New Year's Day (possibly moved to Monday)
            || ((d == 1 || ((d == 2 || d == 3) && w == Monday)) &&
                m == January)
            // Good Friday
            || (dd == em-3)
            // Easter Monday
            || (dd == em)
            || is_bank_holiday(d, w, m, y)
            // Christmas (possibly moved to Monday or Tuesday)
            || ((d == 25 || (d == 27 && (w == Monday || w == Tuesday)))
                && m == December)
            // Boxing Day (possibly moved to Monday or Tuesday)
            || ((d == 26 || (d == 28 && (w == Monday || w == Tuesday)))
                && m == December)
            // December 31st, 1999 only
            || (d == 31 && m == December && y == 1999)
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

fn is_bank_holiday(d: Day, w: Weekday, m: Month, y: Year) -> bool {
    // first Monday of May (Early May Bank Holiday)
    // moved to May 8th in 1995 and 2020 for V.E. day
    (d <= 7 && w == Monday && m == May && y != 1995 && y != 2020)
                || (d == 8 && m == May && (y == 1995 || y == 2020))
                // last Monday of May (Spring Bank Holiday)
                // moved to in 2002, 2012 and 2022 for the Golden, Diamond and Platinum
                // Jubilee with an additional holiday
                || (d >= 25 && w == Monday && m == May && y != 2002 && y != 2012 && y != 2022)
                || ((d == 3 || d == 4) && m == June && y == 2002)
                || ((d == 4 || d == 5) && m == June && y == 2012)
                || ((d == 2 || d == 3) && m == June && y == 2022)
                // last Monday of August (Summer Bank Holiday)
                || (d >= 25 && w == Monday && m == August)
                // April 29th, 2011 only (Royal Wedding Bank Holiday)
                || (d == 29 && m == April && y == 2011)
                // September 19th, 2022 only (The Queen's Funeral Bank Holiday)
                || (d == 19 && m == September && y == 2022)
                // May 8th, 2023 (King Charles III Coronation Bank Holiday)
                || (d == 8 && m == May && y == 2023)
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::datetime::date::Date;
    use crate::datetime::months::Month::*;

    use super::UnitedKingdom;

    #[test]
    fn test_settlement() {
        let expected_hol = vec![
            Date::new(1, January, 2004),
            Date::new(9, April, 2004),
            Date::new(12, April, 2004),
            Date::new(3, May, 2004),
            Date::new(31, May, 2004),
            Date::new(30, August, 2004),
            Date::new(27, December, 2004),
            Date::new(28, December, 2004),
            //
            Date::new(3, January, 2005),
            Date::new(25, March, 2005),
            Date::new(28, March, 2005),
            Date::new(2, May, 2005),
            Date::new(30, May, 2005),
            Date::new(29, August, 2005),
            Date::new(26, December, 2005),
            Date::new(27, December, 2005),
            //
            Date::new(2, January, 2006),
            Date::new(14, April, 2006),
            Date::new(17, April, 2006),
            Date::new(1, May, 2006),
            Date::new(29, May, 2006),
            Date::new(28, August, 2006),
            Date::new(25, December, 2006),
            Date::new(26, December, 2006),
            //
            Date::new(1, January, 2007),
            Date::new(6, April, 2007),
            Date::new(9, April, 2007),
            Date::new(7, May, 2007),
            Date::new(28, May, 2007),
            Date::new(27, August, 2007),
            Date::new(25, December, 2007),
            Date::new(26, December, 2007),
        ];

        let c = UnitedKingdom::new();

        let hol = c.holiday_list(
            Date::new(1, January, 2004),
            Date::new(31, December, 2007),
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

    #[test]
    fn test_exchange() {
        let expected_hol = vec![
            Date::new(1, January, 2004),
            Date::new(9, April, 2004),
            Date::new(12, April, 2004),
            Date::new(3, May, 2004),
            Date::new(31, May, 2004),
            Date::new(30, August, 2004),
            Date::new(27, December, 2004),
            Date::new(28, December, 2004),
            //
            Date::new(3, January, 2005),
            Date::new(25, March, 2005),
            Date::new(28, March, 2005),
            Date::new(2, May, 2005),
            Date::new(30, May, 2005),
            Date::new(29, August, 2005),
            Date::new(26, December, 2005),
            Date::new(27, December, 2005),
            //
            Date::new(2, January, 2006),
            Date::new(14, April, 2006),
            Date::new(17, April, 2006),
            Date::new(1, May, 2006),
            Date::new(29, May, 2006),
            Date::new(28, August, 2006),
            Date::new(25, December, 2006),
            Date::new(26, December, 2006),
            //
            Date::new(1, January, 2007),
            Date::new(6, April, 2007),
            Date::new(9, April, 2007),
            Date::new(7, May, 2007),
            Date::new(28, May, 2007),
            Date::new(27, August, 2007),
            Date::new(25, December, 2007),
            Date::new(26, December, 2007),
        ];

        let c = UnitedKingdom::exchange();

        let hol = c.holiday_list(
            Date::new(1, January, 2004),
            Date::new(31, December, 2007),
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

    #[test]
    fn test_metals() {
        let expected_hol = vec![
            Date::new(1, January, 2004),
            Date::new(9, April, 2004),
            Date::new(12, April, 2004),
            Date::new(3, May, 2004),
            Date::new(31, May, 2004),
            Date::new(30, August, 2004),
            Date::new(27, December, 2004),
            Date::new(28, December, 2004),
            //
            Date::new(3, January, 2005),
            Date::new(25, March, 2005),
            Date::new(28, March, 2005),
            Date::new(2, May, 2005),
            Date::new(30, May, 2005),
            Date::new(29, August, 2005),
            Date::new(26, December, 2005),
            Date::new(27, December, 2005),
            //
            Date::new(2, January, 2006),
            Date::new(14, April, 2006),
            Date::new(17, April, 2006),
            Date::new(1, May, 2006),
            Date::new(29, May, 2006),
            Date::new(28, August, 2006),
            Date::new(25, December, 2006),
            Date::new(26, December, 2006),
            //
            Date::new(1, January, 2007),
            Date::new(6, April, 2007),
            Date::new(9, April, 2007),
            Date::new(7, May, 2007),
            Date::new(28, May, 2007),
            Date::new(27, August, 2007),
            Date::new(25, December, 2007),
            Date::new(26, December, 2007),
        ];

        let c = UnitedKingdom::metals();

        let hol = c.holiday_list(
            Date::new(1, January, 2004),
            Date::new(31, December, 2007),
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
