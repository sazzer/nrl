use crate::infrastructure::database::Database;
use std::sync::Arc;

/// Repository of users data.
pub struct UsersRepository {
    _database: Arc<Database>,
}

impl UsersRepository {
    /// Create a new users repository.
    ///
    /// # Parameters
    /// - `database` - The database connection
    pub fn new(database: Arc<Database>) -> Self {
        Self {
            _database: database,
        }
    }
}
