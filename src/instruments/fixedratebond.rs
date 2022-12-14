use std::{fmt::Debug, rc::Rc};

use crate::{
    cashflows::{
        cashflow::{CashFlow, CashFlowLeg},
        coupon::Coupon,
        fixedrateleg::FixedRateLeg,
        simplecashflow::{AmortizingPayment, Redemption},
    },
    datetime::{
        businessdayconvention::BusinessDayConvention, calendar::Calendar, date::Date,
        daycounter::DayCounter, frequency::Frequency::*, period::Period, schedule::Schedule,
    },
    maths::comparison::close,
    pricingengines::bond::bondfunctions,
    rates::{compounding::Compounding::Simple, interestrate::InterestRate},
    types::{Integer, Real},
};

use super::bond::Bond;

/// Fixed-rate bond
pub struct FixedRateBond {
    pub settlement_days: Integer,
    pub calendar: Calendar,
    pub maturity_date: Date,
    pub issue_date: Date,
    pub notionals: Vec<Real>,
    pub notional_schedule: Vec<Date>,
    pub cashflows: CashFlowLeg,
    pub redemptions: CashFlowLeg,
}

impl Debug for FixedRateBond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FRB/{}-{:02}-{:02}",
            self.maturity_date.year(),
            self.maturity_date.month() as Integer,
            self.maturity_date.day_of_month(),
        )
    }
}

