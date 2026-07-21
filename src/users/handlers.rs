use axum::{extract::Path, response::IntoResponse};



pub async fn read_user(Path(user_id):Path<u32>) -> impl IntoResponse{
    format!("*fetching user with id: {}, project_id: {}", user_id,project_id)
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
){ 
    format!("*creating a new user with username: {}", payload.username)
}