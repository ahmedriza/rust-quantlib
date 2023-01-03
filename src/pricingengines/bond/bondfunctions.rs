use crate::{
    cashflows::cashflow,
    datetime::{date::Date, daycounter::DayCounter, frequency::Frequency},
    instruments::bond::{Bond, BondPriceType},
    maths::solvers1d::newtonsafe::NewtonSafe,
    rates::{compounding::Compounding, interestrate::InterestRate},
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
    let notional = bond.notional(settlement_date);
    dirty_price /= (100.0 / notional);

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

/// Clean price of a bond given a yield `y` and settlement date. 
pub fn clean_price(
    bond: &impl Bond,
    y: &InterestRate,
    settlement_date: Date
) -> Real {
    dirty_price(bond, y, settlement_date) - bond.accrued_amount(settlement_date)
}

/// Dirty price of a bond given a yield `y` and settlement date. 
pub fn dirty_price(
    bond: &impl Bond,
    y: &InterestRate,
    settlement_date: Date
) -> Real {
    assert!(
        is_tradeable(bond, settlement_date),
        "Non tradeable at {:?}, (maturity being {:?})",
        settlement_date,
        bond.maturity_date()
    );
    let notional = bond.notional(settlement_date);
    let npv = cashflow::npv(
        bond.cashflows(),
        y,
        false,
        settlement_date,
        Date::default(),
    );
    npv * 100.0 / notional
}

pub fn is_tradeable(bond: &impl Bond, settlement_date: Date) -> bool {
    bond.notional(settlement_date) != 0.0
}

/// Calculate the price of a zero coupon bond (e.g. US Treasury Bill) given its discount yield 
pub fn price_from_discount_yield(
    bond: &impl Bond,
    discount_yield: Real,
    settlement_date: Date
) -> Real {
    let maturity_date = bond.maturity_date();
    let days = maturity_date - settlement_date;
    let interest = 100.0 * discount_yield * days as Real / 360.0;
    100.0 - interest
}
