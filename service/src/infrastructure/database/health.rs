use super::Database;
use crate::infrastructure::health::Healthchecker;
use async_trait::async_trait;

#[async_trait]
impl Healthchecker for Database {
    /// Check the health of the database.
    ///
    /// # Returns
    /// If the database connection is healthy then returns `Ok(())`.
    /// If the database connection is unhealthy then returns an error message from the connection pool.
    async fn check_health(&self) -> Result<(), String> {
        let conn = self
            .checkout()
            .await
            .map_err(|e| format!("Failed to checkout connection: {}", e))?;

        conn.query_one("SELECT 1", &[])
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        Ok(())
    }
}
