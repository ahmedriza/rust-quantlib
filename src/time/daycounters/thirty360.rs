use std::sync::Arc;

use crate::types::{Integer, Time};

use crate::time::{
    date::Date,
    daycounter::{DayCounter, DayCounterDetail},
    months::Month,
    Day, Year,
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
pub struct Thirty360 {}

impl Thirty360 {
    pub fn usa() -> DayCounter {
        DayCounter::new(Arc::new(US {}))
    }

    pub fn european() -> DayCounter {
        DayCounter::new(Arc::new(EU {}))
    }

    pub fn euro_bond_basis() -> DayCounter {
        Thirty360::european()
    }

    pub fn italian() -> DayCounter {
        DayCounter::new(Arc::new(IT {}))
    }

    pub fn isma() -> DayCounter {
        DayCounter::new(Arc::new(ISMA {}))
    }

    pub fn bond_basis() -> DayCounter {
        Thirty360::isma()
    }

    pub fn isda(termination_date: Date) -> DayCounter {
        DayCounter::new(Arc::new(ISDA { termination_date }))
    }

    pub fn german(termination_date: Date) -> DayCounter {
        Thirty360::isda(termination_date)
    }

    pub fn nasd() -> DayCounter {
        DayCounter::new(Arc::new(NASD {}))
    }
}

trait DayCounterDetailThirty360: DayCounterDetail {
    fn year_fraction(
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

pub struct US {}

impl DayCounterDetailThirty360 for US {}

impl DayCounterDetail for US {
    fn name(&self) -> String {
        "30/360 (US)".into()
    }

    fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
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

        360 * (yy2 - yy1) + 30 * (mm2 as Integer - mm1 as Integer) + (dd2 - dd1) as Integer
    }

    fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        ref_period_start: &Date,
        ref_period_end: &Date,
    ) -> Time {
        DayCounterDetailThirty360::year_fraction(self, d1, d2, ref_period_start, ref_period_end)
    }
}

// -------------------------------------------------------------------------------------------------

pub struct ISMA {}

impl DayCounterDetailThirty360 for ISMA {}

impl DayCounterDetail for ISMA {
    fn name(&self) -> String {
        "30/360 (Bond Basis)".into()
    }

    fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        let mut dd1 = d1.day_of_month();
        let mut dd2 = d2.day_of_month();
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

    fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        ref_period_start: &Date,
        ref_period_end: &Date,
    ) -> Time {
        DayCounterDetailThirty360::year_fraction(self, d1, d2, ref_period_start, ref_period_end)
    }
}

// -------------------------------------------------------------------------------------------------

pub struct EU {}

impl DayCounterDetailThirty360 for EU {}

impl DayCounterDetail for EU {
    fn name(&self) -> String {
        "30E/360 (Eurobond Basis)".into()
    }

    fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
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

    fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        ref_period_start: &Date,
        ref_period_end: &Date,
    ) -> Time {
        DayCounterDetailThirty360::year_fraction(self, d1, d2, ref_period_start, ref_period_end)
    }
}

// -------------------------------------------------------------------------------------------------

pub struct IT {}

impl DayCounterDetailThirty360 for IT {}

impl DayCounterDetail for IT {
    fn name(&self) -> String {
        "30/360 (Italian)".into()
    }

    fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
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

    fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        ref_period_start: &Date,
        ref_period_end: &Date,
    ) -> Time {
        DayCounterDetailThirty360::year_fraction(self, d1, d2, ref_period_start, ref_period_end)
    }
}

// -------------------------------------------------------------------------------------------------

pub struct ISDA {
    pub termination_date: Date,
}

impl DayCounterDetailThirty360 for ISDA {}

impl DayCounterDetail for ISDA {
    fn name(&self) -> String {
        "30E/360 (ISDA)".into()
    }

    fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
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

    fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        ref_period_start: &Date,
        ref_period_end: &Date,
    ) -> Time {
        DayCounterDetailThirty360::year_fraction(self, d1, d2, ref_period_start, ref_period_end)
    }
}

// -------------------------------------------------------------------------------------------------

pub struct NASD {}

impl DayCounterDetailThirty360 for NASD {}

impl DayCounterDetail for NASD {
    fn name(&self) -> String {
        "30/360 (NASD)".into()
    }

    fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
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

    fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        ref_period_start: &Date,
        ref_period_end: &Date,
    ) -> Time {
        DayCounterDetailThirty360::year_fraction(self, d1, d2, ref_period_start, ref_period_end)
    }
}

// -------------------------------------------------------------------------------------------------

fn is_last_of_february(d: Day, m: Month, y: Year) -> bool {
    let leap_adjustment = if Date::is_leap(y) { 1 } else { 0 };
    m as Integer == 2 && d == 28 + leap_adjustment
}
