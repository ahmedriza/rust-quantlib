use crate::datetime::date::Date;

/// This structure holds contextual information necessary for pricing, such
/// as the evaluation date.
#[derive(Clone, Copy, Debug)]
pub struct PricingContext {
    pub eval_date: Date,
}

impl PricingContext {
    pub fn new(eval_date: Date) -> Self {
        PricingContext { eval_date }
    }
}

impl Default for PricingContext {
    fn default() -> Self {
        Self {
            eval_date: Date::todays_date(),
        }
    }
}
