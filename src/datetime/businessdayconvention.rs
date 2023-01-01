/// Business Day conventions.
/// These conventions specify the algorithm used to adjust a date in case it is not a valid
/// business day.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BusinessDayConvention {
    /// ISDA
    /// Choose the first business day after the given holiday.
    Following,
    /// Choose the first business day after the given holiday unless it belongs to a different
    /// month, in which case choose the first business day before the holiday.
    ModifiedFollowing,
    /// Choose the first business day before the given holiday.
    /// NON ISDA
    Preceding,
    /// Choose the first business day before the given holiday unless it belongs to a different
    /// month, in which case choose the first business day after the holiday.
    ModifiedPreceding,
    /// Do not adjust.
    Unadjusted,
    /// Choose the first business day after the given holiday unless that day crosses the mid-month
    /// (15th) or the end of month, in which case choose the first business day before the holiday.
    HalfMonthModifiedFollowing,
    /// Choose the nearest business day to the given holiday. If both the preceding and following
    /// business days are equally far away, default to following business day.
    Nearest,
}
