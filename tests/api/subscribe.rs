use crate::helpers::spawn_app;

#[actix::test]
pub async fn register_returns_200_valid_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = r#"{"username":"some", "phone": "0123456789"}"#;
    let response = client
        .post(format!("{}/subscribe", &app.address))
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await
        .expect("failed to execute request.");

    let saved = sqlx::query!("SELECT name, phone_no FROM subscriptions ")
        .fetch_one(&app.db_pool)
        .await
        .expect("failed to fetch new subscription.");

    assert_eq!(saved.name, String::from("some"));
    assert_eq!(saved.phone_no, String::from("0123456789"));
    assert_eq!(200, response.status().as_u16());
}

#[actix::test]
pub async fn register_returns_400_missing_data() {
    let test_cases = vec![
        (r#"{"name": "some"}"#, "missing phone"),
        (r#"{"phone": "some"}"#, "missing name"),
        (r#"{}"#, "missing both"),
    ];
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscribe", &app.address))
            .header("content-type", "application/json")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute request.");

        assert_eq!(400, response.status().as_u16(),
                   "The api did not fail with 400.\n{}", error_message);
    }
}



