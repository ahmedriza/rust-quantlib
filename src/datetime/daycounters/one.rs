use crate::{
    datetime::date::Date,
    types::{Integer, Time},
};

/// 1/1 day count convention
#[derive(Clone, Copy, Default)]
pub struct One {}

impl One {
    pub fn new() -> Self {
        One {}
    }

    pub fn name(&self) -> String {
        "1/1".into()
    }

    pub fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        if d2 >= d1 {
            1
        } else {
            -1
        }
    }

    pub fn year_fraction(
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
    use crate::datetime::date::Date;
    use crate::datetime::months::Month::*;
    use crate::datetime::period::Period;
    use crate::datetime::timeunit::TimeUnit::*;
    use crate::types::Time;

    use super::One;

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

        let dc = One::new();

        let mut start = first;
        while start <= last {
            for i in 0..periods.len() {
                let p = &periods[i];
                let end = &start + p;
                let calculated = dc.year_fraction(&start, &end, &Date::default(), &Date::default());
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
