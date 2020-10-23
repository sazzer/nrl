use crate::testserver::TestServer;
use actix_http::http::StatusCode;
use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;

#[actix_rt::test]
async fn test_start_blank_provider() {
    let sut = TestServer::new().await;

    let response = sut
        .inject(TestRequest::get().uri("/authentication/%20").to_request())
        .await;

    check!(response.status == StatusCode::NOT_FOUND);
    check!(response.header("content-type").unwrap() == "application/problem+json");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:nrl/2020:problems/not_found",
      "title": "The requested resource was not found",
      "status": 404
    }
    "###);
}

#[actix_rt::test]
async fn test_start_unknown_provider() {
    let sut = TestServer::new().await;

    let response = sut
        .inject(
            TestRequest::get()
                .uri("/authentication/unknown")
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::NOT_FOUND);
    check!(response.header("content-type").unwrap() == "application/problem+json");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
        {
          "type": "tag:nrl/2020:problems/not_found",
          "title": "The requested resource was not found",
          "status": 404
        }
        "###);
}
