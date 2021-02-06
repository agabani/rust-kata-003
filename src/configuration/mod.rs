mod crates_io_configuration;
mod environment;
mod http_server_configuration;
mod postgres_configuration;
mod redis_configuration;

use crate::telemetry::TraceErrorExt;
use config::{Config, File};
use environment::Environment;
use std::convert::TryInto;
use std::env;

pub use crates_io_configuration::*;
pub use http_server_configuration::*;
pub use postgres_configuration::*;
pub use redis_configuration::*;

#[derive(serde::Deserialize)]
pub struct Configuration {
    pub crates_io: CratesIoConfiguration,
    pub http_server: HttpServerConfiguration,
    pub postgres: PostgresConfiguration,
    pub redis: RedisConfiguration,
}

impl Configuration {
    #[tracing::instrument]
    pub fn load(overrides: &[(&str, &str)]) -> Result<Configuration, config::ConfigError> {
        let configuration_directory = env::current_dir()
            .expect("Failed to determine current directory.")
            .join("configuration");

        let environment: Environment = env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("Failed to parse APP_ENVIRONMENT.");

        let mut config = Config::default();
        config
            .merge(File::from(configuration_directory.join("default")).required(true))
            .trace_err()?
            .merge(File::from(configuration_directory.join(environment.as_str())).required(true))
            .trace_err()?
            .merge(config::Environment::with_prefix("APP").separator("__"))
            .trace_err()?;

        for &(key, value) in overrides {
            config.set(key, value).trace_err()?;
        }

        config.try_into()
    }
}
