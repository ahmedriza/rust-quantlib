use crate::types::{Integer, Real};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frequency {
    NoFrequency = -1,
    /// only once, e.g., a zero-coupon
    Once = 0,
    /// once a year
    Annual = 1,
    /// twice a year
    Semiannual = 2,
    /// every fourth month
    EveryFourthMonth = 3,
    /// every third month
    Quarterly = 4,
    /// every second month
    Bimonthly = 6,
    /// once a month
    Monthly = 12,
    /// every fourth week
    EveryFourthWeek = 13,
    /// every second week
    Biweekly = 26,
    /// once a week
    Weekly = 52,
    /// once a day
    Daily = 365,
    /// some other unknown frequency    
    OtherFrequency = 999,
}

impl From<Integer> for Frequency {
    fn from(n: Integer) -> Self {
        match n {
            1 => Self::Annual,
            2 => Self::Semiannual,
            3 => Self::EveryFourthMonth,
            4 => Self::Quarterly,
            6 => Self::Bimonthly,
            12 => Self::Monthly,
            13 => Self::EveryFourthWeek,
            26 => Self::Biweekly,
            52 => Self::Weekly,
            365 => Self::Daily,
            other => panic!("Invalid frequency: {}", other),
        }
    }
}

impl From<Frequency> for Integer {
    fn from(f: Frequency) -> Self {
        f as Integer
    }
}

impl From<Frequency> for Real {
    fn from(f: Frequency) -> Self {
        (f as Integer) as Real
    }
}
