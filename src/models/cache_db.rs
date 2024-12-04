use derive_ctor::ctor;
use rustbreak::deser::Ron;
use rustbreak::FileDatabase;
use crate::{CacheResponse, CustomError};
use crate::models::cache_item::CacheItem;

#[derive(ctor)]
pub struct CacheDb {
    pub(crate) db: FileDatabase<Vec<CacheItem>, Ron>,
    pub(crate) path: String,
    size: usize,
}

impl CacheDb
{
    pub(crate) fn get_item_by_key(&self, key: &str) -> Result<CacheResponse, CustomError> {
        match self.db.read(|db| {
            return match db.iter().position(|item| item.key == key) {
                Some(index) => {
                    let index_for_user_understand = index+1;
                    CacheResponse::new("Found".to_string(), None, Some(index_for_user_understand), Some(index_for_user_understand), Some(db[index].clone()))
                }
                None => CacheResponse::new("Not Found".to_string(), Some("Item not found with this key".to_string()), None, None, None)
            };
        }) {
            Ok(mut cache_response) => {
                if cache_response.status == "Found".to_string() {
                    self.remove_if_present(key)?;
                    self.insert(&cache_response.cache_item.clone().unwrap())?;
                    self.persist()?;
                    cache_response.current_position = Some(1);
                }
                Ok(cache_response)
            },
            Err(_) => Err(CustomError::InternalServerError)
        }
    }

    pub(crate) fn insert(&self, cache_item: &CacheItem) -> Result<CacheItem, CustomError> {
        let index : usize = 0;
        match self.db.write(|db| {
            db.insert(index, cache_item.clone());
        }) {
            Ok(_) => {
                println!("Cache item inserted");
                Ok(cache_item.clone())
            },
            Err(_) => {
                println!("Error inserting cache item");
                Err(CustomError::InternalServerError)
            }
        }
    }

    pub(crate) fn remove_if_present(&self, key: &str) -> Result<Option<usize>, CustomError> {
        match self.db.write(|db| {
            if let Some(index) = db.iter().position(|item| item.key == key) {
                db.remove(index);
                let index_for_user_understand = index+1;
                return Some(index_for_user_understand)
            } else {
                None
            }
        }) {
            Ok(index) => {
                if let Some(index) = index {
                    println!("Cache item removed");
                    Ok(Some(index))
                } else {
                    println!("Cache item not found");
                    Ok(None)
                }
            },
            Err(_) => {
                println!("Error removing cache item");
                Err(CustomError::InternalServerError)
            }
        }
    }

    pub fn persist(&self) -> Result<(), CustomError> {
        match self.db.save() {
            Ok(_) => Ok(()),
            Err(_) => Err(CustomError::InternalServerError)
        }
    }
}
