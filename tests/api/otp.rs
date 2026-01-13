use chrono::{Duration, Utc};
use serde_json::Value;
use wiremock::{Mock, ResponseTemplate, matchers::method};

use crate::{helpers::spawn_app, teacher_login::LoginReqBody};

#[derive(serde::Serialize)]
pub struct VerifyOtpBody {
    pub phone: String,
    pub otp: String,
}

impl VerifyOtpBody {
    pub fn new(phone: &str, otp: &str) -> Self {
        Self {
            phone: phone.to_string(),
            otp: otp.to_string(),
        }
    }
}

#[actix::test]
async fn responds_with_200_ok_for_valid_otp() {
    let app = spawn_app().await;
    let phone = "1234567890";
    app.add_teacher(&phone, "admin").await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.message_server)
        .await;

    let otp = app.request_otp_and_extract(&phone).await;
    let body = VerifyOtpBody::new(phone, &otp);
    let response = app.send_verify_otp_req(&body).await;

    assert_eq!(response.status().as_u16(), 200)
}

#[actix::test]
async fn responds_with_jwt_token_for_successful_login() {
    let app = spawn_app().await;
    let phone = "1234567890";
    app.add_teacher(&phone, "admin").await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.message_server)
        .await;

    let otp = app.request_otp_and_extract(&phone).await;
    let body = VerifyOtpBody::new(phone, &otp);
    let response = app.send_verify_otp_req(&body).await;

    assert!(response.status().is_success());

    let resp_body: Value = response.json().await.expect("failed to parse json");

    assert!(resp_body.get("token").is_some());

    let token = resp_body["token"].as_str().expect("token is not a string");

    assert!(!token.is_empty());
}

#[actix::test]
async fn doesnt_return_jwt_for_invalid() {
    let app = spawn_app().await;
    let phone = "1234567890";
    let body = VerifyOtpBody::new(phone, "123456");
    let response = app.send_verify_otp_req(&body).await;

    assert_eq!(response.status().as_u16(), 401);

    let resp_body: Value = response.json().await.expect("failed to parse json");

    assert!(resp_body.get("token").is_none());
}

#[actix::test]
async fn only_latest_otp_should_verify_2_try() {
    let app = spawn_app().await;
    let phone = "1234567890";
    app.add_teacher(&phone, "admin").await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1..)
        .mount(&app.message_server)
        .await;

    let otp1 = app.request_otp_and_extract(phone).await;
    app.rewind_latest_otp_created_at(phone).await;
    let otp2 = app.request_otp_and_extract(phone).await;

    let res_old = app
        .send_verify_otp_req(&VerifyOtpBody::new(phone, &otp1))
        .await;
    let res_new = app
        .send_verify_otp_req(&VerifyOtpBody::new(phone, &otp2))
        .await;

    assert_eq!(res_old.status(), 401);
    assert_eq!(res_new.status(), 200);
}

#[actix::test]
async fn only_latest_otp_should_verify_3_try() {
    let app = spawn_app().await;
    let phone = "1234567890";
    app.add_teacher(&phone, "admin").await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1..)
        .mount(&app.message_server)
        .await;

    let otp1 = app.request_otp_and_extract(phone).await;
    app.rewind_latest_otp_created_at(phone).await;
    let otp2 = app.request_otp_and_extract(phone).await;
    app.rewind_latest_otp_created_at(phone).await;
    let otp3 = app.request_otp_and_extract(phone).await;

    let res_old = app
        .send_verify_otp_req(&VerifyOtpBody::new(phone, &otp1))
        .await;
    let res_old1 = app
        .send_verify_otp_req(&VerifyOtpBody::new(phone, &otp2))
        .await;
    let res_new = app
        .send_verify_otp_req(&VerifyOtpBody::new(phone, &otp3))
        .await;

    assert_eq!(res_old.status(), 401);
    assert_eq!(res_old1.status(), 401);
    assert_eq!(res_new.status(), 200);
}

#[actix::test]
async fn after_trying_3_times_user_should_be_ban() {
    let app = spawn_app().await;
    let phone = "1234567890";
    app.add_teacher(&phone, "admin").await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1..)
        .mount(&app.message_server)
        .await;
    let body = LoginReqBody::new(phone);
    app.send_login_req(&body).await;
    app.rewind_latest_otp_created_at(phone).await;
    app.send_login_req(&body).await;
    app.rewind_latest_otp_created_at(phone).await;
    app.send_login_req(&body).await;
    app.rewind_latest_otp_created_at(phone).await;
    let res_ban = app.send_login_req(&body).await;

    let count = app.get_otp_attempts(&phone).await;

    dbg!(count);

    assert_eq!(res_ban.status(), 403);
}

#[actix::test]
async fn responds_with_401_for_invalid_otp() {
    let app = spawn_app().await;
    let phone = "1234567890";
    app.add_teacher(&phone, "admin").await;

    let send_login_req_body = LoginReqBody::new(&phone);

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.message_server)
        .await;

    app.send_login_req(&send_login_req_body).await;

    let body = VerifyOtpBody::new(phone, "123456");

    let response = app.send_verify_otp_req(&body).await;

    assert_eq!(response.status().as_u16(), 401)
}

#[actix::test]
async fn multiple_requests_should_increase_attempts_count() {
    let app = spawn_app().await;
    let phone = "1234567890";
    app.add_teacher(&phone, "admin").await;
    let send_login_req_body = LoginReqBody::new(&phone);
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1..)
        .mount(&app.message_server)
        .await;

    // sent once
    app.send_login_req(&send_login_req_body).await;
    // rewinded 30 seconds
    app.rewind_latest_otp_created_at(&phone).await;
    //sent second time
    app.send_login_req(&send_login_req_body).await;
    let count = app.get_otp_attempts(&phone).await;
    assert_eq!(count, 2)
}

#[actix::test]
async fn instant_request_should_return_429() {
    let app = spawn_app().await;
    let phone = "1234567890";
    app.add_teacher(&phone, "admin").await;
    let send_login_req_body = LoginReqBody::new(&phone);
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1..)
        .mount(&app.message_server)
        .await;

    app.send_login_req(&send_login_req_body).await;
    let res = app.send_login_req(&send_login_req_body).await;
    assert_eq!(res.status().as_u16(), 429)
}

#[actix::test]
pub async fn return_forbidden_if_user_for_banned_verify() {
    let app = spawn_app().await;
    let phone = "1234567890";
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

    let body = VerifyOtpBody::new(phone, "123456");
    let response = app.send_verify_otp_req(&body).await;
    assert_eq!(response.status().as_u16(), 403)
}

#[actix::test]
pub async fn return_unautharised_if_no_is_not_registered() {
    let app = spawn_app().await;
    let phone = "1234567890";
    let body = VerifyOtpBody::new(phone, "123456");
    let response = app.send_verify_otp_req(&body).await;
    assert_eq!(response.status().as_u16(), 401)
}
