use std::env;

use crate::{Error, Result};
use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use axum::Json;
use dotenv::dotenv;
use ethers::abi::Address;
use ethers::providers::{Middleware, Provider};
use serde_json::{json, Value};

pub async fn hello_world() -> impl IntoResponse {
    println!("->> {:<12} - hello_world_new", "HANDLER");

    Html("Hello World!!!!!")
}

pub async fn get_balance(Path(address): Path<String>) -> Result<Json<Value>> {
    dotenv().ok();
    println!("->> {:<12} - get_balance", "HANDLER");

    let api_key = match env::var("API_KEY") {
        Ok(api_key) => api_key,
        Err(_) => return Err(Error::NoApiKey),
    };

    let base_url = match env::var("API_URL") {
        Ok(api_url) => api_url,
        Err(_) => return Err(Error::NoApiUrl),
    };

    let rpc_url = format!("{base_url}{api_key}");
    let provider = Provider::try_from(rpc_url)?;

    let address = address
        .parse::<Address>()
        .map_err(|_| Error::UrlParseError("Invalid Ethereum Address".to_string()))?;

    let balance = provider.get_balance(address, None).await?;

    let body = Json(json!({
    "result": {
        "success": true,
        "address": address,
        "balance": balance.to_string()
        }
    }));

    Ok(body)
}

pub async fn get_gas_price() -> Result<Json<Value>> {
    dotenv().ok();
    println!("->> {:<12} - get_gas_price", "HANDLER");

    let api_key = match env::var("API_KEY") {
        Ok(api_key) => api_key,
        Err(_) => return Err(Error::NoApiKey),
    };

    let base_url = match env::var("API_URL") {
        Ok(api_url) => api_url,
        Err(_) => return Err(Error::NoApiUrl),
    };

    let rpc_url = format!("{base_url}{api_key}");
    let provider = Provider::try_from(rpc_url)?;

    let gas_price = provider.get_gas_price().await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "gas_price": gas_price.to_string()
        }
    }));

    Ok(body)
}

pub async fn get_block_number() -> Result<Json<Value>> {
    dotenv().ok();
    println!("->> {:<12} - get_block_number", "HANDLER");

    let api_key = match env::var("API_KEY") {
        Ok(api_key) => api_key,
        Err(_) => return Err(Error::NoApiKey),
    };

    let base_url = match env::var("API_URL") {
        Ok(api_url) => api_url,
        Err(_) => return Err(Error::NoApiUrl),
    };

    let rpc_url = format!("{base_url}{api_key}");
    let provider = Provider::try_from(rpc_url)?;
    let block_number = provider.get_block_number().await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "block_number": block_number.to_string()
        }
    }));

    Ok(body)
}
