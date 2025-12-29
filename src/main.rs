use result_management::configuration::get_configuration;
use result_management::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("{}:{}", configuration.database.host, configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
