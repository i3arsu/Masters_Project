use actix_web::{delete, get, post, web, HttpResponse, Responder};
use crate::db::dynamodb;
use crate::db::coupon;

#[get("/{code}")]
async fn get_coupon_by_code(path: web::Path<String>) -> impl Responder {
    let code = path.into_inner();
    println!("{}",code);
    match coupon::get_coupon_by_code(&code).await {
        Ok(coupon) => HttpResponse::Ok().json(coupon),
        Err(_) => HttpResponse::NotFound().body("Coupon not found"),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_coupon_by_code);
}
