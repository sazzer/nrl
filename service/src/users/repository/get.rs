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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::database::testing::TestDatabase;
    use crate::users::Authentication;
    use assert2::{check, let_assert};
    use nrl_testdatabase::seeddata::SeedUser;

    #[actix_rt::test]
    async fn test_get_by_id_unknown_user() {
        let db = TestDatabase::new().await;
        let sut = UsersRepository::new(db.db);

        let result = sut.get_user_by_id(&UserID::default()).await;
        check!(result.is_none());
    }

    #[actix_rt::test]
    async fn test_get_by_id_bare_user() {
        let seed_user = SeedUser {
            username: None,
            email: None,
            ..SeedUser::default()
        };

        let db = TestDatabase::new().await;
        db.seed(&seed_user).await;

        let sut = UsersRepository::new(db.db);

        let user_id = seed_user.user_id.clone().into();
        let result = sut.get_user_by_id(&user_id).await;

        let_assert!(Some(user) = result);

        check!(user.identity.id == user_id);
        check!(user.identity.version == seed_user.version);
        check!(user.identity.created == seed_user.created);
        check!(user.identity.updated == seed_user.updated);

        check!(user.data.username == None);
        check!(user.data.email == None);
        check!(user.data.display_name == seed_user.display_name.parse().unwrap());
        check!(user.data.authentications == vec![]);
    }

    #[actix_rt::test]
    async fn test_get_by_id_known_user() {
        let seed_user = SeedUser::default();

        let db = TestDatabase::new().await;
        db.seed(&seed_user).await;

        let sut = UsersRepository::new(db.db);

        let user_id = seed_user.user_id.clone().into();
        let result = sut.get_user_by_id(&user_id).await;

        let_assert!(Some(user) = result);

        check!(user.identity.id == user_id);
        check!(user.identity.version == seed_user.version);
        check!(user.identity.created == seed_user.created);
        check!(user.identity.updated == seed_user.updated);

        check!(user.data.username == seed_user.username.map(|v| v.parse().unwrap()));
        check!(user.data.email == seed_user.email.map(|v| v.parse().unwrap()));
        check!(user.data.display_name == seed_user.display_name.parse().unwrap());
        check!(user.data.authentications == vec![]);
    }

    #[actix_rt::test]
    async fn test_get_by_id_known_user_with_authentications() {
        let seed_user = SeedUser::default()
            .with_authentication("google", "googleUserId", "Google User")
            .with_authentication("twitter", "twitterUserId", "Twitter User");

        let db = TestDatabase::new().await;
        db.seed(&seed_user).await;

        let sut = UsersRepository::new(db.db);

        let user_id = seed_user.user_id.clone().into();
        let result = sut.get_user_by_id(&user_id).await;

        let_assert!(Some(user) = result);

        check!(user.identity.id == user_id);
        check!(user.identity.version == seed_user.version);
        check!(user.identity.created == seed_user.created);
        check!(user.identity.updated == seed_user.updated);

        check!(user.data.username == seed_user.username.map(|v| v.parse().unwrap()));
        check!(user.data.email == seed_user.email.map(|v| v.parse().unwrap()));
        check!(user.data.display_name == seed_user.display_name.parse().unwrap());
        check!(
            user.data.authentications
                == vec![
                    Authentication {
                        provider: "google".parse().unwrap(),
                        user: "googleUserId".parse().unwrap(),
                        display_name: "Google User".to_owned(),
                    },
                    Authentication {
                        provider: "twitter".parse().unwrap(),
                        user: "twitterUserId".parse().unwrap(),
                        display_name: "Twitter User".to_owned(),
                    },
                ]
        );
    }
}
