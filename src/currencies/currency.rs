use std::{collections::HashSet, fmt::Debug, rc::Rc};

use crate::maths::rounding::RoundingType;
use crate::types::Integer;

/// Currency specification
pub trait Currency: Debug {
    /// Currency name, e.g, "U.S. Dollar"    
    fn name(&self) -> &str {
        &self.data().name
    }

    /// ISO 4217 three-letter code, e.g, "USD"
    fn code(&self) -> &str {
        &self.data().code
    }

    /// ISO 4217 numeric code, e.g, "840"
    fn numeric_code(&self) -> Integer {
        self.data().numeric_code
    }

    /// Symbol, e.g, "$"
    fn symbol(&self) -> &str {
        &self.data().symbol
    }

    /// Fraction symbol, e.g, "¢"    
    fn fraction_symbol(&self) -> &str {
        &self.data().fraction_symbol
    }

    /// Number of fractionary parts in a unit, e.g, 100
    fn fractions_per_unit(&self) -> Integer {
        self.data().fractions_per_unit
    }

    /// Rounding convention
    fn rounding(&self) -> RoundingType {
        self.data().rounding.clone()
    }

    /// Currency used for triangulated exchange when required
    fn triangulation_currency(&self) -> Option<Rc<CurrencyData>> {
        self.data().triangulation_currency.clone()
    }

    /// Minor unit codes, e.g. GBp, GBX for GBP
    fn minor_unit_codes(&self) -> HashSet<String> {
        self.data().minor_unit_codes.clone()
    }

    fn data(&self) -> &CurrencyData;
}

// -------------------------------------------------------------------------------------------------

/// Data associated with a currency
#[derive(Clone)]
pub struct CurrencyData {
    /// Currency name, e.g, "U.S. Dollar"    
    pub name: String,

    /// ISO 4217 three-letter code, e.g, "USD"
    pub code: String,

    /// ISO 4217 numeric code, e.g, "840"
    pub numeric_code: Integer,

    /// Symbol, e.g, "$"
    pub symbol: String,

    /// Fraction symbol, e.g, "¢"
    pub fraction_symbol: String,

    /// Number of fractionary parts in a unit, e.g, 100
    pub fractions_per_unit: Integer,

    /// Rounding convention
    pub rounding: RoundingType,

    /// Currency used for triangulated exchange when required
    pub triangulation_currency: Option<Rc<CurrencyData>>,

    /// Minor unit codes, e.g. GBp, GBX for GBP
    pub minor_unit_codes: HashSet<String>,
}

impl Debug for CurrencyData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl PartialEq for CurrencyData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl CurrencyData {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        code: String,
        numeric_code: Integer,
        symbol: String,
        fraction_symbol: String,
        fractions_per_unit: Integer,
        rounding: RoundingType,
        triangulation_currency: Option<Rc<CurrencyData>>,
        minor_unit_codes: HashSet<String>,
    ) -> Self {
        Self {
            name,
            code,
            numeric_code,
            symbol,
            fraction_symbol,
            fractions_per_unit,
            rounding,
            triangulation_currency,
            minor_unit_codes,
        }
    }
}
