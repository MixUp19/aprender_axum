use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260726_053354_create_user_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create().table(User::Table).if_not_exists()
            .col(pk_auto(User::Id))
            .col(string_len_uniq(User::Username, 100))
            .col(string_len(User::FullName, 200))
            .col(string(User::Password))
            .col(boolean(User::Disabled))
            .col(date_time(User::CreatedAt))
            .col(integer(User::CreatorId))
            .to_owned()
        ).await?;
        
        manager.create_index(
            Index::create()
            .name("idx_user_creator_id")
            .table(User::Table)
            .col(User::CreatorId)
            .to_owned()
        )
        .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        _manager
        .drop_table(Table::drop().table(User::Table).to_owned())
        .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    FullName,
    Password,
    Disabled,
    CreatedAt,
    CreatorId,
}


