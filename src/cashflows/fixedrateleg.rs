use crate::{
    datetime::{
        businessdayconvention::BusinessDayConvention, calendar::Calendar, daycounter::DayCounter,
        period::Period, schedule::Schedule,
    },
    rates::interestrate::InterestRate,
    types::{Natural, Real},
};

use super::cashflow::Leg;

/// Helper for building a sequence of [FixedRateCoupon] instances
pub struct FixedRateLeg {
    pub schedule: Schedule,
    pub notionals: Vec<Real>,
    pub coupon_rates: Vec<InterestRate>,
    pub first_period_dc: Option<DayCounter>,
    pub last_period_dc: Option<DayCounter>,
    pub payment_calendar: Option<Calendar>,
    pub payment_adjustment: Option<BusinessDayConvention>, // Following
    pub payment_lag: Option<Natural>,                      // 0
    pub ex_coupon_period: Option<Period>,
    pub ex_coupon_calendar: Option<Calendar>,
    pub ex_coupon_adjustment: Option<BusinessDayConvention>, // Following
    pub ex_coupon_end_of_month: Option<bool>,                // false
}

impl FixedRateLeg {
    /// Construct a [FixedRateLeg] from the mandatory parameters
    pub fn new(schedule: Schedule, notionals: Vec<Real>, coupon_rates: Vec<InterestRate>) -> Self {
        Self {
            schedule,
            notionals,
            coupon_rates,
            first_period_dc: None,
            last_period_dc: None,
            payment_calendar: None,
            payment_adjustment: None,
            payment_lag: None,
            ex_coupon_period: None,
            ex_coupon_calendar: None,
            ex_coupon_adjustment: None,
            ex_coupon_end_of_month: None,
        }
    }

    pub fn with_first_period_daycounter(mut self, daycounter: DayCounter) -> Self {
        self.first_period_dc = Some(daycounter);
        self
    }

    pub fn with_last_period_daycounter(mut self, daycounter: DayCounter) -> Self {
        self.last_period_dc = Some(daycounter);
        self
    }

    pub fn with_payment_calendar(mut self, calendar: Calendar) -> Self {
        self.payment_calendar = Some(calendar);
        self
    }

    pub fn with_payment_adjustment(mut self, convention: BusinessDayConvention) -> Self {
        self.payment_adjustment = Some(convention);
        self
    }

    pub fn with_payment_lag(mut self, lag: Natural) -> Self {
        self.payment_lag = Some(lag);
        self
    }

    pub fn with_ex_coupon_period(
        mut self,
        period: Period,
        calendar: Calendar,
        convention: BusinessDayConvention,
        end_of_month: bool,
    ) -> Self {
        self.ex_coupon_period = Some(period);
        self.ex_coupon_calendar = Some(calendar);
        self.ex_coupon_adjustment = Some(convention);
        self.ex_coupon_end_of_month = Some(end_of_month);
        self 
    }

    /// Build [Leg] of fixed rate coupons 
    pub fn build(self) -> Leg {
        todo!()
    }
}
