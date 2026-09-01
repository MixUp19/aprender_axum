use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::NoContent,
};

use crate::{
    context::AppContext,
    error::ApiError,
    users::{
        application::commands::{UpdateUserCommand, UpdateUserCommandHandler},
        om::UpdateUserParams,
        persistence::uow::UnitOfWorkFactory,
    },
};

pub async fn update_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
    Json(payload): Json<UpdateUserParams>,
) -> Result<NoContent, ApiError> {
    let command = UpdateUserCommand {
        id: user_id,
        username: payload.username,
        full_name: payload.full_name,
    };

    let uow_factory = UnitOfWorkFactory::new(Arc::clone(&ctx.conn));

    let handle = UpdateUserCommandHandler { uow_factory };
    handle.handle(command).await?;

    Ok(NoContent)
}
