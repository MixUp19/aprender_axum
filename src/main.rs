use core::panic;
use std::{env, net::SocketAddr};

use axum::{response::IntoResponse};
use f5a_services::{context::AppContext, users};
use sea_orm::Database;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in env");

    println!("db url: {}",database_url);

    let conn = Database::connect(database_url)
        .await
        .expect("Failed to connect to database");

    let ctx = AppContext {conn};

    let port = 4000;
    let addr = SocketAddr::from(([0,0,0,0], port));

    let listener = tokio::net::TcpListener::bind(addr).await
    .unwrap_or_else(|e|{
        panic!("failed to bind to {}:{}", addr, e);
    });

    let router = axum::Router::new()
        .route("/", axum::routing::get(root_handler).post(post_handler))
        .route("/api/users", 
        axum::routing::get(users::handlers::read_users)
            .post(users::http::handlers::create_user)
        )
        .route(
            "/api/users/{user_id}" , 
            axum::routing::get(users::http::handlers::read_user)
            .put(users::handlers::update_user)
            .delete(users::handlers::delete_user)
            .patch(users::handlers::partial_update_user)
        )
        .with_state(ctx);
    
    println!("Listening on {}", listener.local_addr().unwrap());
    

    axum::serve(listener, router).await.unwrap_or_else(|err|{
        panic!("failed to start server: {}", err);
    });
}

async fn root_handler() -> impl IntoResponse{
    println!("processing root handler");
    String::from("perritos peludos")
}


async fn post_handler() -> impl IntoResponse{
    println!("processing post handler");
    String::from("perritos peludos")
}
