#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TimeUnit {
    Days,
    Weeks,
    Months,
    Years,
    Hours,
    Minutes,
    Seconds,
    Milliseconds,
    Microseconds,
}
