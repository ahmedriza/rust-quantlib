use crate::{
    datetime::{date::Date, daycounter::DayCounter, frequency::Frequency::Annual},
    rates::{compounding::Compounding::Simple, interestrate::InterestRate},
    types::{Rate, Real},
};

use super::{cashflow::CashFlow, coupon::Coupon};

/// Coupon paying a fixed interest rate
pub struct FixedRateCoupon {
    pub payment_date: Date,
    pub nominal: Real,
    pub accrual_start_date: Date,
    pub accrual_end_date: Date,
    pub ref_period_start: Date,
    pub ref_period_end: Date,
    pub ex_coupon_date: Date,
    pub rate: InterestRate,
}

impl FixedRateCoupon {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        payment_date: Date,
        nominal: Real,
        rate: Rate,
        daycounter: DayCounter,
        accrual_start_date: Date,
        accrual_end_date: Date,
        ref_period_start: Option<Date>,
        ref_period_end: Option<Date>,
        ex_coupon_date: Option<Date>,
    ) -> Self {
        let interest_rate = InterestRate::new(rate, daycounter, Simple, Annual);
        Self {
            payment_date,
            nominal,
            accrual_start_date,
            accrual_end_date,
            ref_period_start: ref_period_start.unwrap_or_default(),
            ref_period_end: ref_period_end.unwrap_or_default(),
            ex_coupon_date: ex_coupon_date.unwrap_or_default(),
            rate: interest_rate,
        }
    }
}

impl CashFlow for FixedRateCoupon {
    fn accurued_amount(&self, date: Date) -> Real {
        if date <= self.accrual_start_date || date > self.payment_date {
            // out of coupon range
            0.0
        } else if self.trading_ex_coupon(date) {
            let compound_factor = self.rate.compound_factor_between_dates(
                &date,
                &date.max(self.accrual_end_date),
                &self.ref_period_start,
                &self.ref_period_end,
            );
            self.nominal * (compound_factor - 1.0)
        } else {
            // usual case
            let compound_factor = self.rate.compound_factor_between_dates(
                &self.accrual_start_date,
                &date.min(self.accrual_end_date),
                &self.ref_period_start,
                &self.ref_period_end,
            );
            self.nominal * (compound_factor - 1.0)
        }
    }

    fn amount(&self) -> Real {
        let compound_factor = self.rate.compound_factor_between_dates(
            &self.accrual_start_date,
            &self.accrual_end_date,
            &self.ref_period_start,
            &self.ref_period_end,
        );
        self.nominal * (compound_factor - 1.0)
    }

    fn date(&self) -> Date {
        self.payment_date
    }

    fn ex_coupon_date(&self) -> Date {
        self.ex_coupon_date
    }
}

impl Coupon for FixedRateCoupon {
    fn day_counter(&self) -> &DayCounter {
        &self.rate.daycounter
    }

    fn nominal(&self) -> Real {
        self.nominal
    }

    fn accrual_start_date(&self) -> Date {
        self.accrual_start_date
    }

    fn accrual_end_date(&self) -> Date {
        self.accrual_end_date
    }

    fn rate(&self) -> Rate {
        self.rate.rate
    }

    fn reference_period_start(&self) -> Date {
        self.ref_period_start
    }

    fn reference_period_end(&self) -> Date {
        self.ref_period_end
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use crate::{
        cashflows::cashflow::Leg,
        datetime::{date::Date, daycounter::DayCounter, months::Month::*},
    };

    use super::FixedRateCoupon;

    #[test]
    fn test_fixed_rate_coupon() {
        let mut leg: Leg = Leg::new();
        let payment_date = Date::new(3, January, 2023);
        let nominal = 100.0;
        let rate = 0.01;
        let daycounter = DayCounter::usa();

        let accrual_start_date = payment_date + 1;
        let accrual_end_date = payment_date + 10;

        leg.push(Rc::new(FixedRateCoupon::new(
            payment_date,
            nominal,
            rate,
            daycounter,
            accrual_start_date,
            accrual_end_date,
            None,
            None,
            None,
        )));
    }
}