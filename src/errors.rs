use thiserror::Error;

/// Enum representing various errors that can occur in the UniFi client library.
#[derive(Debug, Error)]
pub enum UnifiError {
    /// Represents an HTTP error, wrapping the underlying `reqwest::Error`.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// Represents an API error, containing the status code and error message.
    #[error("API error: {status_code} - {message}")]
    Api {
        /// The HTTP status code returned by the API.
        status_code: u16,
        /// The error message returned by the API.
        message: String,
    },

    /// Represents an error when parsing a URL, wrapping the underlying `url::ParseError`.
    #[error("Invalid URL: {0}")]
    Url(#[from] url::ParseError),

    /// Represents a configuration error, containing a descriptive error message.
    #[error("Configuration error: {0}")]
    Config(String),
}