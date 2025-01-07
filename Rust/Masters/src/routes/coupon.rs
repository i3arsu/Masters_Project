use actix_web::{delete, get, post, web, HttpResponse, Responder};
use crate::db::dynamodb;
use crate::db::coupon;
use crate::errors::errors::CouponError;
use crate::models::coupon::Coupon;

#[get("/get/{code}")]
async fn get_coupon_by_code(path: web::Path<String>) -> impl Responder {
    let code = path.into_inner();
    match coupon::get_coupon_by_code(&code).await {
        Ok(coupon) => HttpResponse::Ok().json(coupon),
        Err(_) => HttpResponse::NotFound().body("Coupon not found"),
    }
}

#[post("/create")]
async fn create_coupon(coupon: web::Json<Coupon>) -> Result<HttpResponse, CouponError> {
    match coupon::create_coupon(coupon).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => Err(err),
    }
}

#[get("/getall")]
async fn get_all_coupons() -> Result<HttpResponse, CouponError> {
    match coupon::get_all_coupons().await{
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => Err(err),
    }
}

#[delete("/remove/{code}")]
async fn remove_coupon(path: web::Path<String>) -> Result<HttpResponse, CouponError> {
    let code = path.into_inner();
    match coupon::delete_coupon_by_code(&code).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => Err(err),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_coupon_by_code);
    cfg.service(create_coupon);
    cfg.service(get_all_coupons);
}
