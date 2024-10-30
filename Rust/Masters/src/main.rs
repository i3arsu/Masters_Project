#![allow(warnings)]

extern crate actix_web;

use actix_web::{web, App, HttpServer, HttpResponse, Responder};

mod routes {
    pub mod coupon;
    pub mod item;
    pub mod order;
}
mod db {
    pub mod coupon;
    pub mod dynamodb;
    pub mod item;
    pub mod order;
}
mod models {
    pub mod order;
    pub mod coupon;
    pub mod item;

}
mod errors{
    pub mod errors;
}

use crate::routes::coupon;
use crate::routes::item;
use crate::routes::order;

async fn index() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "CouponAPI"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::scope("/coupon").configure(routes::coupon::init_routes))
            .service(web::scope("/item").configure(routes::item::init_routes))
            .service(web::scope("/order").configure(routes::order::init_routes))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
