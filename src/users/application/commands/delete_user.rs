use crate::{error::ApiError, users::persistence::uow::UnitOfWorkFactory};

pub struct DeleteUserCommand {
    pub id: i32,
}

pub struct DeleteUserCommandHandler {
    pub uow_factory: UnitOfWorkFactory
}

impl DeleteUserCommandHandler {
    pub async fn handle(&self, command: DeleteUserCommand) -> Result<(), ApiError>{
        println!("deleting the user with id: {}", command.id);
        
        let uow = self.uow_factory.begin().await?;

        let user_repository = uow.user_repository();
        
        user_repository.delete_user(command.id).await?;
        
        uow.commit().await?;

        Ok(())
    }
}