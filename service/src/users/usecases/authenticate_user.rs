use crate::users::{Authentication, DisplayName, Email, UserModel};
use async_trait::async_trait;

#[allow(clippy::empty_enum)]
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum AuthenticateUserError {
    #[error("The email address is registered to another user")]
    DuplicateEmail,

    #[error("An unknown error occurred")]
    UnknownError,
}

#[async_trait]
pub trait AuthenticateUserUseCase {
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
        authentication: Authentication,
        display_name: DisplayName,
        email: Option<Email>,
    ) -> Result<UserModel, AuthenticateUserError>;
}
