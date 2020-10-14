use super::{migrations::migrate, Database};
use nrl_testdatabase::seeddata::SeedData;
use std::sync::Arc;

/// Wrapper around the Postgres test database and the NRL Database connection
pub struct TestDatabase {
    #[allow(dead_code)] // Needed for it's lifetime
    container: nrl_testdatabase::TestDatabase,
    pub db: Arc<Database>,
}

impl TestDatabase {
    /// Create a new test database to work with
    pub async fn new() -> Self {
        let container = nrl_testdatabase::TestDatabase::default();
        let db = Database::new(&container.url);

        migrate(&db).await;

        Self {
            container,
            db: Arc::new(db),
        }
    }

    /// Seed some data into the database
    ///
    /// # Parameters
    /// - `data` - The data to seed
    pub async fn seed(&self, data: &dyn SeedData) -> &Self {
        self.container.seed(data).await;
        self
    }
}
