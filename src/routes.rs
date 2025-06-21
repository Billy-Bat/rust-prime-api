// Module for route declaration

use crate::config::Config;
use crate::handlers::{
    health_handler, hello_handler, secret_prime_handler, set_secret_prime_handler, is_it_prime_handler
};
use crate::state::initialize_shared_state;
use axum::{routing::get, Router};

pub fn initialize_routes(config: Config) -> Router {
    let routes = Router::new()
        .route("/", get(hello_handler))
        .route("/health", get(health_handler))
        .route("/secret_prime", get(secret_prime_handler).post(set_secret_prime_handler))
        .route("/is_it_prime", get(is_it_prime_handler))
        .with_state(initialize_shared_state(config));
    routes
}
