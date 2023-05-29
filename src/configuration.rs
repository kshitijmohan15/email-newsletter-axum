use config::{Config, File, FileFormat, ConfigError};

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub application_port: u16,
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}
#[derive(Debug)]
pub struct Res {
    pub settings: Settings
}
pub fn build_configuration() -> Result<Config, config::ConfigError> {
    // Initialise our configuration reader
    Config::builder()
        .add_source(File::new("configuration", FileFormat::Yaml))
        .build()
}
pub fn get_config() -> Result<Res, ConfigError>{
    let config = build_configuration().expect("Not looking good bruv");
    let configs = Res{settings: config.try_into()?};
    Ok(configs)
}
impl TryFrom<Config> for Settings {
    type Error = config::ConfigError;
    fn try_from(value: Config) -> Result<Self, Self::Error> {
        let app_port = match value.cache.kind {
            config::ValueKind::Table(value) => value
                .get("application_port")
                .unwrap()
                .kind
                .to_string()
                .parse::<u16>(),
            _ => Ok(0),
        };

        Ok(Self {
            application_port: app_port.unwrap(),
        })
    }
}
