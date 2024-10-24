use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItem {
    pub item_id: String,
    pub quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRequest {
    pub items: Vec<OrderItem>,
    pub coupon_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponse {
    pub order_id: String,
    pub total_price: f64,
    pub discount_applied: bool,
    pub final_price: f64,
}