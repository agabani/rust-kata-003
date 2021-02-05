mod support;

use crate::support::spawn_app;
use fake::{Fake, Faker};

#[actix_rt::test]
async fn dependency_query_returns_200() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/dependency", app.address))
        .query(&[
            ("name", Faker.fake::<String>()),
            ("version", Faker.fake::<String>()),
        ])
        .send()
        .await
        .expect("Failed to send request.");

    assert_eq!(response.status().as_u16(), 200);
}

#[actix_rt::test]
async fn dependency_query_returns_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ([("version", Faker.fake::<String>())], "missing name"),
        ([("name", Faker.fake::<String>())], "missing version"),
    ];

    for (query, error_message) in test_cases {
        let response = client
            .get(&format!("{}/dependency", app.address))
            .query(&query)
            .send()
            .await
            .expect("Failed to send request.");

        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
