use crate::crates_io_client::CratesIoClient;
use crate::domain::{CrateMetadata, CrateName, CrateVersion};
use crate::postgres_client::PostgresClient;

#[async_trait::async_trait]
pub trait DependencyResolver {
    fn resolve(&self, name: &CrateName, version: &CrateVersion) -> Self;
    async fn execute<R: Sync, D: Sync>(
        &self,
        registry: &R,
        database: &D,
    ) -> Option<Vec<CrateMetadata>>;
}

#[async_trait::async_trait]
pub trait Database {
    async fn get_crate_metadata(name: &CrateName, version: &CrateVersion) -> Option<CrateMetadata>;
    async fn put_crate_metadata(metadata: &CrateMetadata) -> Option<CrateMetadata>;
}

#[async_trait::async_trait]
pub trait Registry {
    async fn get_crate_metadata(name: &CrateName, version: &CrateVersion) -> Option<CrateMetadata>;
}

pub struct MinimumDependencyResolver {
    name: Option<CrateName>,
    version: Option<CrateVersion>,
}

impl MinimumDependencyResolver {
    pub fn new() -> Self {
        Self {
            name: None,
            version: None,
        }
    }
}

#[async_trait::async_trait]
impl<'a> DependencyResolver for MinimumDependencyResolver {
    fn resolve(&self, name: &CrateName, version: &CrateVersion) -> Self {
        Self {
            name: Some(name.clone()),
            version: Some(version.clone()),
        }
    }

    async fn execute<R: Sync, D: Sync>(
        &self,
        registry: &R,
        database: &D,
    ) -> Option<Vec<CrateMetadata>> {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl Database for PostgresClient {
    async fn get_crate_metadata(name: &CrateName, version: &CrateVersion) -> Option<CrateMetadata> {
        unimplemented!()
    }

    async fn put_crate_metadata(metadata: &CrateMetadata) -> Option<CrateMetadata> {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl Registry for CratesIoClient {
    async fn get_crate_metadata(name: &CrateName, version: &CrateVersion) -> Option<CrateMetadata> {
        unimplemented!()
    }
}
