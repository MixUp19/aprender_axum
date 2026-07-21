use axum::{extract::Path, response::IntoResponse};



pub async fn read_user(Path((user_id, project_id)):Path<(u32,u32)>) -> impl IntoResponse{
    format!("*fetching user with id: {}, project_id: {}", user_id,project_id)
}