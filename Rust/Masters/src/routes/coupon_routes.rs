use actix_web::{web, HttpResponse, Responder};
use aws_sdk_dynamodb::Client;
use serde::{Deserialize};
use crate::db::dynamodb;
use crate::models::coupon::Coupon;

#[derive(Deserialize)]
pub struct CouponRedeemRequest {
    pub code: String,
}

pub async fn redeem_coupon(
    data: web::Json<CouponRedeemRequest>,
    dynamo_client: web::Data<Client>
) -> impl Responder {
    let code = &data.code;

    // Fetch coupon from DynamoDB using the db module
    match dynamodb::get_coupon_by_code(dynamo_client.clone(), code.clone()).await {
        Ok(Some(mut coupon)) => {
            if coupon.is_redeemed || chrono::Utc::now().to_rfc3339() > coupon.expiration_date {
                return HttpResponse::BadRequest().json("Coupon is invalid or expired");
            }

            // Mark coupon as redeemed in DynamoDB
            coupon.is_redeemed = true;
            if dynamodb::update_coupon(coupon, dynamo_client).await.is_err() {
                return HttpResponse::InternalServerError().json("Failed to redeem coupon");
            }

            HttpResponse::Ok().json("Coupon redeemed")
        }
        Ok(None) => HttpResponse::NotFound().json("Coupon not found"),
        Err(_) => HttpResponse::InternalServerError().json("Error fetching coupon"),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/redeem-coupon").route(web::post().to(redeem_coupon)));
}
