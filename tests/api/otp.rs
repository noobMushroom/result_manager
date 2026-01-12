use chrono::{Duration, Utc};
use wiremock::{Mock, ResponseTemplate, matchers::method};

use crate::{
    helpers::{extract_otp, spawn_app},
    teacher_login::LoginReqBody,
};

#[derive(serde::Serialize)]
pub struct VerifyOtpBody {
    pub phone: String,
    pub otp: String,
}

impl VerifyOtpBody {
    fn new(phone: &str, otp: &str) -> Self {
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
    app.add_teacher(&phone).await;

    let send_login_req_body = LoginReqBody::new(&phone);

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.message_server)
        .await;

    let res1 = app.send_login_req(&send_login_req_body).await;
    dbg!(&res1);

    let otp_req = &app.message_server.received_requests().await.unwrap()[0].body;
    let body = String::from_utf8(otp_req.to_vec()).unwrap();
    let otp = extract_otp(&body).unwrap();
    println!("{}", otp);

    let body = VerifyOtpBody::new(phone, &otp);

    let response = app.send_verify_otp_req(&body).await;

    assert_eq!(response.status().as_u16(), 200)
}

#[actix::test]
async fn responds_with_401_for_invalid_otp() {
    let app = spawn_app().await;
    let phone = "1234567890";
    app.add_teacher(&phone).await;

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
    app.add_teacher(&phone).await;

    let send_login_req_body = LoginReqBody::new(&phone);

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.message_server)
        .await;

    app.send_login_req(&send_login_req_body).await;

    let otp_req = &app.message_server.received_requests().await.unwrap()[0].body;
    let body = String::from_utf8(otp_req.to_vec()).unwrap();
    let otp = extract_otp(&body).unwrap();
    let body = VerifyOtpBody::new(phone, &otp);
    let response = app.send_verify_otp_req(&body).await;
    assert_eq!(response.status().as_u16(), 200)
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
