use actix_web::{delete, get, post, web, HttpResponse, Responder};
use crate::models::order::{OrderRequest};
use crate::db::order;
use crate::errors::errors::OrderProcessingError;

#[post("/apply")]
async fn apply_coupon(order_request: web::Json<OrderRequest>) -> Result<HttpResponse, OrderProcessingError> {
    match order::apply_coupon_to_order(order_request).await {
        Ok(order_response) => Ok(HttpResponse::Ok().json(order_response)),
        Err(err) => Err(err),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(apply_coupon);
}
