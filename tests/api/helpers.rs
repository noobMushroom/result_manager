use std::net::TcpListener;

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = result_management::run(listener).expect("failed to start server");
    let _ = actix::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
