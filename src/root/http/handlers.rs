use axum::response::IntoResponse;

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status=200, body=String)
    )
)]
pub async fn root_handler() -> impl IntoResponse {
    println!("processing root handler");
    String::from("perritos peludos")
}

#[utoipa::path(
    post,
    path = "/",
    responses(
        (status=200, body=String)
    )
)]
pub async fn post_handler() -> impl IntoResponse {
    println!("processing post handler");
    String::from("perritos peludos")
}
