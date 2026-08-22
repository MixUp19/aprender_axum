use axum::response::NoContent;
use validator::Validate;

use crate::{error::ApiError, users::persistence::{repository::{PartialUpdateUser}, uow::UnitOfWorkFactory}};

#[derive(Validate)]
pub struct PartialUpdateUserCommand {
    pub id: i32,

    #[validate(length(
        min = 1,
        max = 200,
        message = "name must be between 1 and 200 characters"
    ))]
    pub full_name: Option<String>,

    #[validate(length(
        min = 3,
        max = 100,
        message = "username must be between 3 and 100 characters"
    ))]
    pub username: Option<String>,
}

pub struct PartialUpdateUserCommandHandler {
    pub uow_factory: UnitOfWorkFactory,
}

impl PartialUpdateUserCommandHandler {
    pub async fn handle(&self, command: PartialUpdateUserCommand) -> Result<NoContent, ApiError> {
        println!("Partial update user with id: {}", command.id);
        command.validate()?;

        let uow = self.uow_factory.begin().await?;

        let user_repo = uow.user_repository();

        let change = PartialUpdateUser {
            id: command.id,
            username: command.username,
            full_name: command.full_name,
            disabled: Some(false)
        };
        
        user_repo.partial_update_user(change).await?;
        

        uow.commit().await?;

        Ok(NoContent)
    }
}