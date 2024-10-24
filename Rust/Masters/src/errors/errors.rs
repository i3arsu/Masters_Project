use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use std::fmt;

#[derive(Debug, Error)]
pub enum OrderProcessingError {
    #[error("Failed to fetch items from the database")]
    ItemFetchError,

    #[error("Invalid or expired coupon code")]
    InvalidCouponError,

    #[error("Failed to parse coupon discount")]
    CouponParseError,

    #[error("Coupon not applicable to any items in the cart")]
    CouponNotApplicable,
}

// Implement the ResponseError trait for custom error handling in Actix
impl ResponseError for OrderProcessingError {
    fn error_response(&self) -> HttpResponse {
        match self {
            OrderProcessingError::ItemFetchError => {
                HttpResponse::InternalServerError().body("Failed to fetch items from the database")
            }
            OrderProcessingError::InvalidCouponError => {
                HttpResponse::BadRequest().body("Invalid or expired coupon code")
            }
            OrderProcessingError::CouponParseError => {
                HttpResponse::BadRequest().body("Failed to parse coupon discount")
            }
            OrderProcessingError::CouponNotApplicable => {
                HttpResponse::BadRequest().body("Coupon not applicable to any items in the cart")
            }
        }
    }
}