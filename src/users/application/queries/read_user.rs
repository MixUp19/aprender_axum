use std::sync::Arc;

use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{error::ApiError, users::om::UserPage};

pub struct ReadUserQuery {
    pub user_id : i32,
}

pub struct ReadUserQueryHandler {
    pub conn: Arc<DatabaseConnection>
}

impl ReadUserQueryHandler {
    pub async fn handle(&self, query: ReadUserQuery) -> Result<UserPage, ApiError> {
        println!("*fetching user with id: {}", query.user_id);

        let model = schemas::user::Entity::find_by_id(query.user_id)
            .one(self.conn.as_ref())
            .await?
            .ok_or_else(|| ApiError::NotFound)?;

        Ok(model.into())
    }
}
