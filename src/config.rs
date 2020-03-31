use config::ConfigError;

#[derive(serde::Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(serde::Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub api_key: String,
    pub base_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
