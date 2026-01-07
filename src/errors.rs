use crate::domain::errors::DomainError;
use actix_web::{
    HttpResponse, ResponseError,
    http::{StatusCode, header::ContentType},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    BadRequest(String),

    #[error("{0}")]
    Conflict(String),

    #[error("internal server error")]
    Internal,
}

impl From<DomainError> for AppError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::InvalidName
            | DomainError::InvalidPhone
            | DomainError::InvalidGrade
            | DomainError::InvalidDateOfBirth => AppError::BadRequest(err.to_string()),

            DomainError::DuplicateAdmissionNo => AppError::Conflict(err.to_string()),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::BadRequest(msg) => HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .body(msg),
            AppError::Conflict(msg) => HttpResponse::Conflict().json(msg),

            AppError::Internal => HttpResponse::InternalServerError().finish(),
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}
