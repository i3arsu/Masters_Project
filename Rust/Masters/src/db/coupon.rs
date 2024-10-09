use aws_sdk_dynamodb::{Client, Error};
use aws_sdk_dynamodb::types::AttributeValue;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models;
use actix_web::{HttpResponse};

pub async fn calculate_price(order: &models::orderRequest::OrderRequest, coupon: Option<&models::coupon::Coupon>) -> f64 {
    if coupon.is_none() {
        return order.items.iter().map(|item| item.price).sum();
    }

    let coupon = coupon.unwrap();

    if let Some(expires_at) = &coupon.expiration_date {
        if expires_at < &chrono::Utc::now().to_string() {
            panic!("Coupon has expired");
        }
    }

    let mut discount = Decimal::ZERO;
    let total_price = order.items.iter().map(|item| item.price).sum::<f64>();

    // Calculate the discount
    if let Some(applicable_items) = &coupon.applicable_items {

        for item in &order.items {
            if applicable_items.contains(&item.id) {
                discount += Decimal::from_f64(item.price).unwrap() * (Decimal::from_f64(coupon.discount_percentage / 100.0).unwrap());
            }
        }
    } else {

        discount = Decimal::from_f64(total_price).unwrap() * (Decimal::from_f64(coupon.discount_percentage / 100.0).unwrap());
    }

    let final_price = (Decimal::from_f64(total_price).unwrap() - discount).max(Decimal::ZERO);
    final_price.to_f64().unwrap()
}

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