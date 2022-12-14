use std::f64::consts::E;

use crate::types::{DiscountFactor, Integer, Rate, Real, Time};

use crate::time::{
    compounding::Compounding, date::Date, daycounter::DayCounter, frequency::Frequency,
};

/// Encapsulate the interest rate compounding algebra.
///
/// It manages day-counting conventions, compounding conventions, conversion between different
/// conventions, discount/compound factor calculations, and implied/equivalent rate calculations.
#[derive(Debug)]
pub struct InterestRate {
    pub rate: Rate,
    pub daycounter: DayCounter,
    pub compounding: Compounding,
    pub frequency_makes_sense: bool,
    pub frequency: Real,
}

impl InterestRate {
    pub fn new(
        rate: Rate,
        daycounter: DayCounter,
        compounding: Compounding,
        frequency: Frequency,
    ) -> Self {
        let (frequency_makes_sense, frequency) = if compounding == Compounding::Compounded
            || compounding == Compounding::SimpleThenCompounded
            || compounding == Compounding::CompoundedThenSimple
        {
            assert!(
                frequency != Frequency::Once && frequency != Frequency::NoFrequency,
                "frequency {:?} not allowed for this interest rate",
                frequency
            );
            (true, frequency.into())
        } else {
            (false, Real::default())
        };

        Self {
            rate,
            daycounter,
            compounding,
            frequency_makes_sense,
            frequency,
        }
    }

    /// Return the [Frequency]
    pub fn frequency(&self) -> Frequency {
        if self.frequency_makes_sense {
            Frequency::from(self.frequency as Integer)
        } else {
            Frequency::NoFrequency
        }
    }

    /// Discount factor implied by the rate compounded at time t.
    /// Time must be measured using InterestRate's own day counter.
    pub fn discount_factor(&self, t: Time) -> DiscountFactor {
        1.0 / self.compound_factor(t)
    }

    /// Discount factor implied by the rate compounded between two dates
    pub fn discount_factor_between_dates(
        &self,
        d1: &Date,
        d2: &Date,
        ref_start: &Date,
        ref_end: &Date,
    ) -> DiscountFactor {
        assert!(d2 >= d1, "d1 ({:?}) later than d2 ({:?})", d1, d2);
        let t = self
            .daycounter
            .year_fraction_with_start_end(d1, d2, ref_start, ref_end);
        self.discount_factor(t)
    }

    /// Returns the compound (a.k.a capitalization) factor implied by the rate compounded at time t.
    /// Time must be measured using InterestRate's own day counter.
    pub fn compound_factor(&self, t: Time) -> Real {
        assert!(t >= 0.0, "negative time ({}) is not allowed", t);
        match self.compounding {
            Compounding::Simple => 1.0 + self.rate * t,
            Compounding::Compounded => (1.0 + self.rate / self.frequency).powf(self.frequency * t),
            Compounding::Continuous => (self.rate * t).exp(),
            Compounding::SimpleThenCompounded => {
                if t <= 1.0 / self.frequency {
                    1.0 + self.rate * t
                } else {
                    (1.0 + self.rate / self.frequency).powf(self.frequency * t)
                }
            }
            Compounding::CompoundedThenSimple => {
                if t > 1.0 / self.frequency {
                    1.0 + self.rate * t
                } else {
                    (1.0 + self.rate / self.frequency).powf(self.frequency * t)
                }
            }
        }
    }

    pub fn compound_factor_between_dates(
        &self,
        d1: &Date,
        d2: &Date,
        ref_start: &Date,
        ref_end: &Date,
    ) -> Real {
        assert!(d2 >= d1, "d1 ({:?}) later than d2 ({:?})", d1, d2);
        let t = self
            .daycounter
            .year_fraction_with_start_end(d1, d2, ref_start, ref_end);
        self.compound_factor(t)
    }

