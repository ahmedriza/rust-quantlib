use crate::{datetime::date::Date, types::Real};

use super::cashflow::CashFlow;

/// Predetermined cash flow.
/// This cash flow pays a predetermined amount at a given date.
pub struct SimpleCashFlow {
    /// The amount of the cash flow. The amount is not discounted, i.e., it is the
    /// actual amount paid at the cash flow date.    
    pub amount: Real,
    /// The date at which the cashflow occurs
    pub date: Date,
}

impl SimpleCashFlow {
    pub fn new(amount: Real, date: Date) -> Self {
        Self { amount, date }
    }
}

impl CashFlow for SimpleCashFlow {
    fn accrual_start_date(&self) -> Date {
        Date::default()
    }

    fn accrual_end_date(&self) -> Date {
        Date::default()
    }

    fn accrued_amount(&self, _date: Date) -> Real {
        0.0
    }

    fn amount(&self) -> Real {
        self.amount
    }

    fn date(&self) -> Date {
        self.date
    }

    fn reference_period_start(&self) -> Date {
        Date::default()
    }

    fn reference_period_end(&self) -> Date {
        Date::default()
    }
}

// -------------------------------------------------------------------------------------------------

/// Bond redemption
pub struct Redemption {
    pub cashflow: SimpleCashFlow,
}

impl Redemption {
    pub fn new(amount: Real, date: Date) -> Self {
        Self {
            cashflow: SimpleCashFlow::new(amount, date),
        }
    }
}

impl CashFlow for Redemption {
    fn accrual_start_date(&self) -> Date {
        Date::default()
    }

    fn accrual_end_date(&self) -> Date {
        Date::default()
    }

    fn accrued_amount(&self, _date: Date) -> Real {
        0.0
    }

    fn amount(&self) -> Real {
        self.cashflow.amount
    }

    fn date(&self) -> Date {
        self.cashflow.date
    }

    fn reference_period_start(&self) -> Date {
        Date::default()
    }

    fn reference_period_end(&self) -> Date {
        Date::default()
    }
}

// -------------------------------------------------------------------------------------------------

/// Amortizing payment
pub struct AmortizingPayment {
    pub cashflow: SimpleCashFlow,
}

impl AmortizingPayment {
    pub fn new(amount: Real, date: Date) -> Self {
        Self {
            cashflow: SimpleCashFlow::new(amount, date),
        }
    }
}

impl CashFlow for AmortizingPayment {
    fn accrual_start_date(&self) -> Date {
        Date::default()
    }

    fn accrual_end_date(&self) -> Date {
        Date::default()
    }

    fn accrued_amount(&self, _date: Date) -> Real {
        0.0
    }

    fn amount(&self) -> Real {
        self.cashflow.amount
    }

    fn date(&self) -> Date {
        self.cashflow.date
    }

    fn reference_period_start(&self) -> Date {
        Date::default()
    }

    fn reference_period_end(&self) -> Date {
        Date::default()
    }
}

// -------------------------------------------------------------------------------------------------
