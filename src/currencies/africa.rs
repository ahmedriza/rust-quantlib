use std::collections::HashSet;

use crate::maths::rounding::Rounding;

use crate::currencies::currency::{Currency, CurrencyData};

/// South-African rand. The ISO three-letter code is ZAR; the numeric code is 710.
/// It is divided into 100 cents.
#[derive(Debug, PartialEq)]
pub struct ZARCurrency {
    data: CurrencyData,
}

impl ZARCurrency {
    pub fn new() -> ZARCurrency {
        Self {
            data: CurrencyData::new(
                "South-African rand".to_string(),
                "ZAR".to_string(),
                710,
                "R".to_string(),
                "".to_string(),
                100,
                Rounding::none(),
                None,
                HashSet::new(),
            ),
        }
    }
}

impl Default for ZARCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl Currency for ZARCurrency {
    fn data(&self) -> &CurrencyData {
        &self.data
    }
}

// -------------------------------------------------------------------------------------------------
