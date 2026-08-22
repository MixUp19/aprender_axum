use sea_orm::{DatabaseTransaction, DbErr};

use crate::users::persistence::repository::SeaOrmUserRepository;

pub struct UnitOfWork {
    pub tx: DatabaseTransaction,
}

impl UnitOfWork {
    pub async fn commit (self) -> Result<(), DbErr> {
        self.tx.commit().await
    }

    pub async fn rollback (self) -> Result<(), DbErr>{
        self.tx.rollback().await
    }

    pub fn user_repository(&self) -> SeaOrmUserRepository <'_, DatabaseTransaction> {
        SeaOrmUserRepository::new(&self.tx)
    }
}