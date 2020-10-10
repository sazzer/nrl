use super::postgres::Postgres;
use lazy_static::lazy_static;
use testcontainers::{clients::Cli, Container, Docker};

lazy_static! {
    static ref DOCKER: Cli = Cli::default();
}

/// Wrapper around the Postgres database to use in the tests.
pub struct TestDatabase {
    #[allow(dead_code)]
    node: Container<'static, Cli, Postgres>,
    pub host: String,
    pub port: u16,
    pub url: String,
}

impl Default for TestDatabase {
    fn default() -> Self {
        tracing::info!("Starting Postgres database");
        let node = DOCKER.run(Postgres::default());

        let host = "localhost".to_owned();
        let port = node.get_host_port(5432).unwrap();
        let url = format!("postgres://postgres@{}:{}", host, port);
        tracing::info!(url = ?url, "Running postgres");

        Self {
            node,
            host,
            port,
            url,
        }
    }
}
