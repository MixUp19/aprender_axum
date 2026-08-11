use std::sync::Arc;

use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, IntoActiveModel};

use crate::users::persistence::repository::SaveNewUser;

pub struct SeaOrmUserRepository {
    conn: Arc<DatabaseConnection>,
}

impl SeaOrmUserRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    pub async fn insert(
        &self,
        change: SaveNewUser,
    ) -> Result<schemas::user::Model, DbErr> {
        
        let model =change.into_active_model().insert(self.conn.as_ref()).await?;

        Ok(model)
    }
}
