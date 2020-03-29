use config::ConfigError;

#[derive(serde::Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32
}

#[derive(serde::Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub api_key: String
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}