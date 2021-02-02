use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

async fn health_liveness() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn health_readiness() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new().service(
            web::scope("/health")
                .route("/liveness", web::get().to(health_liveness))
                .route("/readiness", web::get().to(health_readiness)),
        )
    })
    .listen(listener)?
    .run();
    Ok(server)
}
