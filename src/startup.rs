use crate::routes::health_check::health;
use crate::routes::students::register::register_student;
use crate::routes::users::register::subscribe;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listen: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health)
            .service(subscribe)
            .service(register_student)
            .app_data(connection.clone())
    })
    .listen(listen)?
    .run();
    Ok(server)
}
