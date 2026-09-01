use crate::{setup::{TestContext}, test_ext::IntoValue};
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use f5a_services::users::om::CreatedUser;
use sea_orm::EntityTrait;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn it_not_accept_empty_user_request() {
    let ctx = TestContext::new().await;
    let app = ctx.configure();

    let req = Request::post("/api/users")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn it_validate_required_user_request() {
    let ctx = TestContext::new().await;
    let app = ctx.configure();
    let create_user_params = json!({
        "username": "",
        "email": "",
        "fullName": "",
        "website": "",
        "age": 0,
        "password": "",
        "confirmPassword": ""
    });

    let req = Request::post("/api/users")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(create_user_params.to_string()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let body_content = response.into_value::<serde_json::Value>().await;

    let expected_body = json!({
        "detail": "validation failed",
        "errors":[
            {
              "field": "age",
              "reason": "age must be between 18 and 100",
              "code": "range"
            },
            {
              "field": "email",
              "reason": "email address is not valid",
              "code": "email"
            },
            {
              "field": "full_name",
              "reason": "name must be between 1 and 200 characters",
              "code": "length"
            },
            {
              "field": "password",
              "reason": "password must be at least 12 characters long",
              "code": "password_strength"
            },
            {
              "field": "username",
              "reason": "username must be between 3 and 100 characters",
              "code": "length"
            },
            {
              "field": "website",
              "reason": "website url is not valid",
              "code": "url"
            }
        ]
    });

    assert_eq!(body_content, expected_body)
}


#[tokio::test]
async fn it_accepts_and_save_valid_user(){
    let ctx = TestContext::new().await;
    ctx.setup_db_schema().await;
    let app = ctx.configure();
    let create_user_params = json!({
        "username": "Joaquin",
        "email": "joaquin.r@gmail.com",
        "fullName": "Joaquin",
        "website": "https://joaquin.com",
        "age": 19,
        "password": "contrasena123",
        "confirmPassword": "contrasena123"
    });

    let req = Request::post("/api/users")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(create_user_params.to_string()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let user_created = response.into_value::<CreatedUser>().await;

    assert_eq!(user_created.id, 1);

    let user_model = schemas::user::Entity::find_by_id(user_created.id)
      .one(ctx.db.as_ref())
      .await
      .unwrap();

    assert!(user_model.is_some());
    
    let user_model = user_model.unwrap();

    assert_eq!(user_model.id, 1);
    assert_eq!(user_model.full_name, "Joaquin");
    assert_eq!(user_model.username, "Joaquin");
    assert_eq!(user_model.creator_id, 1);
    assert_eq!(user_model.password, "contrasena123");
    assert_eq!(user_model.disabled, false);
}