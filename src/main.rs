use result_management::configuration::get_configuration;
use result_management::startup::run;
use result_management::telemetry;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber =
        telemetry::get_subscriber("result manager".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
