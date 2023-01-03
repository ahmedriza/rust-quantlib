use crate::{
    datetime::{
        businessdayconvention::BusinessDayConvention, calendar::Calendar, date::Date,
        daycounter::DayCounter, frequency::Frequency, period::Period, schedule::Schedule,
    },
    types::{Integer, Real},
};

pub struct FixedRateBond {
    pub frequency: Frequency,
    pub daycounter: DayCounter,
    pub first_period_daycounter: DayCounter,
}

impl FixedRateBond {
    #[allow(unused)]
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
        first_period_day_counter: Option<DayCounter>,
    ) -> Self {
        todo!()
    }
}
