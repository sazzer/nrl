use crate::authorization::{Authorizer, SecurityContext};
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use futures::future::{ok, Ready};

impl FromRequest for Authorizer {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let security_context = SecurityContext::from_request(req, payload)
            .into_inner()
            .ok();

        ok(Self::new(security_context))
    }
}
