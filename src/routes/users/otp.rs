use actix_web::{HttpResponse, post, web};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier, rand_core::OsRng},
};

use argon2::password_hash::{PasswordHasher, SaltString};
use chrono::{DateTime, Utc};
use rand::{Rng, rng};
use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::{errors::DomainError, phone::Phone},
    errors::AppError,
    routes::users::moderate::ensure_not_banned,
};

// Generates the otp
pub fn generate_otp() -> SecretString {
    let otp = rng().random_range(100000..=999999).to_string();
    SecretString::new(otp.into())
}

// Creates the hash from otp
fn hash_otp(otp: &SecretString) -> Result<SecretString, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(otp.expose_secret().as_bytes(), &salt)?
        .to_string();

    Ok(SecretString::new(hash.into()))
}

// This funciton verifies otp by the hash
pub fn verify_otp(
    otp: &SecretString,
    stored_hash: &SecretString,
) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(stored_hash.expose_secret())?;

    Ok(Argon2::default()
        .verify_password(otp.expose_secret().as_bytes(), &parsed_hash)
        .is_ok())
}

#[derive(serde::Deserialize)]
struct VerifyOtpBody {
    otp: String,
    phone: String,
}

pub struct OtpRequest {
    pub id: Uuid,
    pub otp_hash: SecretString,
    pub expires_at: DateTime<Utc>,
    pub attempts: i32,
    pub created_at: DateTime<Utc>,
}

// This funciton verifies the otp by /verify endpoint
#[tracing::instrument(name = "Verifying the otp", skip(pool, body))]
#[post("/verify")]
pub async fn verify_user_otp(
    pool: web::Data<PgPool>,
    body: web::Json<VerifyOtpBody>,
) -> Result<HttpResponse, AppError> {
    let VerifyOtpBody { phone, otp } = body.into_inner();
    let phone = Phone::parse(&phone)?;
    ensure_not_banned(&pool, &phone).await?;
    let otp_hash = get_otp_hash(&phone, &pool).await?;
    let otp = SecretString::new(otp.into());

    if let Some(hash) = otp_hash {
        if !verify_otp(&otp, &hash.otp_hash).map_err(|e| {
            tracing::error!(error=?e, "failed to verify");
            DomainError::Internal
        })? {
            return Err(DomainError::Unauthorised)?;
        }
    } else {
        return Err(DomainError::Unauthorised)?;
    }

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "saving otp in the database", skip(pool, phone))]
pub async fn insert_otp(
    pool: &PgPool,
    phone: &Phone,
    otp: &SecretString,
) -> Result<(), DomainError> {
    let hashed_otp = hash_otp(otp).map_err(|e| {
        tracing::error!(error=?e, "failed to vrify");
        DomainError::Internal
    })?;

    if let Some(hash) = get_otp_hash(phone, pool).await? {
        if hash.attempts > 3 {
            // Ban the user here
            todo!()
        } else {
            add_otp_to_table(pool, phone, hashed_otp, hash.attempts + 1).await?
        }
    } else {
        add_otp_to_table(pool, phone, hashed_otp, 0).await?
    }

    Ok(())
}

#[tracing::instrument(name = "adding otp in the database", skip(pool, phone))]
async fn add_otp_to_table(
    pool: &PgPool,
    phone: &Phone,
    hash: SecretString,
    attempts: i32,
) -> Result<(), DomainError> {
    sqlx::query!(
        r#"
            INSERT INTO otp_requests (id, phone_number, otp_hash, expires_at,attempts, created_at)
            VALUES($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(),
        phone.as_ref(),
        hash.expose_secret(),
        Utc::now() + chrono::Duration::minutes(5),
        attempts,
        Utc::now(),
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!(error = ?e, "Inserting otp" );
        DomainError::Internal
    })?;
    Ok(())
}

#[tracing::instrument(name = "Getting the otp hash", skip(pool, phone))]
pub async fn get_otp_hash(phone: &Phone, pool: &PgPool) -> Result<Option<OtpRequest>, DomainError> {
    let otp = sqlx::query_as!(
        OtpRequest,
        r#"
        SELECT id, otp_hash, expires_at, attempts, created_at
        FROM otp_requests
        WHERE phone_number = $1
          AND expires_at > NOW()
        ORDER BY created_at DESC
        LIMIT 1
        "#,
        phone.as_ref()
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!(error=?e, "error while getting the otp hash");
        DomainError::Internal
    })?;

    Ok(otp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_random_otp() {
        let otp1 = generate_otp();
        let otp2 = generate_otp();

        assert_ne!(otp1.expose_secret(), otp2.expose_secret())
    }

    #[test]
    fn test_generate_decode_hash() {
        let otp = generate_otp();
        let hash = hash_otp(&otp).unwrap();
        let decoded_otp = verify_otp(&otp, &hash).unwrap();
        assert!(decoded_otp)
    }

    #[test]
    fn test_random_hash_should_produce_wrong() {
        let hash = hash_otp(&SecretString::new("123457".into())).unwrap();
        let decoded = verify_otp(&SecretString::new("12345".into()), &hash).unwrap();
        assert!(!decoded)
    }
}
