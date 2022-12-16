use std::{collections::HashSet, sync::Arc};

use crate::types::{Integer, Natural, Size};

use crate::time::{
    businessdayconvention::BusinessDayConvention, date::Date, period::Period, timeunit::TimeUnit,
    weekday::Weekday, Day, Year,
};

// -------------------------------------------------------------------------------------------------

pub trait Weekend {
    fn is_weekend(&self, weekday: Weekday) -> bool;
}

/// Definition of a Western weeekend
pub struct WesternWeekend {}

impl Weekend for WesternWeekend {
    fn is_weekend(&self, weekday: Weekday) -> bool {
        weekday == Weekday::Saturday || weekday == Weekday::Sunday
    }
}

/// Definition of an Orthodox weeekend
pub struct OrthodoxWeekend {}

impl Weekend for OrthodoxWeekend {
    fn is_weekend(&self, weekday: Weekday) -> bool {
        weekday == Weekday::Saturday || weekday == Weekday::Sunday
    }
}

// -------------------------------------------------------------------------------------------------

pub trait Holiday {
    /// Returns the name of the calendar.
    /// This method is used for output and comparison between calendars. It is **not** meant
    /// to be used for writing switch-on-type code.    
    fn name(&self) -> String;

    /// Returns `true` iff the date is a business day for the given market.
    fn is_business_day(&self, date: &Date) -> bool;

    /// Returns `true` iff the weekday is part of the weekend for the given market.
    fn is_weekend(&self, weekday: Weekday) -> bool;
}

// -------------------------------------------------------------------------------------------------

/// Calendar provides methods for determining whether a date is a business day or a holiday
/// for a given market, and for incrementing/decrementing a date of a given number of business days.
///
/// A calendar should be defined for specific exchange holiday schedule or for general country
/// holiday schedule. 
#[derive(Clone)]
pub struct Calendar {
    holiday: Arc<dyn Holiday>,
    added_holidays: HashSet<Date>,
    removed_holidays: HashSet<Date>,
}

impl Calendar {
    pub(crate) fn new(holiday: Arc<dyn Holiday>) -> Self {
        Self {
            holiday,
            added_holidays: HashSet::new(),
            removed_holidays: HashSet::new(),
        }
    }

    pub fn name(&self) -> String {
        self.holiday.name()
    }

    /// Returns the set of added holidays for the given calendar
    pub fn added_holidays(&self) -> &HashSet<Date> {
        &self.added_holidays
    }

    /// Returns the set of removed holidays for the given calendar
    pub fn removed_holidays(&self) -> &HashSet<Date> {
        &self.removed_holidays
    }

    /// Clear the set of added and removed holidays
    pub fn reset_added_and_removed_holidays(&mut self) {
        self.added_holidays.clear();
        self.removed_holidays.clear();
    }

    /// Returns `true` iff the date is a business day for the given market.
    fn is_business_day(&self, date: &Date) -> bool {
        if !self.added_holidays.is_empty() && self.added_holidays.contains(date) {
            return false;
        }
        if !self.removed_holidays.is_empty() && self.removed_holidays.contains(date) {
            return true;
        }
        self.holiday.is_business_day(date)
    }

    /// Returns `true` iff the date is a holiday for the given market
    pub fn is_holiday(&self, date: &Date) -> bool {
        !self.holiday.is_business_day(date)
    }

    /// Returns `true` iff the weekday is part of the weekend for the given market.    
    pub fn is_weekend(&self, weekday: Weekday) -> bool {
        self.holiday.is_weekend(weekday)
    }

    /// Returns `true` iff in the given market, the date is on or after the last business day
    /// for that month.
    pub fn is_end_of_month(&self, date: &Date) -> bool {
        let date_plus_one = date + 1;
        date.month() != self.adjust_with_following(date_plus_one).month()
    }

    /// Last business day of the month to which the given date belongs
    pub fn end_of_month(&self, date: &Date) -> Date {
        self.adjust(date.end_of_month(), BusinessDayConvention::Preceding)
    }

    /// Adds a date to the set of holidays for the given calendar.
    pub fn add_holiday(&mut self, date: Date) {
        // if date was a genuine holiday previously removed, revert the change
        self.removed_holidays.remove(&date);
        // if it's already a holiday, leave the calendar alone, otherwise, add it.
        if self.holiday.is_business_day(&date) {
            self.added_holidays.insert(date);
        }
    }

