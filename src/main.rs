use std::net::SocketAddr;

pub use self::error::{Error, Result};
use axum::http::Method;
use axum::routing::get_service;
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

mod error;
mod web;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any) // allow any origin for now
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(tower_http::cors::Any);

    let routes = Router::new()
        .merge(web::routes::routes())
        .fallback_service(routes_static())
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("->> LISTENING on {addr}\n");

    axum::serve(listener, routes).await.unwrap();
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
