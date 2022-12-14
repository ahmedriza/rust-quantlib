use std::collections::HashSet;

use crate::maths::rounding::RoundingType;

use crate::currencies::currency::{Currency, CurrencyData};

/// Canadian dollar. The ISO three-letter code is CAD; the numeric code is 124.
/// It is divided into 100 cents.
#[derive(Debug, PartialEq)]
pub struct CADCurrency {
    data: CurrencyData,
}

impl CADCurrency {
    pub fn new() -> CADCurrency {
        Self {
            data: CurrencyData::new(
                "Canadian dollar".to_string(),
                "CAD".to_string(),
                124,
                "Can$".to_string(),
                "".to_string(),
                100,
                RoundingType::none(),
                None,
                HashSet::new(),
            ),
        }
    }
}

impl Default for CADCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl Currency for CADCurrency {
    fn data(&self) -> &CurrencyData {
        &self.data
    }
}

// -------------------------------------------------------------------------------------------------

/// U.S. dollar. The ISO three-letter code is USD; the numeric code is 840.
/// It is divided into 100 cents.
#[derive(Debug, PartialEq)]
pub struct USDCurrency {
    data: CurrencyData,
}

impl USDCurrency {
    pub fn new() -> USDCurrency {
        Self {
            data: CurrencyData::new(
                "U.S. dollar".to_string(),
                "USD".to_string(),
                840,
                "$".to_string(),
                "\u{00a2}".to_string(),
                100,
                RoundingType::none(),
                None,
                HashSet::new(),
            ),
        }
    }
}

impl Default for USDCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl Currency for USDCurrency {
    fn data(&self) -> &CurrencyData {
        &self.data
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::currencies::{america::USDCurrency, currency::Currency};

    #[test]
    fn test_usd() {
        let c = USDCurrency::new();
        assert_eq!(c.fraction_symbol(), "\u{00a2}")
    }
}
