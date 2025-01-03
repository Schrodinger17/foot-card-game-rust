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
    pub fn new(date: Date) -> Self {
        Self {
            now: date,
            years: vec![Year::new(date.year())],
        }
    }
}
