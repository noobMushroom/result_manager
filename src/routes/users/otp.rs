use actix_web::{HttpResponse, get, web};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier, rand_core::OsRng},
};

use argon2::password_hash::{PasswordHasher, SaltString};
use chrono::Utc;
use rand::{Rng, rng};
use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::{errors::DomainError, phone::Phone},
    errors::AppError,
};

pub fn generate_otp() -> SecretString {
    let otp = rng().random_range(100000..=999999).to_string();
    SecretString::new(otp.into())
}

fn hash_otp(otp: &SecretString) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2.hash_password(otp.expose_secret().as_bytes(), &salt)?;

    Ok(hash.to_string())
}

pub fn verify_otp(
    otp: &SecretString,
    stored_hash: &str,
) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(stored_hash)?;

    Ok(Argon2::default()
        .verify_password(otp.expose_secret().as_bytes(), &parsed_hash)
        .is_ok())
}

#[tracing::instrument(name = "saving otp in the database", skip(pool, phone))]
pub async fn insert_otp(
    pool: &PgPool,
    phone: &Phone,
    otp: &SecretString,
) -> Result<(), DomainError> {
    let hashed_otp = hash_otp(&otp)?;
    sqlx::query!(
        r#"
            INSERT INTO otp_requests (id, phone_number, otp_hash, expires_at, created_at)
            VALUES($1, $2, $3, $4, $5)
        "#,
        Uuid::new_v4(),
        phone.as_ref(),
        hashed_otp,
        Utc::now() + chrono::Duration::minutes(5),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!(error = ?e, "Inserting otp" );
        DomainError::Internal
    })?;
    Ok(())
}

#[derive(serde::Deserialize)]
struct VerifyOtpBody {
    #[allow(unused)]
    otp: String,
}

// #[tracing::instrument(name = "Verifying the otp", skip(pool, body))]
#[get("/verify")]
pub async fn verify(
    _pool: web::Data<PgPool>,
    _body: web::Json<VerifyOtpBody>,
) -> Result<HttpResponse, AppError> {
    todo!()
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
