use rust_kata_003::configuration::Configuration;
use rust_kata_003::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = Configuration::load().expect("Failed to read configuration.");

    let postgres_pool = configuration.postgres.server_pool();
    let redis_pool = configuration.redis.connection_manager().await;
    let listener = configuration.http_server.tcp_listener();

    let server = run(listener, postgres_pool, redis_pool).expect("Failed to bind address.");
    server.await
}
