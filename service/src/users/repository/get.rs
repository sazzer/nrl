use super::UsersRepository;
use crate::users::{ProviderID, ProviderUserID, UserID, UserModel};
use serde_json::json;

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

    /// Get the user out of the database that has the given authentication details.
    ///
    /// # Parameters
    /// - `provider_id` - The ID of the authentication provider that the user is authenticating with
    /// - `user_id` - The ID of the user with this authentication provider
    ///
    /// # Returns
    /// The user, if it was found. `None` if it doesn't exist.
    pub async fn get_user_by_authentication(
        &self,
        provider_id: &ProviderID,
        user_id: &ProviderUserID,
    ) -> Option<UserModel> {
        tracing::debug!(provider_id = ?provider_id, user_id = ?user_id, "Getting user by authentication details");

        let conn = self
            .database
            .checkout()
            .await
            .expect("Failed to get connection");

        let authentication_bind = json!([{
          "provider": provider_id,
          "user": user_id
        }]);

        let row = conn
            .query_opt(
                "SELECT * FROM users WHERE authentications @> $1",
                &[&authentication_bind],
            )
            .await
            .unwrap()
            .map(UserModel::from);

        tracing::debug!(provider_id = ?provider_id, user_id = ?user_id, user = ?row, "Got user by authentication details");

        row
    }
}
