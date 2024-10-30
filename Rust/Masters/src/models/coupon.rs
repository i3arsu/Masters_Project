use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Coupon {
    pub code: String,
    pub discount: u32,
    pub applicable_items: Option<Vec<String>>,
    pub expires_at: Option<String>,
}
