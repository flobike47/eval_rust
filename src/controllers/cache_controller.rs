use actix_web::{get, post, HttpResponse, Responder};
use actix_web::web::{Json, Path};
use crate::CacheItem;
use crate::services::cache_service;

#[get("/cache/{index}")]
async fn get_cache_item_by_key(key: Path<(String,)>) -> HttpResponse {
    match cache_service::get_item_by_key(&key.0) {
        Ok(cache_response) => {
            if cache_response.status == "Not Found" {
                HttpResponse::NotFound()
                    .json(cache_response)
            } else {
                HttpResponse::Ok()
                    .json(cache_response)
            }
        }
        Err(_) => {
            HttpResponse::InternalServerError()
                .finish()
        }
    }
}

#[post("/cache")]
async fn insert_cache_item(item: Json<CacheItem>) -> impl Responder {
    println!("Inserting cache item: {:?}", item);
    match cache_service::insert_item(item.into_inner()) {
        Ok(cache_response) => {
            HttpResponse::Ok()
                .json(cache_response)
        }
        Err(_) => {
            HttpResponse::InternalServerError()
                .finish()
        }
    }
}


