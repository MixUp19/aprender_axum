use sea_orm::{ActiveValue, IntoActiveModel, entity::prelude::DateTime};

pub struct SaveNewUser {
    pub username: String,
    pub full_name: String,
    pub password: String,
    pub disabled: bool,
    pub created_at: DateTime,
    pub creator_id: i32,
}

impl IntoActiveModel<schemas::user::ActiveModel> for SaveNewUser {
    fn into_active_model(self) -> schemas::user::ActiveModel {
        schemas::user::ActiveModel {
            id: ActiveValue::NotSet,
            username: ActiveValue::Set(self.username),
            full_name: ActiveValue::Set(self.full_name),
            password: ActiveValue::Set(self.password),
            disabled: ActiveValue::set(self.disabled),
            created_at: ActiveValue::set(self.created_at),
            creator_id: ActiveValue::set(self.creator_id)
        }
    }
}