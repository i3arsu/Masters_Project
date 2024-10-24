// src/handlers/order_handler.rs
use crate::models::{order::{OrderRequest, OrderResponse, OrderItem}, coupon::Coupon};
use crate::db::dynamodb::get_dynamodb_client;
use crate::db::coupon::get_coupon_by_code;
use crate::db::item::get_items_by_ids;
use crate::errors::errors::OrderProcessingError;
use actix_web::{web, HttpResponse, Responder, ResponseError};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use serde_json::json;

pub async fn apply_coupon_to_order(
    order_request: web::Json<OrderRequest>,
) -> Result<OrderResponse, OrderProcessingError> {
    let client = get_dynamodb_client().await;

    // Generate a unique order ID
    let order_id = Uuid::new_v4().to_string();

    // Fetch items from the database
    let item_ids: Vec<String> = order_request.items.iter().map(|item| item.item_id.clone()).collect();
    let items = get_items_by_ids(&client, &item_ids)
    .await
    .map_err(|_| OrderProcessingError::ItemFetchError)?;

    // Calculate the total price of items in the order
    let total_price: f64 = items.iter()
        .zip(order_request.items.iter())
        .map(|(item, order_item)| item.price * order_item.quantity as f64)
        .sum();

    // Attempt to apply the coupon if provided
    let (discount_applied, final_price) = if let Some(coupon_code) = &order_request.coupon_code {
        match get_coupon_by_code(coupon_code).await {
            Ok(coupon) => {
                if coupon_applies_to_items(&coupon, &order_request.items) {
                    let discount_amount = (coupon.discount as f64) / 100.0 * total_price;
                    (true, total_price - discount_amount)
                } else {
                    return Err(OrderProcessingError::CouponNotApplicable);
                }
            }
            Err(_) => return Err(OrderProcessingError::InvalidCouponError),
        }
    } else {
        (false, total_price)
    };

    // Create the order response
    let order_response = OrderResponse {
        order_id,
        total_price,
        discount_applied,
        final_price,
    };

    // Return the order response directly
    Ok(order_response)
}


// Helper function to check if the coupon applies to the items
fn coupon_applies_to_items(coupon: &Coupon, items: &[OrderItem]) -> bool {
    if let Some(applicable_items) = &coupon.applicable_items {
        items.iter().any(|item| applicable_items.contains(&item.item_id))
    } else {
        true // If no applicable items are specified, assume the coupon is applicable to all items
    }
}
