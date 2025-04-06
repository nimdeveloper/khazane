use core::fmt;
use duckdb;
use serde::Serialize;
use std::{error::Error as StdError, io};

/// Error source for debugging and logging
#[derive(Debug, Serialize, Clone, Copy)]
pub enum ErrorSource {
    Database,
    DatabaseInitialize,
    IO,
    SerDe,
    Custom,
}

/// Custom error type for this application
#[derive(Debug, Serialize)]
pub struct Error {
    pub source: ErrorSource,
    pub message: String,
    #[serde(skip)]
    pub cause: Option<Box<dyn StdError + Send + Sync>>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.cause
            .as_ref()
            .map(|e| e.as_ref() as &(dyn StdError + 'static))
    }
}

impl From<duckdb::Error> for Error {
    fn from(err: duckdb::Error) -> Self {
        Error {
            source: ErrorSource::Database,
            message: format!("Database error: {}", err),
            cause: Some(Box::new(err)),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error {
            source: ErrorSource::IO,
            message: format!("IO error: {}", err),
            cause: Some(Box::new(err)),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error {
            source: ErrorSource::SerDe,
            message: format!("Serialization error: {}", err),
            cause: Some(Box::new(err)),
        }
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error {
            source: ErrorSource::Custom,
            message: msg,
            cause: None,
        }
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error {
            source: ErrorSource::Custom,
            message: msg.to_string(),
            cause: None,
        }
    }
}

// Type alias for using std::result::Result with our Error type
pub type Result<T> = std::result::Result<T, Error>;

// Helper function to create custom errors
pub fn custom_error<S: Into<String>>(message: S) -> Error {
    Error {
        source: ErrorSource::Custom,
        message: message.into(),
        cause: None,
    }
}
