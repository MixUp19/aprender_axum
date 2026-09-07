use chrono::Utc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr};

pub async fn insert_joaquin_user(
    conn: &impl ConnectionTrait,
) -> Result<schemas::user::Model, DbErr> {
    let created_at = Utc::now();

    let model = schemas::user::ActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        username: sea_orm::ActiveValue::Set("Joaquin".into()),
        full_name: sea_orm::ActiveValue::Set("Joaquin Rodriguez".into()),
        password: sea_orm::ActiveValue::Set("contrasena123".into()),
        disabled: sea_orm::ActiveValue::Set(false),
        created_at: sea_orm::ActiveValue::Set(created_at.naive_utc()),
        creator_id: sea_orm::ActiveValue::Set(1),
    }
    .insert(conn)
    .await?;

    Ok(model)
}
