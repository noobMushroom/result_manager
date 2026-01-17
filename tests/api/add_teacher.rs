use crate::helpers::{mock_server, spawn_app};

#[derive(serde::Serialize)]
pub struct AddTeacherBody {
    pub name: String,
    pub phone: String,
    pub role: String,
}

impl AddTeacherBody {
    pub fn generate() -> Self {
        Self {
            name: name(),
            phone: phone(),
            role: role(),
        }
    }
}

fn name() -> String {
    "test teacher".to_string()
}

fn role() -> String {
    "admin".to_string()
}

fn phone() -> String {
    "2234567890".to_string()
}

#[actix::test]
pub async fn add_teacher_should_return_200_for_valid_data() {
    let app = spawn_app().await;
    let add_teacher_body = AddTeacherBody::generate();
    let response = app
        .send_add_teacher_req(&add_teacher_body, &app.test_user.token)
        .await;

    assert_eq!(response.status().as_u16(), 200);

    let saved =
        sqlx::query!("SELECT  name, phone_no, role FROM teachers where phone_no='2234567890'")
            .fetch_one(&app.db_pool)
            .await
            .expect("failed to fetch new subscription.");

    assert_eq!(saved.name, name());
    assert_eq!(saved.phone_no, phone());
    assert_eq!(saved.role, role());
}

#[actix::test]
pub async fn add_teacher_fails_for_invalid_name() {
    let app = spawn_app().await;
    let mut add_teacher_body = AddTeacherBody::generate();
    add_teacher_body.name = "asuhau ....".to_string();
    let response = app
        .send_add_teacher_req(&add_teacher_body, &app.test_user.token)
        .await;

    assert_eq!(response.status().as_u16(), 400);
}

#[actix::test]
pub async fn add_teacher_fails_for_invalid_phone() {
    let app = spawn_app().await;
    let mut add_teacher_body = AddTeacherBody::generate();
    add_teacher_body.phone = "asuhau ....".to_string();
    let response = app
        .send_add_teacher_req(&add_teacher_body, &app.test_user.token)
        .await;

    assert_eq!(response.status().as_u16(), 400);
}

#[actix::test]
pub async fn add_teacher_fails_for_invalid_role() {
    let app = spawn_app().await;
    let mut add_teacher_body = AddTeacherBody::generate();
    add_teacher_body.role = "asuhau ....".to_string();
    let response = app
        .send_add_teacher_req(&add_teacher_body, &app.test_user.token)
        .await;

    assert_eq!(response.status().as_u16(), 400);
}

#[actix::test]
pub async fn add_teacher_should_return_conflict_for_adding_same_number() {
    let app = spawn_app().await;
    let add_teacher_body = AddTeacherBody::generate();
    app.send_add_teacher_req(&add_teacher_body, &app.test_user.token)
        .await;
    let response = app
        .send_add_teacher_req(&add_teacher_body, &app.test_user.token)
        .await;

    assert_eq!(response.status().as_u16(), 409);
}

#[actix::test]
pub async fn request_without_token_return_unauthorised() {
    let app = spawn_app().await;
    let body = AddTeacherBody::generate();
    let response = app
        .api_client
        .post(format!("{}/teacher/add-teacher", &app.address))
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("failed to execute request.");

    assert_eq!(response.status().as_u16(), 401);
}

#[actix::test]
pub async fn request_with_wrong_token_return_401() {
    let app = spawn_app().await;
    let body = AddTeacherBody::generate();
    let response = app
        .api_client
        .post(format!("{}/teacher/add-teacher", &app.address))
        .bearer_auth("wrong token")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("failed to execute request.");

    assert_eq!(response.status().as_u16(), 401);
}

#[actix::test]
pub async fn request_with_basic_token_return_401() {
    let app = spawn_app().await;
    mock_server(&app.message_server).await;
    let add_teacher_body = AddTeacherBody::generate();
    let token = app.get_token("2233567789", "teacher").await;
    let response = app.send_add_teacher_req(&add_teacher_body, &token).await;
    assert_eq!(response.status().as_u16(), 403);
}
