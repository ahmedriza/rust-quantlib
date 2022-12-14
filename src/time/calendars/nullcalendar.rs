use std::sync::Arc;

use crate::time::{
    calendar::{Calendar, Holiday, Weekend},
    date::Date,
    weekday::Weekday,
};

/// Calendar for reproducing theoretical calculations.
/// This calendar has no holidays. It ensures that dates at whole-month distances have the same
/// day of month.
#[derive(Clone)]
pub struct NullCalendar {
    pub weekend: Arc<dyn Weekend>,
}

impl NullCalendar {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Calendar {
        Calendar::new(Arc::new(NullCalendar {
            weekend: Arc::new(NullWeekend {}),
        }))
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct NullWeekend {}

impl Weekend for NullWeekend {
    fn is_weekend(&self, _weekday: Weekday) -> bool {
        false
    }
}

// -------------------------------------------------------------------------------------------------

impl Holiday for NullCalendar {
    fn name(&self) -> String {
        "Null".into()
    }

    fn is_business_day(&self, _date: &Date) -> bool {
        true
    }

    fn is_weekend(&self, weekday: Weekday) -> bool {
        self.weekend.is_weekend(weekday)
    }
}
