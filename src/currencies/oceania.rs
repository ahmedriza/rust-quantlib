use std::collections::HashSet;

use crate::maths::rounding::Rounding;

use crate::currencies::currency::{Currency, CurrencyData};

/// Australian dollar. The ISO three-letter code is AUD; the numeric code is 36.
/// It is divided into 100 cents.
#[derive(Debug, PartialEq)]
pub struct AUDCurrency {
    data: CurrencyData,
}

impl AUDCurrency {
    pub fn new() -> AUDCurrency {
        Self {
            data: CurrencyData::new(
                "Australian dollar".to_string(),
                "AUD".to_string(),
                36,
                "A$".to_string(),
                "".to_string(),
                100,
                Rounding::none(),
                None,
                HashSet::new(),
            ),
        }
    }
}

impl Default for AUDCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl Currency for AUDCurrency {
    fn data(&self) -> &CurrencyData {
        &self.data
    }
}

// -------------------------------------------------------------------------------------------------

/// New Zealand dollar. The ISO three-letter code is NZD; the numeric code is 554.
/// It is divided into 100 cents.
#[derive(Debug, PartialEq)]
pub struct NZDCurrency {
    data: CurrencyData,
}

impl NZDCurrency {
    pub fn new() -> NZDCurrency {
        Self {
            data: CurrencyData::new(
                "New Zealand dollar".to_string(),
                "NZD".to_string(),
                554,
                "NZ$".to_string(),
                "".to_string(),
                100,
                Rounding::none(),
                None,
                HashSet::new(),
            ),
        }
    }
}

impl Default for NZDCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl Currency for NZDCurrency {
    fn data(&self) -> &CurrencyData {
        &self.data
    }
}
