use crate::{quotes::quote::Quote, types::Real};

/// Market element returning a stored value
#[derive(Default)]
pub struct SimpleQuote {
    pub value: Real,
}

impl SimpleQuote {
    pub fn new(value: Real) -> Self {
        SimpleQuote { value }
    }
}

impl Quote for SimpleQuote {
    fn value(&self) -> Real {
        self.value
    }

    fn is_valid(&self) -> bool {
        true
    }
}
