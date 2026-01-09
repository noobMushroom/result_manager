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
