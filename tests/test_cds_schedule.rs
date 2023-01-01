use std::collections::HashMap;

use rust_quantlib::instruments::creditdefaultswap::cds_maturity;
use rust_quantlib::datetime::{
    businessdayconvention::BusinessDayConvention,
    calendars::weekendsonly::WeekendsOnly,
    context::pricing_context::PricingContext,
    date::Date,
    dategenerationrule::DateGenerationRule,
    months::Month::*,
    months::*,
    period::Period,
    schedule::{Schedule, ScheduleBuilder},
    timeunit::TimeUnit::*,
};

#[test]
fn test_cds2015_convention() {
    let rule = DateGenerationRule::CDS2015;
    let tenor = Period::new(5, Years);

    // From September 20th 2016 to March 19th 2017 of the next year, end date is December 20th 2021
    // for a 5 year CDS. To get the correct schedule, you can first use the cds_maturity function
    // to get the maturity from the tenor.
    let trade_date = Date::new(12, December, 2016);
    let maturity = cds_maturity(&trade_date, tenor, rule);

    let exp_start = Date::new(20, September, 2016);
    let exp_maturity = Date::new(20, December, 2021);

    assert_eq!(maturity, exp_maturity);
    let s = make_cds_schedule(trade_date, maturity, rule);
    assert_eq!(s.start_date(), &exp_start);
    assert_eq!(s.end_date(), &exp_maturity);

    // If we just use 12 Dec 2016 + 5Y = 12 Dec 2021 as termination date in the schedule,
    // the schedule constructor can use any of the allowable CDS dates i.e. 20 Mar, Jun, Sep
    // and Dec. In the constructor, we just use the next one here i.e. 20 Dec 2021.
    // We get the same results as above.
    let maturity = trade_date + tenor;
    let s = make_cds_schedule(trade_date, maturity, rule);
    assert_eq!(s.start_date(), &exp_start);
    assert_eq!(s.end_date(), &exp_maturity);

    // We do the same tests but with a trade date of 1 Mar 2017. Using cds_maturity to get maturity
    // date from 5Y tenor, we get the same maturity as above.
    let trade_date = Date::new(1, March, 2017);
    let maturity = cds_maturity(&trade_date, tenor, rule);
    assert_eq!(maturity, exp_maturity);
    let s = make_cds_schedule(trade_date, maturity, rule);
    let exp_start = Date::new(20, December, 2016);
    assert_eq!(s.start_date(), &exp_start);
    assert_eq!(s.end_date(), &exp_maturity);

    // Using 1 Mar 2017 + 5Y = 1 Mar 2022 as termination date in the schedule, the constructor
    // just uses the next allowable CDS date i.e. 20 Mar 2022. We must update the expected maturity.
    let maturity = trade_date + tenor;
    let s = make_cds_schedule(trade_date, maturity, rule);
    assert_eq!(s.start_date(), &exp_start);
    let exp_maturity = Date::new(20, March, 2022);
    assert_eq!(s.end_date(), &exp_maturity);

    // From March 20th 2017 to September 19th 2017, end date is June 20th 2022 for a 5 year CDS.
    let trade_date = Date::new(20, March, 2017);
    let maturity = cds_maturity(&trade_date, tenor, rule);
    let exp_start = Date::new(20, March, 2017);
    let exp_maturity = Date::new(20, June, 2022);
    assert_eq!(maturity, exp_maturity);
    let s = make_cds_schedule(trade_date, maturity, rule);
    assert_eq!(s.start_date(), &exp_start);
    assert_eq!(s.end_date(), &exp_maturity);
}

