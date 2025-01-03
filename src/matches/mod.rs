mod team;

use team::Team;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Match {
    id: u32,
    team_a: Team,
    team_b: Team,
    score_a: u32,
    score_b: u32,
}
