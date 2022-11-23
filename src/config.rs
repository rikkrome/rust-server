use serde::Deserialize;
use config::ConfigError;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        config::Config::builder()
        .add_source(config::Environment::default())
        .build()?
        .try_deserialize()
    }
}