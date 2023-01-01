#![allow(non_upper_case_globals)]
use crate::types::{Integer, Natural, Size};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

pub const Jan: Month = Month::January;
pub const Feb: Month = Month::February;
pub const Mar: Month = Month::March;
pub const Apr: Month = Month::April;
pub const Jun: Month = Month::June;
pub const Jul: Month = Month::July;
pub const Aug: Month = Month::August;
pub const Sep: Month = Month::September;
pub const Oct: Month = Month::October;
pub const Nov: Month = Month::November;
pub const Dec: Month = Month::December;

impl From<Integer> for Month {
    fn from(n: Integer) -> Self {
        from_integral(n)
    }
}

impl From<Natural> for Month {
    fn from(n: Natural) -> Self {
        from_integral(n as Integer)
    }
}

impl From<Month> for Integer {
    fn from(m: Month) -> Self {
        m as Integer
    }
}

impl From<Month> for Natural {
    fn from(m: Month) -> Self {
        m as Natural
    }
}

impl From<Month> for Size {
    fn from(m: Month) -> Self {
        m as Size
    }
}

fn from_integral(n: Integer) -> Month {
    match n {
        1 => Month::January,
        2 => Month::February,
        3 => Month::March,
        4 => Month::April,
        5 => Month::May,
        6 => Month::June,
        7 => Month::July,
        8 => Month::August,
        9 => Month::September,
        10 => Month::October,
        11 => Month::November,
        12 => Month::December,
        other => panic!("Invalid month number {}", other),
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::Month::{self, *};

    use crate::types::{Integer, Natural, Size};

    #[test]
    fn test_month_conversions() {
        let jan: Month = January;
        assert_eq!(jan, January);

        // Check that we can convert the month to the corresponding integral type
        assert_eq!(Integer::from(jan), 1);
        assert_eq!(Natural::from(jan), 1);

        let _t: Size = jan.into();
        assert_eq!(_t, 1);
    }
}
