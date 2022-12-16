use crate::types::{Decimal, Integer};

pub trait Rounding {
    /// Perform rounding
    fn round(&self, value: Decimal) -> Decimal;
}

#[derive(Clone, PartialEq, Eq)]
pub struct NoRounding {}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq)]
pub struct UpRounding {
    data: RoundingData,
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq)]
pub struct DownRounding {
    data: RoundingData,
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq)]
pub struct ClosestRounding {
    data: RoundingData,
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq)]
pub struct CeilingTruncation {
    data: RoundingData,
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq)]
pub struct FloorTruncation {
    data: RoundingData,
}

// -------------------------------------------------------------------------------------------------

/// Rounding methods.
///
/// The rounding methods follow the OMG specification available
/// at <http://www.omg.org/cgi-bin/doc?formal/00-06-29.pdf>.
///
/// The names of the Floor and Ceiling methods might be misleading. Check the provided reference.
///
/// Rounding methods.
///
/// The rounding methods follow the OMG specification available
/// at <http://www.omg.org/cgi-bin/doc?formal/00-06-29.pdf>.
///
/// The names of the Floor and Ceiling methods might be misleading. Check the provided reference.
///
/// TODO use [enum_dispatch](https://gitlab.com/antonok/enum_dispatch/-/blob/master/README.md)
/// to reduce boiler plate.
#[derive(Clone, PartialEq, Eq)]
pub enum RoundingType {
    /// Does not perform any rounding
    NoRounding(NoRounding),

    /// The first decimal place past the precision will be rounded up. This differs from the
    /// OMG rule which rounds up only if the decimal to be rounded is greater than or equal to
    /// the rounding digit
    UpRounding(UpRounding),

    /// All decimal places past the precision will be truncated
    DownRounding(DownRounding),

    /// The first decimal place past the precision will be rounded up if greater than or equal
    /// to the rounding digit; this corresponds to the OMG round-up rule.  When the rounding
    /// digit is 5, the result will be the one closest to the original number, hence the name.    
    ClosestRounding(ClosestRounding),

    /// Positive numbers will be rounded down and negative numbers will be rounded up using the
    /// OMG round up and round down rules    
    CeilingTruncation(CeilingTruncation),

    /// Positive numbers will be rounded up and negative numbers will be rounded down using the
    /// OMG round up and round down rules    
    FloorTruncation(FloorTruncation),
}

// -------------------------------------------------------------------------------------------------

impl RoundingType {
    /// Return an instance [RoundingType::NoRounding]
    pub fn none() -> RoundingType {
        RoundingType::NoRounding(NoRounding {})
    }

    /// Return an instance [RoundingType::UpRounding]
    pub fn up(precision: Integer, digit: Integer) -> RoundingType {
        RoundingType::UpRounding(UpRounding {
            data: RoundingData::new(precision, digit),
        })
    }

    /// Return an instance [RoundingType::DownRounding]    
    pub fn down(precision: Integer, digit: Integer) -> RoundingType {
        RoundingType::DownRounding(DownRounding {
            data: RoundingData::new(precision, digit),
        })
    }

    /// Return an instance [RoundingType::ClosestRounding]    
    pub fn closest(precision: Integer, digit: Integer) -> RoundingType {
        RoundingType::ClosestRounding(ClosestRounding {
            data: RoundingData::new(precision, digit),
        })
    }

    /// Return an instance [RoundingType::CeilingTruncation]        
    pub fn ceiling(precision: Integer, digit: Integer) -> RoundingType {
        RoundingType::CeilingTruncation(CeilingTruncation {
            data: RoundingData::new(precision, digit),
        })
    }

    /// Return an instance [RoundingType::FloorTruncation]            
    pub fn floor(precision: Integer, digit: Integer) -> RoundingType {
        RoundingType::FloorTruncation(FloorTruncation {
            data: RoundingData::new(precision, digit),
        })
    }
}

// -------------------------------------------------------------------------------------------------

