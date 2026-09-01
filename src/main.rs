use core::panic;
use std::{env, net::SocketAddr, sync::Arc};

use axum::{http::HeaderName};
use f5a_services::{context::AppContext, routes::{self}};
use sea_orm::Database;
use tower::ServiceBuilder;
use tower_http::{
    propagate_header::{PropagateHeaderLayer}, request_id::{MakeRequestUuid, SetRequestIdLayer}, trace::{DefaultMakeSpan, TraceLayer},
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in env");

    println!("db url: {}", database_url);

    let conn = Database::connect(database_url)
        .await
        .expect("Failed to connect to database");

    let ctx = AppContext { conn: Arc::new(conn) };

    let port = 4000;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| {
            panic!("failed to bind to {}:{}", addr, e);
        });

    let service_layer = ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(
            PropagateHeaderLayer::new(HeaderName::from_static("x-request-id"))
        );
    
    let router = routes::router().with_state(ctx).layer(service_layer);
    
    tracing::info!(addr = ?listener.local_addr().unwrap(), app_name = "f5a_services_es","Listening");

    axum::serve(listener, router).await.unwrap_or_else(|err| {
        panic!("failed to start server: {}", err);
    });
}

