use crate::{
    cashflows::cashflow,
    datetime::{date::Date, daycounter::DayCounter, frequency::Frequency},
    instruments::bond::{Bond, BondPriceType},
    maths::solvers1d::newtonsafe::NewtonSafe,
    rates::compounding::Compounding,
    types::{Rate, Real, Size},
};

pub fn accrued_amount(bond: &impl Bond, settlement_date: Date) -> Real {
    if !is_tradeable(bond, settlement_date) {
        return 0.0;
    }
    cashflow::accurued_amount(bond.cashflows(), false, settlement_date) * 100.0
        / bond.notional(settlement_date)
}

#[allow(unused)]
#[allow(clippy::too_many_arguments)]
pub fn bond_yield(
    bond: &impl Bond,
    clean_price: Real,
    daycounter: DayCounter,
    compounding: Compounding,
    frequency: Frequency,
    settlement_date: Date,
    accuracy: Real,
    max_evaluations: Size,
    guess: Real,
    price_type: BondPriceType,
) -> Rate {
    assert!(
        is_tradeable(bond, settlement_date),
        "Non tradeable at {:?}, (maturity being {:?})",
        settlement_date,
        bond.maturity_date()
    );

    let mut dirty_price = clean_price;
    if price_type == BondPriceType::Clean {
        dirty_price += bond.accrued_amount(settlement_date);
    }
    dirty_price = dirty_price / 100.0 / bond.notional(settlement_date);
    println!("dirty price: {}", dirty_price);

    let solver = NewtonSafe::default();
    // TODO set max_evaluations

    cashflow::bond_yield(
        &solver,
        bond.cashflows(),
        dirty_price,
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

pub fn is_tradeable(bond: &impl Bond, settlement_date: Date) -> bool {
    bond.notional(settlement_date) != 0.0
}
