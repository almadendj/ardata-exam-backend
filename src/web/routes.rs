use axum::{routing::get, Router};

use super::handlers;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(handlers::hello_world))
        .route("/get-gas-price", get(handlers::get_gas_price))
}
