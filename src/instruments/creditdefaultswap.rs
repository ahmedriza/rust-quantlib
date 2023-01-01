use crate::datetime::{
    date::Date, dategenerationrule::DateGenerationRule, months::Month::*, period::Period,
    schedule::previous_twentieth, timeunit::TimeUnit::*,
};

// CDS Instrument
pub struct CreditDefaultSwap {}

impl CreditDefaultSwap {
    // TODO
}

pub fn cds_maturity(trade_date: &Date, tenor: Period, rule: DateGenerationRule) -> Date {
    assert!(
        rule == DateGenerationRule::CDS2015
            || rule == DateGenerationRule::CDS
            || rule == DateGenerationRule::OldCDS,
        "cds_maturity should only be used with date generation rule CDS2015, CDS or OldCDS"
    );

    assert!(
        tenor.unit == Years || (tenor.unit == Months && tenor.length % 3 == 0),
        "cds_maturity expects a tenor that is a multiple of 3 months"
    );

    if rule == DateGenerationRule::OldCDS {
        assert!(
            tenor != Period::new(0, Months),
            "A tenor of 0M is not supported for OldCDS"
        );
    }

    let mut anchor_date = previous_twentieth(trade_date, rule);
    if rule == DateGenerationRule::CDS2015
        && (anchor_date == Date::new(20, December, anchor_date.year())
            || anchor_date == Date::new(20, June, anchor_date.year()))
    {
        if tenor.length == 0 {
            return Date::default();
        } else {
            anchor_date -= Period::new(3, Months);
        }
    }

    let maturity = anchor_date + tenor + Period::new(3, Months);
    assert!(
        maturity > *trade_date,
        "Error calculating CDS maturity. Tenor is {:?}, trade date is {:?}, \
             generating a maturity of {:?} <= trade date",
        tenor,
        trade_date,
        maturity
    );

    maturity
}
