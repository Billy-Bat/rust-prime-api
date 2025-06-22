// Module for app state management

use crate::config::Config;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub prime_requested: Arc<Mutex<i32>>,
    pub upper_bound_prime: i32,
    pub lower_bound: i32,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            prime_requested: Arc::new(Mutex::new(8)),
            upper_bound_prime: i32::from(i16::MAX),
            lower_bound: 0,
        }
    }
}

impl From<Config> for AppState {
    fn from(config: Config) -> Self {
        AppState {
            prime_requested: Arc::new(Mutex::new(config.secret_prime)),
            ..Default::default()
        }
    }
}

pub fn initialize_shared_state(config: Config) -> AppState {
    AppState::from(config)
}
