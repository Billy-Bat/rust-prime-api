// Module to handle business logic
use log::debug;

use crate::errors::{APIError, NotPrimeError, PrimeTooLargeError, ResourceLockedError};
use crate::models::SecretPrimeOutput;
use crate::state::AppState;

pub fn is_requested_number_in_bound(state: &AppState, requested: i32) -> Result<(), APIError> {
    if requested < state.lower_bound || requested > state.upper_bound_prime {
        return Err(APIError::PrimeTooLarge(PrimeTooLargeError(requested)));
    }
    Ok(())
}

pub fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as i32;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn set_secret_prime(state: AppState, target_value: i32) -> Result<i32, APIError> {
    if !is_prime(target_value) {
        return Err(APIError::NotPrime(NotPrimeError(target_value)));
    }
    let mut secret_prime_mutex = state
        .prime_requested
        .lock()
        .map_err(|_| APIError::Locked(ResourceLockedError))?;

    *secret_prime_mutex = target_value;

    Ok(target_value)
}

pub fn get_secret_prime(state: AppState) -> Result<SecretPrimeOutput, APIError> {
    match state.prime_requested.lock() {
        Ok(result) => Ok(SecretPrimeOutput {
            secret_prime: *result,
        }),
        Err(err) => {
            debug!(
                "There was an error getting the app secret prime {error}",
                error = err
            );
            Err(APIError::Locked(ResourceLockedError))
        }
    }
}
