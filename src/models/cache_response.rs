use serde::{Deserialize, Serialize};
use crate::CacheItem;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheResponse {
    pub(crate) status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) current_position: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) previous_position: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cache_item: Option<CacheItem>,
}

impl CacheResponse {
    pub(crate) fn new(status: String, details: Option<String>, current_position: Option<usize>, previous_position: Option<usize>, cache_item: Option<CacheItem>) -> Self {
        CacheResponse {
            status,
            details,
            current_position,
            previous_position,
            cache_item,
        }
    }
}