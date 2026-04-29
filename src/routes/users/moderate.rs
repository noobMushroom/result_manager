use crate::domain::{errors::DomainError, phone::Phone};
use redis::{AsyncTypedCommands, aio::ConnectionManager};

#[tracing::instrument(name = "Checking if user is banned", skip(redis, phone), fields(phone = %phone.as_ref()))]
pub async fn ensure_not_banned(redis:&mut ConnectionManager, phone: &Phone) -> Result<(), DomainError> {
    let key = format!("otp_ban:{}", phone.as_ref());
    let user =  redis.exists(key).await.map_err(|e| {
        tracing::error!(error=?e, "Failed to fetch user info from the table");
        DomainError::Internal
    })?;

    if user {
        return Err(DomainError::Forbidden);
    }

    Ok(())
}

