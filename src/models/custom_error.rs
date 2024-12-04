use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CustomError {
    NotFound,
    BadRequest,
    InternalServerError,
    CacheDbLoadError,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::NotFound => write!(f, "404 Not Found"),
            CustomError::BadRequest => write!(f, "400 Bad Request"),
            CustomError::InternalServerError => write!(f, "500 Internal Server Error"),
            CustomError::CacheDbLoadError => write!(f, "Cache DB Load Error"),
        }
    }
}