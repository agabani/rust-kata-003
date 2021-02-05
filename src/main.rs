use rust_kata_003::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (server, _) = run().await;
    server.await
}
