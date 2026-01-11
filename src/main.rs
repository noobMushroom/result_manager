use result_management::configuration::get_configuration;
use result_management::startup::Application;
use result_management::telemetry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber =
        telemetry::get_subscriber("result manager".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration");
    let application = Application::build(&configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
