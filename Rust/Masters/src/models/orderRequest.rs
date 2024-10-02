use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRequest {
    pub order_id: String,
    pub items: Vec<String>,
    pub coupon_code: Option<String>,
}