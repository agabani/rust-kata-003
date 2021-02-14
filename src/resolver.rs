use crate::crates_io_client::CratesIoClient;
use crate::domain::{CrateMetadata, CrateName, CrateVersion};
use crate::postgres_client::PostgresClient;
use std::collections::HashMap;

#[async_trait::async_trait]
pub trait DependencyResolver {
    fn resolve(&self, name: &CrateName, version: &CrateVersion) -> Self;
    async fn execute<R: Registry + Sync, D: Database + Sync>(
        &self,
        registry: &R,
        database: &D,
    ) -> Option<Vec<CrateMetadata>>;
}

#[async_trait::async_trait]
pub trait Database {
    async fn get_crate_metadata(
        &self,
        name: &CrateName,
        version: &CrateVersion,
    ) -> Option<CrateMetadata>;

    async fn put_crate_metadata(&self, metadata: &CrateMetadata);
}

#[async_trait::async_trait]
pub trait Registry {
    async fn get_crate_metadata(
        &self,
        name: &CrateName,
        version: &CrateVersion,
    ) -> Option<CrateMetadata>;
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

    async fn execute<R: Registry + Sync, D: Database + Sync>(
        &self,
        registry: &R,
        database: &D,
    ) -> Option<Vec<CrateMetadata>> {
        let mut stack = Vec::<(String, String)>::new();
        stack.push((
            self.name.as_ref().unwrap().as_str().to_owned(),
            self.version.as_ref().unwrap().as_str().to_owned(),
        ));

        let mut results = HashMap::<(String, String), CrateMetadata>::new();

        while let Some((name, version)) = stack.pop() {
            let name = CrateName::parse(&name).unwrap();
            let version = CrateVersion::parse(&version).unwrap();

            // skip if we already have result in memory
            if results.contains_key(&(name.as_str().to_owned(), version.as_str().to_owned())) {
                continue;
            }

            // skip if we already have result in database
            let result = database.get_crate_metadata(&name, &version).await;
            if let Some(result) = result {
                for dependency in &result.dependencies {
                    stack.push((
                        dependency.name.as_str().to_owned(),
                        dependency.requirement.minimum_version().as_str().to_owned(),
                    ));
                }

                results.insert(
                    (
                        result.name.as_str().to_owned(),
                        result.version.as_str().to_owned(),
                    ),
                    result,
                );

                continue;
            }

            // fresh
            let result = registry.get_crate_metadata(&name, &version).await;
            if let Some(result) = result {
                database.put_crate_metadata(&result).await;

                for dependency in &result.dependencies {
                    stack.push((
                        dependency.name.as_str().to_owned(),
                        dependency.requirement.minimum_version().as_str().to_owned(),
                    ));
                }

                results.insert(
                    (
                        result.name.as_str().to_owned(),
                        result.version.as_str().to_owned(),
                    ),
                    result,
                );

                continue;
            } else {
                return None;
            }
        }

        Some(results.into_iter().map(|((_, _), value)| value).collect())
    }
}

#[async_trait::async_trait]
impl Database for PostgresClient {
    async fn get_crate_metadata(
        &self,
        name: &CrateName,
        version: &CrateVersion,
    ) -> Option<CrateMetadata> {
        self.get_crate_metadata(name, version).await.unwrap()
    }

    async fn put_crate_metadata(&self, metadata: &CrateMetadata) {
        self.save_crate_metadata(metadata).await.unwrap()
    }
}

#[async_trait::async_trait]
impl Registry for CratesIoClient {
    async fn get_crate_metadata(
        &self,
        name: &CrateName,
        version: &CrateVersion,
    ) -> Option<CrateMetadata> {
        self.dependencies(name, version).await
    }
}
