use std::collections::HashSet;

use crate::maths::rounding::RoundingType;

use crate::currencies::currency::{Currency, CurrencyData};

/// Chinese yuan. The ISO three-letter code is CNY; the numeric code is 156.
/// It is divided into 100 fen.
#[derive(Debug, PartialEq)]
pub struct CNYCurrency {
    data: CurrencyData,
}

impl CNYCurrency {
    pub fn new() -> CNYCurrency {
        Self {
            data: CurrencyData::new(
                "Chinese yean".to_string(),
                "CNY".to_string(),
                156,
                "Y".to_string(),
                "".to_string(),
                100,
                RoundingType::none(),
                None,
                HashSet::new(),
            ),
        }
    }
}

impl Default for CNYCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl Currency for CNYCurrency {
    fn data(&self) -> &CurrencyData {
        &self.data
    }
}

// -------------------------------------------------------------------------------------------------

/// Hong Kong dollar. The ISO three-letter code is HKD; the numeric code is 344.
/// It is divided into 100 cents.
#[derive(Debug, PartialEq)]
pub struct HKDCurrency {
    data: CurrencyData,
}

impl HKDCurrency {
    pub fn new() -> HKDCurrency {
        Self {
            data: CurrencyData::new(
                "Hong Kong dollar".to_string(),
                "HKD".to_string(),
                344,
                "HK$".to_string(),
                "".to_string(),
                100,
                RoundingType::none(),
                None,
                HashSet::new(),
            ),
        }
    }
}

impl Default for HKDCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl Currency for HKDCurrency {
    fn data(&self) -> &CurrencyData {
        &self.data
    }
}

// -------------------------------------------------------------------------------------------------

/// Japanese yen. The ISO three-letter code is JPY; the numeric code is 392.
/// It is divided into 100 sen.
#[derive(Debug, PartialEq)]
pub struct JPYCurrency {
    data: CurrencyData,
}

impl JPYCurrency {
    pub fn new() -> JPYCurrency {
        Self {
            data: CurrencyData::new(
                "Japanese yen".to_string(),
                "JPY".to_string(),
                392,
                "\u{00A5}".to_string(),
                "".to_string(),
                100,
                RoundingType::none(),
                None,
                HashSet::new(),
            ),
        }
    }
}

impl Default for JPYCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl Currency for JPYCurrency {
    fn data(&self) -> &CurrencyData {
        &self.data
    }
}

// -------------------------------------------------------------------------------------------------

/// South-Korean won. The ISO three-letter code is KRW; the numeric code is 410.
/// It is divided into 100 chon.
#[derive(Debug, PartialEq)]
pub struct KRWCurrency {
    data: CurrencyData,
}

impl KRWCurrency {
    pub fn new() -> KRWCurrency {
        Self {
            data: CurrencyData::new(
                "South-Korean won".to_string(),
                "KRW".to_string(),
                410,
                "W".to_string(),
                "".to_string(),
                100,
                RoundingType::none(),
                None,
                HashSet::new(),
            ),
        }
    }
}

impl Default for KRWCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl Currency for KRWCurrency {
    fn data(&self) -> &CurrencyData {
        &self.data
    }
}

// -------------------------------------------------------------------------------------------------

/// Saudi riyal. The ISO three-letter code is SAR; the numeric code is 682.
/// It is divided into 100 halalat.
#[derive(Debug, PartialEq)]
pub struct SARCurrency {
    data: CurrencyData,
}

impl SARCurrency {
    pub fn new() -> SARCurrency {
        Self {
            data: CurrencyData::new(
                "Saudi riyal".to_string(),
                "SAR".to_string(),
                682,
                "SRls".to_string(),
                "".to_string(),
                100,
                RoundingType::none(),
                None,
                HashSet::new(),
            ),
        }
    }
}

impl Default for SARCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl Currency for SARCurrency {
    fn data(&self) -> &CurrencyData {
        &self.data
    }
}

// -------------------------------------------------------------------------------------------------

/// Singapore dollar. The ISO three-letter code is SGD; the numeric code is 702.
/// It is divided into 100 cents.
#[derive(Debug, PartialEq)]
pub struct SGDCurrency {
    data: CurrencyData,
}

impl SGDCurrency {
    pub fn new() -> SGDCurrency {
        Self {
            data: CurrencyData::new(
                "Singapore dollar".to_string(),
                "SGD".to_string(),
                702,
                "S$".to_string(),
                "".to_string(),
                100,
                RoundingType::none(),
                None,
                HashSet::new(),
            ),
        }
    }
}

impl Default for SGDCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl Currency for SGDCurrency {
    fn data(&self) -> &CurrencyData {
        &self.data
    }
}

// -------------------------------------------------------------------------------------------------

/// Chinese yuan (Hong Kong). The ISO three-letter code is CNH; there is no numeric code.
/// It is divided into 100 fen.
#[derive(Debug, PartialEq)]
pub struct CNHCurrency {
    data: CurrencyData,
}

impl CNHCurrency {
    pub fn new() -> CNHCurrency {
        Self {
            data: CurrencyData::new(
                "Chinese yuan (Hong Kong)".to_string(),
                "CNH".to_string(),
                156,
                "CNH".to_string(),
                "".to_string(),
                100,
                RoundingType::none(),
                None,
                HashSet::new(),
            ),
        }
    }
}

impl Default for CNHCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl Currency for CNHCurrency {
    fn data(&self) -> &CurrencyData {
        &self.data
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::currencies::{asia::JPYCurrency, currency::Currency};

    #[test]
    fn test_jpy() {
        let c = JPYCurrency::new();
        assert_eq!(c.symbol(), "\u{00a5}")
    }
}
