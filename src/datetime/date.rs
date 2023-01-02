use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::datetime::{
    months::Month, period::Period, timeunit::TimeUnit, weekday::Weekday, Day, SerialNumber, Year,
};
use crate::types::{BigInteger, Integer, Natural, Size, Time};
use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Eq)]
pub struct Date {
    date_time: DateTime<Utc>,
    // This is the way Excel, LibraOffice etc, stores dates. By default, 1900-01-01 is serial
    // number 2 and 2008-01-01 is serial number 39448.
    serial_number: SerialNumber,
}

impl Debug for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.date_time.to_rfc3339())
    }
}

// -------------------------------------------------------------------------------------------------

impl Default for Date {
    fn default() -> Self {
        Date::from_serial(2)
    }
}

// -------------------------------------------------------------------------------------------------

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.serial_number.partial_cmp(&other.serial_number)
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.serial_number.cmp(&other.serial_number)
    }
}

// -------------------------------------------------------------------------------------------------

impl Hash for Date {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.serial_number.hash(state);
    }
}

// -------------------------------------------------------------------------------------------------

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.serial_number == other.serial_number
    }
}

// -------------------------------------------------------------------------------------------------

impl Add<SerialNumber> for Date {
    type Output = Self;

    fn add(self, rhs: SerialNumber) -> Self {
        Date::from_serial(self.serial_number + rhs)
    }
}

impl<'a> Add<SerialNumber> for &'a Date {
    type Output = Date;

    fn add(self, rhs: SerialNumber) -> Self::Output {
        Date::from_serial(self.serial_number + rhs)
    }
}

impl AddAssign<SerialNumber> for Date {
    fn add_assign(&mut self, rhs: SerialNumber) {
        self.date_time = Date::min_date().date_time
            + Duration::days(self.serial_number as BigInteger + rhs as BigInteger - 2);
        self.serial_number += rhs;
    }
}

// -------------------------------------------------------------------------------------------------

impl Sub<SerialNumber> for Date {
    type Output = Self;

    fn sub(self, rhs: SerialNumber) -> Self::Output {
        Date::from_serial(self.serial_number - rhs)
    }
}

impl<'a> Sub<SerialNumber> for &'a Date {
    type Output = Date;

    fn sub(self, rhs: SerialNumber) -> Self::Output {
        Date::from_serial(self.serial_number - rhs)
    }
}

impl SubAssign<SerialNumber> for Date {
    fn sub_assign(&mut self, rhs: SerialNumber) {
        self.date_time = Date::min_date().date_time
            + Duration::days(self.serial_number as BigInteger - rhs as BigInteger - 2);
        self.serial_number -= rhs;
    }
}

// -------------------------------------------------------------------------------------------------

impl Add<Period> for Date {
    type Output = Self;

    fn add(self, rhs: Period) -> Self::Output {
        self.advance(rhs.length, rhs.unit)
    }
}

impl<'a> Add<&'a Period> for &'a Date {
    type Output = Date;

    fn add(self, rhs: &Period) -> Self::Output {
        self.advance(rhs.length, rhs.unit)
    }
}

impl<'a> Add<Period> for &'a Date {
    type Output = Date;

    fn add(self, rhs: Period) -> Self::Output {
        self.advance(rhs.length, rhs.unit)
    }
}

impl AddAssign<Period> for Date {
    fn add_assign(&mut self, rhs: Period) {
        *self = self.advance(rhs.length, rhs.unit);
    }
}

impl Sub<Period> for Date {
    type Output = Self;

    fn sub(self, rhs: Period) -> Self::Output {
        self.advance(-(rhs.length), rhs.unit)
    }
}

impl<'a> Sub<Period> for &'a Date {
    type Output = Date;

    fn sub(self, rhs: Period) -> Self::Output {
        self.advance(-(rhs.length), rhs.unit)
    }
}

impl SubAssign<Period> for Date {
    fn sub_assign(&mut self, rhs: Period) {
        *self = self.advance(-rhs.length, rhs.unit);
    }
}

// -------------------------------------------------------------------------------------------------

impl Sub for Date {
    type Output = SerialNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        self.serial_number() - rhs.serial_number()
    }
}

impl<'a> Sub for &'a Date {
    type Output = SerialNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        self.serial_number() - rhs.serial_number()
    }
}

// -------------------------------------------------------------------------------------------------

