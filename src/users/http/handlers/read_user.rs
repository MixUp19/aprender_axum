use std::sync::Arc;

use axum::{Json, extract::{Path, State}};

use crate::{context::AppContext, error::ApiError, users::{application::queries::{ReadUserQuery, ReadUserQueryHandler}, om::UserPage, persistence::uow::UnitOfWorkFactory}};

#[utoipa::path(
        get,
        path = "/api/users/{user_id}",
        tag = "user",
        summary = "it looks for an existing user",
        params(
            ("user_id" = i32, Path, description = "User unique Id")
        ),
        responses(
            (status= OK, body=UserPage, description = "Get all users paginated"),
            (status = NOT_FOUND, description = "The user was not found"),
            (status = INTERNAL_SERVER_ERROR, description = "Something went wrong")
        )
    )]
pub async fn read_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>
) -> Result<Json<UserPage>, ApiError> {
    println!("*fetching user with id: {}", user_id);

    let query = ReadUserQuery {user_id};

    let uow_factory = UnitOfWorkFactory::new(Arc::clone(&ctx.conn));

    let user_detail = ReadUserQueryHandler {
        uow_factory: Arc::new(uow_factory),
    }
    .handle(query)
    .await?;

    Ok(Json(user_detail))
}