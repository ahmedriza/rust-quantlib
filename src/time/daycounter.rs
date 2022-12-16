use std::fmt::Debug;

use crate::types::{Integer, Time};

use super::{
    date::Date,
    daycounters::{
        actual360::Actual360,
        actual366::Actual366,
        one::One,
        simple::Simple,
        thirty360::{Thirty360, Thiry360Convention, EU, ISDA, ISMA, IT, NASD, US},
        thirty365::Thirty365,
    },
};

/// Day count conventions
#[derive(Clone, Copy)]
pub enum DayCounter {
    /// Actual/360 day count convention, also known as "Act/360", or "A/360".
    Actual360(Actual360),
    /// Actual/366 day count convention, also known as "Act/366".
    Actual366(Actual366),
    /// 1/1 day count convention
    One(One),
    /// Simple day counter for reproducing theoretical calculations.
    Simple(Simple),
    /// 30/360 day count convention
    Thirty360(Thirty360),
    /// 30/365 day count convention
    Thirty365(Thirty365),
}

impl Debug for DayCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Actual360(dc) => write!(f, "{}", dc.name()),
            Self::Actual366(dc) => write!(f, "{}", dc.name()),
            Self::One(dc) => write!(f, "{}", dc.name()),
            Self::Simple(dc) => write!(f, "{}", dc.name()),
            Self::Thirty360(dc) => write!(f, "{}", dc.name()),
            Self::Thirty365(dc) => write!(f, "{}", dc.name()),
        }
    }
}

impl PartialEq for DayCounter {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Actual360(l0), Self::Actual360(r0)) => l0.name() == r0.name(),
            (Self::Actual366(l0), Self::Actual366(r0)) => l0.name() == r0.name(),
            (Self::One(l0), Self::One(r0)) => l0.name() == r0.name(),
            (Self::Simple(l0), Self::Simple(r0)) => l0.name() == r0.name(),
            (Self::Thirty360(l0), Self::Thirty360(r0)) => l0.name() == r0.name(),
            (Self::Thirty365(l0), Self::Thirty365(r0)) => l0.name() == r0.name(),
            _ => false,
        }
    }
}

impl DayCounter {
    /// Return the name of the day counter
    pub fn name(&self) -> String {
        match self {
            DayCounter::Actual360(dc) => dc.name(),
            DayCounter::Actual366(dc) => dc.name(),
            DayCounter::One(dc) => dc.name(),
            DayCounter::Simple(dc) => dc.name(),
            DayCounter::Thirty360(dc) => dc.name(),
            DayCounter::Thirty365(dc) => dc.name(),
        }
    }

    /// Returns the number of days between two dates.
    pub fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        match self {
            DayCounter::Actual360(dc) => dc.day_count(d1, d2),
            DayCounter::Actual366(dc) => dc.day_count(d1, d2),
            DayCounter::One(dc) => dc.day_count(d1, d2),
            DayCounter::Simple(dc) => dc.day_count(d1, d2),
            DayCounter::Thirty360(dc) => dc.day_count(d1, d2),
            DayCounter::Thirty365(dc) => dc.day_count(d1, d2),
        }
    }

    /// Returns the period between two dates as a fraction of year    
    pub fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        ref_period_start: &Date,
        ref_period_end: &Date,
    ) -> Time {
        match self {
            DayCounter::Actual360(dc) => dc.year_fraction(d1, d2, ref_period_start, ref_period_end),
            DayCounter::Actual366(dc) => dc.year_fraction(d1, d2, ref_period_start, ref_period_end),
            DayCounter::One(dc) => dc.year_fraction(d1, d2, ref_period_start, ref_period_end),
            DayCounter::Simple(dc) => dc.year_fraction(d1, d2, ref_period_start, ref_period_end),
            DayCounter::Thirty360(dc) => dc.year_fraction(d1, d2, ref_period_start, ref_period_end),
            DayCounter::Thirty365(dc) => dc.year_fraction(d1, d2, ref_period_start, ref_period_end),
        }
    }

    /// Return an instance of an [Actual360] day counter
    pub fn actual360() -> DayCounter {
        DayCounter::Actual360(Actual360::new())
    }

    /// Return an instance of a [Simple] day counter
    pub fn simple() -> DayCounter {
        DayCounter::Simple(Simple::new())
    }

    /// Return an instance of a [Thirty360] day counter with US conventions
    pub fn usa() -> DayCounter {
        DayCounter::Thirty360(Thirty360 {
            convention: Thiry360Convention::US(US {}),
        })
    }

    /// Return an instance of a [Thirty360] day counter with ISMA conventions
    pub fn isma() -> DayCounter {
        DayCounter::Thirty360(Thirty360 {
            convention: Thiry360Convention::ISMA(ISMA {}),
        })
    }

    /// Return an instance of a [Thirty360] day counter with ISMA conventions
    pub fn bond_basis() -> DayCounter {
        DayCounter::isma()
    }

    /// Return an instance of a [Thirty360] day counter with EU conventions
    pub fn european() -> DayCounter {
        DayCounter::Thirty360(Thirty360 {
            convention: Thiry360Convention::EU(EU {}),
        })
    }

    /// Return an instance of a [Thirty360] day counter with EU conventions
    pub fn euro_bond_basis() -> DayCounter {
        DayCounter::european()
    }

    /// Return an instance of a [Thirty360] day counter with Italian conventions
    pub fn italian() -> DayCounter {
        DayCounter::Thirty360(Thirty360 {
            convention: Thiry360Convention::IT(IT {}),
        })
    }

    /// Return an instance of a [Thirty360] day counter with ISDA conventions
    pub fn isda(termination_date: Date) -> DayCounter {
        DayCounter::Thirty360(Thirty360 {
            convention: Thiry360Convention::ISDA(ISDA { termination_date }),
        })
    }

    /// Return an instance of a [Thirty360] day counter with ISDA conventions
    pub fn german(termination_date: Date) -> DayCounter {
        DayCounter::isda(termination_date)
    }

    /// Return an instance of a [Thirty360] day counter with NASD conventions
    pub fn nasd() -> DayCounter {
        DayCounter::Thirty360(Thirty360 {
            convention: Thiry360Convention::NASD(NASD {}),
        })
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::{
        time::{date::Date, months::Month::*},
        types::Integer,
    };

    use super::DayCounter;

    #[test]
    pub fn test_thirty360() {
        let d1 = Date::new(1, January, 2022);
        let d2 = Date::new(30, June, 2022);
        let d3 = Date::new(1, December, 2022);
        let d4 = Date::new(31, December, 2022);
        let date_pairs = vec![(d1, d2), (d3, d4)];
        let expected = [179, 30];
        let usa = DayCounter::usa();
        for (i, dp) in date_pairs.iter().enumerate() {
            let count = day_count(&usa, &dp.0, &dp.1);
            assert_eq!(
                count, expected[i],
                "day count between {:?} and {:?} is not {}, but {}",
                d1, d2, expected[i], count
            );
        }
    }

    fn day_count(dc: &DayCounter, d1: &Date, d2: &Date) -> Integer {
        dc.day_count(d1, d2)
    }
}
