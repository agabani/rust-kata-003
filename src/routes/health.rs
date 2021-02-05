use actix_web::{web, HttpResponse};
use redis::aio::ConnectionManager;
use sqlx::{Pool, Postgres};

pub async fn health_liveness() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn health_readiness(
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
