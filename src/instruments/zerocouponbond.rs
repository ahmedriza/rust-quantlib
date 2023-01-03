use std::rc::Rc;

use crate::{
    cashflows::{cashflow::Leg, simplecashflow::Redemption},
    datetime::{
        businessdayconvention::BusinessDayConvention::{self, *},
        calendar::Calendar,
        date::Date,
        daycounter::DayCounter,
        frequency::Frequency,
    },
    pricingengines::bond::bondfunctions,
    rates::compounding::Compounding,
    types::{Integer, Rate, Real, Size},
};

use super::bond::{Bond, BondPriceType};

/// Zero coupon bond
pub struct ZeroCouponBond {
    pub settlement_days: Integer,
    pub calendar: Calendar,
    pub face_amount: Real,
    pub maturity_date: Date,
    pub payment_convention: BusinessDayConvention,
    pub redemption: Real,
    pub issue_date: Date,

    pub notionals: Vec<Real>,
    pub notional_schedule: Vec<Date>,
    pub cashflows: Leg,
    pub redemptions: Leg,
}

impl ZeroCouponBond {
    pub fn new(
        settlement_days: Integer,
        calendar: Calendar,
        face_amount: Real,
        maturity_date: Date,
        payment_convention: Option<BusinessDayConvention>,
        redemption: Option<Real>,
        issue_date: Option<Date>,
    ) -> Self {
        let payment_convention = payment_convention.unwrap_or(Following);
        let redemption = redemption.unwrap_or(100.0);
        let issue_date = issue_date.unwrap_or_default();

        let redemption_date = calendar.adjust(maturity_date, payment_convention);

        let redemption_cash_flow = Rc::new(Redemption::new(
            face_amount * redemption / 100.0,
            redemption_date,
        ));

        Self {
            settlement_days,
            calendar,
            face_amount,
            maturity_date,
            payment_convention,
            redemption,
            issue_date,
            notionals: vec![face_amount, 0.0],
            notional_schedule: vec![Date::default(), redemption_cash_flow.cashflow.date],
            cashflows: vec![redemption_cash_flow.clone()],
            redemptions: vec![redemption_cash_flow],
        }
    }
}

impl Bond for ZeroCouponBond {
    fn accrued_amount(&self, settlement_date: Date) -> Real {
        let current_notional = self.notional(settlement_date);
        if current_notional == 0.0 {
            return 0.0;
        }
        bondfunctions::accrued_amount(self, settlement_date)
    }

    fn bond_yield(
        &self,
        clean_price: Real,
        daycounter: DayCounter,
        compounding: Compounding,
        frequency: Frequency,
        settlement_date: Date,
        accuracy: Option<Real>,
        max_evaluations: Option<Size>,
        guess: Option<Real>,
        price_type: Option<BondPriceType>,
    ) -> Rate {
        let accuracy = accuracy.unwrap_or(1.0e-8);
        let max_evaluations = max_evaluations.unwrap_or(100);
        let guess = guess.unwrap_or(0.05);
        let price_type = price_type.unwrap_or(BondPriceType::Clean);

        let current_notional = self.notional(settlement_date);
        if current_notional == 0.0 {
            return 0.0;
        }

        bondfunctions::bond_yield(
            self,
            clean_price,
            daycounter,
            compounding,
            frequency,
            settlement_date,
            accuracy,
            max_evaluations,
            guess,
            price_type,
        )
    }

    fn calendar(&self) -> &Calendar {
        &self.calendar
    }

    fn cashflows(&self) -> &Leg {
        &self.cashflows
    }

    fn issue_date(&self) -> Date {
        self.issue_date
    }

    fn maturity_date(&self) -> Date {
        self.maturity_date
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
            date::Date, daycounter::DayCounter, frequency::Frequency::Semiannual,
            holidays::unitedstates::UnitedStates, months::Month::*,
        },
        instruments::bond::Bond,
        rates::compounding::Compounding,
        types::Real,
    };

    use super::ZeroCouponBond;

    #[test]
    pub fn test_zero_coupon_bond() {
        let pricing_context = PricingContext::new(Date::new(6, June, 2022));
        let settlement_days = 1;
        let settlement_date = pricing_context.eval_date + settlement_days;
        let calendar = UnitedStates::government_bond();
        let face_amount = 100.0;
        let discount_yield = 0.851 / 100.0;        
        let maturity_date = Date::new(5, July, 2022);

        let zcb = ZeroCouponBond::new(1, calendar, face_amount, maturity_date, None, None, None);
        let clean_price = zcb_clean_price(discount_yield, maturity_date, settlement_date);
        let bond_yield = 100.0
            * zcb.bond_yield(
                clean_price,
                DayCounter::actual_actual_old_isma(),
                Compounding::SimpleThenCompounded,
                Semiannual,
                settlement_date,
                None,
                None,
                None,
                None,
            );

        let expected_clean_price = 99.93381111111111;
        let expected_bond_yield = 0.8633917455289686;

        assert!(
            (expected_clean_price - clean_price).abs() < 1.0e-10,
            "expected clean price: {}, actual clean price: {}, diff: {}",
            expected_clean_price,
            clean_price,
            (expected_clean_price - clean_price).abs()
        );
        assert!(
            (expected_bond_yield - bond_yield).abs() < 1.0e-10,
            "expected bond yield: {}, actual bond yield: {}, diff: {}",
            expected_bond_yield,
            bond_yield,
            (expected_bond_yield - bond_yield).abs()
        );
    }

    fn zcb_clean_price(discount_yield: Real, maturity_date: Date, settlement_date: Date) -> Real {
        let days = maturity_date - settlement_date;
        let interest = 100.0 * discount_yield * days as f64 / 360.0;
        100.0 - interest
    }
}