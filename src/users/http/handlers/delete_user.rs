use std::sync::Arc;

use axum::{extract::{Path, State}, response::NoContent};

use crate::{context::AppContext, error::ApiError, users::{application::commands::{DeleteUserCommand, DeleteUserCommandHandler}, persistence::uow::UnitOfWorkFactory}};


#[utoipa::path(
        delete,
        path = "/api/users/{user_id}",
        tag = "user",
        summary = "it deletes a user",
        params(
            ("user_id" = i32, Path, description = "User unique Id")
        ),
        responses(
            (status= NO_CONTENT, description = "User was deleted"),
            (status = NOT_FOUND, description = "User was not found to update"),
            (status = INTERNAL_SERVER_ERROR, description = "Something went wrong")
        )
    )]
pub async fn delete_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
) -> Result<NoContent, ApiError> {

    let command = DeleteUserCommand {id: user_id};

    let uow_factory = UnitOfWorkFactory::new(Arc::clone(&ctx.conn));

    let command_handler = DeleteUserCommandHandler {uow_factory};

    command_handler.handle(command).await?;

    Ok(NoContent)
}