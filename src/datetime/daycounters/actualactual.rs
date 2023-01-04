use crate::{
    datetime::{
        date::Date, months::Month::*, period::Period, schedule::Schedule, timeunit::TimeUnit::*,
    },
    types::{Integer, Real, Time},
};

/// Actual/Actual day count convention
///
/// The day count can be calculated according to:
/// * the ISDA convention, also known as "Actual/Actual (Historical)",  "Actual/Actual",
///   "Act/Act", and according to ISDA also "Actual/365", "Act/365", and "A/365";
/// * the ISMA and US Treasury convention, also known as "Actual/Actual (Bond)";
/// * the AFB convention, also known as "Actual/Actual (Euro)".
///
/// For more details, refer to
/// <https://www.isda.org/a/pIJEE/The-Actual-Actual-Day-Count-Fraction-1999.pdf>
#[derive(Clone)]
pub struct ActualActual {
    pub convention: ActualActualConvention,
}

#[derive(Clone)]
pub enum ActualActualConvention {
    ISMA(Box<ISMA>),
    OldISMA(OldISMA),
    ISDA(ISDA),
    AFB(AFB),
}

impl ActualActual {
    /// Return the name of the day counter
    pub fn name(&self) -> String {
        match &self.convention {
            ActualActualConvention::ISMA(c) => c.name(),
            ActualActualConvention::OldISMA(c) => c.name(),
            ActualActualConvention::ISDA(c) => c.name(),
            ActualActualConvention::AFB(c) => c.name(),
        }
    }

