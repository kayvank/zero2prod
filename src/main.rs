use secrecy::ExposeSecret;
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration,
                startup::run,
                telemetry::{get_subscriber, init_subscriber}
};
use sqlx::PgPool;
use tracing_log::LogTracer;

#[tokio::main]
async fn main() -> std::io::Result<()>{

    LogTracer::init().expect("Failed to set logger"); // redirct all logs to to tracer

    init_subscriber (
        get_subscriber("zero2prod".into(), "info".into(), std::io::stdout)
    );

    let configuration = get_configuration().expect("Failed to read application configurations.");
    let connection_pool = PgPool::connect_lazy(
        &configuration.database.connection_string().expose_secret())
        .expect("Failed to connect Postgres.");

    let address = format!("{}:{}",
                          configuration.application.host,
                          configuration.application.port);
    let listener = TcpListener::bind(address)
        .expect("Failerd to bind to random port");
    run(listener, connection_pool)?.await
}
