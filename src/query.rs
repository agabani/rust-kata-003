use crate::crates_io_client::CratesIoClient;
use crate::domain::{CrateMetadata, CrateName, CrateVersion};
use crate::postgres_client::PostgresClient;
use std::collections::HashMap;

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
        let mut stack = Vec::<(String, String)>::new();
        stack.push((name.as_str().to_owned(), version.as_str().to_owned()));

        let mut results = HashMap::<(String, String), CrateMetadata>::new();

        while let Some((name, version)) = stack.pop() {
            let name = CrateName::parse(&name).unwrap();
            let version = CrateVersion::parse(&version).unwrap();

            // skip if we already have result in memory
            if results.contains_key(&(name.as_str().to_owned(), version.as_str().to_owned())) {
                continue;
            }

            // skip if we already have result in database
            let result = self.database.resolve_dependencies(&name, &version).await;
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
            let result = self.registry.resolve_dependencies(&name, &version).await;
            if let Some(result) = result {
                self.database.persist_dependencies(&result).await;

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
