use crate::{cache_repository, CacheResponse, CustomError};
use crate::models::cache_item::CacheItem;

pub fn get_item_by_key(key: &String) -> Result<CacheResponse,CustomError> {
    println!("Getting cache item by key: {}", key);

    match cache_repository::get_item_by_key(key) {
        Ok(cache_response) => Ok(cache_response),
        Err(_) => Err(CustomError::InternalServerError)
    }

}

pub(crate) fn insert_item(cache_item: CacheItem) -> Result<CacheResponse, CustomError> {
    println!("Saving cache item: {:?}", cache_item);

    match cache_repository::insert_cache_item(&cache_item) {
        Ok(result) => Ok(result),
        Err(_) => Err(CustomError::InternalServerError)
    }
}