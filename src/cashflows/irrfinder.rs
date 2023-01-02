use crate::{
    datetime::{date::Date, daycounter::DayCounter, frequency::Frequency},
    rates::{compounding::Compounding, interestrate::InterestRate},
    types::{Rate, Real},
};

use super::cashflow::{self, Leg};

pub struct IrrFinder<'a> {
    pub cashflows: &'a Leg,
    pub npv: Real,
    pub daycounter: DayCounter,
    pub compounding: Compounding,
    pub frequency: Frequency,
    pub include_settlement_date_flows: bool,
    pub settlement_date: Date,
    pub npv_date: Date,
}

impl<'a> IrrFinder<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cashflows: &'a Leg,
        npv: Real,
        daycounter: DayCounter,
        compounding: Compounding,
        frequency: Frequency,
        include_settlement_date_flows: bool,
        settlement_date: Date,
        npv_date: Date,
    ) -> Self {
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

    pub fn at(&self, y: Rate) -> Real {
        // TODO remove the clones
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

    pub fn derivative(&self, y: Rate) -> Real {
        // TODO remove the clones
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
}
