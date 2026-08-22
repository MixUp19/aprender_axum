use std::sync::Arc;

use axum::{extract::{Path, State}, response::NoContent};

use crate::{context::AppContext, error::ApiError, users::{application::commands::{DeleteUserCommand, DeleteUserCommandHandler}, persistence::uow::UnitOfWorkFactory}};

pub async fn delete_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
) -> Result<NoContent, ApiError> {

    let command = DeleteUserCommand {id: user_id};

    let uow_factory = UnitOfWorkFactory::new(Arc::new(ctx.conn));

    let command_handler = DeleteUserCommandHandler {uow_factory};

    command_handler.handle(command).await?;

    Ok(NoContent)
}