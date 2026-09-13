use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use f5a_services::users::om::UserPage;
use serde_json::json;
use tower::ServiceExt;

use crate::{
    setup::TestContext,
    test_ext::IntoValue,
    users::migrations::{insert_bluebird_user, insert_chameleon_user, insert_joaquin_user},
};

fn create_pagination_query(page: i32, page_size: i32) -> String {
    format!("/api/users?page={}&page_size={}", page, page_size)
}

#[tokio::test]
async fn it_reads_paginated_users() {
    let ctx = TestContext::new().await;
    ctx.setup_db_schema().await;

    insert_joaquin_user(ctx.db.as_ref()).await.unwrap();
    insert_bluebird_user(ctx.db.as_ref()).await.unwrap();
    insert_chameleon_user(ctx.db.as_ref()).await.unwrap();

    let app = ctx.configure();

    let req = Request::get(create_pagination_query(0, 1))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let value = response.into_value::<Vec<UserPage>>().await;

    assert_eq!(value.len(), 1);
    assert_eq!(value[0].id, 3);
    assert_eq!(value[0].username, "Cameleon");
    assert_eq!(value[0].full_name, "Cameleon Rodriguez");
    assert_eq!(value[0].disabled, false);
    assert_eq!(value[0].creator_id, 1);
    assert_eq!(value[0].created_at.to_string(), "2026-03-19 10:10:10 UTC");

    let app = ctx.configure();

    let req = Request::get(create_pagination_query(1, 1))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let value = response.into_value::<Vec<UserPage>>().await;

    assert_eq!(value.len(), 1);
    assert_eq!(value[0].id, 2);
    assert_eq!(value[0].username, "Bluebird");
    assert_eq!(value[0].full_name, "Bluebird Rodriguez");
    assert_eq!(value[0].disabled, false);
    assert_eq!(value[0].creator_id, 1);
    assert_eq!(value[0].created_at.to_string(), "2026-03-19 10:10:10 UTC");

    let app = ctx.configure();

    let req = Request::get(create_pagination_query(2, 1))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let value = response.into_value::<Vec<UserPage>>().await;

    assert_eq!(value.len(), 1);
    assert_eq!(value[0].id, 1);
    assert_eq!(value[0].username, "Joaquin");
    assert_eq!(value[0].full_name, "Joaquin Rodriguez");
    assert_eq!(value[0].disabled, false);
    assert_eq!(value[0].creator_id, 1);
    assert_eq!(value[0].created_at.to_string(), "2026-03-19 10:10:10 UTC");

    let app = ctx.configure();

    let req = Request::get(create_pagination_query(3, 1))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let value = response.into_value::<Vec<UserPage>>().await;

    assert_eq!(value.len(), 0);
}

#[tokio::test]
async fn it_reads_empty_pagianted_users() {
    let ctx = TestContext::new().await;
    ctx.setup_db_schema().await;
    let app = ctx.configure();

    let req = Request::get(create_pagination_query(0, 10))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let value = response.into_value::<Vec<UserPage>>().await;

    assert_eq!(value.len(), 0);
}

#[tokio::test]
async fn it_reads_paginated_users_with_idiomatic_json() {
    let ctx = TestContext::new().await;
    ctx.setup_db_schema().await;

    insert_joaquin_user(ctx.db.as_ref()).await.unwrap();
    insert_bluebird_user(ctx.db.as_ref()).await.unwrap();
    insert_chameleon_user(ctx.db.as_ref()).await.unwrap();

    let app = ctx.configure();

    let req = Request::get(create_pagination_query(0, 3))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_value = response.into_value::<serde_json::Value>().await;

    let expected_body = json!([
    {
        "id": 3,
        "username": "Cameleon",
        "fullName": "Cameleon Rodriguez",
        "disabled": false,
        "createdAt": "2026-03-19T10:10:10Z",
        "creatorId": 1
    },
    {
        "id": 2,
        "username": "Bluebird",
        "fullName": "Bluebird Rodriguez",
        "disabled": false,
        "createdAt": "2026-03-19T10:10:10Z",
        "creatorId": 1
    },
    {
        "id": 1,
        "username": "Joaquin",
        "fullName": "Joaquin Rodriguez",
        "disabled": false,
        "createdAt": "2026-03-19T10:10:10Z",
        "creatorId": 1
    }]);

    assert_eq!(expected_body, content_value);
}
