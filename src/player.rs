#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Defense,
    Goal,
    Middle,
    Frontline,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    role: Role,
}

impl Player {
    pub fn new(name: String, role: Role) -> Self {
        Self { name, role }
    }
}
