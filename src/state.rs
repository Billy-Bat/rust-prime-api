// Module for app state management

use crate::config::Config;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub prime_requested: Arc<Mutex<u32>>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            prime_requested: Arc::new(Mutex::new(8)),
        }
    }
}

impl From<Config> for AppState {
    fn from(config: Config) -> Self {
        AppState {
            prime_requested: Arc::new(Mutex::new(config.secret_prime)),
        }
    }
}

pub fn initialize_shared_state(config: Config) -> AppState {
    AppState::from(config)
}
