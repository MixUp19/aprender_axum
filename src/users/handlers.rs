use axum::{Json, extract::{Path, Query}, response::IntoResponse};

use crate::users::om::{CreateUserParams, CreatedUser, Pagination};



pub async fn read_user(Path(user_id):Path<u32>) -> impl IntoResponse{
    format!("*fetching user with id: {}", user_id)
}

pub async fn read_users (
    Query(pagination): Query<Pagination>
) -> impl IntoResponse {
    format!("*Fetching users on page: {},  page_size: {}",
        pagination.page, pagination.page_size
    )
}

pub async fn create_user(
    Json(payload): Json<CreateUserParams>
) -> impl IntoResponse
{ 
    println!("*creating a new user with username: {}", payload.username);

    Json(CreatedUser {id:99})
}