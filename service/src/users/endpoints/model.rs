use crate::users::{DisplayName, Email, UserID, UserModel, Username};
use actix_http::{http::StatusCode, Error, Response};
use actix_web::{http::header, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use futures::future::{ok, Ready};
use serde::Serialize;
use uuid::Uuid;

/// HTTP API model representing a user.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserApiModel {
    pub user_id: UserID,
    #[serde(skip)]
    pub version: Uuid,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub username: Option<Username>,
    pub email: Option<Email>,
    pub display_name: DisplayName,
}

impl From<UserModel> for UserApiModel {
    fn from(user: UserModel) -> Self {
        Self {
            user_id: user.identity.id,
            version: user.identity.version,
            created: user.identity.created,
            updated: user.identity.updated,
            username: user.data.username,
            email: user.data.email,
            display_name: user.data.display_name,
        }
    }
}

impl Responder for UserApiModel {
    type Error = Error;
    type Future = Ready<Result<Response, Error>>;

    /// Generate an HTTP Response for the User.
    ///
    /// This will include HTTP headers for the Etag and Last Modified dates based on the model data.
    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        let response = HttpResponse::build(StatusCode::OK)
            .set(header::ETag(header::EntityTag::strong(
                self.version.to_string(),
            )))
            .set(header::CacheControl(vec![
                header::CacheDirective::Private,
                header::CacheDirective::MaxAge(3600),
            ]))
            .json(self);

        ok(response)
    }
}
