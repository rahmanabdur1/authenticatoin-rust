use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub id: usize,
    pub name: String,
    pub quantity: u32,
}
