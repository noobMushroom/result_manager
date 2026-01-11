use wiremock::{Mock, ResponseTemplate, matchers::method};

use crate::{helpers::spawn_app, teacher_login::LoginReqBody};

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

    app.send_login_req(&send_login_req_body).await;

    let otp_req = &app.message_server.received_requests().await.unwrap()[0];

    dbg!(otp_req);

    assert!(false)
}

#[actix::test]
async fn responds_with_401_for_invalid_otp() {
    todo!()
}
