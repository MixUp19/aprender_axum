use axum::{
    Json, extract::{Path, Query, State}, response::NoContent,
};
use schemas::user::{self, Model};
use sea_orm::{ActiveModelTrait, ActiveValue, DbConn, EntityTrait, IntoActiveModel, ModelTrait, PaginatorTrait, sea_query::value::prelude::chrono};
use validator::Validate;

use crate::{
    context::AppContext, error::ApiError, users::om::{CreateUserParams, CreatedUser, Pagination, PartialUserParams, UpdateUserParams, UserPage},
};

pub async fn read_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>
) -> Result<Json<UserPage>, ApiError> {
    println!("*fetching user with id: {}", user_id);

    let model  = get_user(&ctx.conn, user_id).await?;

    Ok(Json(model.into()))
}

pub async fn read_users(
    State(ctx): State<AppContext>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<UserPage>>, ApiError> {

    let users = schemas::user::Entity::find()
        .order_by_id_desc()
        .paginate(&ctx.conn, pagination.page_size)
        .fetch_page(pagination.page)
        .await?;

    println!(
        "*Fetching users on page: {},  page_size: {}",
        pagination.page, pagination.page_size
    );

    let user_page = users.into_iter()
        .map(Into::into).collect();

    Ok(Json(user_page))
}

pub async fn create_user(
    State(ctx): State<AppContext>,
    Json(payload): Json<CreateUserParams>,
) -> Result<Json<CreatedUser>, ApiError> {
    println!("*creating a new user with username: {}", payload.username);

    payload.validate()?;

    let model = schemas::user::ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::Set(payload.username),
        full_name: ActiveValue::Set(payload.full_name),
        password: ActiveValue::Set("1234".into()),
        disabled: ActiveValue::Set(true),
        created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        creator_id: ActiveValue::Set(1),
    }
    .insert(&ctx.conn)
    .await?;

    Ok(Json(CreatedUser { id: model.id }))
}

pub async fn update_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
    Json(payload): Json<UpdateUserParams>,
) -> Result<NoContent, ApiError>{
    println!("Updating user with id: {}", user_id);

    let model = get_user(&ctx.conn, user_id).await?;

    let mut user_modifiable = model.into_active_model();

    user_modifiable.username = ActiveValue::Set(payload.username);
    user_modifiable.full_name = ActiveValue::Set(payload.full_name);
    user_modifiable.disabled = ActiveValue::Set(payload.disabled);

    user_modifiable.update(&ctx.conn).await?;

    Ok(NoContent)
}

pub async fn delete_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
) -> Result<NoContent, ApiError> {
    println!("deleting user, {}", user_id);

    let model = get_user(&ctx.conn, user_id).await?;

    model.delete(&ctx.conn).await?;

    Ok(NoContent)
}

pub async fn partial_update_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
    Json(payload): Json<PartialUserParams>,
) -> Result<NoContent, ApiError>{
    println!("Partial update user with id: {}", user_id);

    let model = get_user(&ctx.conn, user_id).await?;

    let mut user_modifiable = model.into_active_model();

    if let Some(username) = payload.username{
        user_modifiable.username = ActiveValue::Set(username);
    }

    if let Some(full_name) = payload.full_name{
        user_modifiable.full_name = ActiveValue::Set(full_name);    
    }

    if let Some(disabled) = payload.disabled{
        user_modifiable.disabled = ActiveValue::Set(disabled);
    }
    
    user_modifiable.update(&ctx.conn).await?;

    Ok(NoContent)
}


async fn get_user(conn: &DbConn, user_id: i32) -> Result<Model, ApiError> {
    schemas::user::Entity::find_by_id(user_id)
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound)
}