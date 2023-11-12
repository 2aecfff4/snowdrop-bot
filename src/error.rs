use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Unknown error.")]
    Unknown,
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorMessage {
    pub error: String,
}
