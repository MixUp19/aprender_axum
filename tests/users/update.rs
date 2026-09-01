use axum::{body::Body, http::{self, Request, StatusCode}};
use serde_json::json;
use tower::ServiceExt;
use crate::{setup::TestContext, test_ext::IntoValue};

fn update_user_url(user_id: i32) ->String{
    format!("/api/users/{}",user_id)
}

#[tokio::test]
async fn it_not_accept_empty_user_request(){
    let ctx = TestContext::new().await;
    let app = ctx.configure();

    let req = Request::put(update_user_url(1))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn it_validate_required_user_request_to_update(){
    let ctx = TestContext::new().await;
    let app = ctx.configure();
    let update_user_params = json!({
        "username": "",
        "fullName": "",
        "disabled": false
    });

    let req = Request::put(update_user_url(1))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(update_user_params.to_string()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let body_content = response.into_value::<serde_json::Value>().await;

    let expected_body = json!({
        "detail": "validation failed",
        "errors":[
            {
              "field": "full_name",
              "reason": "name must be between 1 and 200 characters",
              "code": "length"
            },
            {
              "field": "username",
              "reason": "username must be between 3 and 100 characters",
              "code": "length"
            }
        ]
    });

    assert_eq!(body_content, expected_body)
}

#[tokio::test]
async fn it_not_accept_unknown_user_id(){
    let ctx = TestContext::new().await;
    ctx.setup_db_schema().await;
    let app = ctx.configure();
    let update_user_params = json!({
        "username": "Joaquin",
        "fullName": "Joaquin Rodriguez",
        "disabled": false
    });

    let req = Request::put(update_user_url(1))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(update_user_params.to_string()))
        .unwrap();
    
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND)
}

async fn it_accepts_and_update_user() {}