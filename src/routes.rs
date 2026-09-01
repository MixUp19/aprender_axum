use crate::context::AppContext;
use crate::root::http::handlers::{post_handler, root_handler};
use crate::users;

pub fn router() -> axum::Router<AppContext> {
    axum::Router::new()
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
        )
}
