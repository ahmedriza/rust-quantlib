use crate::{
    datetime::{date::Date, daycounter::DayCounter, frequency::Frequency},
    rates::{compounding::Compounding, interestrate::InterestRate},
    types::{Rate, Real},
};

use super::cashflow::{self, CashFlow};

/// Provides functions to help in the calculation of the internal rate of return of bond
/// cash flows.
pub struct IrrFinder<'a, T: CashFlow> {
    pub cashflows: &'a [T],
    pub npv: Real,
    pub daycounter: DayCounter,
    pub compounding: Compounding,
    pub frequency: Frequency,
    pub include_settlement_date_flows: bool,
    pub settlement_date: Date,
    pub npv_date: Date,
}

impl<'a, T: CashFlow> IrrFinder<'a, T> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cashflows: &'a [T],
        npv: Real,
        daycounter: DayCounter,
        compounding: Compounding,
        frequency: Frequency,
        include_settlement_date_flows: bool,
        settlement_date: Date,
        npv_date: Date,
    ) -> Self {
        IrrFinder::check_sign(
            npv,
            cashflows,
            settlement_date,
            include_settlement_date_flows,
        );
        Self {
            cashflows,
            npv,
            daycounter,
            compounding,
            frequency,
            include_settlement_date_flows,
            settlement_date,
            npv_date,
        }
    }

    /// Calculate the NPV of cash flows at the given yield point
    pub fn at(&self, y: Rate) -> Real {
        let bond_yield = InterestRate::new(
            y,
            self.daycounter.clone(),
            self.compounding.clone(),
            self.frequency,
        );
        let _npv = cashflow::npv(
            self.cashflows,
            &bond_yield,
            self.include_settlement_date_flows,
            self.settlement_date,
            self.npv_date,
        );

        self.npv - _npv
    }

    /// Calculate the modified duration of bond cash flows at the given yield point
    pub fn derivative(&self, y: Rate) -> Real {
        let bond_yield = InterestRate::new(
            y,
            self.daycounter.clone(),
            self.compounding.clone(),
            self.frequency,
        );
        cashflow::modified_duration(
            self.cashflows,
            &bond_yield,
            self.include_settlement_date_flows,
            self.settlement_date,
            self.npv_date,
        )
    }

    /// Depending on the sign of the market price, check that cash flows of the opposite sign
    /// have been specified (otherwise IRR is nonsensical.)
    fn check_sign(
        npv: Real,
        cashflows: &[T],
        settlement_date: Date,
        include_settlement_date_flows: bool,
    ) {
        let mut last_sign = (-npv).signum();
        let mut sign_changes = 0;

        for cf in cashflows {
            if !cf.has_occurred(&settlement_date, include_settlement_date_flows)
                && !cf.trading_ex_coupon(settlement_date)
            {
                let this_sign = cf.amount().signum();
                if last_sign * this_sign < 0.0 {
                    // sign change
                    sign_changes += 1;
                }
                last_sign = this_sign;
            }
        }

        assert!(
            sign_changes > 0,
            "The given cash flows cannot result in the given market price due to \
                 their sign, market price: {}",
            npv
        );
    }
}
