use crate::telemetry::TraceErrorExt;
use config::{Config, File};
use redis::aio::ConnectionManager;
use redis::{ConnectionAddr, ConnectionInfo, RedisResult};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use sqlx::{Pool, Postgres};
use std::convert::{TryFrom, TryInto};
use std::env;
use std::net::TcpListener;

#[derive(serde::Deserialize)]
pub struct Configuration {
    pub http_server: HttpServerConfiguration,
    pub postgres: PostgresConfiguration,
    pub redis: RedisConfiguration,
}

impl Configuration {
    #[tracing::instrument]
    pub fn load() -> Result<Configuration, config::ConfigError> {
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

        config.try_into()
    }
}

#[derive(serde::Deserialize)]
pub struct HttpServerConfiguration {
    pub host: String,
    pub port: u16,
}

impl HttpServerConfiguration {
    #[tracing::instrument(skip(self))]
    pub fn tcp_listener(&self) -> std::io::Result<TcpListener> {
        TcpListener::bind(format!("{}:{}", self.host, self.port)).trace_err()
    }
}

#[derive(serde::Deserialize)]
pub struct PostgresConfiguration {
    pub database_name: String,
    pub host: String,
    pub password: String,
    pub port: u16,
    pub require_ssl: bool,
    pub username: String,
}

impl PostgresConfiguration {
    pub fn server_pool(&self) -> Pool<Postgres> {
        PgPoolOptions::new().connect_lazy_with(self.server_connect_options())
    }

    #[allow(dead_code)]
    pub fn database_pool(&self) -> Pool<Postgres> {
        PgPoolOptions::new().connect_lazy_with(self.database_connect_options())
    }

    fn server_connect_options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password)
            .ssl_mode(match self.require_ssl {
                true => PgSslMode::Require,
                false => PgSslMode::Prefer,
            })
    }

    #[allow(dead_code)]
    fn database_connect_options(&self) -> PgConnectOptions {
        self.server_connect_options().database(&self.database_name)
    }
}

#[derive(serde::Deserialize)]
pub struct RedisConfiguration {
    pub database: i64,
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub ssl: bool,
    pub username: Option<String>,
}

impl RedisConfiguration {
    #[tracing::instrument(skip(self))]
    pub async fn connection_manager(&self) -> RedisResult<ConnectionManager> {
        redis::Client::open(ConnectionInfo {
            username: self.username.to_owned(),
            passwd: self.password.to_owned(),
            addr: Box::new(match self.ssl {
                true => ConnectionAddr::TcpTls {
                    host: self.host.to_owned(),
                    port: self.port,
                    insecure: false,
                },
                false => ConnectionAddr::Tcp(self.host.to_owned(), self.port),
            }),
            db: self.database,
        })
        .trace_err()?
        .get_tokio_connection_manager()
        .await
        .trace_err()
    }
}

#[derive(Debug, PartialEq)]
enum Environment {
    Local,
    Production,
}

impl Environment {
    fn as_str(&self) -> &str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Environment;
    use std::convert::TryFrom;

    #[test]
    fn environment_as_str() {
        assert_eq!(Environment::Local.as_str(), "local");
        assert_eq!(Environment::Production.as_str(), "production");
    }

    #[test]
    fn environment_try_from() {
        assert_eq!(
            Environment::try_from("local".to_owned()),
            Ok(Environment::Local)
        );
        assert_eq!(
            Environment::try_from("production".to_owned()),
            Ok(Environment::Production)
        );
        assert_eq!(
            Environment::try_from("other".to_owned()),
            Err(
                "other is not a supported environment. Use either `local` or `production`."
                    .to_owned()
            )
        );
    }
}
