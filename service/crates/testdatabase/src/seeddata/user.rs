use super::SeedData;
use chrono::{DateTime, Timelike, Utc};
use postgres_types::ToSql;
use serde_json::json;
use uuid::Uuid;

/// Representation of a user ready to seed into the test database.
#[derive(Debug)]
pub struct SeedUser {
    pub user_id: Uuid,
    pub version: Uuid,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub authentications: serde_json::Value,
}

impl Default for SeedUser {
    fn default() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            version: Uuid::new_v4(),
            created: Utc::now().with_nanosecond(0).unwrap(),
            updated: Utc::now().with_nanosecond(0).unwrap(),
            username: format!("{}", Uuid::new_v4()),
            email: format!("{}@example.com", Uuid::new_v4()),
            display_name: format!("{}", Uuid::new_v4()),
            authentications: json!([]),
        }
    }
}

impl SeedUser {
    /// Add a new authentication detail to the user
    ///
    /// # Parameters
    /// - `provider` - The ID of the provider
    /// - `user` - The ID of the user with this provider
    /// - `display_name` - The display name of this user with this provider
    pub fn with_authentication<P, I, D>(mut self, provider: P, user: I, display_name: D) -> Self
    where
        P: Into<String>,
        I: Into<String>,
        D: Into<String>,
    {
        let new_authentication = json!(
            {
                "provider": provider.into(),
                "user": user.into(),
                "display_name": display_name.into(),
            }
        );

        self.authentications
            .as_array_mut()
            .unwrap()
            .push(new_authentication);

        self
    }
}

impl SeedData for SeedUser {
    fn sql(&self) -> &str {
        "INSERT INTO users(user_id, version, created, updated, username, email, display_name, authentications)
          VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
    }

    fn binds(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id,
            &self.version,
            &self.created,
            &self.updated,
            &self.username,
            &self.email,
            &self.display_name,
            &self.authentications,
        ]
    }
}
