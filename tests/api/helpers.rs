use crate::add_students::AddStudentdBody;
use crate::otp::VerifyOtpBody;
use crate::teacher_login::LoginReqBody;
use chrono::{Duration, Utc};
use once_cell::sync::Lazy;
use reqwest::Response;
use result_management::configuration::{DatabaseSettings, get_configuration};
use result_management::startup::{Application, get_connection_pool};
use result_management::telemetry::{get_subscriber, init_subscriber};
use serde_json::Value;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::MockServer;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub message_server: MockServer,
}

pub async fn get_grade_id(grade: &str, db_pool: &PgPool) -> Uuid {
    let record = sqlx::query!(r#"SELECT id FROM grades WHERE name = $1"#, grade)
        .fetch_one(db_pool)
        .await
        .expect("failed to fetch id");

    record.id
}

impl TestApp {
    pub async fn add_student(&self, body: &AddStudentdBody) -> Response {
        let client = reqwest::Client::new();
        client
            .post(format!("{}/student/add_student", &self.address))
            .header("content-type", "application/json")
            .json(body)
            .send()
            .await
            .expect("failed to execute request.")
    }

    pub async fn get_row_count_students(&self) -> Option<i64> {
        sqlx::query_scalar!("SELECT COUNT(*) as count FROM students")
            .fetch_one(&self.db_pool)
            .await
            .expect("Failed to get student")
    }

    pub async fn get_row_count_otp(&self) -> Option<i64> {
        sqlx::query_scalar!("SELECT COUNT(*) as count FROM otp_requests")
            .fetch_one(&self.db_pool)
            .await
            .expect("Failed to get otp row")
    }

    pub async fn send_login_req(&self, body: &LoginReqBody) -> Response {
        let client = reqwest::Client::new();
        client
            .post(format!("{}/auth/login", &self.address))
            .header("content-type", "application/json")
            .json(body)
            .send()
            .await
            .expect("failed to execute request.")
    }

    pub async fn send_verify_otp_req(&self, body: &VerifyOtpBody) -> Response {
        let client = reqwest::Client::new();
        client
            .post(format!("{}/auth/verify", &self.address))
            .header("content-type", "application/json")
            .json(body)
            .send()
            .await
            .expect("failed to execute request.")
    }

    pub async fn add_teacher(&self, phone: &str) {
        sqlx::query!(
            r#"
        INSERT INTO teachers (id, name, phone_no, role, subscribed_at)
        VALUES($1, $2, $3, $4, $5)
        "#,
            Uuid::new_v4(),
            "some",
            phone.to_string(),
            "admin",
            Utc::now()
        )
        .execute(&self.db_pool)
        .await
        .expect("failed to execute query");
    }

    pub async fn rewind_latest_otp_created_at(&self, phone: &str) {
        let duration = Utc::now() - Duration::seconds(32);
        sqlx::query!(
            r#"
        UPDATE otp_requests
        SET created_at = $2
        WHERE id = (
            SELECT id
            FROM otp_requests
            WHERE phone_number = $1
            ORDER BY created_at DESC, id DESC
            LIMIT 1
        )
        "#,
            phone,
            duration
        )
        .execute(&self.db_pool)
        .await
        .expect("failed to change created at time");
    }

    pub async fn get_otp_attempts(&self, phone: &str) -> i32 {
        let otp = sqlx::query!(
            r#"
        SELECT attempts
        FROM otp_requests
        WHERE phone_number = $1
          AND expires_at > NOW()
        ORDER BY created_at DESC
        LIMIT 1
        "#,
            phone
        )
        .fetch_one(&self.db_pool)
        .await
        .expect("failed to get otp");
        otp.attempts
    }

    pub async fn request_otp_and_extract(&self, phone: &str) -> String {
        let body = LoginReqBody::new(phone);
        self.send_login_req(&body).await;

        let requests = self.message_server.received_requests().await.unwrap();
        let last = requests.last().expect("no OTP request found");

        let raw = String::from_utf8(last.body.clone()).unwrap();
        extract_otp(&raw).expect("OTP not found")
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let message_server = MockServer::start().await;

    let configuration = {
        let mut c = get_configuration().expect("failed to read configuration");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c.message_client.base_url = message_server.uri();
        c
    };

    configure_database(&configuration.database).await;

    let server = Application::build(configuration.clone())
        .await
        .expect("failed to build application");
    let address = format!("http://127.0.0.1:{}", server.port());
    let _ = actix::spawn(server.run_until_stopped());
    TestApp {
        address,
        db_pool: get_connection_pool(&configuration.database),
        message_server,
    }
}
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("failed to connect to the database");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("failed to create database page");
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("failed to run migrations");

    connection_pool
}

pub fn extract_otp(body: &str) -> Option<String> {
    let v: Value = serde_json::from_str(body).ok()?;
    let message = v.get("message")?.as_str()?;
    message
        .split_whitespace()
        .find(|word| word.chars().all(|c| c.is_ascii_digit()))
        .map(|s| s.to_string())
}
