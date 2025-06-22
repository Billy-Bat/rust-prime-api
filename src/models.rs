use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct SetSecretPrime {
    pub secretprime: i32,
}

#[derive(Deserialize, ToSchema)]
pub struct GetIsItPrimeQuery {
    pub prime: i32,
}

#[derive(Serialize, ToSchema)]
pub struct SecretPrimeOutput {
    pub secret_prime: i32,
}
