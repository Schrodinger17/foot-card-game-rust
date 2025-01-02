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
    pub fn new(year: u32, week: u32) -> Self {
        Self {
            calendar: Calendar::new(year, week),
            users: Vec::new(),
            players: Vec::new(),
            cards: Vec::new(),
            matches: Vec::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn remove_user(&mut self, user: &User) {
        self.users.retain(|u| u != user);
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn remove_player(&mut self, player: &Player) {
        self.players.retain(|p| p != player);
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn remove_card(&mut self, card: &Card) {
        self.cards.retain(|c| c != card);
    }

    pub fn add_match(&mut self, match_: Match) {
        self.matches.push(match_);
    }

    pub fn remove_match(&mut self, match_: &Match) {
        self.matches.retain(|m| m != match_);
    }

    pub fn find_user<F>(&self, predicate: F) -> Option<&User>
    where
        F: Fn(&&User) -> bool,
    {
        self.users.iter().find(predicate)
    }

    pub fn find_player<F>(&self, predicate: F) -> Option<&Player>
    where
        F: Fn(&&Player) -> bool,
    {
        self.players.iter().find(predicate)
    }

    pub fn find_card<F>(&self, predicate: F) -> Option<&Card>
    where
        F: Fn(&&Card) -> bool,
    {
        self.cards.iter().find(predicate)
    }

    pub fn find_match<F>(&self, predicate: F) -> Option<&Match>
    where
        F: Fn(&&Match) -> bool,
    {
        self.matches.iter().find(predicate)
    }

    pub fn find_user_mut<F>(&mut self, predicate: F) -> Option<&mut User>
    where
        F: Fn(&&mut User) -> bool,
    {
        self.users.iter_mut().find(predicate)
    }

    pub fn find_player_mut<F>(&mut self, predicate: F) -> Option<&mut Player>
    where
        F: Fn(&&mut Player) -> bool,
    {
        self.players.iter_mut().find(predicate)
    }

    pub fn find_card_mut<F>(&mut self, predicate: F) -> Option<&mut Card>
    where
        F: Fn(&&mut Card) -> bool,
    {
        self.cards.iter_mut().find(predicate)
    }

    pub fn find_match_mut<F>(&mut self, predicate: F) -> Option<&mut Match>
    where
        F: Fn(&&mut Match) -> bool,
    {
        self.matches.iter_mut().find(predicate)
    }

    pub fn users(&self) -> &Vec<User> {
        &self.users
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

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
