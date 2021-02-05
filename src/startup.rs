use crate::configuration::Configuration;
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use redis::aio::ConnectionManager;
use sqlx::{Pool, Postgres};

async fn health_liveness() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn health_readiness(
    postgres_pool: web::Data<Pool<Postgres>>,
    redis_pool: web::Data<ConnectionManager>,
) -> Result<HttpResponse, HttpResponse> {
    let _: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(postgres_pool.get_ref())
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    let _: () = redis::cmd("PING")
        .query_async(&mut redis_pool.get_ref().clone())
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().finish())
}

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
