use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletedOrder {
    pub order_id: String,
    pub items: Vec<String>,
    pub total_price: f64,
    pub discount_applied: bool,
    pub final_price: f64,
    pub status: String,
}