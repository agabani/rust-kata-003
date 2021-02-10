use crate::domain::{
    CrateDependency, CrateDependencyType, CrateMetadata, CrateName, CrateRequirement, CrateVersion,
};
use crate::postgres_client::PostgresClient;
use crate::telemetry::TraceErrorExt;
use std::convert::TryFrom;

impl PostgresClient {
    #[tracing::instrument(
        skip(self, name, version),
        fields(
            crate_name = %name.as_str(),
            crate_version = %version.as_str(),
        ),
    )]
    pub async fn get_crate_metadata(
        &self,
        name: &CrateName,
        version: &CrateVersion,
    ) -> Result<Option<CrateMetadata>, sqlx::Error> {
        let crate_name = name.as_str();
        let crate_version = version.as_str();
        let results = sqlx::query!(
            r#"
SELECT cm.name         AS crate_metadata_name,
       cm.version      AS crate_metadata_version,
       cm.dependencies AS crate_metadata_dependencies,
       cd.name         AS "crate_dependency_name?",
       cd.requirement  AS "crate_dependency_requirement?",
       cd.type         AS "crate_dependency_type?"
FROM crate_metadata as cm
         LEFT JOIN crate_dependency cd on cm.id = cd.crate_id
WHERE cm.name = $1
  AND cm.version = $2;
"#,
            crate_name,
            crate_version,
        )
        .fetch_all(&self.pool)
        .await
        .trace_err()?;

        if results.is_empty() {
            return Ok(None);
        }

        let dependencies_found = if results[0].crate_dependency_name.is_none() {
            0usize
        } else {
            results.len()
        };
        if results[0].crate_metadata_dependencies as usize != dependencies_found {
            tracing::warn!("checksum failed");
            return Ok(None);
        }

        if results[0].crate_dependency_name.is_none() {
            return Ok(Some(CrateMetadata {
                name: name.clone(),
                version: version.clone(),
                dependencies: vec![],
            }));
        }

        let result = CrateMetadata {
            name: name.clone(),
            version: version.clone(),
            dependencies: results
                .iter()
                .map(|result| {
                    let name = result.crate_dependency_name.as_ref().unwrap();
                    let requirement = result.crate_dependency_requirement.as_ref().unwrap();
                    let type_ = result.crate_dependency_type.as_ref().unwrap();
                    CrateDependency {
                        name: CrateName::parse(&name).unwrap(),
                        requirement: CrateRequirement::parse(&requirement).unwrap(),
                        type_: CrateDependencyType::try_from(type_.as_ref()).unwrap(),
                    }
                })
                .collect(),
        };

        Ok(Some(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::postgres_client::tests::{name, requirement, spawn_database, version};
    use sqlx::{Pool, Postgres};

    #[actix_rt::test]
    async fn returns_none_when_not_present() {
        // Arrange
        let pool = spawn_database().await;
        seed_database(&pool).await;
        let client = PostgresClient::new(pool.clone());

        // Act
        let result = client
            .get_crate_metadata(&name("not-present"), &version("version-1"))
            .await
            .unwrap();

        // Assert
        assert!(result.is_none());
    }

    #[actix_rt::test]
    async fn returns_none_when_checksum_fails() {
        // Arrange
        let pool = spawn_database().await;
        seed_database(&pool).await;
        let client = PostgresClient::new(pool.clone());

        // Act
        let result = client
            .get_crate_metadata(&name("corrupted"), &version("version-1"))
            .await
            .unwrap();

        // Assert
        assert!(result.is_none());
    }

    #[actix_rt::test]
    async fn returns_empty_dependencies() {
        // Arrange
        let pool = spawn_database().await;
        seed_database(&pool).await;
        let client = PostgresClient::new(pool.clone());

        // Act
        let result = client
            .get_crate_metadata(&name("no-dependencies"), &version("version-1"))
            .await
            .unwrap()
            .unwrap();

        // Assert
        assert_eq!(
            CrateMetadata {
                name: name("no-dependencies"),
                version: version("version-1"),
                dependencies: vec![]
            },
            result
        );
    }

    #[actix_rt::test]
    async fn returns_dependencies() {
        // Arrange
        let pool = spawn_database().await;
        seed_database(&pool).await;
        let client = PostgresClient::new(pool.clone());

        // Act
        let result = client
            .get_crate_metadata(&name("three-dependencies"), &version("version-1"))
            .await;

        // Assert
        let result = result.unwrap().unwrap();

        assert_eq!(
            result,
            CrateMetadata {
                name: name("three-dependencies"),
                version: version("version-1"),
                dependencies: vec![
                    CrateDependency {
                        name: name("name-1"),
                        requirement: requirement("requirement-1"),
                        type_: CrateDependencyType::Build
                    },
                    CrateDependency {
                        name: name("name-2"),
                        requirement: requirement("requirement-2"),
                        type_: CrateDependencyType::Dev
                    },
                    CrateDependency {
                        name: name("name-3"),
                        requirement: requirement("requirement-3"),
                        type_: CrateDependencyType::Normal
                    }
                ]
            }
        );
    }

    async fn seed_database(database_pool: &Pool<Postgres>) {
        seed_no_dependencies(database_pool).await;
        seed_three_dependencies(database_pool).await;
        seed_corrupted(database_pool).await;
    }

    async fn seed_no_dependencies(database_pool: &Pool<Postgres>) {
        sqlx::query(
            r#"
INSERT INTO crate_metadata (name, version, dependencies)
VALUES ('no-dependencies', 'version-1', 0);
"#,
        )
        .execute(database_pool)
        .await
        .unwrap();
    }

    async fn seed_three_dependencies(database_pool: &Pool<Postgres>) {
        let i: i32 = sqlx::query_scalar(
            r#"
INSERT INTO crate_metadata (name, version, dependencies)
VALUES ('three-dependencies', 'version-1', 3) RETURNING id;
"#,
        )
        .fetch_one(database_pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
INSERT INTO crate_dependency (crate_id, name, requirement, type)
VALUES ($1, 'name-1', 'requirement-1', 'build'),
       ($1, 'name-2', 'requirement-2', 'dev'),
       ($1, 'name-3', 'requirement-3', 'normal');
"#,
        )
        .bind(i)
        .execute(database_pool)
        .await
        .unwrap();
    }

    async fn seed_corrupted(database_pool: &Pool<Postgres>) {
        let i: i32 = sqlx::query_scalar(
            r#"
INSERT INTO crate_metadata (name, version, dependencies)
VALUES ('corrupted', 'version-1', 3) RETURNING id;
"#,
        )
        .fetch_one(database_pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
INSERT INTO crate_dependency (crate_id, name, requirement, type)
VALUES ($1, 'name-1', 'requirement-1', 'build');
"#,
        )
        .bind(i)
        .execute(database_pool)
        .await
        .unwrap();
    }
}
