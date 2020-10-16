use super::UsersService;
use crate::users::{GetUserUseCase, UserID, UserModel};
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
}