    /// Implied interest rate for a given compound factor at a given time.
    /// The resulting InterestRate has the day-counter provided as input.
    ///
    /// Time must be measured using the day-counter provided as input.
    pub fn implied_rate(
        &self,
        compound: Real,
        result_dc: &DayCounter,
        compounding: &Compounding,
        frequency: Frequency,
        t: Time,
    ) -> InterestRate {
        assert!(compound > 0.0, "positive compound factor required");
        let r = if compound == 1.0 {
            assert!(t >= 0.0, "non negative time ({}) required", t);
            0.0
        } else {
            assert!(t > 0.0, "positive time ({}) required", t);
            match compounding {
                Compounding::Simple => (compound - 1.0) / t,
                Compounding::Compounded => {
                    let freq = Into::<Real>::into(frequency);
                    (compound.powf(1.0 / (freq * t)) - 1.0) * freq
                }
                Compounding::Continuous => compound.log(E) / t,
                Compounding::SimpleThenCompounded => {
                    let freq = Into::<Real>::into(frequency);
                    if t <= 1.0 / freq {
                        (compound - 1.0) / t
                    } else {
                        (compound.powf(1.0 / (freq * t)) - 1.0) * freq
                    }
                }
                Compounding::CompoundedThenSimple => {
                    let freq = Into::<Real>::into(frequency);
                    if t > 1.0 / freq {
                        (compound - 1.0) / t
                    } else {
                        (compound.powf(1.0 / (freq * t)) - 1.0) * freq
                    }
                }
            }
        };
        InterestRate::new(r, result_dc.clone(), compounding.clone(), frequency)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn implied_rate_between_dates(
        &self,
        compound: Real,
        result_dc: &DayCounter,
        compounding: &Compounding,
        frequency: Frequency,
        d1: &Date,
        d2: &Date,
        ref_start: &Date,
        ref_end: &Date,
    ) -> InterestRate {
        assert!(d2 >= d1, "d1 ({:?}) later than d2 ({:?})", d1, d2);
        let t = result_dc.year_fraction_with_start_end(d1, d2, ref_start, ref_end);
        self.implied_rate(compound, result_dc, compounding, frequency, t)
    }

    /// Equivalent interest rate for a compounding period t.
    /// The resulting InterestRate shares the same implicit day-counting rule of the original
    /// InterestRate instance.
    ///
    /// Time must be measured using the InterestRate's own day counter.
    pub fn equivalent_rate(
        &self,
        compounding: &Compounding,
        frequency: Frequency,
        t: Time,
    ) -> InterestRate {
        self.implied_rate(
            self.compound_factor(t),
            &self.daycounter,
            compounding,
            frequency,
            t,
        )
    }

    /// Equivalent rate for a compounding period between two dates.
    /// The resulting rate is calculated taking the required day-counting rule into account.
    #[allow(clippy::too_many_arguments)]
    pub fn equivalent_rate_between_dates(
        &self,
        result_dc: &DayCounter,
        compounding: &Compounding,
        frequency: Frequency,
        d1: &Date,
        d2: &Date,
        ref_start: &Date,
        ref_end: &Date,
    ) -> InterestRate {
        assert!(d2 >= d1, "d1 ({:?}) later than d2 ({:?})", d1, d2);
        let t1 = self
            .daycounter
            .year_fraction_with_start_end(d1, d2, ref_start, ref_end);
        let t2 = result_dc.year_fraction_with_start_end(d1, d2, ref_start, ref_end);
        self.implied_rate(
            self.compound_factor(t1),
            result_dc,
            compounding,
            frequency,
            t2,
        )
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::maths::rounding::{Rounding, RoundingType};
    use crate::types::{Rate, Size, Time};

    use crate::time::{
        compounding::Compounding::{self, *},
        date::Date,
        daycounters::actual360::Actual360,
        frequency::Frequency::{self, *},
        time_to_days,
    };

    use super::InterestRate;

    #[test]
    fn test_interest_rate() {
        let ir = InterestRate::new(0.1, Actual360::new(), Compounded, Annual);
        assert_eq!(ir.frequency(), Annual);
    }

    pub struct InterestRateData {
        pub r: Rate,
        pub comp: Compounding,
        pub freq: Frequency,
        pub t: Time,
        pub comp2: Compounding,
        pub freq2: Frequency,
        pub expected: Rate,
        pub precision: Size,
    }

    impl InterestRateData {
        #[allow(clippy::too_many_arguments)]
        pub fn new(
            r: Rate,
            comp: Compounding,
            freq: Frequency,
            t: Time,
            comp2: Compounding,
            freq2: Frequency,
            expected: Rate,
            precision: Size,
        ) -> Self {
            Self {
                r,
                comp,
                freq,
                t,
                comp2,
                freq2,
                expected,
                precision,
            }
        }
    }

    #[test]
    fn test_conversions() {
        let cases = vec![
            // data from "Option Pricing Formulas", Haug, pag.181-182
            InterestRateData::new(
                0.0800, Compounded, Quarterly, 1.00, Continuous, Annual, 0.0792, 4,
            ),
            InterestRateData::new(
                0.1200, Continuous, Annual, 1.00, Compounded, Annual, 0.1275, 4,
            ),
            InterestRateData::new(
                0.0800, Compounded, Quarterly, 1.00, Compounded, Annual, 0.0824, 4,
            ),
            InterestRateData::new(
                0.0700, Compounded, Quarterly, 1.00, Compounded, Semiannual, 0.0706, 4,
            ),
            // undocumented, but reasonable :)
            InterestRateData::new(0.0100, Compounded, Annual, 1.00, Simple, Annual, 0.0100, 4),
            InterestRateData::new(0.0200, Simple, Annual, 1.00, Compounded, Annual, 0.0200, 4),
            InterestRateData::new(
                0.0300, Compounded, Semiannual, 0.50, Simple, Annual, 0.0300, 4,
            ),
            InterestRateData::new(
                0.0400, Simple, Annual, 0.50, Compounded, Semiannual, 0.0400, 4,
            ),
            InterestRateData::new(
                0.0500,
                Compounded,
                EveryFourthMonth,
                1.0 / 3.0,
                Simple,
                Annual,
                0.0500,
                4,
            ),
            InterestRateData::new(
                0.0600,
                Simple,
                Annual,
                1.0 / 3.0,
                Compounded,
                EveryFourthMonth,
                0.0600,
                4,
            ),
            InterestRateData::new(
                0.0500, Compounded, Quarterly, 0.25, Simple, Annual, 0.0500, 4,
            ),
            InterestRateData::new(
                0.0600, Simple, Annual, 0.25, Compounded, Quarterly, 0.0600, 4,
            ),
            InterestRateData::new(
                0.0700,
                Compounded,
                Bimonthly,
                1.0 / 6.0,
                Simple,
                Annual,
                0.0700,
                4,
            ),
            InterestRateData::new(
                0.0800,
                Simple,
                Annual,
                1.0 / 6.0,
                Compounded,
                Bimonthly,
                0.0800,
                4,
            ),
            InterestRateData::new(
                0.0900,
                Compounded,
                Monthly,
                1.0 / 12.0,
                Simple,
                Annual,
                0.0900,
                4,
            ),
            InterestRateData::new(
                0.1000,
                Simple,
                Annual,
                1.0 / 12.0,
                Compounded,
                Monthly,
                0.1000,
                4,
            ),
            InterestRateData::new(
                0.0300,
                SimpleThenCompounded,
                Semiannual,
                0.25,
                Simple,
                Annual,
                0.0300,
                4,
            ),
            InterestRateData::new(
                0.0300,
                SimpleThenCompounded,
                Semiannual,
                0.25,
                Simple,
                Semiannual,
                0.0300,
                4,
            ),
            InterestRateData::new(
                0.0300,
                SimpleThenCompounded,
                Semiannual,
                0.25,
                Simple,
                Quarterly,
                0.0300,
                4,
            ),
            InterestRateData::new(
                0.0300,
                SimpleThenCompounded,
                Semiannual,
                0.50,
                Simple,
                Annual,
                0.0300,
                4,
            ),
            InterestRateData::new(
                0.0300,
                SimpleThenCompounded,
                Semiannual,
                0.50,
                Simple,
                Semiannual,
                0.0300,
                4,
            ),
            InterestRateData::new(
                0.0300,
                SimpleThenCompounded,
                Semiannual,
                0.75,
                Compounded,
                Semiannual,
                0.0300,
                4,
            ),
            InterestRateData::new(
                0.0400,
                Simple,
                Semiannual,
                0.25,
                SimpleThenCompounded,
                Quarterly,
                0.0400,
                4,
            ),
            InterestRateData::new(
                0.0400,
                Simple,
                Semiannual,
                0.25,
                SimpleThenCompounded,
                Semiannual,
                0.0400,
                4,
            ),
            InterestRateData::new(
                0.0400,
                Simple,
                Semiannual,
                0.25,
                SimpleThenCompounded,
                Annual,
                0.0400,
                4,
            ),
            InterestRateData::new(
                0.0400,
                Compounded,
                Quarterly,
                0.50,
                SimpleThenCompounded,
                Quarterly,
                0.0400,
                4,
            ),
            InterestRateData::new(
                0.0400,
                Simple,
                Semiannual,
                0.50,
                SimpleThenCompounded,
                Semiannual,
                0.0400,
                4,
            ),
            InterestRateData::new(
                0.0400,
                Simple,
                Semiannual,
                0.50,
                SimpleThenCompounded,
                Annual,
                0.0400,
                4,
            ),
            InterestRateData::new(
                0.0400,
                Compounded,
                Quarterly,
                0.75,
                SimpleThenCompounded,
                Quarterly,
                0.0400,
                4,
            ),
            InterestRateData::new(
                0.0400,
                Compounded,
                Semiannual,
                0.75,
                SimpleThenCompounded,
                Semiannual,
                0.0400,
                4,
            ),
            InterestRateData::new(
                0.0400,
                Simple,
                Semiannual,
                0.75,
                SimpleThenCompounded,
                Annual,
                0.0400,
                4,
            ),
        ];

        let d1 = Date::todays_date();
        for i in cases {
            let ir = InterestRate::new(i.r, Actual360::new(), i.comp, i.freq);
            let d2 = d1 + time_to_days(i.t);

            // check that the compound factor is the inverse of the discount factor
            let compoundf =
                ir.compound_factor_between_dates(&d1, &d2, &Date::default(), &Date::default());
            let disc =
                ir.discount_factor_between_dates(&d1, &d2, &Date::default(), &Date::default());
            let error = (disc - 1.0 / compoundf).abs();
            assert!(
                error <= 1e-15,
                "{}, 1.0/compound_factor: {}, discount_factor: {}, error: {}",
                ir.rate,
                1.0 / compoundf,
                disc,
                error
            );

            // check that the equivalent InterestRate with *same* daycounter,
            // compounding, and frequency is the *same* InterestRate
            let ir2 = ir.equivalent_rate_between_dates(
                &ir.daycounter,
                &ir.compounding,
                ir.frequency(),
                &d1,
                &d2,
                &Date::default(),
                &Date::default(),
            );

            let error = (ir.rate - ir2.rate).abs();
            assert!(
                error <= 1e-15,
                "original interest rate: {:?}, equivalent interest rate: {:?}, rate error {}",
                ir,
                ir2,
                error
            );
            assert_eq!(
                ir.daycounter, ir2.daycounter,
                "day counter error, original interest rate: {:?}, equivalent rate: {:?}",
                ir, ir2
            );
            assert_eq!(
                ir.compounding, ir2.compounding,
                "compounding error, original interest rate: {:?}, equivalent rate: {:?}",
                ir, ir2
            );
            assert_eq!(
                ir.frequency(),
                ir2.frequency(),
                "frequency error, original interest rate: {:?}, equivalent rate: {:?}",
                ir,
                ir2
            );

            // check that the equivalent InterestRate with *different*
            // compounding, and frequency is the *expected* InterestRate
            let ir3 = ir.equivalent_rate_between_dates(
                &ir.daycounter,
                &i.comp2,
                i.freq2,
                &d1,
                &d2,
                &Date::default(),
                &Date::default(),
            );
            let expected_ir =
                InterestRate::new(i.expected, ir.daycounter.clone(), i.comp2, i.freq2);
            let rounding = RoundingType::closest(i.precision as i32, 5);
            let r3 = rounding.round(ir3.rate);
            let error = (r3 - expected_ir.rate).abs();
            assert!(
                error <= 1.0e-17,
                "\n\
                 original interest rate:              {:?}\n\
                 calculated equivalent interest rate: {:?}\n\
                 truncated equivalent rate:           {:?}\n\
                 expected equivalent interest rate:   {:?}\n\
                 rate error: {}",
                ir,
                ir3,
                r3,
                expected_ir,
                error
            );

            assert_eq!(
                ir3.daycounter, expected_ir.daycounter,
                "day counter error, original interest rate: {:?}, equivalent rate: {:?}",
                ir3, expected_ir
            );

            assert_eq!(
                ir3.compounding, expected_ir.compounding,
                "compounding error, original interest rate: {:?}, equivalent rate: {:?}",
                ir3, expected_ir
            );

            assert_eq!(
                ir3.frequency, expected_ir.frequency,
                "frequency error, original interest rate: {:?}, equivalent rate: {:?}",
                ir3, expected_ir
            );
        }
    }
}