#[test]
fn test_cds2015_convention_grid() {
    // Testing against section 11 of ISDA doc FAQs Amending when Single Name CDS roll to
    // new on-the-run contracts. December 20, 2015 Go-Live
    //
    // Test inputs and expected outputs
    //
    // The map key: pair with 1st element equal to trade date and 2nd element equal to CDS tenor.
    // The map value: pair with 1st and 2nd element equal to expected start and end date
    // respectively.
    // The trade dates are from the transition dates in the doc i.e. 20th Mar, Jun, Sep and Dec
    // in 2016 and a day either side. The tenors are selected tenors from the doc i.e. short
    // quarterly tenors less than 1Y, 1Y and 5Y.
    let _inputs = vec![
        (
            (Date::new(19, Mar, 2016), Period::new(3, Months)),
            (Date::new(21, Dec, 2015), Date::new(20, Mar, 2016)),
        ),
        (
            (Date::new(20, Mar, 2016), Period::new(3, Months)),
            (Date::new(21, Dec, 2015), Date::new(20, Sep, 2016)),
        ),
        (
            (Date::new(21, Mar, 2016), Period::new(3, Months)),
            (Date::new(21, Mar, 2016), Date::new(20, Sep, 2016)),
        ),
        (
            (Date::new(19, Jun, 2016), Period::new(3, Months)),
            (Date::new(21, Mar, 2016), Date::new(20, Sep, 2016)),
        ),
        (
            (Date::new(20, Jun, 2016), Period::new(3, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Sep, 2016)),
        ),
        (
            (Date::new(21, Jun, 2016), Period::new(3, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Sep, 2016)),
        ),
        (
            (Date::new(19, Sep, 2016), Period::new(3, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Sep, 2016)),
        ),
        (
            (Date::new(20, Sep, 2016), Period::new(3, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(21, Sep, 2016), Period::new(3, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(19, Dec, 2016), Period::new(3, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(20, Dec, 2016), Period::new(3, Months)),
            (Date::new(20, Dec, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(21, Dec, 2016), Period::new(3, Months)),
            (Date::new(20, Dec, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(19, Mar, 2016), Period::new(6, Months)),
            (Date::new(21, Dec, 2015), Date::new(20, Jun, 2016)),
        ),
        (
            (Date::new(20, Mar, 2016), Period::new(6, Months)),
            (Date::new(21, Dec, 2015), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(21, Mar, 2016), Period::new(6, Months)),
            (Date::new(21, Mar, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(19, Jun, 2016), Period::new(6, Months)),
            (Date::new(21, Mar, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(20, Jun, 2016), Period::new(6, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(21, Jun, 2016), Period::new(6, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(19, Sep, 2016), Period::new(6, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(20, Sep, 2016), Period::new(6, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(21, Sep, 2016), Period::new(6, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(19, Dec, 2016), Period::new(6, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(20, Dec, 2016), Period::new(6, Months)),
            (Date::new(20, Dec, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(21, Dec, 2016), Period::new(6, Months)),
            (Date::new(20, Dec, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(19, Mar, 2016), Period::new(9, Months)),
            (Date::new(21, Dec, 2015), Date::new(20, Sep, 2016)),
        ),
        (
            (Date::new(20, Mar, 2016), Period::new(9, Months)),
            (Date::new(21, Dec, 2015), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(21, Mar, 2016), Period::new(9, Months)),
            (Date::new(21, Mar, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(19, Jun, 2016), Period::new(9, Months)),
            (Date::new(21, Mar, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(20, Jun, 2016), Period::new(9, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(21, Jun, 2016), Period::new(9, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(19, Sep, 2016), Period::new(9, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(20, Sep, 2016), Period::new(9, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(21, Sep, 2016), Period::new(9, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(19, Dec, 2016), Period::new(9, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(20, Dec, 2016), Period::new(9, Months)),
            (Date::new(20, Dec, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(21, Dec, 2016), Period::new(9, Months)),
            (Date::new(20, Dec, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(19, Mar, 2016), Period::new(1, Years)),
            (Date::new(21, Dec, 2015), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(20, Mar, 2016), Period::new(1, Years)),
            (Date::new(21, Dec, 2015), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(21, Mar, 2016), Period::new(1, Years)),
            (Date::new(21, Mar, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(19, Jun, 2016), Period::new(1, Years)),
            (Date::new(21, Mar, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(20, Jun, 2016), Period::new(1, Years)),
            (Date::new(20, Jun, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(21, Jun, 2016), Period::new(1, Years)),
            (Date::new(20, Jun, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(19, Sep, 2016), Period::new(1, Years)),
            (Date::new(20, Jun, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(20, Sep, 2016), Period::new(1, Years)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2017)),
        ),
        (
            (Date::new(21, Sep, 2016), Period::new(1, Years)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2017)),
        ),
        (
            (Date::new(19, Dec, 2016), Period::new(1, Years)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2017)),
        ),
        (
            (Date::new(20, Dec, 2016), Period::new(1, Years)),
            (Date::new(20, Dec, 2016), Date::new(20, Dec, 2017)),
        ),
        (
            (Date::new(21, Dec, 2016), Period::new(1, Years)),
            (Date::new(20, Dec, 2016), Date::new(20, Dec, 2017)),
        ),
        (
            (Date::new(19, Mar, 2016), Period::new(5, Years)),
            (Date::new(21, Dec, 2015), Date::new(20, Dec, 2020)),
        ),
        (
            (Date::new(20, Mar, 2016), Period::new(5, Years)),
            (Date::new(21, Dec, 2015), Date::new(20, Jun, 2021)),
        ),
        (
            (Date::new(21, Mar, 2016), Period::new(5, Years)),
            (Date::new(21, Mar, 2016), Date::new(20, Jun, 2021)),
        ),
        (
            (Date::new(19, Jun, 2016), Period::new(5, Years)),
            (Date::new(21, Mar, 2016), Date::new(20, Jun, 2021)),
        ),
        (
            (Date::new(20, Jun, 2016), Period::new(5, Years)),
            (Date::new(20, Jun, 2016), Date::new(20, Jun, 2021)),
        ),
        (
            (Date::new(21, Jun, 2016), Period::new(5, Years)),
            (Date::new(20, Jun, 2016), Date::new(20, Jun, 2021)),
        ),
        (
            (Date::new(19, Sep, 2016), Period::new(5, Years)),
            (Date::new(20, Jun, 2016), Date::new(20, Jun, 2021)),
        ),
        (
            (Date::new(20, Sep, 2016), Period::new(5, Years)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2021)),
        ),
        (
            (Date::new(21, Sep, 2016), Period::new(5, Years)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2021)),
        ),
        (
            (Date::new(19, Dec, 2016), Period::new(5, Years)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2021)),
        ),
        (
            (Date::new(20, Dec, 2016), Period::new(5, Years)),
            (Date::new(20, Dec, 2016), Date::new(20, Dec, 2021)),
        ),
        (
            (Date::new(21, Dec, 2016), Period::new(5, Years)),
            (Date::new(20, Dec, 2016), Date::new(20, Dec, 2021)),
        ),
        (
            (Date::new(20, Mar, 2016), Period::new(0, Months)),
            (Date::new(21, Dec, 2015), Date::new(20, Jun, 2016)),
        ),
        (
            (Date::new(21, Mar, 2016), Period::new(0, Months)),
            (Date::new(21, Mar, 2016), Date::new(20, Jun, 2016)),
        ),
        (
            (Date::new(19, Jun, 2016), Period::new(0, Months)),
            (Date::new(21, Mar, 2016), Date::new(20, Jun, 2016)),
        ),
        (
            (Date::new(20, Sep, 2016), Period::new(0, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(21, Sep, 2016), Period::new(0, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(19, Dec, 2016), Period::new(0, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2016)),
        ),
    ];

    let inputs: InputData = _inputs.into_iter().collect();
    test_cds_conventions(inputs, DateGenerationRule::CDS2015);
}

#[test]
fn test_cds_convention_grid() {
    // Testing against section 11 of ISDA doc FAQs Amending when Single Name CDS roll to new
    // on-the-run contracts December 20, 2015 Go-Live.
    // Amended the dates in the doc to the pre-2015 expected maturity dates.
    //
    // Test inputs and expected outputs
    // The map key: a pair with 1st element equal to trade date and 2nd element equal to CDS tenor.
    // The map value: a pair with 1st and 2nd element equal to expected start and end date
    // respectively.
    // The trade dates are from the transition dates in the doc i.e. 20th Mar, Jun, Sep and Dec
    // in 2016 and a day  either side. The tenors are selected tenors from the doc
    // i.e. short quarterly tenors less than 1Y, 1Y and 5Y.
    let _inputs = vec![
        (
            (Date::new(19, Mar, 2016), Period::new(3, Months)),
            (Date::new(21, Dec, 2015), Date::new(20, Jun, 2016)),
        ),
        (
            (Date::new(20, Mar, 2016), Period::new(3, Months)),
            (Date::new(21, Dec, 2015), Date::new(20, Sep, 2016)),
        ),
        (
            (Date::new(21, Mar, 2016), Period::new(3, Months)),
            (Date::new(21, Mar, 2016), Date::new(20, Sep, 2016)),
        ),
        (
            (Date::new(19, Jun, 2016), Period::new(3, Months)),
            (Date::new(21, Mar, 2016), Date::new(20, Sep, 2016)),
        ),
        (
            (Date::new(20, Jun, 2016), Period::new(3, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(21, Jun, 2016), Period::new(3, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(19, Sep, 2016), Period::new(3, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(20, Sep, 2016), Period::new(3, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(21, Sep, 2016), Period::new(3, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(19, Dec, 2016), Period::new(3, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(20, Dec, 2016), Period::new(3, Months)),
            (Date::new(20, Dec, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(21, Dec, 2016), Period::new(3, Months)),
            (Date::new(20, Dec, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(19, Mar, 2016), Period::new(6, Months)),
            (Date::new(21, Dec, 2015), Date::new(20, Sep, 2016)),
        ),
        (
            (Date::new(20, Mar, 2016), Period::new(6, Months)),
            (Date::new(21, Dec, 2015), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(21, Mar, 2016), Period::new(6, Months)),
            (Date::new(21, Mar, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(19, Jun, 2016), Period::new(6, Months)),
            (Date::new(21, Mar, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(20, Jun, 2016), Period::new(6, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(21, Jun, 2016), Period::new(6, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(19, Sep, 2016), Period::new(6, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(20, Sep, 2016), Period::new(6, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(21, Sep, 2016), Period::new(6, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(19, Dec, 2016), Period::new(6, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(20, Dec, 2016), Period::new(6, Months)),
            (Date::new(20, Dec, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(21, Dec, 2016), Period::new(6, Months)),
            (Date::new(20, Dec, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(19, Mar, 2016), Period::new(9, Months)),
            (Date::new(21, Dec, 2015), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(20, Mar, 2016), Period::new(9, Months)),
            (Date::new(21, Dec, 2015), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(21, Mar, 2016), Period::new(9, Months)),
            (Date::new(21, Mar, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(19, Jun, 2016), Period::new(9, Months)),
            (Date::new(21, Mar, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(20, Jun, 2016), Period::new(9, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(21, Jun, 2016), Period::new(9, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(19, Sep, 2016), Period::new(9, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(20, Sep, 2016), Period::new(9, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(21, Sep, 2016), Period::new(9, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(19, Dec, 2016), Period::new(9, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(20, Dec, 2016), Period::new(9, Months)),
            (Date::new(20, Dec, 2016), Date::new(20, Dec, 2017)),
        ),
        (
            (Date::new(21, Dec, 2016), Period::new(9, Months)),
            (Date::new(20, Dec, 2016), Date::new(20, Dec, 2017)),
        ),
        (
            (Date::new(19, Mar, 2016), Period::new(1, Years)),
            (Date::new(21, Dec, 2015), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(20, Mar, 2016), Period::new(1, Years)),
            (Date::new(21, Dec, 2015), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(21, Mar, 2016), Period::new(1, Years)),
            (Date::new(21, Mar, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(19, Jun, 2016), Period::new(1, Years)),
            (Date::new(21, Mar, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(20, Jun, 2016), Period::new(1, Years)),
            (Date::new(20, Jun, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(21, Jun, 2016), Period::new(1, Years)),
            (Date::new(20, Jun, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(19, Sep, 2016), Period::new(1, Years)),
            (Date::new(20, Jun, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(20, Sep, 2016), Period::new(1, Years)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2017)),
        ),
        (
            (Date::new(21, Sep, 2016), Period::new(1, Years)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2017)),
        ),
        (
            (Date::new(19, Dec, 2016), Period::new(1, Years)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2017)),
        ),
        (
            (Date::new(20, Dec, 2016), Period::new(1, Years)),
            (Date::new(20, Dec, 2016), Date::new(20, Mar, 2018)),
        ),
        (
            (Date::new(21, Dec, 2016), Period::new(1, Years)),
            (Date::new(20, Dec, 2016), Date::new(20, Mar, 2018)),
        ),
        (
            (Date::new(19, Mar, 2016), Period::new(5, Years)),
            (Date::new(21, Dec, 2015), Date::new(20, Mar, 2021)),
        ),
        (
            (Date::new(20, Mar, 2016), Period::new(5, Years)),
            (Date::new(21, Dec, 2015), Date::new(20, Jun, 2021)),
        ),
        (
            (Date::new(21, Mar, 2016), Period::new(5, Years)),
            (Date::new(21, Mar, 2016), Date::new(20, Jun, 2021)),
        ),
        (
            (Date::new(19, Jun, 2016), Period::new(5, Years)),
            (Date::new(21, Mar, 2016), Date::new(20, Jun, 2021)),
        ),
        (
            (Date::new(20, Jun, 2016), Period::new(5, Years)),
            (Date::new(20, Jun, 2016), Date::new(20, Sep, 2021)),
        ),
        (
            (Date::new(21, Jun, 2016), Period::new(5, Years)),
            (Date::new(20, Jun, 2016), Date::new(20, Sep, 2021)),
        ),
        (
            (Date::new(19, Sep, 2016), Period::new(5, Years)),
            (Date::new(20, Jun, 2016), Date::new(20, Sep, 2021)),
        ),
        (
            (Date::new(20, Sep, 2016), Period::new(5, Years)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2021)),
        ),
        (
            (Date::new(21, Sep, 2016), Period::new(5, Years)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2021)),
        ),
        (
            (Date::new(19, Dec, 2016), Period::new(5, Years)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2021)),
        ),
        (
            (Date::new(20, Dec, 2016), Period::new(5, Years)),
            (Date::new(20, Dec, 2016), Date::new(20, Mar, 2022)),
        ),
        (
            (Date::new(21, Dec, 2016), Period::new(5, Years)),
            (Date::new(20, Dec, 2016), Date::new(20, Mar, 2022)),
        ),
        (
            (Date::new(19, Mar, 2016), Period::new(0, Months)),
            (Date::new(21, Dec, 2015), Date::new(20, Mar, 2016)),
        ),
        (
            (Date::new(20, Mar, 2016), Period::new(0, Months)),
            (Date::new(21, Dec, 2015), Date::new(20, Jun, 2016)),
        ),
        (
            (Date::new(21, Mar, 2016), Period::new(0, Months)),
            (Date::new(21, Mar, 2016), Date::new(20, Jun, 2016)),
        ),
        (
            (Date::new(19, Jun, 2016), Period::new(0, Months)),
            (Date::new(21, Mar, 2016), Date::new(20, Jun, 2016)),
        ),
        (
            (Date::new(20, Jun, 2016), Period::new(0, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Sep, 2016)),
        ),
        (
            (Date::new(21, Jun, 2016), Period::new(0, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Sep, 2016)),
        ),
        (
            (Date::new(19, Sep, 2016), Period::new(0, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Sep, 2016)),
        ),
        (
            (Date::new(20, Sep, 2016), Period::new(0, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(21, Sep, 2016), Period::new(0, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(19, Dec, 2016), Period::new(0, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(20, Dec, 2016), Period::new(0, Months)),
            (Date::new(20, Dec, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(21, Dec, 2016), Period::new(0, Months)),
            (Date::new(20, Dec, 2016), Date::new(20, Mar, 2017)),
        ),
    ];

    let inputs: InputData = _inputs.into_iter().collect();
    test_cds_conventions(inputs, DateGenerationRule::CDS);
}

#[test]
fn test_old_cds_convention_grid() {
    // Testing against section 11 of ISDA doc FAQs Amending when Single Name CDS roll to
    // new on-the-run contracts December 20, 2015 Go-Live. Amended the dates in the doc to the
    // pre-2009 expected start and maturity dates.
    // Test inputs and expected outputs
    // The map key: a pair with 1st element equal to trade date and 2nd element equal to CDS tenor.
    // The map value: a pair with 1st and 2nd element equal to expected start and end date
    // respectively.
    // The trade dates are from the transition dates in the doc i.e. 20th Mar, Jun, Sep and Dec
    // in 2016 and a day either side. The tenors are selected tenors from the doc i.e. short
    // quarterly tenors less than 1Y, 1Y and 5Y.
    let _inputs = vec![
        (
            (Date::new(19, Mar, 2016), Period::new(3, Months)),
            (Date::new(19, Mar, 2016), Date::new(20, Jun, 2016)),
        ),
        (
            (Date::new(20, Mar, 2016), Period::new(3, Months)),
            (Date::new(20, Mar, 2016), Date::new(20, Sep, 2016)),
        ),
        (
            (Date::new(21, Mar, 2016), Period::new(3, Months)),
            (Date::new(21, Mar, 2016), Date::new(20, Sep, 2016)),
        ),
        (
            (Date::new(19, Jun, 2016), Period::new(3, Months)),
            (Date::new(19, Jun, 2016), Date::new(20, Sep, 2016)),
        ),
        (
            (Date::new(20, Jun, 2016), Period::new(3, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(21, Jun, 2016), Period::new(3, Months)),
            (Date::new(21, Jun, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(19, Sep, 2016), Period::new(3, Months)),
            (Date::new(19, Sep, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(20, Sep, 2016), Period::new(3, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(21, Sep, 2016), Period::new(3, Months)),
            (Date::new(21, Sep, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(19, Dec, 2016), Period::new(3, Months)),
            (Date::new(19, Dec, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(20, Dec, 2016), Period::new(3, Months)),
            (Date::new(20, Dec, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(21, Dec, 2016), Period::new(3, Months)),
            (Date::new(21, Dec, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(19, Mar, 2016), Period::new(6, Months)),
            (Date::new(19, Mar, 2016), Date::new(20, Sep, 2016)),
        ),
        (
            (Date::new(20, Mar, 2016), Period::new(6, Months)),
            (Date::new(20, Mar, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(21, Mar, 2016), Period::new(6, Months)),
            (Date::new(21, Mar, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(19, Jun, 2016), Period::new(6, Months)),
            (Date::new(19, Jun, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(20, Jun, 2016), Period::new(6, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(21, Jun, 2016), Period::new(6, Months)),
            (Date::new(21, Jun, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(19, Sep, 2016), Period::new(6, Months)),
            (Date::new(19, Sep, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(20, Sep, 2016), Period::new(6, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(21, Sep, 2016), Period::new(6, Months)),
            (Date::new(21, Sep, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(19, Dec, 2016), Period::new(6, Months)),
            (Date::new(19, Dec, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(20, Dec, 2016), Period::new(6, Months)),
            (Date::new(20, Dec, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(21, Dec, 2016), Period::new(6, Months)),
            (Date::new(21, Dec, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(19, Mar, 2016), Period::new(9, Months)),
            (Date::new(19, Mar, 2016), Date::new(20, Dec, 2016)),
        ),
        (
            (Date::new(20, Mar, 2016), Period::new(9, Months)),
            (Date::new(20, Mar, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(21, Mar, 2016), Period::new(9, Months)),
            (Date::new(21, Mar, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(19, Jun, 2016), Period::new(9, Months)),
            (Date::new(19, Jun, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(20, Jun, 2016), Period::new(9, Months)),
            (Date::new(20, Jun, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(21, Jun, 2016), Period::new(9, Months)),
            (Date::new(21, Jun, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(19, Sep, 2016), Period::new(9, Months)),
            (Date::new(19, Sep, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(20, Sep, 2016), Period::new(9, Months)),
            (Date::new(20, Sep, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(21, Sep, 2016), Period::new(9, Months)),
            (Date::new(21, Sep, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(19, Dec, 2016), Period::new(9, Months)),
            (Date::new(19, Dec, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(20, Dec, 2016), Period::new(9, Months)),
            (Date::new(20, Dec, 2016), Date::new(20, Dec, 2017)),
        ),
        (
            (Date::new(21, Dec, 2016), Period::new(9, Months)),
            (Date::new(21, Dec, 2016), Date::new(20, Dec, 2017)),
        ),
        (
            (Date::new(19, Mar, 2016), Period::new(1, Years)),
            (Date::new(19, Mar, 2016), Date::new(20, Mar, 2017)),
        ),
        (
            (Date::new(20, Mar, 2016), Period::new(1, Years)),
            (Date::new(20, Mar, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(21, Mar, 2016), Period::new(1, Years)),
            (Date::new(21, Mar, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(19, Jun, 2016), Period::new(1, Years)),
            (Date::new(19, Jun, 2016), Date::new(20, Jun, 2017)),
        ),
        (
            (Date::new(20, Jun, 2016), Period::new(1, Years)),
            (Date::new(20, Jun, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(21, Jun, 2016), Period::new(1, Years)),
            (Date::new(21, Jun, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(19, Sep, 2016), Period::new(1, Years)),
            (Date::new(19, Sep, 2016), Date::new(20, Sep, 2017)),
        ),
        (
            (Date::new(20, Sep, 2016), Period::new(1, Years)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2017)),
        ),
        (
            (Date::new(21, Sep, 2016), Period::new(1, Years)),
            (Date::new(21, Sep, 2016), Date::new(20, Dec, 2017)),
        ),
        (
            (Date::new(19, Dec, 2016), Period::new(1, Years)),
            (Date::new(19, Dec, 2016), Date::new(20, Dec, 2017)),
        ),
        (
            (Date::new(20, Dec, 2016), Period::new(1, Years)),
            (Date::new(20, Dec, 2016), Date::new(20, Mar, 2018)),
        ),
        (
            (Date::new(21, Dec, 2016), Period::new(1, Years)),
            (Date::new(21, Dec, 2016), Date::new(20, Mar, 2018)),
        ),
        (
            (Date::new(19, Mar, 2016), Period::new(5, Years)),
            (Date::new(19, Mar, 2016), Date::new(20, Mar, 2021)),
        ),
        (
            (Date::new(20, Mar, 2016), Period::new(5, Years)),
            (Date::new(20, Mar, 2016), Date::new(20, Jun, 2021)),
        ),
        (
            (Date::new(21, Mar, 2016), Period::new(5, Years)),
            (Date::new(21, Mar, 2016), Date::new(20, Jun, 2021)),
        ),
        (
            (Date::new(19, Jun, 2016), Period::new(5, Years)),
            (Date::new(19, Jun, 2016), Date::new(20, Jun, 2021)),
        ),
        (
            (Date::new(20, Jun, 2016), Period::new(5, Years)),
            (Date::new(20, Jun, 2016), Date::new(20, Sep, 2021)),
        ),
        (
            (Date::new(21, Jun, 2016), Period::new(5, Years)),
            (Date::new(21, Jun, 2016), Date::new(20, Sep, 2021)),
        ),
        (
            (Date::new(19, Sep, 2016), Period::new(5, Years)),
            (Date::new(19, Sep, 2016), Date::new(20, Sep, 2021)),
        ),
        (
            (Date::new(20, Sep, 2016), Period::new(5, Years)),
            (Date::new(20, Sep, 2016), Date::new(20, Dec, 2021)),
        ),
        (
            (Date::new(21, Sep, 2016), Period::new(5, Years)),
            (Date::new(21, Sep, 2016), Date::new(20, Dec, 2021)),
        ),
        (
            (Date::new(19, Dec, 2016), Period::new(5, Years)),
            (Date::new(19, Dec, 2016), Date::new(20, Dec, 2021)),
        ),
        (
            (Date::new(20, Dec, 2016), Period::new(5, Years)),
            (Date::new(20, Dec, 2016), Date::new(20, Mar, 2022)),
        ),
        (
            (Date::new(21, Dec, 2016), Period::new(5, Years)),
            (Date::new(21, Dec, 2016), Date::new(20, Mar, 2022)),
        ),
    ];

    let inputs: InputData = _inputs.into_iter().collect();
    test_cds_conventions(inputs, DateGenerationRule::OldCDS);
}

#[test]
fn test_cds_2015_convention_sample_dates() {
    // Testing all dates in sample CDS schedule(s) for rule CDS2015...
    let rule = DateGenerationRule::CDS2015;
    let tenor = Period::new(1, Years);

    // trade date = Fri 18 Sep 2015.
    let trade_date = Date::new(18, Sep, 2015);
    let maturity = cds_maturity(&trade_date, tenor, rule);
    let s = make_cds_schedule(trade_date, maturity, rule);
    let mut expected = vec![
        Date::new(22, Jun, 2015),
        Date::new(21, Sep, 2015),
        Date::new(21, Dec, 2015),
        Date::new(21, Mar, 2016),
        Date::new(20, Jun, 2016),
    ];
    check_dates(&s, &expected);

    // trade date = Sat 19 Sep 2015, no change.
    let trade_date = Date::new(19, Sep, 2015);
    let maturity = cds_maturity(&trade_date, tenor, rule);
    let s = make_cds_schedule(trade_date, maturity, rule);
    check_dates(&s, &expected);

    // trade date = Sun 20 Sep 2015. Roll to new maturity. Trade date still before next coupon
    // payment date of Mon 21 Sep 2015, so keep the first period from 22 Jun 2015 to 21 Sep 2015
    // in schedule.
    let trade_date = Date::new(20, Sep, 2015);
    let maturity = cds_maturity(&trade_date, tenor, rule);
    let s = make_cds_schedule(trade_date, maturity, rule);
    expected.push(Date::new(20, Sep, 2016));
    expected.push(Date::new(20, Dec, 2016));
    check_dates(&s, &expected);

    // trade date = Mon 21 Sep 2015, first period drops out of schedule.
    let trade_date = Date::new(21, Sep, 2015);
    let maturity = cds_maturity(&trade_date, tenor, rule);
    let s = make_cds_schedule(trade_date, maturity, rule);
    expected.remove(0);
    check_dates(&s, &expected);

    // Another sample trade date, Sat 20 Jun 2009.
    let trade_date = Date::new(20, Jun, 2009);
    let maturity = Date::new(20, Dec, 2009);
    let s = make_cds_schedule(trade_date, maturity, rule);
    let mut expected = vec![
        Date::new(20, Mar, 2009),
        Date::new(22, Jun, 2009),
        Date::new(21, Sep, 2009),
        Date::new(20, Dec, 2009),
    ];
    check_dates(&s, &expected);

    // Move forward to Sun 21 Jun 2009
    let trade_date = Date::new(21, Jun, 2009);
    let s = make_cds_schedule(trade_date, maturity, rule);
    check_dates(&s, &expected);

    // Move forward to Mon 22 Jun 2009
    let trade_date = Date::new(22, Jun, 2009);
    let s = make_cds_schedule(trade_date, maturity, rule);
    expected.remove(0);
    check_dates(&s, &expected);
}

#[test]
fn test_cds_convention_sample_dates() {
    // Testing all dates in sample CDS schedule(s) for rule CDS...
    let rule = DateGenerationRule::CDS;
    let tenor = Period::new(1, Years);

    // trade date = Fri 18 Sep 2015.
    let trade_date = Date::new(18, Sep, 2015);
    let maturity = cds_maturity(&trade_date, tenor, rule);
    let s = make_cds_schedule(trade_date, maturity, rule);
    let mut expected = vec![
        Date::new(22, Jun, 2015),
        Date::new(21, Sep, 2015),
        Date::new(21, Dec, 2015),
        Date::new(21, Mar, 2016),
        Date::new(20, Jun, 2016),
        Date::new(20, Sep, 2016),
    ];
    check_dates(&s, &expected);

    // trade date = Sat 19 Sep 2015, no change.
    let trade_date = Date::new(19, Sep, 2015);
    let maturity = cds_maturity(&trade_date, tenor, rule);
    let s = make_cds_schedule(trade_date, maturity, rule);
    check_dates(&s, &expected);

    // trade date = Sun 20 Sep 2015. Roll to new maturity. Trade date still before next coupon
    // payment date of Mon 21 Sep 2015, so keep the first period from 22 Jun 2015 to 21 Sep 2015
    // in schedule.
    let trade_date = Date::new(20, Sep, 2015);
    let maturity = cds_maturity(&trade_date, tenor, rule);
    let s = make_cds_schedule(trade_date, maturity, rule);
    expected.push(Date::new(20, December, 2016));
    check_dates(&s, &expected);

    // trade date = Mon 21 Sep 2015, first period drops out of schedule.
    let trade_date = Date::new(21, Sep, 2015);
    let maturity = cds_maturity(&trade_date, tenor, rule);
    let s = make_cds_schedule(trade_date, maturity, rule);
    expected.remove(0);
    check_dates(&s, &expected);

    // Another sample trade date, Sat 20 Jun 2009.
    let trade_date = Date::new(20, Jun, 2009);
    let maturity = Date::new(20, Dec, 2009);
    let s = make_cds_schedule(trade_date, maturity, rule);
    let mut expected = vec![
        Date::new(20, Mar, 2009),
        Date::new(22, Jun, 2009),
        Date::new(21, Sep, 2009),
        Date::new(20, Dec, 2009),
    ];
    check_dates(&s, &expected);

    // Move forward to Sun 21 Jun 2009
    let trade_date = Date::new(21, Jun, 2009);
    let s = make_cds_schedule(trade_date, maturity, rule);
    check_dates(&s, &expected);

    // Move forward to Mon 22 Jun 2009
    let trade_date = Date::new(22, Jun, 2009);
    let s = make_cds_schedule(trade_date, maturity, rule);
    expected.remove(0);
    check_dates(&s, &expected);
}

#[test]
fn test_old_cds_convention_sample_dates() {
    // Testing all dates in sample CDS schedule(s) for rule OldCDS...
    let rule = DateGenerationRule::OldCDS;
    let tenor = Period::new(1, Years);

    // trade date plus 1D = Fri 18 Sep 201
    let trade_date_plus_one = Date::new(18, Sep, 2015);
    let maturity = cds_maturity(&trade_date_plus_one, tenor, rule);
    let s = make_cds_schedule(trade_date_plus_one, maturity, rule);
    let mut expected = vec![
        Date::new(18, Sep, 2015),
        Date::new(21, Dec, 2015),
        Date::new(21, Mar, 2016),
        Date::new(20, Jun, 2016),
        Date::new(20, Sep, 2016),
    ];
    check_dates(&s, &expected);

    // trade date plus 1D = Sat 19 Sep 2015, no change.
    // OldCDS, schedule start date is not adjusted (kept this).
    let trade_date_plus_one = Date::new(19, Sep, 2015);
    expected[0] = trade_date_plus_one;
    let maturity = cds_maturity(&trade_date_plus_one, tenor, rule);
    let s = make_cds_schedule(trade_date_plus_one, maturity, rule);
    check_dates(&s, &expected);

    // trade date plus 1D = Sun 20 Sep 2015, roll.
    let trade_date_plus_one = Date::new(20, Sep, 2015);
    expected[0] = trade_date_plus_one;
    let maturity = cds_maturity(&trade_date_plus_one, tenor, rule);
    let s = make_cds_schedule(trade_date_plus_one, maturity, rule);
    expected.push(Date::new(20, Dec, 2016));
    check_dates(&s, &expected);

    // trade date plus 1D = Mon 21 Sep 2015, no change.
    let trade_date_plus_one = Date::new(21, Sep, 2015);
    expected[0] = trade_date_plus_one;
    let maturity = cds_maturity(&trade_date_plus_one, tenor, rule);
    let s = make_cds_schedule(trade_date_plus_one, maturity, rule);
    check_dates(&s, &expected);

    // Check the 30 day stub rule by moving closer to the first coupon payment date of
    // Mon 21 Dec 2015.
    // The test here requires long first stub when trade date plus 1D = 21 Nov 2015.
    // The condition in the schedule generation code is if: effective date + 30D > next 20th
    // _unadjusted_. Not sure if we should refer to the actual coupon payment date here
    // i.e. the next 20th _adjusted_ when making the decision.
    //
    // 19 Nov 2015 + 30D = 19 Dec 2015 <= 20 Dec 2015 => short front stub.
    let trade_date_plus_one = Date::new(19, Nov, 2015);
    expected[0] = trade_date_plus_one;
    let s = make_cds_schedule(trade_date_plus_one, maturity, rule);
    check_dates(&s, &expected);

    // 20 Nov 2015 + 30D = 20 Dec 2015 <= 20 Dec 2015 => short front stub.
    let trade_date_plus_one = Date::new(20, Nov, 2015);
    expected[0] = trade_date_plus_one;
    let s = make_cds_schedule(trade_date_plus_one, maturity, rule);
    check_dates(&s, &expected);

    // 21 Nov 2015 + 30D = 21 Dec 2015 > 20 Dec 2015 => long front stub.
    // Note that if we reffered to the next coupon payment date of 21 Dec 2015, it would
    // still be short front.
    let trade_date_plus_one = Date::new(21, Nov, 2015);
    expected[0] = trade_date_plus_one;
    let s = make_cds_schedule(trade_date_plus_one, maturity, rule);
    expected.remove(1);
    check_dates(&s, &expected);
}

#[test]
fn test_cds2015_zero_months_matured() {
    // Testing 0M tenor for CDS2015 where matured...
    let rule = DateGenerationRule::CDS2015;
    let tenor = Period::new(0, Months);

    // Move through selected trade dates from 20 Dec 2015 to 20 Dec 2016 checking that the 0M CDS
    // is matured.
    let inputs = vec![
        Date::new(20, December, 2015),
        Date::new(15, February, 2016),
        Date::new(19, March, 2016),
        Date::new(20, June, 2016),
        Date::new(15, August, 2016),
        Date::new(19, September, 2016),
        Date::new(20, December, 2016),
    ];
    for d in inputs {
        assert_eq!(cds_maturity(&d, tenor, rule), Date::default());
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused)]
type InputData = HashMap<(Date, Period), (Date, Date)>;
#[allow(unused)]
fn test_cds_conventions(inputs: InputData, rule: DateGenerationRule) {
    // Test the generated start and end date against the expected start and end date.
    for ((from, tenor), (expected_start, expected_end)) in inputs {
        let maturity = cds_maturity(&from, tenor, rule);
        assert_eq!(
            maturity, expected_end,
            "Maturity {:?} != expected end date: {:?}",
            maturity, expected_end
        );
        let s = make_cds_schedule(from, maturity, rule);
        let start = s.start_date();
        let end = s.end_date();
        assert_eq!(
            start, &expected_start,
            "Start date {:?} != expected start: {:?}",
            start, expected_start
        );
        assert_eq!(
            end, &expected_end,
            "End date {:?} != expected end: {:?}",
            end, expected_end
        );
    }
}

fn make_cds_schedule(from: Date, to: Date, rule: DateGenerationRule) -> Schedule {
    ScheduleBuilder::new(
        pricing_context(),
        from,
        to,
        Period::new(3, Months),
        WeekendsOnly::new(),
    )
    .with_convention(BusinessDayConvention::Following)
    .with_termination_convention(BusinessDayConvention::Unadjusted)
    .with_rule(rule)
    .build()
}

fn check_dates(s: &Schedule, expected: &[Date]) {
    assert_eq!(
        s.size(),
        expected.len(),
        "expected {} dates, found {}",
        expected.len(),
        s.size()
    );
    for i in 0..expected.len() {
        let computed = s[i];
        assert_eq!(
            computed, expected[i],
            "expected {:?} at index {}, found {:?}",
            expected[i], i, computed
        );
    }
}

fn pricing_context() -> PricingContext {
    PricingContext {
        eval_date: Date::new(1, December, 2022),
    }
}
