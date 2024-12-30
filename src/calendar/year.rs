use super::week::{self, Week};

#[derive(Debug, Clone, Default)]
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
