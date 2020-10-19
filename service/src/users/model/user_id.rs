use crate::authorization::Principal;
use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::Serialize;
use std::str::FromStr;
use uuid::Uuid;

/// The actual ID of the user resource.
#[derive(Debug, PartialEq, FromSql, Serialize)]
pub struct UserID(Uuid);

impl Default for UserID {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<Uuid> for UserID {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<&UserID> for Principal {
    fn from(user_id: &UserID) -> Self {
        Self::User(user_id.0.to_string())
    }
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum UserIDParseError {
    #[error("The user ID is not well formed")]
    Malformed,
}

impl FromStr for UserID {
    type Err = UserIDParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(value).map(Self::from).map_err(|e| {
            tracing::warn!(e = ?e, "Malformed User ID");
            UserIDParseError::Malformed
        })
    }
}

impl ToSql for UserID {
    accepts!(UUID);

    to_sql_checked!();

    fn to_sql(
        &self,
        t: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }
}
