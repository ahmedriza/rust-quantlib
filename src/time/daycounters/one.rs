use std::sync::Arc;

use crate::time::date::Date;
use crate::types::{Integer, Time};

use crate::time::daycounter::{DayCounter, DayCounterDetail};

/// 1/1 day count convention
pub struct OneDayCounter {}

impl OneDayCounter {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> DayCounter {
        DayCounter::new(Arc::new(OneDayCounter {}))
    }
}

impl DayCounterDetail for OneDayCounter {
    fn name(&self) -> String {
        "1/1".into()
    }

    fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        if d2 >= d1 {
            1
        } else {
            -1
        }
    }

    fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        _ref_period_start: &Date,
        _ref_period_end: &Date,
    ) -> Time {
        self.day_count(d1, d2).into()
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::time::date::Date;
    use crate::time::months::Month::*;
    use crate::time::period::Period;
    use crate::time::timeunit::TimeUnit::*;
    use crate::types::Time;

    use super::OneDayCounter;

    #[test]
    fn test_one_day_counter() {
        let periods = vec![
            Period::new(3, Months),
            Period::new(6, Months),
            Period::new(1, Years),
        ];
        let expected: Vec<Time> = vec![1.0, 1.0, 1.0];
        // 1 year should be enough
        let first = Date::new(1, January, 2004);
        let last = Date::new(31, December, 2004);

        let dc = OneDayCounter::new();

        let mut start = first;
        while start <= last {
            for i in 0..periods.len() {
                let p = &periods[i];
                let end = &start + p;
                let calculated = dc.year_fraction(&start, &end);
                let diff = (calculated - expected[i]).abs();
                assert!(
                    diff < 1.0e-12,
                    "from {:?} to {:?}: calculated: {}, expected: {}",
                    start,
                    end,
                    calculated,
                    expected[i]
                );
            }
            start += 1;
        }
    }
}
