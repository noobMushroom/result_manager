use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use std::net::TcpListener;

mod routes;
pub mod configuration;

use crate::routes::health_check::health;

pub fn run(listen: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health)
    }).listen(listen)?.run();

    Ok(server)
}