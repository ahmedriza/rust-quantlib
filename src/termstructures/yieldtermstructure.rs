use crate::time::{
    compounding::Compounding, date::Date, daycounter::DayCounter, frequency::Frequency,
    interestrate::InterestRate, period::Period,
};

use crate::{
    termstructures::termstructure::TermStructure,
    types::{DiscountFactor, Time},
};

/// Interest rate term structure
pub trait YieldTermStructure: TermStructure {
    /// Return the discount factor from a given date to the reference date.
    fn discount_from_date(&self, date: &Date, extrapolate: bool) -> DiscountFactor {
        self.discount_frome_time(self.time_from_references(date), extrapolate)
    }

    /// Return the discount factor from a given time to the reference date.
    /// The time is calculated as a fraction of year from the reference date.
    fn discount_frome_time(&self, time: Time, extrapolate: bool) -> DiscountFactor;

    /// Return the implied zero-yield rate for a given date. The time is calculated as a fraction
    /// of year from the reference date.    
    fn zero_rate_from_date(
        &self,
        date: &Date,
        result_day_counter: &DayCounter,
        compounding: Compounding,
        frequency: Frequency, // TODO default is Annual
        extrapolate: bool,
    ) -> InterestRate;

    /// Return the implied zero-yield rate for a given time.
    /// The resulting interest rate has the same day-counting rule used by the term structure.
    /// The same rule should be used for calculating the passed time t.
    fn zero_rate_from_time(
        &self,
        time: Time,
        compounding: Compounding,
        frequency: Frequency, // TODO default is Annual
        extrapolate: bool,
    ) -> InterestRate;

    /// Returns the forward interest rate between two dates. Ttimes are calculated as fractions of
    /// year from the reference date. If both dates are equal the instantaneous forward rate is
    /// returned.
    ///
    /// The resulting interest rate has the required day-counting rule.
    fn forward_rate_from_dates(
        &self,
        d1: &Date,
        d2: &Date,
        result_day_counter: &DayCounter,
        compounding: Compounding,
        frequency: Frequency, // TODO default is Annual
        extrapolate: bool,
    ) -> InterestRate;

    /// Returns the forward interest rate between `d1` and period `p` after `d1`..
    /// Ttimes are calculated as fractions of year from the reference date. If both dates are
    /// equal the instantaneous forward rate is returned.
    ///
    /// The resulting interest rate has the required day-counting rule. Dates are not adjusted for
    /// holidays.
    fn forward_rate_from_date_period(
        &self,
        d1: &Date,
        p: &Period,
        result_day_counter: &DayCounter,
        compounding: Compounding,
        frequency: Frequency, // TODO default is Annual
        extrapolate: bool,
    ) -> InterestRate {
        self.forward_rate_from_dates(
            d1,
            &(d1 + p),
            result_day_counter,
            compounding,
            frequency,
            extrapolate,
        )
    }

    /// Returns the forward interest rate between two times. If both times are equal the
    /// instantaneous forward rate is returned.
    ///
    /// The resulting interest rate has the same day-counting rule used by the term structure.
    /// The same rule should be used for calculating the passed times `t1` and `t2`.    
    fn forward_rate_from_times(
        &self,
        t1: Time,
        t2: Time,
        compounding: Compounding,
        frequency: Frequency, // TODO default is Annual
        extrapolate: bool,
    ) -> InterestRate;

    /// Return the jump dates
    fn jump_dates(&self) -> Vec<Date>;

    /// Return the jump times
    fn jump_times(&self) -> Vec<Time>;
}
