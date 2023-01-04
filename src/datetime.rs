use crate::types::{Integer, Natural, Time};

pub mod businessdayconvention;
pub mod calendar;
pub mod date;
pub mod dategenerationrule;
pub mod daycounter;
pub mod daycounters;
pub mod frequency;
pub mod holiday;
pub mod holidays;
pub mod imm;
pub mod months;
pub mod period;
pub mod schedule;
pub mod schedulebuilder;
pub mod timeunit;
pub mod weekday;
pub mod weekend;

pub type Day = Natural;

/// SerialNumber type represents the type of the date serial number as used by
/// Excel, LibreOffice Calc etc. In practice, this is actually a non-negative value, but
/// we have defined this as an [Integer], because, some of the internal date related calculations
/// require this to be signed.
pub type SerialNumber = Integer;

pub type Year = Integer;

pub fn time_to_days(t: Time) -> Integer {
    time_to_days_with_days_per_year(t, 360)
}

pub fn time_to_days_with_days_per_year(t: Time, days_per_year: Integer) -> Integer {
    (t * days_per_year as Time).round() as Integer
}
