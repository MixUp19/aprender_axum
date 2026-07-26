use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Pagination {
    pub page: u64,
    pub page_size: u64
}

#[derive(Deserialize)]
pub struct CreateUserParams {
    pub name: String,
    pub username: String,
}

#[derive(Serialize)]
pub struct CreatedUser {
    pub id: u32
}