use crate::calendar::date::Date;
use crate::{calendar::Calendar, matches::Match};

use crate::card::Card;
use crate::player::Player;
use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Data {
    calendar: Calendar,
    users: Vec<User>,
    players: Vec<Player>,
    cards: Vec<Card>,
    matches: Vec<Match>,
}

impl Data {
    pub fn new(date: Date) -> Self {
        Self {
            calendar: Calendar::new(date),
            users: Vec::new(),
            players: Vec::new(),
            cards: Vec::new(),
            matches: Vec::new(),
        }
    }

    #[allow(unused)]
    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    #[allow(unused)]
    pub fn remove_user(&mut self, user: &User) {
        self.users.retain(|u| u != user);
    }

    #[allow(unused)]
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    #[allow(unused)]
    pub fn remove_player(&mut self, player: &Player) {
        self.players.retain(|p| p != player);
    }

    #[allow(unused)]
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    #[allow(unused)]
    pub fn remove_card(&mut self, card: &Card) {
        self.cards.retain(|c| c != card);
    }

    #[allow(unused)]
    pub fn add_match(&mut self, match_: Match) {
        self.matches.push(match_);
    }

    #[allow(unused)]
    pub fn remove_match(&mut self, match_: &Match) {
        self.matches.retain(|m| m != match_);
    }

    #[allow(unused)]
    pub fn find_user<F>(&self, predicate: F) -> Option<&User>
    where
        F: Fn(&&User) -> bool,
    {
        self.users.iter().find(predicate)
    }

    #[allow(unused)]
    pub fn find_player<F>(&self, predicate: F) -> Option<&Player>
    where
        F: Fn(&&Player) -> bool,
    {
        self.players.iter().find(predicate)
    }

    #[allow(unused)]
    pub fn find_card<F>(&self, predicate: F) -> Option<&Card>
    where
        F: Fn(&&Card) -> bool,
    {
        self.cards.iter().find(predicate)
    }

    #[allow(unused)]
    pub fn find_match<F>(&self, predicate: F) -> Option<&Match>
    where
        F: Fn(&&Match) -> bool,
    {
        self.matches.iter().find(predicate)
    }

    #[allow(unused)]
    pub fn find_user_mut<F>(&mut self, predicate: F) -> Option<&mut User>
    where
        F: Fn(&&mut User) -> bool,
    {
        self.users.iter_mut().find(predicate)
    }

    #[allow(unused)]
    pub fn find_player_mut<F>(&mut self, predicate: F) -> Option<&mut Player>
    where
        F: Fn(&&mut Player) -> bool,
    {
        self.players.iter_mut().find(predicate)
    }

    #[allow(unused)]
    pub fn find_card_mut<F>(&mut self, predicate: F) -> Option<&mut Card>
    where
        F: Fn(&&mut Card) -> bool,
    {
        self.cards.iter_mut().find(predicate)
    }

    #[allow(unused)]
    pub fn find_match_mut<F>(&mut self, predicate: F) -> Option<&mut Match>
    where
        F: Fn(&&mut Match) -> bool,
    {
        self.matches.iter_mut().find(predicate)
    }

    #[allow(unused)]
    pub fn users(&self) -> &Vec<User> {
        &self.users
    }

    #[allow(unused)]
    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    #[allow(unused)]
    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    #[allow(unused)]
    pub fn matches(&self) -> &Vec<Match> {
        &self.matches
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            calendar: Calendar::default(),
            users: vec![User::admin("Tristan", "password")],
            players: Vec::new(),
            cards: Vec::new(),
            matches: Vec::new(),
        }
    }
}
