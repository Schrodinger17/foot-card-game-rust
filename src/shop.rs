#[derive(Debug, Clone, Default)]
#[allow(unused)]
pub struct Shop {
    on_sale: Vec<(u32, u32)>,
    requested: Vec<(u32, u32)>,
}
