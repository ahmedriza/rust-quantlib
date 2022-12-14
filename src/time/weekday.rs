use crate::types::Integer;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum Weekday {
    Sunday = 1,
    Monday = 2,
    Tuesday = 3,
    Wednesday = 4,
    Thursday = 5,
    Friday = 6,
    Saturday = 7,
}

impl From<Integer> for Weekday {
    fn from(n: Integer) -> Self {
        match n {
            0 => Self::Saturday,
            1 => Self::Sunday,
            2 => Self::Monday,
            3 => Self::Tuesday,
            4 => Self::Wednesday,
            5 => Self::Thursday,
            6 => Self::Friday,
            7 => Self::Saturday,
            other => panic!("Invalid weekday number {}", other),
        }
    }
}

impl From<Weekday> for Integer {
    fn from(wd: Weekday) -> Self {
        wd as Integer
    }
}
