use core::panic;
use std::net::SocketAddr;

use axum::{response::IntoResponse, routing};

#[tokio::main]
async fn main() {
    let port = 4000;
    let addr = SocketAddr::from(([0,0,0,0], port));

    let listener = tokio::net::TcpListener::bind(addr).await
    .unwrap_or_else(|e|{
        panic!("failed to bind to {}:{}", addr, e);
    });

    let router = axum::Router::new()
        .route("/", axum::routing::get(root_handler).post(post_handler));
    
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
