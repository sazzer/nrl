use super::problem::UNAUTHORIZED;
use crate::authorization::{
    AuthorizeSecurityContextUseCase, SecurityContext, SignedSecurityContext,
};
use crate::http::problem::Problem;
use actix_web::{dev::Payload, web::Data, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use std::sync::Arc;

impl FromRequest for SecurityContext {
    type Config = ();
    type Error = Problem;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let service: &Data<Arc<dyn AuthorizeSecurityContextUseCase>> = req.app_data().unwrap();

        match SignedSecurityContext::from_request(req, payload).into_inner() {
            Ok(ssc) => match service.authorize(ssc) {
                Ok(sc) => ok(sc),
                Err(e) => {
                    tracing::warn!(error = ?e, "Failed to authorize security context");
                    err(UNAUTHORIZED.into())
                }
            },
            Err(e) => err(e),
        }
    }
}