    /// Removes a date from the set of holidays for the given calendar.
    pub fn remove_holiday(&mut self, date: Date) {
        // if date was an artificially-added holiday, revert the change
        self.added_holidays.remove(&date);
        // if it's already a business day, leave the calendar alone, otherwise, add it to
        // the removed holidays.
        if !self.holiday.is_business_day(&date) {
            self.removed_holidays.insert(date);
        }
    }

    /// Returns the holidays between two dates.
    pub fn holiday_list(&self, from: Date, to: Date, include_weekends: bool) -> Vec<Date> {
        assert!(
            to >= from,
            "'from' date ({:?}) must be equal or earlier than 'to' date ({:?})",
            from,
            to
        );
        let mut result = vec![];
        let mut d = from;
        while d <= to {
            if self.is_holiday(&d) && (include_weekends || !self.is_weekend(d.weekday())) {
                result.push(d);
            }
            d += 1;
        }
        result
    }

    /// Returns the business days between two dates.
    pub fn business_day_list(&self, from: Date, to: Date) -> Vec<Date> {
        assert!(
            to >= from,
            "'from' date ({:?}) must be equal or earlier than 'to' date ({:?})",
            from,
            to
        );
        let mut result = vec![];
        let mut d = from;
        while d <= to {
            if self.is_business_day(&d) {
                result.push(d);
            }
            d += 1;
        }
        result
    }

    /// Adjusts a non-business day to the appropriate near business day using the
    /// [BusinessDayConvention::Following]
    pub fn adjust_with_following(&self, date: Date) -> Date {
        self.adjust(date, BusinessDayConvention::Following)
    }

    /// Adjusts a non-business day to the appropriate near business day with respect to the
    /// given convention.
    pub fn adjust(&self, date: Date, convention: BusinessDayConvention) -> Date {
        if convention == BusinessDayConvention::Unadjusted {
            return date;
        }
        let mut d1 = date;
        if convention == BusinessDayConvention::Following
            || convention == BusinessDayConvention::ModifiedFollowing
            || convention == BusinessDayConvention::HalfMonthModifiedFollowing
        {
            while self.is_holiday(&d1) {
                d1 += 1;
            }
            if convention == BusinessDayConvention::ModifiedFollowing
                || convention == BusinessDayConvention::HalfMonthModifiedFollowing
            {
                if d1.month() != date.month() {
                    return self.adjust(date, BusinessDayConvention::Preceding);
                }
                if convention == BusinessDayConvention::HalfMonthModifiedFollowing
                    && date.day_of_month() <= 15
                    && d1.day_of_month() > 15
                {
                    return self.adjust(date, BusinessDayConvention::Preceding);
                }
            }
        } else if convention == BusinessDayConvention::Preceding
            || convention == BusinessDayConvention::ModifiedPreceding
        {
            while self.is_holiday(&d1) {
                d1 -= 1;
            }
            if convention == BusinessDayConvention::ModifiedPreceding && d1.month() != date.month()
            {
                return self.adjust(date, BusinessDayConvention::Following);
            }
        } else if convention == BusinessDayConvention::Nearest {
            let mut d2 = date;
            while self.is_holiday(&d1) && self.is_holiday(&d2) {
                d1 += 1;
                d2 -= 1;
            }
            if self.is_holiday(&d1) {
                return d2;
            } else {
                return d1;
            }
        } else {
            panic!("Unknown business day convention: {:?}", convention);
        }
        d1
    }

    /// Advances the given date by the given number of business days using
    /// [BusinessDayConvention::Following]
    pub fn advance_by_days_with_following(
        &self,
        date: Date,
        n: Integer,
        unit: TimeUnit,
        end_of_month: bool,
    ) -> Date {
        self.advance_by_days(
            date,
            n,
            unit,
            BusinessDayConvention::Following,
            end_of_month,
        )
    }

    /// Advances the given date by the given number of business days and returns the result.
    pub fn advance_by_days(
        &self,
        date: Date,
        n: Integer,
        unit: TimeUnit,
        convention: BusinessDayConvention,
        end_of_month: bool,
    ) -> Date {
        let mut n = n;
        if n == 0 {
            self.adjust(date, convention)
        } else if unit == TimeUnit::Days {
            let mut d1 = date;
            if n > 0 {
                while n > 0 {
                    d1 += 1;
                    while self.is_holiday(&d1) {
                        d1 += 1;
                    }
                    n -= 1;
                }
            } else {
                while n < 0 {
                    d1 -= 1;
                    while self.is_holiday(&d1) {
                        d1 -= 1;
                    }
                    n += 1;
                }
            }
            d1
        } else if unit == TimeUnit::Weeks {
            // n needs to be positive as a Period cannot have negative length.
            let period = Period::new(n, unit);
            let d1 = date + period;
            self.adjust(d1, convention)
        } else {
            let period = Period::new(n, unit);
            let d1 = date + period;
            // we are sure the unit is Months or Years
            if end_of_month && self.is_end_of_month(&date) {
                return self.end_of_month(&d1);
            }
            self.adjust(d1, convention)
        }
    }

