use aws_sdk_dynamodb::{Client, Error};
use aws_sdk_dynamodb::types::AttributeValue;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::coupon::Coupon;
use crate::db::dynamodb::get_dynamodb_client;
use actix_web::HttpResponse;

#[derive(Debug)]
pub enum CouponError {
    NotFound(String),
    ParseError(String),
    // Add other error variants as needed
}

pub async fn get_coupon_by_code(code: &str) -> Result<Coupon, CouponError> {
    let client = get_dynamodb_client().await;

    let request = client
        .get_item()
        .table_name("Coupon")
        .key("code", AttributeValue::S(code.to_string()))
        .send()
        .await.map_err(|e| CouponError::ParseError(format!("Failed to get item from DynamoDB: {}", e)))?;

    if let Some(item) = request.item {
        let discount = item.get("discount_percentage")
            .ok_or_else(|| CouponError::ParseError("Discount not found".to_string()))?
            .as_n()
            .map_err(|_| CouponError::ParseError("Failed to parse discount".to_string()))?
            .parse::<u32>()
            .map_err(|_| CouponError::ParseError("Failed to parse discount to u32".to_string()))?;

        let applicable_items = item.get("applicable_items").and_then(|v| {
            v.as_l().ok().map(|list| {
                list.iter()
                    .filter_map(|val| val.as_s().ok().map(|s| s.to_string()))
                    .collect::<Vec<String>>()
            })
        });

        let expires_at = item.get("expires_at").and_then(|v| {
            v.as_s().ok().map(|s| s.to_string())
        });

        let coupon = Coupon {
            code: item.get("code")
                .ok_or_else(|| CouponError::ParseError("Coupon code not found".to_string()))?
                .as_s()
                .map_err(|_| CouponError::ParseError("Failed to parse coupon code".to_string()))?
                .to_string(),
            discount,
            applicable_items,
            expires_at,
        };
        Ok(coupon)
    } else {
        Err(CouponError::NotFound("Coupon not found".to_string()))
    }
}
