use super::UsersRepository;
use crate::users::{UserID, UserModel};

impl UsersRepository {
    /// Get the user out of the database that has the given User ID.
    ///
    /// # Parameters
    /// - `user_id` - The ID of the user to get
    ///
    /// # Returns
    /// The user, if it was found. `None` if it doesn't exist.
    pub async fn get_user_by_id(&self, user_id: &UserID) -> Option<UserModel> {
        tracing::debug!(user_id = ?user_id, "Getting user by ID");

        let conn = self
            .database
            .checkout()
            .await
            .expect("Failed to get connection");

        let row = conn
            .query_opt("SELECT * FROM users WHERE user_id = $1", &[&user_id])
            .await
            .unwrap()
            .map(UserModel::from);

        tracing::debug!(user_id = ?user_id, user = ?row, "Got user by ID");

        row
    }
}
