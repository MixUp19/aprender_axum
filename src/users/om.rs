use chrono::{DateTime, Utc};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
pub struct Pagination {
    pub page: u64,
    pub page_size: u64,
}

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserParams {
    pub full_name: String,
    pub email: String,
    pub username: String,
    pub website: String,
    pub age: u8,
    #[schema(value_type = String, format = "password", example = "MiPassw0rdSegura!")]
    pub password: SecretString,
    #[schema(value_type = String, format = "password", example = "MiPassw0rdSegura!")]
    pub confirm_password: SecretString,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreatedUser {
    pub id: i32,
}

#[derive(Serialize, Deserialize, ToSchema)]
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

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserParams {
    pub username: String,
    pub full_name: String,
    pub disabled: bool,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PartialUserParams {
    /// unique alphanumeric identifier for the use
    #[schema(example = "MixUp19")]
    pub username: Option<String>,
    ///user legal name
    #[schema(example = "Joaquin")]
    pub full_name: Option<String>,
    /// flag to enable or disable the user account
    /// if true, the user will bw prevented form logging in
    pub disabled: Option<bool>,
}
