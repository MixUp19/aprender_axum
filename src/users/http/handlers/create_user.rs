use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use secrecy::{ExposeSecret};

use crate::{
    context::AppContext,
    error::ApiError,
    users::{
        application::commands::{CreateUserCommand, CreateUserCommandHandler},
        om::{CreateUserParams, CreatedUser},
        persistence::uow::UnitOfWorkFactory,
    },
};

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
