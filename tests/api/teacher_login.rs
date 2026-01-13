use crate::helpers::{mock_server, spawn_app};
use chrono::{Duration, Utc};

#[derive(serde::Serialize)]
pub struct LoginReqBody {
    phone: String,
}

impl LoginReqBody {
    pub fn new(phone: &str) -> Self {
        Self {
            phone: phone.to_string(),
        }
    }
}

#[actix::test]
pub async fn register_teacher_returns_200_invalid_data() {
    let app = spawn_app().await;
    let phone = "1234567890";
    let body = LoginReqBody::new(phone);
    let response = app.send_login_req(&body).await;
    assert_eq!(200, response.status().as_u16());
}

#[actix::test]
pub async fn add_otp_for_valid_phone() {
    let app = spawn_app().await;
    mock_server(&app.message_server).await;

    let phone = "1234567890";
    app.add_teacher(&phone).await;
    let body = LoginReqBody::new(phone);
    let response = app.send_login_req(&body).await;
    let count = app.get_row_count_otp().await;
    assert_eq!(200, response.status().as_u16());
    assert_eq!(count, Some(1))
}

#[actix::test]
pub async fn doesnt_add_otp_for_invalid() {
    let app = spawn_app().await;
    let phone = "1234567892";
    app.add_teacher("1234567890").await;
    let body = LoginReqBody::new(phone);
    let response = app.send_login_req(&body).await;
    let count = app.get_row_count_otp().await;
    assert_eq!(200, response.status().as_u16());
    assert_eq!(count, Some(0))
}

#[actix::test]
pub async fn return_forbidden_if_user_for_banned() {
    let app = spawn_app().await;
    let phone = "1234567892";
    sqlx::query!(
        r#"
            INSERT INTO user_bans (phone_number, banned_until, reason)
            VALUES($1, $2, $3)
        "#,
        phone,
        Utc::now() + Duration::minutes(5),
        "Too many otp attempts"
    )
    .execute(&app.db_pool)
    .await
    .unwrap();
    app.add_teacher(&phone).await;
    let body = LoginReqBody::new(phone);
    let response = app.send_login_req(&body).await;
    let count = app.get_row_count_otp().await;
    assert_eq!(403, response.status().as_u16());
    assert_eq!(count, Some(0))
}
