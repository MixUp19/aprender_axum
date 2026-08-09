use sea_orm::sqlx::types::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Pagination {
    pub page: u64,
    pub page_size: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserParams {
    pub full_name: String,
    pub email: String,
    pub username: String,
    pub website: String,
    pub age: u8,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Serialize)]
pub struct CreatedUser {
    pub id: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPage {
    pub id: i32,
    pub username: String,
    pub full_name: String,
    pub disabled: bool,
    pub created_at: DateTime<Utc>,
    pub creator_id: i32,
}

impl From<schemas::user::Model> for UserPage {
    fn from(value: schemas::user::Model) -> Self {
        Self {
            id: value.id,
            username: value.username,
            full_name: value.full_name,
            disabled: value.disabled,
            created_at: value.created_at.and_utc(),
            creator_id: value.creator_id,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserParams {
    pub username: String,
    pub full_name: String,
    pub disabled: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialUserParams {
    pub username: Option<String>,
    pub full_name: Option<String>,
    pub disabled: Option<bool>,
}
