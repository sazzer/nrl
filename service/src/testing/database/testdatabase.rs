use super::postgres::Postgres;
use crate::infrastructure::database::{migrations::migrate, Database};
use lazy_static::lazy_static;
use std::sync::Arc;
use testcontainers::{clients::Cli, Container, Docker};

lazy_static! {
    static ref DOCKER: Cli = Cli::default();
}

/// Wrapper around the Postgres database to use in the tests.
pub struct TestDatabase {
    #[allow(dead_code)]
    node: Container<'static, Cli, Postgres>,
    pub database: Arc<Database>,
}

impl TestDatabase {
    pub async fn new() -> Self {
        tracing::info!("Starting Postgres database");
        let node = DOCKER.run(Postgres::default());

        let host = "localhost".to_owned();
        let port = node.get_host_port(5432).unwrap();
        let url = format!("postgres://postgres@{}:{}", host, port);
        tracing::info!(url = ?url, "Running postgres");

        let db = Database::new(url);
        migrate(&db).await;

        Self {
            node,
            database: Arc::new(db),
        }
    }
}
