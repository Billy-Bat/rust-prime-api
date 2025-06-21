// Module to handle business logic
use log::debug;
use serde::Serialize;

use crate::errors::{APIError, NotPrimeError, RessourceLockedError};
use crate::state::AppState;

#[derive(Serialize)]
pub struct SecretPrimeOutput {
    secret_prime: u32,
}

pub fn is_prime(number: u32) -> bool {
    if number <= 1 {
        return false;
    }
    for a in 2..number {
        if number % a == 0 {
            return false;
        }
    }
    true
}

pub fn set_secret_prime(state: AppState, target_value: u32) -> Result<u32, APIError> {
    if !is_prime(target_value) {
        return Err(APIError::NotPrime(NotPrimeError(target_value)));
    }
    let mut secret_prime_mutex = state
        .prime_requested
        .lock()
        .map_err(|_| APIError::Locked(RessourceLockedError))?;

    *secret_prime_mutex = target_value;

    Ok(target_value)
}

pub fn get_secret_prime(state: AppState) -> Result<SecretPrimeOutput, APIError> {
    match state.prime_requested.lock() {
        Ok(result) => {
            Ok(SecretPrimeOutput {
                secret_prime: *result,
            })
        }
        Err(err) => {
            debug!(
                "There was an error getting the app secret prime {error}",
                error = err
            );
            Err(APIError::Locked(RessourceLockedError))
        }
    }
}
