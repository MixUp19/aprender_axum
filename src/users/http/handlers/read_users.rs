use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};

use crate::{
    context::AppContext,
    error::ApiError,
    users::{
        application::queries::{ReadUsersQuery, ReadUsersQueryHandler},
        om::{Pagination, UserPage},
        persistence::uow::UnitOfWorkFactory,
    },
};

#[utoipa::path(
        get,
        path = "/api/users",
        tag = "user",
        summary = "it returns every user into range in page ordered descending",
        params(
            Pagination
        ),
        responses(
            (status= OK, body=Vec<UserPage>, description = "Get all users paginated"),
            (status = INTERNAL_SERVER_ERROR, description = "Something went wrong")
        )
    )]
pub async fn read_users(
    State(ctx): State<AppContext>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<UserPage>>, ApiError> {
    let query = ReadUsersQuery { pagination };

    let uow_factory = UnitOfWorkFactory::new(Arc::clone(&ctx.conn));

    let read_users_query_handler = ReadUsersQueryHandler {
        uow_factory: Arc::new(uow_factory),
    };

    let users = read_users_query_handler.handle(query).await?;

    Ok(Json(users))
}
