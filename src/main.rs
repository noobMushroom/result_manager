use result_management::configuration::get_configuration;
use result_management::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;
use result_management::telemetry;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("result manager".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string()).await.expect("Failed to connect to Postgres");
    let address = format!("{}:{}", configuration.database.host, configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
