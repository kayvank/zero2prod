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
    let connection_pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)
        .expect("Failerd to bind to random port");
    run(listener, connection_pool)?.await
}
