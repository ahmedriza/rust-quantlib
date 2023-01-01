use std::ops::Index;

use crate::context::pricing_context::PricingContext;
use crate::maths::bounds::lower_bound;
use crate::types::{Integer, Size};

use crate::datetime::{
    businessdayconvention::BusinessDayConvention, calendar::Calendar,
    calendars::nullcalendar::NullCalendar, date::Date,
    dategenerationrule::DateGenerationRule, imm::IMM, period::Period, timeunit::TimeUnit::*,
    weekday::Weekday::*,
};

// -------------------------------------------------------------------------------------------------

/// Payment Schedule
#[derive(Clone)]
pub struct Schedule {
    pricing_context: PricingContext,
    dates: Vec<Date>,
    calendar: Calendar,
    convention: BusinessDayConvention,
    termination_date_convention: BusinessDayConvention,
    tenor: Period,
    rule: DateGenerationRule,
    end_of_month: bool,
    is_regular: Vec<bool>,
    first_date: Date,
    next_to_last_date: Date,
}

// -------------------------------------------------------------------------------------------------

/// Schedule Builder
pub struct ScheduleBuilder {
    pricing_context: PricingContext,
    effective_date: Date,
    termination_date: Date,
    tenor: Period,
    calendar: Calendar,
    convention: Option<BusinessDayConvention>,
    termination_date_convention: Option<BusinessDayConvention>,
    date_generation_rule: Option<DateGenerationRule>,
    end_of_month: bool,
    first_date: Date,
    next_to_last_date: Date,
}

impl ScheduleBuilder {
    /// Construct the Builder from the mandatory parameters
    pub fn new(
        pricing_context: PricingContext,
        effective_date: Date,
        termination_date: Date,
        tenor: Period,
        calendar: Calendar,
    ) -> Self {
        Self {
            pricing_context,
            effective_date,
            termination_date,
            tenor,
            calendar,
            convention: None,
            termination_date_convention: None,
            date_generation_rule: None,
            end_of_month: false,
            first_date: Date::default(),
            next_to_last_date: Date::default(),
        }
    }

    /// Set business day convention
    pub fn with_convention(mut self, convention: BusinessDayConvention) -> Self {
        self.convention = Some(convention);
        self
    }

    /// Set termination business day convention
    pub fn with_termination_convention(
        mut self,
        termination_date_convention: BusinessDayConvention,
    ) -> Self {
        self.termination_date_convention = Some(termination_date_convention);
        self
    }

    /// Set date generation rule
    pub fn with_rule(mut self, rule: DateGenerationRule) -> Self {
        self.date_generation_rule = Some(rule);
        self
    }

    /// Construct schedule with [DateGenerationRule::Forward]
    pub fn forwards(mut self) -> Self {
        self.date_generation_rule = Some(DateGenerationRule::Forward);
        self
    }

    /// Construct schedule with [DateGenerationRule::Backward]
    pub fn backwards(mut self) -> Self {
        self.date_generation_rule = Some(DateGenerationRule::Backward);
        self
    }

    /// Set end of month
    pub fn with_end_of_month(mut self, end_of_month: bool) -> Self {
        self.end_of_month = end_of_month;
        self
    }

    /// Set the first date
    pub fn with_first_date(mut self, first_date: Date) -> Self {
        self.first_date = first_date;
        self
    }

    /// Set the next to last date
    pub fn with_next_to_last_date(mut self, next_to_last_date: Date) -> Self {
        self.next_to_last_date = next_to_last_date;
        self
    }

    /// Build the [Schedule]
    pub fn build(self) -> Schedule {
        let convention = if self.convention.is_some() {
            self.convention.unwrap()
        } else {
            BusinessDayConvention::Following
        };
        let termination_date_convention = if self.termination_date_convention.is_some() {
            self.termination_date_convention.unwrap()
        } else {
            // Unadjusted as per ISDA specification
            convention
        };
        let date_generation_rule = if self.date_generation_rule.is_some() {
            self.date_generation_rule.unwrap()
        } else {
            DateGenerationRule::Backward
        };

        Schedule::new(
            self.pricing_context,
            self.effective_date,
            self.termination_date,
            self.tenor,
            self.calendar,
            convention,
            termination_date_convention,
            date_generation_rule,
            self.end_of_month,
            self.first_date,
            self.next_to_last_date,
        )
    }
}

// -------------------------------------------------------------------------------------------------

/// Convenient indexing operation on the Schedule.
///
/// Returns the i-th schedule date.
impl Index<Size> for Schedule {
    type Output = Date;

    fn index(&self, index: Size) -> &Self::Output {
        &self.dates[index]
    }
}

