mod fixtures;
mod support;

use crate::fixtures::fixture;
use crate::support::spawn_app;
use fake::{Fake, Faker};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[actix_rt::test]
async fn dependency_query_returns_200() {
    // Arrange
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/crates/proc-macro2/1.0.24/dependencies"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(fixture("proc-macro2-1.0.24.json")))
        .expect(1)
        .mount(&mock_server)
        .await;

    let app = spawn_app(&[("crates_io.base_address", mock_server.uri().as_str())]).await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/dependency", app.address))
        .query(&[("name", "proc-macro2"), ("version", "1.0.24")])
        .send()
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status().as_u16(), 200);
}

#[actix_rt::test]
async fn dependency_query_returns_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app(&[]).await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ([("version", Faker.fake::<String>())], "missing name"),
        ([("name", Faker.fake::<String>())], "missing version"),
    ];

    for (query, error_message) in test_cases {
        // Act
        let response = client
            .get(&format!("{}/dependency", app.address))
            .query(&query)
            .send()
            .await
            .unwrap();

        // Assert
        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[actix_rt::test]
async fn dependency_query_returns_404_when_crate_data_does_not_exist() {
    // Arrange
    let crate_name = Faker.fake::<String>();
    let crate_version = Faker.fake::<String>();

    let relative_path = format!(
        "/api/v1/crates/{}/{}/dependencies",
        crate_name, crate_version
    );

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(relative_path))
        .respond_with(wiremock::ResponseTemplate::new(404).set_body_bytes(fixture("404.json")))
        .expect(1)
        .mount(&mock_server)
        .await;

    let app = spawn_app(&[("crates_io.base_address", mock_server.uri().as_str())]).await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/dependency", app.address))
        .query(&[("name", crate_name), ("version", crate_version)])
        .send()
        .await
        .unwrap();

    // Assert
    assert_eq!(404, response.status().as_u16());
}
