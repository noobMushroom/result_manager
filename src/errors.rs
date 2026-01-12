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

    #[error("Unouthorised")]
    Unauthorised,

    #[error("Forbidden")]
    Forbidden,

    #[error("Too many Requests")]
    TooManyRequests(String),
}

#[derive(serde::Serialize)]
pub struct ErrorResponse<'a> {
    pub error: &'a str,
}

impl From<DomainError> for AppError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::InvalidName
            | DomainError::InvalidPhone
            | DomainError::InvalidGrade
            | DomainError::InvalidDateOfBirth
            | DomainError::InvalidRole
            | DomainError::UserNotFound => AppError::BadRequest(err.to_string()),

            DomainError::DuplicateAdmissionNo | DomainError::DuplicatePhoneNo => {
                AppError::Conflict(err.to_string())
            }
            DomainError::Internal => AppError::Internal,
            DomainError::Unauthorised => AppError::Unauthorised,
            DomainError::Forbidden => AppError::Forbidden,
            DomainError::TooManyRequest(err) => AppError::TooManyRequests(err.to_string()),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let mut builder = HttpResponse::build(self.status_code());

        match self {
            AppError::BadRequest(msg) | AppError::Conflict(msg) => builder
                .insert_header(ContentType::json())
                .json(ErrorResponse { error: msg }),
            AppError::Internal => builder
                .insert_header(ContentType::json())
                .json(ErrorResponse {
                    error: "Internal server error",
                }),
            AppError::Unauthorised => {
                builder
                    .insert_header(ContentType::json())
                    .json(ErrorResponse {
                        error: "Unauthorised user",
                    })
            }
            AppError::Forbidden => builder
                .insert_header(ContentType::json())
                .json(ErrorResponse {
                    error: "You are banned from the service",
                }),

            AppError::TooManyRequests(msg) => builder
                .insert_header(ContentType::json())
                .json(ErrorResponse { error: msg }),
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorised => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
        }
    }
}
