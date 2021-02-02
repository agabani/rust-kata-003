use rust_kata_003::startup::run;
use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_liveness_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health/liveness", app.address))
        .send()
        .await
        .expect("Failed to send request.");

    assert_eq!(response.status().as_u16(), 200);
}

#[actix_rt::test]
async fn health_check_readiness_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health/readiness", app.address))
        .send()
        .await
        .expect("Failed to send request.");

    assert_eq!(response.status().as_u16(), 200);
}

struct TestApp {
    pub address: String,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let server = run(listener).expect("Failed to bind address.");
    let _ = tokio::spawn(server);
    TestApp { address }
}
