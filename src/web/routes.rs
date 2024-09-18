use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

pub fn routes() -> Router {
    Router::new().route("/", get(hello_world))
}

async fn hello_world() -> impl IntoResponse {
    println!("->> {:<12} - hello_world_new", "HANDLER");

    Html("Hello World!!!!!")
}
