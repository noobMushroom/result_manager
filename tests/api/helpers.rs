use crate::add_students::AddStudentdBody;
use crate::add_teacher::AddTeacherBody;
use crate::otp::VerifyOtpBody;
use crate::teacher_login::LoginReqBody;
use chrono::{NaiveDate, Utc};
use fake::Fake;
use fake::faker::name::raw::Name;
use fake::locales::EN;
use once_cell::sync::Lazy;
use rand::Rng;
use redis::AsyncCommands;
use reqwest::Response;
use result_management::auth::jwt::generate_jwt;
use result_management::configuration::{DatabaseSettings, get_configuration};
use result_management::domain::roles::Role;
use result_management::redis::repo::RedisRepo;
use result_management::redis::utils::OtpTimers;
use result_management::routes::academics::get_assesment::{AssessmentBody, AssessmentResponse};
use result_management::routes::academics::get_grades::GradeBodyResponse;
use result_management::routes::academics::get_terms::TermsResponse;
use result_management::routes::result::add_result::AddMarksData;
use result_management::routes::students::get_student::SearchStudentsQuery;
use result_management::startup::{Application, get_connection_pool};
use result_management::telemetry::{get_subscriber, init_subscriber};
use secrecy::SecretString;
use serde_json::Value;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::matchers::method;
use wiremock::{Mock, MockServer, ResponseTemplate};

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

pub struct TestUser {
    pub id: Uuid,
    pub role: String,
    pub name: String,
    pub phone: String,
    pub token: String,
}

impl TestUser {
    pub fn generate(secret: &SecretString) -> Self {
        let id = Uuid::new_v4();
        let token = generate_jwt(id, Role::Admin, secret).unwrap();
        Self {
            id,
            role: "admin".to_string(),
            name: Name(EN).fake(),
            phone: generate_phone(),
            token,
        }
    }

    pub fn get_phone(&self) -> &str {
        &self.phone
    }

    pub async fn store(&self, pool: &PgPool) {
        sqlx::query!(
            r#"
        INSERT INTO teachers (id, name, phone_no, role, subscribed_at)
        VALUES($1, $2, $3, $4, $5)
        "#,
            self.id,
            self.name,
            self.phone,
            self.role,
            Utc::now()
        )
        .execute(pool)
        .await
        .expect("failed to execute query");
    }
}

pub fn generate_phone() -> String {
    (0..10)
        .map(|_| char::from(b'0' + rand::rng().random_range(0..10) as u8))
        .collect()
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub message_server: MockServer,
    pub redis: RedisRepo,
    pub api_client: reqwest::Client,
    pub test_user: TestUser,
}

