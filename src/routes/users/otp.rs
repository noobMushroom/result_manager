use actix_web::HttpResponse;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier, rand_core::OsRng},
};

use argon2::password_hash::{PasswordHasher, SaltString};
use chrono::Utc;
use rand::{Rng, rng};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{errors::DomainError, phone::Phone};

fn generate_otp() -> String {
    rng().random_range(100000..=999999).to_string()
}

fn hash_otp(otp: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2.hash_password(otp.as_bytes(), &salt)?;

    Ok(hash.to_string())
}

pub fn verify_otp(otp: &str, stored_hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(stored_hash)?;

    Ok(Argon2::default()
        .verify_password(otp.as_bytes(), &parsed_hash)
        .is_ok())
}

#[tracing::instrument(name = "saving otp in the database", skip(pool, phone))]
pub async fn insert_otp(pool: &PgPool, phone: &Phone) -> Result<HttpResponse, DomainError> {
    let otp = generate_otp();
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
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_random_otp() {
        let otp1 = generate_otp();
        let otp2 = generate_otp();

        assert_ne!(otp1, otp2)
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
        let hash = hash_otp("123457").unwrap();
        let decoded = verify_otp("1222233", &hash).unwrap();
        assert!(!decoded)
    }
}
