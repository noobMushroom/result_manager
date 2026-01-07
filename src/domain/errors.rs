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
}
