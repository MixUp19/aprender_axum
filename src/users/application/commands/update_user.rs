use axum::response::NoContent;
use validator::Validate;

use crate::{error::ApiError, users::persistence::{repository::{UpdateUser}, uow::UnitOfWorkFactory}};

#[derive(Validate)]
pub struct UpdateUserCommand {
    pub id: i32,

    #[validate(length(
        min = 1,
        max = 200,
        message = "name must be between 1 and 200 characters"
    ))]
    pub full_name: String,

    #[validate(length(
        min = 3,
        max = 100,
        message = "username must be between 3 and 100 characters"
    ))]
    pub username: String,
}

pub struct UpdateUserCommandHandler {
    pub uow_factory: UnitOfWorkFactory,
}

impl UpdateUserCommandHandler {
    pub async fn handle(&self, command: UpdateUserCommand) -> Result<NoContent, ApiError> {
        println!("Updating user with id: {}", command.id);
        command.validate()?;

        let uow = self.uow_factory.begin().await?;

        let user_repo = uow.user_repository();

        let change = UpdateUser {
            id: command.id,
            username: command.username,
            full_name: command.full_name,
            disabled: false
        };
        
        user_repo.update_user(change).await?;
        

        uow.commit().await?;

        Ok(NoContent)
    }
}
