use crate::users::{
    repository::SaveUserError, service::UsersService, AuthenticateUserError,
    AuthenticateUserUseCase, Authentication, DisplayName, Email, UserData, UserModel,
};
use async_trait::async_trait;

#[async_trait]
impl AuthenticateUserUseCase for UsersService {
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
    ) -> Result<UserModel, AuthenticateUserError> {
        if let Some(user) = self
            .repository
            .get_user_by_authentication(&authentication.provider, &authentication.user)
            .await
        {
            // A user with these authentication details already exists, so return it as-is
            Ok(user)
        } else {
            // A user with these authentication details don't exist, so lets create a new one.
            self.repository
                .create_user(UserData {
                    username: None,
                    email,
                    display_name,
                    authentications: vec![authentication],
                })
                .await
                .map_err(|e| match e {
                    SaveUserError::DuplicateEmail => AuthenticateUserError::DuplicateEmail,
                    SaveUserError::UnknownError => AuthenticateUserError::UnknownError,
                })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::database::testing::TestDatabase;
    use crate::users::{config::Config, Authentication};
    use assert2::{check, let_assert};
    use nrl_testdatabase::seeddata::SeedUser;

    #[actix_rt::test]
    async fn test_authenticate_known_user() {
        let seed_user =
            SeedUser::default().with_authentication("twitter", "twitterUserId", "Twitter User");

        let db = TestDatabase::new().await;
        db.seed(&seed_user).await;

        let sut = Config::new(db.db).service;

        let user_id = seed_user.user_id.clone().into();
        let result = sut
            .authenticate_user(
                Authentication {
                    provider: "twitter".parse().unwrap(),
                    user: "twitterUserId".parse().unwrap(),
                    display_name: "Twitter User".parse().unwrap(),
                },
                "Twitter User".parse().unwrap(),
                None,
            )
            .await;

        let_assert!(Ok(user) = result);

        check!(user.identity.id == user_id);
        check!(user.identity.version == seed_user.version);
        check!(user.identity.created == seed_user.created);
        check!(user.identity.updated == seed_user.updated);

        check!(user.data.username == seed_user.username.map(|v| v.parse().unwrap()));
        check!(user.data.email == seed_user.email.map(|v| v.parse().unwrap()));
        check!(user.data.display_name == seed_user.display_name.parse().unwrap());
        check!(
            user.data.authentications
                == vec![Authentication {
                    provider: "twitter".parse().unwrap(),
                    user: "twitterUserId".parse().unwrap(),
                    display_name: "Twitter User".to_owned(),
                },]
        );
    }

    #[actix_rt::test]
    async fn test_authenticate_new_user_no_email() {
        let db = TestDatabase::new().await;

        let sut = Config::new(db.db).service;

        let result = sut
            .authenticate_user(
                Authentication {
                    provider: "twitter".parse().unwrap(),
                    user: "twitterUserId".parse().unwrap(),
                    display_name: "Twitter User".parse().unwrap(),
                },
                "Twitter User".parse().unwrap(),
                None,
            )
            .await;

        let_assert!(Ok(user) = result);

        check!(user.data.username == None);
        check!(user.data.email == None);
        check!(user.data.display_name == "Twitter User".parse().unwrap());
        check!(
            user.data.authentications
                == vec![Authentication {
                    provider: "twitter".parse().unwrap(),
                    user: "twitterUserId".parse().unwrap(),
                    display_name: "Twitter User".to_owned(),
                },]
        );
    }

    #[actix_rt::test]
    async fn test_authenticate_new_user_with_email() {
        let db = TestDatabase::new().await;

        let sut = Config::new(db.db).service;

        let result = sut
            .authenticate_user(
                Authentication {
                    provider: "twitter".parse().unwrap(),
                    user: "twitterUserId".parse().unwrap(),
                    display_name: "Twitter User".parse().unwrap(),
                },
                "Twitter User".parse().unwrap(),
                Some("testuser@example.com".parse().unwrap()),
            )
            .await;

        let_assert!(Ok(user) = result);

        check!(user.data.username == None);
        check!(user.data.email == Some("testuser@example.com".parse().unwrap()));
        check!(user.data.display_name == "Twitter User".parse().unwrap());
        check!(
            user.data.authentications
                == vec![Authentication {
                    provider: "twitter".parse().unwrap(),
                    user: "twitterUserId".parse().unwrap(),
                    display_name: "Twitter User".to_owned(),
                },]
        );
    }

    #[actix_rt::test]
    async fn test_authenticate_new_user_duplicate_email() {
        let seed_user = SeedUser {
            email: Some("testuser@example.com".to_owned()),
            ..SeedUser::default()
        };

        let db = TestDatabase::new().await;
        db.seed(&seed_user).await;

        let sut = Config::new(db.db).service;

        let result = sut
            .authenticate_user(
                Authentication {
                    provider: "twitter".parse().unwrap(),
                    user: "twitterUserId".parse().unwrap(),
                    display_name: "Twitter User".parse().unwrap(),
                },
                "Twitter User".parse().unwrap(),
                Some("testuser@example.com".parse().unwrap()),
            )
            .await;

        let_assert!(Err(err) = result);
        check!(err == AuthenticateUserError::DuplicateEmail);
    }
}
