use crate::card::Card;
use crate::player::Player;
use crate::user::User;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data {
    pub users: Vec<User>,
    players: Vec<Player>,
    cards: Vec<Card>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
            players: Vec::new(),
            cards: Vec::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn get_user(&self, username: &str) -> Option<&User> {
        self.users.iter().find(|user| user.username == username)
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            users: vec![User::admin("Tristan", "password")],
            players: Vec::new(),
            cards: Vec::new(),
        }
    }
}
