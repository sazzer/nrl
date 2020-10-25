use crate::users::{
    service::UsersService, AuthenticateUserError, AuthenticateUserUseCase, Authentication,
    DisplayName, Email, UserModel,
};
use async_trait::async_trait;

#[async_trait]
impl AuthenticateUserUseCase for UsersService {
    /// Authenticate a user.
    /// If a user with the provided `Authentication` details already exists then return it as-is.
    /// If a user with the provided `Authentication` details doesn't exist then creates a new one first.
    ///
    /// # Parameters
    /// - `authentication` - The authentication details of the user.
    /// - `display_name` - The display name for the user.
    /// - `email` - The email address for the user, if known.
    ///
    /// # Returns
    /// The user that we have authenticated as.
    async fn authenticate_user(
        &self,
        _authentication: Authentication,
        _display_name: DisplayName,
        _email: Option<Email>,
    ) -> Result<UserModel, AuthenticateUserError> {
        todo!()
    }
}
