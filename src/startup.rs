use crate::configuration::Configuration;
use crate::routes::{health_liveness, health_readiness};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

pub async fn run() -> (Server, u16) {
    let configuration = Configuration::load().expect("Failed to read configuration.");

    let listener = configuration.http_server.tcp_listener();
    let postgres_pool = web::Data::new(configuration.postgres.server_pool());
    let redis_pool = web::Data::new(configuration.redis.connection_manager().await);

    let port = listener.local_addr().unwrap().port();

    let server = HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/health")
                    .route("/liveness", web::get().to(health_liveness))
                    .route("/readiness", web::get().to(health_readiness)),
            )
            .app_data(postgres_pool.clone())
            .app_data(redis_pool.clone())
    })
    .listen(listener)
    .expect("Failed to bind address.")
    .run();

    (server, port)
}
