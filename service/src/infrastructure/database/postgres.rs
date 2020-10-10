use deadpool::managed::{Object, PoolError};
use deadpool_postgres::{ClientWrapper, Manager, ManagerConfig, Pool, RecyclingMethod};
use std::str::FromStr;

/// Wrapper around the database connections
pub struct Database {
    /// The actual database connection pool.
    pub(super) pool: Pool,
}

impl Database {
    /// Create a new database connection.
    ///
    /// # Parameters
    /// - `url` - The database connection URL.
    pub fn new<S>(url: S) -> Self
    where
        S: Into<String>,
    {
        let url = url.into();
        tracing::info!(url = ?url, "Connecting to database");

        let pg_config = tokio_postgres::Config::from_str(&url).unwrap();

        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };
        let mgr = Manager::from_config(pg_config, tokio_postgres::NoTls, mgr_config);
        let pool = Pool::new(mgr, 16);

        Self { pool }
    }

    /// Check out a connection from the database pool in order to make queries
    ///
    /// # Returns
    /// The connection to use
    ///
    /// # Errors
    /// If the pool is unable to return a viable connection
    pub async fn checkout(
        &self,
    ) -> Result<Object<ClientWrapper, tokio_postgres::Error>, PoolError<tokio_postgres::Error>>
    {
        let conn = self.pool.get().await?;

        Ok(conn)
    }
}
