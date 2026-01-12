use crate::domain::{errors::DomainError, phone::Phone};
use sqlx::PgPool;

#[tracing::instrument(name = "Checking if user is saved in the ban table", skip(pool, phone), fields(phone = %phone.as_ref()))]
pub async fn ensure_not_banned(pool: &PgPool, phone: &Phone) -> Result<(), DomainError> {
    let user = sqlx::query!(
        r#"
            SELECT banned_until from user_bans WHERE phone_number=$1 AND banned_until > NOW() 
        "#,
        phone.as_ref()
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!(error=?e, "Failed to fetch user from table");
        DomainError::Internal
    })?;

    if user.is_some() {
        return Err(DomainError::Forbidden);
    }

    Ok(())
}
