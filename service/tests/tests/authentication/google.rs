use std::collections::HashMap;

use crate::testserver::TestServer;
use actix_http::http::StatusCode;
use actix_web::test::TestRequest;
use assert2::{check, let_assert};
use url::Url;

#[actix_rt::test]
async fn test_start_google() {
    let sut = TestServer::new().await;

    let response = sut
        .inject(
            TestRequest::get()
                .uri("/authentication/google?redirect_url=http%3A%2F%2Fwww.example.com/callback")
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::FOUND);
    check!(
        response.header("set-cookie").unwrap()
            == "redirect_url=http://www.example.com/callback; HttpOnly; Path=/authentication"
    );

    let_assert!(Some(location) = response.header("location"));
    let location = Url::parse(location.to_str().unwrap()).unwrap();

    check!(location.scheme() == "https");
    check!(location.username() == "");
    check!(location.password() == None);
    check!(location.host_str() == Some("accounts.google.com"));
    check!(location.port() == None);
    check!(location.path() == "/o/oauth2/v2/auth");
    check!(location.fragment() == None);
    check!(!location.cannot_be_a_base());

    let query: HashMap<std::borrow::Cow<'_, str>, std::borrow::Cow<'_, str>> =
        location.query_pairs().collect();
    check!(query["client_id"] == "googleClientId");
    check!(query["response_type"] == "code");
    check!(query["scope"] == "openid email profile");
    check!(query["redirect_uri"] == "http://www.example.com/authentication/google/complete");
    check!(query.get("state").is_some());
}
