use crate::matches::Match;
use crate::player::Player;

#[derive(Debug, Clone, Default)]
pub struct Week {
    week: u32,
    active_players: Vec<Player>,
    matches: Vec<Match>,
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
