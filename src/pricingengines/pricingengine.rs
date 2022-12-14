/// Arguments to the pricing engine
pub trait Arguments {}

/// Results from the pricing engine
pub trait Results {}

/// Pricing engine interface
pub trait PricingEngine {
    type A: Arguments;
    type R: Results;

    fn calculate(&self, arguments: Self::A) -> Self::R;
}

// -------------------------------------------------------------------------------------------------
