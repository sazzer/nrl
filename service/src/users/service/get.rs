use super::UsersService;
use crate::users::{Email, GetUserUseCase, UserID, UserModel, Username};
use async_trait::async_trait;

#[async_trait]
impl GetUserUseCase for UsersService {
    /// Get the user that has the given User ID.
    ///
    /// # Parameters
    /// - `user_id` - The ID of the user to get
    ///
    /// # Returns
    /// The user, if it was found. `None` if it doesn't exist.
    #[tracing::instrument(skip(self))]
    async fn get_user_by_id(&self, user_id: &UserID) -> Option<UserModel> {
        self.repository.get_user_by_id(user_id).await
    }

    /// Get the user that has the given Username.
    ///
    /// # Parameters
    /// - `username` - The username of the user to get
    ///
    /// # Returns
    /// The user, if it was found. `None` if it doesn't exist.
    #[tracing::instrument(skip(self))]
    async fn get_user_by_username(&self, username: &Username) -> Option<UserModel> {
        self.repository.get_user_by_username(username).await
    }

    /// Get the user that has the given Email.
    ///
    /// # Parameters
    /// - `email` - The email of the user to get
    ///
    /// # Returns
    /// The user, if it was found. `None` if it doesn't exist.
    #[tracing::instrument(skip(self))]
    async fn get_user_by_email(&self, email: &Email) -> Option<UserModel> {
        self.repository.get_user_by_email(email).await
    }
}
