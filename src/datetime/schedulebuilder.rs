use crate::context::pricing_context::PricingContext;

use crate::datetime::{
    businessdayconvention::BusinessDayConvention, calendar::Calendar, date::Date,
    dategenerationrule::DateGenerationRule, period::Period,
};

use super::schedule::Schedule;

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
