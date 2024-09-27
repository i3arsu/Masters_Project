use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Coupon {
    pub code: String,
    pub discount: u32,
    pub is_redeemed: bool,
    pub expiration_date: String,
}
