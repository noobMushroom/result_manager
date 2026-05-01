use actix_web::{HttpResponse, post, web};
use secrecy::SecretString;

use crate::{
    auth::jwt::generate_jwt,
    domain::{phone::Phone, roles::Role},
    errors::AppError,
    otp::service::verify_otp_hash,
    repostiory::{otp_repo::OtpRepo, teacher_repo::TeacherRepo},
};

#[derive(serde::Deserialize)]
struct VerifyOtpBody {
    otp: SecretString,
    phone: String,
}

/// Verifies the otp if right sends back the jwt token and if wrong increase the wrong otp attempts
/// and return unouthorized
#[tracing::instrument(name = "Verifying the otp", skip(teacher_repo, body, secret, redis_connection))]
#[post("/verify")]
pub async fn verify_otp(
    teacher_repo: web::Data<TeacherRepo>,
    body: web::Json<VerifyOtpBody>,
    secret: web::Data<SecretString>,
    redis_connection: web::Data<OtpRepo>,
) -> Result<HttpResponse, AppError> {
    let VerifyOtpBody { otp, phone } = body.into_inner();
    let phone = Phone::parse(&phone)?;
    redis_connection.ensure_not_banned(&phone).await?;
    // Getting otp from the db
    let otp_hash = SecretString::new(redis_connection.get_otp(&phone).await?.into());

    if !verify_otp_hash(&otp, &otp_hash).map_err(|e| {
        tracing::error!(error=?e, "Argon error falied to verify");
        AppError::Internal
    })? {
        redis_connection
            .increment_wrong_attempts(&phone, 600)
            .await?;
        return Err(AppError::Unauthorised);
    }
    redis_connection.reset_bans(&phone).await?;

    let user = teacher_repo.get_teacher(&phone).await?;
    let role = Role::try_from(user.role)?;

    let jwt_token =
        generate_jwt(user.id, role, &secret.into_inner()).map_err(|_| AppError::Internal)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": jwt_token,
        "token_type": "Bearer"
    })))
}
