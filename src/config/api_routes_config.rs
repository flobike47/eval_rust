use actix_web::web;
use crate::controllers::ping_controller::ping;
use crate::{get_cache_item_by_key, insert_cache_item};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(insert_cache_item)
        .service(get_cache_item_by_key)
        .service(ping)
    ;
}