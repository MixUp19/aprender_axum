use chrono;
use validator::Validate;

use crate::{error::ApiError, users::persistence::repository::{SaveNewUser, SeaOrmUserRepository}};

#[derive(Validate)]
pub struct CreateUserCommand {
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

    #[validate(range(min = 18, max = 100, message = "age must be between 18 and 100"))]
    pub age: u8,

    #[validate(custom(function = "crate::validators::password_strength"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "password do not match"))]
    pub confirm_password: String,

    pub creator_id: i32
}

pub struct CreateUserCommandHandler {
    pub user_repo: SeaOrmUserRepository,
    //pub conn: Arc<DatabaseConnection>,
}

impl CreateUserCommandHandler {
    pub async fn handle(&self, command: CreateUserCommand) -> Result<i32, ApiError> {
        command.validate()?;

        let created_at = chrono::Utc::now();

        let change = SaveNewUser {
            username: command.username,
            full_name: command.full_name,
            password: command.password,
            disabled: false,
            created_at: created_at.naive_local(),
            creator_id: command.creator_id
        };
        
        let model =self.user_repo.insert(change).await?;
        

        Ok(model.id)
    }
}
