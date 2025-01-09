use aws_sdk_dynamodb::{Client, Error};
use aws_sdk_dynamodb::types::AttributeValue;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::coupon::Coupon;
use crate::db::dynamodb::get_dynamodb_client;
use crate::errors::errors::CouponError;
use actix_web::{web, HttpResponse};


pub async fn get_coupon_by_code(code: &str) -> Result<Coupon, CouponError> {
    let client = get_dynamodb_client().await;

    let request = client
        .get_item()
        .table_name("Coupon")
        .key("code", AttributeValue::S(code.to_string()))
        .send()
        .await.map_err(|e| CouponError::ParseError)?;

    if let Some(item) = request.item {
        let discount = item.get("discount_percentage")
            .ok_or_else(|| CouponError::ParseError)?
            .as_n()
            .map_err(|_| CouponError::ParseError)?
            .parse::<u32>()
            .map_err(|_| CouponError::ParseError)?;

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
                .ok_or_else(|| CouponError::ParseError)?
                .as_s()
                .map_err(|_| CouponError::ParseError)?
                .to_string(),
            discount,
            applicable_items,
            expires_at,
        };
        Ok(coupon)
    } else {
        Err(CouponError::NotFound)
    }
}

pub async fn create_coupon(coupon: web::Json<Coupon>) -> Result<String, CouponError> {
    let client = get_dynamodb_client().await;

    let new_coupon = coupon.into_inner();

    // Perform the put_item operation to insert the coupon into the table
    let mut request = client
        .put_item()
        .table_name("Coupon")
        .item("code", AttributeValue::S(new_coupon.code.clone()))
        .item("discount_percentage", AttributeValue::N(new_coupon.discount.to_string()));

    if let Some(applicable_items) = &new_coupon.applicable_items {
        let items = applicable_items.iter()
            .map(|item| AttributeValue::S(item.clone()))
            .collect::<Vec<AttributeValue>>();
        request = request.item("applicable_items", AttributeValue::L(items));
    }

    if let Some(expires_at) = &new_coupon.expires_at {
        request = request.item("expires_at", AttributeValue::S(expires_at.clone()));
    }

    request
        .send()
        .await
        .map_err(|err| CouponError::DynamoDbError)?;

    Ok("Coupon created successfully.".to_string())
}

pub async fn get_all_coupons() -> Result<Vec<Coupon>, CouponError> {
    let client = get_dynamodb_client().await;

    let request = client
        .scan()
        .table_name("Coupon")
        .send()
        .await
        .map_err(|err| CouponError::DynamoDbError)?;

    let items = request.items.unwrap_or_default();

    let fetched_coupons: Result<Vec<Coupon>, CouponError> = items.into_iter().map(|item| {
        let code = item.get("code")
            .and_then(|v| v.as_s().ok())
            .ok_or(CouponError::MissingAttribute)?
            .to_string();

            let discount = match item.get("discount") {
                Some(attr) => attr.as_n()
                    .map_err(|_| CouponError::MissingAttribute)?
                    .parse::<u32>()
                    .map_err(|_| CouponError::ParseError)?,
                None => {
                    eprintln!("Warning: 'discount' attribute missing for coupon with code '{}'. Defaulting to 0.", code);
                    0
                }
            };

        let applicable_items = item.get("applicable_items")
            .and_then(|v| v.as_l().ok())
            .map(|attr_list| {
                attr_list.iter().filter_map(|attr| attr.as_s().ok().map(String::from)).collect()
            });

        let expires_at = item.get("expires_at")
            .and_then(|v| v.as_s().ok())
            .map(String::from);

        Ok(Coupon { code, discount, applicable_items, expires_at })
    }).collect();

    fetched_coupons
}

pub async fn delete_coupon_by_code(code: &str) -> Result<(), CouponError> {
    let client = get_dynamodb_client().await;

    let request = client
        .delete_item()
        .table_name("Coupon")
        .key("code", AttributeValue::S(code.to_string()))
        .send()
        .await;

    match request {
        Ok(_) => Ok(()),
        Err(_) => Err(CouponError::DynamoDbError),
    }
}

