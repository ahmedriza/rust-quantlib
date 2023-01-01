use rust_quantlib::context::pricing_context::PricingContext;
use rust_quantlib::quotes::simplequote::SimpleQuote;
use rust_quantlib::datetime::{
    calendars::target::Target, date::Date,
    months::Month::*, timeunit::TimeUnit::*,
};

/// This example shows how to set up a term structure and then price some simple bonds.
#[allow(unused)]
pub fn main() {
    let calendar = Target::new();
    // must be a business day
    let settlement_date = Date::new(18, September, 2008);

    let fixing_days = 3;
    let settlement_days = 3;

    let todays_date =
        calendar.advance_by_days_with_following(settlement_date, -fixing_days, Days, false);
    // set evaluation date
    let pricing_context = PricingContext::new(todays_date);

    println!("Today: {:?}, {:?}", todays_date.weekday(), todays_date);
    println!(
        "Settlement date: {:?}, {:?}",
        settlement_date.weekday(),
        settlement_date
    );

    // Building of the bonds discounting yield curve

    // ZC rates for the short end
    let zc_3m_quote = 0.0096;
    let zc_6m_quote = 0.0145;
    let zc_1y_quote = 0.0194;

    let zc_3m_rate = SimpleQuote::new(zc_3m_quote);
    let zc_6m_rate = SimpleQuote::new(zc_6m_quote);
    let zc_1y_rate = SimpleQuote::new(zc_1y_quote);

    // TODO
}
