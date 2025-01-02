use serde::{Deserialize, Serialize};

use super::week::Week;

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Year {
    year: u32,
    weeks: Vec<Week>,
}

impl Year {
    pub fn new(year: u32) -> Self {
        let weeks = (0..52).map(Week::new).collect();
        Self { year, weeks }
    }
}
