use chrono::{NaiveDate};
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr};

pub async fn insert_joaquin_user(
    conn: &impl ConnectionTrait,
) -> Result<schemas::user::Model, DbErr> {
    let created_at = NaiveDate::from_ymd_opt(2026, 03, 19)
    .and_then(|date| date.and_hms_opt(10, 10, 10))
    .unwrap();

    let model = schemas::user::ActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        username: sea_orm::ActiveValue::Set("Joaquin".into()),
        full_name: sea_orm::ActiveValue::Set("Joaquin Rodriguez".into()),
        password: sea_orm::ActiveValue::Set("contrasena123".into()),
        disabled: sea_orm::ActiveValue::Set(false),
        created_at: sea_orm::ActiveValue::Set(created_at),
        creator_id: sea_orm::ActiveValue::Set(1),
    }
    .insert(conn)
    .await?;

    Ok(model)
}

pub async fn insert_bluebird_user(
    conn: &impl ConnectionTrait,
) -> Result<schemas::user::Model, DbErr> {
    let created_at = NaiveDate::from_ymd_opt(2026, 03, 19)
    .and_then(|date| date.and_hms_opt(10, 10, 10))
    .unwrap();

    let model = schemas::user::ActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        username: sea_orm::ActiveValue::Set("Bluebird".into()),
        full_name: sea_orm::ActiveValue::Set("Bluebird Rodriguez".into()),
        password: sea_orm::ActiveValue::Set("contrasena123".into()),
        disabled: sea_orm::ActiveValue::Set(false),
        created_at: sea_orm::ActiveValue::Set(created_at),
        creator_id: sea_orm::ActiveValue::Set(1),
    }
    .insert(conn)
    .await?;

    Ok(model)
}

pub async fn insert_chameleon_user(
    conn: &impl ConnectionTrait,
) -> Result<schemas::user::Model, DbErr> {
    let created_at = NaiveDate::from_ymd_opt(2026, 03, 19)
    .and_then(|date| date.and_hms_opt(10, 10, 10))
    .unwrap();

    let model = schemas::user::ActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        username: sea_orm::ActiveValue::Set("Cameleon".into()),
        full_name: sea_orm::ActiveValue::Set("Cameleon Rodriguez".into()),
        password: sea_orm::ActiveValue::Set("contrasena123".into()),
        disabled: sea_orm::ActiveValue::Set(false),
        created_at: sea_orm::ActiveValue::Set(created_at),
        creator_id: sea_orm::ActiveValue::Set(1),
    }
    .insert(conn)
    .await?;

    Ok(model)
}

