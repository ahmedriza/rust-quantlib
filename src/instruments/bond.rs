use crate::{
    cashflows::cashflow::Leg,
    datetime::{
        calendar::Calendar, date::Date, daycounter::DayCounter, frequency::Frequency,
        timeunit::TimeUnit::Days,
    },
    maths::bounds::lower_bound,
    pricingengines::bond::bondfunctions,
    rates::{compounding::Compounding, interestrate::InterestRate},
    types::{Integer, Rate, Real, Size},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BondPriceType {
    Clean,
    Dirty,
}

pub trait Bond {
    fn accrued_amount(&self, settlement_date: Date) -> Real {
        if !self.is_tradeable(settlement_date) {
            return 0.0;
        }
        let current_notional = self.notional(settlement_date);
        if current_notional == 0.0 {
            return 0.0;
        }
        bondfunctions::accrued_amount(
            self.cashflows(),
            self.notional(settlement_date),
            settlement_date,
        )
    }

    #[allow(clippy::too_many_arguments)]
    /// Calculate the yield given a (clean) price and settlement date.
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
    ) -> Rate {
        assert!(
            self.is_tradeable(settlement_date),
            "Non tradeable at {:?}, (maturity being {:?})",
            settlement_date,
            self.maturity_date()
        );

        let accuracy = accuracy.unwrap_or(1.0e-8);
        let max_evaluations = max_evaluations.unwrap_or(100);
        let guess = guess.unwrap_or(0.05);

        let current_notional = self.notional(settlement_date);
        if current_notional == 0.0 {
            return 0.0;
        }

        let mut dirty_price = clean_price + self.accrued_amount(settlement_date);
        dirty_price /= 100.0 / self.notional(settlement_date);

        bondfunctions::bond_yield(
            self.cashflows(),
            dirty_price,
            daycounter,
            compounding,
            frequency,
            settlement_date,
            accuracy,
            max_evaluations,
            guess,
        )
    }

    /// Return the [Calendar] associated with this Bond
    fn calendar(&self) -> &Calendar;

    /// Return the cashflows
    fn cashflows(&self) -> &Leg;

    /// Theoretical clean price.
    ///
    /// The theoretical price calculated from a flat term structure might differ slightly from
    /// the price calculated from the corresponding yield.
    fn clean_price(&self, pricing_date: Date, settlement_date: Date) -> Real {
        self.dirty_price(pricing_date, settlement_date) - self.accrued_amount(settlement_date)
    }

    /// Theoretical dirty price
    ///
    /// The theoretical price calculated from a flat term structure might differ slightly from
    /// the price calculated from the corresponding yield.
    fn dirty_price(&self, pricing_date: Date, _settlement_date: Date) -> Real {
        let settlement_date = self.settlement_date(pricing_date);
        let current_notional = self.notional(settlement_date);
        if current_notional == 0.0 {
            return 0.0;
        }
        // TODO needs bond pricing engine implementation
        todo!()
    }

    /// Clean price given a yield and settlement date
    fn clean_price_from_yield(
        &self,
        y: Rate,
        daycounter: DayCounter,
        compounding: Compounding,
        frequency: Frequency,
        settlement_date: Date,
    ) -> Real {
        self.dirty_price_from_yield(y, daycounter, compounding, frequency, settlement_date)
            - self.accrued_amount(settlement_date)
    }

    /// Dirty price given a yield and settlement date
    fn dirty_price_from_yield(
        &self,
        y: Rate,
        daycounter: DayCounter,
        compounding: Compounding,
        frequency: Frequency,
        settlement_date: Date,
    ) -> Real {
        assert!(
            self.is_tradeable(settlement_date),
            "Non tradeable at {:?}, (maturity being {:?})",
            settlement_date,
            self.maturity_date()
        );
        let y = InterestRate::new(y, daycounter, compounding, frequency);
        bondfunctions::dirty_price(
            self.notional(settlement_date),
            self.cashflows(),
            &y,
            settlement_date,
        )
    }

    /// Returns whether this Bond still tradeable or not
    fn is_tradeable(&self, settlement_date: Date) -> bool {
        self.notional(settlement_date) != 0.0
    }

    /// Return the Bond issue date
    fn issue_date(&self) -> Date;

    /// Return the maturity date
    fn maturity_date(&self) -> Date;

    /// Return the notional schedule dates
    fn notional_schedule(&self) -> &Vec<Date>;

    fn notional(&self, date: Date) -> Real {
        let notional_schedule = self.notional_schedule();
        let last_notional_schedule = notional_schedule
            .last()
            .expect("Notional schedule is empty");
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
