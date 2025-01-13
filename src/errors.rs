use std::fmt::{Display, Formatter};
use serde_json;

#[derive(Debug)]
pub enum CustomError {
    NotFound,
    BadRequest,
    IoError(std::io::Error),
    CacheDbLoadError,
    CacheDbCapacityError,
    CacheDbSaveError,
    SerializationError(serde_json::Error),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::NotFound => write!(f, "404 Not Found"),
            CustomError::BadRequest => write!(f, "400 Bad Request"),
            CustomError::IoError(err) => write!(f, "IO Error: {}", err),
            CustomError::CacheDbLoadError => write!(f, "Cache DB Load Error"),
            CustomError::CacheDbCapacityError => write!(f, "Cache DB Capacity Error"),
            CustomError::CacheDbSaveError => write!(f, "Cache DB Save Error"),
            CustomError::SerializationError(err) => write!(f, "Serialization Error: {}", err),
        }
    }
}

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> Self {
        CustomError::IoError(err)
    }
}

impl From<serde_json::Error> for CustomError {
    fn from(err: serde_json::Error) -> Self {
        CustomError::SerializationError(err)
    }
}