impl Date {
    /// Create a new [Date] from day, [Month] and year.
    pub fn new(d: Day, m: Month, y: Year) -> Self {
        assert!(
            (1900..2200).contains(&y),
            "Year must be in the range [1900, 2200)"
        );
        assert!(
            (Into::<Integer>::into(m) > 0 && Into::<Integer>::into(m) < 13),
            "Month is outside [January, December] range"
        );
        let leap = Date::is_leap(y);
        let len = Date::month_length(m, leap);
        assert!(
            d <= len && d > 0,
            "day outside month ({:?}) day-range [1, {}])",
            m,
            len
        );

        let month_offset = Date::month_offset(m, leap);
        let year_offset = Date::year_offset(y);
        let serial_number = 1 + d + month_offset + year_offset;
        let date_time = Utc.with_ymd_and_hms(y, m.into(), d, 0, 0, 0).unwrap();
        Date {
            date_time,
            // NOTE: If this conversion fails, we'll just panic as that's the best course
            // of action.
            serial_number: Integer::try_from(serial_number)
                .unwrap_or_else(|_| panic!("date serial number ({}) out of range", serial_number)),
        }
    }

    /// Create a [Date] taking a serial number as given by Applix or Excel.
    pub fn from_serial(serial_number: SerialNumber) -> Self {
        // check serial number
        assert!(
            serial_number >= MINIMUM_SERIAL_NUMBER,
            "Date's serial number ({}) is less than minimum ({}), i.e. ({:?})",
            serial_number,
            MINIMUM_SERIAL_NUMBER,
            Date::min_date()
        );
        assert!(
            serial_number <= MAXIMUM_SERIAL_NUMBER,
            "Date's serial number ({}) is greater than maximum ({}), i.e. ({:?})",
            serial_number,
            MAXIMUM_SERIAL_NUMBER,
            Date::max_date()
        );

        let date_time =
            Date::min_date().date_time + Duration::days(serial_number as BigInteger - 2);
        Date {
            date_time,
            serial_number,
        }
    }

    pub fn todays_date() -> Date {
        let now = Utc::now();
        let d = now.day();
        let m = now.month();
        let y = now.year();
        Date::new(d, m.into(), y)
    }

    pub fn serial_number(&self) -> SerialNumber {
        self.serial_number
    }

    pub fn weekday(&self) -> Weekday {
        let w = self.serial_number % 7;
        Weekday::from(w)
    }

    pub fn day_of_month(&self) -> Day {
        self.date_time.day()
    }

    pub fn day_of_year(&self) -> Day {
        self.date_time.ordinal()
    }

    pub fn month(&self) -> Month {
        (self.date_time.month()).into()
    }

    pub fn year(&self) -> Year {
        self.date_time.year()
    }

    pub fn end_of_month(&self) -> Self {
        let m = self.month();
        let y = self.year();
        let d = Date::month_length(m, Date::is_leap(y));
        Date::new(d, m, y)
    }

    pub fn is_end_of_month(&self) -> bool {
        let leap = Date::is_leap(self.year());
        self.day_of_month() == Date::month_length(self.month(), leap)
    }

    pub fn min_date() -> Self {
        Date {
            // Jan 1st, 1900
            date_time: Utc.with_ymd_and_hms(1900, 1, 1, 0, 0, 0).unwrap(),
            serial_number: MINIMUM_SERIAL_NUMBER,
        }
    }

    pub fn max_date() -> Self {
        Date {
            // Dec 31st, 2199
            date_time: Utc.with_ymd_and_hms(2199, 12, 31, 0, 0, 0).unwrap(),
            serial_number: MAXIMUM_SERIAL_NUMBER,
        }
    }

    /// Difference in days (including fraction of days) between dates
    pub fn days_between(d1: &Date, d2: &Date) -> Time {
        (d2 - d1).into()
    }

    /// n-th given weekday in the given month and year
    /// E.g., the 4th Thursday of March, 1998 was March 26th, 1998.
    ///
    /// see <http://www.cpearson.com/excel/DateTimeWS.htm>
    pub fn nth_weekday(nth: Size, day_of_week: Weekday, m: Month, y: Year) -> Date {
        assert!(
            nth > 0,
            "zeroth day of week in a given (month, year) is undefined"
        );
        assert!(nth < 6, "no more than 5 weekdays in a given (month, year)");
        let first = Date::new(1, m, y).weekday();
        let skip = if day_of_week >= first { nth - 1 } else { nth };

        let d = (1 + day_of_week as Natural + skip as Natural * 7) - first as Natural;
        Date::new(d, m, y)
    }

