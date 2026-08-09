use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{context::AppContext, error::ApiError, users::{application::commands::{CreateUserCommand, CreateUserCommandHandler}, om::{CreateUserParams, CreatedUser}}};

pub async fn create_user(
    State(ctx): State<AppContext>,
    Json(payload): Json<CreateUserParams>,
) -> Result<Json<CreatedUser>, ApiError> {
    println!("*creating a new user with username: {}", payload.username);

    let command = CreateUserCommand {
        full_name: payload.full_name,
        email: payload.email,
        username: payload.username,
        website: payload.website,
        age: payload.age,
        password: payload.password,
        confirm_password: payload.confirm_password,
        creator_id: 1
    };

    let user_id = CreateUserCommandHandler {
        conn: Arc::new(ctx.conn)
    }.handle(command)
    .await?;

    Ok(Json(CreatedUser { id: user_id }))
}
