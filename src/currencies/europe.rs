use std::collections::HashSet;

use crate::maths::rounding::Rounding;

use crate::currencies::currency::{Currency, CurrencyData};

/// Swiss Franc. The ISO three-letter code is CHF; the numeric code is 756.
/// It is divided into 100 cents.
#[derive(Debug, PartialEq)]
pub struct CHFCurrency {
    data: CurrencyData,
}

impl CHFCurrency {
    pub fn new() -> CHFCurrency {
        Self {
            data: CurrencyData::new(
                "Swiss franc".to_string(),
                "CHF".to_string(),
                756,
                "SwF".to_string(),
                "".to_string(),
                100,
                Rounding::none(),
                None,
                HashSet::new(),
            ),
        }
    }
}

impl Default for CHFCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl Currency for CHFCurrency {
    fn data(&self) -> &CurrencyData {
        &self.data
    }
}

// -------------------------------------------------------------------------------------------------

/// European Euro. The ISO three-letter code is EUR; the numeric code is 978.
/// It is divided into 100 cents.
#[derive(Debug, PartialEq)]
pub struct EURCurrency {
    data: CurrencyData,
}

impl EURCurrency {
    pub fn new() -> EURCurrency {
        Self {
            data: CurrencyData::new(
                "European Euro".to_string(),
                "EUR".to_string(),
                978,
                "\u{20ac}".to_string(),
                "".to_string(),
                100,
                Rounding::closest(2, 5),
                None,
                HashSet::new(),
            ),
        }
    }
}

impl Default for EURCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl Currency for EURCurrency {
    fn data(&self) -> &CurrencyData {
        &self.data
    }
}

// -------------------------------------------------------------------------------------------------

/// British pound sterling. The ISO three-letter code is GBP; the numeric code is 826.
/// It is divided into 100 pence.
#[derive(Debug, PartialEq)]
pub struct GBPCurrency {
    data: CurrencyData,
}

impl GBPCurrency {
    pub fn new() -> GBPCurrency {
        Self {
            data: CurrencyData::new(
                "British pound sterling".to_string(),
                "GBP".to_string(),
                826,
                "\u{00A3}".to_string(),
                "p".to_string(),
                100,
                Rounding::none(),
                None,
                HashSet::new(),
            ),
        }
    }
}

impl Default for GBPCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl Currency for GBPCurrency {
    fn data(&self) -> &CurrencyData {
        &self.data
    }
}

// -------------------------------------------------------------------------------------------------

/// Norwegian krone. The ISO three-letter code is NOK; the numeric code is 578.
/// It is divided into 100 øre.
#[derive(Debug, PartialEq)]
pub struct NOKCurrency {
    data: CurrencyData,
}

impl NOKCurrency {
    pub fn new() -> NOKCurrency {
        Self {
            data: CurrencyData::new(
                "Norwegian krone".to_string(),
                "NOK".to_string(),
                578,
                "NKr".to_string(),
                "".to_string(),
                100,
                Rounding::none(),
                None,
                HashSet::new(),
            ),
        }
    }
}

impl Default for NOKCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl Currency for NOKCurrency {
    fn data(&self) -> &CurrencyData {
        &self.data
    }
}

// -------------------------------------------------------------------------------------------------

/// Swedish krona. The ISO three-letter code is SEK; the numeric code is 752.
/// It is divided into 100 öre.
#[derive(Debug, PartialEq)]
pub struct SEKCurrency {
    data: CurrencyData,
}

impl SEKCurrency {
    pub fn new() -> SEKCurrency {
        Self {
            data: CurrencyData::new(
                "Swedish krona".to_string(),
                "SEK".to_string(),
                752,
                "kr".to_string(),
                "".to_string(),
                100,
                Rounding::none(),
                None,
                HashSet::new(),
            ),
        }
    }
}

impl Default for SEKCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl Currency for SEKCurrency {
    fn data(&self) -> &CurrencyData {
        &self.data
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::currencies::{currency::Currency, europe::EURCurrency};

    #[test]
    fn test_euro() {
        let c = EURCurrency::new();
        assert_eq!(c.symbol(), "\u{20ac}")
    }
}
