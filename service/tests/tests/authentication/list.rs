use crate::testserver::TestServer;
use actix_http::http::StatusCode;
use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;

#[actix_rt::test]
async fn test_list_authentication_providers() {
    let sut = TestServer::new().await;

    let response = sut
        .inject(TestRequest::get().uri("/authentication").to_request())
        .await;

    check!(response.status == StatusCode::OK);
    check!(response.header("content-type").unwrap() == "application/json");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "entries": [
        "google"
      ],
      "pagination": {
        "offset": 0,
        "total": 1
      }
    }
    "###);
}
