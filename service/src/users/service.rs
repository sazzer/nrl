use super::repository::UsersRepository;

/// The users service to work with user records.
pub struct UsersService {
    _repository: UsersRepository,
}

impl UsersService {
    /// Create a new Users service.
    ///
    /// # Parameters
    /// - `repository` - The repository of user records
    pub const fn new(repository: UsersRepository) -> Self {
        Self {
            _repository: repository,
        }
    }
}
