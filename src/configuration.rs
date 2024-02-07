use secrecy::{Secret, ExposeSecret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::PgConnectOptions;

#[derive(serde:: Deserialize)]
pub struct DatabaseSettings{
    pub username: String,
    pub password: Secret<String>,
    pub host : String,
    pub database_name: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub require_ssl: bool,
}
impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!("postgres://{}:{}@{}:{}/{}",
                self.username,
                self.password.expose_secret(),
                self.host,
                self.port,
                self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!("postgres://{}:{}@{}:{}",
                self.username,
                self.password.expose_secret(),
                self.host,
                self.port
        ))
    }
}

#[derive(serde:: Deserialize)]
pub struct Settings{
    pub database: DatabaseSettings,
    pub application_port: u16,
}
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("config/local-settings"))?;
    settings.try_into()
}
