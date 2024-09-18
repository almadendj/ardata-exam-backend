use crate::{Error, Result};
use axum::response::{Html, IntoResponse};
use axum::Json;
use ethers::providers::{Middleware, Provider};
use serde_json::{json, Value};

pub async fn hello_world() -> impl IntoResponse {
    println!("->> {:<12} - hello_world_new", "HANDLER");

    Html("Hello World!!!!!")
}

pub async fn get_gas_price() -> Result<Json<Value>> {
    println!("->> {:<12} - get_gas_price", "HANDLER");

    // TODO: move to env
    let api_key = "wzaVIkJ_jhBVL-CxeUDOZdHvfzz9lliP";

    if api_key.is_empty() {
        return Err(Error::NoApiKey);
    }

    let rpc_url = format!("https://eth-mainnet.g.alchemy.com/v2/{api_key}");
    let provider = Provider::try_from(rpc_url).unwrap();

    let gas_price = provider.get_gas_price().await.unwrap();

    println!("gas price: {gas_price}");

    let body = Json(json!({
        "result": {
            "success": true,
            "gas_price": gas_price.to_string()
        }
    }));

    Ok(body)
}
