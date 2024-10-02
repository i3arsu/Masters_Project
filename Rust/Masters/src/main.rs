use actix_web::{web, App, HttpServer, HttpResponse, Responder};
mod routes;

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
            .configure(routes::coupon_routes::init_routes)
            .configure(routes::item_routes::init_routes)
            .configure(routes::order_routes::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
