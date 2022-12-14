use std::rc::Rc;

use crate::types::Decimal;

use crate::currencies::currency::Currency;

/// Cash amount in a given currency
#[derive(Debug, Clone)]
pub struct Money {
    pub value: Decimal,
    pub currency: Rc<dyn Currency>,
}

impl Money {
    pub fn new(currency: Rc<dyn Currency>, value: Decimal) -> Self {
        Self {
            value,
            currency,
        }
    }
}
