use crate::time::date::Date;
use crate::types::{Natural, Time};

/// Basic term structure functionality
///
/// There are three ways in which a term structure can keep track of its reference date.
/// * The first is that such date is fixed
/// * The second is that it is determined by advancing the current date by a given number of
///   business days
/// * The third is that it is based on the reference date of some other structure.
///
pub trait TermStructure {
    /// Date/Time conversion
    fn time_from_references(&self, date: &Date) -> Time;

    /// The latest date for which the curve can return values
    fn max_date(&self) -> Date;

    /// The latest time for which the curve can return values
    fn max_time(&self) -> Time;

    /// The date at which discount = 1.0 and/or variance = 0.0
    fn reference_date(&self) -> Date;

    /// The settlementDays used for reference date calculation
    fn settlement_days(&self) -> Natural;
}
