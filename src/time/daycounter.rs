use std::{fmt::Debug, sync::Arc};

use crate::types::{Integer, Time};

use crate::time::date::Date;

pub trait DayCounterDetail {
    /// Returns the name of the day counter.            
    fn name(&self) -> String;

    /// Returns the number of days between two dates.
    fn day_count(&self, d1: &Date, d2: &Date) -> Integer;

    /// Returns the period between two dates as a fraction of year
    fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        ref_period_start: &Date,
        ref_period_end: &Date,
    ) -> Time;
}

/// Day counter provides methods for determining the length of a time period according to given
/// market convention, both as a number of days and as a year fraction.
#[derive(Clone)]
pub struct DayCounter {
    detail: Arc<dyn DayCounterDetail>,
}

impl Debug for DayCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl PartialEq for DayCounter {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl DayCounter {
    pub fn new(detail: Arc<dyn DayCounterDetail>) -> Self {
        Self { detail }
    }

    /// This method is used for output and comparison between day counters. It is **not** meant
    /// to be used for writing switch-on-type code.
    ///    
    /// Returns the name of the day counter.        
    pub fn name(&self) -> String {
        self.detail.name()
    }

    /// Returns the number of days between two dates.
    pub fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        self.detail.day_count(d1, d2)
    }

    /// Returns the period between two dates as a fraction of year    
    pub fn year_fraction(&self, d1: &Date, d2: &Date) -> Time {
        self.year_fraction_with_start_end(d1, d2, &Date::default(), &Date::default())
    }

    /// Returns the period between two dates as a fraction of year    
    pub fn year_fraction_with_start_end(
        &self,
        d1: &Date,
        d2: &Date,
        ref_period_start: &Date,
        ref_period_end: &Date,
    ) -> Time {
        self.detail
            .year_fraction(d1, d2, ref_period_start, ref_period_end)
    }
}
