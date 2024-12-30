mod week;
mod year;

use year::Year;

#[derive(Debug, Clone, Default)]
pub struct Calendar {
    year: u32,
    week: u32,
    years: Vec<Year>,
}

impl Calendar {
    pub fn new(year: u32, week: u32) -> Self {
        assert!(week < 52, "Week must be less than 52");
        Self {
            year,
            week,
            years: vec![Year::new(year)],
        }
    }
}
