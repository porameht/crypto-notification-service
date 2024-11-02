use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    RequestError(String),
    ParseError(String),
    ApiError(String),
}

impl std::error::Error for ServiceError {}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::RequestError(msg) => write!(f, "Request error: {}", msg),
            ServiceError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ServiceError::ApiError(msg) => write!(f, "API error: {}", msg),
        }
    }
}

impl From<reqwest::Error> for ServiceError {
    fn from(err: reqwest::Error) -> Self {
        ServiceError::RequestError(err.to_string())
    }
}

impl From<std::num::ParseFloatError> for ServiceError {
    fn from(err: std::num::ParseFloatError) -> Self {
        ServiceError::ParseError(err.to_string())
    }
}

impl From<std::time::SystemTimeError> for ServiceError {
    fn from(err: std::time::SystemTimeError) -> Self {
        ServiceError::RequestError(err.to_string())
    }
} 