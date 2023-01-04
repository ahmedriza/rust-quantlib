use std::rc::Rc;

use crate::{
    datetime::{
        businessdayconvention::BusinessDayConvention::{self, *},
        calendar::Calendar,
        date::Date,
        daycounter::DayCounter,
        period::Period,
        schedule::Schedule,
        timeunit::TimeUnit::Days,
    },
    rates::interestrate::InterestRate,
    types::{Integer, Real},
};

use super::{cashflow::Leg, fixedratecoupon::FixedRateCoupon};

/// Helper for building a sequence of [FixedRateCoupon] instances
pub struct FixedRateCouponBuilder {
    pub schedule: Schedule,
    pub notionals: Vec<Real>,
    pub coupon_rates: Vec<InterestRate>,
    pub first_period_dc: Option<DayCounter>,
    pub last_period_dc: Option<DayCounter>,
    pub payment_calendar: Option<Calendar>,
    pub payment_adjustment: Option<BusinessDayConvention>, // Following
    pub payment_lag: Option<Integer>,                      // 0
    pub ex_coupon_period: Option<Period>,
    pub ex_coupon_calendar: Option<Calendar>,
    pub ex_coupon_adjustment: Option<BusinessDayConvention>, // Following
    pub ex_coupon_end_of_month: Option<bool>,                // false
}

impl FixedRateCouponBuilder {
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

    pub fn with_payment_lag(mut self, lag: Integer) -> Self {
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
        let mut leg = Leg::new();

        let payment_calendar = self
            .payment_calendar
            .as_ref()
            .unwrap_or_else(|| self.schedule.calendar());
        let payment_adjustment = self.payment_adjustment.unwrap_or(Following);
        let payment_lag = self.payment_lag.unwrap_or(0);

        // first period might be short or long
        let start = self.schedule[0];
        let end = self.schedule[1];

        let payment_date =
            payment_calendar.advance_by_days(end, payment_lag, Days, payment_adjustment, false);

        let interest_rate = &self.coupon_rates[0];
        let nominal = self.notionals[0];
        let ex_coupon_date = self.make_ex_coupon_date(payment_date);
        let ref_date = if self.schedule.has_is_regular() && !self.schedule.is_regular(1) {
            self.schedule.calendar().advance_by_period(
                end,
                -self.schedule.tenor(),
                self.schedule.business_day_convention(),
                self.schedule.end_of_month(),
            )
        } else {
            start
        };
        let first_dc = if self.first_period_dc.is_none() {
            &interest_rate.daycounter
        } else {
            self.first_period_dc.as_ref().unwrap()
        };
        let r = InterestRate::new(
            interest_rate.rate,
            first_dc.clone(),
            interest_rate.compounding.clone(),
            interest_rate.frequency(),
        );
        leg.push(Rc::new(FixedRateCoupon::with_interest_rate(
            payment_date,
            nominal,
            r,
            start,
            end,
            Some(ref_date),
            Some(end),
            Some(ex_coupon_date),
        )));

        // regular periods
        for i in 2..self.schedule.size() - 1 {
            let start = end;
            let end = self.schedule[i];
            let payment_date =
                payment_calendar.advance_by_days(end, payment_lag, Days, payment_adjustment, false);
            let ex_coupon_date = self.make_ex_coupon_date(payment_date);
            let rate = if (i - 1) < self.coupon_rates.len() {
                &self.coupon_rates[i - 1]
            } else {
                &self.coupon_rates[self.coupon_rates.len() - 1]
            };
            let nominal = if (i - 1) < self.notionals.len() {
                self.notionals[i - 1]
            } else {
                self.notionals[self.notionals.len() - 1]
            };
            leg.push(Rc::new(FixedRateCoupon::with_interest_rate(
                payment_date,
                nominal,
                rate.clone(),
                start,
                end,
                Some(start),
                Some(end),
                Some(ex_coupon_date),
            )))
        }

        if self.schedule.size() > 2 {
            // last period might be short or long
            let n = self.schedule.size();
            let start = end;
            let end = self.schedule[n - 1];
            let payment_date =
                payment_calendar.advance_by_days(end, payment_lag, Days, payment_adjustment, false);
            let ex_coupon_date = self.make_ex_coupon_date(payment_date);

            let interest_rate = if (n - 2) < self.coupon_rates.len() {
                &self.coupon_rates[n - 2]
            } else {
                &self.coupon_rates[self.coupon_rates.len() - 1]
            };
            let nominal = if (n - 2) < self.notionals.len() {
                self.notionals[n - 2]
            } else {
                self.notionals[self.notionals.len() - 1]
            };
            let last_dc = if self.last_period_dc.is_none() {
                &interest_rate.daycounter
            } else {
                self.last_period_dc.as_ref().unwrap()
            };
            let r = InterestRate::new(
                interest_rate.rate,
                last_dc.clone(),
                interest_rate.compounding.clone(),
                interest_rate.frequency(),
            );
            if self.schedule.has_is_regular() && self.schedule.is_regular(n - 1) {
                leg.push(Rc::new(FixedRateCoupon::with_interest_rate(
                    payment_date,
                    nominal,
                    r,
                    start,
                    end,
                    Some(start),
                    Some(end),
                    Some(ex_coupon_date),
                )))
            } else {
                let ref_date = self.schedule.calendar().advance_by_period(
                    start,
                    self.schedule.tenor(),
                    self.schedule.business_day_convention(),
                    self.schedule.end_of_month(),
                );
                leg.push(Rc::new(FixedRateCoupon::with_interest_rate(
                    payment_date,
                    nominal,
                    r,
                    start,
                    end,
                    Some(ref_date),
                    Some(end),
                    Some(ex_coupon_date),
                )))
            }
        }
        leg
    }

    fn make_ex_coupon_date(&self, payment_date: Date) -> Date {
        if self.ex_coupon_period.is_some() {
            let ex_coupon_period = self.ex_coupon_period.unwrap();
            let ex_coupon_adjustment = self.ex_coupon_adjustment.unwrap_or_else(|| {
                panic!(
                    "ex-coupon period is {:?}, but ex-coupon adjustment has not been set",
                    self.ex_coupon_period.unwrap()
                )
            });
            let ex_coupon_end_of_month = self.ex_coupon_end_of_month.unwrap_or_else(|| {
                panic!(
                    "ex-coupon period is {:?}, but ex-coupon end of month has not been set",
                    self.ex_coupon_period.unwrap()
                )
            });
            let ex_coupon_calendar = self.ex_coupon_calendar.as_ref().unwrap_or_else(|| {
                panic!(
                    "ex-coupon period is {:?}, but ex-coupon calendar has not been set",
                    self.ex_coupon_period.unwrap()
                )
            });
            ex_coupon_calendar.advance_by_period(
                payment_date,
                -ex_coupon_period,
                ex_coupon_adjustment,
                ex_coupon_end_of_month,
            )
        } else {
            Date::default()
        }
    }
}
