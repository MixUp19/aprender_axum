use axum::{http::StatusCode, response::IntoResponse};
use sea_orm::DbErr;
use validator::ValidationErrors;

use crate::{
    shared::response::UnprocessableParams,
};

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    Unexpected(Box<dyn std::error::Error + Send + Sync>),
    #[error("Not found")]
    NotFound,
    #[error("validation error: {0}")]
    Validation(ValidationErrors),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::Unexpected(err) => {
                println!("Internal server: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR).into_response()
            }
            ApiError::NotFound => (StatusCode::NOT_FOUND).into_response(),
            ApiError::Validation(err) => UnprocessableParams(err).into_response(),
        }
    }
}

impl From<DbErr> for ApiError {
    fn from(value: DbErr) -> Self {
        match value {
            DbErr::RecordNotFound(_) => Self::NotFound,
            _ => Self::Unexpected(Box::new(value))
        }   
    }
}


impl From<ValidationErrors> for ApiError {
    fn from(value: ValidationErrors) -> Self {
        Self::Validation(value)
    }
}
