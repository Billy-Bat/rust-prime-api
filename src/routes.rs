// Module for route declaration
use axum::{routing::get, Router};
use utoipa::{openapi::OpenApiBuilder, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use crate::config::Config;
use crate::handlers::{
    health_handler, hello_handler, is_it_prime_handler, secret_prime_handler,
    set_secret_prime_handler, ApiDoc,
};
use crate::state::initialize_shared_state;

pub fn initialize_routes(config: Config) -> Router {
    let _builder: OpenApiBuilder = ApiDoc::openapi().into();

    Router::new()
        .route("/", get(hello_handler))
        .route("/health", get(health_handler))
        .route(
            "/secret_prime",
            get(secret_prime_handler).post(set_secret_prime_handler),
        )
        .route("/is_it_prime", get(is_it_prime_handler))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(initialize_shared_state(config))
}
