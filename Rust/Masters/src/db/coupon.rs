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
        .table_name("Coupons")
        .key("code", AttributeValue::S(code.to_string()))
        .send()
        .await.map_err(|e| CouponError::ParseError(format!("Failed to get item from DynamoDB: {}", e)))?;

    if let Some(item) = request.item {
        let discount = item.get("discount")
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



// pub async fn calculate_price(order: &models::orderRequest::OrderRequest, coupon: Option<&models::coupon::Coupon>) -> f64 {
//     if coupon.is_none() {
//         return order.items.iter().map(|item| item.price).sum();
//     }

//     let coupon = coupon.unwrap();

//     if let Some(expires_at) = &coupon.expiration_date {
//         if expires_at < &chrono::Utc::now().to_string() {
//             panic!("Coupon has expired");
//         }
//     }

//     let mut discount = Decimal::ZERO;
//     let total_price = order.items.iter().map(|item| item.price).sum::<f64>();

//     // Calculate the discount
//     if let Some(applicable_items) = &coupon.applicable_items {

//         for item in &order.items {
//             if applicable_items.contains(&item.id) {
//                 discount += Decimal::from_f64(item.price).unwrap() * (Decimal::from_f64(coupon.discount_percentage / 100.0).unwrap());
//             }
//         }
//     } else {

//         discount = Decimal::from_f64(total_price).unwrap() * (Decimal::from_f64(coupon.discount_percentage / 100.0).unwrap());
//     }

//     let final_price = (Decimal::from_f64(total_price).unwrap() - discount).max(Decimal::ZERO);
//     final_price.to_f64().unwrap()
// }

// pub async fn apply_coupon(client: &Client, order: &OrderRequest) -> Result<HttpResponse, HttpException> {
//     let mut coupon = None;

//     if let Some(coupon_code) = &order.coupon_code {
//         let response = client
//             .get_item()
//             .table_name("Coupon")
//             .key("code", AttributeValue::S(coupon_code.clone()))
//             .send()
//             .await?;

//         if response.item.is_none() {
//             return Err(HttpException::new(400, "Coupon not found!"));
//         }
//         coupon = Some(Coupon::from(response.item.unwrap()));
//     }

//     let total_price = order.items.iter().map(|item| item.price).sum::<f64>();
//     let final_price = calculate_price(order, coupon.as_ref());

//     Ok(HttpResponse::Ok().json(json!({
//         "order_id": Uuid::new_v4().to_string(),
//         "total_price": total_price,
//         "final_price": final_price,
//         "coupon_code": order.coupon_code.clone()
//     })))
// }

// pub async fn complete_order(client: &Client, order: &OrderRequest) -> Result<HttpResponse, HttpException> {
//     let mut coupon = None;

//     if let Some(coupon_code) = &order.coupon_code {
//         let response = client
//             .get_item()
//             .table_name("Coupon")
//             .key("code", AttributeValue::S(coupon_code.clone()))
//             .send()
//             .await?;

//         if response.item.is_none() {
//             return Err(HttpException::new(400, "Coupon not found"));
//         }
//         coupon = Some(Coupon::from(response.item.unwrap()));
//     }

//     let total_price = Decimal::from_f64(order.items.iter().map(|item| item.price).sum::<f64>()).unwrap();
//     let final_price = apply_coupon(client, order).await?;
    
//     let order_id = Uuid::new_v4().to_string();
    
//     // Store order in the DB
//     let order_data = {
//         "order_id": order_id,
//         "items": order.items.clone(),
//         "total_price": total_price,
//         "final_price": final_price,
//         "coupon_code": order.coupon_code.clone(),
//         "status": "completed"
//     };

//     client.put_item().table_name("Order").item(order_data).send().await?;

//     Ok(HttpResponse::Ok().json(json!({
//         "order_id": order_id,
//         "status": "completed",
//         "total_price": total_price.to_f64().unwrap(),
//         "final_price": final_price.to_f64().unwrap(),
//         "coupon_code": order.coupon_code.clone(),
//         "items": order.items
//     })))
// }

// pub async fn create_coupon(client: &Client, coupon: Coupon) -> Result<Coupon, HttpException> {
//     let coupon_data = coupon.clone();

//     client.put_item().table_name("Coupon").item(coupon_data).send().await?;
    
//     Ok(coupon)
// }

// pub async fn get_all_coupons(client: &Client) -> Result<Vec<Coupon>, HttpException> {
//     let response = client.scan().table_name("Coupon").send().await?;

//     let coupons: Vec<Coupon> = response.items().unwrap_or_default()
//         .iter()
//         .map(|item| Coupon::from(item.clone()))
//         .collect();

//     Ok(coupons)
// }