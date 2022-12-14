use std::sync::Arc;

use crate::time::date::Date;
use crate::types::{Integer, Time};

use crate::time::daycounter::{DayCounter, DayCounterDetail};

/// Actual/366 day count convention, also known as "Act/366".
pub struct Actual366 {
    include_last_day: bool,
}

impl Actual366 {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> DayCounter {
        Actual366::with_last_day(false)
    }

    pub fn with_last_day(include_last_day: bool) -> DayCounter {
        DayCounter::new(Arc::new(Actual366 { include_last_day }))
    }
}

impl DayCounterDetail for Actual366 {
    fn name(&self) -> String {
        if self.include_last_day {
            "Actual/366 (inc)".into()
        } else {
            "Actual/366".into()
        }
    }

    fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        if self.include_last_day {
            (d2 - d1) + 1
        } else {
            d2 - d1
        }
    }

    fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        _ref_period_start: &Date,
        _ref_period_end: &Date,
    ) -> Time {
        self.day_count(d1, d2) as Time / 366.0
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::time::date::Date;
    use crate::time::months::Month::*;

    use super::Actual366;

    #[test]
    fn test_actual366() {
        let test_dates = vec![
            Date::new(1, February, 2002),
            Date::new(4, February, 2002),
            Date::new(16, May, 2003),
            Date::new(17, December, 2003),
            Date::new(17, December, 2004),
            Date::new(19, December, 2005),
            Date::new(2, January, 2006),
            Date::new(13, March, 2006),
            Date::new(15, May, 2006),
            Date::new(17, March, 2006),
            Date::new(15, May, 2006),
            Date::new(26, July, 2006),
            Date::new(28, June, 2007),
            Date::new(16, September, 2009),
            Date::new(26, July, 2016),
        ];

        let expected = vec![
            0.00819672131147541,
            1.27322404371585,
            0.587431693989071,
            1.0000000000000,
            1.00273224043716,
            0.0382513661202186,
            0.191256830601093,
            0.172131147540984,
            -0.16120218579235,
            0.16120218579235,
            0.19672131147541,
            0.920765027322404,
            2.21584699453552,
            6.84426229508197,
        ];

        let dc = Actual366::new();

        for i in 1..test_dates.len() {
            let calculated = dc.year_fraction(&test_dates[i - 1], &test_dates[i]);

            let diff = (calculated - expected[i - 1]).abs();
            assert!(
                diff < 1.0e-12,
                "from {:?} to {:?}: calculated: {}, expected: {}",
                test_dates[i - 1],
                test_dates[i],
                calculated,
                expected[i - 1]
            );
        }
    }
}
