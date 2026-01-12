use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("invalid name")]
    InvalidName,

    #[error("invalid phone number")]
    InvalidPhone,

    #[error("invalid grade")]
    InvalidGrade,

    #[error("invalid date of birth")]
    InvalidDateOfBirth,

    #[error("admission number already exists")]
    DuplicateAdmissionNo,

    #[error("phone no already exists")]
    DuplicatePhoneNo,

    #[error("User not found")]
    UserNotFound,

    #[error("Internal server error")]
    Internal,

    #[error("Invalid Role")]
    InvalidRole,

    #[error("Unauthorised")]
    Unauthorised,

    #[error("Forbidden")]
    Forbidden,

    #[error("Too Many Requests {0}")]
    TooManyRequest(String),
}
