use secrecy::{ExposeSecret, SecretString};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> SecretString {
        SecretString::new(
            format!(
                "postgres://{}:{}@{}:{}/{}",
                self.username,
                self.password.expose_secret(),
                self.host,
                self.port,
                self.database_name
            )
            .into(),
        )
    }

    pub fn connection_string_without_db(&self) -> SecretString {
        SecretString::new(
            format!(
                "postgres://{}:{}@{}:{}",
                self.username,
                self.password.expose_secret(),
                self.host,
                self.port
            )
            .into(),
        )
    }
}

/// # Errors
///
/// Will return `Err` if `configuration.toml` does not exist or the user does not have
/// permission to read it.
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current dir");
    let config_directory = base_path.join("configuration");

    let environment = std::env::var("APP_ENVIRONMENT").unwrap_or("local".into());

    let environment_filename = format!("{}.toml", environment);

    let settings = config::Config::builder()
        .add_source(config::File::from(config_directory.join("base.toml")))
        .add_source(config::File::from(
            config_directory.join(environment_filename),
        ))
        .build()?;
    settings.try_deserialize()
}
