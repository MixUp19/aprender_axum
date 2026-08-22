use std::sync::Arc;

use crate::{error::ApiError, users::{om::{Pagination, UserPage}, persistence::uow::UnitOfWorkFactory}};

pub struct ReadUsersQuery {
    pub pagination: Pagination
}

pub struct ReadUsersQueryHandler {
    pub uow_factory: Arc<UnitOfWorkFactory>
}

impl ReadUsersQueryHandler {
    pub async fn handle(&self, query: ReadUsersQuery) -> Result<Vec<UserPage>, ApiError> {
        println!(
            "*Fetching users on page: {},  page_size: {}",
            query.pagination.page, query.pagination.page_size
        );

        let uow = self.uow_factory.begin().await?;

        let user_repository = uow.user_repository();

        let model = user_repository.get_users(query.pagination)
        .await?;

        uow.commit().await?;
        
        let page = model.into_iter().map(Into::into).collect();

        Ok(page)
    }
}