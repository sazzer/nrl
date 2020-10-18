use super::problem::UNAUTHORIZED;
use crate::authorization::SignedSecurityContext;
use crate::http::problem::Problem;
use actix_web::{dev::Payload, http::header, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};

impl FromRequest for SignedSecurityContext {
    type Config = ();
    type Error = Problem;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let authorization = req.headers().get(header::AUTHORIZATION);
        tracing::debug!("Processing authorization header: {:?}", authorization);

        match authorization {
            Some(value) => {
                match value
                    .to_str()
                    .ok()
                    .filter(|value| value.starts_with("Bearer "))
                    .map(|value| &value[7..])
                {
                    Some(ssc) => ok(Self(ssc.to_owned())),
                    None => err(UNAUTHORIZED.into()),
                }
            }
            None => err(UNAUTHORIZED.into()),
        }
    }
}
