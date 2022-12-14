#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DateGenerationRule {
    /// Backward from termination date to effective date
    Backward,
    /// Forward from effective date to termination date
    Forward,
    /// No intermediate dates between effective date and termination date.
    Zero,
    /// All dates but effective date and termination date are taken to be on the third wednesday
    /// of their month (with forward calculation.)
    ThirdWednesday,
    /// All dates including effective date and termination date are taken to be on the third
    /// wednesday of their month (with forward calculation.)
    ThirdWednesdayInclusive,
    /// All dates but the effective date are taken to be the twentieth of their month
    /// (used for CDS schedules in emerging markets.)  The termination date is also modified.
    Twentieth,
    /// All dates but the effective date are taken to be the twentieth of an IMM month
    /// (used for CDS schedules.)  The termination date is also modified.
    TwentiethIMM,
    /// Same as TwentiethIMM with unrestricted date ends and log/short stub coupon period (old
    /// CDS convention).
    OldCDS,
    /// Credit derivatives standard rule since 'Big Bang' changes in 2009.
    CDS,
    /// Credit derivatives standard rule since December 20th, 2015.
    CDS2015,
}
