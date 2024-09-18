use axum::{routing::get, Router};

use super::handlers;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(handlers::hello_world))
        .route("/get-gas-price", get(handlers::get_gas_price))
        .route("/get-block-number", get(handlers::get_block_number))
        .route("/get-balance/:address", get(handlers::get_balance))
}
