use actix_web::{HttpResponse, Result, post, web};
use sqlx::PgPool;

use crate::{
    domain::{errors::DomainError, phone::Phone},
    errors::AppError,
    message_client::MessageClient,
    otp::service::send_otp,
    redis::repo::RedisRepo,
};

#[derive(serde::Deserialize)]
pub struct UserDetails {
    pub phone: Phone,
}

#[tracing::instrument(
    name = "Log in: Checking the credentials and sending otp",
    skip(user, pool, message_client, redis),
    fields(
        name = %user.phone.as_ref(),
    )
)]
#[post("/login")]
pub async fn teacher_login(
    user: web::Json<UserDetails>,
    pool: web::Data<PgPool>,
    redis: web::Data<RedisRepo>,
    message_client: web::Data<MessageClient>,
) -> Result<HttpResponse, AppError> {
    // let phone = Phone::parse(&user.phone)?;
    if check_user(&user.phone, &pool).await.is_err() {
        return Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "If the number is registered, an OTP has been sent"
        })));
    }
    send_otp(&redis, &message_client, &user.phone).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "If the number is registered, an OTP has been sent"
    })))
}

#[tracing::instrument(name = "checking if the teacher is registered", skip(phone, pool))]
pub async fn check_user(phone: &Phone, pool: &PgPool) -> Result<(), DomainError> {
    let user = sqlx::query_scalar!(
        "SELECT 1 FROM teachers WHERE phone_no = $1 LIMIT 1",
        phone.as_ref()
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| DomainError::Internal)?
    .is_some();

    if !user {
        return Err(DomainError::UserNotFound);
    }

    Ok(())
}
