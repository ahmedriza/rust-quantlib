# rust-quantlib


This is a pure Rust implementation of
[QuantLib](https://www.quantlib.org/).  We have tried to follow Rust idioms as much as possible.

## Example

The following demonstrates an example of using the library for pricing bonds. More examples
can be found in the [examples](https://github.com/ahmedriza/rust-quantlib/tree/master/examples)
directory of the source. We will add more examples there as work on the library progresses.


```rust

use rust_quantlib::context::pricing_context::PricingContext;
use rust_quantlib::datetime::calendar::Calendar;
use rust_quantlib::datetime::daycounter::DayCounter;
use rust_quantlib::datetime::frequency::Frequency;
use rust_quantlib::datetime::holidays::unitedstates::UnitedStates;
use rust_quantlib::datetime::period::Period;
use rust_quantlib::datetime::schedulebuilder::ScheduleBuilder;
use rust_quantlib::datetime::timeunit::TimeUnit::*;
use rust_quantlib::datetime::{date::Date, months::Month::*};
use rust_quantlib::instruments::bond::Bond;
use rust_quantlib::instruments::fixedratebond::FixedRateBond;
use rust_quantlib::instruments::zerocouponbond::ZeroCouponBond;
use rust_quantlib::rates::compounding::Compounding;
use rust_quantlib::types::{Integer, Real};

/// This example shows how to calcualate the maturity yield on US Treasury Bills, Notes
/// and Bonds using real market data.
pub fn main() {
    let pricing_context = PricingContext::new(Date::new(6, June, 2022));
    let common_data = CommonData::new(pricing_context, 1);

    let mut results = vec![];
    zero_coupon_bonds(&common_data, make_tbill_market_data(), &mut results);
    fixed_rate_bonds(&common_data, make_bond_market_data(), &mut results);

    show_results(results);
}

pub fn zero_coupon_bonds(
    cd: &CommonData,
    market_data: TBillMarketData,
    results: &mut Vec<Result>
) {
    for i in 0..market_data.maturities.len() {
        let maturity_date = market_data.maturities[i];
        let discount_yield = market_data.discount_yields[i] / 100.0;

        let zcb = ZeroCouponBond::new(
            cd.settlement_days,
            &cd.calendar,
            cd.face_amount,
            maturity_date,
        );
        // Calcualte the price from the discount yield
        let price = zcb.price_from_discount_yield(discount_yield, cd.settlement_date);
        // Calculate yield
        let bond_yield = 100.0
            * zcb.bond_yield(
                price,
                cd.daycounter.clone(),
                cd.compounding.clone(),
                cd.frequency,
                cd.settlement_date,
            );

        results.push(Result {
            bond_description: format!("{:?}/{:?}", zcb, zcb.period(cd.pricing_context.eval_date)),
            price,
            bond_yield,
        });
    }
}

pub fn fixed_rate_bonds(cd: &CommonData, md: BondMarketData, results: &mut Vec<Result>) {
    for (i, maturity) in md.maturities.iter().enumerate() {
        let ref_start = maturity - md.periods[i];
        let schedule = ScheduleBuilder::new(
            cd.pricing_context,
            ref_start,
            *maturity,
            Period::from(cd.frequency),
            cd.calendar.clone(),
        )
        .build();
        let frb = FixedRateBond::new(
            cd.settlement_days,
            cd.face_amount,
            schedule,
            vec![md.coupons[i] / 100.0],
            cd.daycounter.clone(),
        );
        let price = md.clean_prices[i];
        // Calculate yield
        let bond_yield = 100.0
            * frb.bond_yield(
                price,
                cd.daycounter.clone(),
                cd.compounding.clone(),
                cd.frequency,
                cd.settlement_date,
            );

        results.push(Result {
            bond_description: format!("{:?}/{:?}", frb, md.periods[i]),
            price,
            bond_yield,
        })
    }
}

pub fn make_tbill_market_data() -> TBillMarketData {
    TBillMarketData {
        maturities: vec![
            Date::new(5, July, 2022),
            Date::new(2, August, 2022),
            Date::new(8, September, 2022),
            Date::new(8, December, 2022),
            Date::new(18, May, 2023),
        ],
        discount_yields: vec![0.851, 1.016, 1.214, 1.694, 2.111],
    }
}

pub fn make_bond_market_data() -> BondMarketData {
    BondMarketData {
        coupons: vec![2.5, 2.75, 2.625, 2.75, 2.875, 3.25, 2.875],
        clean_prices: vec![
            99.0 + (18.0 + 3.0 / 4.0) / 32.0,
            99.0 + (17.0 + 3.0 / 8.0) / 32.0,
            98.0 + (7.0 + 1.0 / 2.0) / 32.0,
            98.0 + (4.0 + 1.0 / 4.0) / 32.0,
            98.0 + (25.0 + 3.0 / 4.0) / 32.0,
            97.0 + (25.0 + 1.0 / 2.0) / 32.0,
            94.0 + (12.0 + 1.0 / 2.0) / 32.0,
        ],
        periods: vec![
            Period::new(2, Years),
            Period::new(3, Years),
            Period::new(5, Years),
            Period::new(7, Years),
            Period::new(10, Years),
            Period::new(20, Years),
            Period::new(30, Years),
        ],
        maturities: vec![
            Date::new(31, May, 2024),
            Date::new(15, May, 2025),
            Date::new(31, May, 2027),
            Date::new(31, May, 2029),
            Date::new(15, May, 2032),
            Date::new(15, May, 2042),
            Date::new(15, May, 2052),
        ],
    }
}

pub fn show_results(results: Vec<Result>) {
    println!(
        "{:>2} {:<20} {:6} {:8}",
        "#", "Bond Description", "price", "yield"
    );
    println!("{:<}", "-".repeat(37));
    for (i, r) in results.iter().enumerate() {
        println!(
            "{:2} {:<20} {:.3} {:.3}%",
            i + 1,
            r.bond_description,
            r.price,
            r.bond_yield
        );
    }
}

/// Common data for pricing
pub struct CommonData {
    pub pricing_context: PricingContext,
    pub settlement_days: Integer,
    pub settlement_date: Date,
    pub calendar: Calendar,
    pub daycounter: DayCounter,
    pub compounding: Compounding,
    pub frequency: Frequency,
    pub face_amount: Real,
}

/// US Treasury Bill market data
pub struct TBillMarketData {
    pub discount_yields: Vec<Real>,
    pub maturities: Vec<Date>,
}

/// US Note and Bond market data
pub struct BondMarketData {
    pub coupons: Vec<Real>,
    pub clean_prices: Vec<Real>,
    pub periods: Vec<Period>,
    pub maturities: Vec<Date>,
}

pub struct Result {
    pub bond_description: String,
    pub price: Real,
    pub bond_yield: Real,
}

impl CommonData {
    pub fn new(pricing_context: PricingContext, settlement_days: Integer) -> Self {
        let settlement_date = pricing_context.eval_date + settlement_days;
        Self {
            pricing_context,
            settlement_days,
            settlement_date,
            calendar: UnitedStates::government_bond(),
            daycounter: DayCounter::actual_actual_old_isma(),
            compounding: Compounding::SimpleThenCompounded,
            frequency: Frequency::Semiannual,
            face_amount: 100.0,
        }
    }
}
````
#### Output

| # | Bond Description  |  price | yield  |
|---|-------------------|--------|--------|
| 1 | ZCB/2022-07-05/1M | 99.934 | 0.863% |
| 2 | ZCB/2022-08-02/2M | 99.842 | 1.032% |
| 3 | ZCB/2022-09-08/3M | 99.686 | 1.235% |
| 4 | ZCB/2022-12-08/6M | 99.134 | 1.732% |
| 5 | ZCB/2023-05-18/12M| 97.977 | 2.174% |
| 6 | FRB/2024-05-31/2Y | 99.586 | 2.716% |
| 7 | FRB/2025-05-15/3Y | 99.543 | 2.913% |
| 8 | FRB/2027-06-01/5Y | 98.234 | 3.009% |
| 9 | FRB/2029-05-31/7Y | 98.133 | 3.049% |
|10 | FRB/2032-05-17/10Y| 98.805 | 3.015% |
|11 | FRB/2042-05-15/20Y| 97.797 | 3.403% |
|12 | FRB/2052-05-15/30Y| 94.391 | 3.166% |

