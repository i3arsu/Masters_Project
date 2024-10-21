use actix_web::{delete, get, post, web, HttpResponse, Responder};
use crate::db::item;

#[get("/{item_id}")]
async fn get_item_by_id(path: web::Path<String>) -> impl Responder {
    let item_id = path.into_inner();
    println!("{}", item_id);
    match item::get_item_by_id(&item_id).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::NotFound().body("Item not found"),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_item_by_id);
}
