use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Date {
    year: u32,
    week: u32,
}

impl Date {
    pub fn new(year: u32, week: u32) -> Date {
        assert!(week < 52, "Week must be less than 52");
        Date { year, week }
    }

    #[allow(unused)]
    pub fn year(&self) -> u32 {
        self.year
    }

    #[allow(unused)]
    pub fn week(&self) -> u32 {
        self.week
    }

    #[allow(unused)]
    pub fn next(&self) -> Date {
        match (self.year, self.week) {
            (year, 52) => Date {
                year: year + 1,
                week: 0,
            },
            (year, week) => Date {
                year,
                week: week + 1,
            },
        }
    }

    #[allow(unused)]
    pub fn prev(&self) -> Date {
        match (self.year, self.week) {
            (0, 0) => panic!("Can't go before J.C. !"),
            (year, 0) => Date {
                year: year - 1,
                week: 52,
            },
            (year, week) => Date {
                year,
                week: week - 1,
            },
        }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Year: {}, Week: {}", self.year, self.week)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_date_next() {
        let date = Date::new(2021, 51);
        let next_date = date.next();

        assert_eq!(next_date.year(), 2021);
        assert_eq!(next_date.week(), 52);

        let next_date = next_date.next();

        assert_eq!(next_date.year(), 2022);
        assert_eq!(next_date.week(), 0);
    }

    #[test]
    fn test_date_prev() {
        let date = Date::new(2021, 0);
        let prev_date = date.prev();

        assert_eq!(prev_date.year(), 2020);
        assert_eq!(prev_date.week(), 52);

        let prev_date = prev_date.prev();

        assert_eq!(prev_date.year(), 2020);
        assert_eq!(prev_date.week(), 51);
    }
}
