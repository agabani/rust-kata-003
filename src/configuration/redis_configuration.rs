use crate::telemetry::TraceErrorExt;
use redis::aio::ConnectionManager;
use redis::{ConnectionAddr, ConnectionInfo, RedisResult};

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
