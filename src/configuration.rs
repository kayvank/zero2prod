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
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

pub enum Environment {
    Local,
    Production,
}
impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error=String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok( Environment::Local),
            "production" => Ok( Environment::Production),
            other => Err (format! (
                "{} is not a supported environment. Use either `local` or `production`.", other))
        }
    }
}

#[derive(serde:: Deserialize)]
pub struct Settings{
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let path = std::env::current_dir().expect("Failed to dtermine the current directory.");
    let path = path.join("configuration");
    settings.merge(config::File::from(path.join("base")).required(true))?;

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");
    settings.merge(
        config::File::from(path.join(environment.as_str())).required(true))?;
    settings.try_into()

}
