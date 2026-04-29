use thiserror::Error;

#[derive(Debug, Error)]
pub enum OtpErrors {
    #[error("OtpExpiredOrNotFound: {0}")]
    OtpExpiredOrNotFound(String),
}
