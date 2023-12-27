use axum::routing::get;
use axum::Router;

pub mod chat;
pub mod gang;
mod user;

use user::User;

pub struct API;

impl API {
    pub fn generate_route() -> Router {
        Router::new()
            // health route
            .route("/health-check", get(handle_health_check))
            // User routes
            .nest("/user", User::generate_routes())
    }
}

async fn handle_health_check() -> &'static str {
    "Server is online!"
}