impl Schedule {
    /// Rule based constructor
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        pricing_context: PricingContext,
        effective_date: Date,
        termination_date: Date,
        tenor: Period,
        calendar: Calendar,
        convention: BusinessDayConvention,
        termination_date_convention: BusinessDayConvention,
        date_generation_rule: DateGenerationRule,
        eom: bool, // end of month
        first: Date,
        next_to_last: Date,
    ) -> Self {
        let eval_date = &pricing_context.eval_date;
        let mut result = Self {
            pricing_context,
            dates: vec![],
            calendar,
            convention,
            termination_date_convention,
            tenor,
            rule: date_generation_rule,
            end_of_month: if allows_end_of_month(&tenor) {
                eom
            } else {
                false
            },
            is_regular: vec![],
            first_date: if first == effective_date {
                Date::default()
            } else {
                first
            },
            next_to_last_date: if next_to_last == termination_date {
                Date::default()
            } else {
                next_to_last
            },
        };
        // sanity checks
        assert!(
            termination_date != Date::default(),
            "Termination date cannot be the null Date: {:?}",
            Date::default()
        );

        // in many cases (e.g. non-expired bonds) the effective date is not
        // really necessary. In these cases a decent placeholder is enough
        let mut effective_date = effective_date;
        if effective_date == Date::default()
            && first == Date::default()
            && date_generation_rule == DateGenerationRule::Backward
        {
            assert!(
                eval_date < &termination_date,
                "eval date ({:?}) is >= termination date ({:?})",
                eval_date,
                termination_date
            );
            if next_to_last != Date::default() {
                let date_diff_serial = &next_to_last - eval_date;
                let y = date_diff_serial / 366 + 1;
                effective_date = next_to_last - Period::new(y, Years);
            }
        } else {
            assert!(
                effective_date != Date::default(),
                "null effective date: {:?}",
                effective_date
            );
        }

        assert!(
            effective_date < termination_date,
            "effective date ({:?}) later than or equal to termination date ({:?})",
            effective_date,
            termination_date
        );

        if tenor.length == 0 {
            result.rule = DateGenerationRule::Zero;
        } else {
            assert!(
                tenor.length > 0,
                "non positive tenor ({:?}) not allowed",
                tenor
            );
        }

        if result.first_date != Date::default() {
            match result.rule {
                DateGenerationRule::Backward | DateGenerationRule::Forward => {
                    assert!(
                        result.first_date > effective_date && result.first_date <= termination_date,
                        "first date ({:?}) out of effective-termination date range ({:?}, {:?}]",
                        result.first_date,
                        effective_date,
                        termination_date
                    );
                    // we should ensure that the above condition is still
                    // verified after adjustment
                }
                DateGenerationRule::ThirdWednesday => {
                    let imm = IMM::new(pricing_context);
                    assert!(
                        imm.is_imm_date(&result.first_date, false),
                        "first date ({:?}) is not an IMM date",
                        result.first_date
                    );
                }
                DateGenerationRule::Zero
                | DateGenerationRule::Twentieth
                | DateGenerationRule::TwentiethIMM
                | DateGenerationRule::OldCDS
                | DateGenerationRule::CDS
                | DateGenerationRule::CDS2015 => {
                    panic!(
                        "first date incompatible with {:?} date generation rule",
                        result.rule
                    )
                }
                other => panic!("Invalid date generation rule {:?}", other),
            }
        }

        if result.next_to_last_date != Date::default() {
            match result.rule {
                DateGenerationRule::Backward | DateGenerationRule::Forward => {
                    assert!(
                        result.next_to_last_date >= effective_date
                            && result.next_to_last_date < termination_date,
                        "next to last date ({:?}) out of effective-termination date range \
                         ({:?}, {:?}]",
                        result.next_to_last_date,
                        effective_date,
                        termination_date
                    );
                    // we should ensure that the above condition is still
                    // verified after adjustment
                }
                DateGenerationRule::ThirdWednesday => {
                    let imm = IMM::new(pricing_context);
                    assert!(
                        imm.is_imm_date(&result.next_to_last_date, false),
                        "next to last date ({:?}) is not an IMM date",
                        result.next_to_last_date
                    );
                }
                DateGenerationRule::Zero
                | DateGenerationRule::Twentieth
                | DateGenerationRule::TwentiethIMM
                | DateGenerationRule::OldCDS
                | DateGenerationRule::CDS
                | DateGenerationRule::CDS2015 => {
                    panic!(
                        "next to last date incompatible with {:?} date generation rule",
                        result.rule
                    )
                }
                other => panic!("Invalid date generation rule {:?}", other),
            }
        }

        // calendar needed for endOfMonth adjustment
        let null_calendar = NullCalendar::new();
        let mut periods = 1;
        let mut seed = Date::default();
        let mut exit_date = Date::default();
        match result.rule {
            DateGenerationRule::Zero => {
                result.tenor = Period::new(0, Years);
                result.dates.push(effective_date);
                result.dates.push(termination_date);
                result.is_regular.push(true);
            }
            DateGenerationRule::Backward => {
                result.dates.push(termination_date);
                seed = termination_date;
                if result.next_to_last_date != Date::default() {
                    result.dates.insert(0, result.next_to_last_date); // add to front
                    let period = result.tenor * (-periods);
                    let temp = null_calendar.advance_by_period(
                        seed,
                        &period,
                        result.convention,
                        result.end_of_month,
                    );
                    if temp != result.next_to_last_date {
                        result.is_regular.insert(0, false);
                    } else {
                        result.is_regular.insert(0, true);
                    }
                    seed = result.next_to_last_date;
                }
                exit_date = effective_date;
                if result.first_date != Date::default() {
                    exit_date = result.first_date;
                }
                loop {
                    let period = result.tenor * (-periods);
                    let temp = null_calendar.advance_by_period(
                        seed,
                        &period,
                        result.convention,
                        result.end_of_month,
                    );
                    if temp < exit_date {
                        if result.first_date != Date::default()
                            && (result
                                .calendar
                                .adjust(*result.dates.first().unwrap(), convention)
                                != result.calendar.adjust(result.first_date, convention))
                        {
                            result.dates.insert(0, result.first_date);
                            result.is_regular.insert(0, false);
                        }
                        break;
                    } else {
                        // skip dates that would result in duplicates after adjustment
                        if result
                            .calendar
                            .adjust(*result.dates.first().unwrap(), convention)
                            != result.calendar.adjust(temp, convention)
                        {
                            result.dates.insert(0, temp);
                            result.is_regular.insert(0, true);
                        }
                        periods += 1;
                    }
                }

                if result
                    .calendar
                    .adjust(*result.dates.first().unwrap(), convention)
                    != result.calendar.adjust(effective_date, convention)
                {
                    result.dates.insert(0, effective_date);
                    result.is_regular.insert(0, false);
                }
            }

            DateGenerationRule::Twentieth
            | DateGenerationRule::TwentiethIMM
            | DateGenerationRule::ThirdWednesday
            | DateGenerationRule::ThirdWednesdayInclusive
            | DateGenerationRule::OldCDS
            | DateGenerationRule::CDS
            | DateGenerationRule::CDS2015
            | DateGenerationRule::Forward => {
                // non-forward rule
                if result.rule != DateGenerationRule::Forward {
                    assert!(
                        !result.end_of_month,
                        "end of month convention incompatible with {:?} date generation rule",
                        result.rule
                    );
                }
                // CDS rules
                if result.rule == DateGenerationRule::CDS
                    || result.rule == DateGenerationRule::CDS2015
                {
                    let prev20th = previous_twentieth(&effective_date, result.rule);
                    if result.calendar.adjust(prev20th, convention) > effective_date {
                        result.dates.push(prev20th - Period::new(3, Months));
                        result.is_regular.push(true);
                    }
                    result.dates.push(prev20th);
                } else {
                    result.dates.push(effective_date);
                }

                seed = result.dates[result.dates.len() - 1];

                if result.first_date != Date::default() {
                    result.dates.push(result.first_date);
                    let period = result.tenor * periods;
                    let temp = null_calendar.advance_by_period(
                        seed,
                        &period,
                        convention,
                        result.end_of_month,
                    );
                    if temp != result.first_date {
                        result.is_regular.push(false);
                    } else {
                        result.is_regular.push(true);
                    }
                    seed = result.first_date;
                } else if result.rule == DateGenerationRule::Twentieth
                    || result.rule == DateGenerationRule::TwentiethIMM
                    || result.rule == DateGenerationRule::OldCDS
                    || result.rule == DateGenerationRule::CDS
                    || result.rule == DateGenerationRule::CDS2015
                {
                    let mut next20th = next_twentieth(&effective_date, result.rule);
                    if result.rule == DateGenerationRule::OldCDS {
                        // distance rule enforced in natural days
                        let stub_days = 30;
                        if (next20th - effective_date) < stub_days {
                            // +1 will skip this one and get the next
                            next20th = next_twentieth(&(next20th + 1), result.rule);
                        }
                    }
                    if next20th != effective_date {
                        result.dates.push(next20th);
                        result.is_regular.push(
                            result.rule == DateGenerationRule::CDS
                                || result.rule == DateGenerationRule::CDS2015,
                        );
                        seed = next20th;
                    }
                }

                exit_date = termination_date;

                if result.next_to_last_date != Date::default() {
                    exit_date = result.next_to_last_date;
                }

                loop {
                    let period = result.tenor * periods;
                    let temp = null_calendar.advance_by_period(
                        seed,
                        &period,
                        convention,
                        result.end_of_month,
                    );
                    if temp > exit_date {
                        if result.next_to_last_date != Date::default()
                            && (result
                                .calendar
                                .adjust(result.dates[result.dates.len() - 1], convention)
                                != result.calendar.adjust(result.next_to_last_date, convention))
                        {
                            result.dates.push(result.next_to_last_date);
                            result.is_regular.push(false);
                        }
                        break;
                    } else {
                        // skip dates that would result in duplicates after adjustments
                        if result
                            .calendar
                            .adjust(result.dates[result.dates.len() - 1], convention)
                            != result.calendar.adjust(temp, convention)
                        {
                            result.dates.push(temp);
                            result.is_regular.push(true);
                        }
                        periods += 1;
                    }
                }

                if result.calendar.adjust(
                    result.dates[result.dates.len() - 1],
                    result.termination_date_convention,
                ) != result
                    .calendar
                    .adjust(termination_date, result.termination_date_convention)
                {
                    if result.rule == DateGenerationRule::Twentieth
                        || result.rule == DateGenerationRule::TwentiethIMM
                        || result.rule == DateGenerationRule::OldCDS
                        || result.rule == DateGenerationRule::CDS
                        || result.rule == DateGenerationRule::CDS2015
                    {
                        result
                            .dates
                            .push(next_twentieth(&termination_date, result.rule));
                        result.is_regular.push(true);
                    } else {
                        result.dates.push(termination_date);
                        result.is_regular.push(false);
                    }
                }
            }
        }

        // adjustments
        if result.rule == DateGenerationRule::ThirdWednesday {
            for i in 1..result.dates.len() - 1 {
                result.dates[i] = Date::nth_weekday(
                    3,
                    Wednesday,
                    result.dates[i].month(),
                    result.dates[i].year(),
                );
            }
        } else if result.rule == DateGenerationRule::ThirdWednesdayInclusive {
            for d in result.dates.iter_mut() {
                *d = Date::nth_weekday(3, Wednesday, d.month(), d.year());
            }
        }

        if result.end_of_month && result.calendar.is_end_of_month(&seed) {
            // adjust to end of month
            if convention == BusinessDayConvention::Unadjusted {
                for i in 1..result.dates.len() - 1 {
                    result.dates[i] = result.dates[i].end_of_month();
                }
            } else {
                for i in 1..result.dates.len() - 1 {
                    result.dates[i] = result.calendar.end_of_month(&result.dates[i]);
                }
            }
            let mut d1 = result.dates[0];
            let mut d2 = result.dates[result.dates.len() - 1];
            if result.termination_date_convention != BusinessDayConvention::Unadjusted {
                d1 = result.calendar.end_of_month(&result.dates[0]);
                d2 = result
                    .calendar
                    .end_of_month(&result.dates[result.dates.len() - 1]);
            } else {
                // the termination date is the first if going backwards, the last otherwise.
                if result.rule == DateGenerationRule::Backward {
                    d2 = result.dates[result.dates.len() - 1].end_of_month();
                } else {
                    d1 = result.dates[0].end_of_month();
                }
            }
            // if the eom adjustment leads to a single date schedule we do not apply it
            if d1 != d2 {
                result.dates[0] = d1;
                let len = result.dates.len();
                result.dates[len - 1] = d2;
            }
        } else {
            // first date not adjusted for old CDS schedules
            if result.rule != DateGenerationRule::OldCDS {
                result.dates[0] = result.calendar.adjust(result.dates[0], convention);
            }
            for i in 1..result.dates.len() - 1 {
                result.dates[i] = result.calendar.adjust(result.dates[i], convention);
            }

            // termination date is NOT adjusted as per ISDA specifications, unless otherwise
            // specified in the confirmation of the deal or unless we're creating a CDS schedule
            if result.termination_date_convention != BusinessDayConvention::Unadjusted
                && result.rule != DateGenerationRule::CDS
                && result.rule != DateGenerationRule::CDS2015
            {
                let len = result.dates.len();
                result.dates[len - 1] = result.calendar.adjust(result.dates[len - 1], convention);
            }
        }

        // Final safety checks to remove extra next-to-last date, if necessary. It can happen to
        // be equal or later than the end date due to EOM adjustments (see the Schedule test suite
        // for an example).
        if result.dates.len() >= 2
            && result.dates[result.dates.len() - 2] >= result.dates[result.dates.len() - 1]
        {
            // there might be two dates only, then is_regular has size one
            if result.is_regular.len() >= 2 {
                let len = result.is_regular.len();
                result.is_regular[len - 2] =
                    result.dates[result.dates.len() - 2] == result.dates[result.dates.len() - 1];
            }
            let len = result.dates.len();
            result.dates[len - 2] = result.dates[len - 1];
            result.dates.pop();
            result.is_regular.pop();
        }

        if result.dates.len() >= 2 && result.dates[1] <= result.dates[0] {
            result.is_regular[1] = result.dates[1] == result.dates[0];
            result.dates[1] = result.dates[0];
            result.dates.remove(0); // this is expensive
            result.is_regular.remove(0); // this is expensive
        }

        assert!(
            result.dates.len() > 1,
            "Degenerate single date ({:?}) schedule \
                 \n seed date: {:?} \
                 \n exit date: {:?} \
                 \n effective date: {:?} \
                 \n first date: {:?} \
                 \n next to last date: {:?} \
                 \n termination date: {:?} \
                 \n generation rule: {:?} \
                 \n end of month: {:?}
                 ",
            result.dates[0],
            seed,
            exit_date,
            effective_date,
            first,
            next_to_last,
            termination_date,
            result.rule,
            result.end_of_month
        );

        result
    }

    /// Size of the schedule, i.e. the number of dates.
    pub fn size(&self) -> Size {
        self.dates.len()
    }

    /// Return the first date that is bigger than `ref_date` in the schedule.
    pub fn next_date(&self, ref_date: &Date) -> Date {
        let i = self.lower_bound(ref_date);
        if i != self.dates.len() {
            self.dates[i]
        } else {
            Date::default()
        }
    }

    /// Return the first date that is smaller than `ref_date` in the schedule.
    pub fn previous_date(&self, ref_date: &Date) -> Date {
        let i = self.lower_bound(ref_date);
        if i != 0 {
            self.dates[i - 1]
        } else {
            Date::default()
        }
    }

    pub fn has_is_regular(&self) -> bool {
        !self.is_regular.is_empty()
    }

    pub fn is_regular(&self, i: Size) -> bool {
        assert!(
            i <= self.is_regular.len() && i > 0,
            "index ({}) must be in [1, {}]",
            i,
            self.is_regular.len()
        );
        self.is_regular[i - 1]
    }

    /// Check whether the schedule has been constructed or not.
    pub fn empty(&self) -> bool {
        self.dates.is_empty()
    }

    /// Return the first date of the schedule.
    pub fn start_date(&self) -> &Date {
        &self.dates[0]
    }

    /// Return the last date of the schedule.
    pub fn end_date(&self) -> &Date {
        &self.dates[self.dates.len() - 1]
    }

    /// Truncate schedule, i.e. remove dates strictly before the given `truncation_date`. That is,
    /// produce a new schedule with dates greater than or equal to the `truncation_date`.
    pub fn after(&self, truncation_date: &Date) -> Self {
        assert!(
            truncation_date < &self.dates[self.dates.len() - 1],
            "Truncation date {:?} must be before the last schedule date {:?}",
            truncation_date,
            self.dates.last()
        );

        let mut result = self.clone();

        if truncation_date > &result.dates[0] {
            // remove earlier dates
            while &result.dates[0] < truncation_date {
                result.dates.remove(0);
                if !result.is_regular.is_empty() {
                    result.is_regular.remove(0);
                }
            }
            // add truncation date if missing
            if *truncation_date != result.dates[0] {
                result.dates.insert(0, *truncation_date);
                result.is_regular.insert(0, false);
                result.termination_date_convention = BusinessDayConvention::Unadjusted;
            }

            if &result.next_to_last_date <= truncation_date {
                result.next_to_last_date = Date::default();
            }
            if &result.first_date <= truncation_date {
                result.first_date = Date::default();
            }
        }
        result
    }

    /// Truncate schedule, i.e. remove dates strictly after the given `truncation_date`. That is,
    /// produce a new schedule with dates up to the `truncation_date`.
    pub fn until(&self, truncation_date: &Date) -> Self {
        assert!(
            truncation_date > &self.dates[0],
            "Truncation date {:?} must be later than schedule first date {:?}",
            truncation_date,
            self.dates[0]
        );

        let mut result = self.clone();

        if truncation_date < &result.dates[result.dates.len() - 1] {
            // remove later dates
            let mut idx = result.dates.len() - 1;
            while &result.dates[idx] > truncation_date {
                result.dates.pop();
                idx = result.dates.len() - 1;
                if !result.is_regular.is_empty() {
                    result.is_regular.pop();
                }
            }
            // add truncation date if missing
            if truncation_date != &result.dates[result.dates.len() - 1] {
                result.dates.push(*truncation_date);
                result.is_regular.push(false);
                result.termination_date_convention = BusinessDayConvention::Unadjusted;
            }
            if &result.next_to_last_date >= truncation_date {
                result.next_to_last_date = Date::default();
            }
            if &result.first_date >= truncation_date {
                result.first_date = Date::default();
            }
        }

        result
    }

    fn lower_bound(&self, ref_date: &Date) -> Size {
        let d = if ref_date == &Date::default() {
            self.pricing_context.eval_date
        } else {
            *ref_date
        };
        lower_bound(&self.dates, d)
    }
}

