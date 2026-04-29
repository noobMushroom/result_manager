use crate::helpers::{mock_server, spawn_app};

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
    mock_server(&app.message_server).await;
    let phone = app.test_user.get_phone();
    let body = LoginReqBody::new(phone);
    let response = app.send_login_req(&body).await;
    assert_eq!(200, response.status().as_u16());
}
