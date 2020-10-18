use super::model::UserApiModel;
use crate::authorization::SecurityContext;
use crate::http::problem::{Problem, NOT_FOUND};
use crate::users::{GetUserUseCase, UsersService};
use actix_web::web::{Data, Path};
use std::sync::Arc;

/// HTTP Handler for getting a user by ID.
///
/// # Parameters
///
/// # Returns
/// The HTTP Model for the response
#[tracing::instrument(
    fields(http_method = "GET", http_path = "/users/:id"),
    skip(user_service)
)]
pub async fn get_user_by_id(
    path: Path<String>,
    ssc: Option<SecurityContext>,
    user_service: Data<Arc<UsersService>>,
) -> Result<UserApiModel, Problem> {
    let user_id = path.0.parse().map_err(|e| {
        tracing::warn!(user_id = ?path.0, e = ?e, "Received invalid User ID");
        Problem::new(NOT_FOUND)
    })?;

    let user = user_service.get_user_by_id(&user_id).await;

    user.map(UserApiModel::from).ok_or_else(|| NOT_FOUND.into())
}
