use std::rc::Rc;

use crate::datetime::SerialNumber;
use crate::datetime::date::Date;
use crate::datetime::daycounter::DayCounter;
use crate::datetime::frequency::Frequency;
use crate::datetime::period::Period;
use crate::datetime::timeunit::TimeUnit;
use crate::maths::solvers1d::solver1d::Solver1D;
use crate::rates::compounding::Compounding;
use crate::rates::interestrate::InterestRate;
use crate::types::{Rate, Real, Size, Time};

use super::coupon::{Coupon, CouponLeg};
use super::irrfinder::IrrFinder;

/// Sequence of cashflows
pub type CashFlowLeg = Vec<Rc<dyn CashFlow>>;

pub trait CashFlow {
    /// Accrued amount at the given date
    fn accrued_amount(&self, date: Date) -> Real;
    
    /// Returns the amount of the cash flow. The amount is not discounted, i.e., it is the
    /// actual amount paid at the cash flow date.
    fn amount(&self) -> Real;

    /// Returns the date at which the cashflow occurs
    fn date(&self) -> Date;

    /// Returns the date that the cash flow trades ex-coupon
    fn ex_coupon_date(&self) -> Date {
        Date::default()
    }

    /// Returns true if a cashflow has already occurred before a date.
    fn has_occurred(&self, ref_date: &Date, include_ref_date: bool) -> bool {
        // easy and quick handling of most cases
        if ref_date != &Date::default() {
            let cf = self.date();
            if ref_date < &cf {
                return false;
            }
            if &cf < ref_date {
                return true;
            }
        }
        // TODO check whether to override include_ref_date
        if include_ref_date {
            &self.date() < ref_date
        } else {
            &self.date() <= ref_date
        }
    }

    /// Returns true if the cashflow is trading ex-coupon on the `ref_date`.
    fn trading_ex_coupon(&self, ref_date: Date) -> bool {
        let ecd = self.ex_coupon_date();
        if ecd == Date::default() {
            return false;
        }
        ecd <= ref_date
    }
}

impl<T> CashFlow for Rc<T>
where
    // Note that `?Sized` is needed here as otherwise the compiler will default to `Sized`
    // which will then forbid a type such as `Rc<dyn CashFlow>`
    T: CashFlow + ?Sized,
{
    fn accrued_amount(&self, settlement_date: Date) -> Real {
        (**self).accrued_amount(settlement_date)
    }    
    
    fn amount(&self) -> Real {
        (**self).amount()
    }

    fn date(&self) -> Date {
        (**self).date()
    }
}

// -------------------------------------------------------------------------------------------------

pub fn accurued_amount<T: CashFlow>(
    leg: &Vec<T>,
    include_settlement_date_flows: bool,
    settlement_date: Date,
) -> Real {
    let mut result = 0.0;
    if let Some(mut i) = __next_cashflow(leg, include_settlement_date_flows, settlement_date) {
        let payment_date = leg[i].date();
        while i < leg.len() {
            let cf = &leg[i];
            if cf.date() == payment_date {
                result += cf.accrued_amount(settlement_date);
            }
            i += 1;
        }
    }
    result
}

pub fn accurued_days<T: Coupon>(
    leg: &Vec<T>,
    include_settlement_date_flows: bool,
    settlement_date: Date,
) -> SerialNumber {
    if let Some(mut i) = __next_cashflow(leg, include_settlement_date_flows, settlement_date) {
        let payment_date = leg[i].date();
        while i < leg.len() {
            let cf = &leg[i];
            if cf.date() == payment_date {
                return cf.accrued_days(settlement_date);
            }
            i += 1;
        }
    }
    0
}

pub fn accrued_period(
    leg: &CouponLeg,
    include_settlement_date_flows: bool,
    settlement_date: Date,
) -> Time {
    if let Some(mut i) = next_cashflow(leg, include_settlement_date_flows, settlement_date) {
        let payment_date = leg[i].date();
        while i < leg.len() {
            let cf = &leg[i];
            if cf.date() == payment_date {
                return cf.accrued_period(settlement_date);
            }
            i += 1;
        }
    }
    0.0
}

/// Implied internal rate of return.
/// The function verifies the theoretical existence of an IRR and numerically establishes the IRR
/// to the desired precision.
#[allow(clippy::too_many_arguments)]
pub fn bond_yield(
    solver: &impl Solver1D,
    cashflows: &CashFlowLeg,
    npv: Real,
    daycounter: DayCounter,
    compounding: Compounding,
    frequency: Frequency,
    include_settlement_date_flows: bool,
    settlement_date: Date,
    npv_date: Date,
    accuracy: Real,
    guess: Rate,
) -> Rate {
    let irr_finder = IrrFinder::new(
        cashflows,
        npv,
        daycounter,
        compounding,
        frequency,
        include_settlement_date_flows,
        settlement_date,
        npv_date,
    );

    solver.solve(
        |y| irr_finder.at(y),
        |y| irr_finder.derivative(y),
        accuracy,
        guess,
        guess / 10.0,
    )
}

