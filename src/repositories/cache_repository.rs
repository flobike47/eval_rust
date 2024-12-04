use std::path::Path;
use std::sync::LazyLock;
use rustbreak::deser::Ron;
use rustbreak::FileDatabase;
use crate::CacheResponse;
use crate::models::cache_db::CacheDb;
use crate::models::cache_item::CacheItem;
use crate::models::custom_error::CustomError;

static CACHE_DB_PATH: LazyLock<String> = LazyLock::new(|| String::from("cache_default_db.db"));
static CACHE_DB_SIZE: LazyLock<usize> = LazyLock::new(|| 10);


fn load_cache_db(path: Option<&str>, db_size: Option<usize>) -> Result<CacheDb, CustomError> {
    let db: FileDatabase<Vec<CacheItem>, Ron> = match FileDatabase::load_from_path_or_default(
        Path::new(path.unwrap_or(&CACHE_DB_PATH)),
    ) {
        Ok(db) => db,
        Err(_) => return Err(CustomError::CacheDbLoadError)
    };

    let cache_db = CacheDb::new(
        db,
        String::from(path.unwrap_or(&CACHE_DB_PATH)),
        db_size.unwrap_or(CACHE_DB_SIZE.clone())
    );

    Ok(cache_db)
}

pub fn get_item_by_key(key: &String) -> Result<CacheResponse,CustomError> {
    let repo = load_cache_db(None, None)?;
    match repo.get_item_by_key(&key){
        Ok(cache_response) => Ok(cache_response),
        Err(_) => Err(CustomError::InternalServerError)
    }
}

pub(crate) fn insert_cache_item(item: &CacheItem) -> Result<CacheResponse, CustomError> {
    let repo = load_cache_db(None, None)?;
    let previous_index: Option<usize> = match repo.remove_if_present(&item.key) {
        Ok(previous_index) => previous_index,
        Err(_) => return Err(CustomError::InternalServerError)
    };

    repo.insert(&item)?;

    match repo.db.save() {
        Ok(_) => {
            let index_readable: usize = 0+1;
            Ok(CacheResponse::new("Inserted".to_string(), None, Some(index_readable), previous_index, Some(item.clone())))
        },
        Err(_) => {
            Err(CustomError::InternalServerError)
        }
    }

}