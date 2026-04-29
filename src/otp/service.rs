use crate::auth::jwt::generate_jwt;
use crate::domain::phone::Phone;
use crate::domain::roles::Role;
use crate::routes::users::otp::get_user;
use crate::{errors::AppError, message_client::MessageClient, redis::repo::RedisRepo};
use actix_web::{HttpResponse, post, web};
use argon2::password_hash::{PasswordHasher, SaltString};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier, rand_core::OsRng},
};
use rand::{Rng, rng};
use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;


#[derive(serde::Deserialize)]
struct VerifyOtpBody {
    otp: SecretString,
    phone: Phone,
}


/// Verifies the otp if right sends back the jwt token and if wrong increase the wrong otp attempts
/// and return unouthorized
#[tracing::instrument(name = "Verifying the otp", skip(pool, body, secret, redis_connection))]
#[post("/verify")]
pub async fn verify_otp(
    pool: web::Data<PgPool>,
    body: web::Json<VerifyOtpBody>,
    secret: web::Data<SecretString>,
    redis_connection: web::Data<RedisRepo>,
) -> Result<HttpResponse, AppError> {
    let VerifyOtpBody { otp, phone} = body.into_inner();
    redis_connection.ensure_not_banned(&phone).await?;
    // Getting otp from the db
    let otp_hash = SecretString::new(redis_connection.get_otp(&phone).await?.into());
    
    if !verify_otp_hash(&otp, &otp_hash).map_err(|e| {
        tracing::error!(error=?e, "Argon error falied to verify");
        AppError::Internal
    })? {
        redis_connection.increment_wrong_attempts(&phone, 600).await?;
        return Err(AppError::Unauthorised);
    }
    redis_connection.reset_bans(&phone).await?;
    
    let user = get_user(&phone, &pool).await?;
    let role = Role::try_from(user.role)?;

    let jwt_token = generate_jwt(user.id, role, &secret.into_inner()).map_err(|_| AppError::Internal)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": jwt_token,
        "token_type": "Bearer"
    })))
}

/// Checks if the user is banned or not and then set cool down timer for the otp (30 seconds) and
/// increment the otp requests, inserts the otp and sends the otp to user
#[tracing::instrument(name = "saving otp in the redis database and sending it", skip(redis_connection, message_client), fields(phone = %phone.as_ref()))]
pub async fn send_otp(
    redis_connection: &RedisRepo,
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
