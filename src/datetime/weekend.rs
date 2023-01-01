use super::weekday::Weekday::{self, *};

/// Type of weekend
#[derive(Debug, Clone, Copy)]
pub enum Weekend {
    /// Western weekend
    WesternWeekend(WesternWeekend),
    /// Orthodox weekend
    OrthodoxWeekend(OrthodoxWeekend),
    /// No weekends
    NilWeekend(NilWeekend),
}

impl Weekend {
    pub fn is_weekend(&self, weekday: Weekday) -> bool {
        match self {
            Weekend::WesternWeekend(w) => w.is_weekend(weekday),
            Weekend::OrthodoxWeekend(w) => w.is_weekend(weekday),
            Weekend::NilWeekend(w) => w.is_weekend(weekday),
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct WesternWeekend {}

impl WesternWeekend {
    pub fn is_weekend(&self, weekday: Weekday) -> bool {
        weekday == Saturday || weekday == Sunday
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct OrthodoxWeekend {}

impl OrthodoxWeekend {
    pub fn is_weekend(&self, weekday: Weekday) -> bool {
        weekday == Saturday || weekday == Sunday
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct NilWeekend {}

impl NilWeekend {
    pub fn is_weekend(&self, _weekday: Weekday) -> bool {
        false
    }
}
