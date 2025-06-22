// Module for route handlers
use log::debug;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{extract::Query, Json};

use utoipa::OpenApi;

use crate::errors::APIError;
use crate::models::SecretPrimeOutput;
use crate::models::{GetIsItPrimeQuery, SetSecretPrime};
use crate::prime::{get_secret_prime, is_prime, is_requested_number_in_bound, set_secret_prime};
use crate::state::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(
        hello_handler,
        health_handler,
        is_it_prime_handler,
        secret_prime_handler,
        set_secret_prime_handler
    ),
    info(description = "Primer API for Prime lovers")
)]
pub struct ApiDoc;

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Welcome to the API, this is the root endpoint")
    ),
)]
pub async fn hello_handler() -> String {
    debug!("Calling the hello handler");
    "Hello, Welcome to primer API".to_string()
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health endpoint, everything is good as long as i am ok")
    ),
)]
pub async fn health_handler() -> String {
    debug!("Calling the Health handler");
    "Alive and well !".to_string()
}

#[utoipa::path(
    get,
    path = "/is_it_prime",
    responses(
        (status = 200, description = "Send a number here to check if its prime")
    ),
    params(
        ("id" = i32, Path, description = "Prime to check against"),
    )
)]
pub async fn is_it_prime_handler(
    Query(payload): Query<GetIsItPrimeQuery>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, APIError> {
    is_requested_number_in_bound(&state, payload.prime)?;

    debug!("Calling the is_it_prime handler");
    let msg = if is_prime(payload.prime) {
        format!("The number {} is indeed prime", payload.prime)
    } else {
        format!("The number {} is not a prime!", payload.prime)
    };
    Ok((StatusCode::OK, msg))
}

#[utoipa::path(
    get,
    path = "/secret_prime",
    responses(
        (status = 200, description = "Returns the one and only secret prime", body = SecretPrimeOutput)
    )
)]
pub async fn secret_prime_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, APIError> {
    debug!("Calling the Secret primer handler");

    let result = get_secret_prime(state)?;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/secret_prime",
    responses(
        (status = 200, description = "Returns a text acknowledging the new value of the secret prime")
    )
)]
pub async fn set_secret_prime_handler(
    State(state): State<AppState>,
    Json(payload): Json<SetSecretPrime>,
) -> Result<impl IntoResponse, APIError> {
    is_requested_number_in_bound(&state, payload.secretprime)?;
    debug!("Calling the secret_prime setter handler");
    let result = set_secret_prime(state, payload.secretprime)?;
    Ok((
        StatusCode::OK,
        format!("Prime was successfully set to {}", result),
    )
        .into_response())
}
