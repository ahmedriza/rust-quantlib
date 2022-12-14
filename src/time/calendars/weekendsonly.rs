use std::sync::Arc;

use crate::time::{calendar::{Calendar, Holiday, Weekend, WesternWeekend}, date::Date, weekday::Weekday};

/// Weekends-only calendar
///
/// This calendar has no bank holidays except for weekends (Saturdays and Sundays) as required
/// by ISDA for calculating conventional CDS spreads.
#[derive(Clone)]
pub struct WeekendsOnly {
    pub weekend: Arc<dyn Weekend>,
}

impl WeekendsOnly {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Calendar {
        Calendar::new(Arc::new(WeekendsOnly {
            weekend: Arc::new(WesternWeekend {}),
        }))
    }
}

impl Holiday for WeekendsOnly {
    fn name(&self) -> String {
        "weekends only".into()
    }

    fn is_business_day(&self, date: &Date) -> bool {
        !self.is_weekend(date.weekday())
    }

    fn is_weekend(&self, weekday: Weekday) -> bool {
        self.weekend.is_weekend(weekday)
    }
}
