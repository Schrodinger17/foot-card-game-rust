use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rarity {
    #[default]
    Common,
    Rare,
    Epic,
}

impl Rarity {
    #[allow(unused)]
    pub fn get_max_copies(&self) -> u32 {
        match self {
            Rarity::Common => 5,
            Rarity::Rare => 3,
            Rarity::Epic => 1,
        }
    }

    #[allow(unused)]
    pub fn get_all() -> Vec<Rarity> {
        vec![Rarity::Common, Rarity::Rare, Rarity::Epic]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    player_id: u32,
    rarity: Rarity,
    id: u32,
}

impl Card {
    #[allow(unused)]
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

    #[test]
    fn test_rarity_get_max_copies() {
        assert_eq!(Rarity::Common.get_max_copies(), 5);
        assert_eq!(Rarity::Rare.get_max_copies(), 3);
        assert_eq!(Rarity::Epic.get_max_copies(), 1);
    }

    #[test]
    fn test_default_rarity() {
        let rarity = Rarity::default();
        assert_eq!(rarity, Rarity::Common);
    }

    #[test]
    fn test_rarity_get_all() {
        let rarities = Rarity::get_all();
        assert_eq!(rarities.len(), 3);
        assert!(rarities.contains(&Rarity::Common));
        assert!(rarities.contains(&Rarity::Rare));
        assert!(rarities.contains(&Rarity::Epic));
    }
}
