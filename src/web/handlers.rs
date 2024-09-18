use axum::response::{Html, IntoResponse};

pub async fn hello_world() -> impl IntoResponse {
    println!("->> {:<12} - hello_world_new", "HANDLER");

    Html("Hello World!!!!!")
}

pub async fn get_gas_price() {
    println!("->> {:<12} - get_gas_price", "HANDLER");
}