    pub fn is_leap(y: Year) -> bool {
        assert!(
            (1900..2200).contains(&y),
            "Year must be in the range (1900, 2200)"
        );
        // NOTE: guaranteed not to panic due to the assertion above;
        let idx: Size = (y - 1900).try_into().unwrap();
        YEAR_IS_LEAP[idx]
    }

    fn month_length(m: Month, leap_year: bool) -> Day {
        let idx: Size = m.into();
        assert!(idx > 0, "Invalid month: {:?}", m);
        if leap_year {
            MONTH_LEAP_LENGTH[idx - 1]
        } else {
            MONTH_LENGTH[idx - 1]
        }
    }

    fn month_offset(m: Month, leap_year: bool) -> Day {
        let idx: Size = m.into();
        assert!(idx > 0, "Invalid month: {:?}", m);
        if leap_year {
            MONTH_LEAP_OFFSET[idx - 1]
        } else {
            MONTH_OFFSET[idx - 1]
        }
    }

    fn year_offset(y: Year) -> Natural {
        assert!(
            (1900..2200).contains(&y),
            "Year must be in the range (1900, 2200)"
        );
        // NOTE: guaranteed not to panic due to the assertion above;
        let idx: Size = (y - 1900).try_into().unwrap();
        YEAR_OFFSET[idx]
    }

    /// Advance the date by the given amount of time units
    fn advance(&self, n: Integer, unit: TimeUnit) -> Date {
        match unit {
            TimeUnit::Days => self + n,
            TimeUnit::Weeks => self + 7 * n,
            TimeUnit::Months => {
                let mut d = self.day_of_month();
                let m_ordinal: Integer = self.month().into();
                let mut m = m_ordinal + n;
                let mut y = self.year();
                while m > 12 {
                    m -= 12;
                    y += 1;
                }
                while m < 1 {
                    m += 12;
                    y -= 1;
                }
                assert!(
                    (1900..2200).contains(&y),
                    "Year must be in the range (1900, 2199)"
                );
                let length = Date::month_length(m.into(), Date::is_leap(y));
                if d > length {
                    d = length;
                }
                Date::new(d, m.into(), y)
            }
            TimeUnit::Years => {
                let mut d = self.day_of_month();
                let m = self.month();
                // we need to be able to subtract years in case `n` is negative
                let y = self.year() + n;
                assert!(
                    (1900..2200).contains(&y),
                    "Year must be in the range (1900, 2199)"
                );
                if d == 29 && m == Month::February && !Date::is_leap(y) {
                    d = 28;
                }
                Date::new(d, m, y)
            }
            other => panic!("Invalid time unit: {:?}", other),
        }
    }
}

// -------------------------------------------------------------------------------------------------

const YEAR_IS_LEAP: [bool; 301] = [
    // Note that in the Quantlib C++ implementation, 1900 is taken as a leap year because
    // that's how Excel treats it (it's a bug in Excel). However, we correct that here,
    // i.e. 1900 is not a leap year.
    false, false, false, false, true, false, false, false, true, false, // 1900-1909
    false, false, true, false, false, false, true, false, false, false, // 1910-1919
    true, false, false, false, true, false, false, false, true, false, // 1920-1929
    false, false, true, false, false, false, true, false, false, false, // 1930-1939
    true, false, false, false, true, false, false, false, true, false, // 1940-1949
    false, false, true, false, false, false, true, false, false, false, // 1950-1959
    true, false, false, false, true, false, false, false, true, false, // 1960-1969
    false, false, true, false, false, false, true, false, false, false, // 1970-1979
    true, false, false, false, true, false, false, false, true, false, // 1980-1989
    false, false, true, false, false, false, true, false, false, false, // 1990-1999
    true, false, false, false, true, false, false, false, true, false, // 2000-2009
    false, false, true, false, false, false, true, false, false, false, // 2010-2019
    true, false, false, false, true, false, false, false, true, false, // 2020-2029
    false, false, true, false, false, false, true, false, false, false, // 2030-2039
    true, false, false, false, true, false, false, false, true, false, // 2040-2049
    false, false, true, false, false, false, true, false, false, false, // 2050-2059
    true, false, false, false, true, false, false, false, true, false, // 2060-2069
    false, false, true, false, false, false, true, false, false, false, // 2070-2079
    true, false, false, false, true, false, false, false, true, false, // 2080-2089
    false, false, true, false, false, false, true, false, false, false, // 2090-2099
    false, false, false, false, true, false, false, false, true, false, // 2100-2109
    false, false, true, false, false, false, true, false, false, false, // 2110-2119
    true, false, false, false, true, false, false, false, true, false, // 2120-2129
    false, false, true, false, false, false, true, false, false, false, // 2130-2139
    true, false, false, false, true, false, false, false, true, false, // 2140-2149
    false, false, true, false, false, false, true, false, false, false, // 2150-2159
    true, false, false, false, true, false, false, false, true, false, // 2160-2169
    false, false, true, false, false, false, true, false, false, false, // 2170-2179
    true, false, false, false, true, false, false, false, true, false, // 2180-2189
    false, false, true, false, false, false, true, false, false, false, // 2190-2199
    false, // 2200
];

