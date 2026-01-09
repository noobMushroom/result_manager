use actix_web::{HttpResponse, post, web};
use sqlx::PgPool;

use crate::{
    domain::{errors::DomainError, phone::Phone},
    errors::AppError,
    routes::users::otp::insert_otp,
};

#[derive(serde::Deserialize)]
struct UserDetails {
    phone: String,
}

#[tracing::instrument(
    name = "adding a new student",
    skip(user, pool),
    fields(
        name = %user.phone,
    )
)]
#[post("/login")]
pub async fn teacher_login(
    user: web::Json<UserDetails>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let phone = Phone::parse(&user.phone)?;
    if !check_user(&phone, &pool).await.is_ok() {
        return Ok(HttpResponse::Ok().finish());
    }
    insert_otp(&pool, &phone).await?;

    Ok(HttpResponse::Ok().finish())
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
