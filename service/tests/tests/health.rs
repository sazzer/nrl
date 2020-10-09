use crate::testserver::TestServer;
use actix_http::http::StatusCode;
use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;

#[actix_rt::test]
async fn test_healthchecks() {
    let sut = TestServer::new().await;

    let response = sut
        .inject(TestRequest::get().uri("/health").to_request())
        .await;

    check!(response.status == StatusCode::OK);
    check!(response.header("content-type").unwrap() == "application/json");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
        {
          "healthy": true,
          "components": {
            "db": {
              "healthy": true
            }
          }
        }
        "###);
}
