use std::sync::Arc;

use sea_orm::{DatabaseConnection, DbErr, TransactionTrait};

use crate::users::persistence::uow::UnitOfWork;

pub struct UnitOfWorkFactory {
    conn: Arc<DatabaseConnection>
}

impl UnitOfWorkFactory {
    pub fn new (conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    pub async fn begin(&self) -> Result<UnitOfWork, DbErr> {
        let tx = self.conn.begin().await?;
        Ok(UnitOfWork { tx })
    }
}