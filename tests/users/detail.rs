use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;

use crate::{setup::TestContext, test_ext::IntoValue, users::migrations::insert_joaquin_user};

fn user_detail_url(user_id: i32) -> String {
    format!("/api/users/{}", user_id)
}

#[tokio::test]
async fn it_read_user_detail() {
    let ctx = TestContext::new().await;
    ctx.setup_db_schema().await;

    insert_joaquin_user(ctx.db.as_ref()).await.unwrap();

    let app = ctx.configure();

    let req = Request::get(user_detail_url(1))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    //let user_detail = response.into_value::<UserPage>().await;

    let content_body = response.into_value::<serde_json::Value>().await;

    let expected_body = json!({
        "id": 1,
        "username": "Joaquin",
        "fullName": "Joaquin Rodriguez",
        "disabled": false,
        "createdAt": "2026-03-19T10:10:10Z",
        "creatorId": 1
    });
    assert_eq!(expected_body, content_body);
}

#[tokio::test]

async fn it_returns_not_found_for_missing_user() {
    let ctx = TestContext::new().await;
    ctx.setup_db_schema().await;

    let app = ctx.configure();

    let req = Request::get(user_detail_url(1))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
