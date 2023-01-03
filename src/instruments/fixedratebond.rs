use crate::{
    datetime::{
        businessdayconvention::BusinessDayConvention, calendar::Calendar, date::Date,
        daycounter::DayCounter, frequency::Frequency, period::Period, schedule::Schedule,
    },
    types::{Integer, Real},
};

pub struct FixedRateBond {
    pub settlement_days: Integer,
    pub calendar: Calendar,
    pub maturity_date: Date,
    pub issue_date: Date,

    pub frequency: Frequency,
    pub daycounter: DayCounter,
    pub first_period_daycounter: Option<DayCounter>,
}

impl FixedRateBond {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        settlement_days: Integer,
        face_amount: Real,
        schedule: Schedule,
        coupons: Vec<Real>,
        accrual_daycounter: DayCounter,
        payment_convention: BusinessDayConvention,
        redemption: Option<Real>,                            // 100.0
        issue_date: Option<Date>,                            // default,
        payment_calendar: Option<Calendar>,                  // None,
        ex_coupon_period: Period,                            // default
        ex_coupon_calendar: Option<Calendar>,                // None,
        ex_coupon_convention: Option<BusinessDayConvention>, // Unadjusted
        ex_coupon_end_of_month: Option<bool>,                // false
        first_period_daycounter: Option<DayCounter>,         // None
    ) -> Self {
        let calendar = payment_calendar.unwrap_or_else(|| schedule.calendar().clone());
        Self {
            settlement_days,
            calendar,
            maturity_date: *schedule.end_date(),
            issue_date: issue_date.unwrap_or_default(),
            frequency: schedule.tenor().frequency(),
            daycounter: accrual_daycounter,
            first_period_daycounter,
        }
    }
}