// -------------------------------------------------------------------------------------------------

/// Helper function for returning the date on or before date `d` that is the 20th of
/// the month and obeserves the given date generation `rule` if it is relevant.
pub fn previous_twentieth(d: &Date, rule: DateGenerationRule) -> Date {
    let mut result = Date::new(20, d.month(), d.year());
    if &result > d {
        result -= Period::new(1, Months);
    }
    if rule == DateGenerationRule::TwentiethIMM
        || rule == DateGenerationRule::OldCDS
        || rule == DateGenerationRule::CDS
        || rule == DateGenerationRule::CDS2015
    {
        let m = result.month();
        if m as Integer % 3 != 0 {
            // not a main IMM month
            let skip = m as Integer % 3;
            result -= Period::new(skip, Months);
        }
    }
    result
}

fn next_twentieth(d: &Date, rule: DateGenerationRule) -> Date {
    let mut result = Date::new(20, d.month(), d.year());
    if &result < d {
        result += Period::new(1, Months);
    }
    if rule == DateGenerationRule::TwentiethIMM
        || rule == DateGenerationRule::OldCDS
        || rule == DateGenerationRule::CDS
        || rule == DateGenerationRule::CDS2015
    {
        let m = result.month();
        if m as Integer % 3 != 0 {
            // not a main IMM month
            let skip = 3 - m as Integer % 3;
            result += Period::new(skip, Months);
        }
    }
    result
}