    /// Returns the number of days between two dates.    
    pub fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        match &self.convention {
            ActualActualConvention::ISMA(c) => c.day_count(d1, d2),
            ActualActualConvention::OldISMA(c) => c.day_count(d1, d2),
            ActualActualConvention::ISDA(c) => c.day_count(d1, d2),
            ActualActualConvention::AFB(c) => c.day_count(d1, d2),
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
            ActualActualConvention::ISMA(c) => {
                c.year_fraction(d1, d2, ref_period_start, ref_period_end)
            }
            ActualActualConvention::OldISMA(c) => {
                c.year_fraction(d1, d2, ref_period_start, ref_period_end)
            }
            ActualActualConvention::ISDA(c) => {
                c.year_fraction(d1, d2, ref_period_start, ref_period_end)
            }
            ActualActualConvention::AFB(c) => {
                c.year_fraction(d1, d2, ref_period_start, ref_period_end)
            }
        }
    }

    /// Create an instance of [ActualActualIsma] day counter
    pub fn actual_actual_isma(schedule: Schedule) -> ISMA {
        ISMA { schedule }
    }

    /// Create an instance of [ActualActualOldIsma] day counter
    pub fn actual_actual_old_isma() -> OldISMA {
        OldISMA {}
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct ISMA {
    pub schedule: Schedule,
}

impl ISMA {
    pub fn name(&self) -> String {
        "Actual/Actual (ISMA)".into()
    }

    pub fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        d2 - d1
    }

    #[allow(clippy::comparison_chain)]
    pub fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        ref_period_start: &Date,
        ref_period_end: &Date,
    ) -> Time {
        if d1 == d2 {
            return 0.0;
        } else if d2 < d1 {
            return -self.year_fraction(d2, d1, ref_period_start, ref_period_end);
        }

        let coupon_dates = self.get_list_of_period_dates_including_quasi_payments();

        let first_date = coupon_dates
            .iter()
            .min()
            .expect("Failed to get mininum date from schedule coupon dates");
        let last_date = coupon_dates
            .iter()
            .max()
            .expect("Failed to get maximum date from schedule coupon dates");

        assert!(
            d1 >= first_date && d2 <= last_date,
            "Dates out of range of schedule: date 1: {:?}, date 2: {:?}, \
                 first date: {:?}, last date: {:?}",
            d1,
            d2,
            first_date,
            last_date
        );

        let mut year_fraction_sum = 0.0;
        for i in 0..coupon_dates.len() - 1 {
            let start_reference_period = coupon_dates[i];
            let end_reference_period = coupon_dates[i + 1];
            if d1 < &end_reference_period && d2 > &start_reference_period {
                year_fraction_sum += self.year_fraction_with_reference_dates(
                    d1.max(&start_reference_period),
                    d2.min(&end_reference_period),
                    &start_reference_period,
                    &end_reference_period,
                );
            }
        }
        year_fraction_sum
    }

    // -------------------------------------------------------------------------------------------------

    fn get_list_of_period_dates_including_quasi_payments(&self) -> Vec<Date> {
        // Process the schedule into an array of dates.
        let issue_date = self.schedule[0];
        let mut new_dates = self.schedule.dates();

        if !self.schedule.has_is_regular() || !self.schedule.is_regular(1) {
            let first_coupon = self.schedule[1];
            let notional_first_coupon = self.schedule.calendar().advance_by_period(
                first_coupon,
                -self.schedule.tenor(),
                self.schedule.business_day_convention(),
                self.schedule.end_of_month(),
            );

            new_dates[0] = notional_first_coupon;
            // long first coupon
            if notional_first_coupon > issue_date {
                let prior_notional_coupon = self.schedule.calendar().advance_by_period(
                    notional_first_coupon,
                    -self.schedule.tenor(),
                    self.schedule.business_day_convention(),
                    self.schedule.end_of_month(),
                );
                // insert as the first element
                new_dates.insert(0, prior_notional_coupon);
            }
        }

        if !self.schedule.has_is_regular() || !self.schedule.is_regular(self.schedule.size() - 1) {
            let notional_last_coupon = self.schedule.calendar().advance_by_period(
                self.schedule[self.schedule.size() - 2],
                self.schedule.tenor(),
                self.schedule.business_day_convention(),
                self.schedule.end_of_month(),
            );
            new_dates[self.schedule.size() - 1] = notional_last_coupon;
            if notional_last_coupon < *self.schedule.end_date() {
                let next_notional_coupon = self.schedule.calendar().advance_by_period(
                    notional_last_coupon,
                    self.schedule.tenor(),
                    self.schedule.business_day_convention(),
                    self.schedule.end_of_month(),
                );
                new_dates.push(next_notional_coupon);
            }
        }

        new_dates
    }

    fn year_fraction_with_reference_dates(
        &self,
        d1: &Date,
        d2: &Date,
        d3: &Date,
        d4: &Date,
    ) -> Time {
        assert!(
            d1 <= d2,
            "This function is only correct if d1 <= d2, d1: {:?}, d2: {:?}",
            d1,
            d2
        );
        let mut reference_day_count = self.day_count(d3, d4);
        // guess how many coupon periods per year
        let coupons_per_year;
        if reference_day_count < 16 {
            coupons_per_year = 1;
            reference_day_count = self.day_count(d1, &(d1 + Period::new(1, Years)));
        } else {
            coupons_per_year = self.find_coupons_per_year(d3, d4);
        }

        self.day_count(d1, d2) as f64 / (reference_day_count * coupons_per_year) as f64
    }

    fn find_coupons_per_year(&self, ref_start: &Date, ref_end: &Date) -> Integer {
        // This will only work for day counts longer than 15 days.
        let day_count = self.day_count(ref_start, ref_end);
        let years = day_count as Real / 365.0;
        let months = (12.0 * years).round() as Integer;
        (12.0 / months as f64).round() as Integer
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct OldISMA {}

impl OldISMA {
    pub fn name(&self) -> String {
        "Actual/Actual (ISMA)".into()
    }

    pub fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        d2 - d1
    }

    pub fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        ref_period_start: &Date,
        ref_period_end: &Date,
    ) -> Time {
        if d1 == d2 {
            return 0.0;
        }
        if d1 > d2 {
            return -self.year_fraction(d2, d1, ref_period_start, ref_period_end);
        }

        // when the reference period is not specified, try taking it equal to (d1,d2)
        let mut ref_period_start = if ref_period_start != &Date::default() {
            *ref_period_start
        } else {
            *d1
        };
        let mut ref_period_end = if ref_period_end != &Date::default() {
            *ref_period_end
        } else {
            *d2
        };

        assert!(
            ref_period_end > ref_period_start && ref_period_end > *d1,
            "Invalid reference period, date 1: {:?}, date 2: {:?}, \
                 reference priod start: {:?}, reference period end: {:?}",
            d1,
            d2,
            ref_period_start,
            ref_period_end
        );

        // estimate roughly the length in months of a period
        let mut months =
            (12.0 * (ref_period_end - ref_period_start) as Real / 365.0).round() as Integer;

        // for short periods
        if months == 0 {
            // take the reference period as 1 year from d1
            ref_period_start = *d1;
            ref_period_end = d1 + Period::new(1, Years);
            months = 12;
        }

        let period = months as Real / 12.0;

        if d2 <= &ref_period_end {
            // here refPeriodEnd is a future (notional?) payment date
            if d1 >= &ref_period_start {
                // here refPeriodStart is the last (maybe notional) payment date.
                // refPeriodStart <= d1 <= d2 <= refPeriodEnd
                // [maybe the equality should be enforced, since
                // refPeriodStart < d1 <= d2 < refPeriodEnd could give wrong results]
                period * Date::days_between(d1, d2)
                    / Date::days_between(&ref_period_start, &ref_period_end)
            } else {
                // here refPeriodStart is the next (maybe notional)
                // payment date and refPeriodEnd is the second next (maybe notional) payment date.
                // d1 < refPeriodStart < refPeriodEnd
                // AND d2 <= refPeriodEnd
                // this case is long first coupon
                //
                // the last notional payment date
                let previous_ref = ref_period_start - Period::new(months, Months);
                if d2 > &ref_period_start {
                    self.year_fraction(d1, &ref_period_start, &previous_ref, &ref_period_start)
                        + self.year_fraction(
                            &ref_period_start,
                            d2,
                            &ref_period_start,
                            &ref_period_end,
                        )
                } else {
                    self.year_fraction(d1, d2, &previous_ref, &ref_period_start)
                }
            }
        } else {
            // here refPeriodEnd is the last (notional?) payment date
            // d1 < refPeriodEnd < d2 AND refPeriodStart < refPeriodEnd
            assert!(
                &ref_period_start <= d1,
                "Invalid dates, d1 ({:?}) < ref_period_start ({:?}) < ref_period_end ({:?}) \
                     < d2 ({:?})",
                d1,
                ref_period_start,
                ref_period_end,
                d2
            );
            // the part from d1 to refPeriodEnd
            let mut sum =
                self.year_fraction(d1, &ref_period_end, &ref_period_start, &ref_period_end);
            // the part from refPeriodEnd to d2
            // count how many regular periods are in [refPeriodEnd, d2],
            // then add the remaining time
            let mut i = 0;
            let mut new_ref_start;
            let mut new_ref_end;
            loop {
                new_ref_start = ref_period_end + Period::new(months * i, Months);
                new_ref_end = ref_period_end + Period::new(months * (i + 1), Months);
                if d2 < &new_ref_end {
                    break;
                } else {
                    sum += period;
                    i += 1;
                }
            }
            sum += self.year_fraction(&new_ref_start, d2, &new_ref_start, &new_ref_end);
            sum
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct ISDA {}

impl ISDA {
    pub fn name(&self) -> String {
        "Actual/Actual (ISDA)".into()
    }

    pub fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        d2 - d1
    }

    pub fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        ref_period_start: &Date,
        ref_period_end: &Date,
    ) -> Time {
        if d1 == d2 {
            return 0.0;
        }
        if d1 > d2 {
            return -self.year_fraction(d2, d1, ref_period_start, ref_period_end);
        }

        let y1 = d1.year();
        let y2 = d2.year();

        let dib1 = if Date::is_leap(y1) { 366.0 } else { 365.0 };
        let dib2 = if Date::is_leap(y2) { 366.0 } else { 365.0 };

        let mut sum = (y2 - y1 - 1) as Time;
        sum += Date::days_between(d1, &Date::new(1, January, y1 + 1)) / dib1;
        sum += Date::days_between(&Date::new(1, January, y2), d2) / dib2;
        sum
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct AFB {}

impl AFB {
    pub fn name(&self) -> String {
        "Actual/Actual (AFB)".into()
    }

    pub fn day_count(&self, d1: &Date, d2: &Date) -> Integer {
        d2 - d1
    }

    pub fn year_fraction(
        &self,
        d1: &Date,
        d2: &Date,
        ref_period_start: &Date,
        ref_period_end: &Date,
    ) -> Time {
        if d1 == d2 {
            return 0.0;
        }
        if d1 > d2 {
            return -self.year_fraction(d2, d1, ref_period_start, ref_period_end);
        }

        let mut new_d2 = *d2;
        let mut temp = *d2;
        let mut sum = 0.0;
        while &temp > d1 {
            temp = new_d2 - Period::new(1, Years);
            if temp.day_of_month() == 28 && temp.month() == February && Date::is_leap(temp.year()) {
                temp += 1;
            }
            if &temp >= d1 {
                sum += 1.0;
                new_d2 = temp;
            }
        }

        let mut den = 365.0;
        if Date::is_leap(new_d2.year()) {
            temp = Date::new(29, February, new_d2.year());
            if new_d2 > temp && d1 <= &temp {
                den += 1.0;
            }
        } else if Date::is_leap(d1.year()) {
            temp = Date::new(29, February, d1.year());
            if new_d2 > temp && d1 <= &temp {
                den += 1.0;
            }
        }
        sum + Date::days_between(d1, &new_d2) / den
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::{
        context::pricing_context::PricingContext,
        datetime::{
            businessdayconvention::BusinessDayConvention::Unadjusted, date::Date,
            daycounter::DayCounter, frequency::Frequency, holidays::target::Target,
            months::Month::*, period::Period, schedulebuilder::ScheduleBuilder,
        },
        types::Real,
    };

    use super::ActualActual;

    #[test]
    fn test_actual_actual_isma() {
        let end_of_month = false;
        let frequency = Frequency::Semiannual;
        let interest_accrual_date = Date::new(30, January, 1999);
        let maturity_date = Date::new(30, June, 2000);
        let first_coupon_date = Date::new(30, July, 1999);
        let penultimate_coupon_date = Date::new(30, January, 2000);
        let d1 = Date::new(30, January, 2000);
        let d2 = Date::new(30, June, 2000);

        let expected = 152.0 / (182.0 * 2.0);

        do_test_actual_actual_isma(
            end_of_month,
            frequency,
            interest_accrual_date,
            maturity_date,
            first_coupon_date,
            penultimate_coupon_date,
            d1,
            d2,
            expected,
        );

        ///////////////////////////////////////////

        let end_of_month = true;
        let frequency = Frequency::Quarterly;
        let interest_accrual_date = Date::new(31, May, 1999);
        let maturity_date = Date::new(30, April, 2000);
        let first_coupon_date = Date::new(31, August, 1999);
        let penultimate_coupon_date = Date::new(30, November, 1999);
        let d1 = Date::new(30, November, 1999);
        let d2 = Date::new(30, April, 2000);

        let expected = 91.0 / (91.0 * 4.0) + 61.0 / (92.0 * 4.0);

        do_test_actual_actual_isma(
            end_of_month,
            frequency,
            interest_accrual_date,
            maturity_date,
            first_coupon_date,
            penultimate_coupon_date,
            d1,
            d2,
            expected,
        );

        ///////////////////////////////////////////

        let end_of_month = false;
        let frequency = Frequency::Quarterly;
        let interest_accrual_date = Date::new(31, May, 1999);
        let maturity_date = Date::new(30, April, 2000);
        let first_coupon_date = Date::new(31, August, 1999);
        let penultimate_coupon_date = Date::new(30, November, 1999);
        let d1 = Date::new(30, November, 1999);
        let d2 = Date::new(30, April, 2000);

        let expected = 91.0 / (91.0 * 4.0) + 61.0 / (90.0 * 4.0);

        do_test_actual_actual_isma(
            end_of_month,
            frequency,
            interest_accrual_date,
            maturity_date,
            first_coupon_date,
            penultimate_coupon_date,
            d1,
            d2,
            expected,
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn do_test_actual_actual_isma(
        end_of_month: bool,
        frequency: Frequency,
        interest_accrual_date: Date,
        maturity_date: Date,
        first_coupon_date: Date,
        penultimate_coupon_date: Date,
        d1: Date,
        d2: Date,
        expected: Real,
    ) {
        let schedule = ScheduleBuilder::new(
            pricing_context(interest_accrual_date),
            interest_accrual_date,
            maturity_date,
            Period::from(frequency),
            Target::new(),
        )
        .with_convention(Unadjusted) // ignore the calendar
        .with_first_date(first_coupon_date)
        .with_next_to_last_date(penultimate_coupon_date)
        .with_end_of_month(end_of_month)
        .build();

        let day_counter = ActualActual::actual_actual_isma(schedule);
        let calculated = day_counter.year_fraction(&d1, &d2, &Date::default(), &Date::default());

        assert!(
            (calculated - expected).abs() <= 1.0e-10,
            "period: {:?} to {:?}, first coupon date: {:?}, penultimate coupon date: {:?},\
             calculated: {}, expected: {}, diff: {}",
            d1,
            d2,
            first_coupon_date,
            penultimate_coupon_date,
            calculated,
            expected,
            (calculated - expected).abs(),
        );
    }

    #[test]
    fn test_actual_actual() {
        let test_cases = vec![
            // first example
            ActualActualTestCase::new(
                DayCounter::actual_actual_old_isma(),
                Date::new(1, November, 2003),
                Date::new(1, May, 2004),
                Date::new(1, November, 2003),
                Date::new(1, May, 2004),
                0.500000000000,
            ),
            ActualActualTestCase::new(
                DayCounter::actual_actual_isda(),
                Date::new(1, November, 2003),
                Date::new(1, May, 2004),
                Date::default(),
                Date::default(),
                0.497724380567,
            ),
            ActualActualTestCase::new(
                DayCounter::actual_actual_afb(),
                Date::new(1, November, 2003),
                Date::new(1, May, 2004),
                Date::default(),
                Date::default(),
                0.497267759563,
            ),
            // short first calculation period (first period)
            ActualActualTestCase::new(
                DayCounter::actual_actual_old_isma(),
                Date::new(1, February, 1999),
                Date::new(1, July, 1999),
                Date::new(1, July, 1998),
                Date::new(1, July, 1999),
                0.410958904110,
            ),
            ActualActualTestCase::new(
                DayCounter::actual_actual_isda(),
                Date::new(1, February, 1999),
                Date::new(1, July, 1999),
                Date::default(),
                Date::default(),
                0.410958904110,
            ),
            ActualActualTestCase::new(
                DayCounter::actual_actual_afb(),
                Date::new(1, February, 1999),
                Date::new(1, July, 1999),
                Date::default(),
                Date::default(),
                0.410958904110,
            ),
            // short first calculation period (second period)
            ActualActualTestCase::new(
                DayCounter::actual_actual_old_isma(),
                Date::new(1, July, 1999),
                Date::new(1, July, 2000),
                Date::new(1, July, 1999),
                Date::new(1, July, 2000),
                1.000000000000,
            ),
            ActualActualTestCase::new(
                DayCounter::actual_actual_isda(),
                Date::new(1, July, 1999),
                Date::new(1, July, 2000),
                Date::default(),
                Date::default(),
                1.001377348600,
            ),
            ActualActualTestCase::new(
                DayCounter::actual_actual_afb(),
                Date::new(1, July, 1999),
                Date::new(1, July, 2000),
                Date::default(),
                Date::default(),
                1.000000000000,
            ),
            // long first calculation period (first period)
            ActualActualTestCase::new(
                DayCounter::actual_actual_old_isma(),
                Date::new(15, August, 2002),
                Date::new(15, July, 2003),
                Date::new(15, January, 2003),
                Date::new(15, July, 2003),
                0.915760869565,
            ),
            ActualActualTestCase::new(
                DayCounter::actual_actual_isda(),
                Date::new(15, August, 2002),
                Date::new(15, July, 2003),
                Date::default(),
                Date::default(),
                0.915068493151,
            ),
            ActualActualTestCase::new(
                DayCounter::actual_actual_afb(),
                Date::new(15, August, 2002),
                Date::new(15, July, 2003),
                Date::default(),
                Date::default(),
                0.915068493151,
            ),
            // long first calculation period (second period)
            /* Warning: the ISDA case is in disagreement with mktc1198.pdf */
            ActualActualTestCase::new(
                DayCounter::actual_actual_old_isma(),
                Date::new(15, July, 2003),
                Date::new(15, January, 2004),
                Date::new(15, July, 2003),
                Date::new(15, January, 2004),
                0.500000000000,
            ),
            ActualActualTestCase::new(
                DayCounter::actual_actual_isda(),
                Date::new(15, July, 2003),
                Date::new(15, January, 2004),
                Date::default(),
                Date::default(),
                0.504004790778,
            ),
            ActualActualTestCase::new(
                DayCounter::actual_actual_afb(),
                Date::new(15, July, 2003),
                Date::new(15, January, 2004),
                Date::default(),
                Date::default(),
                0.504109589041,
            ),
            // short final calculation period (penultimate period)
            ActualActualTestCase::new(
                DayCounter::actual_actual_old_isma(),
                Date::new(30, July, 1999),
                Date::new(30, January, 2000),
                Date::new(30, July, 1999),
                Date::new(30, January, 2000),
                0.500000000000,
            ),
            ActualActualTestCase::new(
                DayCounter::actual_actual_isda(),
                Date::new(30, July, 1999),
                Date::new(30, January, 2000),
                Date::default(),
                Date::default(),
                0.503892506924,
            ),
            ActualActualTestCase::new(
                DayCounter::actual_actual_afb(),
                Date::new(30, July, 1999),
                Date::new(30, January, 2000),
                Date::default(),
                Date::default(),
                0.504109589041,
            ),
            // short final calculation period (final period)
            ActualActualTestCase::new(
                DayCounter::actual_actual_old_isma(),
                Date::new(30, January, 2000),
                Date::new(30, June, 2000),
                Date::new(30, January, 2000),
                Date::new(30, July, 2000),
                0.417582417582,
            ),
            ActualActualTestCase::new(
                DayCounter::actual_actual_isda(),
                Date::new(30, January, 2000),
                Date::new(30, June, 2000),
                Date::default(),
                Date::default(),
                0.415300546448,
            ),
            ActualActualTestCase::new(
                DayCounter::actual_actual_afb(),
                Date::new(30, January, 2000),
                Date::new(30, June, 2000),
                Date::default(),
                Date::default(),
                0.41530054644,
            ),
        ];

        do_test_actual_actual(test_cases);
    }

    fn do_test_actual_actual(test_cases: Vec<ActualActualTestCase>) {
        for test_case in test_cases {
            let calculated = test_case.daycounter.year_fraction(
                &test_case.d1,
                &test_case.d2,
                &test_case.ref_period_start,
                &test_case.ref_period_end,
            );
            assert!(
                (calculated - test_case.expected).abs() <= 1.0e-10,
                "Daycounter: {:?}, period: {:?} to {:?}, reference period: {:?}, to: {:?},\
                 calculated: {}, expected: {}, diff: {}",
                test_case.daycounter,
                test_case.d1,
                test_case.d2,
                test_case.ref_period_start,
                test_case.ref_period_end,
                calculated,
                test_case.expected,
                (calculated - test_case.expected).abs(),
            );
        }
    }

    fn pricing_context(eval_date: Date) -> PricingContext {
        PricingContext { eval_date }
    }

    struct ActualActualTestCase {
        pub daycounter: DayCounter,
        pub d1: Date,
        pub d2: Date,
        pub ref_period_start: Date,
        pub ref_period_end: Date,
        pub expected: Real,
    }

    impl ActualActualTestCase {
        pub fn new(
            daycounter: DayCounter,
            d1: Date,
            d2: Date,
            ref_period_start: Date,
            ref_period_end: Date,
            expected: Real,
        ) -> Self {
            Self {
                daycounter,
                d1,
                d2,
                ref_period_start,
                ref_period_end,
                expected,
            }
        }
    }
}
