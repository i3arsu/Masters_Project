use actix_web::{App, HttpServer};
use aws_sdk_dynamodb::Client;
use crate::routes::coupon_routes;
use std::io::Result;

mod models;
mod routes;
mod db;

#[actix_web::main]
async fn main() -> Result<()> {
    // Load AWS configuration and create DynamoDB client
    let config = aws_config::load_from_env().await;
    let dynamo_client = Client::new(&config);

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(dynamo_client.clone()))  // Pass DynamoDB client
            .configure(coupon_routes::init_routes)  // Set up routes
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
