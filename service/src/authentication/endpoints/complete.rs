use crate::authentication::{
    AuthenticationService, AuthenticatorID, CompleteAuthenticationUseCase,
};
use crate::http::problem::{Problem, NOT_FOUND};
use actix_web::{
    web::{Data, HttpRequest, HttpResponse, Path, Query},
    HttpMessage,
};
use std::{collections::HashMap, sync::Arc};
use uritemplate::UriTemplate;

/// HTTP Handler for listing the authenticators we can use.
///
/// # Parameters
/// - `authentication_service` - The authentication service.
///
/// # Returns
/// The HTTP Model for the response
#[tracing::instrument(
    fields(http_method = "GET", http_path = "/authentication/:id/complete"),
    skip(authentication_service, req)
)]
pub async fn complete_authentication(
    authentication_service: Data<Arc<AuthenticationService>>,
    params: Query<HashMap<String, String>>,
    path: Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, Problem> {
    let authenticator: AuthenticatorID = path.0.parse().map_err(|_| Problem::from(NOT_FOUND))?;
    let redirect_url = req
        .cookie("redirect_url")
        .map(|c| c.value().to_owned())
        .ok_or_else(|| Problem::from(NOT_FOUND))?;

    let (user, security_context, signed_security_context) = authentication_service
        .complete(authenticator, params.0)
        .await
        .unwrap();

    tracing::info!(user = ?user, security_context = ?security_context, signed_security_context = ?signed_security_context, "Authenticated");

    let redirect_url = UriTemplate::new(&format!(
        "{}#user_id={{user_id}}&token={{token}}&expires={{expires}}",
        redirect_url
    ))
    .set("user_id", user.identity.id)
    .set("token", signed_security_context)
    .set("expires", security_context.expires.to_rfc3339())
    .build();

    Ok(HttpResponse::Found()
        .header("Location", redirect_url)
        .finish())
}
