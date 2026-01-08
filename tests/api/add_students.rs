use crate::helpers::{get_grade_id, spawn_app};
use chrono::NaiveDate;
use serde_json::Value;

#[actix::test]
pub async fn register_student_returns_200_valid_data() {
    let app = spawn_app().await;
    let grade = "LKG";
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "name": "student",
        "date_of_birth": "12-12-2025",
        "admission_no": 12,
        "father_name": "daddy",
        "grade": grade
    })
    .to_string();
    let response = client
        .post(format!("{}/add_student", &app.address))
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await
        .expect("failed to execute request.");

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
        NaiveDate::parse_from_str("12-12-2025", "%d-%m-%Y").unwrap()
    );
    assert_eq!(saved.father_name, String::from("daddy"));
    assert_eq!(saved.admission_no, 12);
    assert_eq!(saved.grade_id, grade_id);

    assert_eq!(200, response.status().as_u16());
}

#[actix::test]
async fn register_fails_if_admission_no_exists() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "name": "student",
        "date_of_birth": "12-12-2025",
        "admission_no": 12,
        "father_name": "daddy",
        "grade": "LKG"
    })
    .to_string();

    client
        .post(format!("{}/add_student", &app.address))
        .header("content-type", "application/json")
        .body(body.clone())
        .send()
        .await
        .expect("failed to execute request.");

    let response = client
        .post(format!("{}/add_student", &app.address))
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await
        .expect("failed to execute request.");

    assert_eq!(response.status().as_u16(), 409);

    let resp_body: Value = response.json().await.expect("failed to get json");
    assert_eq!(resp_body["error"], "admission number already exists");

    let count = sqlx::query!("SELECT COUNT(*) as count FROM students")
        .fetch_one(&app.db_pool)
        .await
        .unwrap()
        .count
        .unwrap();

    assert_eq!(count, 1);
}

#[actix::test]
async fn register_fails_for_invalid_grade() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "name": "student",
        "date_of_birth": "02-12-2023",
        "admission_no": 99,
        "father_name": "dad",
        "grade": "INVALID"
    })
    .to_string();

    let response = client
        .post(format!("{}/add_student", &app.address))
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 400);

    let resp_body: Value = response.json().await.expect("failed to get json");

    assert_eq!(resp_body["error"], "invalid grade");

    let count = sqlx::query!("SELECT COUNT(*) as count FROM students")
        .fetch_one(&app.db_pool)
        .await
        .unwrap()
        .count
        .unwrap();

    assert_eq!(count, 0);
}

#[actix::test]
async fn register_fails_for_invalid_dob_format() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = r#"
    {
        "name": "student",
        "date_of_birth": "2023-12-10",
        "admission_no": 1,
        "father_name": "dad",
        "grade": "LKG"
    }
    "#;

    let response = client
        .post(format!("{}/add_student", &app.address))
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 400);

    let resp_body: Value = response.json().await.expect("failed to get json");
    assert_eq!(resp_body["error"], "invalid date of birth");

    let count = sqlx::query!("SELECT COUNT(*) as count FROM students")
        .fetch_one(&app.db_pool)
        .await
        .unwrap()
        .count
        .unwrap();

    assert_eq!(count, 0);
}

#[actix::test]
async fn register_fails_for_invalid_name() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = r#"
    {
        "name": "student ...,,",
        "date_of_birth": "23-12-2223",
        "admission_no": 1,
        "father_name": "dad",
        "grade": "LKG"
    }
    "#;

    let response = client
        .post(format!("{}/add_student", &app.address))
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 400);

    let resp_body: Value = response.json().await.expect("failed to get json");
    assert_eq!(resp_body["error"], "invalid name");

    let count = sqlx::query!("SELECT COUNT(*) as count FROM students")
        .fetch_one(&app.db_pool)
        .await
        .unwrap()
        .count
        .unwrap();

    assert_eq!(count, 0);
}
