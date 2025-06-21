// Module for route handlers

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use log::debug;
use serde::Deserialize;

use crate::errors::APIError;

use crate::prime::{get_secret_prime, is_prime, set_secret_prime};
use crate::state::AppState;

pub async fn hello_handler() -> String {
    debug!("Calling the hello handler");
    "Hello, Welcome to primer API".to_string()
}

pub async fn health_handler() -> String {
    debug!("Calling the Health handler");
    "Alive and well !".to_string()
}

#[derive(Deserialize)]
pub struct GetIsItPrime {
    prime: u32,
}

pub async fn is_it_prime_handler(Json(payload): Json<GetIsItPrime>) -> impl IntoResponse {
    debug!("Calling the is_it_prime handler");
    let msg = if is_prime(payload.prime) {
        format!("The number {} is indeed prime", payload.prime)
    } else {
        format!("The number {} is not a prime!", payload.prime)
    };
    (StatusCode::OK, msg)
}

pub async fn secret_prime_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, APIError> {
    debug!("Calling the Secret primer handler");

   let result = get_secret_prime(state)?;
Ok(Json(result))
}

#[derive(Deserialize)]
pub struct SetSecretPrime {
    secretprime: u32,
}

pub async fn set_secret_prime_handler(
    State(state): State<AppState>,
    Json(payload): Json<SetSecretPrime>,
) -> Result<impl IntoResponse, APIError> {
    debug!("Calling the secret_prime setter handler");
    let result = set_secret_prime(state, payload.secretprime)?;
    Ok((
        StatusCode::OK,
        format!("Prime was successfully set to {}", result),
    ).into_response())
}
