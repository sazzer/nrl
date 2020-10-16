use super::{UserID, UserModel};
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
}
