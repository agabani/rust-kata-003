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
    postgres(postgres_pool.get_ref())
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    redis(redis_pool.get_ref())
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(skip(pool))]
async fn postgres(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let _: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(pool)
        .await
        .map_err(|error| {
            tracing::error!(%error);
            error
        })?;
    Ok(())
}

#[tracing::instrument(skip(pool))]
async fn redis(pool: &ConnectionManager) -> Result<(), redis::RedisError> {
    let _: () = redis::cmd("PING")
        .query_async(&mut pool.clone())
        .await
        .map_err(|error| {
            tracing::error!(%error);
            error
        })?;
    Ok(())
}
