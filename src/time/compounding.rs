#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Compounding {
    /// 1+rt
    Simple = 0,
    /// (1+r)^t
    Compounded = 1,
    /// e^{rt}
    Continuous = 2,
    /// Simple up to the first period then Compounded
    SimpleThenCompounded,
    /// Compounded up to the first period then Simple
    CompoundedThenSimple,
}
