use axum::{http::StatusCode, response::IntoResponse};
use sea_orm::{DbErr, sea_query::value};
use validator::ValidationErrors;

use crate::shared::response::UnprocessableParams;

pub enum ApiError {
    Unexpected(Box<dyn std::error::Error + Send + Sync>),
    NotFound,
    Validation(ValidationErrors)
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::Unexpected(err) => {
                println!("Internal server: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR).into_response()
            }
            ApiError::NotFound => (StatusCode::NOT_FOUND).into_response(),
            ApiError::Validation( err) => {
                UnprocessableParams(err).into_response()
            }
        }
    }
}

impl From<DbErr> for ApiError {
    fn from(value: DbErr) -> Self {
        Self::Unexpected(Box::new(value))
    }
}


impl From<ValidationErrors> for ApiError  {
    fn from(value:  ValidationErrors) -> Self {
        Self::Validation(value)   
    }
}