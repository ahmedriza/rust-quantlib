use std::sync::Arc;

use crate::datetime::{
    calendar::{easter_monday, Calendar, Holiday, Weekend, WesternWeekend},
    date::Date,
    months::Month,
    weekday::Weekday,
};

/// TARGET calendar
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
#[derive(Clone)]
pub struct Target {
    pub weekend: Arc<dyn Weekend>,
}

impl Target {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Calendar {
        Calendar::new(Arc::new(Target {
            weekend: Arc::new(WesternWeekend {}),
        }))
    }
}

impl Holiday for Target {
    fn name(&self) -> String {
        "TARGET".into()
    }

    fn is_business_day(&self, date: &Date) -> bool {
        let weekday = date.weekday();
        let day_of_month = date.day_of_month();
        let day_of_year = date.day_of_year();
        let month = date.month();
        let year = date.year();
        let easter_monday = easter_monday(year);

        if self.is_weekend(weekday)
            // New Year's day 
            || (day_of_month == 1 && month == Month::January)
            // Good Friday 
            || (day_of_year == easter_monday - 3 && year >= 2000)
            // Easter Monday
            || (day_of_year == easter_monday && year >= 2000)
            // Labour Day 
            || (day_of_month == 1 && month == Month::May && year >= 2000)
            // Christmas 
            || (day_of_month == 25 && month == Month::December)
            // Day of Goodwill 
            || (day_of_month == 26 && month == Month::December && year >= 2000)
            // December 31st, 1998, 1999, and 2001 only
            || (day_of_month == 31 && month == Month::December &&
                (year == 1998 || year == 1999 || year == 2001))
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
    use crate::datetime::{calendar::Calendar, date::Date, months::Month};

    use super::Target;

    #[test]
    fn test_target() {
        let expected_hol = vec![
            Date::new(1, Month::January, 1999),
            Date::new(31, Month::December, 1999),
            //
            Date::new(21, Month::April, 2000),
            Date::new(24, Month::April, 2000),
            Date::new(1, Month::May, 2000),
            Date::new(25, Month::December, 2000),
            Date::new(26, Month::December, 2000),
            //
            Date::new(1, Month::January, 2001),
            Date::new(13, Month::April, 2001),
            Date::new(16, Month::April, 2001),
            Date::new(1, Month::May, 2001),
            Date::new(25, Month::December, 2001),
            Date::new(26, Month::December, 2001),
            Date::new(31, Month::December, 2001),
            //
            Date::new(1, Month::January, 2002),
            Date::new(29, Month::March, 2002),
            Date::new(1, Month::April, 2002),
            Date::new(1, Month::May, 2002),
            Date::new(25, Month::December, 2002),
            Date::new(26, Month::December, 2002),
            //
            Date::new(1, Month::January, 2003),
            Date::new(18, Month::April, 2003),
            Date::new(21, Month::April, 2003),
            Date::new(1, Month::May, 2003),
            Date::new(25, Month::December, 2003),
            Date::new(26, Month::December, 2003),
            //
            Date::new(1, Month::January, 2004),
            Date::new(9, Month::April, 2004),
            Date::new(12, Month::April, 2004),
            //
            Date::new(25, Month::March, 2005),
            Date::new(28, Month::March, 2005),
            Date::new(26, Month::December, 2005),
            //
            Date::new(14, Month::April, 2006),
            Date::new(17, Month::April, 2006),
            Date::new(1, Month::May, 2006),
            Date::new(25, Month::December, 2006),
            Date::new(26, Month::December, 2006),
        ];

        let cal: Calendar = Target::new();

        let holiday_list = cal.holiday_list(
            Date::new(1, Month::January, 1999),
            Date::new(31, Month::December, 2006),
            false,
        );

        let min_len = holiday_list.len().min(expected_hol.len());
        for i in 0..min_len {
            assert!(
                holiday_list[i] == expected_hol[i],
                "expected holiday was {:?}, while calculated holiday is {:?}",
                expected_hol[i],
                holiday_list[i]
            );
        }
        assert!(
            holiday_list.len() == expected_hol.len(),
            "there were {} expected holidays, while there are {} calculated holidays",
            expected_hol.len(),
            holiday_list.len()
        );
    }
}
