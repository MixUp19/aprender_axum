use std::sync::Arc;

use axum::{Json, extract::{Path, State}};

use crate::{context::AppContext, error::ApiError, users::{application::queries::{ReadUserQuery, ReadUserQueryHandler}, om::UserPage, persistence::uow::UnitOfWorkFactory}};

pub async fn read_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>
) -> Result<Json<UserPage>, ApiError> {
    println!("*fetching user with id: {}", user_id);

    let query = ReadUserQuery {user_id};

    let uow_factory = UnitOfWorkFactory::new(Arc::new(ctx.conn));

    let user_detail = ReadUserQueryHandler {
        uow_factory: Arc::new(uow_factory),
    }
    .handle(query)
    .await?;

    Ok(Json(user_detail))
}