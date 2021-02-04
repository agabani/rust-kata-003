use rust_kata_003::configuration::Configuration;
use rust_kata_003::startup::run;
use std::env;

pub struct TestApp {
    pub address: String,
}

pub async fn spawn_app() -> TestApp {
    env::set_var("app_http_server__port", "0");
    let configuration = Configuration::load().expect("Failed to read configuration.");

    let postgres_pool = configuration.postgres.server_pool();
    let redis_pool = configuration.redis.connection_manager().await;
    let listener = configuration.http_server.tcp_listener();

    let address = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());

    let server = run(listener, postgres_pool, redis_pool).expect("Failed to bind address.");
    let _ = tokio::spawn(server);

    TestApp { address }
}
