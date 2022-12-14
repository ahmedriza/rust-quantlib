use std::{
    hash::Hash,
    ops::{AddAssign, Div, DivAssign, Mul, Neg, SubAssign},
};

use crate::types::{Integer, Real};

use crate::time::{
    frequency::Frequency, frequency::Frequency::*, timeunit::TimeUnit, timeunit::TimeUnit::*,
};

#[derive(Clone, Copy, Debug, Eq)]
/// Provides a period represented by a length and [TimeUnit].
pub struct Period {
    pub length: Integer,
    pub unit: TimeUnit,
}

// -------------------------------------------------------------------------------------------------

impl PartialEq for Period {
    fn eq(&self, other: &Self) -> bool {
        !(self < other || other < self)
    }
}

impl Hash for Period {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.length.hash(state);
        self.unit.hash(state);
    }
}

// -------------------------------------------------------------------------------------------------

impl PartialOrd for Period {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let p1 = self;
        let p2 = other;

        // special cases
        if p1.length == 0 {
            return 0.partial_cmp(&p2.length);
        }
        if p2.length == 0 {
            return 0.partial_cmp(&p1.length);
        }

        // exact comparisons
        if p1.unit == p2.unit {
            return p1.length.partial_cmp(&p2.length);
        }
        if p1.unit == Months && p2.unit == Years {
            return p1.length.partial_cmp(&(12 * p2.length));
        }
        if p1.unit == Years && p2.unit == Months {
            return (12 * p1.length).partial_cmp(&p2.length);
        }
        if p1.unit == Days && p2.unit == Weeks {
            return p1.length.partial_cmp(&(7 * p2.length));
        }
        if p1.unit == Weeks && p2.unit == Days {
            return (7 * p1.length).partial_cmp(&p2.length);
        }

        // inexact comparisons (handled by converting to days and using limits)
        let p1_lim = p1.days_min_max();
        let p2_lim = p2.days_min_max();
        p1_lim.partial_cmp(&p2_lim)
    }
}

// -------------------------------------------------------------------------------------------------

