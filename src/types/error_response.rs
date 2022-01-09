use chrono::{DateTime, Utc};
use failure::{Error, Fail};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

impl From<Error> for ErrorResponse {
    fn from(error: Error) -> Self {
        ErrorResponse {
            message: format!("{}", error),
            timestamp: Utc::now(),
        }
    }
}
