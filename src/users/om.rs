use sea_orm::sqlx::types::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize)]
pub struct Pagination {
    pub page: u64,
    pub page_size: u64,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserParams {
    #[validate(length(
        min = 1,
        max = 200,
        message = "name must be between 1 and 200 characters"
    ))]
    pub full_name: String,

    #[validate(email(message = "email address is not valid"))]
    pub email: String,

    #[validate(length(
        min = 3,
        max = 100,
        message = "username must be between 3 and 100 characters"
    ))]
    pub username: String,

    #[validate(url(message = "website url is not valid"))]
    pub website: String,

    #[validate(range(min=18, max = 100, message = "age must be between 18 and 100"))]
    pub age: u8,

    #[validate(custom(function = "crate::validators::password_strength"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "password do not match"))]
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
