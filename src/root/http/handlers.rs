use axum::response::IntoResponse;

pub async fn root_handler() -> impl IntoResponse {
    println!("processing root handler");
    String::from("perritos peludos")
}

pub async fn post_handler() -> impl IntoResponse {
    println!("processing post handler");
    String::from("perritos peludos")
}