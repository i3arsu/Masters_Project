use actix_web::{delete, get, post, web, HttpResponse, Responder};
use crate::db::item;

#[get("/id/{item_id}")]
async fn get_item_by_id(path: web::Path<String>) -> impl Responder {
    let item_id = path.into_inner();
    println!("{}", item_id);
    match item::get_item_by_id(&item_id).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::NotFound().body("Item not found"),
    }
}

#[get("/all")]
async fn all_items() -> impl Responder {
    match item::get_all_items().await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::NotFound().body("No items found."),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_item_by_id);
    cfg.service(all_items);
}