impl FixedRateBond {
    pub fn new(
        settlement_days: Integer,
        face_amount: Real,
        schedule: Schedule,
        coupons: Vec<Real>,
        accrual_daycounter: DayCounter,
    ) -> Self {
        Self::new_with_options(
            settlement_days,
            face_amount,
            schedule,
            coupons,
            accrual_daycounter,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_options(
        settlement_days: Integer,
        face_amount: Real,
        schedule: Schedule,
        coupons: Vec<Real>,
        accrual_daycounter: DayCounter,
        payment_convention: Option<BusinessDayConvention>,
        redemption: Option<Real>,
        issue_date: Option<Date>,
        payment_calendar: Option<Calendar>,
        ex_coupon_period: Option<Period>,
        ex_coupon_calendar: Option<Calendar>,
        ex_coupon_convention: Option<BusinessDayConvention>,
        ex_coupon_end_of_month: Option<bool>,
        first_period_daycounter: Option<DayCounter>,
    ) -> Self {
        let calendar = payment_calendar
            .as_ref()
            .unwrap_or_else(|| schedule.calendar());

        let mut coupon_rates = vec![];
        for c in coupons {
            coupon_rates.push(InterestRate::new(
                c,
                accrual_daycounter.clone(),
                Simple,
                Annual,
            ));
        }
        let mut fixed_rate_coupon_builder = FixedRateLeg::new(
            schedule.clone(),
            vec![face_amount], // notional amount is face amount
            coupon_rates,
        )
        .with_payment_adjustment(payment_convention.unwrap_or(BusinessDayConvention::Following))
        .with_payment_calendar(calendar.clone());
        if let Some(first_period_daycounter) = first_period_daycounter {
            fixed_rate_coupon_builder =
                fixed_rate_coupon_builder.with_first_period_daycounter(first_period_daycounter);
        };
        if let (
            Some(ex_coupon_period),
            Some(ex_coupon_calendar),
            Some(ex_coupon_convention),
            Some(ex_coupon_end_of_month),
        ) = (
            ex_coupon_period,
            ex_coupon_calendar,
            ex_coupon_convention,
            ex_coupon_end_of_month,
        ) {
            fixed_rate_coupon_builder = fixed_rate_coupon_builder.with_ex_coupon_period(
                ex_coupon_period,
                ex_coupon_calendar,
                ex_coupon_convention,
                ex_coupon_end_of_month,
            );
        }
        let coupons = fixed_rate_coupon_builder.build();

        // Gather the notional information from the cashflows
        let (notionals, notional_schedule) =
            FixedRateBond::calculate_notionals_from_cashflows(&coupons);

        let redemptions = FixedRateBond::calculate_redemptions(
            &notionals,
            &notional_schedule,
            &[redemption.unwrap_or(100.0)], // redemption defaults to 100.0
        );

        // All cashflows including redemptions
        let mut cashflows = CashFlowLeg::new();
        for c in coupons.iter() {
            cashflows.push(Rc::new(c.clone()));
        }
        for r in redemptions.iter() {
            cashflows.push(r.clone());
        }

        // Move the redemptions to the right places while ensuring that they follow coupons
        // with the same date (stable sort).
        cashflows.sort_by_key(|a| a.date());

        assert!(!cashflows.is_empty(), "bond with no cashflows");
        assert_eq!(
            redemptions.len(),
            1,
            "Expected a single redemption, but {} redemptions",
            redemptions.len()
        );

        Self {
            settlement_days,
            calendar: calendar.clone(),
            maturity_date: *schedule.end_date(),
            issue_date: issue_date.unwrap_or_default(),
            notionals,
            notional_schedule,
            cashflows,
            redemptions,
        }
    }

    fn calculate_notionals_from_cashflows<T: Coupon>(coupons: &[T]) -> (Vec<Real>, Vec<Date>) {
        let mut notionals: Vec<Real> = Vec::new();
        let mut notional_schedule = vec![Date::default()];
        let mut last_payment_date = Date::default();

        for cf in coupons.iter() {
            let notional = cf.nominal();
            if notionals.is_empty() {
                last_payment_date = cf.date();
                notionals.push(notional);
            } else if !close(notional, notionals[notionals.len() - 1]) {
                notionals.push(cf.nominal());
                notional_schedule.push(last_payment_date);
                last_payment_date = cf.date();
            } else {
                last_payment_date = cf.date();
            }
        }
        assert!(!notionals.is_empty(), "No coupons provided");
        notionals.push(0.0);
        notional_schedule.push(last_payment_date);

        (notionals, notional_schedule)
    }

    fn calculate_redemptions(
        notionals: &[Real],
        notional_schedule: &[Date],
        redemption_values: &[Real],
    ) -> CashFlowLeg {
        let mut redemptions = CashFlowLeg::new();
        for i in 1..notional_schedule.len() {
            let r = if i < redemption_values.len() {
                redemption_values[i]
            } else if !redemption_values.is_empty() {
                redemption_values[redemption_values.len() - 1]
            } else {
                100.0
            };
            let amount = (r / 100.0) * (notionals[i - 1] - notionals[i]);
            let payment: Rc<dyn CashFlow> = if i < notional_schedule.len() - 1 {
                Rc::new(AmortizingPayment::new(amount, notional_schedule[i]))
            } else {
                Rc::new(Redemption::new(amount, notional_schedule[i]))
            };
            redemptions.push(payment);
        }

        redemptions
    }
}

impl Bond for FixedRateBond {
    fn calendar(&self) -> &Calendar {
        &self.calendar
    }

    fn cashflows(&self) -> &CashFlowLeg {
        &self.cashflows
    }

    fn issue_date(&self) -> Date {
        self.issue_date
    }

    fn maturity_date(&self) -> Date {
        bondfunctions::maturity_date(&self.cashflows)
    }

    fn notional_schedule(&self) -> &Vec<Date> {
        &self.notional_schedule
    }

    fn notionals(&self) -> &Vec<Real> {
        &self.notionals
    }

    fn settlement_days(&self) -> Integer {
        self.settlement_days
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::{
        context::pricing_context::PricingContext,
        datetime::{
            date::Date, daycounter::DayCounter, frequency::Frequency,
            holidays::unitedstates::UnitedStates, months::Month::*, period::Period,
            schedulebuilder::ScheduleBuilder, timeunit::TimeUnit::*,
        },
        instruments::bond::Bond,
        rates::compounding::Compounding,
    };

    use super::FixedRateBond;

    #[test]
    fn test_fixedratebond() {
        let pricing_date = Date::new(6, June, 2022);
        let pricing_context = PricingContext::new(pricing_date);
        let settlement_days = 1;
        let settlement = pricing_date + settlement_days;

        let calendar = UnitedStates::government_bond();
        let daycounter = DayCounter::actual_actual_old_isma();
        let compounding = Compounding::SimpleThenCompounded;
        let frequency = Frequency::Semiannual;

        let maturity = Date::new(31, May, 2024);
        let ref_start = maturity - Period::new(2, Years);
        let face_amount = 100.0;
        let coupons = vec![2.5 / 100.0];

        let schedule = ScheduleBuilder::new(
            pricing_context,
            ref_start,
            maturity,
            Period::from(frequency),
            calendar,
        )
        .build();

        let bond: Box<dyn Bond> = Box::new(FixedRateBond::new(
            settlement_days,
            face_amount,
            schedule,
            coupons,
            daycounter.clone(),
        ));

        let clean_price = 99.0 + (18.0 + 3.0 / 4.0) / 32.0;
        let bond_yield =
            100.0 * bond.bond_yield(clean_price, daycounter, compounding, frequency, settlement);
        let expected_bond_yield = 2.715783233393491;
        assert!(
            (expected_bond_yield - bond_yield).abs() < 1.0e-10,
            "Expected bond yield: {}, but got: {}",
            expected_bond_yield,
            bond_yield
        );
    }
}
