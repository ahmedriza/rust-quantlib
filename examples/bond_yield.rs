use rust_quantlib::context::pricing_context::PricingContext;
use rust_quantlib::datetime::daycounter::DayCounter;
use rust_quantlib::datetime::frequency::Frequency::Semiannual;
use rust_quantlib::datetime::holidays::unitedstates::UnitedStates;
use rust_quantlib::datetime::{date::Date, months::Month::*};
use rust_quantlib::instruments::bond::Bond;
use rust_quantlib::instruments::zerocouponbond::ZeroCouponBond;
use rust_quantlib::rates::compounding::Compounding::*;

/// This example shows how to calcualate the maturity yield on US Treasury Bills using
/// real market data.
pub fn main() {
    let pricing_context = PricingContext::new(Date::new(6, June, 2022));
    let settlement_days = 1;
    let settlement_date = pricing_context.eval_date + settlement_days;
    let calendar = UnitedStates::government_bond();
    let face_amount = 100.0;

    let depo_maturity_dates = vec![
        Date::new(5, July, 2022),
        Date::new(2, August, 2022),
        Date::new(8, September, 2022),
        Date::new(8, December, 2022),
        Date::new(18, May, 2023),
    ];

    let depo_discount_yields = vec![0.851, 1.016, 1.214, 1.694, 2.111];

    for i in 0..depo_maturity_dates.len() {
        let maturity_date = depo_maturity_dates[i];
        let discount_yield = depo_discount_yields[i] / 100.0;
        let zcb = ZeroCouponBond::new(
            settlement_days,
            &calendar,
            face_amount,
            maturity_date,
            None,
            None,
            None,
        );

        let price = zcb.price_from_discount_yield(discount_yield, settlement_date);

        let bond_yield = 100.0
            * zcb.bond_yield(
                price,
                DayCounter::actual_actual_old_isma(),
                SimpleThenCompounded,
                Semiannual,
                settlement_date,
                None,
                None,
                None,
            );

        println!(
            "{:?}, price: {}, bond yield: {:.3}%",
            zcb, price, bond_yield
        );
    }
}
