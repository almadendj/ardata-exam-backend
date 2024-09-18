use std::net::SocketAddr;

pub use self::error::{Error, Result};
use axum::routing::get_service;
use axum::Router;
use tower_http::services::ServeDir;

mod error;
mod web;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .merge(web::routes::routes())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("->> LISTENING on {addr}\n");

    axum::serve(listener, routes_hello).await.unwrap();
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
