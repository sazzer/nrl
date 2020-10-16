use crate::testserver::TestServer;
use actix_http::http::StatusCode;
use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;
use nrl_testdatabase::seeddata::SeedUser;

#[actix_rt::test]
async fn get_unknown_user() {
    let sut = TestServer::new().await;

    let response = sut
        .inject(
            TestRequest::get()
                .uri("/users/a4e7e6c3-7e2b-4c0f-be6e-bfcd9ee87b35")
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::NOT_FOUND);
    check!(response.header("content-type").unwrap() == "application/problem+json");
    check!(response.header("cache-control") == None);
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:nrl/2020:problems/not_found",
      "title": "The requested resource was not found",
      "status": 404
    }
    "###);
}

#[actix_rt::test]
async fn get_malformed_id() {
    let sut = TestServer::new().await;

    let response = sut
        .inject(TestRequest::get().uri("/users/not-a-uuid").to_request())
        .await;

    check!(response.status == StatusCode::NOT_FOUND);
    check!(response.header("content-type").unwrap() == "application/problem+json");
    check!(response.header("cache-control") == None);
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:nrl/2020:problems/not_found",
      "title": "The requested resource was not found",
      "status": 404
    }
    "###);
}

#[actix_rt::test]
async fn get_known_user() {
    let seed_user = SeedUser {
        user_id: "dfbe0860-5c5e-4e3a-a4b0-c1d02510e768".parse().unwrap(),
        version: "fb3b7922-b79b-47a2-a273-31b8d4a32e84".parse().unwrap(),
        created: "2020-10-12T06:40:41Z".parse().unwrap(),
        updated: "2020-10-16T06:40:41Z".parse().unwrap(),
        username: Some("testuser".to_owned()),
        email: Some("testuser@example.com".to_owned()),
        display_name: "Test User".to_owned(),
        ..SeedUser::default()
    }
    .with_authentication("twitter", "twitterUserId", "@testuser")
    .with_authentication("google", "googleUserId", "testuser@example.com");

    let sut = TestServer::new().await;
    sut.seed(&seed_user).await;

    let response = sut
        .inject(
            TestRequest::get()
                .uri("/users/dfbe0860-5c5e-4e3a-a4b0-c1d02510e768")
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::OK);
    check!(response.header("content-type").unwrap() == "application/json");
    check!(response.header("cache-control").unwrap() == "private, max-age=3600");
    check!(response.header("etag").unwrap() == "\"fb3b7922-b79b-47a2-a273-31b8d4a32e84\"");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "userId": "dfbe0860-5c5e-4e3a-a4b0-c1d02510e768",
      "created": "2020-10-12T06:40:41Z",
      "updated": "2020-10-16T06:40:41Z",
      "username": "testuser",
      "email": "testuser@example.com",
      "displayName": "Test User",
      "authentications": [
        {
          "provider": "google",
          "user": "googleUserId",
          "displayName": "testuser@example.com"
        },
        {
          "provider": "twitter",
          "user": "twitterUserId",
          "displayName": "@testuser"
        }
      ]
    }
    "###);
}

#[actix_rt::test]
async fn get_known_bare_user() {
    let seed_user = SeedUser {
        user_id: "dfbe0860-5c5e-4e3a-a4b0-c1d02510e768".parse().unwrap(),
        version: "fb3b7922-b79b-47a2-a273-31b8d4a32e84".parse().unwrap(),
        created: "2020-10-12T06:40:41Z".parse().unwrap(),
        updated: "2020-10-16T06:40:41Z".parse().unwrap(),
        username: None,
        email: None,
        display_name: "Test User".to_owned(),
        ..SeedUser::default()
    };

    let sut = TestServer::new().await;
    sut.seed(&seed_user).await;

    let response = sut
        .inject(
            TestRequest::get()
                .uri("/users/dfbe0860-5c5e-4e3a-a4b0-c1d02510e768")
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::OK);
    check!(response.header("content-type").unwrap() == "application/json");
    check!(response.header("cache-control").unwrap() == "private, max-age=3600");
    check!(response.header("etag").unwrap() == "\"fb3b7922-b79b-47a2-a273-31b8d4a32e84\"");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "userId": "dfbe0860-5c5e-4e3a-a4b0-c1d02510e768",
      "created": "2020-10-12T06:40:41Z",
      "updated": "2020-10-16T06:40:41Z",
      "displayName": "Test User",
      "authentications": []
    }
    "###);
}
