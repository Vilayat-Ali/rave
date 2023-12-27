pub mod db;
pub mod routes;
pub mod utils;
pub mod ws;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ENV {
    pub rust_log: String,
    pub jwt_secret: String,
    pub mongo_uri: String,
}
