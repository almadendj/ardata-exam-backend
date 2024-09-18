use std::env;

use crate::{Error, Result};
use axum::response::{Html, IntoResponse};
use axum::Json;
use dotenv::dotenv;
use ethers::providers::{Middleware, Provider};
use serde_json::{json, Value};

pub async fn hello_world() -> impl IntoResponse {
    println!("->> {:<12} - hello_world_new", "HANDLER");

    Html("Hello World!!!!!")
}

pub async fn get_gas_price() -> Result<Json<Value>> {
    dotenv().ok();
    println!("->> {:<12} - get_gas_price", "HANDLER");

    let api_key = match env::var("API_KEY") {
        Ok(api_key) => api_key,
        Err(_) => return Err(Error::NoApiKey),
    };

    let rpc_url = format!("https://eth-mainnet.g.alchemy.com/v2/{api_key}");
    let provider = Provider::try_from(rpc_url).unwrap();

    let gas_price = provider.get_gas_price().await.unwrap();

    let body = Json(json!({
        "result": {
            "success": true,
            "gas_price": gas_price.to_string()
        }
    }));

    Ok(body)
}
