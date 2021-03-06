use crate::users::{DisplayName, Email, ProviderID, ProviderUserID, UserID, UserModel, Username};
use actix_http::{http::StatusCode, Error, Response};
use actix_web::{http::header, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use futures::future::{ok, Ready};
use serde::Serialize;
use std::cmp::Ordering;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<Username>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    pub display_name: DisplayName,
    pub authentications: Vec<AuthenticationApiModel>,
}

/// HTTP API model representing a simple user, for cases where the client is not authorized to all the details.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleUserApiModel {
    pub user_id: UserID,
    #[serde(skip)]
    pub version: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<Username>,
    pub display_name: DisplayName,
}

/// HTTP API model representing the authentication details of a user.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationApiModel {
    pub provider: ProviderID,
    pub user: ProviderUserID,
    pub display_name: String,
}

impl From<UserModel> for UserApiModel {
    fn from(user: UserModel) -> Self {
        let mut authentications: Vec<AuthenticationApiModel> = user
            .data
            .authentications
            .into_iter()
            .map(|a| AuthenticationApiModel {
                provider: a.provider,
                user: a.user,
                display_name: a.display_name,
            })
            .collect();

        authentications.sort_by(|a, b| {
            match (
                a.provider.partial_cmp(&b.provider),
                a.user.partial_cmp(&b.user),
                a.display_name.partial_cmp(&b.display_name),
            ) {
                (Some(Ordering::Less), _, _)
                | (Some(Ordering::Equal), Some(Ordering::Less), _)
                | (Some(Ordering::Equal), Some(Ordering::Equal), Some(Ordering::Less)) => {
                    Ordering::Less
                }
                (Some(Ordering::Greater), _, _)
                | (Some(Ordering::Equal), Some(Ordering::Greater), _)
                | (Some(Ordering::Equal), Some(Ordering::Equal), Some(Ordering::Greater)) => {
                    Ordering::Greater
                }
                (_, _, _) => Ordering::Equal,
            }
        });

        Self {
            user_id: user.identity.id,
            version: user.identity.version,
            created: user.identity.created,
            updated: user.identity.updated,
            username: user.data.username,
            email: user.data.email,
            display_name: user.data.display_name,
            authentications,
        }
    }
}

impl From<UserModel> for SimpleUserApiModel {
    fn from(user: UserModel) -> Self {
        Self {
            user_id: user.identity.id,
            version: user.identity.version,
            username: user.data.username,
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

impl Responder for SimpleUserApiModel {
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