const MONTH_LENGTH: [Day; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

const MONTH_LEAP_LENGTH: [Day; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

const MONTH_OFFSET: [Natural; 13] = [
    0, 31, 59, 90, 120, 151, // Jan - Jun
    181, 212, 243, 273, 304, 334, // Jun - Dec
    365, // used in dayOfMonth to bracket day]
];

const MONTH_LEAP_OFFSET: [Natural; 13] = [
    0, 31, 60, 91, 121, 152, // Jan - Jun
    182, 213, 244, 274, 305, 335, // Jun - Dec
    366, // used in dayOfMonth to bracket day
];

// Note that in the Quantlib C++ implementation, 1900 is taken as a leap year because
// that's how Excel treats it (it's a bug in Excel). However, we correct that here,
// i.e. 1900 is not a leap year.
const YEAR_OFFSET: [Natural; 301] = [
    0, 365, 730, 1095, 1460, 1826, 2191, 2556, 2921, 3287, // 1900-1909
    3652, 4017, 4382, 4748, 5113, 5478, 5843, 6209, 6574, 6939, // 1910-1919
    7304, 7670, 8035, 8400, 8765, 9131, 9496, 9861, 10226, 10592, // 1920-1929
    10957, 11322, 11687, 12053, 12418, 12783, 13148, 13514, 13879, 14244, // 1930-1939
    14609, 14975, 15340, 15705, 16070, 16436, 16801, 17166, 17531, 17897, // 1940-1949
    18262, 18627, 18992, 19358, 19723, 20088, 20453, 20819, 21184, 21549, // 1950-1959
    21914, 22280, 22645, 23010, 23375, 23741, 24106, 24471, 24836, 25202, // 1960-1969
    25567, 25932, 26297, 26663, 27028, 27393, 27758, 28124, 28489, 28854, // 1970-1979
    29219, 29585, 29950, 30315, 30680, 31046, 31411, 31776, 32141, 32507, // 1980-1989
    32872, 33237, 33602, 33968, 34333, 34698, 35063, 35429, 35794, 36159, // 1990-1999
    36524, 36890, 37255, 37620, 37985, 38351, 38716, 39081, 39446, 39812, // 2000-2009
    40177, 40542, 40907, 41273, 41638, 42003, 42368, 42734, 43099, 43464, // 2010-2019
    43829, 44195, 44560, 44925, 45290, 45656, 46021, 46386, 46751, 47117, // 2020-2029
    47482, 47847, 48212, 48578, 48943, 49308, 49673, 50039, 50404, 50769, // 2030-2039
    51134, 51500, 51865, 52230, 52595, 52961, 53326, 53691, 54056, 54422, // 2040-2049
    54787, 55152, 55517, 55883, 56248, 56613, 56978, 57344, 57709, 58074, // 2050-2059
    58439, 58805, 59170, 59535, 59900, 60266, 60631, 60996, 61361, 61727, // 2060-2069
    62092, 62457, 62822, 63188, 63553, 63918, 64283, 64649, 65014, 65379, // 2070-2079
    65744, 66110, 66475, 66840, 67205, 67571, 67936, 68301, 68666, 69032, // 2080-2089
    69397, 69762, 70127, 70493, 70858, 71223, 71588, 71954, 72319, 72684, // 2090-2099
    73049, 73414, 73779, 74144, 74509, 74875, 75240, 75605, 75970, 76336, // 2100-2109
    76701, 77066, 77431, 77797, 78162, 78527, 78892, 79258, 79623, 79988, // 2110-2119
    80353, 80719, 81084, 81449, 81814, 82180, 82545, 82910, 83275, 83641, // 2120-2129
    84006, 84371, 84736, 85102, 85467, 85832, 86197, 86563, 86928, 87293, // 2130-2139
    87658, 88024, 88389, 88754, 89119, 89485, 89850, 90215, 90580, 90946, // 2140-2149
    91311, 91676, 92041, 92407, 92772, 93137, 93502, 93868, 94233, 94598, // 2150-2159
    94963, 95329, 95694, 96059, 96424, 96790, 97155, 97520, 97885, 98251, // 2160-2169
    98616, 98981, 99346, 99712, 100077, 100442, 100807, 101173, 101538, 101903, // 2170-2179
    // 2180-2189
    102268, 102634, 102999, 103364, 103729, 104095, 104460, 104825, 105190, 105556,
    // 2190-2199
    105921, 106286, 106651, 107017, 107382, 107747, 108112, 108478, 108843, 109208,
    // 2200
    109573,
];

const MINIMUM_SERIAL_NUMBER: SerialNumber = 2; // Jan 1st, 1900
const MAXIMUM_SERIAL_NUMBER: SerialNumber = 109574; // Dec 31st, 2199

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::types::Integer;

    use crate::datetime::{date::Month, period::Period, timeunit::TimeUnit, weekday::Weekday};

    use super::Date;

    #[test]
    fn test_new_date() {
        let date = Date::new(1, Month::January, 1900);
        assert_eq!(date.serial_number(), 2);

        let date = Date::new(1, Month::January, 2008);
        assert_eq!(date.serial_number(), 39448);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.year(), 2008);
        assert_eq!(date.weekday(), Weekday::Tuesday);
        assert_eq!(date.day_of_month(), 1);
        assert_eq!(date.day_of_year(), 1);

        assert_eq!(date.end_of_month().day_of_month(), 31);
        assert!(date.end_of_month().is_end_of_month());
    }

    #[test]
    fn test_from_serial() {
        let date = Date::from_serial(39448);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.year(), 2008);
        assert_eq!(date.weekday(), Weekday::Tuesday);
        assert_eq!(date.day_of_month(), 1);
        assert_eq!(date.day_of_year(), 1);

        let date = Date::from_serial(44896);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.year(), 2022);
        assert_eq!(date.weekday(), Weekday::Thursday);
        assert_eq!(date.day_of_month(), 1);
        assert_eq!(date.day_of_year(), 335);
    }

    #[test]
    fn test_hash() {
        let mut dates = HashSet::new();
        dates.insert(Date::from_serial(39448));
        dates.insert(Date::from_serial(39448));
        dates.insert(Date::from_serial(44896));
        assert_eq!(dates.len(), 2);
        assert!(dates.contains(&Date::new(1, Month::January, 2008)));
        assert!(dates.contains(&Date::new(1, Month::December, 2022)));
    }

    #[test]
    fn test_add_serial_number() {
        let d = Date::new(1, Month::January, 2008);
        let d2 = d + 1;
        assert_eq!(d2, Date::new(2, Month::January, 2008));
    }

    #[test]
    fn test_add_assign_serial_number() {
        let mut d = Date::new(1, Month::January, 2008);
        d += 10;
        assert_eq!(d.serial_number, 39458);
        assert_eq!(d, Date::new(11, Month::January, 2008));
    }

    #[test]
    fn test_sub_serial_number() {
        let d = Date::new(1, Month::January, 2008);
        let d2 = d - 10;
        assert_eq!(d2, Date::new(22, Month::December, 2007));
    }

    #[test]
    fn test_sub_assign_serial_number() {
        let mut d = Date::new(1, Month::January, 2008);
        d -= 10;
        assert_eq!(d.serial_number, 39438);
        assert_eq!(d, Date::new(22, Month::December, 2007));
    }

    #[test]
    fn test_add_period() {
        let d = Date::new(1, Month::January, 2008);
        let p = Period::new(1, TimeUnit::Days);
        assert_eq!(d + p, Date::new(2, Month::January, 2008));

        let d = Date::new(1, Month::January, 2008);
        let p = Period::new(1, TimeUnit::Weeks);
        assert_eq!(d + p, Date::new(8, Month::January, 2008));

        let d = Date::new(1, Month::January, 2008);
        let p = Period::new(1, TimeUnit::Months);
        assert_eq!(d + p, Date::new(1, Month::February, 2008));

        let d = Date::new(1, Month::January, 2008);
        let p = Period::new(10, TimeUnit::Years);
        assert_eq!(d + p, Date::new(1, Month::January, 2018));
    }

    #[test]
    fn test_sub_period() {
        let d = Date::new(1, Month::January, 2008);
        let p = Period::new(1, TimeUnit::Days);
        assert_eq!(d - p, Date::new(31, Month::December, 2007));

        let d = Date::new(1, Month::January, 2008);
        let p = Period::new(1, TimeUnit::Weeks);
        assert_eq!(d - p, Date::new(25, Month::December, 2007));

        let d = Date::new(1, Month::January, 2008);
        let p = Period::new(1, TimeUnit::Months);
        assert_eq!(d - p, Date::new(1, Month::December, 2007));

        let d = Date::new(1, Month::January, 2008);
        let p = Period::new(10, TimeUnit::Years);
        assert_eq!(d - p, Date::new(1, Month::January, 1998));
    }

    #[test]
    fn test_days_between() {
        let d1 = Date::new(1, Month::January, 2022);
        let d2 = Date::new(1, Month::December, 2022);
        let days = Date::days_between(&d1, &d2);
        assert_eq!(days, 334.0);
    }

    #[test]
    fn test_consistency() {
        let min_date = Date::min_date().serial_number() + 1;
        let max_date = Date::max_date().serial_number();

        let mut dyold = Date::from_serial(min_date - 1).day_of_year();
        let mut dold = Date::from_serial(min_date - 1).day_of_month();
        let mut mold = Date::from_serial(min_date - 1).month();
        let mut yold = Date::from_serial(min_date - 1).year();
        let mut wold = Date::from_serial(min_date - 1).weekday();

        for i in min_date..=max_date {
            let t = Date::from_serial(i);
            let mut serial = t.serial_number();
            // check serial number consistency
            assert_eq!(serial, i);

            let dy = t.day_of_year();
            let d = t.day_of_month();
            let m = t.month();
            let y = t.year();
            let wd = t.weekday();

            // check if skipping any date
            // let is_leap = Date::is_leap(yold);
            // println!("i: {}, dyold: {}, yold: {}, is_leap: {}", i, dyold, yold, is_leap);

            assert!(
                ((dy == dyold + 1)
                    || (dy == 1 && dyold == 365 && !Date::is_leap(yold))
                    || (dy == 1 && dyold == 366 && Date::is_leap(yold))),
                "wrong day of year increment: \n date: {:?} \n day of year: {} \n previous: {}",
                t,
                dy,
                dyold
            );

            dyold = dy;

            let m_ordinal: Integer = m.into();
            let mold_ordinal: Integer = mold.into();
            assert!(
                (d == dold + 1 && m == mold && y == yold)
                    || (d == 1 && m_ordinal == (mold_ordinal + 1) && y == yold)
                    || (d == 1 && m_ordinal == 1 && y == yold + 1)
            );

            dold = d;
            mold = m;
            yold = y;

            // check month definition
            let m_ordinal: Integer = m.into();
            assert!(
                m_ordinal >= 1 || m_ordinal <= 12,
                "invalid month, date: {:?}, month: {:?}",
                t,
                m
            );

            // check day definition
            assert!(d >= 1, "invalid day of month, date: {:?}, day: {}", t, d);
            assert!(
                (m == Month::January && d <= 31)
                    || (m == Month::February && d <= 28)
                    || (m == Month::February && d == 29 && Date::is_leap(y))
                    || (m == Month::March && d <= 31)
                    || (m == Month::April && d <= 30)
                    || (m == Month::May && d <= 31)
                    || (m == Month::June && d <= 30)
                    || (m == Month::July && d <= 31)
                    || (m == Month::August && d <= 31)
                    || (m == Month::September && d <= 30)
                    || (m == Month::October && d <= 31)
                    || (m == Month::November && d <= 30)
                    || (m == Month::December && d <= 31),
                "invalid day of month, date: {:?}, day: {}",
                t,
                d
            );

            // check weekday definition
            let wd_ordinal: Integer = wd.into();
            let wold_ordinal: Integer = wold.into();
            assert!(
                (wd_ordinal == wold_ordinal + 1) || (wd_ordinal == 1 && wold_ordinal == 7),
                "invalid weekday, date: {:?}, weekday: {:?}, previous: {:?}",
                t,
                wd,
                wold
            );

            wold = wd;

            // create the same date with a different constructor
            let s = Date::new(d, m, y);
            // check serial number consistency
            serial = s.serial_number();
            assert!(
                serial == i,
                "inconsistent serial number:\n date: {:?}\n serial number: {}\
                 \n cloned date: {:?}\n serial number: {}",
                t,
                i,
                s,
                serial
            );
        }
    }
}
