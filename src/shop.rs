use crate::{card::Card, player::Player};

#[derive(Debug, Clone, Default)]
pub struct Shop {
    on_sale: Vec<(u32, Card)>,
    requested: Vec<(Player, Card)>,
}
