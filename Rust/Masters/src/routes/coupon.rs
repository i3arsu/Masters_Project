use actix_web::{delete, get, post, web, HttpResponse, Responder};
use aws_sdk_dynamodb::Client;
use serde::{Deserialize};
use crate::db::dynamodb;
use crate::db::coupon;
use crate::models::coupon::Coupon;

#[get("/{code}")]
async fn get_coupon_by_code(path: web::Path<String>) -> impl Responder {
    let code = path.into_inner();
    match coupon::get_coupon_by_code(&code).await {
        Ok(coupon) => HttpResponse::Ok().json(coupon),
        Err(_) => HttpResponse::NotFound().body("Coupon not found"),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_coupon_by_code);
}
