use rust_kata_003::startup::run;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_pool = redis::Client::open("redis://127.0.0.1/")
        .expect("Failed to create redis client.")
        .get_tokio_connection_manager()
        .await
        .expect("Failed to connect to redis.");

    let postgres_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(1))
        .connect_lazy("postgres://postgres:password@127.0.0.1/rust-kata-003")
        .expect("Failed to connect to postgres.");

    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind port.");
    let server = run(listener, postgres_pool, redis_pool).expect("Failed to bind address.");
    server.await
}
