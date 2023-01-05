use std::rc::Rc;

use crate::{
    datetime::{date::Date, daycounter::DayCounter, SerialNumber},
    types::{Rate, Real, Time},
};

use super::cashflow::CashFlow;

/// Sequence of coupons
pub type CouponLeg = Vec<Rc<dyn Coupon>>;

/// Coupon accruing over a fixed period
pub trait Coupon: CashFlow {
    /// Accrual period in days
    fn accrual_days(&self) -> SerialNumber {
        self.day_counter()
            .day_count(&self.accrual_start_date(), &self.accrual_end_date())
    }

    /// Accrual period as fraction of year
    fn accrual_period(&self) -> Time {
        self.day_counter().year_fraction(
            &self.accrual_start_date(),
            &self.accrual_end_date(),
            &self.reference_period_start(),
            &self.reference_period_end(),
        )
    }

    /// Accrued days at the given date
    fn accrued_days(&self, date: Date) -> SerialNumber {
        if date <= self.accrual_start_date() || date > self.date() {
            0
        } else {
            self.day_counter().day_count(
                &self.accrual_start_date(),
                &date.min(self.accrual_end_date()),
            )
        }
    }

    /// Accrued period as fraction of year at the given date
    fn accrued_period(&self, date: Date) -> Time {
        if date <= self.accrual_start_date() || date > self.date() {
            0.0
        } else if self.trading_ex_coupon(date) {
            -self.day_counter().year_fraction(
                &date,
                &date.max(self.accrual_end_date()),
                &self.reference_period_start(),
                &self.reference_period_end(),
            )
        } else {
            self.day_counter().year_fraction(
                &self.accrual_start_date(),
                &date.min(self.accrual_end_date()),
                &self.reference_period_start(),
                &self.reference_period_end(),
            )
        }
    }

    /// Day counter for accrual calculation
    fn day_counter(&self) -> &DayCounter;

    /// Return the nominal
    fn nominal(&self) -> Real;

    /// Accrued rate
    fn rate(&self) -> Rate;
}

impl<T> Coupon for Rc<T>
where
    // Note that `?Sized` is needed here as otherwise the compiler will default to `Sized`
    // which will then forbid a type such as `Rc<dyn Coupon>`
    T: Coupon + ?Sized,
{
    fn day_counter(&self) -> &DayCounter {
        (**self).day_counter()
    }

    fn nominal(&self) -> Real {
        (**self).nominal()
    }

    fn rate(&self) -> Rate {
        (**self).rate()
    }
}
