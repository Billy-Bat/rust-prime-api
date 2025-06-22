use log::{error, info};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub secret_prime: i32,
    pub rust_log: Option<String>,
}

pub fn load_env_variables() -> Result<Config, envy::Error> {
    // For local development, load .env values
    dotenv::dotenv().ok();

    info!("Attempting to read environment variables");
    match envy::from_env::<Config>() {
        Ok(config_values) => {
            info!("Successfully read config values from env");
            Ok(config_values)
        }
        Err(error) => {
            error!(
                "Could not successfully read value from the environment variables\n\
                Program will now terminate..."
            );
            Err(error)
        }
    }
}
