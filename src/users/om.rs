use serde::Deserialize;

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