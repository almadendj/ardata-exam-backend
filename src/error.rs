use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use url::ParseError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoApiKey,
    ProviderError(String),
    UrlParseError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES_ERROR");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
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
