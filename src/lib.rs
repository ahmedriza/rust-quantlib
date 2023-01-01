#![allow(clippy::needless_doctest_main)]
//!
//! This is a pure Rust implementation of
//! [QuantLib](https://www.quantlib.org/).  We have tried to follow Rust idioms as much as possible.
//!
//! We plan to provide Python and R bindings to the library in due course.
//!
//! # Example
//!
//! The following demonstrates an example of using the library for pricing bonds. More examples
//! can be found in the [examples](https://github.com/ahmedriza/rust-quantlib/tree/master/examples)
//! directory of the source. We will add more examples there as work on the library progresses.
//!
//!
//! ```no_run
//! use rust_quantlib::context::pricing_context::PricingContext;
//! use rust_quantlib::quotes::simplequote::SimpleQuote;
//! use rust_quantlib::datetime::{
//!    holidays::target::Target, date::Date,
//!    months::Month::*, timeunit::TimeUnit::*,
//! };
//!
//! pub fn main() {
//!     let calendar = Target::new();
//!     // must be a business day
//!     let settlement_date = Date::new(18, September, 2008);
//!
//!     let fixing_days = 3;
//!     let settlement_days = 3;
//!
//!     let todays_date =
//!         calendar.advance_by_days_with_following(settlement_date, -fixing_days, Days, false);
//!     // set evaluation date. Note that this is not a global but a local context.
//!     let pricing_context = PricingContext::new(todays_date);
//!
//!     println!("Today: {:?}, {:?}", todays_date.weekday(), todays_date);
//!     println!(
//!         "Settlement date: {:?}, {:?}",
//!         settlement_date.weekday(),
//!         settlement_date
//!     );
//!
//!     // Building of the bonds discounting yield curve
//!
//!     // ZC rates for the short end
//!     let zc_3m_rate = SimpleQuote::new(0.0096);
//!     let zc_6m_rate = SimpleQuote::new(0.0145);
//!     let zc_1y_rate = SimpleQuote::new(0.0194);
//!
//!     // TODO
//!}
//!```
//!
pub mod cashflows;
pub mod context;
pub mod currencies;
pub mod datetime;
pub mod instruments;
pub mod maths;
pub mod misc;
pub mod pricingengines;
pub mod processes;
pub mod quotes;
pub mod rates;
pub mod termstructures;
pub mod types;
pub mod utils;
