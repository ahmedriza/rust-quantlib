use crate::{
    datetime::{date::Date, daycounter::DayCounter, frequency::Frequency},
    instruments::bond::{Bond, BondPriceType},
    rates::compounding::Compounding,
    types::{Rate, Real, Size},
};

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
    todo!()
}