pub fn __next_cashflow<T: CashFlow>(
    leg: &Vec<T>,
    include_settlement_date_flows: bool,
    settlement_date: Date,
) -> Option<Size> {
    for (index, cf) in leg.iter().enumerate() {
        if !cf.has_occurred(&settlement_date, include_settlement_date_flows) {
            return Some(index);
        }
    }
    None
}

/// Return `Some(index)` where `index` is index of first cash flow in the [Leg] if there are
/// cash flows.  Otherwise return `None`.
pub fn next_cashflow(
    leg: &CouponLeg,
    include_settlement_date_flows: bool,
    settlement_date: Date,
) -> Option<Size> {
    for (index, cf) in leg.iter().enumerate() {
        if !cf.has_occurred(&settlement_date, include_settlement_date_flows) {
            return Some(index);
        }
    }
    None
}

/// NPV of the cash flows.
/// The NPV is the sum of the cash flows, each discounted according to the given term structure.
pub fn npv(
    cashflows: &CashFlowLeg,
    interestrate: &InterestRate,
    include_settlement_date_flows: bool,
    settlement_date: Date,
    npv_date: Date,
) -> Real {
    if cashflows.is_empty() {
        return 0.0;
    }
    let npv_date = if npv_date == Date::default() {
        settlement_date
    } else {
        npv_date
    };
    let mut npv = 0.0;
    let mut discount = 1.0;
    let mut last_date = npv_date;

    let daycounter = &interestrate.daycounter;
    for cf in cashflows {
        if cf.has_occurred(&settlement_date, include_settlement_date_flows) {
            continue;
        }
        let mut amount = cf.amount();
        if cf.trading_ex_coupon(settlement_date) {
            amount = 0.0;
        }
        let t = get_stepwise_discount_time(cf, daycounter, npv_date, last_date);
        let b = interestrate.discount_factor(t);

        discount *= b;
        last_date = cf.date();
        npv += amount * discount;
    }
    npv
}

/// Calculate Time-To-Discount for each stage when calculating discount factor stepwisely
pub fn get_stepwise_discount_time(
    cashflow: &Rc<dyn CashFlow>,
    daycounter: &DayCounter,
    npv_date: Date,
    last_date: Date,
) -> Time {
    let cashflow_date = cashflow.date();
    // TODO
    // get ref_start_date and ref_end_date from Coupon
    let ref_start_date = if last_date == npv_date {
        // we don't have a previous coupon date, so we fake it
        cashflow_date - Period::new(1, TimeUnit::Years)
    } else {
        last_date
    };
    let ref_end_date = cashflow_date;
    // TODO handle coupon
    daycounter.year_fraction(&last_date, &cashflow_date, &ref_start_date, &ref_end_date)
}

///
/// Calculate the modified duration which is defined as
///
/// D_modified = (−1/P)(∂P/∂y)
///
/// where `P` is the present value of the cash flows according to the given IRR `y`.
pub fn modified_duration(
    cashflows: &CashFlowLeg,
    y: &InterestRate,
    include_settlement_date_flows: bool,
    settlement_date: Date,
    npv_date: Date,
) -> Real {
    if cashflows.is_empty() {
        return 0.0;
    }

    let npv_date = if npv_date == Date::default() {
        settlement_date
    } else {
        npv_date
    };

    let mut p: Real = 0.0;
    let mut t: Time = 0.0;
    let mut dpdy: Real = 0.0;
    let r: Rate = y.rate;
    let n = y.frequency;
    let mut last_date = npv_date;
    let daycounter = &y.daycounter;

    for cf in cashflows {
        if cf.has_occurred(&settlement_date, include_settlement_date_flows) {
            continue;
        }
        let mut c = cf.amount();
        if cf.trading_ex_coupon(settlement_date) {
            c = 0.0;
        }
        t += get_stepwise_discount_time(cf, daycounter, npv_date, last_date);
        let discount_factor = y.discount_factor(t);
        p += c * discount_factor;
        match y.compounding {
            Compounding::Simple => dpdy -= c * discount_factor * discount_factor * t,
            Compounding::Compounded => dpdy -= c * t * discount_factor / (1.0 + r / n),
            Compounding::Continuous => dpdy -= c * discount_factor * t,
            Compounding::SimpleThenCompounded => {
                if t <= 1.0 / n {
                    dpdy -= c * discount_factor * discount_factor * t;
                } else {
                    dpdy -= c * t * discount_factor / (1.0 + r / n);
                }
            }
            Compounding::CompoundedThenSimple => {
                if t > 1.0 / n {
                    dpdy -= c * discount_factor * discount_factor * t;
                } else {
                    dpdy -= c * t * discount_factor / (1.0 + r / n);
                }
            }
        }
        last_date = cf.date();
    }
    if p == 0.0 {
        // no cash flows
        return 0.0;
    }
    -dpdy / p // reverse derivative sign
}
