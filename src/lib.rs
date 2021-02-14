mod configuration;
mod crates_io_client;
mod domain;
mod postgres_client;
mod resolver;
mod routes;
mod startup;
pub mod telemetry;

pub use startup::run;
