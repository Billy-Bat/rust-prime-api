use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

pub enum APIError {
    Locked(ResourceLockedError),
    NotPrime(NotPrimeError),
    PrimeTooLarge(PrimeTooLargeError),
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            APIError::Locked(err) => err.fmt(f),
            APIError::NotPrime(err) => err.fmt(f),
            APIError::PrimeTooLarge(err) => err.fmt(f),
        }
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        match self {
            APIError::Locked(err) => err.into_response(),
            APIError::NotPrime(err) => err.into_response(),
            APIError::PrimeTooLarge(err) => err.into_response(),
        }
    }
}

#[derive(Debug)]
pub struct ResourceLockedError;

impl std::error::Error for ResourceLockedError {}

impl fmt::Display for ResourceLockedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "The secret prime is currently inaccessible for some unknown reasons
                try later..."
        )
    }
}

impl IntoResponse for ResourceLockedError {
    fn into_response(self) -> Response {
        (StatusCode::LOCKED, self.to_string()).into_response()
    }
}

#[derive(Debug)]
pub struct NotPrimeError(pub i32);

impl std::error::Error for NotPrimeError {}

impl fmt::Display for NotPrimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The provided number {} is not prime", self.0)
    }
}

impl IntoResponse for NotPrimeError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

#[derive(Debug)]
pub struct PrimeTooLargeError(pub i32);

impl std::error::Error for PrimeTooLargeError {}

impl fmt::Display for PrimeTooLargeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The prime requested may be too large, lower your expectations, {} is probably too much", self.0)
    }
}

impl IntoResponse for PrimeTooLargeError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}
