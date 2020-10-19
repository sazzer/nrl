use super::model::{SimpleUserApiModel, UserApiModel};
use crate::authorization::{Authorizer, Principal};
use crate::http::problem::{Problem, NOT_FOUND};
use crate::users::{GetUserUseCase, UsersService};
use actix_web::{
    web::{Data, Path},
    Either,
};
use std::sync::Arc;

/// HTTP Handler for getting a user by ID.
///
/// # Parameters
/// - `path` - The URL Path fields
/// - `authorizer` - The means to authorize the request
/// - `user_service` - The users service to access the user records.
///
/// # Returns
/// The HTTP Model for the response
#[tracing::instrument(
    fields(http_method = "GET", http_path = "/users/:id"),
    skip(user_service, authorizer)
)]
pub async fn get_user_by_id(
    path: Path<String>,
    authorizer: Authorizer,
    user_service: Data<Arc<UsersService>>,
) -> Result<Either<SimpleUserApiModel, UserApiModel>, Problem> {
    let user_id = path.0.parse().map_err(|e| {
        tracing::warn!(user_id = ?path.0, e = ?e, "Received invalid User ID");
        Problem::new(NOT_FOUND)
    })?;

    let user = user_service.get_user_by_id(&user_id).await;

    let result = if authorizer
        .begin()
        .same_principal(&Principal::from(&user_id))
        .to_result()
        .is_ok()
    {
        user.map(|u| Either::B(u.into()))
    } else {
        user.map(|u| Either::A(u.into()))
    };

    result.ok_or_else(|| NOT_FOUND.into())
}
