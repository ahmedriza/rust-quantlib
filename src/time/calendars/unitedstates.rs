use std::sync::Arc;

use crate::time::{
    calendar::{easter_monday, Calendar, Holiday, Weekend, WesternWeekend},
    date::Date,
    months::Month::{self, *},
    weekday::Weekday::{self, *},
    Day, Year,
};

/// United States calendars
#[derive(Clone)]
pub struct UnitedStates {}

impl UnitedStates {
    pub fn settlement() -> Calendar {
        UnitedStatesSettlement::new()
    }

    pub fn libor_impact() -> Calendar {
        UnitedStatesLiborImpact::new()
    }

    pub fn nyse() -> Calendar {
        UnitedStatesNyse::new()
    }

    pub fn government_bond() -> Calendar {
        UnitedStatesGovernmentBond::new()
    }

    pub fn nerc() -> Calendar {
        UnitedStatesNerc::new()
    }

    pub fn federal_reserve() -> Calendar {
        UnitedStatesFederalReserve::new()
    }
}

// -------------------------------------------------------------------------------------------------

fn is_washington_birthday(d: Day, m: Month, y: Year, w: Weekday) -> bool {
    if y >= 1971 {
        // third Monday in February
        (15..=21).contains(&d) && w == Monday && m == February
    } else {
        // February 22nd, possily adjusted
        (d == 22 || (d == 23 && w == Monday) || (d == 21 && w == Friday)) && m == February
    }
}

fn is_memorial_day(d: Day, m: Month, y: Year, w: Weekday) -> bool {
    if y >= 1971 {
        // last Monday in May
        d >= 25 && w == Monday && m == May
    } else {
        // May 30th, possibly adjusted
        (d == 30 || (d == 31 && w == Monday) || (d == 29 && w == Friday)) && m == May
    }
}

fn is_juneteenth(d: Day, m: Month, y: Year, w: Weekday) -> bool {
    // declared in 2021, but only observed by exchanges since 2022
    (d == 19 || (d == 20 && w == Monday) || (d == 18 && w == Friday)) && m == June && y >= 2022
}

fn is_labor_day(d: Day, m: Month, _y: Year, w: Weekday) -> bool {
    // first Monday in September
    d <= 7 && w == Monday && m == September
}

fn is_columbus_day(d: Day, m: Month, y: Year, w: Weekday) -> bool {
    // second Monday in October
    (8..=14).contains(&d) && w == Monday && m == October && y >= 1971
}

fn is_veterans_day(d: Day, m: Month, y: Year, w: Weekday) -> bool {
    if y <= 1970 || y >= 1978 {
        // November 11th, adjusted
        (d == 11 || (d == 12 && w == Monday) || (d == 10 && w == Friday)) && m == November
    } else {
        // fourth Monday in October
        (22..=28).contains(&d) && w == Monday && m == October
    }
}

fn is_veterans_day_no_saturday(d: Day, m: Month, y: Year, w: Weekday) -> bool {
    if y <= 1970 || y >= 1978 {
        // November 11th, adjusted, but no Saturday to Friday
        (d == 11 || (d == 12 && w == Monday)) && m == November
    } else {
        // fourth Monday in October
        (22..=28).contains(&d) && w == Monday && m == October
    }
}

