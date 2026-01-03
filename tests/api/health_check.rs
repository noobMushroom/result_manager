use crate::helpers::spawn_app;
#[actix::test]
async fn health_check() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let res = client
        .get(&format!("{}/health", &app.address))
        .send()
        .await
        .expect("Can't retrieve client");
    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}
