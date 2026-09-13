use crate::context::AppContext;
use crate::users;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(description = "F5a services (ES)"), 
    tags(
        (name = "user", description = "User API endpoints")
    )
)]
pub struct ApiDoc;

pub fn router() -> axum::Router<AppContext> {
    let(router, open_api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
    .routes(routes!(
        crate::root::http::handlers::root_handler,
        crate::root::http::handlers::post_handler
    ))
    .routes(
        routes!(
            users::http::handlers::read_user,
            users::http::handlers::update_user,
            users::http::handlers::delete_user,
            users::http::handlers::partial_update_user
        )
    )
    .routes(
        routes!(
            users::http::handlers::read_users,
            users::http::handlers::create_user,
        )
    )
    .split_for_parts();

    router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", open_api))
    /*axum::Router::new()
        .route("/", axum::routing::get(root_handler).post(post_handler))
        .route(
            "/api/users",
            axum::routing::get(users::http::handlers::read_users)
                .post(users::http::handlers::create_user),
        )
        .route(
            "/api/users/{user_id}",
            axum::routing::get(users::http::handlers::read_user)
                .put(users::http::handlers::update_user)
                .delete(users::http::handlers::delete_user)
                .patch(users::http::handlers::partial_update_user),
        )*/
}
