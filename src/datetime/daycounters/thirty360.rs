use crate::{
    datetime::{date::Date, months::Month, Day, Year},
    types::{Integer, Time},
};

/// 30/360 day count convention
///
/// The 30/360 day count can be calculated according to a number of conventions.
///
/// * US convention: if the starting date is the 31st of a month or the last day of February, it
///   becomes equal to the 30th of the same month.  If the ending date is the 31st of a month and
///   the starting date is the 30th or 31th of a month, the ending date becomes equal to the 30th.
///   If the ending date is the last of February and the starting date is also the last of February,
///   the ending date becomes equal to the 30th.
///   Also known as "30/360" or "360/360".
///
/// * Bond Basis convention: if the starting date is the 31st of a month, it becomes equal to the
///   30th of the same month. If the ending date is the 31st of a month and the starting date is
///   the 30th or 31th of a month, the ending date also becomes equal to the 30th of the month.
///   Also known as "US (ISMA)".
///
/// * European convention: starting dates or ending dates that occur on the 31st of a month become
///   equal to the 30th of the same month.
///   Also known as "30E/360", or "Eurobond Basis".
///
/// * Italian convention: starting dates or ending dates that occur on February and are greater
///   than 27 become equal to 30 for computational sake.
///
/// * ISDA convention: starting or ending dates on the 31st of the month become equal to 30;
///   starting dates or ending dates that occur on the last day of February also become equal to 30,
///   except for the termination date.  Also known as "30E/360
///   ISDA", "30/360 ISDA", or "30/360 German".
///
/// * NASD convention: if the starting date is the 31st of a month, it becomes equal to the 30th of
///   the same month. If the ending date is the 31st of a month and the starting date is earlier
///   than the 30th of a month, the ending date becomes equal to the 1st of the next month,
///   otherwise the ending date becomes equal to the 30th of the same month.
#[derive(Clone, Copy)]
pub struct Thirty360 {
    pub convention: Thiry360Convention,
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub enum Thiry360Convention {
    US(US),
    ISMA(ISMA),
    EU(EU),
    IT(IT),
    ISDA(ISDA),
    NASD(NASD),
}

// -------------------------------------------------------------------------------------------------

impl Thirty360 {
    /// Return the name of the day counter
    pub fn name(&self) -> String {
        match &self.convention {
            Thiry360Convention::US(c) => c.name(),
            Thiry360Convention::ISMA(c) => c.name(),
            Thiry360Convention::EU(c) => c.name(),
            Thiry360Convention::IT(c) => c.name(),
            Thiry360Convention::ISDA(c) => c.name(),
            Thiry360Convention::NASD(c) => c.name(),
        }
    }

    /// Returns the number of days between two dates.    
    pub fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        match &self.convention {
            Thiry360Convention::US(c) => c.day_count(d1, d2),
            Thiry360Convention::ISMA(c) => c.day_count(d1, d2),
            Thiry360Convention::EU(c) => c.day_count(d1, d2),
            Thiry360Convention::IT(c) => c.day_count(d1, d2),
            Thiry360Convention::ISDA(c) => c.day_count(d1, d2),
            Thiry360Convention::NASD(c) => c.day_count(d1, d2),
        }
    }

