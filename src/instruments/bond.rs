use crate::{
    datetime::{
        calendar::Calendar, date::Date, daycounter::DayCounter, frequency::Frequency,
        timeunit::TimeUnit::Days,
    },
    rates::compounding::Compounding,
    types::{Integer, Rate, Real, Size}, maths::bounds::lower_bound,
};

pub enum BondPriceType {
    Clean,
    Dirty,
}

pub trait Bond {
    #[allow(clippy::too_many_arguments)]
    /// Calculate the yield given a (clean) price and settlement date. 
    /// 
    /// The settlement date can default to the evaluation date. 
    fn bond_yield(
        &self,
        clean_price: Real,
        daycounter: DayCounter,
        compounding: Compounding,
        frequency: Frequency,
        settlement_date: Date,
        accuracy: Option<Real>,
        max_evaluations: Option<Size>,
        guess: Option<Real>,
        price_type: Option<BondPriceType>,
    ) -> Rate;

    /// Return the [Calendar] associated with this Bond
    fn calendar(&self) -> &Calendar;

    /// Return the Bond issue date 
    fn issue_date(&self) -> Date;

    /// Return the notional schedule dates
    fn notional_schedule(&self) -> &Vec<Date>;
    
    fn notional(&self, date: Date) -> Real {
        let notional_schedule = self.notional_schedule();
        let last_notional_schedule = notional_schedule.last().expect(
            "Notional schedule is empty"
        );
        if &date > last_notional_schedule {
            // after maturity
            return 0.0;
        }

        // After the check above, `date` is between the schedule boundaries.  We search starting
        // from the second notional date, since the first is null.  After the call to lower_bound,
        // `i` is the earliest date which is greater or equal than `date`.
        // Its index is greater or equal to 1.
        let i = lower_bound(notional_schedule, date);
        if date < notional_schedule[i] {
            // no doubt about what to return
            return self.notionals()[i - 1];
        } else {
            // `date` is equal to a redemption date.
            // As per bond conventions, the payment has occurred; the bond already changed notional.
            return self.notionals()[i];
        }
    }

    /// Return the notionals
    fn notionals(&self) -> &Vec<Real>;

    /// Calculate the settlement date 
    fn settlement_date(&self, date: Date) -> Date {
        // usually, the settlement is at T+n...
        let settlement = self.calendar().advance_by_days_with_following(
            date,
            self.settlement_days(),
            Days,
            false,
        );
        // ...but the bond won't be traded until the issue date (if given.)
        if self.issue_date() == Date::default() {
            settlement
        } else {
            settlement.max(self.issue_date())
        }
    }

    /// Return the number of settlement days
    fn settlement_days(&self) -> Integer;
}
