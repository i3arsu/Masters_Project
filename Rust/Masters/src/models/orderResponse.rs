use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponse {
    pub order_id: String,
    pub total_price: f64,
    pub discount_applied: bool,
    pub final_price: f64,
}