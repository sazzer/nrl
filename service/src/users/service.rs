mod get;

use super::repository::UsersRepository;

/// The users service to work with user records.
pub struct UsersService {
    repository: UsersRepository,
}

impl UsersService {
    /// Create a new Users service.
    ///
    /// # Parameters
    /// - `repository` - The repository of user records
    pub const fn new(repository: UsersRepository) -> Self {
        Self { repository }
    }
}
