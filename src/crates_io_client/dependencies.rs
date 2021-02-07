use crate::crates_io_client::CratesIoClient;
use crate::domain::{CrateMetadata, CrateName, CrateRelationship, CrateVersion};
use std::convert::TryFrom;

#[derive(Debug, serde::Deserialize)]
struct Response {
    #[serde(rename = "dependencies")]
    dependencies: Vec<DependencyResponse>,
}

#[derive(Debug, serde::Deserialize)]
struct DependencyResponse {
    #[serde(rename = "id")]
    id: i64,
    #[serde(rename = "version_id")]
    version_id: i64,
    #[serde(rename = "crate_id")]
    crate_id: String,
    #[serde(rename = "req")]
    req: String,
    #[serde(rename = "optional")]
    optional: bool,
    #[serde(rename = "default_features")]
    default_features: bool,
    #[serde(rename = "features")]
    features: Option<Vec<String>>,
    #[serde(rename = "target")]
    target: Option<String>,
    #[serde(rename = "kind")]
    kind: String,
    #[serde(rename = "downloads")]
    downloads: i64,
}

impl CratesIoClient {
    pub async fn dependencies(
        &self,
        name: &CrateName,
        version: &CrateVersion,
    ) -> Option<Vec<CrateMetadata>> {
        let url = format!(
            "/api/v1/crates/{}/{}/dependencies",
            name.as_str(),
            version.as_str()
        );

        let response = self.get::<Response>(&url).await?;

        let result = response
            .dependencies
            .iter()
            .map(|d| CrateMetadata {
                name: CrateName::parse(&d.crate_id).expect("Failed to parse name."),
                version: CrateVersion::parse(&d.req).expect("Failed to parse version."),
                relationship: CrateRelationship::try_from(d.kind.as_str())
                    .expect("Failed to parse relationship."),
            })
            .collect();

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};
    use std::env;
    use wiremock::matchers::{any, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[actix_rt::test]
    async fn dependencies_returns_200() {
        // Arrange
        let user_agent: String = Faker.fake();

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/crates/proc-macro2/1.0.24/dependencies"))
            .and(header("user-agent", user_agent.as_str()))
            .respond_with(
                ResponseTemplate::new(200).set_body_bytes(fixture("proc-macro2-1.0.24.json")),
            )
            .expect(1)
            .mount(&server)
            .await;

        let client = CratesIoClient::new(&server.uri(), &user_agent).unwrap();

        // Act
        let result = client
            .dependencies(
                &CrateName::parse("proc-macro2").unwrap(),
                &CrateVersion::parse("1.0.24").unwrap(),
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(2, result.len());
        assert_eq!(&"quote", &result[0].name.as_str());
        assert_eq!(&"^1.0", &result[0].version.as_str());
        assert_eq!(&"dev", &result[0].relationship.as_str());
        assert_eq!(&"unicode-xid", &result[1].name.as_str());
        assert_eq!(&"^0.2", &result[1].version.as_str());
        assert_eq!(&"normal", &result[1].relationship.as_str());
    }

    #[actix_rt::test]
    async fn dependencies_returns_404() {
        // Arrange
        let server = MockServer::start().await;
        Mock::given(any())
            .respond_with(ResponseTemplate::new(404).set_body_bytes(fixture("404.json")))
            .expect(1)
            .mount(&server)
            .await;

        let client = CratesIoClient::new(&server.uri(), &Faker.fake::<String>()).unwrap();

        // Act
        let result = client
            .dependencies(
                &CrateName::parse(&Faker.fake::<String>()).unwrap(),
                &CrateVersion::parse(&Faker.fake::<String>()).unwrap(),
            )
            .await;

        // Assert
        assert!(result.is_none());
    }

    fn fixture(filename: &str) -> Vec<u8> {
        let path = env::current_dir()
            .unwrap()
            .join("tests")
            .join("fixtures")
            .join(filename);

        std::fs::read(path).unwrap()
    }
}
