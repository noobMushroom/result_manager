use crate::domain::phone::Phone;
use crate::repostiory::otp_repo::OtpRepo;
use crate::{errors::AppError, message_client::MessageClient};
use argon2::password_hash::{PasswordHasher, SaltString};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier, rand_core::OsRng},
};
use rand::{Rng, rng};
use secrecy::{ExposeSecret, SecretString};

/// Checks if the user is banned or not and then set cool down timer for the otp (30 seconds) and
/// increment the otp requests, inserts the otp and sends the otp to user
#[tracing::instrument(name = "saving otp in the redis database and sending it", skip(redis_connection, message_client), fields(phone = %phone.as_ref()))]
pub async fn send_otp(
    redis_connection: &OtpRepo,
    message_client: &MessageClient,
    phone: &Phone,
) -> Result<(), AppError> {
    redis_connection.set_cooldown(phone, 30).await?;
    redis_connection.increment_otp_requests(phone, 600).await?;
    redis_connection.ensure_not_banned(phone).await?;
    let otp = generate_otp();
    let otp_hash = hash_otp(&otp).map_err(|e| {
        tracing::error!(error=?e, "failed to hash the otp");
        AppError::Internal
    })?;
    redis_connection.insert_otp(otp_hash, phone, 300).await?;
    message_client.send_otp(otp, &phone).await.map_err(|e| {
        tracing::error!(error=?e, "error while sending otp");
        AppError::Internal
    })?;

    Ok(())
}

/// Generates a random otp
pub fn generate_otp() -> SecretString {
    let otp = rng().random_range(100000..=999999).to_string();
    SecretString::new(otp.into())
}

/// Creates the hash from the given string
fn hash_otp(otp: &SecretString) -> Result<SecretString, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(otp.expose_secret().as_bytes(), &salt)?
        .to_string();

    Ok(SecretString::new(hash.into()))
}

/// Verifies the hash with given hash
pub fn verify_otp_hash(
    otp: &SecretString,
    stored_hash: &SecretString,
) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(stored_hash.expose_secret())?;

    Ok(Argon2::default()
        .verify_password(otp.expose_secret().as_bytes(), &parsed_hash)
        .is_ok())
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
        let decoded_otp = verify_otp_hash(&otp, &hash).unwrap();
        assert!(decoded_otp)
    }

    #[test]
    fn test_random_hash_should_produce_wrong() {
        let hash = hash_otp(&SecretString::new("123457".into())).unwrap();
        let decoded = verify_otp_hash(&SecretString::new("12345".into()), &hash).unwrap();
        assert!(!decoded)
    }
}
