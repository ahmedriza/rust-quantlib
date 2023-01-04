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

    #[allow(clippy::too_many_arguments)]
    pub fn with_interest_rate(
        payment_date: Date,
        nominal: Real,
        interest_rate: InterestRate,
        accrual_start_date: Date,
        accrual_end_date: Date,
        ref_period_start: Option<Date>,
        ref_period_end: Option<Date>,
        ex_coupon_date: Option<Date>,
    ) -> Self {
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
    fn accrued_amount(&self, date: Date) -> Real {
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
    use crate::{
        cashflows::{cashflow, fixedratecouponbuilder::FixedRateCouponBuilder},
        context::pricing_context::PricingContext,
        datetime::{
            businessdayconvention::BusinessDayConvention,
            date::Date,
            daycounter::DayCounter,
            frequency::Frequency,
            holidays::{nilholiday::NilHoliday, target::Target},
            months::Month::*,
            period::Period,
            schedule::ScheduleBuilder,
            timeunit::TimeUnit::{Days, Months, Years},
        },
        rates::{compounding::Compounding, interestrate::InterestRate},
    };

    #[test]
    fn test_settlement_date_accruals() {
        let today = Date::new(4, January, 2023);
        let pricing_context = pricing_context(today);

        let from = today - Period::new(2, Months);
        let to = today + Period::new(4, Months);
        let schedule = ScheduleBuilder::new(
            pricing_context,
            from,
            to,
            Period::from(Frequency::Semiannual),
            Target::new(),
        )
        .with_convention(BusinessDayConvention::Unadjusted)
        .backwards()
        .build();

        let notionals = vec![100.0];
        let coupon_rates = vec![InterestRate::new(
            0.03,
            DayCounter::actual360(),
            Compounding::Simple,
            Frequency::Annual,
        )];
        let leg = FixedRateCouponBuilder::new(schedule, notionals, coupon_rates)
            .with_payment_calendar(Target::new())
            .with_payment_adjustment(BusinessDayConvention::Following)
            .build();

        let expected_accrued_amount = 0.5083333333333329;
        let expected_accrued_days = 61;
        let expected_accrued_period = 0.16944444444444445;

        let accrued_amount = cashflow::accurued_amount(&leg, false, pricing_context.eval_date);
        assert!(
            (expected_accrued_amount - accrued_amount).abs() < 1.0e-10,
            "Expected accrued amount: {}, but got: {}",
            expected_accrued_amount,
            accrued_amount
        );

        let accrued_days = cashflow::accurued_days(&leg, false, pricing_context.eval_date);
        assert_eq!(
            accrued_days, expected_accrued_days,
            "Expected accrued days: {}, but got: {}",
            expected_accrued_days, accrued_days
        );

        let accrued_period = cashflow::accrued_period(&leg, false, pricing_context.eval_date);
        assert!(
            (expected_accrued_period - accrued_period).abs() < 1.0e-10,
            "Expected accrued period: {}, but got: {}",
            expected_accrued_period,
            accrued_period
        );
    }

    #[test]
    fn test_ex_coupon_dates() {
        let today = Date::new(4, January, 2023);
        let pricing_context = pricing_context(today);

        let from = today;
        let to = today + Period::new(5, Years);
        let schedule = ScheduleBuilder::new(
            pricing_context,
            from,
            to,
            Period::from(Frequency::Monthly),
            Target::new(),
        )
        .with_convention(BusinessDayConvention::Following)
        .build();

        let notionals = vec![100.0];
        let coupon_rates = vec![InterestRate::new(
            0.03,
            DayCounter::actual360(),
            Compounding::Simple,
            Frequency::Annual,
        )];
        // no ex-coupon dates
        let leg =
            FixedRateCouponBuilder::new(schedule.clone(), notionals.clone(), coupon_rates.clone())
                .build();
        for cf in leg {
            let ex_coupon_date = cf.ex_coupon_date();
            assert_eq!(
                ex_coupon_date,
                Date::default(),
                "Ex-coupon date found: {:?}, none expected",
                ex_coupon_date
            )
        }

        // calendar days
        let leg =
            FixedRateCouponBuilder::new(schedule.clone(), notionals.clone(), coupon_rates.clone())
                .with_ex_coupon_period(
                    Period::new(2, Days),
                    NilHoliday::new(),
                    BusinessDayConvention::Unadjusted,
                    false,
                )
                .build();
        for cf in leg {
            let expected = cf.accrual_end_date() - 2;
            assert_eq!(
                cf.ex_coupon_date(),
                expected,
                "Ex-coupon date = {:?}, but expected: {:?}",
                cf.ex_coupon_date(),
                expected
            )
        }

        // business days
        let calendar = Target::new();
        let leg = FixedRateCouponBuilder::new(schedule, notionals, coupon_rates)
            .with_ex_coupon_period(
                Period::new(2, Days),
                calendar.clone(),
                BusinessDayConvention::Preceding,
                false,
            )
            .build();
        for cf in leg {
            let expected = calendar.advance_by_days(
                cf.accrual_end_date(),
                -2,
                Days,
                BusinessDayConvention::Following,
                false,
            );
            assert_eq!(
                cf.ex_coupon_date(),
                expected,
                "Ex-coupon date = {:?}, but expected: {:?}",
                cf.ex_coupon_date(),
                expected
            )
        }
    }

    #[test]
    fn test_irregular_first_coupon_reference_dates_at_end_of_month() {
        let today = Date::new(17, January, 2017);
        let pricing_context = pricing_context(today);

        let from = today;
        let to = Date::new(28, February, 2018);
        let schedule = ScheduleBuilder::new(
            pricing_context,
            from,
            to,
            Period::from(Frequency::Semiannual),
            Target::new(),
        )
        .with_convention(BusinessDayConvention::Unadjusted)
        .with_end_of_month(true)
        .backwards()
        .build();

        let notionals = vec![100.0];
        let coupon_rates = vec![InterestRate::new(
            0.01,
            DayCounter::actual360(),
            Compounding::Simple,
            Frequency::Annual,
        )];
        let leg = FixedRateCouponBuilder::new(schedule, notionals, coupon_rates).build();

        let first_coupon = &leg[0];
        let expected = Date::new(31, August, 2016);
        assert_eq!(
            first_coupon.reference_period_start(),
            expected,
            "Expected reference start date at end of month: {:?}, got {:?}",
            expected,
            first_coupon.reference_period_start(),
        );
    }

    fn pricing_context(eval_date: Date) -> PricingContext {
        PricingContext { eval_date }
    }
}
