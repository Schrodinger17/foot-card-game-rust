use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

use crate::id_manager;
use crate::id_manager::IdManager;
use crate::player::Player;
use crate::player::Role;
use crate::user::User;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    player: Player,
    rarity: Rarity,
    id: u32,
    owner: Option<User>,
    id_manager: Rc<RefCell<IdManager>>,
}

impl Card {
    pub fn new(player: Player, rarity: Rarity, max_copies: u32) -> Self {
        Self {
            player,
            rarity,
            id: 1,
            owner: None,
            id_manager: Rc::new(RefCell::new(IdManager::new(max_copies))),
        }
    }
}

impl Iterator for Card {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.id_manager.as_ref().borrow_mut().next()?;
        Some(Card {
            player: self.player.clone(),
            rarity: self.rarity,
            id,
            owner: None,
            id_manager: self.id_manager.clone(),
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_card_new() {
        let player = Player::new("Player 1".to_owned(), Role::Defense);
        let card = Card::new(player, Rarity::Common, 5);

        assert_eq!(card.player.name, "Player 1");
        assert_eq!(card.rarity, Rarity::Common);
        assert_eq!(card.id_manager.borrow().get_max_id(), 5);
    }

    #[test]
    fn test_card_iterator() {
        let player = Player::new("Player 1".to_owned(), Role::Defense);
        let card = Card::new(player, Rarity::Common, 5);

        let card_iter = card.id_manager.clone();

        assert_eq!(card_iter.as_ref().borrow_mut().next(), Some(1));
        assert_eq!(card_iter.as_ref().borrow_mut().next(), Some(2));
        assert_eq!(card_iter.as_ref().borrow_mut().next(), Some(3));
        assert_eq!(card_iter.as_ref().borrow_mut().next(), Some(4));
        assert_eq!(card_iter.as_ref().borrow_mut().next(), Some(5));
        assert_eq!(card_iter.as_ref().borrow_mut().next(), None);
    }

    #[test]
    fn test_card_iterator2() {
        let player = Player::new("Player 1".to_owned(), Role::Defense);
        let card = Card::new(player, Rarity::Common, 5);

        assert_eq!(card.id_manager.as_ref().borrow_mut().next(), Some(1));
        assert_eq!(card.id_manager.as_ref().borrow_mut().next(), Some(2));
        assert_eq!(card.id_manager.as_ref().borrow_mut().next(), Some(3));
        assert_eq!(card.id_manager.as_ref().borrow_mut().next(), Some(4));
        assert_eq!(card.id_manager.as_ref().borrow_mut().next(), Some(5));
        assert_eq!(card.id_manager.as_ref().borrow_mut().next(), None);
    }
}