impl Rounding for RoundingType {
    fn round(&self, value: Decimal) -> Decimal {
        match self {
            RoundingType::NoRounding(_) => value,
            RoundingType::UpRounding(r) => r.data.round(value, self),
            RoundingType::DownRounding(r) => r.data.round(value, self),
            RoundingType::ClosestRounding(r) => r.data.round(value, self),
            RoundingType::CeilingTruncation(r) => r.data.round(value, self),
            RoundingType::FloorTruncation(r) => r.data.round(value, self),
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
struct RoundingData {
    precision: Integer,
    digit: Integer,
}

impl RoundingData {
    fn new(precision: Integer, digit: Integer) -> Self {
        Self { precision, digit }
    }

    fn round(&self, value: Decimal, rounding: &RoundingType) -> Decimal {
        let mult = 10_f64.powf(self.precision as f64);
        let mut lvalue = value.abs() * mult;
        let mod_val = lvalue.fract();
        lvalue -= mod_val;

        match rounding {
            RoundingType::NoRounding(_) => {}
            RoundingType::UpRounding(_) => {
                if mod_val != 0.0 {
                    lvalue += 1.0;
                }
            }
            RoundingType::DownRounding(_) => {}
            RoundingType::ClosestRounding(_) => {
                if mod_val >= self.digit as f64 / 10.0 {
                    lvalue += 1.0;
                }
            }
            RoundingType::CeilingTruncation(_) => {
                if (value < 0.0) && mod_val >= self.digit as f64 / 10.0 {
                    lvalue += 1.0;
                }
            }
            RoundingType::FloorTruncation(_) => {
                if (value >= 0.0) && mod_val >= self.digit as f64 / 10.0 {
                    lvalue += 1.0;
                }
            }
        }
        if value < 0.0 {
            -(lvalue / mult)
        } else {
            lvalue / mult
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::types::{Decimal, Integer};

    use crate::maths::{
        comparison::close_n,
        rounding::{Rounding, RoundingType},
    };

    #[test]
    fn test_round__() {
        let value = 11.2;

        let rounding = RoundingType::up(0, 1);
        let r = rounding.round(value);
        assert_eq!(r, 12.0);

        let rounding = RoundingType::down(0, 1);
        let r = rounding.round(value);
        assert_eq!(r, 11.0);

        let rounding = RoundingType::closest(0, 1);
        let r = rounding.round(value);
        assert_eq!(r, 12.0);

        let rounding = RoundingType::ceiling(0, 1);
        let r = rounding.round(value);
        assert_eq!(r, 11.0);

        let rounding = RoundingType::floor(0, 1);
        let r = rounding.round(value);
        assert_eq!(r, 12.0);
    }

    #[test]
    fn test_round() {
        let value = 11.2;

        let rounding = RoundingType::up(0, 1);
        let r = rounding.round(value);
        assert_eq!(r, 12.0);

        let rounding = RoundingType::down(0, 1);
        let r = rounding.round(value);
        assert_eq!(r, 11.0);

        let rounding = RoundingType::closest(0, 1);
        let r = rounding.round(value);
        assert_eq!(r, 12.0);

        let rounding = RoundingType::ceiling(0, 1);
        let r = rounding.round(value);
        assert_eq!(r, 11.0);

        let rounding = RoundingType::floor(0, 1);
        let r = rounding.round(value);
        assert_eq!(r, 12.0);
    }

    #[test]
    fn test_closest() {
        let test_data = make_test_data();
        for i in test_data {
            let rouding = RoundingType::closest(i.precision, 5);
            let calculated = rouding.round(i.x);
            let expected = i.closest;
            assert!(
                close_n(calculated, expected, 1),
                "Original number: {}, expected: {}, calculated: {}",
                i.x,
                expected,
                calculated
            );
        }
    }

    #[test]
    fn test_up() {
        let test_data = make_test_data();
        for i in test_data {
            let rouding = RoundingType::up(i.precision, 5);
            let calculated = rouding.round(i.x);
            let expected = i.up;
            assert!(
                close_n(calculated, expected, 1),
                "Original number: {}, expected: {}, calculated: {}",
                i.x,
                expected,
                calculated
            );
        }
    }

    #[test]
    fn test_down() {
        let test_data = make_test_data();
        for i in test_data {
            let rouding = RoundingType::down(i.precision, 5);
            let calculated = rouding.round(i.x);
            let expected = i.down;
            assert!(
                close_n(calculated, expected, 1),
                "Original number: {}, expected: {}, calculated: {}",
                i.x,
                expected,
                calculated
            );
        }
    }

    #[test]
    fn test_floor() {
        let test_data = make_test_data();
        for i in test_data {
            let rouding = RoundingType::floor(i.precision, 5);
            let calculated = rouding.round(i.x);
            let expected = i.floor;
            assert!(
                close_n(calculated, expected, 1),
                "Original number: {}, expected: {}, calculated: {}",
                i.x,
                expected,
                calculated
            );
        }
    }

    #[test]
    fn test_ceiling() {
        let test_data = make_test_data();
        for i in test_data {
            let rouding = RoundingType::ceiling(i.precision, 5);
            let calculated = rouding.round(i.x);
            let expected = i.ceiling;
            assert!(
                close_n(calculated, expected, 1),
                "Original number: {}, expected: {}, calculated: {}",
                i.x,
                expected,
                calculated
            );
        }
    }

    struct TestCase {
        x: Decimal,
        precision: Integer,
        closest: Decimal,
        up: Decimal,
        down: Decimal,
        floor: Decimal,
        ceiling: Decimal,
    }

    impl TestCase {
        fn new(
            x: Decimal,
            precision: Integer,
            closest: Decimal,
            up: Decimal,
            down: Decimal,
            floor: Decimal,
            ceiling: Decimal,
        ) -> Self {
            Self {
                x,
                precision,
                closest,
                up,
                down,
                floor,
                ceiling,
            }
        }
    }

    fn make_test_data() -> Vec<TestCase> {
        vec![
            TestCase::new(0.86313513, 5, 0.86314, 0.86314, 0.86313, 0.86314, 0.86313),
            TestCase::new(0.86313, 5, 0.86313, 0.86313, 0.86313, 0.86313, 0.86313),
            TestCase::new(-7.64555346, 1, -7.6, -7.7, -7.6, -7.6, -7.6),
            TestCase::new(0.13961605, 2, 0.14, 0.14, 0.13, 0.14, 0.13),
            TestCase::new(0.14344179, 4, 0.1434, 0.1435, 0.1434, 0.1434, 0.1434),
            TestCase::new(-4.74315016, 2, -4.74, -4.75, -4.74, -4.74, -4.74),
            TestCase::new(
                -7.82772074,
                5,
                -7.82772,
                -7.82773,
                -7.82772,
                -7.82772,
                -7.82772,
            ),
            TestCase::new(2.74137947, 3, 2.741, 2.742, 2.741, 2.741, 2.741),
            TestCase::new(2.13056714, 1, 2.1, 2.2, 2.1, 2.1, 2.1),
            TestCase::new(-1.06228670, 1, -1.1, -1.1, -1.0, -1.0, -1.1),
            TestCase::new(8.29234094, 4, 8.2923, 8.2924, 8.2923, 8.2923, 8.2923),
            TestCase::new(7.90185598, 2, 7.90, 7.91, 7.90, 7.90, 7.90),
            TestCase::new(-0.26738058, 1, -0.3, -0.3, -0.2, -0.2, -0.3),
            TestCase::new(1.78128713, 1, 1.8, 1.8, 1.7, 1.8, 1.7),
            TestCase::new(4.23537260, 1, 4.2, 4.3, 4.2, 4.2, 4.2),
            TestCase::new(3.64369953, 4, 3.6437, 3.6437, 3.6436, 3.6437, 3.6436),
            TestCase::new(6.34542470, 2, 6.35, 6.35, 6.34, 6.35, 6.34),
            TestCase::new(-0.84754962, 4, -0.8475, -0.8476, -0.8475, -0.8475, -0.8475),
            TestCase::new(4.60998652, 1, 4.6, 4.7, 4.6, 4.6, 4.6),
            TestCase::new(6.28794223, 3, 6.288, 6.288, 6.287, 6.288, 6.287),
            TestCase::new(7.89428221, 2, 7.89, 7.90, 7.89, 7.89, 7.89),
        ]
    }
}
