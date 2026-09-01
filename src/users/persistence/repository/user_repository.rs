use sea_orm::{
    ActiveModelTrait, ActiveValue, ConnectionTrait, DbErr, EntityTrait, IntoActiveModel,
    ModelTrait, PaginatorTrait,
};

use crate::users::{
    om::Pagination, persistence::repository::{PartialUpdateUser, SaveNewUser, UpdateUser},
};

pub struct SeaOrmUserRepository<'a, C: ConnectionTrait> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> SeaOrmUserRepository<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        Self { conn }
    }

    pub async fn insert(&self, change: SaveNewUser) -> Result<schemas::user::Model, DbErr> {
        let model = change.into_active_model().insert(self.conn).await?;

        Ok(model)
    }

    pub async fn get_user(&self, id: i32) -> Result<Option<schemas::user::Model>, DbErr> {
        let model = schemas::user::Entity::find_by_id(id).one(self.conn).await?;

        Ok(model)
    }

    pub async fn get_users(
        &self,
        pagination: Pagination,
    ) -> Result<Vec<schemas::user::Model>, DbErr> {
        let model = schemas::user::Entity::find()
            .order_by_id_desc()
            .paginate(self.conn, pagination.page_size)
            .fetch_page(pagination.page)
            .await?;

        Ok(model)
    }

    pub async fn update_user(&self, change: UpdateUser) -> Result<(), DbErr> {
        let model = self
            .get_user(change.id)
            .await?
            .ok_or(DbErr::RecordNotFound("".to_string()))?;
        let mut active_model = model.into_active_model();

        active_model.full_name = ActiveValue::Set(change.full_name);
        active_model.username = ActiveValue::set(change.username);
        active_model.disabled = ActiveValue::Set(false);

        active_model.update(self.conn).await?;

        Ok(())
    }

    pub async fn delete_user(&self, id: i32) -> Result<(), DbErr> {
        let model = self.get_user(id).await?.ok_or(DbErr::RecordNotFound("".to_string()))?;

        model.delete(self.conn).await?;

        Ok(())
    }

    pub async fn partial_update_user(&self, change: PartialUpdateUser) -> Result<(), DbErr> {
        let model = self
            .get_user(change.id)
            .await?
            .ok_or(DbErr::RecordNotFound("".to_string()))?;

        let mut active_model = model.into_active_model();

        if let Some(username) = change.username {
            active_model.username = ActiveValue::Set(username);
        }

        if let Some(full_name) = change.full_name {
            active_model.full_name = ActiveValue::Set(full_name);
        }

        if let Some(disabled) = change.disabled {
            active_model.disabled = ActiveValue::Set(disabled);
        }

        active_model.update(self.conn).await?;

        Ok(())
    }
}
