mod get;
mod model;

use crate::infrastructure::database::Database;
use std::sync::Arc;

/// Repository of users data.
pub struct UsersRepository {
    database: Arc<Database>,
}

impl UsersRepository {
    /// Create a new users repository.
    ///
    /// # Parameters
    /// - `database` - The database connection
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}
