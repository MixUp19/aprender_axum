use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use secrecy::{ExposeSecret};

use crate::{
    context::AppContext, error::ApiError, shared::response::ProblemDetails, users::{
        application::commands::{CreateUserCommand, CreateUserCommandHandler},
        om::{CreateUserParams, CreatedUser},
        persistence::uow::UnitOfWorkFactory,
    },
};

#[utoipa::path(
        post,
        path = "/api/users",
        tag = "user",
        summary = "it creates a new  user",
        request_body = CreateUserParams,
        responses(
            (status= OK, body=CreatedUser, description = "User was created"),
            (status = UNPROCESSABLE_ENTITY, body = ProblemDetails, description = "The object was not correct"),
            (status = INTERNAL_SERVER_ERROR, description = "Something went wrong")
        )
    )]
#[tracing::instrument(skip(ctx), err)]
pub async fn create_user(
    State(ctx): State<AppContext>,
    Json(payload): Json<CreateUserParams>,
) -> Result<(StatusCode, Json<CreatedUser>), ApiError> {
    tracing::info!(
        username = payload.username,
        "*creating a new user with username"
    );

    let command = CreateUserCommand {
        full_name: payload.full_name,
        email: payload.email,
        username: payload.username,
        website: payload.website,
        age: payload.age,
        password: payload.password.expose_secret().to_string(),
        confirm_password: payload.confirm_password.expose_secret().to_string(),
        creator_id: 1,
    };

    let user_id = CreateUserCommandHandler {
        uow_factory: UnitOfWorkFactory::new(Arc::clone(&ctx.conn)),
    }
    .handle(command)
    .await?;

    Ok((StatusCode::CREATED, Json(CreatedUser { id: user_id })))
}
