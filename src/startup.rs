use crate::configuration::Configuration;
use crate::postgres_client::PostgresClient;
use crate::routes::{dependency_query, health_liveness, health_readiness};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;

pub async fn run(overrides: &[(&str, &str)]) -> (Server, u16, Configuration) {
    let configuration = Configuration::load(overrides).expect("Failed to read configuration.");

    let listener = configuration
        .http_server
        .tcp_listener()
        .expect("Failed to bind port.");
    let port = listener.local_addr().unwrap().port();

    let postgres_pool = configuration.postgres.database_pool();
    let postgres_client = PostgresClient::new(postgres_pool.clone());

    let postgres_pool = web::Data::new(postgres_pool);
    let postgres_client = web::Data::new(postgres_client);

    let redis_pool = configuration
        .redis
        .connection_manager()
        .await
        .expect("Failed to connect to redis.");
    let redis_pool = web::Data::new(redis_pool);

    let crates_io_client = configuration
        .crates_io
        .client()
        .expect("Failed to create client.");
    let crates_io_client = web::Data::new(crates_io_client);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .service(
                web::scope("/health")
                    .route("/liveness", web::get().to(health_liveness))
                    .route("/readiness", web::get().to(health_readiness)),
            )
            .service(web::scope("/dependency").route("", web::get().to(dependency_query)))
            .app_data(crates_io_client.clone())
            .app_data(postgres_client.clone())
            .app_data(postgres_pool.clone())
            .app_data(redis_pool.clone())
    })
    .listen(listener)
    .expect("Failed to bind address.")
    .run();

    (server, port, configuration)
}
