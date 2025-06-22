use log::Level;
use log::{debug, info};

mod config;
mod errors;
mod handlers;
mod models;
mod prime;
mod routes;
mod state;

use crate::config::load_env_variables;
use crate::routes::initialize_routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Booting up Primer API");
    let configuration = load_env_variables()?;

    // Overwrite logger if variable is provided in env
    if let Some(log_level) = &configuration.rust_log {
        println!("Overwriting logger with level `{}`", log_level);
        simple_logger::init_with_env().unwrap();
    } else {
        println!("Setting logger with Info level");
        simple_logger::init_with_level(Level::Info).unwrap();
    }

    debug!("Defining routes");
    let routes = initialize_routes(configuration);

    debug!("Initialize openapi swagger");

    info!("Starting Server");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    info!("The server will now Listening on 127.0.0.1:8080");
    axum::serve(listener, routes).await.unwrap();

    Ok(())
}
