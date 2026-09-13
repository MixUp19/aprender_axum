use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::NoContent,
};

use crate::{
    shared::response::ProblemDetails,
    context::AppContext,
    error::ApiError,
    users::{
        application::commands::{UpdateUserCommand, UpdateUserCommandHandler},
        om::UpdateUserParams,
        persistence::uow::UnitOfWorkFactory,
    },
};

#[utoipa::path(
        put,
        path = "/api/user/{user_id}",
        tag = "user",
        request_body = UpdateUserParams,
        summary = "it updates all updatable params for a user",
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
pub async fn update_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
    Json(payload): Json<UpdateUserParams>,
) -> Result<NoContent, ApiError> {
    let command = UpdateUserCommand {
        id: user_id,
        username: payload.username,
        full_name: payload.full_name,
        disabled: payload.disabled
    };

    let uow_factory = UnitOfWorkFactory::new(Arc::clone(&ctx.conn));

    let handle = UpdateUserCommandHandler { uow_factory };
    handle.handle(command).await?;

    Ok(NoContent)
}
