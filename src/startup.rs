use crate::routes::health_check::health;
use crate::routes::users::register::subscribe;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listen: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .service(health)
            .service(subscribe)
            .app_data(connection.clone())
    }).listen(listen)?.run();
    Ok(server)
}
