use crate::crates_io_client::CratesIoClient;
use crate::domain::{CrateMetadata, CrateName, CrateVersion};
use crate::postgres_client::PostgresClient;

pub struct Query<'a, R: Registry, D: Database> {
    registry: &'a R,
    database: &'a D,
}

impl<'a, R: Registry, D: Database> Query<'a, R, D> {
    pub fn new(registry: &'a R, database: &'a D) -> Self {
        Self { registry, database }
    }

    pub async fn dependency_graph(
        &self,
        name: &CrateName,
        version: &CrateVersion,
    ) -> Option<Vec<CrateMetadata>> {
        let result = self.database.resolve_dependencies(name, version).await;

        if let Some(result) = result {
            return Some(vec![result]);
        }

        let result = self.registry.resolve_dependencies(name, version).await;

        if let Some(result) = result {
            self.database.persist_dependencies(&result).await;

            return Some(vec![result]);
        } else {
            None
        }
    }
}

#[async_trait::async_trait]
pub trait Registry {
    async fn resolve_dependencies(
        &self,
        name: &CrateName,
        version: &CrateVersion,
    ) -> Option<CrateMetadata>;
}

#[async_trait::async_trait]
impl Registry for CratesIoClient {
    async fn resolve_dependencies(
        &self,
        name: &CrateName,
        version: &CrateVersion,
    ) -> Option<CrateMetadata> {
        self.dependencies(name, version).await
    }
}

#[async_trait::async_trait]
pub trait Database {
    async fn resolve_dependencies(
        &self,
        name: &CrateName,
        version: &CrateVersion,
    ) -> Option<CrateMetadata>;

    async fn persist_dependencies(&self, metadata: &CrateMetadata);
}

#[async_trait::async_trait]
impl Database for PostgresClient {
    async fn resolve_dependencies(
        &self,
        name: &CrateName,
        version: &CrateVersion,
    ) -> Option<CrateMetadata> {
        self.get_crate_metadata(name, version).await.unwrap()
    }

    async fn persist_dependencies(&self, metadata: &CrateMetadata) {
        self.save_crate_metadata(metadata).await.unwrap()
    }
}
