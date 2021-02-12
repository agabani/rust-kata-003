use crate::domain::CrateMetadata;
use crate::postgres_client::PostgresClient;
use crate::telemetry::TraceErrorExt;

impl PostgresClient {
    #[tracing::instrument(
    skip(self, crate_metadata),
        fields(
            crate_name = %crate_metadata.name.as_str(),
            crate_version = %crate_metadata.version.as_str(),
        ),
    )]
    pub async fn save_crate_metadata(
        &self,
        crate_metadata: &CrateMetadata,
    ) -> Result<(), sqlx::Error> {
        let crate_name = crate_metadata.name.as_str();
        let crate_version = crate_metadata.version.as_str();
        let crate_dependencies = crate_metadata.dependencies.len() as i32;
        let row = sqlx::query!(
            r#"
INSERT INTO crate_metadata (name, version, dependencies)
VALUES ($1, $2, $3)
ON CONFLICT (name, version) DO UPDATE
    SET dependencies = EXCLUDED.dependencies
RETURNING id;
"#,
            crate_name,
            crate_version,
            crate_dependencies
        )
        .fetch_one(&self.pool)
        .await
        .trace_err()?;

        for dependency in &crate_metadata.dependencies {
            let crate_metadata_name = dependency.name.as_str();
            let crate_metadata_requirement = dependency.requirement.as_str();
            let crate_dependency_type = dependency.type_.as_str();
            sqlx::query!(
                r#"
INSERT INTO crate_dependency (crate_id, name, requirement, type)
VALUES ($1, $2, $3, $4)
ON CONFLICT (name, type, crate_id) DO UPDATE SET requirement = EXCLUDED.requirement;
"#,
                row.id,
                crate_metadata_name,
                crate_metadata_requirement,
                crate_dependency_type
            )
            .execute(&self.pool)
            .await
            .trace_err()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{CrateDependency, CrateDependencyType};
    use crate::postgres_client::tests::{name, requirement, spawn_database, version};
    use sqlx::{Pool, Postgres, Row};

    #[actix_rt::test]
    async fn saves_with_no_dependencies() {
        // Arrange
        let pool = spawn_database().await;
        let client = PostgresClient::new(pool.clone());

        // Act
        client
            .save_crate_metadata(&CrateMetadata {
                name: name("no-dependencies"),
                version: version("version-1"),
                dependencies: vec![],
            })
            .await
            .unwrap();

        // Assert
        assert(
            &[("no-dependencies", "version-1", 0, None, None, None)],
            &pool,
            "no-dependencies",
            "version-1",
        )
        .await
    }

    #[actix_rt::test]
    async fn saves_with_dependencies() {
        // Arrange
        let pool = spawn_database().await;
        let client = PostgresClient::new(pool.clone());

        // Act
        client
            .save_crate_metadata(&CrateMetadata {
                name: name("three-dependencies"),
                version: version("version-1"),
                dependencies: vec![
                    CrateDependency {
                        name: name("name-1"),
                        requirement: requirement("requirement-1"),
                        type_: CrateDependencyType::Build,
                    },
                    CrateDependency {
                        name: name("name-2"),
                        requirement: requirement("requirement-2"),
                        type_: CrateDependencyType::Dev,
                    },
                    CrateDependency {
                        name: name("name-3"),
                        requirement: requirement("requirement-3"),
                        type_: CrateDependencyType::Normal,
                    },
                ],
            })
            .await
            .unwrap();

        // Assert
        assert(
            &vec![
                (
                    "three-dependencies",
                    "version-1",
                    3,
                    Some("name-1"),
                    Some("requirement-1"),
                    Some("build"),
                ),
                (
                    "three-dependencies",
                    "version-1",
                    3,
                    Some("name-2"),
                    Some("requirement-2"),
                    Some("dev"),
                ),
                (
                    "three-dependencies",
                    "version-1",
                    3,
                    Some("name-3"),
                    Some("requirement-3"),
                    Some("normal"),
                ),
            ],
            &pool,
            "three-dependencies",
            "version-1",
        )
        .await
    }

    #[actix_rt::test]
    async fn saves_with_dependencies_when_persistence_is_corrupted() {
        // Arrange
        let pool = spawn_database().await;
        let client = PostgresClient::new(pool.clone());

        let i: i32 = sqlx::query_scalar(
            r#"
INSERT INTO crate_metadata (name, version, dependencies)
VALUES ('three-dependencies-corrupted', 'version-1', 2) RETURNING id;
"#,
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
INSERT INTO crate_dependency (crate_id, name, requirement, type)
VALUES ($1, 'name-1', 'wrong-requirement-1', 'build');
"#,
        )
        .bind(i)
        .execute(&pool)
        .await
        .unwrap();

        // Act
        client
            .save_crate_metadata(&CrateMetadata {
                name: name("three-dependencies-corrupted"),
                version: version("version-1"),
                dependencies: vec![
                    CrateDependency {
                        name: name("name-1"),
                        requirement: requirement("requirement-1"),
                        type_: CrateDependencyType::Build,
                    },
                    CrateDependency {
                        name: name("name-2"),
                        requirement: requirement("requirement-2"),
                        type_: CrateDependencyType::Dev,
                    },
                    CrateDependency {
                        name: name("name-3"),
                        requirement: requirement("requirement-3"),
                        type_: CrateDependencyType::Normal,
                    },
                ],
            })
            .await
            .unwrap();

        // Assert
        assert(
            &vec![
                (
                    "three-dependencies-corrupted",
                    "version-1",
                    3,
                    Some("name-1"),
                    Some("requirement-1"),
                    Some("build"),
                ),
                (
                    "three-dependencies-corrupted",
                    "version-1",
                    3,
                    Some("name-2"),
                    Some("requirement-2"),
                    Some("dev"),
                ),
                (
                    "three-dependencies-corrupted",
                    "version-1",
                    3,
                    Some("name-3"),
                    Some("requirement-3"),
                    Some("normal"),
                ),
            ],
            &pool,
            "three-dependencies-corrupted",
            "version-1",
        )
        .await
    }

    #[allow(clippy::type_complexity)]
    async fn assert(
        data: &[(&str, &str, i32, Option<&str>, Option<&str>, Option<&str>)],
        pool: &Pool<Postgres>,
        name: &str,
        version: &str,
    ) {
        let rows = sqlx::query(
            r#"
SELECT cm.id,
       cm.name         AS crate_metadata_name,
       cm.version      AS crate_metadata_version,
       cm.dependencies AS crate_metadata_dependencies,
       cd.name         AS crate_dependency_name,
       cd.requirement  AS crate_dependency_requirement,
       cd.type         AS crate_dependency_type
FROM crate_metadata AS cm
         LEFT JOIN crate_dependency AS cd on cm.id = cd.crate_id
WHERE cm.name = $1
  AND cm.version = $2;
        "#,
        )
        .bind(name)
        .bind(version)
        .fetch_all(pool)
        .await
        .unwrap();

        assert_eq!(data.len(), rows.len());

        for i in 0..data.len() {
            let actual_crate_metadata_name: &str = rows[i].get("crate_metadata_name");
            let actual_crate_metadata_version: &str = rows[i].get("crate_metadata_version");
            let actual_crate_metadata_dependencies: i32 =
                rows[i].get("crate_metadata_dependencies");
            let actual_crate_dependency_name: Option<&str> = rows[i].get("crate_dependency_name");
            let actual_crate_dependency_requirement: Option<&str> =
                rows[i].get("crate_dependency_requirement");
            let actual_crate_dependency_type: Option<&str> = rows[i].get("crate_dependency_type");

            let (
                expected_crate_metadata_name,
                expected_crate_metadata_version,
                expected_crate_metadata_dependencies,
                expected_crate_dependency_name,
                expected_crate_dependency_requirement,
                expected_crate_dependency_type,
            ) = data[i];
            assert_eq!(expected_crate_metadata_name, actual_crate_metadata_name);
            assert_eq!(
                expected_crate_metadata_version,
                actual_crate_metadata_version
            );
            assert_eq!(
                expected_crate_metadata_dependencies,
                actual_crate_metadata_dependencies
            );
            assert_eq!(expected_crate_dependency_name, actual_crate_dependency_name);
            assert_eq!(
                expected_crate_dependency_requirement,
                actual_crate_dependency_requirement
            );
            assert_eq!(expected_crate_dependency_type, actual_crate_dependency_type);
        }
    }
}
