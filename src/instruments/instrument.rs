use crate::pricingengines::pricingengine::Results;
use crate::time::date::Date;
use crate::types::Real;

// -------------------------------------------------------------------------------------------------

pub struct InstrumentResults {
    pub npv: Real,
    pub error_estimate: Real,
    pub valuation_date: Date,
}

impl Results for InstrumentResults {}

// -------------------------------------------------------------------------------------------------

pub trait Instrument {
    fn calculate(&self) -> InstrumentResults {
        self.perform_calculations()
    }

    /// Returns the net present value of the instrument
    fn npv(&self) -> Real {
        let results = self.calculate();
        results.npv
    }

    /// In case a pricing engine is **not** used, this method must be overridden to perform
    /// the actual calculations and set any needed results. In case a pricing engine is used, the
    /// default implementation can be used.
    fn perform_calculations(&self) -> InstrumentResults;
}

// -------------------------------------------------------------------------------------------------
