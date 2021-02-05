pub struct TestApp {
    pub address: String,
}

pub async fn spawn_app() -> TestApp {
    std::env::set_var("app_http_server__port", "0");

    let (server, port) = rust_kata_003::run().await;
    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
    }
}
