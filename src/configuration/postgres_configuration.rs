use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use sqlx::{Pool, Postgres};

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

    fn database_connect_options(&self) -> PgConnectOptions {
        self.server_connect_options().database(&self.database_name)
    }
}