pub async fn get_grade_id(grade: &str, db_pool: &PgPool) -> Uuid {
    let record = sqlx::query!(r#"SELECT id FROM grades WHERE name = $1"#, grade)
        .fetch_one(db_pool)
        .await
        .expect("failed to fetch id");

    record.id
}

impl TestApp {
    pub async fn add_student(&self, body: &AddStudentdBody, token: &str) -> Response {
        self.api_client
            .post(format!("{}/student/add_student", &self.address))
            .bearer_auth(token)
            .header("content-type", "application/json")
            .json(body)
            .send()
            .await
            .expect("failed to execute request.")
    }

    pub async fn get_assesment_scheme(&self, body: &AssessmentBody) -> Vec<AssessmentResponse> {
        let response = self
            .api_client
            .get(format!(
                "{}/academics/assessment-scheme?grade={}&term={}",
                self.address, body.grade, body.term
            ))
            .bearer_auth(&self.test_user.token)
            .send()
            .await
            .expect("failed to send request");

        response.json::<Vec<_>>().await.expect("failed to convert")
    }

    pub async fn get_students(&self, grade: &str, token: &str) -> Response {
        self.api_client
            .get(format!("{}/student/get_students/{}", &self.address, &grade))
            .bearer_auth(token)
            .send()
            .await
            .expect("failed to execute request.")
    }

    pub async fn search_student(&self, query: &SearchStudentsQuery, token: &str) -> Response {
        let mut request = self
            .api_client
            .get(format!("{}/student/search", self.address))
            .bearer_auth(token);

        if let Some(ref q) = query.q {
            request = request.query(&[("q", q)]);
        }

        if let Some(grade_id) = query.grade_id {
            request = request.query(&[("grade_id", grade_id.to_string())]);
        }

        if let Some(adm) = query.admission_no {
            request = request.query(&[("admission_no", adm)]);
        }

        request.send().await.expect("failed to execute request")
    }

    pub async fn get_term(&self, token: &str, term: &str) -> TermsResponse {
        let response = self
            .api_client
            .get(format!("{}/academics/terms", &self.address))
            .bearer_auth(&token)
            .send()
            .await
            .expect("failed to send request");

        let item = response
            .json::<Vec<TermsResponse>>()
            .await
            .expect("failed to get terrms");

        item.into_iter().find(|v| v.name == term).unwrap()
    }

    pub async fn get_grade(&self, token: &str, grade: &str) -> GradeBodyResponse {
        let response = self
            .api_client
            .get(format!("{}/academics/grades", &self.address))
            .bearer_auth(&token)
            .send()
            .await
            .expect("failed to send request");

        let item = response
            .json::<Vec<GradeBodyResponse>>()
            .await
            .expect("failed to get terrms");

        item.into_iter().find(|v| v.name == grade).unwrap()
    }

    pub async fn send_add_marks_request(&self, body: &AddMarksData, token: &str) -> Response {
        self.api_client
            .post(format!("{}/results/add_marks", &self.address))
            .bearer_auth(token)
            .header("content-type", "application/json")
            .json(body)
            .send()
            .await
            .expect("failed to execute request.")
    }

    pub async fn send_add_teacher_req(&self, body: &AddTeacherBody, token: &str) -> Response {
        self.api_client
            .post(format!("{}/teacher/add_teacher", &self.address))
            .bearer_auth(token)
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


    pub async fn send_login_req(&self, body: &LoginReqBody) -> Response {
        self.api_client
            .post(format!("{}/auth/login", &self.address))
            .header("content-type", "application/json")
            .json(body)
            .send()
            .await
            .expect("failed to execute request.")
    }

    pub async fn send_verify_otp_req(&self, body: &VerifyOtpBody) -> Response {
        self.api_client
            .post(format!("{}/auth/verify", &self.address))
            .header("content-type", "application/json")
            .json(body)
            .send()
            .await
            .expect("failed to execute request.")
    }

    pub async fn add_teacher(&self, phone: &str, role: &str) {
        sqlx::query!(
            r#"
        INSERT INTO teachers (id, name, phone_no, role, subscribed_at)
        VALUES($1, $2, $3, $4, $5)
        "#,
            Uuid::new_v4(),
            "some",
            phone.to_string(),
            role,
            Utc::now()
        )
        .execute(&self.db_pool)
        .await
        .expect("failed to execute query");
    }

    pub async fn add_student_to_db(&self, student: &AddStudentdBody, grade: Uuid) -> Uuid {
        let uuid = Uuid::new_v4();
        let date = NaiveDate::parse_from_str(&student.date_of_birth, "%d-%m-%Y").unwrap();
        sqlx::query!(
            r#"
            INSERT INTO students (id, grade_id, name, father_name, admission_no, date_of_birth)
            VALUES ($1, $2, $3, $4, $5, $6)
        "#,
            uuid,
            grade,
            student.name,
            student.father_name,
            student.admission_no,
            date
        )
        .execute(&self.db_pool)
        .await
        .expect("failed to execute query");
        uuid
    }

    pub async fn expire_cooldown_time(&self, phone: &str) {
        let key = OtpTimers::OtpCooldown(phone).key();
        let mut con = self.redis.con.clone();
        let _: () = con.expire(&key, -1).await.unwrap();
    }

    pub async fn get_otp_attempts(&self, phone: &str) -> i32 {
        let mut con = self.redis.con.clone();
        let key = OtpTimers::OtpAttempts(phone).key();

        let otp_requests: Option<i32> = con.get(&key).await.unwrap();

        otp_requests.unwrap_or(0)
    }

    pub async fn get_otp_requests(&self, phone: &str) -> i32 {
        let mut con = self.redis.con.clone();
        let key = OtpTimers::OtpRequests(phone).key();

        let otp_requests: Option<i32> = con.get(&key).await.unwrap();

        otp_requests.unwrap_or(0)
    }

    pub async fn request_otp_and_extract(&self, phone: &str) -> String {
        let body = LoginReqBody::new(phone);
        self.send_login_req(&body).await;

        let requests = self.message_server.received_requests().await.unwrap();
        let last = requests.last().expect("no OTP request found");

        let raw = String::from_utf8(last.body.clone()).unwrap();
        extract_otp(&raw).expect("OTP not found")
    }

    pub async fn get_token(&self, phone: &str, role: &str) -> String {
        self.add_teacher(&phone, role).await;
        let body = LoginReqBody::new(phone);
        self.send_login_req(&body).await;
        let requests = self.message_server.received_requests().await.unwrap();
        let last = requests.last().expect("no OTP request found");
        let raw = String::from_utf8(last.body.clone()).unwrap();
        let otp = extract_otp(&raw).expect("OTP not found");
        let verifyotpbodf = VerifyOtpBody::new(phone, &otp);
        let login_response = self.send_verify_otp_req(&verifyotpbodf).await;
        let resp_body: Value = login_response.json().await.expect("failed to parse json");
        let token = resp_body["token"].as_str().expect("token is not a string");
        token.to_string()
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let message_server = MockServer::start().await;
    let api_client = reqwest::Client::builder()
        .pool_max_idle_per_host(5)
        .build()
        .expect("failed to build api client");

    let configuration = {
        let mut c = get_configuration().expect("failed to read configuration");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c.message_client.base_url = message_server.uri();
        c.redis.database_index = 5;
        c
    };

    let redis = RedisRepo::new(&configuration.redis.connection_string())
        .await
        .unwrap();

    configure_database(&configuration.database).await;
    let server = Application::build(configuration.clone())
        .await
        .expect("failed to build application");

    let db_pool = get_connection_pool(&configuration.database);

    let test_user = TestUser::generate(&configuration.jwt.secret_token());

    test_user.store(&db_pool).await;

    let address = format!("http://127.0.0.1:{}", server.port());
    let _ = actix::spawn(server.run_until_stopped());
    TestApp {
        address,
        db_pool,
        redis,
        message_server,
        api_client,
        test_user,
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

pub async fn mock_server(server: &MockServer) {
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1..)
        .mount(server)
        .await
}
