use serde::{Deserialize, Serialize};

use crate::{card::Card, id::Id};

static STARTING_MONEY: u32 = 10000;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum UserRole {
    #[default]
    Standard,
    Admin,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    id: u32,
    password: Option<String>,
    role: UserRole,
    cards: Vec<Card>,
    money: u32,
}

impl User {
    pub fn new(username: &str, password: Option<&str>) -> Self {
        Self {
            username: username.to_owned(),
            id: Id::new_u32(),
            password: password.map(|password| password.to_owned()),
            role: UserRole::Standard,
            cards: Vec::new(),
            money: STARTING_MONEY,
        }
    }
    pub fn admin(username: &str, password: &str) -> Self {
        Self {
            username: username.to_owned(),
            id: Id::new_u32(),
            password: Some(password.to_owned()),
            role: UserRole::Admin,
            cards: Vec::new(),
            money: u32::MAX,
        }
    }
    pub fn check_password(&self, password: &str) -> bool {
        self.password.as_ref().map_or(true, |p| p == password)
    }
    pub fn has_password(&self) -> bool {
        self.password.is_some()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_password() {
        let user1 = User::new("user1", Some("password"));
        assert!(user1.has_password());

        let user1 = User::new("user1", None);
        assert!(!user1.has_password());
    }

    #[test]
    fn test_check_password() {
        let user1 = User::new("user1", Some("password"));
        assert!(user1.check_password("password"));
    }
}
