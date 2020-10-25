use super::UsersRepository;
use crate::users::{UserData, UserID, UserModel};
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum CreateUserError {
    #[error("An unknown error occurred")]
    UnknownError,
}

impl UsersRepository {
    /// Create a new user record.
    ///
    /// # Parameters
    /// - `data` - The details of the user to create
    ///
    /// # Returns
    /// The newly created user.
    pub async fn create_user(&self, data: UserData) -> Result<UserModel, CreateUserError> {
        tracing::debug!(data = ?data, "Creating new user");

        let conn = self
            .database
            .checkout()
            .await
            .expect("Failed to get connection");

        let now = Utc::now();
        let authentications = serde_json::to_value(data.authentications).unwrap();

        let user = conn.query_one(
          "INSERT INTO users(user_id, version, created, updated, username, email, display_name, authentications)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *",
          &[
            &UserID::default(),
            &Uuid::new_v4(),
            &now,
            &now,
            &data.username,
            &data.email,
            &data.display_name,
            &authentications,
          ]
        )
          .await
          .map(UserModel::from)
            .map_err(|e| {
              CreateUserError::UnknownError
            });

        tracing::debug!(user = ?user, "Created new user");

        user
    }
}
