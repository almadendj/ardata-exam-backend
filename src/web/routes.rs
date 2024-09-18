use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/get-gas-price", get(get_gas_price))
}

async fn hello_world() -> impl IntoResponse {
    println!("->> {:<12} - hello_world_new", "HANDLER");

    Html("Hello World!!!!!")
}

async fn get_gas_price() {
    println!("->> {:<12} - get_gas_price", "HANDLER");
}
