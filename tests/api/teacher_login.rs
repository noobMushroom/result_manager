use chrono::Utc;
use uuid::Uuid;

use crate::helpers::spawn_app;

#[actix::test]
pub async fn register_teacher_returns_200_invalid_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let phone = "1234567890";
    let body = serde_json::json!({
        "phone": phone
    })
    .to_string();
    let response = client
        .post(format!("{}/auth/login", &app.address))
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await
        .expect("failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[actix::test]
pub async fn add_otp_for_valid_phone() {
    let app = spawn_app().await;
    let phone = "1234567890";
    sqlx::query!(
        r#"
        INSERT INTO teachers (id, name, phone_no, role, subscribed_at)
        VALUES($1, $2, $3, $4, $5)
        "#,
        Uuid::new_v4(),
        "some",
        "1234567890",
        "admin",
        Utc::now()
    )
    .execute(&app.db_pool)
    .await
    .expect("failed to execute query");

    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "phone": phone
    })
    .to_string();
    let response = client
        .post(format!("{}/auth/login", &app.address))
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await
        .expect("failed to execute request.");

    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM otp_requests")
        .fetch_one(&app.db_pool)
        .await
        .expect("falied");

    assert_eq!(200, response.status().as_u16());
    assert_eq!(count, Some(1))
}

#[actix::test]
pub async fn doesnt_add_otp_for_invalid() {
    let app = spawn_app().await;
    let phone = "1234567892";
    sqlx::query!(
        r#"
        INSERT INTO teachers (id, name, phone_no, role, subscribed_at)
        VALUES($1, $2, $3, $4, $5)
        "#,
        Uuid::new_v4(),
        "some",
        "1234567890",
        "admin",
        Utc::now()
    )
    .execute(&app.db_pool)
    .await
    .expect("failed to execute query");

    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "phone": phone
    })
    .to_string();
    let response = client
        .post(format!("{}/auth/login", &app.address))
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await
        .expect("failed to execute request.");

    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM otp_requests")
        .fetch_one(&app.db_pool)
        .await
        .expect("falied");

    assert_eq!(200, response.status().as_u16());
    assert_eq!(count, Some(0))
}
