use crate::model::Identity;
use crate::users::{Authentication, UserData, UserModel};
use tokio_postgres::Row;

impl From<Row> for UserModel {
    fn from(row: Row) -> Self {
        let authentications: Vec<Authentication> =
            serde_json::from_value(row.get("authentications")).unwrap();

        Self {
            identity: Identity {
                id: row.get("user_id"),
                version: row.get("version"),
                created: row.get("created"),
                updated: row.get("updated"),
            },
            data: UserData {
                username: row.get("username"),
                email: row.get("email"),
                display_name: row.get("display_name"),
                authentications,
            },
        }
    }
}
