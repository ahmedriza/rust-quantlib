use std::sync::Arc;

use crate::{
    instruments::instrument::{Instrument, InstrumentResults},
    types::Real,
};
use crate::{quotes::quote::Quote, datetime::date::Date};

// Simple stock
pub struct Stock {
    pub quote: Arc<dyn Quote>,
}

impl Stock {
    pub fn new(quote: Arc<dyn Quote>) -> Self {
        Self { quote }
    }
}

impl Instrument for Stock {
    fn perform_calculations(&self) -> InstrumentResults {
        InstrumentResults {
            npv: self.quote.value(),
            error_estimate: Real::default(),
            valuation_date: Date::default(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use crate::quotes::simplequote::SimpleQuote;
    use crate::datetime::date::Date;
    use crate::types::Real;

    use crate::instruments::instrument::Instrument;

    use super::Stock;

    #[test]
    fn test_stock() {
        let quote = Arc::new(SimpleQuote::new(1.5));
        let stock = Stock::new(quote);
        let results = stock.calculate();
        assert_eq!(results.npv, 1.5);
        assert_eq!(results.error_estimate, Real::default());
        assert_eq!(results.valuation_date, Date::default());
    }
}
