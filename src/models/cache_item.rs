use derive_ctor::ctor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[derive(ctor)]
pub struct CacheItem{
    pub(crate) key: String,
    pub(crate) value: String
}