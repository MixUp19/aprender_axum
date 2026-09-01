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
        application::commands::{PartialUpdateUserCommand, PartialUpdateUserCommandHandler},
        om::{PartialUserParams},
        persistence::uow::UnitOfWorkFactory,
    },
};

pub async fn partial_update_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
    Json(payload): Json<PartialUserParams>,
) -> Result<NoContent, ApiError> {
    let uow_factory = UnitOfWorkFactory::new(Arc::clone(&ctx.conn));

    let command = PartialUpdateUserCommand {
        id: user_id,
        username: payload.username,
        full_name: payload.full_name,
    };

    let command_handler = PartialUpdateUserCommandHandler { uow_factory };

    command_handler.handle(command).await?;

    Ok(NoContent)
}
