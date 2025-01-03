use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    Defense,
    Goal,
    Middle,
    Frontline,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    id: u32,
    role: Role,
}

impl Player {
    #[allow(unused)]
    pub fn new(name: String, role: Role) -> Self {
        Self {
            name,
            id: Uuid::new_v4().as_u128() as u32,
            role,
        }
    }
}
