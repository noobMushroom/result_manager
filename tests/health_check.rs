use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = result_management::run(listener).expect("failed to start server");
    let _ = actix::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
#[actix::test]
async fn health_check() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let res = client
        .get(&format!("{}/health", address))
        .send()
        .await
        .expect("Can't retrieve client");
    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}