use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};
use url::ParseError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoApiKey,
    NoApiUrl,
    ProviderError(String),
    UrlParseError(String),
    UnhandledError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES_ERROR");

        let (status, error_message) = match self {
            Error::NoApiKey => (
                StatusCode::BAD_REQUEST,
                into_json("API Key is missing.".to_string()),
            ),
            Error::ProviderError(_) => (
                StatusCode::BAD_GATEWAY,
                into_json("Something went wrong.".to_string()),
            ),
            Error::UrlParseError(_) => (
                StatusCode::BAD_REQUEST,
                into_json("Wrong API url".to_string()),
            ),
            Error::NoApiUrl => (
                StatusCode::BAD_REQUEST,
                into_json("API url is missing.".to_string()),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                into_json("UNHANDLED_CLIENT_ERROR".to_string()),
            ),
        };

        (status, error_message).into_response()
    }
}

fn into_json(message: String) -> Json<Value> {
    Json(json!({
    "error": message
    }))
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Error::UrlParseError(value.to_string())
    }
}

impl From<ethers::providers::ProviderError> for Error {
    fn from(value: ethers::providers::ProviderError) -> Self {
        println!("->> {:<12} - {value:?}", "ETHERS_ERROR");

        Error::ProviderError(value.to_string())
    }
}
