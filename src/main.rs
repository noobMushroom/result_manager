use result_management::configuration::get_configuration;
use result_management::startup::run;
use sqlx::{Connection, PgPool};
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string()).await.expect("Failed to connect to Postgres");
    let address = format!("{}:{}", configuration.database.host, configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
