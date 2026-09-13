use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::NoContent,
};

use crate::{
    context::AppContext,
    error::ApiError,
    shared::response::ProblemDetails,
    users::{
        application::commands::{PartialUpdateUserCommand, PartialUpdateUserCommandHandler},
        om::PartialUserParams,
        persistence::uow::UnitOfWorkFactory,
    },
};

#[utoipa::path(
        patch,
        path = "/api/user/{user_id}",
        tag = "user",
        request_body = PartialUserParams,
        summary = "it partially update a user",
        params(
            ("user_id" = i32, Path, description = "User unique Id")
        ),
        responses(
            (status = NO_CONTENT, description = "User was updated"),
            (status = NOT_FOUND, description = "User was not found to update"),
            (status = UNPROCESSABLE_ENTITY, body = ProblemDetails, description = "The object was not correct"),
            (status = INTERNAL_SERVER_ERROR, description = "Something went wrong")
        )
    )]
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
        disable: payload.disabled,
    };

    let command_handler = PartialUpdateUserCommandHandler { uow_factory };

    command_handler.handle(command).await?;

    Ok(NoContent)
}
