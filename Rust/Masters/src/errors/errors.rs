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

#[derive(Debug, Error)]
pub enum ItemError {
    #[error("DynamoDB service error")]
    ServiceError,

    #[error("Request to DynamoDB timed out")]
    TimeoutError,

    #[error("Error interacting with DynamoDB")]
    DynamoDbError,

    #[error("Failed to parse item data")]
    ParseError,

    #[error("Item not found")]
    NotFound,
}

// Implement the ResponseError trait for custom error handling in Actix
impl ResponseError for ItemError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ItemError::ServiceError => HttpResponse::InternalServerError().body("DynamoDB service error"),
            ItemError::TimeoutError => HttpResponse::GatewayTimeout().body("Request to DynamoDB timed out"),
            ItemError::DynamoDbError => {
                HttpResponse::InternalServerError().body("Error interacting with DynamoDB")
            }
            ItemError::ParseError => HttpResponse::BadRequest().body("Failed to parse item data"),
            ItemError::NotFound => HttpResponse::NotFound().body("Item not found"),
        }
    }
}

#[derive(Debug, Error)]
pub enum CouponError {

    #[error("Item not found")]
    NotFound,

    #[error("Failed to parse item data")]
    ParseError,

    #[error("Error interacting with DynamoDB")]
    DynamoDbError,

    #[error("Wrong attribute.")]
    MissingAttribute,
}

impl ResponseError for CouponError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CouponError::DynamoDbError => {
                HttpResponse::InternalServerError().body("Error interacting with DynamoDB")
            }
            CouponError::ParseError => HttpResponse::BadRequest().body("Failed to parse coupon data"),
            CouponError::NotFound => HttpResponse::NotFound().body("Coupon not found"),
            CouponError::MissingAttribute => HttpResponse::InternalServerError().body("Missing Attribute Error"),
        }
    }
}