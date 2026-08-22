use std::sync::Arc;

use crate::{error::ApiError, users::{om::UserPage, persistence::uow::UnitOfWorkFactory}};

pub struct ReadUserQuery {
    pub user_id : i32,
}

pub struct ReadUserQueryHandler {
    pub uow_factory: Arc<UnitOfWorkFactory>
}

impl ReadUserQueryHandler {
    pub async fn handle(&self, query: ReadUserQuery) -> Result<UserPage, ApiError> {
        println!("*fetching user with id: {}", query.user_id);

        let uow = self.uow_factory.begin().await?;

        let user_repository = uow.user_repository();

        let model = user_repository.get_user(query.user_id)
        .await?
        .ok_or_else(|| ApiError::NotFound)?;

        uow.commit().await?;

        Ok(model.into())
    }
}
