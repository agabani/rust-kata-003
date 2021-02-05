#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (server, _) = rust_kata_003::run().await;
    server.await
}
