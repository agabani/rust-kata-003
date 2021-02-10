use rust_kata_003::telemetry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    telemetry::init(telemetry::configure("info"));

    let (server, _, _) = rust_kata_003::run(&[]).await;
    server.await
}
