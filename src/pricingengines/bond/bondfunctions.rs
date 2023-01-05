use crate::{
    cashflows::{
        cashflow::{self, CashFlowLeg, CashFlow},
        coupon::Coupon,
    },
    datetime::{date::Date, daycounter::DayCounter, frequency::Frequency, SerialNumber},
    maths::solvers1d::newtonsafe::NewtonSafe,
    rates::{compounding::Compounding, interestrate::InterestRate},
    types::{Rate, Real, Size, Time},
};

pub fn accrued_amount<T: CashFlow>(
    cashflows: &Vec<T>,
    notional: Real,
    settlement_date: Date,
) -> Real {
    let accrued_amount = cashflow::accrued_amount(cashflows, false, settlement_date);
    accrued_amount * 100.0 / notional
}

pub fn accrued_days<T: Coupon>(coupons: &Vec<T>, settlement_date: Date) -> SerialNumber {
    cashflow::accrued_days(coupons, false, settlement_date)
}

pub fn accrued_period<T: Coupon>(coupons: &Vec<T>, settlement_date: Date) -> Time {
    cashflow::accrued_period(coupons, false, settlement_date)
}

#[allow(clippy::too_many_arguments)]
pub fn bond_yield(
    cashflows: &CashFlowLeg,
    price: Real,
    daycounter: DayCounter,
    compounding: Compounding,
    frequency: Frequency,
    settlement_date: Date,
    accuracy: Real,
    max_evaluations: Size,
    guess: Real,
) -> Rate {
    let solver = NewtonSafe::with_max_evaluations(max_evaluations);
    cashflow::bond_yield(
        &solver,
        cashflows,
        price,
        daycounter,
        compounding,
        frequency,
        false,
        settlement_date,
        settlement_date,
        accuracy,
        guess,
    )
}

/// Dirty price of a bond given a yield `y` and settlement date.
pub fn dirty_price(
    notional: Real,
    cashflows: &CashFlowLeg,
    y: &InterestRate,
    settlement_date: Date,
) -> Real {
    let npv = cashflow::npv(cashflows, y, false, settlement_date, Date::default());
    npv * 100.0 / notional
}

pub fn maturity_date<T: CashFlow>(cashflows: &Vec<T>) -> Date {
    cashflow::maturity_date(cashflows)
}
