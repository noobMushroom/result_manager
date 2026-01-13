use crate::helpers::{get_grade_id, mock_server, spawn_app};
use chrono::NaiveDate;
use serde_json::Value;

#[derive(serde::Serialize)]
pub struct AddStudentdBody {
    pub name: String,
    pub date_of_birth: String,
    pub admission_no: u16,
    pub father_name: String,
    pub grade: String,
}

impl AddStudentdBody {
    fn new(
        name: &str,
        date_of_birth: &str,
        admission_no: u16,
        father_name: &str,
        grade: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            date_of_birth: date_of_birth.to_string(),
            admission_no,
            father_name: father_name.to_string(),
            grade: grade.to_string(),
        }
    }
}

fn get_valid_phone<'a>() -> &'a str {
    "1234567890"
}

#[actix::test]
pub async fn register_student_returns_200_valid_data() {
    let app = spawn_app().await;
    mock_server(&app.message_server).await;
    let grade = "LKG";
    let body = AddStudentdBody::new("student", "12-12-2014", 12, "daddy", grade);
    let token = app.get_token(get_valid_phone()).await;
    let response = app.add_student(&body, &token).await;

    let saved = sqlx::query!(
        "SELECT name, date_of_birth, admission_no, father_name, grade_id FROM students"
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("failed to fetch new subscription.");

    let grade_id = get_grade_id("LKG", &app.db_pool).await;

    assert_eq!(saved.name, String::from("student"));
    assert_eq!(
        saved.date_of_birth,
        NaiveDate::parse_from_str("12-12-2014", "%d-%m-%Y").unwrap()
    );
    assert_eq!(saved.father_name, String::from("daddy"));
    assert_eq!(saved.admission_no, 12);
    assert_eq!(saved.grade_id, grade_id);

    assert_eq!(200, response.status().as_u16());
}

#[actix::test]
async fn register_fails_if_admission_no_exists() {
    let app = spawn_app().await;

    mock_server(&app.message_server).await;

    let body = AddStudentdBody::new("student", "12-12-2025", 12, "daddy", "LKG");

    let token = app.get_token(get_valid_phone()).await;
    app.add_student(&body, &token).await;

    let response = app.add_student(&body, &token).await;

    assert_eq!(response.status().as_u16(), 409);

    let resp_body: Value = response.json().await.expect("failed to get json");
    assert_eq!(resp_body["error"], "admission number already exists");

    let count = app.get_row_count_students().await;

    assert_eq!(count, Some(1));
}

#[actix::test]
async fn register_fails_for_invalid_grade() {
    let app = spawn_app().await;

    mock_server(&app.message_server).await;
    let body = AddStudentdBody::new("student", "12-12-2023", 99, "daddy", "INVALID");
    let token = app.get_token(get_valid_phone()).await;
    let response = app.add_student(&body, &token).await;
    assert_eq!(response.status().as_u16(), 400);
    let resp_body: Value = response.json().await.expect("failed to get json");
    assert_eq!(resp_body["error"], "invalid grade");

    let count = app.get_row_count_students().await;
    assert_eq!(count, Some(0));
}

#[actix::test]
async fn register_fails_for_invalid_dob_format() {
    let app = spawn_app().await;

    mock_server(&app.message_server).await;

    let body = AddStudentdBody::new("student", "2023-12-10", 1, "daddy", "LKG");
    let token = app.get_token(get_valid_phone()).await;
    let response = app.add_student(&body, &token).await;

    assert_eq!(response.status().as_u16(), 400);

    let resp_body: Value = response.json().await.expect("failed to get json");
    assert_eq!(resp_body["error"], "invalid date of birth");

    let count = app.get_row_count_students().await;

    assert_eq!(count, Some(0));
}

#[actix::test]
async fn register_fails_for_invalid_name() {
    let app = spawn_app().await;

    mock_server(&app.message_server).await;
    let body = AddStudentdBody::new("student eauua .....", "23-12-2010", 1, "daddy", "LKG");
    let token = app.get_token(get_valid_phone()).await;
    let response = app.add_student(&body, &token).await;

    assert_eq!(response.status().as_u16(), 400);

    let resp_body: Value = response.json().await.expect("failed to get json");
    assert_eq!(resp_body["error"], "invalid name");

    let count = app.get_row_count_students().await;

    assert_eq!(count, Some(0));
}

#[actix::test]
async fn register_returns_unauthoriseed_for_invalid_token() {
    let app = spawn_app().await;

    let body = AddStudentdBody::new("student", "23-12-2010", 1, "daddy", "LKG");
    let token = "Invalid token";
    let response = app.add_student(&body, &token).await;

    assert_eq!(response.status().as_u16(), 401);

    let count = app.get_row_count_students().await;

    assert_eq!(count, Some(0));
}

#[actix::test]
async fn register_returns_unauthoriseed_for_without_token() {
    let app = spawn_app().await;

    let body = AddStudentdBody::new("student", "23-12-2010", 1, "daddy", "LKG");
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/student/add_student", &app.address))
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("failed to execute request.");

    assert_eq!(response.status().as_u16(), 401);

    let count = app.get_row_count_students().await;

    assert_eq!(count, Some(0));
}