    /// Returns the period between two dates as a fraction of year    
    pub fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        ref_period_start: &Date,
        ref_period_end: &Date,
    ) -> Time {
        match &self.convention {
            Thiry360Convention::US(c) => c.year_fraction(d1, d2, ref_period_start, ref_period_end),
            Thiry360Convention::ISMA(c) => {
                c.year_fraction(d1, d2, ref_period_start, ref_period_end)
            }
            Thiry360Convention::EU(c) => c.year_fraction(d1, d2, ref_period_start, ref_period_end),
            Thiry360Convention::IT(c) => c.year_fraction(d1, d2, ref_period_start, ref_period_end),
            Thiry360Convention::ISDA(c) => {
                c.year_fraction(d1, d2, ref_period_start, ref_period_end)
            }
            Thiry360Convention::NASD(c) => {
                c.year_fraction(d1, d2, ref_period_start, ref_period_end)
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct US {}

impl US {
    pub fn name(&self) -> String {
        "30/360 (US)".to_string()
    }

    pub fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        let mut dd1 = d1.day_of_month();
        let mut dd2 = d2.day_of_month();
        let mm1 = d1.month();
        let mm2 = d2.month();
        let yy1 = d1.year();
        let yy2 = d2.year();

        if dd1 == 31 {
            dd1 = 30;
        }
        if dd2 == 31 && dd1 >= 30 {
            dd2 = 30;
        }

        if is_last_of_february(dd2, mm2, yy2) && is_last_of_february(dd1, mm1, yy1) {
            dd2 = 30;
        }
        if is_last_of_february(dd1, mm1, yy1) {
            dd1 = 30;
        }

        360 * (yy2 - yy1)
            + 30 * (mm2 as Integer - mm1 as Integer)
            + (dd2 as Integer - dd1 as Integer) as Integer
    }

    pub fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        _ref_period_start: &Date,
        _ref_period_end: &Date,
    ) -> Time {
        self.day_count(d1, d2) as Time / 360.0
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct ISMA {}

impl ISMA {
    pub fn name(&self) -> String {
        "30/360 (Bond Basis)".into()
    }

    pub fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        let mut dd1 = d1.day_of_month() as Integer;
        let mut dd2 = d2.day_of_month() as Integer;
        let mm1 = d1.month() as Integer;
        let mm2 = d2.month() as Integer;
        let yy1 = d1.year();
        let yy2 = d2.year();

        if dd1 == 31 {
            dd1 = 30;
        }
        if dd2 == 31 && dd1 == 30 {
            dd2 = 30;
        }
        360 * (yy2 - yy1) + 30 * (mm2 - mm1) + (dd2 - dd1) as Integer
    }

    pub fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        _ref_period_start: &Date,
        _ref_period_end: &Date,
    ) -> Time {
        self.day_count(d1, d2) as Time / 360.0
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct EU {}

impl EU {
    pub fn name(&self) -> String {
        "30E/360 (Eurobond Basis)".into()
    }

    pub fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        let mut dd1 = d1.day_of_month();
        let mut dd2 = d2.day_of_month();
        let mm1 = d1.month() as Integer;
        let mm2 = d2.month() as Integer;
        let yy1 = d1.year();
        let yy2 = d2.year();

        if dd1 == 31 {
            dd1 = 30;
        }
        if dd2 == 31 {
            dd2 = 30;
        }

        360 * (yy2 - yy1) + 30 * (mm2 - mm1) + (dd2 - dd1) as Integer
    }

    pub fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        _ref_period_start: &Date,
        _ref_period_end: &Date,
    ) -> Time {
        self.day_count(d1, d2) as Time / 360.0
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct IT {}

impl IT {
    pub fn name(&self) -> String {
        "30/360 (Italian)".into()
    }

    pub fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        let mut dd1 = d1.day_of_month();
        let mut dd2 = d2.day_of_month();
        let mm1 = d1.month() as Integer;
        let mm2 = d2.month() as Integer;
        let yy1 = d1.year();
        let yy2 = d2.year();

        if dd1 == 31 {
            dd1 = 30;
        }
        if dd2 == 31 {
            dd2 = 30;
        }

        if mm1 as Integer == 2 && dd1 > 27 {
            dd1 = 30;
        }
        if mm2 as Integer == 2 && dd2 > 27 {
            dd2 = 30;
        }

        360 * (yy2 - yy1) + 30 * (mm2 - mm1) + (dd2 - dd1) as Integer
    }

    pub fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        _ref_period_start: &Date,
        _ref_period_end: &Date,
    ) -> Time {
        self.day_count(d1, d2) as Time / 360.0
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct ISDA {
    pub termination_date: Date,
}

impl ISDA {
    pub fn name(&self) -> String {
        "30E/360 (ISDA)".into()
    }

    pub fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        let mut dd1 = d1.day_of_month();
        let mut dd2 = d2.day_of_month();
        let mm1 = d1.month();
        let mm2 = d2.month();
        let yy1 = d1.year();
        let yy2 = d2.year();

        if dd1 == 31 {
            dd1 = 30;
        }
        if dd2 == 31 {
            dd2 = 30;
        }

        if is_last_of_february(dd1, mm1, yy1) {
            dd1 = 30;
        }

        if d2 != &self.termination_date && is_last_of_february(dd2, mm2, yy2) {
            dd2 = 30;
        }

        360 * (yy2 - yy1) + 30 * (mm2 as Integer - mm1 as Integer) + (dd2 - dd1) as Integer
    }

    pub fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        _ref_period_start: &Date,
        _ref_period_end: &Date,
    ) -> Time {
        self.day_count(d1, d2) as Time / 360.0
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct NASD {}

impl NASD {
    pub fn name(&self) -> String {
        "30/360 (NASD)".into()
    }

    pub fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        let mut dd1 = d1.day_of_month();
        let mut dd2 = d2.day_of_month();
        let mm1 = d1.month() as Integer;
        let mut mm2 = d2.month() as Integer;
        let yy1 = d1.year();
        let yy2 = d2.year();

        if dd1 == 31 {
            dd1 = 30;
        }
        if dd2 == 31 && dd1 >= 30 {
            dd2 = 30;
        }
        if dd2 == 31 && dd1 < 30 {
            dd2 = 1;
            mm2 += 1;
        }

        360 * (yy2 - yy1) + 30 * (mm2 - mm1) + (dd2 - dd1) as Integer
    }

    pub fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        _ref_period_start: &Date,
        _ref_period_end: &Date,
    ) -> Time {
        self.day_count(d1, d2) as Time / 360.0
    }
}

// -------------------------------------------------------------------------------------------------

fn is_last_of_february(d: Day, m: Month, y: Year) -> bool {
    let leap_adjustment = if Date::is_leap(y) { 1 } else { 0 };
    m as Integer == 2 && d == 28 + leap_adjustment
}
