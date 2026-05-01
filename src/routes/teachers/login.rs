use actix_web::{HttpResponse, Result, post, web};

use crate::{
    domain::phone::Phone,
    errors::AppError,
    message_client::MessageClient,
    otp::service::send_otp,
    repostiory::{otp_repo::OtpRepo, teacher_repo::TeacherRepo},
};

#[derive(serde::Deserialize)]
pub struct UserDetails {
    pub phone: String,
}

#[tracing::instrument(
    name = "Log in: Checking the credentials and sending otp",
    skip(user, teacher_repo, message_client, redis),
    fields(
        name = %user.phone,
    )
)]
#[post("/login")]
pub async fn teacher_login(
    user: web::Json<UserDetails>,
    teacher_repo: web::Data<TeacherRepo>,
    redis: web::Data<OtpRepo>,
    message_client: web::Data<MessageClient>,
) -> Result<HttpResponse, AppError> {
    let UserDetails { phone } = user.into_inner();
    let phone = Phone::parse(&phone)?;

    if !teacher_repo.check_teacher_exist(&phone).await? {
        return Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "If the number is registered, an OTP has been sent"
        })));
    }
    send_otp(&redis, &message_client, &phone).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "If the number is registered, an OTP has been sent"
    })))
}
