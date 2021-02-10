use rust_kata_003::telemetry;
use uuid::Uuid;

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

    let defaults = &[
        ("http_server.port", "0"),
        (
            "postgres.database_name",
            &format!("test-{}", Uuid::new_v4()),
        ),
    ];

    let (server, port, configuration) = rust_kata_003::run(&[defaults, overrides].concat()).await;

    let server_pool = configuration.postgres.server_pool();

    sqlx::query(&format!(
        r#"CREATE DATABASE "{}""#,
        configuration.postgres.database_name
    ))
    .execute(&server_pool)
    .await
    .unwrap();

    let database_pool = configuration.postgres.database_pool();

    sqlx::migrate!("./migrations")
        .run(&database_pool)
        .await
        .unwrap();

    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
    }
}
