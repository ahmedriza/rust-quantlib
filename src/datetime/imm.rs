use crate::types::Integer;

use crate::context::pricing_context::PricingContext;
use crate::datetime::{date::Date, months::Month::*, weekday::Weekday::*};

/// Main cycle of the International Monetary Market (a.k.a. IMM) months
///
/// <https://en.wikipedia.org/wiki/IMM_dates>:
///
/// The IMM dates are the four quarterly dates of each year which EuroDollar and Foreign
/// Exchange futures contracts and option contracts use as their scheduled maturity date or
/// termination date. The dates are the third Wednesday of March, June, September and
/// December (i.e., between the 15th and 21st, whichever such day is a Wednesday).
pub struct IMM {
    pricing_context: PricingContext,
}

pub enum IMMMonth {
    F = 1,
    G = 2,
    H = 3,
    J = 4,
    K = 5,
    M = 6,
    N = 7,
    Q = 8,
    U = 9,
    V = 10,
    X = 11,
    Z = 12,
}

impl IMM {
    pub fn new(pricing_context: PricingContext) -> Self {
        Self { pricing_context }
    }

    /// Returns whether or not the given date is an IMM date
    pub fn is_imm_date(&self, date: &Date, main_cycle: bool) -> bool {
        if date.weekday() != Wednesday {
            return false;
        }

        let d = date.day_of_month();
        if !(15..=21).contains(&d) {
            return false;
        }

        if !main_cycle {
            return true;
        }

        matches!(date.month(), March | June | September | December)
    }

    /// Returns whether or not the given `input` string is an IMM code
    pub fn is_imm_code(&self, input: &str, main_cycle: bool) -> bool {
        if input.len() != 2 {
            return false;
        }
        let mut str1 = "0123456789";
        let sub = &input[1..2];
        let found = str1.find(sub);
        if found.is_none() {
            return false;
        }
        if main_cycle {
            str1 = "hmzuHMZU";
        } else {
            str1 = "fghjkmnquvxzFGHJKMNQUVXZ";
        }
        let found = str1.find(&input[0..1]);
        found.is_some()
    }

    /// Returns the IMM code for the given date (e.g. H3 for March 20th, 2013).
    pub fn code(&self, date: &Date) -> String {
        assert!(
            self.is_imm_date(date, false),
            "{:?} is not an IMM date",
            date
        );
        let y = date.year() % 10;
        match date.month() {
            January => format!("F{}", y),
            February => format!("G{}", y),
            March => format!("H{}", y),
            April => format!("J{}", y),
            May => format!("K{}", y),
            June => format!("M{}", y),
            July => format!("N{}", y),
            August => format!("Q{}", y),
            September => format!("U{}", y),
            October => format!("V{}", y),
            November => format!("X{}", y),
            December => format!("Z{}", y),
        }
    }

    /// Returns the IMM date for the given IMM code (e.g. March 20th, 2013 for H3).
    pub fn date(&self, imm_code: &str, reference_date: &Date) -> Date {
        let eval_date = &self.pricing_context.eval_date;
        assert!(
            self.is_imm_code(imm_code, false),
            "{} is not a valid IMM code",
            imm_code
        );

        let reference_date = if *reference_date != Date::default() {
            reference_date
        } else {
            eval_date
        };

        let code = imm_code.to_uppercase();
        let ms = &code[0..1];

        let month = match ms {
            "F" => January,
            "G" => February,
            "H" => March,
            "J" => April,
            "K" => May,
            "M" => June,
            "N" => July,
            "Q" => August,
            "U" => September,
            "V" => October,
            "X" => November,
            "Z" => December,
            _ => unreachable!(), // code is checked for validity above
        };

        // This cannot panic, since we have already checked abvoe that the code is valid.
        let mut y = code[1..2].parse::<Integer>().unwrap();

        // year < 1900 are not valid QuantLib years: to avoid a run-time
        // exception few lines below we need to add 10 years right away
        if y == 0 && reference_date.year() < 1909 {
            y += 10;
        }
        let reference_year = reference_date.year() % 10;
        y += reference_date.year() - reference_year;
        let result = self.next_date(&Date::new(1, month, y), false);
        if &result < reference_date {
            return self.next_date(&Date::new(1, month, y + 10), false);
        }
        result
    }

    /// Next IMM date following the given date.
    ///
    /// Returns the 1st delivery date for next contract listed in the International Money Market
    /// section of the Chicago Mercantile Exchange.    
    pub fn next_date(&self, date: &Date, main_cycle: bool) -> Date {
        let eval_date = &self.pricing_context.eval_date;
        let ref_date = if date == &Date::default() {
            eval_date
        } else {
            date
        };

        let mut y = ref_date.year();
        let mut m = ref_date.month();
        let offset = if main_cycle { 3 } else { 1 };

        let mut skip_months = offset - (m as Integer % offset);
        if skip_months != offset || ref_date.day_of_month() > 21 {
            skip_months += m as Integer;
            if skip_months <= 12 {
                m = skip_months.into();
            } else {
                m = (skip_months - 12).into();
                y += 1;
            }
        }

        let mut result = Date::nth_weekday(3, Wednesday, m, y);
        if &result <= ref_date {
            result = self.next_date(&Date::new(22, m, y), main_cycle);
        }

        result
    }

