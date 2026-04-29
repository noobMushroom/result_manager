use serde_json::Value;

use crate::{
    helpers::{mock_server, spawn_app},
    teacher_login::LoginReqBody,
};

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
    let phone = app.test_user.get_phone();
    mock_server(&app.message_server).await;
    let otp = app.request_otp_and_extract(phone).await;
    let body = VerifyOtpBody::new(phone, &otp);
    let response = app.send_verify_otp_req(&body).await;
    assert_eq!(response.status().as_u16(), 200);
}

#[actix::test]
async fn responds_with_jwt_token_for_successful_login() {
    let app = spawn_app().await;
    let phone = app.test_user.get_phone();
    mock_server(&app.message_server).await;
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
async fn returns_400_if_otp_trying_to_verify_without_requesting_otp() {
    let app = spawn_app().await;
    let phone = app.test_user.get_phone();
    println!("{}", phone);
    let body = VerifyOtpBody::new(phone, "123456");
    let response = app.send_verify_otp_req(&body).await;

    assert_eq!(response.status().as_u16(), 400);

    let resp_body: Value = response.json().await.expect("failed to parse json");

    assert!(resp_body.get("token").is_none());
}

#[actix::test]
async fn only_latest_otp_should_verify_2_try() {
    let app = spawn_app().await;
    let phone = app.test_user.get_phone();
    mock_server(&app.message_server).await;

    let otp1 = app.request_otp_and_extract(phone).await;
    app.expire_cooldown_time(phone).await;
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
    let phone = app.test_user.get_phone();
    mock_server(&app.message_server).await;
    let otp1 = app.request_otp_and_extract(phone).await;
    app.expire_cooldown_time(phone).await;
    let otp2 = app.request_otp_and_extract(phone).await;
    app.expire_cooldown_time(phone).await;
    let otp3 = app.request_otp_and_extract(phone).await;

    let res_old = app
        .send_verify_otp_req(&VerifyOtpBody::new(phone, &otp1))
        .await;

    println!("{}", res_old.status());
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
async fn after_requesting_3_times_user_should_be_ban() {
    let app = spawn_app().await;
    let phone = app.test_user.get_phone();
    mock_server(&app.message_server).await;
    let body = LoginReqBody::new(phone);

    for _ in 0..3 {
        app.send_login_req(&body).await;
        app.expire_cooldown_time(phone).await;
    }

    let res_ban = app.send_login_req(&body).await;

    let count = app.get_otp_requests(&phone).await;

    assert!(count > 3);

    assert_eq!(res_ban.status(), 403);
}

#[actix::test]
async fn responds_with_401_for_invalid_otp() {
    let app = spawn_app().await;
    let phone = app.test_user.get_phone();

    let send_login_req_body = LoginReqBody::new(&phone);

    mock_server(&app.message_server).await;

    app.send_login_req(&send_login_req_body).await;

    let body = VerifyOtpBody::new(phone, "123456");

    let response = app.send_verify_otp_req(&body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[actix::test]
async fn attempting_after_5_wrong_attempt_user_should_be_banned() {
    let app = spawn_app().await;
    let phone = app.test_user.get_phone();
    mock_server(&app.message_server).await;

    let otp = app.request_otp_and_extract(phone).await;

    for _ in 0..=5 {
        app.send_verify_otp_req(&VerifyOtpBody::new(phone, "123456"))
            .await;
    }

    let count = app.get_otp_attempts(&phone).await;
    assert!(count > 5);

    let res = app
        .send_verify_otp_req(&VerifyOtpBody::new(phone, &otp))
        .await;

    assert_eq!(res.status().as_u16(), 403);
}

#[actix::test]
async fn attempting_5th_time_user_shouldnt_be_banned() {
    let app = spawn_app().await;
    let phone = app.test_user.get_phone();
    mock_server(&app.message_server).await;

    let otp = app.request_otp_and_extract(phone).await;

    for _ in 0..5 {
        app.send_verify_otp_req(&VerifyOtpBody::new(phone, "123456"))
            .await;
    }
    let res = app
        .send_verify_otp_req(&VerifyOtpBody::new(phone, &otp))
        .await;
    assert_eq!(res.status().as_u16(), 200);
}

#[actix::test]
async fn instant_request_should_return_429() {
    let app = spawn_app().await;
    let phone = app.test_user.get_phone();
    let send_login_req_body = LoginReqBody::new(&phone);
    mock_server(&app.message_server).await;
    app.send_login_req(&send_login_req_body).await;
    let res = app.send_login_req(&send_login_req_body).await;
    assert_eq!(res.status().as_u16(), 429);
}
