use rust_kata_003::telemetry;

lazy_static::lazy_static! {
    static ref TRACING: () = {
        telemetry::init(telemetry::configure("error"));
    };
}

pub struct TestApp {
    pub address: String,
}

pub async fn spawn_app(overrides: &[(&str, &str)]) -> TestApp {
    lazy_static::initialize(&TRACING);

    let defaults = &[("http_server.port", "0")];

    let (server, port) = rust_kata_003::run(&[defaults, overrides].concat()).await;
    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
    }
}