    /// Next IMM date following the given IMM code.
    ///
    /// Returns the 1st delivery date for next contract listed in the International Money Market
    /// section of the Chicago Mercantile Exchange.    
    pub fn next_date_from_code(
        &self,
        imm_code: &str,
        main_cycle: bool,
        reference_date: &Date,
    ) -> Date {
        let imm_date = self.date(imm_code, reference_date);
        self.next_date(&(imm_date + 1), main_cycle)
    }

    /// Next IMM code following the given date.
    ///
    /// Returns the IMM code for next contract listed in the International Money Market section
    /// of the Chicago Mercantile Exchange.
    pub fn next_code(&self, d: &Date, main_cycle: bool) -> String {
        let date = self.next_date(d, main_cycle);
        self.code(&date)
    }

    /// Next IMM code following the given code.
    ///
    /// Returns the IMM code for next contract listed in the International Money Market section
    /// of the Chicago Mercantile Exchange.
    pub fn next_code_from_code(
        &self,
        imm_code: &str,
        main_cycle: bool,
        reference_date: &Date,
    ) -> String {
        let date = self.next_date_from_code(imm_code, main_cycle, reference_date);
        self.code(&date)
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::context::pricing_context::PricingContext;
    use crate::datetime::{date::Date, imm::IMM, months::Month::*};

    #[test]
    fn test_is_imm_code() {
        let code = "H3";
        let imm = IMM::new(PricingContext {
            eval_date: Date::default(),
        });
        assert!(imm.is_imm_code(code, true));
    }

    #[test]
    fn test_code() {
        let date = Date::new(20, March, 2013);
        let imm = IMM::new(PricingContext {
            eval_date: Date::default(),
        });
        let code = imm.code(&date);
        assert_eq!(code, "H3".to_string());
    }

    #[test]
    fn test_date() {
        let imm = IMM::new(PricingContext {
            eval_date: Date::new(30, November, 2023),
        });
        let date = imm.date("H4", &Date::default());
        assert_eq!(date, Date::new(20, March, 2024));
    }

    #[test]
    fn test_imm_dates() {
        let imm_codes = [
            "F0", "G0", "H0", "J0", "K0", "M0", "N0", "Q0", "U0", "V0", "X0", "Z0", "F1", "G1",
            "H1", "J1", "K1", "M1", "N1", "Q1", "U1", "V1", "X1", "Z1", "F2", "G2", "H2", "J2",
            "K2", "M2", "N2", "Q2", "U2", "V2", "X2", "Z2", "F3", "G3", "H3", "J3", "K3", "M3",
            "N3", "Q3", "U3", "V3", "X3", "Z3", "F4", "G4", "H4", "J4", "K4", "M4", "N4", "Q4",
            "U4", "V4", "X4", "Z4", "F5", "G5", "H5", "J5", "K5", "M5", "N5", "Q5", "U5", "V5",
            "X5", "Z5", "F6", "G6", "H6", "J6", "K6", "M6", "N6", "Q6", "U6", "V6", "X6", "Z6",
            "F7", "G7", "H7", "J7", "K7", "M7", "N7", "Q7", "U7", "V7", "X7", "Z7", "F8", "G8",
            "H8", "J8", "K8", "M8", "N8", "Q8", "U8", "V8", "X8", "Z8", "F9", "G9", "H9", "J9",
            "K9", "M9", "N9", "Q9", "U9", "V9", "X9", "Z9",
        ];

        let imm = IMM::new(PricingContext {
            eval_date: Date::new(1, January, 2020),
        });

        let mut counter = Date::new(1, January, 2000);
        let last = Date::new(1, January, 2040);

        while counter <= last {
            let imm_date = imm.next_date(&counter, false);
            assert!(
                imm_date > counter,
                "{:?} {:?} is not greater than {:?} {:?}",
                imm_date.weekday(),
                imm_date,
                counter.weekday(),
                counter
            );

            // check that imm is an IMM date
            assert!(
                imm.is_imm_date(&imm_date, false),
                "{:?} {:?} is not an IMM date (calculated from {:?} {:?})",
                imm_date.weekday(),
                imm_date,
                counter.weekday(),
                counter
            );

            // check that imm is <= to the next IMM date in the main cycle
            assert!(
                imm_date <= imm.next_date(&counter, true),
                "{:?} {:?} is not less than or equal to the next future in the main cycle {:?}",
                imm_date.weekday(),
                imm_date,
                imm.next_date(&counter, true)
            );

            // check that for every date IMMdate is the inverse of IMMcode
            let imm_code = imm.code(&imm_date);
            let date = imm.date(&imm_code, &counter);
            assert_eq!(
                date, imm_date,
                "{}, at calendar day {:?} is not the IMM code matching {:?}",
                imm_code, counter, imm_date
            );

            // check that for every date the 120 IMM codes refer to future dates
            for imm_code in imm_codes.iter().take(40) {
                let imm_date = imm.date(imm_code, &counter);
                assert!(
                    imm_date >= counter,
                    "{:?} is wrong for {} at reference date {:?}",
                    imm_date,
                    imm_code,
                    counter
                );
            }

            counter += 1;
        }
    }
}
