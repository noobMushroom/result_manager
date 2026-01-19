use crate::auth::middleware::{jwt_middleware, jwt_middleware_teacher};
use crate::configuration::{DatabaseSettings, Settings};
use crate::message_client::MessageClient;
use crate::routes::academics::get_assesment::get_assessment_scheme;
use crate::routes::academics::get_exam_types::get_exam_types;
use crate::routes::academics::get_grades::get_grades;
use crate::routes::academics::get_terms::get_terms;
use crate::routes::health_check::health;
use crate::routes::result::add_result::add_result;
use crate::routes::students::get_student::{get_students, seach_students};
use crate::routes::students::register::register_student;
use crate::routes::users::login::teacher_login;
use crate::routes::users::otp::verify_user_otp;
use crate::routes::users::register::add_teacher;
use actix_web::dev::Server;
use actix_web::middleware::from_fn;
use actix_web::{App, HttpServer, web};
use secrecy::SecretString;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, Box<dyn std::error::Error>> {
        let connection_pool = get_connection_pool(&configuration.database);
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let message_client = MessageClient::new(
            configuration.message_client.timeout(),
            configuration.message_client.base_url,
            configuration.message_client.api_key,
            configuration.message_client.device_id,
        )?;
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            message_client,
            configuration.jwt.secret_token(),
        )?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(10))
        .connect_lazy_with(configuration.with_db())
}
pub fn run(
    listen: TcpListener,
    db_pool: PgPool,
    message_client: MessageClient,
    jwt_secret: SecretString,
) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_pool);
    let message_client = web::Data::new(message_client);
    let jwt_secret = web::Data::new(jwt_secret);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(
                web::scope("/results")
                    .wrap(from_fn(jwt_middleware))
                    .service(add_result),
            )
            .service(
                web::scope("/academics")
                    .wrap(from_fn(jwt_middleware))
                    .service(get_grades)
                    .service(get_terms)
                    .service(get_exam_types)
                    .service(get_assessment_scheme),
            )
            .service(
                web::scope("/teacher")
                    .wrap(from_fn(jwt_middleware_teacher))
                    .service(add_teacher),
            )
            .service(
                web::scope("/auth")
                    .service(teacher_login)
                    .service(verify_user_otp),
            )
            .service(
                web::scope("/student")
                    .wrap(from_fn(jwt_middleware))
                    .service(get_students)
                    .service(register_student)
                    .service(seach_students),
            )
            .service(health)
            .app_data(connection.clone())
            .app_data(jwt_secret.clone())
            .app_data(message_client.clone())
    })
    .listen(listen)?
    .run();
    Ok(server)
}
