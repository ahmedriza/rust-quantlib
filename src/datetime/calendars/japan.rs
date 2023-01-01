use std::sync::Arc;

use crate::datetime::date::Date;
use crate::types::Time;

use crate::datetime::{
    calendar::{Calendar, Holiday, Weekend, WesternWeekend},
    months::Month::*,
    weekday::Weekday::{self, *},
    Day,
};

#[derive(Clone)]
pub struct Japan {
    pub weekend: Arc<dyn Weekend>,
}

impl Japan {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Calendar {
        Calendar::new(Arc::new(Japan {
            weekend: Arc::new(WesternWeekend {}),
        }))
    }
}

impl Holiday for Japan {
    fn name(&self) -> String {
        "Japan".into()
    }

    fn is_business_day(&self, date: &Date) -> bool {
        let w = date.weekday();
        let d = date.day_of_month();
        let m = date.month();
        let y = date.year();

        // equinox calculation
        let moving_amount = (y - 2000) as Time * DIFF_PER_YEAR;
        let number_of_leap_years = (y - 2000) / 4 + (y - 2000) / 100 - (y - 2000) / 400;
        let ve = // vernal equinox day
            (EXACT_VERNAL_EQUINOX_TIME + moving_amount - number_of_leap_years as Time) as Day;
        let ae = // autumnal equinox day
            (EXACT_AUTUMNAL_EQUINOX_TIME + moving_amount - number_of_leap_years as Time) as Day;
        // checks

        if self.is_weekend(w)
            // New Year's Day
            || (d == 1  && m == January)
            // Bank Holiday
            || (d == 2  && m == January)
            // Bank Holiday
            || (d == 3  && m == January)
            // Coming of Age Day (2nd Monday in January),
            // was January 15th until 2000
            || (w == Monday && (8..=14).contains(&d) && m == January
                && y >= 2000)
            || ((d == 15 || (d == 16 && w == Monday)) && m == January
                && y < 2000)
            // National Foundation Day
            || ((d == 11 || (d == 12 && w == Monday)) && m == February)
            // Emperor's Birthday (Emperor Naruhito)
            || ((d == 23 || (d == 24 && w == Monday)) && m == February
                && y >= 2020)
            // Emperor's Birthday (Emperor Akihito)
            || ((d == 23 || (d == 24 && w == Monday)) && m == December
                && (1989..2019).contains(&y))
            // Vernal Equinox
            || ((d == ve || (d == ve+1 && w == Monday)) && m == March)
            // Greenery Day
            || ((d == 29 || (d == 30 && w == Monday)) && m == April)
            // Constitution Memorial Day
            || (d == 3  && m == May)
            // Holiday for a Nation
            || (d == 4  && m == May)
            // Children's Day
            || (d == 5  && m == May)
            // any of the three above observed later if on Saturday or Sunday
            || (d == 6 && m == May
                && (w == Monday || w == Tuesday || w == Wednesday))
            // Marine Day (3rd Monday in July),
            // was July 20th until 2003, not a holiday before 1996,
            // July 23rd in 2020 due to Olympics games
            // July 22nd in 2021 due to Olympics games
            || (w == Monday && (15..=21).contains(&d) && m == July
                && ((2003..2020).contains(&y) || y >= 2022))
            || ((d == 20 || (d == 21 && w == Monday)) && m == July
                && (1996..2003).contains(&y))
            || (d == 23 && m == July && y == 2020)
            || (d == 22 && m == July && y == 2021)
            // Mountain Day
            // (moved in 2020 due to Olympics games)
            // (moved in 2021 due to Olympics games)
            || ((d == 11 || (d == 12 && w == Monday)) && m == August
                && ((2016..2020).contains(&y) || y >= 2022))
            || (d == 10 && m == August && y == 2020)
            || (d == 9 && m == August && y == 2021)
            // Respect for the Aged Day (3rd Monday in September),
            // was September 15th until 2003
            || (w == Monday && (15..=21).contains(&d) && m == September
                && y >= 2003)
            || ((d == 15 || (d == 16 && w == Monday)) && m == September
                && y < 2003)
            // If a single day falls between Respect for the Aged Day
            // and the Autumnal Equinox, it is holiday
            || (w == Tuesday && d+1 == ae && (16..=22).contains(&d)
                && m == September && y >= 2003)
            // Autumnal Equinox
            || ((d == ae || (d == ae+1 && w == Monday)) && m == September)
            // Health and Sports Day (2nd Monday in October),
            // was October 10th until 2000,
            // July 24th in 2020 due to Olympics games
            // July 23rd in 2021 due to Olympics games
            || (w == Monday && (8..=14).contains(&d) && m == October
                && ((2000..2020).contains(&y) || y >= 2022))
            || ((d == 10 || (d == 11 && w == Monday)) && m == October
                && y < 2000)
            || (d == 24 && m == July && y == 2020)
            || (d == 23 && m == July && y == 2021)
            // National Culture Day
            || ((d == 3  || (d == 4 && w == Monday)) && m == November)
            // Labor Thanksgiving Day
            || ((d == 23 || (d == 24 && w == Monday)) && m == November)
            // Bank Holiday
            || (d == 31 && m == December)
            // one-shot holidays
            // Marriage of Prince Akihito
            || (d == 10 && m == April && y == 1959)
            // Rites of Imperial Funeral
            || (d == 24 && m == February && y == 1989)
            // Enthronement Ceremony (Emperor Akihito)
            || (d == 12 && m == November && y == 1990)
            // Marriage of Prince Naruhito
            || (d == 9 && m == June && y == 1993)
            // Special holiday based on Japanese public holidays law
            || (d == 30 && m == April && y == 2019)
            // Enthronement Day (Emperor Naruhito)
            || (d == 1 && m == May && y == 2019)
            // Special holiday based on Japanese public holidays law
            || (d == 2 && m == May && y == 2019)
            // Enthronement Ceremony (Emperor Naruhito)
            || (d == 22 && m == October && y == 2019)
        {
            return false;
        }

        true
    }

    fn is_weekend(&self, weekday: Weekday) -> bool {
        self.weekend.is_weekend(weekday)
    }
}

const EXACT_VERNAL_EQUINOX_TIME: Time = 20.69115;
const EXACT_AUTUMNAL_EQUINOX_TIME: Time = 23.09;
const DIFF_PER_YEAR: Time = 0.242194;
