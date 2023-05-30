use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct FinalSettings {
    pub application: Settings,
    pub database: DatabaseSettings,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub port: u16,
    pub host: String,
}

pub fn get_config() -> Result<FinalSettings, config::ConfigError> {
    let config = Config::builder()
        .add_source(File::new("configuration", FileFormat::Yaml))
        .build()?;

    let final_settings = config.try_deserialize::<FinalSettings>()?;
    Ok(final_settings)
}
