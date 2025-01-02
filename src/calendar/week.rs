use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Week {
    week: u32,
    active_players: Vec<u32>,
    matches: Vec<u32>,
}

impl Week {
    pub fn new(week: u32) -> Self {
        Self {
            week,
            active_players: Vec::new(),
            matches: Vec::new(),
        }
    }
}