    /// Advances the given date as specified by the given period using
    /// [BusinessDayConvention::Following]
    pub fn advance_by_period_with_following(
        &self,
        date: Date,
        period: &Period,
        end_of_month: bool,
    ) -> Date {
        self.advance_by_period(date, period, BusinessDayConvention::Following, end_of_month)
    }

    /// Advances the given date as specified by the given period and returns the result.
    pub fn advance_by_period(
        &self,
        date: Date,
        period: &Period,
        convention: BusinessDayConvention,
        end_of_month: bool,
    ) -> Date {
        self.advance_by_days(date, period.length, period.unit, convention, end_of_month)
    }

    /// Calculates the number of business days between two given dates and returns the result.
    pub fn business_days_between(
        &self,
        from: Date,
        to: Date,
        include_first: bool,
        include_last: bool,
    ) -> Integer {
        let mut wd = 0;
        if from != to {
            if from < to {
                // the last one is treated separately to avoid incrementing Date::maxDate()
                let mut d = from;
                while d < to {
                    if self.is_business_day(&d) {
                        wd += 1;
                    }
                    d += 1;
                }
                if self.is_business_day(&to) {
                    wd += 1;
                }
            } else if from > to {
                let mut d = to;
                while d < from {
                    if self.is_business_day(&d) {
                        wd += 1;
                    }
                    d += 1;
                }
                if self.is_business_day(&from) {
                    wd += 1;
                }
            }
            if self.is_business_day(&from) && !include_first {
                wd -= 1;
            }
            if self.is_business_day(&to) && !include_last {
                wd -= 1;
            }
            if from > to {
                wd = -wd;
            }
        } else if include_first && include_last && self.is_business_day(&from) {
            wd = 1;
        }
        wd
    }
}

pub fn easter_monday(year: Year) -> Day {
    assert!(
        (1900..2200).contains(&year),
        "Year must be in the range (1900, 2200)"
    );
    // NOTE: guaranteed not to panic due to the assertion above;
    let idx: Size = (year - 1900).try_into().unwrap();
    EASTER_MONDAYS[idx]
}

pub fn easter_monday_orthodox(year: Year) -> Day {
    assert!(
        (1900..2200).contains(&year),
        "Year must be in the range (1900, 2200)"
    );
    // NOTE: guaranteed not to panic due to the assertion above;
    let idx: Size = (year - 1900).try_into().unwrap();
    ORTHODOX_EASTER_MONDAYS[idx]
}

// Note that unlike Quantlib C++, we start from the year 1900 (and not 1901)
const EASTER_MONDAYS: [Natural; 300] = [
    106, 98, 90, 103, 95, 114, 106, 91, 111, 102, // 1900-1909
    87, 107, 99, 83, 103, 95, 115, 99, 91, 111, // 1910-1919
    96, 87, 107, 92, 112, 103, 95, 108, 100, 91, // 1920-1929
    111, 96, 88, 107, 92, 112, 104, 88, 108, 100, // 1930-1939
    85, 104, 96, 116, 101, 92, 112, 97, 89, 108, // 1940-1949
    100, 85, 105, 96, 109, 101, 93, 112, 97, 89, // 1950-1959
    109, 93, 113, 105, 90, 109, 101, 86, 106, 97, // 1960-1969
    89, 102, 94, 113, 105, 90, 110, 101, 86, 106, // 1970-1979
    98, 110, 102, 94, 114, 98, 90, 110, 95, 86, // 1980-1989
    106, 91, 111, 102, 94, 107, 99, 90, 103, 95, // 1990-1999
    115, 106, 91, 111, 103, 87, 107, 99, 84, 103, // 2000-2009
    95, 115, 100, 91, 111, 96, 88, 107, 92, 112, // 2010-2019
    104, 95, 108, 100, 92, 111, 96, 88, 108, 92, // 2020-2029
    112, 104, 89, 108, 100, 85, 105, 96, 116, 101, // 2030-2039
    93, 112, 97, 89, 109, 100, 85, 105, 97, 109, // 2040-2049
    101, 93, 113, 97, 89, 109, 94, 113, 105, 90, // 2050-2059
    110, 101, 86, 106, 98, 89, 102, 94, 114, 105, // 2060-2069
    90, 110, 102, 86, 106, 98, 111, 102, 94, 114, // 2070-2079
    99, 90, 110, 95, 87, 106, 91, 111, 103, 94, // 2080-2089
    107, 99, 91, 103, 95, 115, 107, 91, 111, 103, // 2090-2099
    88, 108, 100, 85, 105, 96, 109, 101, 93, 112, // 2100-2109
    97, 89, 109, 93, 113, 105, 90, 109, 101, 86, // 2110-2119
    106, 97, 89, 102, 94, 113, 105, 90, 110, 101, // 2120-2129
    86, 106, 98, 110, 102, 94, 114, 98, 90, 110, // 2130-2139
    95, 86, 106, 91, 111, 102, 94, 107, 99, 90, // 2140-2149
    103, 95, 115, 106, 91, 111, 103, 87, 107, 99, // 2150-2159
    84, 103, 95, 115, 100, 91, 111, 96, 88, 107, // 2160-2169
    92, 112, 104, 95, 108, 100, 92, 111, 96, 88, // 2170-2179
    108, 92, 112, 104, 89, 108, 100, 85, 105, 96, // 2180-2189
    116, 101, 93, 112, 97, 89, 109, 100, 85, 105, // 2190-2199
];

