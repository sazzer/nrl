use super::UsersRepository;
use crate::users::{Email, UserID, UserModel, Username};

impl UsersRepository {
    /// Get the user out of the database that has the given User ID.
    ///
    /// # Parameters
    /// - `user_id` - The ID of the user to get
    ///
    /// # Returns
    /// The user, if it was found. `None` if it doesn't exist.
    pub async fn get_user_by_id(&self, user_id: UserID) -> Option<UserModel> {
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

    /// Get the user out of the database that has the given Username.
    ///
    /// # Parameters
    /// - `username` - The Username of the user to get
    ///
    /// # Returns
    /// The user, if it was found. `None` if it doesn't exist.
    pub async fn get_user_by_username(&self, username: Username) -> Option<UserModel> {
        tracing::debug!(username = ?username, "Getting user by username");

        let conn = self
            .database
            .checkout()
            .await
            .expect("Failed to get connection");

        let row = conn
            .query_opt("SELECT * FROM users WHERE username = $1", &[&username])
            .await
            .unwrap()
            .map(UserModel::from);

        tracing::debug!(username = ?username, user = ?row, "Got user by username");

        row
    }

    /// Get the user out of the database that has the given Email.
    ///
    /// # Parameters
    /// - `email` - The Email of the user to get
    ///
    /// # Returns
    /// The user, if it was found. `None` if it doesn't exist.
    pub async fn get_user_by_email(&self, email: Email) -> Option<UserModel> {
        tracing::debug!(email = ?email, "Getting user by email");

        let conn = self
            .database
            .checkout()
            .await
            .expect("Failed to get connection");

        let row = conn
            .query_opt("SELECT * FROM users WHERE email = $1", &[&email])
            .await
            .unwrap()
            .map(UserModel::from);

        tracing::debug!(email = ?email, user = ?row, "Got user by email");

        row
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::database::testing::TestDatabase;
    use assert2::check;

    #[actix_rt::test]
    async fn test_get_by_id_unknown_user() {
        let db = TestDatabase::new().await;
        let sut = UsersRepository::new(db.db);

        let result = sut.get_user_by_id(UserID::default()).await;
        check!(result.is_none());
    }
}
