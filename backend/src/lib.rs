pub mod db;
pub mod routes;
pub mod utils;
pub mod ws;

use dotenv::dotenv;
use envy::from_env;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ENV {
    pub rust_log: String,
    pub port: u16,
    pub jwt_secret: String,
    pub mongo_uri: String,
}

pub fn import_envs() -> ENV {
    // since error in envs is a fatal error so we can unwrap and panic
    dotenv().ok();
    from_env::<ENV>().unwrap()
}