// Note that unlike Quantlib C++, we start from the year 1900 (and not 1901)
const ORTHODOX_EASTER_MONDAYS: [Natural; 300] = [
    113, 105, 118, 110, 102, 121, 106, 126, 118, 102, // 1900-1909
    122, 114, 99, 118, 110, 95, 115, 106, 126, 111, // 1910-1919
    103, 122, 107, 99, 119, 110, 123, 115, 107, 126, // 1920-1929
    111, 103, 123, 107, 99, 119, 104, 123, 115, 100, // 1930-1939
    120, 111, 96, 116, 108, 127, 112, 104, 124, 115, // 1940-1949
    100, 120, 112, 96, 116, 108, 128, 112, 104, 124, // 1950-1959
    109, 100, 120, 105, 125, 116, 101, 121, 113, 104, // 1960-1969
    117, 109, 101, 120, 105, 125, 117, 101, 121, 113, // 1970-1979
    98, 117, 109, 129, 114, 105, 125, 110, 102, 121, // 1980-1989
    106, 98, 118, 109, 122, 114, 106, 118, 110, 102, // 1990-1999
    122, 106, 126, 118, 103, 122, 114, 99, 119, 110, // 2000-2009
    95, 115, 107, 126, 111, 103, 123, 107, 99, 119, // 2010-2019
    111, 123, 115, 107, 127, 111, 103, 123, 108, 99, // 2020-2029
    119, 104, 124, 115, 100, 120, 112, 96, 116, 108, // 2030-2039
    128, 112, 104, 124, 116, 100, 120, 112, 97, 116, // 2040-2049
    108, 128, 113, 104, 124, 109, 101, 120, 105, 125, // 2050-2059
    117, 101, 121, 113, 105, 117, 109, 101, 121, 105, // 2060-2069
    125, 110, 102, 121, 113, 98, 118, 109, 129, 114, // 2070-2079
    106, 125, 110, 102, 122, 106, 98, 118, 110, 122, // 2080-2089
    114, 99, 119, 110, 102, 115, 107, 126, 118, 103, // 2090-2099
    123, 115, 100, 120, 112, 96, 116, 108, 128, 112, // 2100-2109
    104, 124, 109, 100, 120, 105, 125, 116, 108, 121, // 2110-2119
    113, 104, 124, 109, 101, 120, 105, 125, 117, 101, // 2120-2129
    121, 113, 98, 117, 109, 129, 114, 105, 125, 110, // 2130-2139
    102, 121, 113, 98, 118, 109, 129, 114, 106, 125, // 2140-2149
    110, 102, 122, 106, 126, 118, 103, 122, 114, 99, // 2150-2159
    119, 110, 102, 115, 107, 126, 111, 103, 123, 114, // 2160-2169
    99, 119, 111, 130, 115, 107, 127, 111, 103, 123, // 2170-2179
    108, 99, 119, 104, 124, 115, 100, 120, 112, 103, // 2180-2189
    116, 108, 128, 119, 104, 124, 116, 100, 120, 112, // 2190-2199
];

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::time::{
        calendars::{brazil::Brazil, target::Target},
        date::Date,
        months::Month,
        period::Period,
        timeunit::TimeUnit,
    };

    #[test]
    fn test_end_of_month() {
        // any calendar should be ok
        let c = Target::new();

        let mut counter = Date::min_date();
        let last = Date::max_date() - Period::new(2, TimeUnit::Months);

        while counter <= last {
            let eom = c.end_of_month(&counter);
            // check that eom is eom
            assert!(
                c.is_end_of_month(&eom),
                "{:?}, {:?} is not the last business day in {:?} {:?} according to {}",
                eom.weekday(),
                eom,
                eom.month(),
                eom.year(),
                c.name()
            );
            // check that eom is in the same month as counter
            assert!(
                eom.month() == counter.month(),
                "{:?} is not the same month as {:?}",
                eom,
                counter
            );
            counter += 1
        }
    }

    #[allow(unused)]
    #[test]
    fn test_business_days_between() {
        let test_dates = vec![
            Date::new(1, Month::February, 2002),  // is_business_day = true
            Date::new(4, Month::February, 2002),  // is_business_day = true
            Date::new(16, Month::May, 2003),      // is_business_day = true
            Date::new(17, Month::December, 2003), // is_business_day = true
            Date::new(17, Month::December, 2004), // is_business_day = true
            Date::new(19, Month::December, 2005), // is_business_day = true
            Date::new(2, Month::January, 2006),   // is_business_day = true
            Date::new(13, Month::March, 2006),    // is_business_day = true
            Date::new(15, Month::May, 2006),      // is_business_day = true
            Date::new(17, Month::March, 2006),    // is_business_day = true
            Date::new(15, Month::May, 2006),      // is_business_day = true
            Date::new(26, Month::July, 2006),     // is_business_day = true
            Date::new(26, Month::July, 2006),     // is_business_day = true
            Date::new(27, Month::July, 2006),     // is_business_day = true
            Date::new(29, Month::July, 2006),     // is_business_day = false
            Date::new(29, Month::July, 2006),     // is_business_day = false
        ];

        // default params: from date included, to excluded
        let expected = vec![1, 321, 152, 251, 252, 10, 48, 42, -38, 38, 51, 0, 1, 2, 0];

        // exclude from, include to
        let expected_include_to = vec![1, 321, 152, 251, 252, 10, 48, 42, -38, 38, 51, 0, 1, 1, 0];

        // include both from and to
        let expected_include_all = vec![2, 322, 153, 252, 253, 11, 49, 43, -39, 39, 52, 1, 2, 2, 0];

        // exclude both from and to
        let expected_exclude_all = vec![0, 320, 151, 250, 251, 9, 47, 41, -37, 37, 50, 0, 0, 1, 0];

        let c = Brazil::new();
        for i in 1..test_dates.len() {
            let calculated = c.business_days_between(
                test_dates[i - 1].clone(), // TODO remove clone
                test_dates[i].clone(),     // TODO remove clone
                true,
                false,
            );
            assert!(
                calculated == expected[i - 1],
                "from {:?} included to {:?} excluded, calculated: \
                     {}, expected: {:?}",
                test_dates[i - 1],
                test_dates[i],
                calculated,
                expected[i - 1]
            );

            let calculated = c.business_days_between(
                test_dates[i - 1].clone(), // TODO remove clone
                test_dates[i].clone(),     // TODO remove clone
                false,
                true,
            );
            assert!(
                calculated == expected_include_to[i - 1],
                "from {:?} excluded to {:?} included, calculated: \
                     {}, expected: {:?}",
                test_dates[i - 1],
                test_dates[i],
                calculated,
                expected_include_to[i - 1]
            );

            let calculated = c.business_days_between(
                test_dates[i - 1].clone(), // TODO remove clone
                test_dates[i].clone(),     // TODO remove clone
                true,
                true,
            );
            assert!(
                calculated == expected_include_all[i - 1],
                "from {:?} included to {:?} included, calculated: \
                     {}, expected: {:?}",
                test_dates[i - 1],
                test_dates[i],
                calculated,
                expected_include_all[i - 1]
            );

            let calculated = c.business_days_between(
                test_dates[i - 1].clone(), // TODO remove clone
                test_dates[i].clone(),     // TODO remove clone
                false,
                false,
            );
            assert!(
                calculated == expected_exclude_all[i - 1],
                "from {:?} excluded to {:?} excluded, calculated: \
                     {}, expected: {:?}",
                test_dates[i - 1],
                test_dates[i],
                calculated,
                expected_exclude_all[i - 1]
            );
        }
    }
}