impl From<Frequency> for Period {
    fn from(f: Frequency) -> Self {
        let f_ordinal: Integer = f.into();
        match f {
            NoFrequency => Period::new(0, Days),
            Once => Period::new(0, Years),
            Annual => Period::new(1, Years),
            Semiannual => Period::new(12 / f_ordinal, Months),
            EveryFourthMonth => Period::new(12 / f_ordinal, Months),
            Quarterly => Period::new(12 / f_ordinal, Months),
            Bimonthly => Period::new(12 / f_ordinal, Months),
            Monthly => Period::new(12 / f_ordinal, Months),
            EveryFourthWeek => Period::new(52 / f_ordinal, Weeks),
            Biweekly => Period::new(52 / f_ordinal, Weeks),
            Weekly => Period::new(52 / f_ordinal, Weeks),
            Daily => Period::new(1, Days),
            OtherFrequency => panic!("Unknown frequency"),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Neg for Period {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            length: -self.length,
            unit: self.unit,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl AddAssign for Period {
    fn add_assign(&mut self, rhs: Self) {
        if self.length == 0 {
            self.length = rhs.length;
            self.unit = rhs.unit;
        } else if self.unit == rhs.unit {
            // no conversion needed
            self.length += rhs.length;
        } else {
            match self.unit {
                Years => match rhs.unit {
                    Months => {
                        self.unit = Months;
                        self.length = self.length * 12 + rhs.length;
                    }
                    Days | Weeks => {
                        assert!(
                            rhs.length == 0,
                            "impossible addition between {:?} and {:?}",
                            self,
                            rhs
                        )
                    }
                    other => panic!("Invalid time unit {:?}", other),
                },
                Months => match rhs.unit {
                    Years => {
                        self.length += rhs.length * 12;
                    }
                    Days | Weeks => {
                        assert!(
                            rhs.length == 0,
                            "impossible addition between {:?} and {:?}",
                            self,
                            rhs
                        )
                    }
                    other => panic!("Invalid time unit {:?}", other),
                },
                Weeks => match rhs.unit {
                    Days => {
                        self.unit = Days;
                        self.length = self.length * 7 + rhs.length;
                    }
                    Months | Years => {
                        assert!(
                            rhs.length == 0,
                            "impossible addition between {:?} and {:?}",
                            self,
                            rhs
                        )
                    }
                    other => panic!("Invalid time unit {:?}", other),
                },
                Days => match rhs.unit {
                    Weeks => {
                        self.length += rhs.length * 7;
                    }
                    Months | Years => {
                        assert!(
                            rhs.length == 0,
                            "impossible addition between {:?} and {:?}",
                            self,
                            rhs
                        )
                    }
                    other => panic!("Invalid time unit {:?}", other),
                },
                other => panic!("Invalid time unit {:?}", other),
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl SubAssign for Period {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

// -------------------------------------------------------------------------------------------------

impl DivAssign<Integer> for Period {
    fn div_assign(&mut self, n: Integer) {
        assert!(n != 0, "Cannot divide by 0");
        if self.length % n == 0 {
            // keep the original units. If the user created a
            // 24-months period, he'll probably want a 12-months one
            // when he halves it.
            self.length /= n;
        } else {
            // try
            let mut unit = self.unit;
            let mut length = self.length;
            match unit {
                Years => {
                    length *= 12;
                    unit = Months;
                }
                Weeks => {
                    length *= 7;
                    unit = Days;
                }
                _ => {}
            }
            assert!(length % n == 0, "{:?} cannot be divided by {}", self, n);
            self.length = length / n;
            self.unit = unit;
        }
    }
}

impl Div<Integer> for Period {
    type Output = Self;

    fn div(self, n: Integer) -> Self::Output {
        let mut result = self;
        result /= n;
        result
    }
}

// -------------------------------------------------------------------------------------------------

impl Mul<Integer> for Period {
    type Output = Self;

    fn mul(self, rhs: Integer) -> Self::Output {
        Self {
            length: rhs * self.length,
            unit: self.unit,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Period {
    /// Create a new Period from `length` and [TimeUnit] `units`.
    pub fn new(length: Integer, unit: TimeUnit) -> Self {
        Self { length, unit }
    }

    /// Return the [Frequency] that corresponds to this [Period].
    pub fn frequency(&self) -> Frequency {
        if self.length == 0 {
            if self.unit == Years {
                return Once;
            }
            return NoFrequency;
        }
        match self.unit {
            Days => {
                if self.length == 1 {
                    Daily
                } else {
                    OtherFrequency
                }
            }
            Weeks => {
                if self.length == 1 {
                    Weekly
                } else if self.length == 2 {
                    Biweekly
                } else if self.length == 4 {
                    EveryFourthWeek
                } else {
                    OtherFrequency
                }
            }
            Months => {
                if 12 % self.length == 0 && self.length <= 12 {
                    Frequency::from(12 / self.length)
                } else {
                    OtherFrequency
                }
            }
            Years => {
                if self.length == 1 {
                    Annual
                } else {
                    OtherFrequency
                }
            }
            other => panic!("Invalid timeunit: {:?}", other),
        }
    }

    /// Normalise length and units
    pub fn normalise(&mut self) {
        if self.length == 0 {
            self.unit = Days;
        } else {
            match self.unit {
                Months => {
                    if self.length % 12 == 0 {
                        self.length /= 12;
                        self.unit = Years;
                    }
                }
                Days => {
                    if self.length % 7 == 0 {
                        self.length /= 7;
                        self.unit = Weeks;
                    }
                }

                Weeks => {}
                Years => {}
                other => panic!("Invalid timeunit: {:?}", other),
            }
        }
    }

    /// Create a normalised copy of self
    pub fn normalised(&self) -> Period {
        let mut p = *self;
        p.normalise();
        p
    }

    /// Return the minimum and maximum number of days possible with this [Period].
    pub fn days_min_max(&self) -> (Integer, Integer) {
        match self.unit {
            Days => (self.length, self.length),
            Weeks => (7 * self.length, 7 * self.length),
            Months => (28 * self.length, 31 * self.length),
            Years => (365 * self.length, 366 * self.length),
            other => panic!("Invalid timeunit: {:?}", other),
        }
    }

    /// Return the number of years represented by this [Period].
    pub fn years(&self) -> Real {
        if self.length == 0 {
            return 0.0;
        }
        match self.unit {
            Days => panic!("Cannot convert days into years"),
            Weeks => panic!("Cannot convert weeks into years"),
            Months => self.length as Real / 12.0,
            Years => self.length as Real,
            other => panic!("Invalid timeunit: {:?}", other),
        }
    }

    /// Return the number of months represented by this [Period].
    pub fn months(&self) -> Real {
        if self.length == 0 {
            return 0.0;
        }
        match self.unit {
            Days => panic!("Cannot convert days into months"),
            Weeks => panic!("Cannot convert weeks into months"),
            Months => self.length as Real,
            Years => self.length as Real * 12.0,
            other => panic!("Invalid timeunit: {:?}", other),
        }
    }

    /// Return the number of weeks represented by this [Period].
    pub fn weeks(&self) -> Real {
        if self.length == 0 {
            return 0.0;
        }
        match self.unit {
            Days => self.length as Real / 7.0,
            Weeks => self.length as Real,
            Months => panic!("Cannot convert months into weeks"),
            Years => panic!("Cannot convert years into weeks"),
            other => panic!("Invalid timeunit: {:?}", other),
        }
    }

    /// Return the number of days represented by this [Period].
    pub fn days(&self) -> Real {
        if self.length == 0 {
            return 0.0;
        }
        match self.unit {
            Days => self.length as Real,
            Weeks => self.length as Real * 7.0,
            Months => panic!("Cannot convert months into days"),
            Years => panic!("Cannot convert years into days"),
            other => panic!("Invalid timeunit: {:?}", other),
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::time::{frequency::Frequency::*, timeunit::TimeUnit::*};

    use super::Period;

    #[test]
    fn test_from_frequency() {
        let p = Period::from(NoFrequency);
        assert_eq!(p.length, 0);
        assert_eq!(p.unit, Days);

        let p = Period::from(Once);
        assert_eq!(p.length, 0);
        assert_eq!(p.unit, Years);

        let p = Period::from(Annual);
        assert_eq!(p.length, 1);
        assert_eq!(p.unit, Years);

        let p = Period::from(Semiannual);
        assert_eq!(p.length, 6);
        assert_eq!(p.unit, Months);

        let p = Period::from(EveryFourthMonth);
        assert_eq!(p.length, 4);
        assert_eq!(p.unit, Months);

        let p = Period::from(Quarterly);
        assert_eq!(p.length, 3);
        assert_eq!(p.unit, Months);

        let p = Period::from(Bimonthly);
        assert_eq!(p.length, 2);
        assert_eq!(p.unit, Months);

        let p = Period::from(Monthly);
        assert_eq!(p.length, 1);
        assert_eq!(p.unit, Months);

        let p = Period::from(EveryFourthWeek);
        assert_eq!(p.length, 4);
        assert_eq!(p.unit, Weeks);

        let p = Period::from(Biweekly);
        assert_eq!(p.length, 2);
        assert_eq!(p.unit, Weeks);

        let p = Period::from(Weekly);
        assert_eq!(p.length, 1);
        assert_eq!(p.unit, Weeks);

        let p = Period::from(Daily);
        assert_eq!(p.length, 1);
        assert_eq!(p.unit, Days);
    }

    #[test]
    fn test_frequency() {
        let p = Period::new(0, Years);
        assert_eq!(p.frequency(), Once);

        let p = Period::new(0, Days);
        assert_eq!(p.frequency(), NoFrequency);
        let p = Period::new(1, Days);
        assert_eq!(p.frequency(), Daily);
        let p = Period::new(2, Days);
        assert_eq!(p.frequency(), OtherFrequency);
        let p = Period::new(1, Weeks);
        assert_eq!(p.frequency(), Weekly);
        let p = Period::new(2, Weeks);
        assert_eq!(p.frequency(), Biweekly);
        let p = Period::new(4, Weeks);
        assert_eq!(p.frequency(), EveryFourthWeek);
        let p = Period::new(5, Weeks);
        assert_eq!(p.frequency(), OtherFrequency);

        let p = Period::new(1, Months);
        assert_eq!(p.frequency(), Monthly);
        let p = Period::new(2, Months);
        assert_eq!(p.frequency(), Bimonthly);
        let p = Period::new(3, Months);
        assert_eq!(p.frequency(), Quarterly);
        let p = Period::new(4, Months);
        assert_eq!(p.frequency(), EveryFourthMonth);
        let p = Period::new(6, Months);
        assert_eq!(p.frequency(), Semiannual);
        let p = Period::new(12, Months);
        assert_eq!(p.frequency(), Annual);

        let p = Period::new(1, Years);
        assert_eq!(p.frequency(), Annual);
        let p = Period::new(2, Years);
        assert_eq!(p.frequency(), OtherFrequency);
    }

    #[test]
    fn test_normalise() {
        let mut p = Period::new(0, Years);
        p.normalise();
        assert_eq!(p.unit, Days);

        let mut p = Period::new(12, Months);
        p.normalise();
        assert_eq!(p.length, 1);
        assert_eq!(p.unit, Years);

        let mut p = Period::new(24, Months);
        p.normalise();
        assert_eq!(p.length, 2);
        assert_eq!(p.unit, Years);

        let mut p = Period::new(7, Days);
        p.normalise();
        assert_eq!(p.length, 1);
        assert_eq!(p.unit, Weeks);

        let mut p = Period::new(14, Days);
        p.normalise();
        assert_eq!(p.length, 2);
        assert_eq!(p.unit, Weeks);

        let mut p = Period::new(4, Weeks);
        p.normalise();
        assert_eq!(p.length, 4);
        assert_eq!(p.unit, Weeks);

        let mut p = Period::new(4, Years);
        p.normalise();
        assert_eq!(p.length, 4);
        assert_eq!(p.unit, Years);
    }

    #[test]
    fn test_days_min_max() {
        let p = Period::new(2, Days);
        assert_eq!(p.days_min_max(), (2, 2));

        let p = Period::new(1, Weeks);
        assert_eq!(p.days_min_max(), (7, 7));

        let p = Period::new(1, Months);
        assert_eq!(p.days_min_max(), (28, 31));

        let p = Period::new(1, Years);
        assert_eq!(p.days_min_max(), (365, 366));
    }

    #[test]
    fn test_years() {
        let p = Period::new(6, Months);
        assert_eq!(p.years(), 0.5);

        let p = Period::new(2, Years);
        assert_eq!(p.years(), 2.0);
    }

    #[test]
    fn test_months() {
        let p = Period::new(6, Months);
        assert_eq!(p.months(), 6.0);

        let p = Period::new(2, Years);
        assert_eq!(p.months(), 24.0);
    }

    #[test]
    fn test_weeks() {
        let p = Period::new(14, Days);
        assert_eq!(p.weeks(), 2.0);

        let p = Period::new(2, Weeks);
        assert_eq!(p.weeks(), 2.0);
    }

    #[test]
    fn test_days() {
        let p = Period::new(14, Days);
        assert_eq!(p.days(), 14.0);

        let p = Period::new(2, Weeks);
        assert_eq!(p.days(), 14.0);
    }

    #[test]
    pub fn test_years_months_algebra() {
        let one_year = Period::new(1, Years);
        let six_months = Period::new(6, Months);
        let three_months = Period::new(3, Months);

        let n = 4;
        assert!(
            one_year / n == three_months,
            "division error: {:?} / {} != {:?}",
            one_year,
            n,
            three_months
        );

        let n = 2;
        assert!(
            one_year / n == six_months,
            "division error: {:?} / {} != {:?}",
            one_year,
            n,
            three_months
        );

        let mut sum = three_months;
        sum += six_months;
        assert!(
            sum == Period::new(9, Months),
            "sum error: {:?} + {:?} != {:?}",
            three_months,
            six_months,
            Period::new(9, Months)
        );

        sum += one_year;
        assert!(
            sum == Period::new(21, Months),
            "sum error: {:?} + {:?} + {:?} != {:?}",
            three_months,
            six_months,
            one_year,
            Period::new(21, Months)
        );

        let twelve_months = Period::new(12, Months);
        assert_eq!(
            twelve_months.length, 12,
            "normalisation error: twelve_months.length is {}, instead of 12",
            twelve_months.length
        );
        assert_eq!(
            twelve_months.unit, Months,
            "normalisation error: twelve_months.units is {:?}, instead of Months",
            twelve_months.unit
        );

        let mut normalised_twelve_months = Period::new(12, Months);
        normalised_twelve_months.normalise();
        assert_eq!(
            normalised_twelve_months.length, 1,
            "normalisation error: normalised_twelve_months.length is {}, instead of 1",
            normalised_twelve_months.length
        );
        assert_eq!(
            normalised_twelve_months.unit, Years,
            "normalisation error: normalised_twelve_months.units is {:?}, instead of Years",
            normalised_twelve_months.unit
        );
    }

    #[test]
    fn test_weekday_algebra() {
        let two_weeks = Period::new(2, Weeks);
        let one_week = Period::new(1, Weeks);
        let three_days = Period::new(3, Days);
        let one_day = Period::new(1, Days);

        let n = 2;
        assert_eq!(
            two_weeks / n,
            one_week,
            "division error: {:?} / {} != {:?}",
            two_weeks,
            n,
            one_week
        );

        let n = 7;
        assert_eq!(
            one_week / n,
            one_day,
            "division error: {:?} / {} != {:?}",
            one_week,
            n,
            one_day
        );

        let mut sum = three_days;
        sum += one_day;
        assert_eq!(
            sum,
            Period::new(4, Days),
            "sum error: {:?} + {:?} != {:?}",
            three_days,
            one_day,
            Period::new(4, Days)
        );

        sum += one_week;
        assert_eq!(
            sum,
            Period::new(11, Days),
            "sum error: {:?} + {:?} + {:?} != {:?}",
            three_days,
            one_day,
            one_week,
            Period::new(11, Days)
        );

        let seven_days = Period::new(7, Days);
        assert_eq!(
            seven_days.length, 7,
            "normalisation error: seven_days length is {} instead of 7",
            seven_days.length
        );
        assert_eq!(
            seven_days.unit, Days,
            "normalisation error: seven_days unit is {:?} instead of Days",
            seven_days.unit
        )
    }

    #[test]
    fn test_normalisation() {
        let test_values = vec![
            Period::new(0, Days),
            Period::new(0, Weeks),
            Period::new(0, Months),
            Period::new(0, Years),
            Period::new(3, Days),
            Period::new(7, Days),
            Period::new(14, Days),
            Period::new(30, Days),
            Period::new(60, Days),
            Period::new(365, Days),
            Period::new(1, Weeks),
            Period::new(2, Weeks),
            Period::new(4, Weeks),
            Period::new(8, Weeks),
            Period::new(52, Weeks),
            Period::new(1, Months),
            Period::new(2, Months),
            Period::new(6, Months),
            Period::new(12, Months),
            Period::new(18, Months),
            Period::new(24, Months),
            Period::new(1, Years),
            Period::new(2, Years),
        ];

        let mut test_values_copy = test_values.clone();

        for p1 in test_values {
            let n1 = p1.normalised();
            assert_eq!(
                n1, p1,
                "Normalising {:?} yields {:?}, which are not equal",
                p1, n1
            );

            for p2 in &mut test_values_copy {
                let n2 = p2.normalised();
                let cmp = p1 == *p2;
                if cmp {
                    // periods which compare equal must normalize to exactly the same period
                    assert!(
                        n1.unit == n2.unit && n1.length == n2.length,
                        "{:?} and {:?} compare equal, but normalise to {:?} and {:?}",
                        p1,
                        p2,
                        n1,
                        n2
                    );
                }

                if n1.unit == n2.unit && n1.length == n2.length {
                    // periods normalizing to exactly the same period must compare equal
                    assert_eq!(
                        p1, *p2,
                        "{:?} and {:?} compare different, but normalise to {:?} and {:?}",
                        p1, p2, n1, n2
                    );
                }
            }
        }
    }
}
