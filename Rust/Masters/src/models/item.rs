use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub item_id: String,
    pub name: String,
    pub price: f64,
}