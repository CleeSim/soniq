//! Error types for the Soniq client.

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Errors that can occur when interacting with the Last.fm API.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An API secret is required for this method but was not provided.
    #[error("API secret is required for signed calls")]
    MissingApiSecret,

    /// A Last.fm-specific error response.
    /// See [`ErrorResponse`].
    #[error("Last.fm API error: {0}")]
    LastFm(#[from] ErrorResponse),

    /// An HTTP error that is not a specific Last.fm error.
    /// This can happen for non-2xx responses that don't conform to the Last.fm error format.
    #[error("HTTP error {status}: {text}")]
    Http { status: StatusCode, text: String },

    /// A network or request-related error from the underlying HTTP client.
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    /// A JSON deserialization error.
    #[error("JSON deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    /// An error parsing a URL.
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),
}

/// An error response from the Last.fm API.
///
/// See <https://www.last.fm/api/errorcodes> for a list of error codes.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct ErrorResponse {
    pub error: u32,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {}: {}", self.error, self.message)
    }
}

impl std::error::Error for ErrorResponse {}
