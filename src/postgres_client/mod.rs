use sqlx::{Pool, Postgres};
mod get_crate_metadata;
mod save_crate_metadata;

pub struct PostgresClient {
    pool: Pool<Postgres>,
}

impl PostgresClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[cfg(test)]
mod tests {
    use sqlx::{Pool, Postgres};
    use uuid::Uuid;

    use crate::configuration::Configuration;

    use crate::domain::{CrateName, CrateRequirement, CrateVersion};

    pub async fn spawn_database() -> Pool<Postgres> {
        let mut configuration = Configuration::load(&[]).unwrap();
        configuration.postgres.database_name = format!("test-{}", Uuid::new_v4().to_string());

        let server_pool = configuration.postgres.server_pool();

        sqlx::query(&format!(
            r#"CREATE DATABASE "{}""#,
            configuration.postgres.database_name
        ))
        .execute(&server_pool)
        .await
        .unwrap();

        let database_pool = configuration.postgres.database_pool();

        sqlx::migrate!("./migrations")
            .run(&database_pool)
            .await
            .unwrap();

        database_pool
    }

    pub fn name(value: &str) -> CrateName {
        CrateName::parse(value).unwrap()
    }

    pub fn requirement(value: &str) -> CrateRequirement {
        CrateRequirement::parse(value).unwrap()
    }

    pub fn version(value: &str) -> CrateVersion {
        CrateVersion::parse(value).unwrap()
    }
}
