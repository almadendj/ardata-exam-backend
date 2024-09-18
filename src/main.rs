use std::net::SocketAddr;

use axum::response::Html;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>World</strong>") }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("->> LISTENING on {addr}\n");

    axum::serve(listener, routes_hello).await.unwrap();
}
