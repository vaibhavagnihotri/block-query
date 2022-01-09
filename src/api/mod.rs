use crate::types::{ErrorResponse, Result};
use actix_web::HttpResponse;
use serde::Serialize;
use serde_json::json;

pub mod block;

pub fn http_response<T>(result: Result<T>) -> HttpResponse
where
    T: Serialize,
{
    match result {
        Ok(v) => HttpResponse::Ok()
            .content_type(mime::APPLICATION_JSON)
            .json(v),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(mime::APPLICATION_JSON)
            .json(ErrorResponse::from(e)),
    }
}
