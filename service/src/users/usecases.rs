use super::{Email, UserID, UserModel, Username};
use async_trait::async_trait;

#[async_trait]
pub trait GetUserUseCase {
    /// Get the user that has the given User ID.
    ///
    /// # Parameters
    /// - `user_id` - The ID of the user to get
    ///
    /// # Returns
    /// The user, if it was found. `None` if it doesn't exist.
    async fn get_user_by_id(&self, user_id: &UserID) -> Option<UserModel>;

    /// Get the user that has the given Username.
    ///
    /// # Parameters
    /// - `username` - The username of the user to get
    ///
    /// # Returns
    /// The user, if it was found. `None` if it doesn't exist.
    async fn get_user_by_username(&self, username: &Username) -> Option<UserModel>;

    /// Get the user that has the given Email.
    ///
    /// # Parameters
    /// - `email` - The email of the user to get
    ///
    /// # Returns
    /// The user, if it was found. `None` if it doesn't exist.
    async fn get_user_by_email(&self, email: &Email) -> Option<UserModel>;
}
