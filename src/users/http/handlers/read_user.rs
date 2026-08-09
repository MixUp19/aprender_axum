use std::sync::Arc;

use axum::{Json, extract::{Path, State}};

use crate::{context::AppContext, error::ApiError, users::{application::queries::{ReadUserQuery, ReadUserQueryHandler}, om::UserPage}};

pub async fn read_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>
) -> Result<Json<UserPage>, ApiError> {
    println!("*fetching user with id: {}", user_id);

    let query = ReadUserQuery {user_id};

    let user_detail = ReadUserQueryHandler {
        conn: Arc::new(ctx.conn),
    }
    .handle(query)
    .await?;

    Ok(Json(user_detail))
}