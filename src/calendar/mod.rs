use serde::{Deserialize, Serialize};

pub mod date;
pub mod week;
pub mod year;

use date::Date;
use year::Year;

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Calendar {
    now: Date,
    years: Vec<Year>,
}

impl Calendar {
    pub fn new(year: u32, week: u32) -> Self {
        assert!(week < 52, "Week must be less than 52");
        Self {
            now: Date::new(year, week),
            years: vec![Year::new(year)],
        }
    }
}
