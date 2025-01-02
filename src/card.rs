use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
}

impl Rarity {
    pub fn get_max_copies(&self) -> u32 {
        match self {
            Rarity::Common => 5,
            Rarity::Rare => 3,
            Rarity::Epic => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    player_id: u32,
    rarity: Rarity,
    id: u32,
}

impl Card {
    pub fn new(rarity: Rarity, player_id: u32) -> Self {
        Self {
            player_id,
            rarity,
            id: 1,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_card_new() {
        let card = Card::new(Rarity::Common, 0);

        assert_eq!(card.rarity, Rarity::Common);
        assert_eq!(card.player_id, 0);
    }
}