fn allows_end_of_month(tenor: &Period) -> bool {
    (tenor.unit == Months || tenor.unit == Years) && tenor >= &Period::new(1, Months)
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::context::pricing_context::PricingContext;
    use crate::datetime::{
        businessdayconvention::BusinessDayConvention,
        calendars::{japan::Japan, target::Target, unitedstates::UnitedStates},
        date::Date,
        frequency::Frequency,
        months::Month::*,
        period::Period,
        timeunit::TimeUnit::*,
    };

    use super::{Schedule, ScheduleBuilder};

    #[test]
    fn test_next_date() {
        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(1, December, 2022),
            Date::new(31, December, 2023),
            Period::new(2, Weeks),
            Target::new(),
        )
        .with_convention(BusinessDayConvention::Following)
        .forwards()
        .build();
        assert_eq!(s.size(), 30);
        assert_eq!(s.start_date(), &Date::new(1, December, 2022));
        assert_eq!(s.end_date(), &Date::new(2, January, 2024));
        let next_date = s.next_date(&Date::new(1, December, 2022));
        // Two weeks from 2022-12-01 is 2022-12-15
        assert_eq!(next_date, Date::new(1, December, 2022));
    }

    #[test]
    fn test_previous_date() {
        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(1, December, 2022),
            Date::new(31, December, 2023),
            Period::new(2, Weeks),
            Target::new(),
        )
        .with_convention(BusinessDayConvention::Following)
        .forwards()
        .build();
        assert_eq!(s.size(), 30);
        let previous_date = s.previous_date(&Date::new(16, December, 2022));
        // The date in the schedule before 2022-12-16 is 2022-12-15
        assert_eq!(previous_date, Date::new(15, December, 2022));
    }

    #[test]
    fn test_after() {
        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(1, December, 2022),
            Date::new(31, December, 2023),
            Period::new(2, Weeks),
            Target::new(),
        )
        .with_convention(BusinessDayConvention::Following)
        .forwards()
        .build();
        assert_eq!(s.size(), 30);

        assert_eq!(s.start_date(), &Date::new(1, December, 2022));
        assert_eq!(s.end_date(), &Date::new(2, January, 2024));

        // Truncate the schedule
        let t = s.after(&Date::new(30, November, 2023));
        assert_eq!(t.start_date(), &Date::new(30, November, 2023));
        assert_eq!(t.size(), 4);
    }

    #[test]
    fn test_until() {
        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(1, December, 2022),
            Date::new(31, December, 2023),
            Period::new(2, Weeks),
            Target::new(),
        )
        .with_convention(BusinessDayConvention::Following)
        .forwards()
        .build();
        assert_eq!(s.size(), 30);

        assert_eq!(s.start_date(), &Date::new(1, December, 2022));
        assert_eq!(s.end_date(), &Date::new(2, January, 2024));

        // Truncate the schedule
        let t = s.until(&Date::new(30, November, 2023));
        assert_eq!(t.end_date(), &Date::new(30, November, 2023));
        assert_eq!(t.size(), 27);
    }

    #[test]
    fn test_daily_schedule() {
        let start_date = Date::new(17, January, 2012);
        let s = ScheduleBuilder::new(
            pricing_context(),
            start_date,
            start_date + 7,
            Period::from(Frequency::Daily),
            Target::new(),
        )
        .with_convention(BusinessDayConvention::Preceding)
        .build();

        let expected = vec![
            Date::new(17, January, 2012),
            Date::new(18, January, 2012),
            Date::new(19, January, 2012),
            Date::new(20, January, 2012),
            Date::new(23, January, 2012),
            Date::new(24, January, 2012),
        ];

        check_dates(&s, &expected);
    }

    #[test]
    fn test_end_date_with_eom_adjustment() {
        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(30, September, 2009),
            Date::new(15, June, 2012),
            Period::new(6, Months),
            Japan::new(),
        )
        .with_convention(BusinessDayConvention::Following)
        .forwards()
        .with_end_of_month(true)
        .build();

        let mut expected = vec![
            Date::new(30, September, 2009),
            Date::new(31, March, 2010),
            Date::new(30, September, 2010),
            Date::new(31, March, 2011),
            Date::new(30, September, 2011),
            Date::new(30, March, 2012),
            Date::new(29, June, 2012),
        ];
        check_dates(&s, &expected);

        // now with unadjusted termination date...
        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(30, September, 2009),
            Date::new(15, June, 2012),
            Period::new(6, Months),
            Japan::new(),
        )
        .with_convention(BusinessDayConvention::Following)
        .with_termination_convention(BusinessDayConvention::Unadjusted)
        .forwards()
        .with_end_of_month(true)
        .build();
        // ...which should leave it alone.
        expected[6] = Date::new(15, June, 2012);
        check_dates(&s, &expected);
    }

    #[test]
    fn test_dates_past_end_date_with_eom_adjustment() {
        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(28, March, 2013),
            Date::new(30, March, 2015),
            Period::new(1, Years),
            Target::new(),
        )
        .with_convention(BusinessDayConvention::Unadjusted)
        .forwards()
        .with_end_of_month(true)
        .build();

        let expected = vec![
            Date::new(31, March, 2013),
            Date::new(31, March, 2014),
            // March 31st 2015, coming from the EOM adjustment of March 28th,
            // should be discarded as past the end date.
            Date::new(30, March, 2015),
        ];
        check_dates(&s, &expected);

        // also, the last period should not be regular.
        assert!(!s.is_regular(2), "last period should not be regular")
    }

    #[test]
    fn test_dates_same_as_end_date_with_eom_adjustment() {
        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(28, March, 2013),
            Date::new(31, March, 2015),
            Period::new(1, Years),
            Target::new(),
        )
        .with_convention(BusinessDayConvention::Unadjusted)
        .forwards()
        .with_end_of_month(true)
        .build();

        let expected = vec![
            Date::new(31, March, 2013),
            Date::new(31, March, 2014),
            // March 31st 2015, coming from the EOM adjustment of March 28th,
            Date::new(31, March, 2015),
        ];
        check_dates(&s, &expected);

        // also, the last period should be regular.
        assert!(s.is_regular(2), "last period should be regular")
    }

    #[test]
    fn test_forward_dates_with_eom_adjustment() {
        // Testing that the last date is not adjusted for EOM when
        // termination date convention is unadjusted...
        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(31, August, 1996),
            Date::new(15, September, 1997),
            Period::new(6, Months),
            UnitedStates::government_bond(),
        )
        .with_convention(BusinessDayConvention::Unadjusted)
        .forwards()
        .with_end_of_month(true)
        .build();

        let expected = vec![
            Date::new(31, August, 1996),
            Date::new(28, February, 1997),
            Date::new(31, August, 1997),
            Date::new(15, September, 1997),
        ];
        check_dates(&s, &expected);
    }

    #[test]
    fn test_backward_dates_with_eom_adjustment() {
        // Testing that the first date is not adjusted for EOM "
        // going backward when termination date convention is unadjusted..."
        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(22, August, 1996),
            Date::new(31, August, 1997),
            Period::new(6, Months),
            UnitedStates::government_bond(),
        )
        .with_convention(BusinessDayConvention::Unadjusted)
        .backwards()
        .with_end_of_month(true)
        .build();

        let expected = vec![
            Date::new(22, August, 1996),
            Date::new(31, August, 1996),
            Date::new(28, February, 1997),
            Date::new(31, August, 1997),
        ];
        check_dates(&s, &expected);
    }

    #[test]
    fn test_double_first_date_with_eom_adjustment() {
        // Testing that the first date is not duplicated due to
        // EOM convention when going backwards...
        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(22, August, 1996),
            Date::new(31, August, 1997),
            Period::new(6, Months),
            UnitedStates::government_bond(),
        )
        .with_convention(BusinessDayConvention::Following)
        .backwards()
        .with_end_of_month(true)
        .build();

        let expected = vec![
            Date::new(30, August, 1996),
            Date::new(28, February, 1997),
            Date::new(29, August, 1997),
        ];
        check_dates(&s, &expected);
    }

    #[test]
    fn test_four_weeks_tenor() {
        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(13, January, 2016),
            Date::new(4, May, 2016),
            Period::new(4, Weeks),
            Target::new(),
        )
        .with_convention(BusinessDayConvention::Following)
        .forwards()
        .build();
        assert_eq!(s.size(), 5);
    }

    #[test]
    fn test_schedule_always_has_a_start_date() {
        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(10, January, 2017),
            Date::new(28, February, 2026),
            Period::from(Frequency::Semiannual),
            UnitedStates::government_bond(),
        )
        .with_first_date(Date::new(31, August, 2017))
        .with_convention(BusinessDayConvention::Unadjusted)
        .backwards()
        .build();
        assert_eq!(
            s.start_date(),
            &Date::new(10, January, 2017),
            "first element should always be the start date"
        );

        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(10, January, 2017),
            Date::new(28, February, 2026),
            Period::from(Frequency::Semiannual),
            UnitedStates::government_bond(),
        )
        .with_first_date(Date::new(31, August, 2017))
        .with_convention(BusinessDayConvention::Unadjusted)
        .backwards()
        .build();
        assert_eq!(
            s.start_date(),
            &Date::new(10, January, 2017),
            "first element should always be the start date"
        );

        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(31, August, 2017),
            Date::new(28, February, 2026),
            Period::from(Frequency::Semiannual),
            UnitedStates::government_bond(),
        )
        .with_convention(BusinessDayConvention::Unadjusted)
        .backwards()
        .build();
        assert_eq!(
            s.start_date(),
            &Date::new(31, August, 2017),
            "first element should always be the start date"
        );
    }

    #[test]
    fn test_short_eom_schedule() {
        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(21, February, 2019),
            Date::new(28, February, 2019),
            Period::new(1, Years),
            Target::new(),
        )
        .with_convention(BusinessDayConvention::ModifiedFollowing)
        .backwards()
        .with_end_of_month(true)
        .build();

        assert_eq!(s.size(), 2);
        assert_eq!(s.start_date(), &Date::new(21, February, 2019));
        assert_eq!(s.end_date(), &Date::new(28, February, 2019));
    }

    #[test]
    fn test_first_date_on_maturity() {
        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(20, September, 2016),
            Date::new(20, December, 2016),
            Period::from(Frequency::Quarterly),
            UnitedStates::government_bond(),
        )
        .with_first_date(Date::new(20, December, 2016))
        .with_convention(BusinessDayConvention::Unadjusted)
        .backwards()
        .build();

        let expected = vec![
            Date::new(20, September, 2016),
            Date::new(20, December, 2016),
        ];
        check_dates(&s, &expected);

        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(20, September, 2016),
            Date::new(20, December, 2016),
            Period::from(Frequency::Quarterly),
            UnitedStates::government_bond(),
        )
        .with_first_date(Date::new(20, December, 2016))
        .with_convention(BusinessDayConvention::Unadjusted)
        .forwards()
        .build();
        check_dates(&s, &expected);
    }

    #[test]
    fn test_next_to_last_date_on_start() {
        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(20, September, 2016),
            Date::new(20, December, 2016),
            Period::from(Frequency::Quarterly),
            UnitedStates::government_bond(),
        )
        .with_next_to_last_date(Date::new(20, September, 2016))
        .with_convention(BusinessDayConvention::Unadjusted)
        .backwards()
        .build();

        let expected = vec![
            Date::new(20, September, 2016),
            Date::new(20, December, 2016),
        ];
        check_dates(&s, &expected);

        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(20, September, 2016),
            Date::new(20, December, 2016),
            Period::from(Frequency::Quarterly),
            UnitedStates::government_bond(),
        )
        .with_next_to_last_date(Date::new(20, September, 2016))
        .with_convention(BusinessDayConvention::Unadjusted)
        .forwards()
        .build();

        check_dates(&s, &expected);
    }

    #[test]
    fn test_truncation() {
        let s = ScheduleBuilder::new(
            pricing_context(),
            Date::new(30, September, 2009),
            Date::new(15, June, 2020),
            Period::new(6, Months),
            Japan::new(),
        )
        .with_convention(BusinessDayConvention::Following)
        .with_termination_convention(BusinessDayConvention::Following)
        .forwards()
        .with_end_of_month(true)
        .build();

        // Until
        let t = s.until(&Date::new(1, January, 2014));
        let expected = vec![
            Date::new(30, September, 2009),
            Date::new(31, March, 2010),
            Date::new(30, September, 2010),
            Date::new(31, March, 2011),
            Date::new(30, September, 2011),
            Date::new(30, March, 2012),
            Date::new(28, September, 2012),
            Date::new(29, March, 2013),
            Date::new(30, September, 2013),
            Date::new(1, January, 2014),
        ];
        check_dates(&t, &expected);
        let idx = t.is_regular.len() - 1;
        assert!(
            !t.is_regular[idx],
            "expected is_regular[{}] to be false",
            idx
        );

        // Until, with truncation date falling on a schedule date
        let t = s.until(&Date::new(30, September, 2013));
        let expected = vec![
            Date::new(30, September, 2009),
            Date::new(31, March, 2010),
            Date::new(30, September, 2010),
            Date::new(31, March, 2011),
            Date::new(30, September, 2011),
            Date::new(30, March, 2012),
            Date::new(28, September, 2012),
            Date::new(29, March, 2013),
            Date::new(30, September, 2013),
        ];
        check_dates(&t, &expected);
        let idx = t.is_regular.len() - 1;
        assert!(t.is_regular[idx], "expected is_regular[{}] to be true", idx);

        // After
        let t = s.after(&Date::new(1, January, 2014));
        let expected = vec![
            Date::new(1, January, 2014),
            Date::new(31, March, 2014),
            Date::new(30, September, 2014),
            Date::new(31, March, 2015),
            Date::new(30, September, 2015),
            Date::new(31, March, 2016),
            Date::new(30, September, 2016),
            Date::new(31, March, 2017),
            Date::new(29, September, 2017),
            Date::new(30, March, 2018),
            Date::new(28, September, 2018),
            Date::new(29, March, 2019),
            Date::new(30, September, 2019),
            Date::new(31, March, 2020),
            Date::new(30, June, 2020),
        ];
        check_dates(&t, &expected);
        assert!(!t.is_regular[0], "expected is_regular[{}] to be false", 0);

        // After, with truncation date falling on a schedule date
        let t = s.after(&Date::new(28, September, 2018));
        let expected = vec![
            Date::new(28, September, 2018),
            Date::new(29, March, 2019),
            Date::new(30, September, 2019),
            Date::new(31, March, 2020),
            Date::new(30, June, 2020),
        ];
        check_dates(&t, &expected);
        assert!(t.is_regular[0], "expected is_regular[{}] to be true", 0);
    }

    fn check_dates(s: &Schedule, expected: &[Date]) {
        assert_eq!(
            s.size(),
            expected.len(),
            "expected {} dates, found {}",
            expected.len(),
            s.size()
        );
        for i in 0..expected.len() {
            let computed = s[i];
            assert_eq!(
                computed, expected[i],
                "expected {:?} at index {}, found {:?}",
                expected[i], i, computed
            );
        }
    }

    fn pricing_context() -> PricingContext {
        PricingContext {
            eval_date: Date::new(1, December, 2022),
        }
    }
}
