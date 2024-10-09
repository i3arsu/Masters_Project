use actix_web::{web, App, HttpServer, HttpResponse, Responder};
mod routes {
    mod coupon;
    mod item;
    mod order;
}
mod db {
    mod coupon;
    mod dynamodb;
    mod item;
}
mod models {
    mod completedOrder;
    mod coupon;
    mod item;
    mod orderRequest;
    mod orderResponse;
}

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
            .configure(routes::coupon)
            .configure(routes::item)
            .configure(routes::order)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