fn is_settlement_business_day(date: &Date) -> bool {
    let w = date.weekday();
    let d = date.day_of_month();
    let m = date.month();
    let y = date.year();

    // New Year's Day (possibly moved to Monday if on Sunday)
    if ((d == 1 || (d == 2 && w == Monday)) && m == January)
            // (or to Friday if on Saturday)
            || (d == 31 && w == Friday && m == December)
            // Martin Luther King's birthday (third Monday in January)
            || ((15..=21).contains(&d) && w == Monday && m == January && y >= 1983)
            // Washington's birthday (third Monday in February)
            || is_washington_birthday(d, m, y, w)
            // Memorial Day (last Monday in May)
            || is_memorial_day(d, m, y, w)
            // Juneteenth (Monday if Sunday or Friday if Saturday)
            || is_juneteenth(d, m, y, w)
            // Independence Day (Monday if Sunday or Friday if Saturday)
            || ((d == 4 || (d == 5 && w == Monday) ||
                 (d == 3 && w == Friday)) && m == July)
            // Labor Day (first Monday in September)
            || is_labor_day(d, m, y, w)
            // Columbus Day (second Monday in October)
            || is_columbus_day(d, m, y, w)
            // Veteran's Day (Monday if Sunday or Friday if Saturday)
            || is_veterans_day(d, m, y, w)
            // Thanksgiving Day (fourth Thursday in November)
            || ((22..=28).contains(&d) && w == Thursday && m == November)
            // Christmas (Monday if Sunday or Friday if Saturday)
            || ((d == 25 || (d == 26 && w == Monday) ||
                 (d == 24 && w == Friday)) && m == December)
    {
        return false;
    }

    true
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct UnitedStatesSettlement {
    pub weekend: Arc<dyn Weekend>,
}

impl UnitedStatesSettlement {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> Calendar {
        Calendar::new(Arc::new(UnitedStatesSettlement {
            weekend: Arc::new(WesternWeekend {}),
        }))
    }
}

impl Holiday for UnitedStatesSettlement {
    fn name(&self) -> String {
        "US settlement".into()
    }

    fn is_business_day(&self, date: &Date) -> bool {
        let w = date.weekday();
        self.is_weekend(w) || is_settlement_business_day(date)
    }

    fn is_weekend(&self, weekday: Weekday) -> bool {
        self.weekend.is_weekend(weekday)
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct UnitedStatesLiborImpact {
    pub weekend: Arc<dyn Weekend>,
}

impl UnitedStatesLiborImpact {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> Calendar {
        Calendar::new(Arc::new(UnitedStatesLiborImpact {
            weekend: Arc::new(WesternWeekend {}),
        }))
    }
}

impl Holiday for UnitedStatesLiborImpact {
    fn name(&self) -> String {
        "US with Libor impact".into()
    }

    fn is_business_day(&self, date: &Date) -> bool {
        // Since 2015 Independence Day only impacts Libor if it falls on a weekday
        let w = date.weekday();
        let d = date.day_of_month();
        let m = date.month();
        let y = date.year();

        if ((d == 5 && w == Monday) || (d == 3 && w == Friday)) && m == July && y >= 2015 {
            return true;
        }

        self.is_weekend(w) || is_settlement_business_day(date)
    }

    fn is_weekend(&self, weekday: Weekday) -> bool {
        self.weekend.is_weekend(weekday)
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct UnitedStatesNyse {
    pub weekend: Arc<dyn Weekend>,
}

impl UnitedStatesNyse {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> Calendar {
        Calendar::new(Arc::new(UnitedStatesNyse {
            weekend: Arc::new(WesternWeekend {}),
        }))
    }
}

impl Holiday for UnitedStatesNyse {
    fn name(&self) -> String {
        "New York stock exchange".into()
    }

    fn is_business_day(&self, date: &Date) -> bool {
        let w = date.weekday();
        let d = date.day_of_month();
        let dd = date.day_of_year();
        let m = date.month();
        let y = date.year();
        let em = easter_monday(y);

        if self.is_weekend(w)
            // New Year's Day (possibly moved to Monday if on Sunday)
            || ((d == 1 || (d == 2 && w == Monday)) && m == January)
            // Washington's birthday (third Monday in February)
            || is_washington_birthday(d, m, y, w)
            // Good Friday
            || (dd == em-3)
            // Memorial Day (last Monday in May)
            || is_memorial_day(d, m, y, w)
            // Juneteenth (Monday if Sunday or Friday if Saturday)
            || is_juneteenth(d, m, y, w)
            // Independence Day (Monday if Sunday or Friday if Saturday)
            || ((d == 4 || (d == 5 && w == Monday) ||
                 (d == 3 && w == Friday)) && m == July)
            // Labor Day (first Monday in September)
            || is_labor_day(d, m, y, w)
            // Thanksgiving Day (fourth Thursday in November)
            || ((22..=28).contains(&d) && w == Thursday && m == November)
            // Christmas (Monday if Sunday or Friday if Saturday)
            || ((d == 25 || (d == 26 && w == Monday) ||
                 (d == 24 && w == Friday)) && m == December)
        {
            return false;
        }

        if y >= 1998 && (15..=21).contains(&d) && w == Monday && m == January {
            // Martin Luther King's birthday (third Monday in January)
            return false;
        }

        if (y <= 1968 || (y <= 1980 && y % 4 == 0)) && m == November && d <= 7 && w == Tuesday {
            // Presidential election days
            return false;
        }

        // Special closings
        if
        // President Bush's Funeral
        (y == 2018 && m == December && d == 5)
            // Hurricane Sandy
            || (y == 2012 && m == October && (d == 29 || d == 30))
            // President Ford's funeral
            || (y == 2007 && m == January && d == 2)
            // President Reagan's funeral
            || (y == 2004 && m == June && d == 11)
            // September 11-14, 2001
            || (y == 2001 && m == September && (11..=14).contains(&d))
            // President Nixon's funeral
            || (y == 1994 && m == April && d == 27)
            // Hurricane Gloria
            || (y == 1985 && m == September && d == 27)
            // 1977 Blackout
            || (y == 1977 && m == July && d == 14)
            // Funeral of former President Lyndon B. Johnson.
            || (y == 1973 && m == January && d == 25)
            // Funeral of former President Harry S. Truman
            || (y == 1972 && m == December && d == 28)
            // National Day of Participation for the lunar exploration.
            || (y == 1969 && m == July && d == 21)
            // Funeral of former President Eisenhower.
            || (y == 1969 && m == March && d == 31)
            // Closed all day - heavy snow.
            || (y == 1969 && m == February && d == 10)
            // Day after Independence Day.
            || (y == 1968 && m == July && d == 5)
            // June 12-Dec. 31, 1968
            // Four day week (closed on Wednesdays) - Paperwork Crisis
            || (y == 1968 && dd >= 163 && w == Wednesday)
            // Day of mourning for Martin Luther King Jr.
            || (y == 1968 && m == April && d == 9)
            // Funeral of President Kennedy
            || (y == 1963 && m == November && d == 25)
            // Day before Decoration Day
            || (y == 1961 && m == May && d == 29)
            // Day after Christmas
            || (y == 1958 && m == December && d == 26)
            // Christmas Eve
            || ((y == 1954 || y == 1956 || y == 1965)
                && m == December && d == 24)
        {
            return false;
        }

        true
    }

    fn is_weekend(&self, weekday: Weekday) -> bool {
        self.weekend.is_weekend(weekday)
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct UnitedStatesGovernmentBond {
    pub weekend: Arc<dyn Weekend>,
}

impl UnitedStatesGovernmentBond {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> Calendar {
        Calendar::new(Arc::new(UnitedStatesGovernmentBond {
            weekend: Arc::new(WesternWeekend {}),
        }))
    }
}

impl Holiday for UnitedStatesGovernmentBond {
    fn name(&self) -> String {
        "US government bond market".into()
    }

    fn is_business_day(&self, date: &Date) -> bool {
        let w = date.weekday();
        let d = date.day_of_month();
        let dd = date.day_of_year();
        let m = date.month();
        let y = date.year();
        let em = easter_monday(y);

        if self.is_weekend(w)
            // New Year's Day (possibly moved to Monday if on Sunday)
            || ((d == 1 || (d == 2 && w == Monday)) && m == January)
            // Martin Luther King's birthday (third Monday in January)
            || ((15..=21).contains(&d) && w == Monday && m == January
                && y >= 1983)
            // Washington's birthday (third Monday in February)
            || is_washington_birthday(d, m, y, w)
            // Good Friday (2015 was half day due to NFP report)
            || (dd == em-3 && y != 2015)
            // Memorial Day (last Monday in May)
            || is_memorial_day(d, m, y, w)
            // Juneteenth (Monday if Sunday or Friday if Saturday)
            || is_juneteenth(d, m, y, w)
            // Independence Day (Monday if Sunday or Friday if Saturday)
            || ((d == 4 || (d == 5 && w == Monday) ||
                 (d == 3 && w == Friday)) && m == July)
            // Labor Day (first Monday in September)
            || is_labor_day(d, m, y, w)
            // Columbus Day (second Monday in October)
            || is_columbus_day(d, m, y, w)
            // Veteran's Day (Monday if Sunday)
            || is_veterans_day_no_saturday(d, m, y, w)
            // Thanksgiving Day (fourth Thursday in November)
            || ((22..=28).contains(&d) && w == Thursday && m == November)
            // Christmas (Monday if Sunday or Friday if Saturday)
            || ((d == 25 || (d == 26 && w == Monday) ||
                 (d == 24 && w == Friday)) && m == December)
        {
            return false;
        }

        // Special closings
        if
        // President Bush's Funeral
        (y == 2018 && m == December && d == 5)
            // Hurricane Sandy
            || (y == 2012 && m == October && (d == 30))
            // President Reagan's funeral
            || (y == 2004 && m == June && d == 11)
        {
            return false;
        }

        true
    }

    fn is_weekend(&self, weekday: Weekday) -> bool {
        self.weekend.is_weekend(weekday)
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct UnitedStatesNerc {
    pub weekend: Arc<dyn Weekend>,
}

impl UnitedStatesNerc {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> Calendar {
        Calendar::new(Arc::new(UnitedStatesNerc {
            weekend: Arc::new(WesternWeekend {}),
        }))
    }
}

impl Holiday for UnitedStatesNerc {
    fn name(&self) -> String {
        "North American Energy Reliability Council".into()
    }

    fn is_business_day(&self, date: &Date) -> bool {
        let w = date.weekday();
        let d = date.day_of_month();
        let m = date.month();
        let y = date.year();

        if self.is_weekend(w)
            // New Year's Day (possibly moved to Monday if on Sunday)
            || ((d == 1 || (d == 2 && w == Monday)) && m == January)
            // Memorial Day (last Monday in May)
            || is_memorial_day(d, m, y, w)
            // Independence Day (Monday if Sunday)
            || ((d == 4 || (d == 5 && w == Monday)) && m == July)
            // Labor Day (first Monday in September)
            || is_labor_day(d, m, y, w)
            // Thanksgiving Day (fourth Thursday in November)
            || ((22..=28).contains(&d) && w == Thursday && m == November)
            // Christmas (Monday if Sunday)
            || ((d == 25 || (d == 26 && w == Monday)) && m == December)
        {
            return false;
        }

        true
    }

    fn is_weekend(&self, weekday: Weekday) -> bool {
        self.weekend.is_weekend(weekday)
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct UnitedStatesFederalReserve {
    pub weekend: Arc<dyn Weekend>,
}

impl UnitedStatesFederalReserve {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> Calendar {
        Calendar::new(Arc::new(UnitedStatesFederalReserve {
            weekend: Arc::new(WesternWeekend {}),
        }))
    }
}

impl Holiday for UnitedStatesFederalReserve {
    fn name(&self) -> String {
        "Federal Reserve Bankwire System".into()
    }

    fn is_business_day(&self, date: &Date) -> bool {
        let w = date.weekday();
        let d = date.day_of_month();
        let m = date.month();
        let y = date.year();

        if self.is_weekend(w)
            // New Year's Day (possibly moved to Monday if on Sunday)
            || ((d == 1 || (d == 2 && w == Monday)) && m == January)
            // Martin Luther King's birthday (third Monday in January)
            || ((15..=21).contains(&d) && w == Monday && m == January
                && y >= 1983)
            // Washington's birthday (third Monday in February)
            || is_washington_birthday(d, m, y, w)
            // Memorial Day (last Monday in May)
            || is_memorial_day(d, m, y, w)
            // Juneteenth (Monday if Sunday or Friday if Saturday)
            || is_juneteenth(d, m, y, w)
            // Independence Day (Monday if Sunday)
            || ((d == 4 || (d == 5 && w == Monday)) && m == July)
            // Labor Day (first Monday in September)
            || is_labor_day(d, m, y, w)
            // Columbus Day (second Monday in October)
            || is_columbus_day(d, m, y, w)
            // Veteran's Day (Monday if Sunday)
            || is_veterans_day_no_saturday(d, m, y, w)
            // Thanksgiving Day (fourth Thursday in November)
            || ((22..=28).contains(&d) && w == Thursday && m == November)
            // Christmas (Monday if Sunday)
            || ((d == 25 || (d == 26 && w == Monday)) && m == December)
        {
            return false;
        }

        true
    }

    fn is_weekend(&self, weekday: Weekday) -> bool {
        self.weekend.is_weekend(weekday)
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::time::date::Date;
    use crate::time::months::Month::*;

    use super::UnitedStates;

    #[test]
    fn test_settlement() {
        let expected_hol = vec![
            Date::new(1, January, 2004),
            Date::new(19, January, 2004),
            Date::new(16, February, 2004),
            Date::new(31, May, 2004),
            Date::new(5, July, 2004),
            Date::new(6, September, 2004),
            Date::new(11, October, 2004),
            Date::new(11, November, 2004),
            Date::new(25, November, 2004),
            Date::new(24, December, 2004),
            Date::new(31, December, 2004),
            //
            Date::new(17, January, 2005),
            Date::new(21, February, 2005),
            Date::new(30, May, 2005),
            Date::new(4, July, 2005),
            Date::new(5, September, 2005),
            Date::new(10, October, 2005),
            Date::new(11, November, 2005),
            Date::new(24, November, 2005),
            Date::new(26, December, 2005),
        ];

        let c = UnitedStates::settlement();

        let hol = c.holiday_list(
            Date::new(1, January, 2004),
            Date::new(31, December, 2005),
            false,
        );
        assert!(
            hol.len() == expected_hol.len(),
            "there were {} expected holidays, while there are {} calculated holidays",
            expected_hol.len(),
            hol.len()
        );

        for i in 0..hol.len() {
            assert!(
                hol[i] == expected_hol[i],
                "expected holiday was {:?} while calculated holiday is {:?}",
                expected_hol[i],
                hol[i]
            );
        }

        // before Uniform Monday Holiday Act
        let expected_hol = vec![
            Date::new(2, January, 1961),
            Date::new(22, February, 1961),
            Date::new(30, May, 1961),
            Date::new(4, July, 1961),
            Date::new(4, September, 1961),
            Date::new(10, November, 1961),
            Date::new(23, November, 1961),
            Date::new(25, December, 1961),
        ];
        let hol = c.holiday_list(
            Date::new(1, January, 1961),
            Date::new(31, December, 1961),
            false,
        );
        assert!(
            hol.len() == expected_hol.len(),
            "there were {} expected holidays, while there are {} calculated holidays",
            expected_hol.len(),
            hol.len()
        );

        for i in 0..hol.len() {
            assert!(
                hol[i] == expected_hol[i],
                "expected holiday was {:?} while calculated holiday is {:?}",
                expected_hol[i],
                hol[i]
            );
        }
    }

    #[test]
    fn test_government_bond() {
        let expected_hol = vec![
            Date::new(1, January, 2004),
            Date::new(19, January, 2004),
            Date::new(16, February, 2004),
            Date::new(9, April, 2004),
            Date::new(31, May, 2004),
            Date::new(11, June, 2004), // Reagan's funeral
            Date::new(5, July, 2004),
            Date::new(6, September, 2004),
            Date::new(11, October, 2004),
            Date::new(11, November, 2004),
            Date::new(25, November, 2004),
            Date::new(24, December, 2004),
        ];

        let c = UnitedStates::government_bond();

        let hol = c.holiday_list(
            Date::new(1, January, 2004),
            Date::new(31, December, 2004),
            false,
        );

        assert!(
            hol.len() == expected_hol.len(),
            "there were {} expected holidays, while there are {} calculated holidays",
            expected_hol.len(),
            hol.len()
        );

        for i in 0..hol.len() {
            assert!(
                hol[i] == expected_hol[i],
                "expected holiday was {:?} while calculated holiday is {:?}",
                expected_hol[i],
                hol[i]
            );
        }
    }

    #[test]
    fn test_new_york_stock_exchange() {
        let expected_hol = vec![
            Date::new(1, January, 2004),
            Date::new(19, January, 2004),
            Date::new(16, February, 2004),
            Date::new(9, April, 2004),
            Date::new(31, May, 2004),
            Date::new(11, June, 2004),
            Date::new(5, July, 2004),
            Date::new(6, September, 2004),
            Date::new(25, November, 2004),
            Date::new(24, December, 2004),
            //
            Date::new(17, January, 2005),
            Date::new(21, February, 2005),
            Date::new(25, March, 2005),
            Date::new(30, May, 2005),
            Date::new(4, July, 2005),
            Date::new(5, September, 2005),
            Date::new(24, November, 2005),
            Date::new(26, December, 2005),
            //
            Date::new(2, January, 2006),
            Date::new(16, January, 2006),
            Date::new(20, February, 2006),
            Date::new(14, April, 2006),
            Date::new(29, May, 2006),
            Date::new(4, July, 2006),
            Date::new(4, September, 2006),
            Date::new(23, November, 2006),
            Date::new(25, December, 2006),
        ];

        let c = UnitedStates::nyse();

        let hol = c.holiday_list(
            Date::new(1, January, 2004),
            Date::new(31, December, 2006),
            false,
        );

        assert!(
            hol.len() == expected_hol.len(),
            "there were {} expected holidays, while there are {} calculated holidays",
            expected_hol.len(),
            hol.len()
        );

        for i in 0..expected_hol.len() {
            assert!(
                hol[i] == expected_hol[i],
                "expected holiday was {:?} while calculated holiday is {:?}",
                expected_hol[i],
                hol[i]
            );
        }

        let hist_close = vec![
            Date::new(30, October, 2012),   // Hurricane Sandy
            Date::new(29, October, 2012),   // Hurricane Sandy
            Date::new(11, June, 2004),      // Reagan's funeral
            Date::new(14, September, 2001), // September 11, 2001
            Date::new(13, September, 2001), // September 11, 2001
            Date::new(12, September, 2001), // September 11, 2001
            Date::new(11, September, 2001), // September 11, 2001
            Date::new(27, April, 1994),     // Nixon's funeral
            Date::new(27, September, 1985), // Hurricane Gloria
            Date::new(14, July, 1977),      // 1977 Blackout
            Date::new(25, January, 1973),   // Johnson's funeral
            Date::new(28, December, 1972),  // Truman's funeral
            Date::new(21, July, 1969),      // Lunar exploration nat. day
            Date::new(31, March, 1969),     // Eisenhower's funeral
            Date::new(10, February, 1969),  // heavy snow
            Date::new(5, July, 1968),       // Day after Independence Day
            Date::new(9, April, 1968),      // Mourning for MLK
            Date::new(24, December, 1965),  // Christmas Eve
            Date::new(25, November, 1963),  // Kennedy's funeral
            Date::new(29, May, 1961),       // Day before Decoration Day
            Date::new(26, December, 1958),  // Day after Christmas
            Date::new(24, December, 1956),  // Christmas Eve
            Date::new(24, December, 1954),  // Christmas Eve
            // June 12-Dec. 31, 1968
            // Four day week (closed on Wednesdays) - Paperwork Crisis
            Date::new(12, June, 1968),
            Date::new(19, June, 1968),
            Date::new(26, June, 1968),
            Date::new(3, July, 1968),
            Date::new(10, July, 1968),
            Date::new(17, July, 1968),
            Date::new(20, November, 1968),
            Date::new(27, November, 1968),
            Date::new(4, December, 1968),
            Date::new(11, December, 1968),
            Date::new(18, December, 1968),
            // Presidential election days
            Date::new(4, November, 1980),
            Date::new(2, November, 1976),
            Date::new(7, November, 1972),
            Date::new(5, November, 1968),
            Date::new(3, November, 1964),
        ];

        for d in hist_close {
            assert!(
                c.is_holiday(&d),
                "{:?} should be holiday (historical close)",
                d
            );
        }
    }
}
