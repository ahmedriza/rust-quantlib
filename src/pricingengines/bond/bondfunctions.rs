use crate::{
    cashflows::cashflow::{self, Leg},
    datetime::{date::Date, daycounter::DayCounter, frequency::Frequency},
    maths::solvers1d::newtonsafe::NewtonSafe,
    rates::{compounding::Compounding, interestrate::InterestRate},
    types::{Rate, Real, Size},
};

pub fn accrued_amount(cashflows: &Leg, notional: Real, settlement_date: Date) -> Real {
    cashflow::accurued_amount(cashflows, false, settlement_date) * 100.0 / notional
}

#[allow(clippy::too_many_arguments)]
pub fn bond_yield(
    cashflows: &Leg,
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
    cashflows: &Leg,
    y: &InterestRate,
    settlement_date: Date,
) -> Real {
    let npv = cashflow::npv(cashflows, y, false, settlement_date, Date::default());
    npv * 100.0 / notional
}
